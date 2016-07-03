// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;
use std::error::{Error};
use std::fmt::{self,Display};
use std::hash::Hash;
use std::io;

#[cfg(feature = "bigint")]
use num::bigint::{BigInt,BigUint,ToBigUint};

use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ASN1Error {
    Eof, Extra, IntegerOverflow, StackOverflow, Invalid,
}

pub type ASN1Result<T> = Result<T, ASN1Error>;

impl Display for ASN1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{:?}", self));
        return Ok(());
    }
}

impl Error for ASN1Error {
    fn description(&self) -> &str {
        match *self {
            ASN1Error::Eof => "End of file",
            ASN1Error::Extra => "Extra data in file",
            ASN1Error::IntegerOverflow => "Integer overflow",
            ASN1Error::StackOverflow => "Stack overflow",
            ASN1Error::Invalid => "Invalid data",
        }
    }
}

impl From<ASN1Error> for io::Error {
    fn from(e: ASN1Error) -> Self {
        return io::Error::new(io::ErrorKind::InvalidData, e);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BERMode {
    Ber, Cer, Der,
}

#[derive(Debug)]
enum TagState {
    None,
    Cached(Tag, PC),
    Implicit(PC, Option<usize>),
}

#[derive(Debug)]
pub struct BERReader<'a> {
    buf: &'a [u8],
    pos: usize,
    mode: BERMode,
    tag_state: TagState,
    depth: usize,
}

const BER_READER_STACK_DEPTH : usize = 100;

impl<'a> BERReader<'a> {
    fn new(buf: &'a [u8], mode: BERMode) -> Self {
        return BERReader {
            buf: buf,
            pos: 0,
            mode: mode,
            tag_state: TagState::None,
            depth: 0,
        };
    }
    fn read_u8(&mut self) -> ASN1Result<u8> {
        if self.pos < self.buf.len() {
            let ret = self.buf[self.pos];
            self.pos += 1;
            return Ok(ret);
        } else {
            return Err(ASN1Error::Eof);
        }
    }
    pub fn remaining_buffer(&self) -> &'a [u8] {
        return &self.buf[self.pos..];
    }
    fn fetch_remaining_buffer(&mut self) -> &'a [u8] {
        let ret = &self.buf[self.pos..];
        self.pos = self.buf.len();
        return ret;
    }
    fn end_of_buf(&mut self) -> ASN1Result<()> {
        if self.pos != self.buf.len() {
            return Err(ASN1Error::Extra);
        }
        return Ok(());
    }
    fn end_of_contents(&mut self) -> ASN1Result<()> {
        let (tag, pc) = match self.tag_state {
            TagState::None => try!(self.parse_identifier()),
            TagState::Cached(tag, pc) => (tag, pc),
            TagState::Implicit(_, _) => { return Err(ASN1Error::Invalid); },
        };
        if tag != TAG_EOC || pc != PC::Primitive {
            return Err(ASN1Error::Invalid);
        }
        let b = try!(self.read_u8());
        if b != 0 {
            return Err(ASN1Error::Invalid);
        }
        return Ok(());
    }
    fn parse_identifier(&mut self) -> ASN1Result<(Tag, PC)> {
        let tagbyte = try!(self.read_u8());
        let tag_class = TAG_CLASSES[(tagbyte >> 6) as usize];
        let pc = PCS[((tagbyte >> 5) & 1) as usize];
        let mut tag_number = (tagbyte & 31) as u64;
        if tag_number == 31 {
            tag_number = 0;
            loop {
                let b = try!(self.read_u8()) as u64;
                let x =
                    try!(tag_number.checked_mul(128).ok_or(
                        ASN1Error::IntegerOverflow));
                tag_number = x + (b & 127);
                if (b & 128) == 0 {
                    break;
                }
            }
            if tag_number < 31 {
                return Err(ASN1Error::Invalid);
            }
        }
        let tag = Tag {
            tag_class: tag_class,
            tag_number: tag_number,
        };
        return Ok((tag, pc));
    }
    fn parse_length(&mut self) -> ASN1Result<Option<usize>> {
        let lbyte = try!(self.read_u8()) as usize;
        if lbyte == 128 {
            return Ok(None);
        }
        if lbyte == 255 {
            return Err(ASN1Error::Invalid);
        }
        if (lbyte & 128) == 0 {
            return Ok(Some(lbyte));
        }
        let mut length : usize = 0;
        for _ in 0..(lbyte & 127) {
            let x = try!(length.checked_mul(256).ok_or(ASN1Error::Eof));
            length = x + (try!(self.read_u8()) as usize);
        }
        if (self.mode == BERMode::Der || self.mode == BERMode::Cer)
                && length < 128 {
            return Err(ASN1Error::Invalid);
        }
        return Ok(Some(length));
    }
    fn parse_general<T, F>
            (&mut self, tag: Tag, tag_type: TagType, mut fun: F)
            -> ASN1Result<T>
            where F: FnMut(&mut Self, PC) -> ASN1Result<T> {
        if self.depth > BER_READER_STACK_DEPTH {
            return Err(ASN1Error::StackOverflow);
        }
        let pc;
        let length_spec;
        match self.tag_state {
            TagState::None => {
                let (tag2, pc2) = try!(self.parse_identifier());
                if tag2 != tag {
                    self.tag_state = TagState::Cached(tag2, pc2);
                    return Err(ASN1Error::Invalid);
                }
                pc = pc2;
                length_spec = try!(self.parse_length());
            },
            TagState::Cached(tag2, pc2) => {
                if tag2 != tag {
                    return Err(ASN1Error::Invalid);
                }
                pc = pc2;
                length_spec = try!(self.parse_length());
            },
            TagState::Implicit(pc2, length_spec2) => {
                pc = pc2;
                length_spec = length_spec2;
            }
        };
        let old_buf = self.buf;
        if tag_type == TagType::Implicit {
            self.tag_state = TagState::Implicit(pc, length_spec);
        } else {
            self.tag_state = TagState::None;
        }
        match length_spec {
            Some(length) => {
                if self.mode == BERMode::Cer && pc != PC::Primitive {
                    return Err(ASN1Error::Invalid);
                }
                let limit = self.pos+length;
                if old_buf.len() < limit {
                    return Err(ASN1Error::Eof);
                }
                self.buf = &old_buf[..limit];
            },
            None => {
                if pc != PC::Constructed {
                    return Err(ASN1Error::Invalid);
                }
                if self.mode == BERMode::Der {
                    return Err(ASN1Error::Invalid);
                }
            },
        };
        self.depth += 1;
        let result = try!(fun(self, pc));
        self.depth -= 1;
        match length_spec {
            Some(_) => {
                try!(self.end_of_buf());
            },
            None => {
                try!(self.end_of_contents());
            },
        };
        self.buf = old_buf;
        return Ok(result);
    }
    pub fn parse_optional<T, F>(&mut self, mut fun: F) -> ASN1Result<Option<T>>
            where F: FnMut(&mut Self) -> ASN1Result<T> {
        if self.pos == self.buf.len() {
            return Ok(None);
        }
        match self.tag_state {
            TagState::None => {
                let (tag, pc) = try!(self.parse_identifier());
                self.tag_state = TagState::Cached(tag, pc);
            },
            TagState::Cached(_, _) => {},
            TagState::Implicit(_, _) => {},
        };
        let old_pos = self.pos;
        match fun(self) {
            Ok(result) => Ok(Some(result)),
            Err(e) =>
                if old_pos == self.pos {
                    Ok(None)
                } else {
                    Err(e)
                },
        }
    }
    pub fn parse_default<T, F>(&mut self, default: T, fun: F) -> ASN1Result<T>
            where F: FnMut(&mut Self) -> ASN1Result<T>, T: Eq {
        match try!(self.parse_optional(fun)) {
            Some(result) => {
                if (self.mode == BERMode::Der || self.mode == BERMode::Cer) &&
                        result == default {
                    return Err(ASN1Error::Invalid);
                }
                return Ok(result);
            },
            None => Ok(default),
        }
    }
    pub fn parse_with_buffer<T, F>(&mut self, mut fun: F)
            -> ASN1Result<(T, &'a [u8])>
            where F: FnMut(&mut Self) -> ASN1Result<T> {
        let old_pos = self.pos;
        let result = try!(fun(self));
        let new_pos = self.pos;
        let buf = &self.buf[old_pos..new_pos];
        return Ok((result, buf));
    }
    pub fn parse_tagged<T, F>
            (&mut self, tag: Tag, tag_type: TagType, mut fun: F)
            -> ASN1Result<T>
            where F: FnMut(&mut Self) -> ASN1Result<T> {
        self.parse_general(tag, tag_type, |parser, pc| {
            if tag_type == TagType::Explicit && pc != PC::Constructed {
                return Err(ASN1Error::Invalid);
            }
            fun(parser)
        })
    }
    pub fn from_buf<T, F>(buf: &'a [u8], mode: BERMode, mut fun: F)
            -> ASN1Result<T>
            where F: FnMut(&mut Self) -> ASN1Result<T> {
        let mut parser = Self::new(buf, mode);
        let result = try!(fun(&mut parser));
        try!(parser.end_of_buf());
        return Ok(result);
    }
    pub fn parse_sequence<T, F>(&mut self, mut fun: F) -> ASN1Result<T>
            where F: FnMut(&mut Self) -> ASN1Result<T> {
        self.parse_general(TAG_SEQUENCE, TagType::Explicit, |parser, pc| {
            if pc != PC::Constructed {
                return Err(ASN1Error::Invalid);
            }
            return fun(parser);
        })
    }
    pub fn parse_set<T, F>(&mut self, mut fun: F) -> ASN1Result<T>
            where F: FnMut(&mut Self) -> ASN1Result<T> {
        self.parse_general(TAG_SET, TagType::Explicit, |parser, pc| {
            if pc != PC::Constructed {
                return Err(ASN1Error::Invalid);
            }
            return fun(parser);
        })
    }
    fn parse_bitstring(&mut self) -> ASN1Result<BitString> {
        self.parse_general(TAG_BITSTRING, TagType::Explicit, |parser, pc| {
            if pc == PC::Constructed {
                // TODO: implement recursive encoding
                return Err(ASN1Error::Invalid);
            } else {
                // TODO: Canonicity check in DER
                let buf = parser.fetch_remaining_buffer();
                if buf.len() == 0 {
                    return Ok(BitString::from_buf(0, Vec::new()));
                }
                let remain = buf[0] as usize;
                return Ok(BitString::from_buf(
                    remain % 8,
                    buf[1..buf.len()-remain/8].to_vec()
                ));
            }
        })
    }
    fn parse_octetstring_impl(&mut self, vec: &mut Vec<u8>, depth: usize)
            -> ASN1Result<()> {
        self.parse_general(TAG_OCTETSTRING, TagType::Explicit, |parser, pc| {
            if parser.mode == BERMode::Cer && depth >= 2 {
                return Err(ASN1Error::Invalid);
            }
            if pc == PC::Constructed {
                if parser.mode == BERMode::Der {
                    return Err(ASN1Error::Invalid);
                }
                if parser.mode == BERMode::Cer && depth >= 1 {
                    return Err(ASN1Error::Invalid);
                }
                let mut expected_len : Option<usize> = None;
                loop {
                    let result = try!(parser.parse_optional(|parser| {
                        parser.parse_octetstring_impl(vec, depth+1)
                    }));
                    match result {
                        Some(()) => {
                            if parser.mode == BERMode::Cer &&
                                    vec.len() % 1000 != 0 {
                                if expected_len != None {
                                    return Err(ASN1Error::Invalid);
                                }
                                expected_len = Some(vec.len());
                            }
                        },
                        None => { break; },
                    }
                }
                match expected_len {
                    Some(el) => if el != vec.len() {
                        return Err(ASN1Error::Invalid);
                    },
                    None => {},
                };
                return Ok(());
            } else {
                if parser.mode == BERMode::Cer &&
                        parser.remaining_buffer().len() > 1000 {
                    return Err(ASN1Error::Invalid);
                }
                vec.extend(parser.fetch_remaining_buffer());
                return Ok(());
            }
        })
    }
    pub fn parse_octetstring(&mut self) -> ASN1Result<Vec<u8>> {
        let mut ret = Vec::new();
        try!(self.parse_octetstring_impl(&mut ret, 0));
        return Ok(ret);
    }
    pub fn parse<T:FromBER>(&mut self) -> ASN1Result<T> {
        return T::from_ber(self);
    }
}

pub trait FromBER: Sized + Eq + Hash {
    fn from_ber<'a>(parser: &mut BERReader<'a>) -> ASN1Result<Self>;

    fn from_buf(src: &[u8], mode: BERMode) -> ASN1Result<Self> {
        return BERReader::from_buf(src, mode, |parser| {
            return Self::from_ber(parser);
        });
    }
}

impl<T> FromBER for Vec<T> where T: Sized + Eq + Hash + FromBER {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_sequence(|parser| {
            let mut ret = Vec::new();
            loop {
                let result = try!(parser.parse_optional(|parser| {
                    T::from_ber(parser)
                }));
                match result {
                    Some(result) => {
                        ret.push(result);
                    },
                    None => {
                        break;
                    }
                };
            }
            return Ok(ret);
        })
    }
}

impl<T> FromBER for SetOf<T> where T: Sized + Eq + Hash + FromBER {
    fn from_ber<'a>(parser: &mut BERReader<'a>) -> ASN1Result<Self> {
        parser.parse_set(|parser| {
            let mut ret = SetOf::new();
            let mut old_buf : Option<&'a [u8]> = None;
            loop {
                let (result, buf) = try!(parser.parse_with_buffer(|parser| {
                    parser.parse_optional(|parser| {
                        T::from_ber(parser)
                    })
                }));
                match result {
                    Some(result) => {
                        ret.vec.push(result);
                    },
                    None => {
                        break;
                    },
                };
                if parser.mode == BERMode::Der || parser.mode == BERMode::Cer {
                    match old_buf {
                        Some(old_buf) => {
                            match old_buf.iter().cmp(buf.iter()) {
                                Ordering::Less => {},
                                Ordering::Equal => {
                                    if old_buf.len() > buf.len() {
                                        return Err(ASN1Error::Invalid);
                                    }
                                },
                                Ordering::Greater => {
                                    return Err(ASN1Error::Invalid);
                                },
                            }
                        }
                        None => {},
                    }
                }
                old_buf = Some(buf);
            }
            return Ok(ret);
        })
    }
}

impl FromBER for i64 {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_general(TAG_INTEGER, TagType::Explicit, |parser, pc| {
            if pc != PC::Primitive {
                return Err(ASN1Error::Invalid);
            }
            let buf = parser.fetch_remaining_buffer();
            if buf.len() == 0 {
                return Err(ASN1Error::Invalid);
            } else if buf.len() == 1 {
                return Ok(buf[0] as i8 as i64);
            }
            let mut x = ((buf[0] as i8 as i64) << 8) + (buf[1] as i64);
            if -128 <= x && x < 128 {
                return Err(ASN1Error::Invalid);
            }
            if buf.len() > 8 {
                return Err(ASN1Error::IntegerOverflow);
            }
            for &b in buf[2..].iter() {
                x = (x << 8) | (b as i64);
            }
            return Ok(x);
        })
    }
}

#[cfg(feature = "bigint")]
impl FromBER for BigInt {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_general(TAG_INTEGER, TagType::Explicit, |parser, pc| {
            if pc != PC::Primitive {
                return Err(ASN1Error::Invalid);
            }
            let buf = parser.fetch_remaining_buffer();
            if buf.len() == 0 {
                return Err(ASN1Error::Invalid);
            } else if buf.len() == 1 {
                return Ok(BigInt::from(buf[0] as i8));
            }
            let mut x = (BigInt::from(buf[0] as i8) << 8) +
                BigInt::from(buf[1] as i64);
            if BigInt::from(-128) <= x && x < BigInt::from(128) {
                return Err(ASN1Error::Invalid);
            }
            for &b in buf[2..].iter() {
                x = (x << 8) + BigInt::from(b);
            }
            return Ok(x);
        })
    }
}

#[cfg(feature = "bigint")]
impl FromBER for BigUint {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        match try!(parser.parse::<BigInt>()).to_biguint() {
            Some(result) => Ok(result),
            None => Err(ASN1Error::Invalid),
        }
    }
}

impl FromBER for () {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_general(TAG_NULL, TagType::Explicit, |parser, pc| {
            if pc != PC::Primitive {
                return Err(ASN1Error::Invalid);
            }
            let buf = parser.fetch_remaining_buffer();
            if buf.len() != 0 {
                return Err(ASN1Error::Invalid);
            }
            return Ok(());
        })
    }
}

const TAG_CLASSES : [TagClass; 4] = [
    TagClass::Universal,
    TagClass::Application,
    TagClass::ContextSpecific,
    TagClass::Private,
];

const TAG_EOC : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 0,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum PC {
    Primitive = 0, Constructed = 1,
}

const PCS : [PC; 2] = [PC::Primitive, PC::Constructed];

impl FromBER for bool {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_general(TAG_BOOLEAN, TagType::Explicit, |parser, pc| {
            if pc != PC::Primitive {
                return Err(ASN1Error::Invalid);
            }
            let buf = parser.fetch_remaining_buffer();
            if buf.len() != 1 {
                return Err(ASN1Error::Invalid);
            }
            let b = buf[0];
            if (parser.mode == BERMode::Der || parser.mode == BERMode::Cer) &&
                    b != 0 && b != 255 {
                return Err(ASN1Error::Invalid);
            }
            return Ok(b != 0);
        })
    }
}

impl FromBER for BitString {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_bitstring()
    }
}

impl FromBER for Vec<u8> {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_octetstring()
    }
}

impl FromBER for ObjectIdentifier {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_general(TAG_OID, TagType::Explicit, |parser, pc| {
            if pc != PC::Primitive {
                return Err(ASN1Error::Invalid);
            }
            let mut ids = Vec::new();
            let buf = parser.fetch_remaining_buffer();
            if buf.len() == 0 || buf[buf.len()-1] >= 128 {
                return Err(ASN1Error::Invalid);
            }
            let mut subid : u64 = 0;
            for &b in buf.iter() {
                if b == 128 {
                    return Err(ASN1Error::Invalid);
                }
                subid = try!(subid.checked_mul(128)
                    .ok_or(ASN1Error::IntegerOverflow)) + ((b & 127) as u64);
                if (b & 128) == 0 {
                    if ids.len() == 0 {
                        let id0 = if subid < 40 {
                            0
                        } else if subid < 80 {
                            1
                        } else {
                            2
                        };
                        let id1 = subid - 40 * id0;
                        ids.push(id0);
                        ids.push(id1);
                    } else {
                        ids.push(subid);
                    }
                    subid = 0;
                }
            }
            return Ok(Self::new(ids));
        })
    }
}

impl FromBER for PrintableString {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_tagged(TAG_PRINTABLESTRING, TagType::Implicit, |parser| {
            let octets = try!(parser.parse_octetstring());
            return PrintableString::from_bytes(octets)
                .ok_or(ASN1Error::Invalid);
        })
    }
}

impl FromBER for UtcTime {
    fn from_ber(parser: &mut BERReader) -> ASN1Result<Self> {
        parser.parse_tagged(TAG_UTCTIME, TagType::Implicit, |parser| {
            let octets = try!(parser.parse_octetstring());
            // TODO: format check
            return Ok(UtcTime::new(octets));
        })
    }
}

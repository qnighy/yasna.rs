// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod error;

#[cfg(feature = "bigint")]
use num::bigint::BigInt;

use super::{Tag,TagClass,TagType};
use super::{TAG_BOOLEAN,TAG_INTEGER,TAG_BITSTRING,TAG_OCTETSTRING,TAG_NULL,TAG_OID,TAG_SEQUENCE,TAG_SET};
use super::{ObjectIdentifier,BitString};
use super::FromBER;
pub use self::error::*;

pub fn parse_ber_general<'a, T, F>(buf: &'a [u8], mode: BERMode, callback: F)
        -> ASN1Result<T>
        where F: for<'b> FnOnce(&mut BERReader<'a, 'b>) -> ASN1Result<T> {
    let mut reader_impl = BERReaderImpl::new(buf, mode);
    let result;
    {
        let mut reader = BERReader {
            inner: &mut reader_impl,
        };
        result = try!(callback(&mut reader));
    }
    try!(reader_impl.end_of_buf());
    return Ok(result);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BERMode {
    Ber, Der,
}

#[derive(Debug)]
enum TagState {
    None,
    Cached(Tag, PC),
    Implicit(PC, Option<usize>),
}

#[derive(Debug)]
struct BERReaderImpl<'a> {
    buf: &'a [u8],
    pos: usize,
    mode: BERMode,
    tag_state: TagState,
    depth: usize,
}

#[derive(Debug)]
pub struct BERReader<'a, 'b> where 'a: 'b {
    inner: &'b mut BERReaderImpl<'a>,
}

const BER_READER_STACK_DEPTH : usize = 100;

impl<'a> BERReaderImpl<'a> {
    fn new(buf: &'a [u8], mode: BERMode) -> Self {
        return BERReaderImpl {
            buf: buf,
            pos: 0,
            mode: mode,
            tag_state: TagState::None,
            depth: 0,
        };
    }

    fn generate_error(&self, kind: ASN1ErrorKind) -> ASN1Error {
        ASN1Error::new(kind)
    }

    fn read_u8(&mut self) -> ASN1Result<u8> {
        if self.pos < self.buf.len() {
            let ret = self.buf[self.pos];
            self.pos += 1;
            return Ok(ret);
        } else {
            return Err(self.generate_error(ASN1ErrorKind::Eof));
        }
    }

    fn remaining_buffer(&self) -> &'a [u8] {
        return &self.buf[self.pos..];
    }

    fn fetch_remaining_buffer(&mut self) -> &'a [u8] {
        let ret = &self.buf[self.pos..];
        self.pos = self.buf.len();
        return ret;
    }

    fn end_of_buf(&mut self) -> ASN1Result<()> {
        if self.pos != self.buf.len() {
            return Err(self.generate_error(ASN1ErrorKind::Extra));
        }
        return Ok(());
    }

    fn end_of_contents(&mut self) -> ASN1Result<()> {
        let (tag, pc) = match self.tag_state {
            TagState::None => try!(self.parse_identifier()),
            TagState::Cached(tag, pc) => (tag, pc),
            TagState::Implicit(_, _) => {
                return Err(self.generate_error(ASN1ErrorKind::Invalid));
            },
        };
        if tag != TAG_EOC || pc != PC::Primitive {
            return Err(self.generate_error(ASN1ErrorKind::Invalid));
        }
        let b = try!(self.read_u8());
        if b != 0 {
            return Err(self.generate_error(ASN1ErrorKind::Invalid));
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
                        self.generate_error(ASN1ErrorKind::IntegerOverflow)));
                tag_number = x + (b & 127);
                if (b & 128) == 0 {
                    break;
                }
            }
            if tag_number < 31 {
                return Err(self.generate_error(ASN1ErrorKind::Invalid));
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
            return Err(self.generate_error(ASN1ErrorKind::Invalid));
        }
        if (lbyte & 128) == 0 {
            return Ok(Some(lbyte));
        }
        let mut length : usize = 0;
        for _ in 0..(lbyte & 127) {
            let x = try!(length.checked_mul(256).ok_or(
                self.generate_error(ASN1ErrorKind::Eof)));
            length = x + (try!(self.read_u8()) as usize);
        }
        if self.mode == BERMode::Der && length < 128 {
            return Err(self.generate_error(ASN1ErrorKind::Invalid));
        }
        return Ok(Some(length));
    }

    fn parse_general<T, F>
            (&mut self, tag: Tag, tag_type: TagType, callback: F)
            -> ASN1Result<T>
            where F: FnOnce(&mut Self, PC) -> ASN1Result<T> {
        if self.depth > BER_READER_STACK_DEPTH {
            return Err(self.generate_error(ASN1ErrorKind::StackOverflow));
        }
        let pc;
        let length_spec;
        match self.tag_state {
            TagState::None => {
                let (tag2, pc2) = try!(self.parse_identifier());
                if tag2 != tag {
                    self.tag_state = TagState::Cached(tag2, pc2);
                    return Err(self.generate_error(ASN1ErrorKind::Invalid));
                }
                pc = pc2;
                length_spec = try!(self.parse_length());
            },
            TagState::Cached(tag2, pc2) => {
                if tag2 != tag {
                    return Err(self.generate_error(ASN1ErrorKind::Invalid));
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
                let limit = self.pos+length;
                if old_buf.len() < limit {
                    return Err(self.generate_error(ASN1ErrorKind::Eof));
                }
                self.buf = &old_buf[..limit];
            },
            None => {
                if pc != PC::Constructed {
                    return Err(self.generate_error(ASN1ErrorKind::Invalid));
                }
                if self.mode == BERMode::Der {
                    return Err(self.generate_error(ASN1ErrorKind::Invalid));
                }
            },
        };
        self.depth += 1;
        let result = try!(callback(self, pc));
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

    fn parse_with_buffer<'b, T, F>(&'b mut self, callback: F)
            -> ASN1Result<(T, &'a [u8])>
            where F: FnOnce(&mut Self) -> ASN1Result<T> {
        let old_pos = self.pos;
        let result = try!(callback(self));
        let new_pos = self.pos;
        let buf = &self.buf[old_pos..new_pos];
        return Ok((result, buf));
    }

    fn parse_optional<T, F>(&mut self, callback: F) -> ASN1Result<Option<T>>
            where F: FnOnce(&mut Self) -> ASN1Result<T> {
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
        match callback(self) {
            Ok(result) => Ok(Some(result)),
            Err(e) =>
                if old_pos == self.pos {
                    Ok(None)
                } else {
                    Err(e)
                },
        }
    }

    fn parse_octetstring_impl(&mut self, vec: &mut Vec<u8>) -> ASN1Result<()> {
        self.parse_general(TAG_OCTETSTRING, TagType::Explicit, |inner, pc| {
            if pc == PC::Constructed {
                if inner.mode == BERMode::Der {
                    return Err(inner.generate_error(ASN1ErrorKind::Invalid));
                }
                loop {
                    let result = try!(inner.parse_optional(|inner| {
                        inner.parse_octetstring_impl(vec)
                    }));
                    match result {
                        Some(()) => {},
                        None => { break; },
                    }
                }
                return Ok(());
            } else {
                vec.extend(inner.fetch_remaining_buffer());
                return Ok(());
            }
        })
    }
}

impl<'a, 'b> BERReader<'a, 'b> {
    pub fn mode(&self) -> BERMode {
        self.inner.mode
    }

    pub fn generate_error(&self, kind: ASN1ErrorKind) -> ASN1Error {
        self.inner.generate_error(kind)
    }

    pub fn read_bool(&mut self) -> ASN1Result<bool> {
        self.inner.parse_general(TAG_BOOLEAN, TagType::Explicit, |inner, pc| {
            if pc != PC::Primitive {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let buf = inner.fetch_remaining_buffer();
            if buf.len() != 1 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let b = buf[0];
            if inner.mode == BERMode::Der && b != 0 && b != 255 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            return Ok(b != 0);
        })
    }

    pub fn read_i64(&mut self) -> ASN1Result<i64> {
        self.inner.parse_general(TAG_INTEGER, TagType::Explicit, |inner, pc| {
            if pc != PC::Primitive {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let buf = inner.fetch_remaining_buffer();
            if buf.len() == 0 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            } else if buf.len() == 1 {
                return Ok(buf[0] as i8 as i64);
            }
            let mut x = ((buf[0] as i8 as i64) << 8) + (buf[1] as i64);
            if -128 <= x && x < 128 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            if buf.len() > 8 {
                return Err(inner.generate_error(
                    ASN1ErrorKind::IntegerOverflow));
            }
            for &b in buf[2..].iter() {
                x = (x << 8) | (b as i64);
            }
            return Ok(x);
        })
    }

    #[cfg(feature = "bigint")]
    pub fn read_bigint(&mut self) -> ASN1Result<BigInt> {
        self.inner.parse_general(TAG_INTEGER, TagType::Explicit, |inner, pc| {
            if pc != PC::Primitive {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let buf = inner.fetch_remaining_buffer();
            if buf.len() == 0 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            } else if buf.len() == 1 {
                return Ok(BigInt::from(buf[0] as i8));
            }
            let mut x = (BigInt::from(buf[0] as i8) << 8) +
                BigInt::from(buf[1] as i64);
            if BigInt::from(-128) <= x && x < BigInt::from(128) {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            for &b in buf[2..].iter() {
                x = (x << 8) + BigInt::from(b);
            }
            return Ok(x);
        })
    }

    pub fn parse_bitstring(&mut self) -> ASN1Result<BitString> {
        self.inner.parse_general(TAG_BITSTRING, TagType::Explicit,
                                 |inner, pc| {
            if pc == PC::Constructed {
                // TODO: implement recursive encoding
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            } else {
                // TODO: Canonicity check in DER
                let buf = inner.fetch_remaining_buffer();
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

    pub fn parse_octetstring(&mut self) -> ASN1Result<Vec<u8>> {
        let mut ret = Vec::new();
        try!(self.inner.parse_octetstring_impl(&mut ret));
        return Ok(ret);
    }

    pub fn read_null(&mut self) -> ASN1Result<()> {
        self.inner.parse_general(TAG_NULL, TagType::Explicit, |inner, pc| {
            if pc != PC::Primitive {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let buf = inner.fetch_remaining_buffer();
            if buf.len() != 0 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            return Ok(());
        })
    }

    pub fn read_oid(&mut self) -> ASN1Result<ObjectIdentifier> {
        self.inner.parse_general(TAG_OID, TagType::Explicit, |inner, pc| {
            if pc != PC::Primitive {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let mut ids = Vec::new();
            let buf = inner.fetch_remaining_buffer();
            if buf.len() == 0 || buf[buf.len()-1] >= 128 {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            let mut subid : u64 = 0;
            for &b in buf.iter() {
                if b == 128 {
                    return Err(inner.generate_error(ASN1ErrorKind::Invalid));
                }
                subid = try!(subid.checked_mul(128)
                    .ok_or(inner.generate_error(
                        ASN1ErrorKind::IntegerOverflow))) + ((b & 127) as u64);
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
            return Ok(ObjectIdentifier::new(ids));
        })
    }

    pub fn parse_optional<T, F>(&mut self, callback: F)
            -> ASN1Result<Option<T>>
            where F: for<'c> FnOnce(&mut BERReader<'a, 'c>) -> ASN1Result<T> {
        self.inner.parse_optional(|inner| {
            callback(&mut BERReader { inner: inner, })
        })
    }
    pub fn parse_default<T, F>(&mut self, default: T, callback: F)
            -> ASN1Result<T>
            where F: for<'c> FnOnce(&mut BERReader<'a, 'c>) -> ASN1Result<T>,
            T: Eq {
        match try!(self.parse_optional(callback)) {
            Some(result) => {
                if self.inner.mode == BERMode::Der && result == default {
                    return Err(self.generate_error(ASN1ErrorKind::Invalid));
                }
                return Ok(result);
            },
            None => Ok(default),
        }
    }
    pub fn parse_with_buffer<T, F>(&mut self, callback: F)
            -> ASN1Result<(T, &'a [u8])>
            where F: for<'c> FnOnce(&mut BERReader<'a, 'c>) -> ASN1Result<T> {
        self.inner.parse_with_buffer(|inner| {
            callback(&mut BERReader { inner: inner, })
        })
    }

    pub fn parse_tagged<T, F>
            (&mut self, tag: Tag, tag_type: TagType, callback: F)
            -> ASN1Result<T>
            where F: for<'c> FnOnce(&mut BERReader<'a, 'c>) -> ASN1Result<T> {
        self.inner.parse_general(tag, tag_type, |inner, pc| {
            if tag_type == TagType::Explicit && pc != PC::Constructed {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            callback(&mut BERReader { inner: inner, })
        })
    }

    pub fn parse_sequence<T, F>(&mut self, callback: F) -> ASN1Result<T>
            where F: for<'c> FnOnce(&mut BERReader<'a, 'c>) -> ASN1Result<T> {
        self.inner.parse_general(TAG_SEQUENCE, TagType::Explicit, |inner, pc| {
            if pc != PC::Constructed {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            return callback(&mut BERReader { inner: inner, });
        })
    }

    pub fn parse_set<T, F>(&mut self, callback: F) -> ASN1Result<T>
            where F: for<'c> FnOnce(&mut BERReader<'a, 'c>) -> ASN1Result<T> {
        self.inner.parse_general(TAG_SET, TagType::Explicit, |inner, pc| {
            if pc != PC::Constructed {
                return Err(inner.generate_error(ASN1ErrorKind::Invalid));
            }
            return callback(&mut BERReader { inner: inner, });
        })
    }

    pub fn parse<T:FromBER>(&mut self) -> ASN1Result<T> {
        T::from_ber(self)
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

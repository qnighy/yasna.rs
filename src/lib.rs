// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A library for reading and writing ASN.1 data.
//!
//! # Example
//!
//! To serialize ASN.1 data, you can use [`construct_der`][construct_der].
//!
//! [construct_der]: fn.construct_der.html
//!
//! ```
//! extern crate yasna;
//!
//! fn main() {
//!     let der = yasna::construct_der(|writer| {
//!         writer.write_sequence(|writer| {
//!             writer.next().write_i64(10);
//!             writer.next().write_bool(true);
//!         })
//!     });
//!     println!("(10, true) = {:?}", der);
//! }
//! ```
//!
//! To deserialize ASN.1 data, you can use [`parse_ber`][parse_ber]
//! or [`parse_der`][parse_der].
//!
//! [parse_ber]: fn.parse_ber.html
//! [parse_der]: fn.parse_der.html
//!
//! ```
//! extern crate yasna;
//!
//! fn main() {
//!     let asn = yasna::parse_der(&[48, 6, 2, 1, 10, 1, 1, 255], |reader| {
//!         reader.read_sequence(|reader| {
//!             let i = try!(reader.next().read_i64());
//!             let b = try!(reader.next().read_bool());
//!             return Ok((i, b));
//!         })
//!     }).unwrap();
//!     println!("{:?} = [48, 6, 2, 1, 10, 1, 1, 255]", asn);
//! }
//! ```

#[cfg(feature = "bigint")]
extern crate num;

mod basics;
mod writer;
mod reader;
mod deserializer;

pub use basics::{BitString,SetOf,ObjectIdentifier,PrintableString,UtcTime};

pub use writer::{construct_der,construct_der_seq,DERWriterSeq,DERWriter,DERWriterSet};
pub use reader::{BERReader,BERReaderSeq,ASN1Error,ASN1ErrorKind,BERMode,ASN1Result,parse_ber_general,parse_ber,parse_der};
pub use deserializer::FromBER;

/// An ASN.1 tag class, used in [`Tag`][tag].
///
/// [tag]: struct.Tag.html
///
/// A tag class is one of:
///
/// - UNIVERSAL
/// - APPLICATION
/// - context specific
/// - PRIVATE
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TagClass {
    Universal = 0, Application = 1, ContextSpecific = 2, Private = 3,
}

/// An ASN.1 tag.
///
/// An ASN.1 tag is a pair of a tag class and a tag number.
///
/// - A tag class is one of:
///   - UNIVERSAL
///   - APPLICATION
///   - context specific
///   - PRIVATE
/// - A tag number is a nonnegative integer.
///   In this library. Tag numbers are assumed to be in `u64`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tag {
    pub tag_class: TagClass,
    pub tag_number: u64,
}

impl Tag {
    /// Constructs an APPLICATION tag, namely \[APPLICATION n\].
    pub fn application(tag_number: u64) -> Tag {
        return Tag {
            tag_class: TagClass::Application,
            tag_number: tag_number,
        }
    }
    /// Constructs a context specific tag, namely \[n\].
    pub fn context(tag_number: u64) -> Tag {
        return Tag {
            tag_class: TagClass::ContextSpecific,
            tag_number: tag_number,
        }
    }
    /// Constructs a PRIVATE tag, namely \[PRIVATE n\].
    pub fn private(tag_number: u64) -> Tag {
        return Tag {
            tag_class: TagClass::Private,
            tag_number: tag_number,
        }
    }
}

const TAG_BOOLEAN : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 1,
};

const TAG_INTEGER : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 2,
};

const TAG_BITSTRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 3,
};

const TAG_OCTETSTRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 4,
};

const TAG_NULL : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 5,
};

const TAG_OID : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 6,
};

const TAG_SEQUENCE : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 16,
};

const TAG_SET : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 17,
};

const TAG_PRINTABLESTRING : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 19,
};

const TAG_UTCTIME : Tag = Tag {
    tag_class: TagClass::Universal,
    tag_number: 23,
};

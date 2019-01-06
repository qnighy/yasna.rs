// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A library for reading and writing ASN.1 data.
//!
//! # Examples
//!
//! ## Encoding/decoding simple data
//!
//! A type implementing [`DEREncodable`][derencodable] can be easily encoded:
//!
//! [derencodable]: trait.DEREncodable.html
//!
//! ```
//! extern crate yasna;
//!
//! fn main() {
//!     let der = yasna::encode_der(&(10, true));
//!     println!("(10, true) = {:?}", der);
//! }
//! ```
//!
//! Similarly, a type implementing [`BERDecodable`][berdecodable] can be
//! easily decoded:
//!
//! [berdecodable]: trait.BERDecodable.html
//!
//! ```
//! extern crate yasna;
//!
//! fn main() {
//!     let asn: (i64, bool) = yasna::decode_der(
//!         &[48, 6, 2, 1, 10, 1, 1, 255]).unwrap();
//!     println!("{:?} = [48, 6, 2, 1, 10, 1, 1, 255]", asn);
//! }
//! ```
//!
//! ## Encoding/decoding by hand
//!
//! Default `DEREncodable`/`BERDecodable` implementations can't handle
//! all ASN.1 type. In many cases you have to write your reader/writer
//! by hand.
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

#[cfg(feature = "num-bigint")]
extern crate num_bigint;
#[cfg(test)]
extern crate num_traits;

#[cfg(feature = "bit-vec")]
extern crate bit_vec;
#[cfg(feature = "chrono")]
extern crate chrono;

pub mod tags;
pub mod models;
mod writer;
mod reader;
mod deserializer;
mod serializer;

pub use writer::{construct_der,construct_der_seq};
pub use writer::{DERWriter,DERWriterSeq,DERWriterSet};
pub use reader::{parse_ber_general,parse_ber,parse_der,BERMode};
pub use reader::{BERReader,BERReaderSeq,BERReaderSet};
pub use reader::{ASN1Error,ASN1ErrorKind,ASN1Result};
pub use deserializer::{BERDecodable,decode_ber_general,decode_ber,decode_der};
pub use serializer::{DEREncodable,encode_der};

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

const TAG_CLASSES : [TagClass; 4] = [
    TagClass::Universal,
    TagClass::Application,
    TagClass::ContextSpecific,
    TagClass::Private,
];

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

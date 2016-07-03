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

pub use basics::*;

pub use writer::{construct_der,construct_der_seq,DERWriterSeq,DERWriter,DERWriterSet};
pub use reader::{BERReader,BERReaderSeq,ASN1Error,ASN1ErrorKind,BERMode,ASN1Result,parse_ber_general,parse_ber,parse_der};
pub use deserializer::FromBER;

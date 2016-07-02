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
//! ```
//! extern crate yasna;
//!
//! fn main() {
//!     let der = yasna::construct_der(|writer| {
//!         writer.write_i64(10)
//!     }).unwrap();
//!     println!("10 = {:?}", der);
//! }
//! ```

#[cfg(feature = "bigint")]
extern crate num;

mod basics;
mod writer;
pub mod ber;

pub use basics::*;

pub use writer::{construct_der,DERWriter};

// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(missing_docs)]

#[cfg(feature = "num-bigint")]
use num_bigint::{BigInt,BigUint};
#[cfg(feature = "bit-vec")]
use bit_vec::BitVec;

use super::{ASN1Result,BERMode,BERReader,parse_ber_general};
use super::models::{ObjectIdentifier,TaggedDerValue};
#[cfg(feature = "chrono")]
use super::models::{UTCTime,GeneralizedTime};

/// Types decodable in BER.
///
/// # Examples
///
/// ```
/// use yasna;
/// let asn : i64 = yasna::decode_der(&[2, 3, 0, 255, 255]).unwrap();
/// assert_eq!(asn, 65535);
/// ```
///
/// # Limitations
///
/// Rust types don't correspond to ASN.1 types one-to-one. Not all kinds
/// of ASN.1 types can be decoded via default `BERDecodable` implementation.
///
/// If you want to decode ASN.1, you may implement `BERDecodable` for your
/// own types or use [`parse_der`][parse_der]/[`parse_ber`][parse_ber].
///
/// [parse_der]: fn.parse_der.html
/// [parse_ber]: fn.parse_ber.html
///
/// # Default implementations
///
/// - The decoder for `Vec<T>` is implemented as SEQUENCE OF decoder.
/// - `()` as NULL decoder.
/// - Tuples (except `()`) as SEQUENCE decoder.
/// - `Vec<u8>` as OCTETSTRING decoder.
/// - `BitVec` as BITSTRING decoder.
/// - `String` as UTF8String decoder.
/// - `i64`, `u64`, `i32`, `u32`, `i16`, `u16`, `BigInt`, `BigUint`
///   as INTEGER decoder. (`u8` is avoided because of confliction.)
/// - `bool` as BOOLEAN decoder.
/// - `ObjectIdentifier` as OBJECTT IDENTIFIER decoder.
/// - `UTCTime`/`GeneralizedTime` as UTCTime/GeneralizedTime decoder.
pub trait BERDecodable: Sized {
    /// Reads an ASN.1 value from `BERReader` and converts it to `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use yasna::{BERDecodable,BERReader,ASN1Result};
    /// struct Entry {
    ///     name: String,
    ///     age: i64,
    /// }
    ///
    /// impl BERDecodable for Entry {
    ///     fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
    ///         reader.read_sequence(|reader| {
    ///             let name = try!(reader.next().read_visible_string());
    ///             let age = try!(reader.next().read_i64());
    ///             return Ok(Entry {
    ///                 name: name,
    ///                 age: age,
    ///             });
    ///         })
    ///     }
    /// }
    /// fn main() {
    ///     let entry : Entry = yasna::decode_der(
    ///         &[48, 9, 26, 4, 74, 111, 104, 110, 2, 1, 32]).unwrap();
    ///     assert_eq!(entry.name, "John");
    ///     assert_eq!(entry.age, 32);
    /// }
    /// ```
    fn decode_ber<'a, 'b>(reader: BERReader<'a, 'b>) -> ASN1Result<Self>;
}

/// Decodes DER/BER-encoded data.
///
/// [`decode_ber`][decode_ber] and [`decode_der`][decode_der] are shorthands
/// for this function.
///
/// [decode_ber]: fn.decode_ber.html
/// [decode_der]: fn.decode_der.html
pub fn decode_ber_general<T:BERDecodable>(src: &[u8], mode: BERMode)
        -> ASN1Result<T> {
    parse_ber_general(src, mode, |reader| {
        T::decode_ber(reader)
    })
}

/// Reads an ASN.1 value from `&[u8]`.
///
/// If you want to accept only DER-encoded data,
/// use [`decode_der`][decode_der].
///
/// [decode_der]: fn.decode_der.html
///
/// # Examples
///
/// ```
/// use yasna;
/// let asn : i64 = yasna::decode_ber(&[2, 3, 0, 255, 255]).unwrap();
/// assert_eq!(asn, 65535);
/// ```
pub fn decode_ber<T:BERDecodable>(src: &[u8]) -> ASN1Result<T> {
    decode_ber_general(src, BERMode::Ber)
}

/// Reads an ASN.1 value from `&[u8]`.
///
/// If you want to decode BER-encoded data in general,
/// use [`decode_ber`][decode_ber].
///
/// [decode_ber]: fn.decode_ber.html
///
/// # Examples
///
/// ```
/// use yasna;
/// let asn : i64 = yasna::decode_der(&[2, 3, 0, 255, 255]).unwrap();
/// assert_eq!(asn, 65535);
/// ```
pub fn decode_der<T:BERDecodable>(src: &[u8]) -> ASN1Result<T> {
    decode_ber_general(src, BERMode::Der)
}

impl<T> BERDecodable for Vec<T> where T: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let mut ret = Vec::new();
            loop {
                let result = try!(reader.read_optional(|reader| {
                    T::decode_ber(reader)
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

impl BERDecodable for i64 {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_i64()
    }
}

impl BERDecodable for u64 {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_u64()
    }
}

impl BERDecodable for i32 {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_i32()
    }
}

impl BERDecodable for u32 {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_u32()
    }
}

impl BERDecodable for i16 {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_i16()
    }
}

impl BERDecodable for u16 {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_u16()
    }
}

#[cfg(feature = "num-bigint")]
impl BERDecodable for BigInt {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_bigint()
    }
}

#[cfg(feature = "num-bigint")]
impl BERDecodable for BigUint {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_biguint()
    }
}

impl BERDecodable for bool {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_bool()
    }
}

#[cfg(feature = "bit-vec")]
impl BERDecodable for BitVec {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_bitvec()
    }
}

impl BERDecodable for Vec<u8> {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_bytes()
    }
}

impl BERDecodable for String {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_utf8string()
    }
}

impl BERDecodable for ObjectIdentifier {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_oid()
    }
}

#[cfg(feature = "chrono")]
impl BERDecodable for UTCTime {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_utctime()
    }
}

#[cfg(feature = "chrono")]
impl BERDecodable for GeneralizedTime {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_generalized_time()
    }
}

impl BERDecodable for () {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_null()
    }
}

impl<T0> BERDecodable for (T0,)
        where T0: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            return Ok((t0,));
        })
    }
}

impl<T0, T1> BERDecodable for (T0, T1)
        where T0: BERDecodable, T1: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            return Ok((t0, t1));
        })
    }
}

impl<T0, T1, T2> BERDecodable for (T0, T1, T2)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            return Ok((t0, t1, t2));
        })
    }
}

impl<T0, T1, T2, T3> BERDecodable for (T0, T1, T2, T3)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3));
        })
    }
}

impl<T0, T1, T2, T3, T4> BERDecodable for (T0, T1, T2, T3, T4)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5> BERDecodable for (T0, T1, T2, T3, T4, T5)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> BERDecodable for (T0, T1, T2, T3, T4, T5, T6)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable,
            T6: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            let t6 = try!(T6::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5, t6));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> BERDecodable
        for (T0, T1, T2, T3, T4, T5, T6, T7)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable,
            T6: BERDecodable, T7: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            let t6 = try!(T6::decode_ber(reader.next()));
            let t7 = try!(T7::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5, t6, t7));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> BERDecodable
        for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable,
            T6: BERDecodable, T7: BERDecodable, T8: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            let t6 = try!(T6::decode_ber(reader.next()));
            let t7 = try!(T7::decode_ber(reader.next()));
            let t8 = try!(T8::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5, t6, t7, t8));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> BERDecodable
        for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable,
            T6: BERDecodable, T7: BERDecodable, T8: BERDecodable,
            T9: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            let t6 = try!(T6::decode_ber(reader.next()));
            let t7 = try!(T7::decode_ber(reader.next()));
            let t8 = try!(T8::decode_ber(reader.next()));
            let t9 = try!(T9::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5, t6, t7, t8, t9));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> BERDecodable
        for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable,
            T6: BERDecodable, T7: BERDecodable, T8: BERDecodable,
            T9: BERDecodable, T10: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            let t6 = try!(T6::decode_ber(reader.next()));
            let t7 = try!(T7::decode_ber(reader.next()));
            let t8 = try!(T8::decode_ber(reader.next()));
            let t9 = try!(T9::decode_ber(reader.next()));
            let t10 = try!(T10::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10));
        })
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> BERDecodable
        for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
        where T0: BERDecodable, T1: BERDecodable, T2: BERDecodable,
            T3: BERDecodable, T4: BERDecodable, T5: BERDecodable,
            T6: BERDecodable, T7: BERDecodable, T8: BERDecodable,
            T9: BERDecodable, T10: BERDecodable, T11: BERDecodable {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let t0 = try!(T0::decode_ber(reader.next()));
            let t1 = try!(T1::decode_ber(reader.next()));
            let t2 = try!(T2::decode_ber(reader.next()));
            let t3 = try!(T3::decode_ber(reader.next()));
            let t4 = try!(T4::decode_ber(reader.next()));
            let t5 = try!(T5::decode_ber(reader.next()));
            let t6 = try!(T6::decode_ber(reader.next()));
            let t7 = try!(T7::decode_ber(reader.next()));
            let t8 = try!(T8::decode_ber(reader.next()));
            let t9 = try!(T9::decode_ber(reader.next()));
            let t10 = try!(T10::decode_ber(reader.next()));
            let t11 = try!(T11::decode_ber(reader.next()));
            return Ok((t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11));
        })
    }
}

impl BERDecodable for TaggedDerValue  {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_tagged_der()
    }
}

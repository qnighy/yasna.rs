// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "bigint")]
use num::bigint::{BigUint, BigInt};

use super::*;

#[test]
fn test_write_bool() {
    let data = construct_der_seq(|writer| {
        for &val in [false, true].iter() {
            try!(writer.write_bool(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![1, 1, 0, 1, 1, 255]);
}

#[test]
fn test_write_i64() {
    let data = construct_der_seq(|writer| {
        for &val in [
            -9223372036854775808, -65537, -65536, -32769, -32768, -129, -128,
            -1, 0, 1, 127, 128, 32767, 32768, 65535, 65536,
            9223372036854775807].iter() {
            try!(writer.write_i64(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 8, 128, 0, 0, 0, 0, 0, 0, 0, 2, 3, 254, 255, 255, 2, 3, 255, 0, 0,
        2, 3, 255, 127, 255, 2, 2, 128, 0, 2, 2, 255, 127, 2, 1, 128,
        2, 1, 255, 2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255, 2, 3, 1, 0, 0,
        2, 8, 127, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn test_write_u64() {
    let data = construct_der_seq(|writer| {
        for &val in [
            0, 1, 127, 128, 32767, 32768, 65535, 65536,
            9223372036854775807, 18446744073709551615].iter() {
            try!(writer.write_u64(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255, 2, 3, 1, 0, 0,
        2, 8, 127, 255, 255, 255, 255, 255, 255, 255,
        2, 9, 0, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn test_write_i32() {
    let data = construct_der_seq(|writer| {
        for &val in [
            -2147483648, -65537, -65536, -32769, -32768, -129, -128, -1, 0, 1,
            127, 128, 32767, 32768, 65535, 65536, 2147483647].iter() {
            try!(writer.write_i32(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 4, 128, 0, 0, 0, 2, 3, 254, 255, 255, 2, 3, 255, 0, 0,
        2, 3, 255, 127, 255, 2, 2, 128, 0, 2, 2, 255, 127, 2, 1, 128,
        2, 1, 255, 2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255, 2, 3, 1, 0, 0,
        2, 4, 127, 255, 255, 255]);
}

#[test]
fn test_write_u32() {
    let data = construct_der_seq(|writer| {
        for &val in [
            0, 1, 127, 128, 32767, 32768, 65535, 65536, 2147483647,
            4294967295].iter() {
            try!(writer.write_u32(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255, 2, 3, 1, 0, 0,
        2, 4, 127, 255, 255, 255, 2, 5, 0, 255, 255, 255, 255]);
}

#[test]
fn test_write_i16() {
    let data = construct_der_seq(|writer| {
        for &val in [-32768, -129, -128, -1, 0, 1, 127, 128, 32767].iter() {
            try!(writer.write_i16(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 2, 128, 0, 2, 2, 255, 127, 2, 1, 128, 2, 1, 255, 2, 1, 0, 2, 1, 1,
        2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255]);
}

#[test]
fn test_write_u16() {
    let data = construct_der_seq(|writer| {
        for &val in [0, 1, 127, 128, 32767, 32768, 65535].iter() {
            try!(writer.write_u16(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255]);
}

#[test]
fn test_write_i8() {
    let data = construct_der_seq(|writer| {
        for &val in [-128, -1, 0, 1, 127].iter() {
            try!(writer.write_i8(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![2, 1, 128, 2, 1, 255, 2, 1, 0, 2, 1, 1, 2, 1, 127]);
}

#[test]
fn test_write_u8() {
    let data = construct_der_seq(|writer| {
        for &val in [0, 1, 127, 255].iter() {
            try!(writer.write_u8(val));
        }
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 255]);
}

#[cfg(feature = "bigint")]
#[test]
fn test_write_bigint() {
    use num::FromPrimitive;
    let data = construct_der_seq(|writer| {
        for &val in [
            -9223372036854775808, -65537, -65536, -32769, -32768, -129, -128,
            -1, 0, 1, 127, 128, 32767, 32768, 65535, 65536,
            9223372036854775807, ].iter() {
            try!(writer.write_bigint(&BigInt::from_i64(val).unwrap()));
        }
        try!(writer.write_bigint(&BigInt::parse_bytes(
            b"1234567890123456789012345678901234567890", 10).unwrap()));
        try!(writer.write_bigint(&BigInt::parse_bytes(
            b"-1234567890123456789012345678901234567890", 10).unwrap()));
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 8, 128, 0, 0, 0, 0, 0, 0, 0, 2, 3, 254, 255, 255, 2, 3, 255, 0, 0,
        2, 3, 255, 127, 255, 2, 2, 128, 0, 2, 2, 255, 127, 2, 1, 128,
        2, 1, 255, 2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255, 2, 3, 1, 0, 0,
        2, 8, 127, 255, 255, 255, 255, 255, 255, 255,
        2, 17, 3, 160, 201, 32, 117, 192, 219,
        243, 184, 172, 188, 95, 150, 206, 63, 10, 210,
        2, 17, 252, 95, 54, 223, 138, 63, 36,
        12, 71, 83, 67, 160, 105, 49, 192, 245, 46]);
}

#[cfg(feature = "bigint")]
#[test]
fn test_write_biguint() {
    use num::FromPrimitive;
    let data = construct_der_seq(|writer| {
        for &val in [
            0, 1, 127, 128, 32767, 32768, 65535, 65536,
            9223372036854775807, 18446744073709551615].iter() {
            try!(writer.write_biguint(&BigUint::from_u64(val).unwrap()));
        }
        try!(writer.write_biguint(&BigUint::parse_bytes(
            b"1234567890123456789012345678901234567890", 10).unwrap()));
        return Ok(());
    }).unwrap();
    assert_eq!(data, vec![
        2, 1, 0, 2, 1, 1, 2, 1, 127, 2, 2, 0, 128, 2, 2, 127, 255,
        2, 3, 0, 128, 0, 2, 3, 0, 255, 255, 2, 3, 1, 0, 0,
        2, 8, 127, 255, 255, 255, 255, 255, 255, 255,
        2, 9, 0, 255, 255, 255, 255, 255, 255, 255, 255,
        2, 17, 3, 160, 201, 32, 117, 192, 219,
        243, 184, 172, 188, 95, 150, 206, 63, 10, 210]);
}

#[test]
fn test_write_bytes() {
    let data = construct_der_seq(|writer| {
        writer.write_bytes(&[1, 0, 100, 255])
    }).unwrap();
    assert_eq!(data, vec![4, 4, 1, 0, 100, 255]);
}

#[test]
fn test_write_null() {
    let data = construct_der_seq(|writer| {
        writer.write_null()
    }).unwrap();
    assert_eq!(data, vec![5, 0]);
}

#[test]
fn test_write_sequence_small() {
    let data = construct_der_seq(|writer| {
        writer.write_sequence(|_| {
            return Ok(());
        })
    }).unwrap();
    assert_eq!(data, vec![48, 0]);

    let data = construct_der_seq(|writer| {
        writer.write_sequence(|writer| {
            try!(writer.write_bytes(&vec![91; 20]));
            return Ok(());
        })
    }).unwrap();
    assert_eq!(data, vec![
        48, 22, 4, 20, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
        91, 91, 91, 91, 91, 91]);

    let data = construct_der_seq(|writer| {
        writer.write_sequence(|writer| {
            try!(writer.write_bytes(&vec![91; 200]));
            return Ok(());
        })
    }).unwrap();
    assert_eq!(data[0..9].to_vec(),
        vec![48, 129, 203, 4, 129, 200, 91, 91, 91]);
    assert_eq!(data.len(), 206);

    let data = construct_der_seq(|writer| {
        writer.write_sequence(|writer| {
            try!(writer.write_bytes(&vec![91; 2000]));
            return Ok(());
        })
    }).unwrap();
    assert_eq!(data[0..11].to_vec(),
        vec![48, 130, 7, 212, 4, 130, 7, 208, 91, 91, 91]);
    assert_eq!(data.len(), 2008);
}

#[test]
fn test_write_sequence_medium() {
    let data = construct_der_seq(|writer| {
        writer.write_sequence(|writer| {
            try!(writer.write_bytes(&vec![91; 200000]));
            return Ok(());
        })
    }).unwrap();
    assert_eq!(data[0..13].to_vec(),
        vec![48, 131, 3, 13, 69, 4, 131, 3, 13, 64, 91, 91, 91]);
    assert_eq!(data.len(), 200010);
}

#[test]
#[ignore]
fn test_write_sequence_large() {
    let data = construct_der_seq(|writer| {
        writer.write_sequence(|writer| {
            try!(writer.write_bytes(&vec![91; 20000000]));
            return Ok(());
        })
    }).unwrap();
    assert_eq!(data[0..15].to_vec(),
        vec![48, 132, 1, 49, 45, 6, 4, 132, 1, 49, 45, 0, 91, 91, 91]);
    assert_eq!(data.len(), 20000012);
}

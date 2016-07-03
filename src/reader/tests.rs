// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;

#[test]
fn test_der_read_bool_ok() {
    let tests : &[(bool, &[u8])] = &[
        (false, &[1, 1, 0]),
        (true, &[1, 1, 255]),
    ];
    for &(evalue, data) in tests {
        let value = parse_der(data, |reader| {
            reader.read_bool()
        }).unwrap();
        assert_eq!(value, evalue);
    }
}

#[test]
fn test_der_read_bool_err() {
    let tests : &[&[u8]] = &[
        &[], &[1], &[0, 0], &[0, 1, 0], &[2, 1, 0], &[33, 1, 0], &[65, 1, 0],
        &[1, 0], &[1, 2, 0, 0], &[1, 128, 1, 1, 0, 0, 0],
        &[1, 1, 1], &[1, 1, 191], &[1, 1, 254],
    ];
    for &data in tests {
        parse_der(data, |reader| {
            reader.read_bool()
        }).unwrap_err();
    }
}

#[test]
fn test_ber_read_bool_ok() {
    let tests : &[(bool, &[u8])] = &[
        (false, &[1, 1, 0]),
        (true, &[1, 1, 1]),
        (true, &[1, 1, 191]),
        (true, &[1, 1, 254]),
        (true, &[1, 1, 255]),
    ];
    for &(evalue, data) in tests {
        let value = parse_ber(data, |reader| {
            reader.read_bool()
        }).unwrap();
        assert_eq!(value, evalue);
    }
}

#[test]
fn test_ber_read_bool_err() {
    let tests : &[&[u8]] = &[
        &[], &[1], &[0, 0], &[0, 1, 0], &[2, 1, 0], &[33, 1, 0], &[65, 1, 0],
        &[1, 0], &[1, 2, 0, 0], &[1, 128, 1, 1, 0, 0, 0],
    ];
    for &data in tests {
        parse_ber(data, |reader| {
            reader.read_bool()
        }).unwrap_err();
    }
}

#[test]
fn test_der_read_i64_ok() {
    let tests : &[(i64, &[u8])] = &[
        (-9223372036854775808, &[2, 8, 128, 0, 0, 0, 0, 0, 0, 0]),
        (-65537, &[2, 3, 254, 255, 255]),
        (-65536, &[2, 3, 255, 0, 0]),
        (-32769, &[2, 3, 255, 127, 255]),
        (-32768, &[2, 2, 128, 0]),
        (-129, &[2, 2, 255, 127]),
        (-128, &[2, 1, 128]),
        (-1, &[2, 1, 255]),
        (0, &[2, 1, 0]),
        (1, &[2, 1, 1]),
        (127, &[2, 1, 127]),
        (128, &[2, 2, 0, 128]),
        (32767, &[2, 2, 127, 255]),
        (32768, &[2, 3, 0, 128, 0]),
        (65535, &[2, 3, 0, 255, 255]),
        (65536, &[2, 3, 1, 0, 0]),
        (9223372036854775807, &[2, 8, 127, 255, 255, 255, 255, 255, 255, 255]),
    ];
    for &(evalue, data) in tests {
        let value = parse_der(data, |reader| {
            reader.read_i64()
        }).unwrap();
        assert_eq!(value, evalue);
    }
}

#[test]
fn test_der_read_i64_err() {
    let tests : &[&[u8]] = &[
        &[], &[2], &[0, 0], &[0, 1, 0], &[1, 1, 0], &[34, 1, 0], &[66, 1, 0],
        &[2, 0], &[2, 128, 2, 1, 0, 0, 0], &[2, 2, 0], &[2, 1, 1, 1],
        &[2, 2, 255, 128], &[2, 2, 255, 200], &[2, 2, 0, 127], &[2, 2, 0, 56],
        &[2, 3, 255, 151, 55], &[2, 3, 0, 1, 2],
    ];
    for &data in tests {
        parse_der(data, |reader| {
            reader.read_i64()
        }).unwrap_err();
    }
}

#[test]
fn test_ber_read_i64_ok() {
    let tests : &[(i64, &[u8])] = &[
        (-9223372036854775808, &[2, 8, 128, 0, 0, 0, 0, 0, 0, 0]),
        (-65537, &[2, 3, 254, 255, 255]),
        (-65536, &[2, 3, 255, 0, 0]),
        (-32769, &[2, 3, 255, 127, 255]),
        (-32768, &[2, 2, 128, 0]),
        (-129, &[2, 2, 255, 127]),
        (-128, &[2, 1, 128]),
        (-1, &[2, 1, 255]),
        (0, &[2, 1, 0]),
        (1, &[2, 1, 1]),
        (127, &[2, 1, 127]),
        (128, &[2, 2, 0, 128]),
        (32767, &[2, 2, 127, 255]),
        (32768, &[2, 3, 0, 128, 0]),
        (65535, &[2, 3, 0, 255, 255]),
        (65536, &[2, 3, 1, 0, 0]),
        (9223372036854775807, &[2, 8, 127, 255, 255, 255, 255, 255, 255, 255]),
    ];
    for &(evalue, data) in tests {
        let value = parse_ber(data, |reader| {
            reader.read_i64()
        }).unwrap();
        assert_eq!(value, evalue);
    }
}

#[test]
fn test_ber_read_i64_err() {
    let tests : &[&[u8]] = &[
        &[], &[2], &[0, 0], &[0, 1, 0], &[1, 1, 0], &[34, 1, 0], &[66, 1, 0],
        &[2, 0], &[2, 128, 2, 1, 0, 0, 0], &[2, 2, 0], &[2, 1, 1, 1],
        &[2, 2, 255, 128], &[2, 2, 255, 200], &[2, 2, 0, 127], &[2, 2, 0, 56],
        &[2, 3, 255, 151, 55], &[2, 3, 0, 1, 2],
    ];
    for &data in tests {
        parse_ber(data, |reader| {
            reader.read_i64()
        }).unwrap_err();
    }
}

#[test]
fn test_der_read_bytes_ok() {
    let tests : &[(&[u8], &[u8])] = &[
        (&[1, 0, 100, 255], &[4, 4, 1, 0, 100, 255]),
        (&[], &[4, 0]),
        (&[4, 4, 4, 4], &[4, 4, 4, 4, 4, 4]),
    ];
    for &(evalue, data) in tests {
        let value = parse_der(data, |reader| {
            reader.read_bytes()
        }).unwrap();
        assert_eq!(value, evalue);
    }
}

#[test]
fn test_der_read_bytes_err() {
    let tests : &[&[u8]] = &[
        &[], &[4], &[0, 0], &[0, 1, 0], &[1, 1, 0], &[36, 1, 0], &[68, 1, 0],
        &[4, 4, 0], &[4, 1, 1, 1], &[36, 128, 1, 0, 0],
        &[36, 128, 0, 0],
        &[36, 128, 4, 2, 12, 34, 0, 0],
        &[36, 128, 36, 128, 4, 3, 12, 34, 56, 0, 0, 0, 0],
        &[36, 128, 36, 128, 36, 128, 36, 128, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        &[36, 128, 4, 1, 2, 36, 128, 4, 2, 3, 1, 0, 0, 0, 0],
        &[36, 0],
        &[36, 4, 4, 2, 12, 34],
        &[36, 128, 36, 5, 4, 3, 12 ,34, 56, 0, 0],
        &[36, 9, 36, 128, 4, 3, 12, 34, 56, 0, 0],
        &[36, 7, 36, 5, 4, 3, 12 ,34, 56],
    ];
    for &data in tests {
        parse_der(data, |reader| {
            reader.read_bytes()
        }).unwrap_err();
    }
}

#[test]
fn test_ber_read_bytes_ok() {
    let tests : &[(&[u8], &[u8])] = &[
        (&[1, 0, 100, 255], &[4, 4, 1, 0, 100, 255]),
        (&[], &[4, 0]),
        (&[4, 4, 4, 4], &[4, 4, 4, 4, 4, 4]),
        (&[], &[36, 128, 0, 0]),
        (&[12, 34], &[36, 128, 4, 2, 12, 34, 0, 0]),
        (&[12, 34, 56], &[36, 128, 36, 128, 4, 3, 12, 34, 56, 0, 0, 0, 0]),
        (&[], &[36, 128, 36, 128, 36, 128, 36,
             128, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (&[2, 3, 1], &[36, 128, 4, 1, 2, 36, 128, 4, 2, 3, 1, 0, 0, 0, 0]),
        (&[], &[36, 0]),
        (&[12, 34], &[36, 4, 4, 2, 12, 34]),
        (&[12, 34, 56], &[36, 128, 36, 5, 4, 3, 12 ,34, 56, 0, 0]),
        (&[12, 34, 56], &[36, 9, 36, 128, 4, 3, 12, 34, 56, 0, 0]),
        (&[12, 34, 56], &[36, 7, 36, 5, 4, 3, 12 ,34, 56]),
    ];
    for &(evalue, data) in tests {
        println!("{:?}", data);
        let value = parse_ber(data, |reader| {
            reader.read_bytes()
        }).unwrap();
        assert_eq!(value, evalue);
    }
}

#[test]
fn test_ber_read_bytes_err() {
    let tests : &[&[u8]] = &[
        &[], &[4], &[0, 0], &[0, 1, 0], &[1, 1, 0], &[36, 1, 0], &[68, 1, 0],
        &[4, 4, 0], &[4, 1, 1, 1], &[4, 128, 1, 0, 0],
    ];
    for &data in tests {
        println!("{:?}", data);
        parse_ber(data, |reader| {
            reader.read_bytes()
        }).unwrap_err();
    }
}

#[test]
fn test_der_read_null_ok() {
    let value = parse_der(&[5, 0], |reader| {
        reader.read_null()
    }).unwrap();
    assert_eq!(value, ());
}

#[test]
fn test_der_read_null_err() {
    let tests : &[&[u8]] = &[
        &[], &[5], &[0, 0], &[0, 1, 0], &[2, 1, 0], &[37, 0], &[69, 0],
        &[5, 128, 0], &[37, 128, 0], &[5, 1, 0], &[5, 2, 0, 0],
        &[5, 1], &[5, 0, 1],
    ];
    for &data in tests {
        parse_der(data, |reader| {
            reader.read_null()
        }).unwrap_err();
    }
}

#[test]
fn test_ber_read_null_ok() {
    let value = parse_ber(&[5, 0], |reader| {
        reader.read_null()
    }).unwrap();
    assert_eq!(value, ());
}

#[test]
fn test_ber_read_null_err() {
    let tests : &[&[u8]] = &[
        &[], &[5], &[0, 0], &[0, 1, 0], &[2, 1, 0], &[37, 0], &[69, 0],
        &[5, 128, 0], &[37, 128, 0], &[5, 1, 0], &[5, 2, 0, 0],
        &[5, 1], &[5, 0, 1],
    ];
    for &data in tests {
        parse_ber(data, |reader| {
            reader.read_null()
        }).unwrap_err();
    }
}

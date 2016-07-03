# yasna.rs: ASN.1 library for Rust

[![Build Status](https://travis-ci.org/qnighy/yasna.rs.svg?branch=master)](https://travis-ci.org/qnighy/yasna.rs)

This is a Rust library for reading and writing ASN.1 data.

- [crate.io/crates/yasna](https://crates.io/crates/yasna)
- [Documentation](https://qnighy.github.io/yasna.rs/yasna/index.html)

Since this library is at an early stage, the APIs are subject to great change.

## Serialization/Construction

Serialization in DER (Distinguished Encoding Rules) is supported. It can also be used for serialization in BER (Basic Encoding Rules).

Currently supported datatypes are limited.

```rust
extern crate yasna;

fn main() {
    let der = yasna::construct_der(|writer| {
        writer.write_sequence(|writer| {
            writer.next().write_i64(10);
            writer.next().write_bool(true);
            return Ok(());
        })
    });
    println!("(10, true) = {:?}", der);
}
```

## Deserialization/Parsing

Deserialization in BER (Basic Encoding Rules) or DER (Distinguished Encoding Rules) is supported.

```rust
extern crate yasna;

fn main() {
    let asn = yasna::parse_der(&[48, 6, 2, 1, 10, 1, 1, 255], |reader| {
        reader.read_sequence(|reader| {
            let i = try!(reader.next().read_i64());
            let b = try!(reader.next().read_bool());
            return Ok((i, b));
        })
    }).unwrap();
    println!("{:?} = [48, 6, 2, 1, 10, 1, 1, 255]", asn);
}
```

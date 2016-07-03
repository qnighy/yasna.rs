# yasna.rs: ASN.1 library for Rust

[![Build Status](https://travis-ci.org/qnighy/yasna.rs.svg?branch=master)](https://travis-ci.org/qnighy/yasna.rs)

This is a Rust library for reading and writing ASN.1 data.

- [crate.io/crates/yasna](https://crates.io/crates/yasna)
- [Documentation](https://qnighy.github.io/yasna.rs/yasna/index.html)

## Serialization

Serialization in DER (Distinguished Encoding Rules) is supported. It can also be used for serialization in BER (Basic Encoding Rules).

Currently supported datatypes are very limited.

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

## Deserialization

Deserialization in BER/CER/DER is supported.

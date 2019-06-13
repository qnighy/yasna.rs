# 0.3.1
- Support for reading/writing BITSTRING without needing the bit-vec crate
- Addition of try_{construct_der,construct_der_seq}

# 0.3.0
- Increase MSRV to 1.17.0
- Update to bit-vec 0.6.1
- Support for reading/writing IA5String, BMPString

# 0.2.1

- Ability to read/write raw DER
- Addition of `#![deny(missing_docs)]` to the library root
- Ability to read/write ENUMERATED

# 0.2.0

- Support for more datatypes (R/W):
  - UTF8String, NumericString, PrintableString, VisibleString,
  - UTCTime, GeneralizedTime,
- Completed support for these datatypes:
  - SEQUENCE OF, DEFAULT, OPTIONAL,
- Addition of traits BERDecodable and DEREncodable
- Reduction of dependencies
- Updates of dependencies (num to 0.2, bit-vec to 0.5)
- Features were renamed and turned off by default
- Addition of [@est31](http://github.com/est31) as maintainer

# 0.1.3

- Both readers and writers implement common core datatypes, including:
  - BOOLEAN, INTEGER, BITSTRING, OCTETSTRING, NULL, OBJECT IDENTIFIER,
  - SEQUENCE, SET, SET OF,
  - Explicitly/Implicitly tagged types.
- Public APIs of readers and writers are documented and most of them are tested. These seem almost frozen.
- Now it depends on "bit-vec" crate to handle bit strings.
- ASN.1 data models are now in `asn1::models`. These are waiting for refactoring and API change, as well as `FromBer` traits.

# 0.1.2

Both writers and readers are refactored.

- Writing SET values and tagged values.
- Now `DERWriter` and `DERWriterSeq` are different.
- Readers are currently in thorough refactoring.
  - They are split into several files internally.
  - `BERReader` and `BERReaderSeq` are now different.
  - Several changes on naming.
  - The CER (Canonical Encoding Rules) support is dropped.
  - Some tests are added.

# 0.1.1

From this version, writing DER is supported with very limited functionality.

- Documented writer functionality with support for:
  - BOOLEAN
  - INTEGER
  - OCTET STRING
  - NULL
  - SEQUENCE

# 0.1.0

The project is splitted from [the author's early work on TLS](https://github.com/qnighy/crypt-impl-rust/tree/f8b2758bfc757a80d47b15a3210bc0d62dd8f1cf/src/misc/asn1).

- Undocumented reader functionality with support for:
  - BOOLEAN
  - INTEGER
  - BIT STRING
  - OCTET STRING
  - NULL
  - SEQUENCE
  - SEQUENCE OF
  - SET OF
  - Tagged values
  - Object identifiers
- Deserialization functionality, mainly for reading X.509 certificates.

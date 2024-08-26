searchState.loadedDescShard("yasna", 0, "A library for reading and writing ASN.1 data.\nThe APPLICATION tag class\nTypes decodable in BER.\nUsed by <code>BERReader</code> to determine whether or not to enforce …\nA reader object for BER/DER-encoded ASN.1 data.\nA reader object for a sequence of BER/DER-encoded ASN.1 …\nA reader object for a set of BER/DER-encoded ASN.1 data.\nUse BER (Basic Encoding Rules).\nThe bit’s value is “Constructed”\nThe CONTEXT-SPECIFIC tag class\nTypes encodable in DER.\nA writer object that accepts an ASN.1 value.\nA writer object that accepts ASN.1 values.\nA writer object that accepts ASN.1 values.\nUse DER (Distinguished Encoding Rules).\nContains the error value\nContains the success value\nA value of the ASN.1 primitive/constructed (“P/C”) bit.\nThe bit’s value is “Primitive”\nThe PRIVATE tag class\nAn ASN.1 tag.\nAn ASN.1 tag class, used in <code>Tag</code>.\nThe UNIVERSAL tag class\nConstructs an APPLICATION tag, namely [APPLICATION n].\nCollects an ASN.1 SEQUENCE OF value.\nCollects an ASN.1 SET OF value.\nConstructs DER-encoded data as <code>Vec&lt;u8&gt;</code>.\nConstructs DER-encoded sequence of data as <code>Vec&lt;u8&gt;</code>.\nConstructs a context specific tag, namely [n].\nReads an ASN.1 value from <code>&amp;[u8]</code>.\nReads an ASN.1 value from <code>BERReader</code> and converts it to <code>Self</code>…\nDecodes DER/BER-encoded data.\nReads an ASN.1 value from <code>&amp;[u8]</code>.\nEncodes a value to DER-encoded ASN.1 data.\nWrites the value as an DER-encoded ASN.1 value.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLookaheads the tag in the next value. Used to parse CHOICE …\nTells which format we are parsing, BER or DER.\nTells which format we are parsing, BER or DER.\nTells which format we are parsing, BER or DER.\nProvides datatypes which correspond to ASN.1 types.\nGenerates a new <code>DERWriter</code>.\nGenerates a new <code>DERWriter</code>.\nGenerates a new <code>BERReader</code>.\nGenerates a new <code>BERReader</code>.\nParses BER-encoded data.\nParses DER/BER-encoded data.\nParses DER-encoded data.\nConstructs a PRIVATE tag, namely [PRIVATE n].\nReads an ASN.1 INTEGER value as <code>BigInt</code>.\nReads an ASN.1 INTEGER value as <code>Vec&lt;u8&gt;</code> and a sign bit.\nReads an ASN.1 INTEGER value as <code>BigUint</code>.\nReads an ASN.1 BITSTRING value as <code>BitVec</code>.\nReads an ASN.1 BITSTRING value as <code>(Vec&lt;u8&gt;, usize)</code>.\nReads an ASN.1 BMPString.\nReads an ASN.1 BOOLEAN value as <code>bool</code>.\nReads an ASN.1 OCTETSTRING value as <code>Vec&lt;u8&gt;</code>.\nSimilar to <code>read_optional</code>, but uses <code>default</code> if it fails.\nSimilar to <code>read_optional</code>, but uses <code>default</code> if it fails.\nReads a DER object as raw bytes. Tag and length are …\nReads an ASN.1 ENUMERATED value as <code>i64</code>.\nReads an ASN.1 GeneralizedTime.\nReads an ASN.1 INTEGER value as <code>i16</code>.\nReads an ASN.1 INTEGER value as <code>i32</code>.\nReads an ASN.1 INTEGER value as <code>i64</code>.\nReads an ASN.1 INTEGER value as <code>i8</code>.\nReads an ASN.1 IA5String.\nReads the ASN.1 NULL value.\nReads an ASN.1 NumericString.\nReads an ASN.1 object identifier.\nTries to read an ASN.1 value. If it fails at the first tag,\nIf there is a set element with a tag in <code>tag_hint</code>, reads an …\nReads an ASN.1 PrintableString.\nReads an ASN.1 SEQUENCE value.\nReads an ASN.1 SEQUENCE OF value.\nReads an ASN.1 SET value.\nReads an ASN.1 SET OF value.\nReads a (explicitly) tagged value.\nRead an arbitrary (tag, value) pair as a TaggedDerValue. …\nReads an implicitly tagged value.\nReads an ASN.1 INTEGER value as <code>u16</code>.\nReads an ASN.1 INTEGER value as <code>u32</code>.\nReads an ASN.1 INTEGER value as <code>u64</code>.\nReads an ASN.1 INTEGER value as <code>u8</code>.\nReads an ASN.1 UTCTime.\nReads an ASN.1 UTF8String.\nReads an ASN.1 VisibleString.\nThe tag class\nThe tag number\nProvides universal tag constants.\nTries to construct DER-encoded data as <code>Vec&lt;u8&gt;</code>.\nTries to construct a DER-encoded sequence of data as …\nWrites <code>BigInt</code> as an ASN.1 INTEGER value.\nWrites <code>&amp;[u8]</code> and <code>bool</code> as an ASN.1 INTEGER value.\nWrites <code>BigUint</code> as an ASN.1 INTEGER value.\nWrites <code>BitVec</code> as an ASN.1 BITSTRING value.\nWrites <code>&amp;[u8]</code> and <code>usize</code> as an ASN.1 BITSTRING value.\nWrites <code>&amp;str</code> as an ASN.1 BMPString value.\nWrites <code>bool</code> as an ASN.1 BOOLEAN value.\nWrites <code>&amp;[u8]</code> as an ASN.1 OCTETSTRING value.\nWrites <code>&amp;[u8]</code> into the DER output buffer directly. Properly …\nWrites <code>i64</code> as an ASN.1 ENUMERATED value.\nWrites an ASN.1 GeneralizedTime.\nWrites <code>i16</code> as an ASN.1 INTEGER value.\nWrites <code>i32</code> as an ASN.1 INTEGER value.\nWrites <code>i64</code> as an ASN.1 INTEGER value.\nWrites <code>i8</code> as an ASN.1 INTEGER value.\nWrites <code>&amp;str</code> as an ASN.1 IA5String value.\nWrites the ASN.1 NULL value.\nWrites an ASN.1 NumericString.\nWrites an ASN.1 object identifier.\nWrites an ASN.1 PrintableString.\nWrites ASN.1 SEQUENCE.\nWrites ASN.1 SEQUENCE OF.\nWrites ASN.1 SET.\nWrites ASN.1 SET OF.\nWrites an (explicitly) tagged value.\nWrites the arbitrary tagged DER value in <code>der</code>.\nWrites an implicitly tagged value.\nWrites <code>u16</code> as an ASN.1 INTEGER value.\nWrites <code>u32</code> as an ASN.1 INTEGER value.\nWrites <code>u64</code> as an ASN.1 INTEGER value.\nWrites <code>u8</code> as an ASN.1 INTEGER value.\nWrites an ASN.1 UTCTime.\nWrites <code>&amp;str</code> as an ASN.1 UTF8String value.\nWrites an ASN.1 UTF8String.\nWrites an ASN.1 VisibleString.\nDate and time between 0000-01-01T00:00:00Z and …\nA type that represents object identifiers.\nAn error indicating failure to parse an Object identifier\nContainer for a tag and arbitrary DER value.\nDate and time between 1950-01-01T00:00:00Z and …\nIf the value is something that contains raw bytes, returns …\nIf the value is something string-like, returns it as …\nBorrows its internal vector of components.\nMutably borrows its internal vector of components.\nReturns the <code>OffsetDateTime</code> it represents.\nReturns the <code>OffsetDateTime</code> it represents.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs <code>UTCTime</code> from an <code>OffsetDateTime</code>.\nConstructs <code>GeneralizedTime</code> from an <code>OffsetDateTime</code>.\nConstructs <code>GeneralizedTime</code> from an <code>OffsetDateTime</code> and …\nConstructs <code>GeneralizedTime</code> from an <code>OffsetDateTime</code> and …\nConstructs <code>UTCTime</code> from an <code>OffsetDateTime</code>.\nConstructs <code>GeneralizedTime</code> from an <code>OffsetDateTime</code>.\nConstructs a new <code>TaggedDerValue</code> as an octet string\nConstructs a new <code>ObjectIdentifier</code> from <code>&amp;[u64]</code>.\nConstructs a new <code>TaggedDerValue</code> from its tag and content\nConstructs a new <code>TaggedDerValue</code> from its tag, …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExtracts its internal vector of components.\nConstructs a new <code>ObjectIdentifier</code> from <code>Vec&lt;u64&gt;</code>.\nParses ASN.1 string representation of UTCTime.\nParses ASN.1 string representation of GeneralizedTime.\nParses ASN.1 string representation of GeneralizedTime, …\nReturns the primitive/constructed bit\nReturns sub-nanoseconds digits of the datetime.\nReturns the tag\nReturns ASN.1 canonical representation of the datetime as …\nReturns ASN.1 canonical representation of the datetime as …\nReturns ASN.1 canonical representation of the datetime as …\nReturns ASN.1 canonical representation of the datetime as …\nReturns the value\nA universal tag for BITSTRING.\nA universal tag for BMPString.\nA universal tag for BOOLEAN.\nA universal tag for DATE.\nA universal tag for DATE-TIME.\nA universal tag for DURATION.\nA universal tag for embedded-pdv types.\nA universal tag for enumerated types.\nA special tag representing “end of contents”.\nA universal tag for external/instance-of types.\nA universal tag for GeneralizedTime.\nA universal tag for GeneralString.\nA universal tag for GraphicString.\nA universal tag for IA5String.\nA universal tag for INTEGER.\nA universal tag for NULL.\nA universal tag for NumericString.\nA universal tag for object descriptors.\nA universal tag for OCTETSTRING.\nA universal tag for object identifiers.\nA universal tag for OID internationalized resource …\nA universal tag for PrintableString.\nA universal tag for REAL.\nA universal tag for relative object identifiers.\nA universal tag for relative OID internationalized …\nA universal tag for SEQUENCE/SEQUENCE OF.\nA universal tag for SET/SET OF.\nA universal tag for TeletexString.\nA universal tag for TIME.\nA universal tag for TIME-OF-DAY.\nA universal tag for UniversalString.\nA universal tag for UTCTime.\nA universal tag for UTF8String.\nA universal tag for VideotexString.\nA universal tag for VisibleString.")
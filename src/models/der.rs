// Copyright 2017 Fortanix, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::super::{PCBit, Tag};
use super::super::tags::{TAG_OCTETSTRING, TAG_SEQUENCE, TAG_SET};

/// Container for a tag and arbitrary DER value.
///
/// When obtained by `BERReader::read_tagged_der` in DER mode,
/// the reader verifies that the payload is actually valid DER.
/// When constructed from bytes, the caller is responsible for
/// providing valid DER.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TaggedDerValue {
    tag: Tag,
    pcbit: PCBit,
    value: Vec<u8>,
}

impl TaggedDerValue {
    pub fn from_octetstring(bytes: Vec<u8>) -> Self {
        TaggedDerValue {
            tag: TAG_OCTETSTRING,
            pcbit: PCBit::Primitive,
            value: bytes,
        }
    }

    pub fn from_tag_and_bytes(tag: Tag, bytes: Vec<u8>) -> Self {
        let pcbit = match tag {
            TAG_SEQUENCE | TAG_SET => PCBit::Constructed,
            _ => PCBit::Primitive,
        };
        TaggedDerValue {
            tag: tag,
            pcbit: pcbit,
            value: bytes,
        }
    }

    pub fn from_tag_pc_and_bytes(tag: Tag, pcbit: PCBit, bytes: Vec<u8>) -> Self {
        TaggedDerValue {
            tag: tag,
            pcbit: pcbit,
            value: bytes,
        }
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn pcbit(&self) -> PCBit {
        self.pcbit
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

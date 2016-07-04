// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BitString {
    pub unused_bits: usize,
    pub buf: Vec<u8>,
}

impl BitString {
    pub fn new() -> Self {
        return BitString {
            unused_bits: 0,
            buf: Vec::new(),
        };
    }
    pub fn from_buf(unused_bits : usize, buf: Vec<u8>) -> Self {
        return BitString {
            unused_bits: unused_bits,
            buf: buf,
        };
    }
    pub fn push(&mut self, b: bool) {
        if self.unused_bits == 0 {
            self.buf.push(0);
            self.unused_bits = 8;
        }
        let last = self.buf.last_mut().unwrap();
        self.unused_bits -= 1;
        *last = *last | ((b as u8) << self.unused_bits);
    }
}


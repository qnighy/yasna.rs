// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BitString {
    unused_bits: usize,
    bytes: Vec<u8>,
}

impl BitString {
    pub fn new() -> Self {
        BitString {
            unused_bits: 0,
            bytes: Vec::new(),
        }
    }
    pub fn from_bytes(unused_bits: usize, mut bytes: Vec<u8>) -> Self {
        assert!(unused_bits < 8, "unused_bits = {:?} too large", unused_bits);
        if bytes.len() == 0 {
            assert!(unused_bits == 0,
                "unused_bits = {:?} positive, but bytes empty", unused_bits);
        } else {
            let len = bytes.len();
            bytes[len-1] &= !((1 << unused_bits) - 1);
        }
        return BitString {
            unused_bits: unused_bits,
            bytes: bytes,
        };
    }
    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }
    pub fn len(&self) -> usize {
        self.bytes.len() - self.unused_bits
    }
    pub fn unused_bits(&self) -> usize {
        self.unused_bits
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn full_bytes(&self) -> &[u8] {
        if self.unused_bits == 0 {
            &self.bytes
        } else {
            let len = self.bytes.len();
            &self.bytes[..len-1]
        }
    }
    pub fn full_bytes_mut(&mut self) -> &mut [u8] {
        if self.unused_bits == 0 {
            &mut self.bytes
        } else {
            let len = self.bytes.len();
            &mut self.bytes[..len-1]
        }
    }
    pub fn fill_unused_bits(&mut self) {
        self.unused_bits = 0;
    }
    pub fn remove_trailing_bits(&mut self) {
        if self.unused_bits > 0 {
            self.bytes.pop().unwrap();
            self.unused_bits = 0;
        }
    }
    pub fn extend_from_bitstring(&mut self, other: &BitString) {
        if self.unused_bits == 0 {
            self.bytes.extend_from_slice(&other.bytes);
            self.unused_bits = other.unused_bits;
        } else {
            let mut last_byte = self.bytes.pop().unwrap();
            for &b in &other.bytes {
                let x = ((last_byte as usize) << 8)
                    | ((b as usize) << self.unused_bits);
                self.bytes.push((x >> 8) as u8);
                last_byte = x as u8;
            }
            let unused_bits = self.unused_bits + other.unused_bits;
            if unused_bits >= 8 {
                self.unused_bits = unused_bits - 8;
            } else {
                self.unused_bits = unused_bits;
                self.bytes.push(last_byte);
            }
        }
    }
    pub fn extend_from_bytes
        (&mut self, other_unused_bits: usize, other_bytes: &[u8]) {
        assert!(other_unused_bits < 8,
            "other_unused_bits = {:?} too large", other_unused_bits);
        if other_bytes.len() == 0 {
            assert!(other_unused_bits == 0,
                "other_unused_bits = {:?} positive, but bytes empty",
                other_unused_bits);
        }
        if self.unused_bits == 0 {
            self.bytes.extend_from_slice(&other_bytes);
            self.unused_bits = other_unused_bits;
        } else {
            let mut last_byte = self.bytes.pop().unwrap();
            for &b in other_bytes {
                let x = ((last_byte as usize) << 8)
                    | ((b as usize) << self.unused_bits);
                self.bytes.push((x >> 8) as u8);
                last_byte = x as u8;
            }
            let unused_bits = self.unused_bits + other_unused_bits;
            if unused_bits >= 8 {
                self.unused_bits = unused_bits - 8;
            } else {
                self.unused_bits = unused_bits;
                self.bytes.push(last_byte);
            }
        }
        if self.bytes.len() > 0 {
            let len = self.bytes.len();
            self.bytes[len-1] &= !((1 << self.unused_bits) - 1);
        }
    }
}


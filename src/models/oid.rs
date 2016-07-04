// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ObjectIdentifier {
    ids: Vec<u64>,
}

use std::slice::Iter;
impl ObjectIdentifier {
    pub fn new(ids: Vec<u64>) -> Self {
        return ObjectIdentifier {
            ids: ids,
        };
    }
    pub fn iter(&self) -> Iter<u64> {
        self.ids.iter()
    }
}

impl Deref for ObjectIdentifier {
    type Target = [u64];
    fn deref(&self) -> &Self::Target {
        return &self.ids;
    }
}

impl DerefMut for ObjectIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.ids;
    }
}

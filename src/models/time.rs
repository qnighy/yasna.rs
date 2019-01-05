// Copyright 2016 Masaki Hara
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UtcTime {
    bytes: Vec<u8>,
}

impl UtcTime {
    pub fn new(bytes: Vec<u8>) -> Self {
        return UtcTime {
            bytes: bytes,
        };
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GeneralizedTimeKind {
    /// Local time
    Local,
    /// Local time with offset specified
    LocalWithOffset {
        /// true if the offset is positive, false otherwise
        is_positive: bool,
        /// Hours (0-24)
        hour: u16,
        /// Minutes (0-59)
        minute: u16,
    },
    /// Utc time
    Utc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An ASN.1 GeneralizedTime
pub struct GeneralizedTime {
    pub kind: GeneralizedTimeKind,
    /// Year (0-9999)
    pub year: u16,
    /// Month (1-12)
    pub month: u16,
    /// Day (1-31)
    pub day: u16,
    /// Hour (0-11)
    pub hour: u16,
    /// Minute (0-59)
    pub minute: Option<u16>,
    /// Second (0-60)
    ///
    /// Only allowed to be Some if `minute` is Some.
    pub second: Option<u16>,
    /// Milisecond (0-999)
    ///
    /// Only allowed to be Some if `minute` and `second` are Some.
    pub milisecond: Option<u16>,
}

impl GeneralizedTime {
    /// Helper constructor
    pub fn from_utc_ymd(year: u16, month: u16, day: u16) -> Self {
        GeneralizedTime {
            kind: GeneralizedTimeKind::Utc,
            year,
            month,
            day,
            hour: 0,
            minute: None,
            second: None,
            milisecond: None,
        }
    }
    /// Whether the given GeneralizedTime can be represented
    pub fn is_valid(&self) -> bool {
        if self.year > 9999 {
            return false;
        }
        if self.month < 1 || self.month > 12 {
            return false;
        }
        if self.day < 1 || self.day > 31 {
            return false;
        }
        if self.hour > 11 {
            return false;
        }
        if let Some(minute) = self.minute {
            if minute > 59 {
                return false;
            }
            if let Some(second) = self.second {
                if second > 60 {
                    return false;
                }
                if let Some(milisecond) = self.milisecond {
                    if milisecond > 999 {
                        return false;
                    }
                }
            } else if self.milisecond.is_some() {
                return false;
            }
        } else if self.second.is_some() || self.milisecond.is_some() {
            return false;
        }

        return true;
    }
}

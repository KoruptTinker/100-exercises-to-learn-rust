// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(n: u16) -> SaturatingU16 {
        return SaturatingU16 { value: n };
    }
}

impl From<u8> for SaturatingU16 {
    fn from(n: u8) -> SaturatingU16 {
        return SaturatingU16 {
            value: u16::from(n),
        };
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(n: &u16) -> SaturatingU16 {
        return SaturatingU16 {
            value: u16::from(*n),
        };
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(n: &u8) -> Self {
        return SaturatingU16 {
            value: u16::from(*n),
        };
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, n: SaturatingU16) -> Self::Output {
        return SaturatingU16 {
            value: self.value.saturating_add(n.value),
        };
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, n: u16) -> Self::Output {
        return SaturatingU16 {
            value: self.value.saturating_add(n),
        };
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, n: &SaturatingU16) -> Self::Output {
        return SaturatingU16 {
            value: self.value.saturating_add(n.value),
        };
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, n: &u16) -> Self::Output {
        return SaturatingU16 {
            value: self.value.saturating_add(*n),
        };
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        return &self.value == other;
    }
}

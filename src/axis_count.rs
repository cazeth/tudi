use std::num::NonZeroU32;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AxisCount(u32);

impl AxisCount {
    pub const MAX: Self = Self(u32::MAX);
    pub const MIN: Self = Self(u32::MIN);

    pub fn as_u64(&self) -> u64 {
        u64::from(self.0) + 1
    }

    pub(crate) fn from_len(length: u32) -> Self {
        debug_assert!(length < u32::MAX);
        Self(length)
    }

    #[cfg(test)]
    pub(crate) fn from_u64_unchecked(count: u64) -> Self {
        assert!(count > 0 && count <= u32::MAX as u64 + 1);
        let value: u32 = u32::try_from(count - 1).unwrap();
        Self(value)
    }
}

impl std::fmt::Debug for AxisCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u64().fmt(f)
    }
}

impl PartialEq<u64> for AxisCount {
    fn eq(&self, other: &u64) -> bool {
        self.as_u64() == *other
    }
}

impl PartialOrd<u64> for AxisCount {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        Some(self.as_u64().cmp(other))
    }
}

impl From<NonZeroU32> for AxisCount {
    fn from(value: NonZeroU32) -> Self {
        let value: u32 = value.into();
        Self(value - 1)
    }
}

macro_rules! try_from_unsigned {
    ($($number_type:ty),+ $(,)?) => {
        $(
            impl TryFrom<$number_type> for AxisCount {
                type Error = AxisCountError;

                fn try_from(count: $number_type) -> Result<Self, Self::Error> {
                    let count = count as u64;
                    if count == 0 {
                        Err(AxisCountError::Zero)
                    } else if count > u32::MAX as u64 + 1 {
                        Err(AxisCountError::TooLarge(count))
                    } else if count == u32::MAX as u64 + 1 {
                        Ok(AxisCount::MAX)
                    } else {
                        Ok(AxisCount(count as u32 - 1))
                    }
                }
            }
        )+
    };
}

macro_rules! try_from_signed {
    ($($number_type:ty),+ $(,)?) => {
        $(
            impl TryFrom<$number_type> for AxisCount {
                type Error = AxisCountError;

                fn try_from(count: $number_type) -> Result<Self, Self::Error> {
                    if count < 0 {
                        return Err(AxisCountError::Negative(count as i64))
                    } else if count as u64 > u32::MAX as u64 + 1 {
                        return Err(AxisCountError::TooLarge(count as u64))
                    } else {
                        AxisCount::try_from(count as u64)
                    }
                }
            }
        )+
    };
}

try_from_unsigned!(u8, u16, u32, usize, u64);
try_from_signed!(i8, i16, i32, isize, i64);

impl std::fmt::Display for AxisCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == u32::MAX {
            write!(f, "{}", u32::MAX as u64 + 1)
        } else {
            write!(f, "{}", self.0 + 1)
        }
    }
}

impl TryFrom<AxisCount> for u32 {
    type Error = AxisCountError;
    fn try_from(value: AxisCount) -> Result<Self, Self::Error> {
        if value.0 == u32::MAX {
            Err(AxisCountError::Zero)
        } else {
            Ok(value.0 + 1)
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
#[non_exhaustive]
#[error("Axis Count Error")]
pub enum AxisCountError {
    #[error("Tried to create an axis count that was too large {0}")]
    TooLarge(u64),
    #[error("Tried to create a zero axis count")]
    Zero,
    #[error("Tried to create an axis count from a negative number {0}")]
    Negative(i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroes_err() {
        assert_eq!(AxisCount::try_from(0_u8), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_u16), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_u32), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_usize), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_u64), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_i8), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_i16), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_i32), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_isize), Err(AxisCountError::Zero));
        assert_eq!(AxisCount::try_from(0_i64), Err(AxisCountError::Zero));
    }

    #[test]
    fn large_u64_errs() {
        let value: u64 = u32::MAX as u64 + 2;
        assert_eq!(
            AxisCount::try_from(value),
            Err(AxisCountError::TooLarge(value))
        );
    }

    #[test]
    fn axis_count_max_is_ok() {
        assert_eq!(AxisCount::try_from(u32::MAX as u64 + 1), Ok(AxisCount::MAX));
    }

    #[test]
    fn i64_max_errs() {
        assert_eq!(
            AxisCount::try_from(i64::MAX),
            Err(AxisCountError::TooLarge(i64::MAX as u64))
        )
    }

    #[test]
    fn negative_errs() {
        assert_eq!(
            AxisCount::try_from(-1_i8),
            Err(AxisCountError::Negative(-1))
        );
        assert_eq!(
            AxisCount::try_from(-1_i16),
            Err(AxisCountError::Negative(-1))
        );
        assert_eq!(
            AxisCount::try_from(-1_i32),
            Err(AxisCountError::Negative(-1))
        );
        assert_eq!(
            AxisCount::try_from(-1_isize),
            Err(AxisCountError::Negative(-1))
        );
        assert_eq!(
            AxisCount::try_from(-1_i64),
            Err(AxisCountError::Negative(-1))
        );
    }
}

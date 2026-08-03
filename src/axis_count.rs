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
}

impl std::fmt::Debug for AxisCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_u64().fmt(f)
    }
}

impl From<NonZeroU32> for AxisCount {
    fn from(value: NonZeroU32) -> Self {
        let value: u32 = value.into();
        Self(value - 1)
    }
}

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

use crate::AxisCount;

use thiserror::Error;
/// A length along a single dimension.
///
/// The minimum length is zero (a length containing a single point).
///
/// The maximum length is `u32::MAX - 1`.
///
/// Generally, `AxisLength = AxisCount - 1`.
///
/// See also [`AxisCount`]
///
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AxisLength(u32);

impl AxisLength {
    pub const MAX: Self = Self(u32::MAX - 1);
    pub const MIN: Self = Self(0);
}

impl PartialEq<u32> for AxisLength {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<u32> for AxisLength {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

macro_rules! impl_from {
    ($($type:ty),* $(,)?) => {
        $(
        impl From<$type> for AxisLength {
            fn from(value: $type) -> Self {
                Self(value.into())
            }
        })*
    };
}

macro_rules! impl_try_from {
    ($($type:ty),* $(,)?) => {
        $(
        impl TryFrom<$type> for AxisLength {
            type Error = AxisLengthError;
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                match u32::try_from(value) {
                    Ok(value) if value < u32::MAX => return Ok(Self(value)),
                    _ => {}
                }

                #[allow(unused_comparisons)]
                if value < 0 {
                    Err(AxisLengthError::Negative)
                } else {
                    Err(AxisLengthError::TooLarge)
                }
            }
        })*
    };
}

macro_rules! impl_from_axis_length {
    ($($type:ty),* $(,)?) => {
        $(
            impl From<AxisLength> for $type {
                fn from(value: AxisLength) -> $type {
                    value.0.into()
                }
            }
        )*
    }
}

macro_rules! impl_try_from_axis_length {
    ($($type:ty),* $(,)?) => {
        $(
            impl TryFrom<AxisLength> for $type {
                type Error = AxisLengthError;
                fn try_from(value: AxisLength) -> Result<$type, Self::Error> {
                    <$type>::try_from(u32::from(value)).map_err(|_| AxisLengthError::TooLarge)
                }
            }
        )*
    }
}

impl From<AxisCount> for AxisLength {
    fn from(value: AxisCount) -> Self {
        Self(value.as_u32() - 1)
    }
}

#[derive(Error, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum AxisLengthError {
    #[error("Axis length value too large")]
    TooLarge,

    #[error("Negative axis length value")]
    Negative,
}

impl_from!(u8, u16);
impl_try_from!(u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_from_axis_length!(u32, u64, u128, i64, i128);
impl_try_from_axis_length!(u8, u16, i8, i16, i32, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_conversion_error {
        ($error:expr; $($value:expr),+ $(,)?) => {
            $(
                assert_eq!(
                    AxisLength::try_from($value),
                    Err($error),
                    "{} should error",
                    stringify!($value),
                );
            )+
        };
    }

    #[test]
    fn valid_values_convert() {
        assert_eq!(AxisLength::from(0_u8), AxisLength(0));
        assert_eq!(AxisLength::from(0_u16), AxisLength(0));
        assert_eq!(AxisLength::try_from(0_u32), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_u64), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_u128), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_usize), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_i8), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_i16), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_i32), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_i64), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_i128), Ok(AxisLength(0)));
        assert_eq!(AxisLength::try_from(0_isize), Ok(AxisLength(0)));
    }

    #[test]
    fn smallest_nonzero_values_convert() {
        assert_eq!(AxisLength::from(1_u8), AxisLength(1));
        assert_eq!(AxisLength::from(1_u16), AxisLength(1));
        assert_eq!(AxisLength::try_from(1_u32), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_u64), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_u128), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_usize), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_i8), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_i16), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_i32), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_i64), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_i128), Ok(AxisLength(1)));
        assert_eq!(AxisLength::try_from(1_isize), Ok(AxisLength(1)));
    }

    #[test]
    fn maximum_value_converts() {
        let maximum = u32::MAX - 1;
        assert_eq!(AxisLength::try_from(maximum), Ok(AxisLength(maximum)));
        assert_eq!(
            AxisLength::try_from(u64::from(maximum)),
            Ok(AxisLength(maximum))
        );
        assert_eq!(
            AxisLength::try_from(u128::from(maximum)),
            Ok(AxisLength(maximum))
        );
        assert_eq!(
            AxisLength::try_from(i64::from(maximum)),
            Ok(AxisLength(maximum))
        );
        assert_eq!(
            AxisLength::try_from(i128::from(maximum)),
            Ok(AxisLength(maximum))
        );

        if usize::BITS >= u32::BITS {
            assert_eq!(
                AxisLength::try_from(maximum as usize),
                Ok(AxisLength(maximum))
            );
        }
        if isize::BITS > u32::BITS {
            assert_eq!(
                AxisLength::try_from(maximum as isize),
                Ok(AxisLength(maximum))
            );
        }
    }

    #[test]
    fn out_of_bounds_max_values_error() {
        assert_conversion_error!(
            AxisLengthError::TooLarge;
            u32::MAX, u64::MAX, u128::MAX, i64::MAX, i128::MAX,
        );

        if usize::BITS >= u32::BITS {
            assert_conversion_error!(AxisLengthError::TooLarge; usize::MAX);
        }
        if isize::BITS > u32::BITS {
            assert_conversion_error!(AxisLengthError::TooLarge; isize::MAX);
        }
    }

    #[test]
    fn negative_values_error() {
        assert_conversion_error!(
            AxisLengthError::Negative;
            -1_i8, -1_i16, -1_i32, -1_i64, -1_i128, -1_isize,
        );
    }
}

//! Provides OneBased* unsigned int types, which wraps several integers as 1-based index.
//!
//! Example:
//! ```
//! # use one_based::{OneBasedU32, OneBasedError};
//! # use std::num::NonZeroU32;
//! let v: OneBasedU32 = "5".parse().unwrap();
//! assert_eq!(v.as_zero_based(), 4);
//! assert_eq!(v.as_one_based(), NonZeroU32::new(5).unwrap());
//!
//! let v: Result<OneBasedU32, OneBasedError> = OneBasedU32::from_one_based(0);
//! assert_eq!(v.unwrap_err(), OneBasedError::ZeroIndex);
//! ```

use std::{
    fmt::Display,
    num::{
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, ParseIntError,
    },
    str::FromStr,
};

macro_rules! define_one_based {
    ($name:ident, $itype:ty, $nonzerotype:ty) => {
        #[doc = concat!(r" Represents 1-based index of ", stringify!($itype), r".")]
        ///
        /// To describe configuration by humans, often 1-based index is easier than 0-based to understand.
        /// On the other hand, 0-based index is easier to use in the programming.
        /// Also, it's quite hard to track if the index is 0-based or 1-based.
        /// `$name` provides ergonomics to handle user provided 1-baed index safely.
        ///
        #[doc = r" ```"]
        #[doc = r" # fn main() -> Result<(), one_based::OneBasedError> {"]
        #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
        #[doc = r" // Creates from 1-based index"]
        #[doc = concat!(r" let v = ", stringify!($name),r"::from_one_based(5)?;")]
        #[doc = r" assert_eq!(v.as_zero_based(), 4);"]
        #[doc = r""]
        #[doc = r" // Creates from 0-based index"]
        #[doc = concat!(r" let v = ", stringify!($name),r"::from_zero_based(0)?;")]
        #[doc = r" assert_eq!(v.as_one_based().get(), 1);"]
        #[doc = r" # Ok(())"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name($nonzerotype);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.as_one_based().fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v: $nonzerotype = s.parse()?;
                Ok(Self::from_one_based_nonzero(v))
            }
        }

        impl $name {
            /// Creates `$name` from 1-based index value.
            /// Returns error if the given index is zero.
            pub fn from_one_based(v: $itype) -> Result<Self, OneBasedError> {
                let v = <$nonzerotype>::new(v).ok_or(OneBasedError::ZeroIndex)?;
                Ok($name(v))
            }

            /// Creates `$name` from 1-based index value as [`$nonzerotype`].
            /// This will always succeed.
            #[inline]
            pub fn from_one_based_nonzero(v: $nonzerotype) -> Self {
                Self(v)
            }

            /// Creates `$name` from 0-based index value.
            /// Returns error if the given index is MAX value,
            /// as that would case overflow when converted to 1-based.
            pub fn from_zero_based(v: $itype) -> Result<Self, OneBasedError> {
                if v == <$nonzerotype>::MAX.get() {
                    return Err(OneBasedError::OverflowIndex);
                }
                let v = unsafe {
                    // this won't overflow, and cannot be zero (note all $itype is unsigned).
                    <$nonzerotype>::new_unchecked(v + 1)
                };
                Ok($name(v))
            }

            /// Returns regular 0-based index.
            pub fn as_zero_based(&self) -> $itype {
                self.0.get() - 1
            }

            /// Returns 1-based index.
            pub fn as_one_based(&self) -> $nonzerotype {
                self.0
            }
        }
    };
}

define_one_based!(OneBasedU8, u8, NonZeroU8);
define_one_based!(OneBasedU16, u16, NonZeroU16);
define_one_based!(OneBasedU32, u32, NonZeroU32);
define_one_based!(OneBasedU64, u64, NonZeroU64);
define_one_based!(OneBasedU128, u128, NonZeroU128);
define_one_based!(OneBasedUsize, usize, NonZeroUsize);

/// Error type used when converting integer to OneBased* types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneBasedError {
    ZeroIndex,
    OverflowIndex,
}

impl Display for OneBasedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OneBasedError::ZeroIndex => f.write_str("0 passed as 1-based index"),
            OneBasedError::OverflowIndex => {
                f.write_str("unsigned::MAX cannot be used as 0-based index")
            }
        }
    }
}

impl std::error::Error for OneBasedError {}

#[cfg(test)]
mod tests {
    use std::num::IntErrorKind;

    use super::*;

    #[test]
    fn in_range_ints_converts_each_other() {
        assert_eq!(
            OneBasedUsize::from_one_based_nonzero(NonZeroUsize::new(1).unwrap()).as_zero_based(),
            0
        );
        assert_eq!(
            OneBasedU16::from_zero_based(u16::MAX - 1)
                .unwrap()
                .as_one_based(),
            NonZeroU16::MAX
        );
    }

    #[test]
    fn overflow_fails_on_zero_based() {
        assert_eq!(
            Err(OneBasedError::OverflowIndex),
            OneBasedU8::from_zero_based(u8::MAX)
        );
        assert_eq!(
            Err(OneBasedError::OverflowIndex),
            OneBasedU16::from_zero_based(u16::MAX)
        );
        assert_eq!(
            Err(OneBasedError::OverflowIndex),
            OneBasedU32::from_zero_based(u32::MAX)
        );
        assert_eq!(
            Err(OneBasedError::OverflowIndex),
            OneBasedU64::from_zero_based(u64::MAX)
        );
        assert_eq!(
            Err(OneBasedError::OverflowIndex),
            OneBasedU128::from_zero_based(u128::MAX)
        );
    }

    #[test]
    fn from_str_and_to_string() {
        let v: OneBasedU16 = "12345".parse().unwrap();
        assert_eq!(v.as_zero_based(), 12344u16);
        assert_eq!(&v.to_string(), "12345");
    }

    #[test]
    fn from_str_failures() {
        let err = OneBasedU8::from_str("0").unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::Zero);

        let err = OneBasedU8::from_str("256").unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::PosOverflow);
    }
}

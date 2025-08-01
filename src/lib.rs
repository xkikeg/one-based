//! Provides OneBased* unsigned int types, which wraps several integers as 1-based index.
//!
//! Example:
//! ```
//! # use one_based::{OneBasedU32, OneBasedU64, OneBasedError};
//! # use std::num::NonZeroU64;
//! // constructs from 1-based.
//! let v = OneBasedU32::from_one_based(1).unwrap();
//! assert_eq!(v.as_zero_based(), 0);
//!
//! // constructs from 0-based.
//! let v = OneBasedU64::from_zero_based(0).unwrap();
//! assert_eq!(v.as_one_based(), NonZeroU64::new(1).unwrap());
//!
//! // fails to construct from zero.
//! let v: Result<OneBasedU32, OneBasedError> = OneBasedU32::from_one_based(0);
//! assert_eq!(v.unwrap_err(), OneBasedError::ZeroIndex);
//!
//! // string format uses 1-based.
//! let v: OneBasedU32 = "5".parse().unwrap();
//! assert_eq!(v.as_zero_based(), 4);
//! assert_eq!(&v.to_string(), "5");
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    fmt::Display,
    num::{
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, ParseIntError,
    },
    str::FromStr,
};

trait OneBased {
    type IntType;
    type NonZeroType;
}

macro_rules! define_one_based {
    ($name:ident, $itype:ty, $nonzerotype:ty) => {
        #[doc = concat!(r" Represents 1-based index of ", stringify!($itype), r".")]
        ///
        /// To describe configuration by humans, often 1-based index is easier than 0-based to understand.
        /// On the other hand, 0-based index is easier to use in the programming.
        /// Also, it's quite hard to track if the index is 0-based or 1-based.
        /// `$name` provides ergonomics to handle user provided 1-baed index safely.
        ///
        /// ```
        #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
        #[doc = r" // Creates from 1-based index"]
        #[doc = concat!(r" let v = ", stringify!($name),r"::from_one_based(5)?;")]
        #[doc = r" assert_eq!(v.as_zero_based(), 4);"]
        #[doc = r""]
        #[doc = r" // Creates from 0-based index"]
        #[doc = concat!(r" let v = ", stringify!($name),r"::from_zero_based(0)?;")]
        #[doc = r" assert_eq!(v.as_one_based().get(), 1);"]
        #[doc = r" # Ok::<(), one_based::OneBasedError>(())"]
        /// ```
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name($nonzerotype);

        impl OneBased for $name {
            type IntType = $itype;
            type NonZeroType = $nonzerotype;
        }

        impl Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            #[inline]
            pub const fn from_one_based(v: $itype) -> Result<Self, OneBasedError> {
                match <$nonzerotype>::new(v) {
                    None => return Err(OneBasedError::ZeroIndex),
                    Some(v) => Ok($name(v)),
                }
            }

            /// Creates `$name` from 1-based index value without check.
            ///
            /// # Safety
            ///
            /// Input must be greater than zero.
            #[inline]
            pub const unsafe fn from_one_based_unchecked(v: $itype) -> Self {
                $name(<$nonzerotype>::new_unchecked(v))
            }

            /// Creates `$name` from 1-based index value as [`$nonzerotype`].
            /// This will always succeed.
            #[inline]
            pub const fn from_one_based_nonzero(v: $nonzerotype) -> Self {
                Self(v)
            }

            /// Creates `$name` from 0-based index value.
            /// Returns error if the given index is MAX value,
            /// as that would case overflow when converted to 1-based.
            #[inline]
            pub const fn from_zero_based(v: $itype) -> Result<Self, OneBasedError> {
                if v == <$nonzerotype>::MAX.get() {
                    return Err(OneBasedError::OverflowIndex);
                }
                // this won't overflow, and cannot be zero (note all $itype is unsigned).
                Ok($name(unsafe { <$nonzerotype>::new_unchecked(v + 1) }))
            }

            /// Creates `$name` from 0-based index value without check.
            ///
            /// # Safety
            #[doc = concat!(r" This function results in undefined behavior when `v == ", stringify!($itype), r"::MAX`.")]
            /// ```no_run
            #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
            /// // This should cause undefined behavior
            /// unsafe {
            #[doc = concat!(r"     ", stringify!($name), "::from_zero_based_unchecked(", stringify!($itype), r"::MAX);")]
            /// }
            /// ```
            #[inline]
            pub const unsafe fn from_zero_based_unchecked(v: $itype) -> Self {
                // this won't overflow, and cannot be zero (note all $itype is unsigned).
                $name(unsafe { <$nonzerotype>::new_unchecked(v + 1) })
            }

            /// Returns regular 0-based index.
            pub const fn as_zero_based(&self) -> $itype {
                self.0.get() - 1
            }

            /// Returns 1-based index.
            pub const fn as_one_based(&self) -> $nonzerotype {
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

macro_rules! impl_from_one_based {
    ($source:ty => $($target:ty),+) => {$(
        impl core::convert::From<$source> for $target {
            #[doc = concat!(r"Converts [`", stringify!($source), r"`] to [`", stringify!($target), r"`].")]
            #[inline]
            fn from(value: $source) -> Self {
                use core::convert::Into as _;
                let v: <$target as OneBased>::NonZeroType = value.as_one_based().into();
                <$target>::from_one_based_nonzero(v)
            }
        }
    )*};
}

impl_from_one_based!(OneBasedU8  => OneBasedU16, OneBasedU32, OneBasedU64, OneBasedU128);
impl_from_one_based!(OneBasedU16 => OneBasedU32, OneBasedU64, OneBasedU128);
impl_from_one_based!(OneBasedU32 => OneBasedU64, OneBasedU128);
impl_from_one_based!(OneBasedU64 => OneBasedU128);

macro_rules! impl_try_from_one_based {
    ($source:ty => $($target:ty),+) => {$(
        impl core::convert::TryFrom<$source> for $target {
            type Error = core::num::TryFromIntError;

            #[doc = concat!(r"Attempts to convert [`", stringify!($source), r"`] to [`", stringify!($target), r"`].")]
            #[inline]
            fn try_from(value: $source) -> Result<Self, Self::Error> {
                use core::convert::TryInto as _;
                let v: <$target as OneBased>::NonZeroType = value.as_one_based().try_into()?;
                Ok(<$target>::from_one_based_nonzero(v))
            }
        }
    )*};
}

impl_try_from_one_based!(OneBasedU8 => OneBasedUsize);
impl_try_from_one_based!(OneBasedU16 => OneBasedUsize, OneBasedU8);
impl_try_from_one_based!(OneBasedU32 => OneBasedUsize, OneBasedU8, OneBasedU16);
impl_try_from_one_based!(OneBasedU64 => OneBasedUsize, OneBasedU8, OneBasedU16, OneBasedU32);
impl_try_from_one_based!(OneBasedU128 => OneBasedUsize, OneBasedU8, OneBasedU16, OneBasedU32, OneBasedU64);
impl_try_from_one_based!(OneBasedUsize => OneBasedU8, OneBasedU16, OneBasedU32, OneBasedU64, OneBasedU128);

/// Error type used when converting integer to OneBased* types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneBasedError {
    ZeroIndex,
    OverflowIndex,
}

impl Display for OneBasedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OneBasedError::ZeroIndex => f.write_str("0 passed as 1-based index"),
            OneBasedError::OverflowIndex => {
                f.write_str("unsigned::MAX cannot be used as 0-based index")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OneBasedError {}

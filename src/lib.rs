//! Provides OneBased* unsigned int types, which wraps several integers as 1-based index.
//!
//! Example:
//! ```
//! # use one_based::{OneBasedU32, OneBasedU64};
//! # use std::num::NonZeroU64;
//! // constructs from 1-based.
//! let v = OneBasedU32::from_one_based(1).unwrap();
//! assert_eq!(v.as_zero_based(), 0);
//!
//! // constructs from 0-based.
//! let v = OneBasedU64::from_zero_based(0).unwrap();
//! assert_eq!(v.as_one_based(), NonZeroU64::new(1).unwrap());
//!
//! // fails to construct from zero as one-based.
//! let v: Option<OneBasedU32> = OneBasedU32::from_one_based(0);
//! assert_eq!(v, None);
//!
//! // fails to construct from max as zero-based.
//! let v: Option<OneBasedU32> = OneBasedU32::from_zero_based(u32::MAX);
//! assert_eq!(v, None);
//!
//! // string format uses 1-based.
//! let v: OneBasedU32 = "5".parse().unwrap();
//! assert_eq!(v.as_zero_based(), 4);
//! assert_eq!(v.to_string(), "5");
//! ```

#![no_std]

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
        #[doc = concat!(r" Represents 1-based index of [`", stringify!($itype), r"`].")]
        ///
        /// To describe configuration by humans, often 1-based index is easier than 0-based to understand.
        /// On the other hand, 0-based index is easier to use in the programming.
        /// Also, it's quite hard to track if the index is 0-based or 1-based.
        #[doc = concat!(r" `", stringify!($name), r"` provides ergonomics to handle user provided 1-baed index safely.")]
        ///
        /// ```
        #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
        #[doc = r" // Creates from 1-based index"]
        #[doc = concat!(r" let v = ", stringify!($name),r"::from_one_based(5).unwrap();")]
        #[doc = r" assert_eq!(v.as_zero_based(), 4);"]
        #[doc = r""]
        #[doc = r" // Creates from 0-based index"]
        #[doc = concat!(r" let v = ", stringify!($name),r"::from_zero_based(0).unwrap();")]
        #[doc = r" assert_eq!(v.as_one_based().get(), 1);"]
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
            /// The size of this one-based integer type in bits.
            ///
            #[doc = concat!(r" This value is equal to ", stringify!($itype), r"::BITS.")]
            pub const BITS: u32 = <$itype>::BITS;

            /// The smallest value that can be represented by this one-based integer type, `from_one_based(1)`.
            pub const MIN: Self = Self::from_one_based_nonzero(<$nonzerotype>::MIN);

            #[doc = concat!(r" The largest value that can be represented by this one-based integer type, equal to `from_one_based(", stringify!($itype), r"::MAX)`.")]
            pub const MAX: Self = Self::from_one_based_nonzero(<$nonzerotype>::MAX);

            /// Creates a one-based integer from 1-based index value.
            /// Returns `None` if the given index is zero.
            ///
            /// Note you can define a constant easily given [`Option::unwrap()`] or [`Option::expect()`] are also `const`.
            /// ```
            #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
           #[doc = concat!(r" const ONE_BASED_TEN: ", stringify!($name), r" = ", stringify!($name), r#"::from_one_based(10).expect("10 is non zero");"#)]
            /// assert_eq!(ONE_BASED_TEN.as_zero_based(), 9);
            /// ```
            #[inline]
            pub const fn from_one_based(v: $itype) -> Option<Self> {
                match <$nonzerotype>::new(v) {
                    None => None,
                    Some(v) => Some($name(v)),
                }
            }

            /// Creates a one-based integer from 1-based index value without check.
            ///
            /// # Safety
            ///
            /// Input must be greater than zero.
            #[inline]
            pub const unsafe fn from_one_based_unchecked(v: $itype) -> Self {
                $name(<$nonzerotype>::new_unchecked(v))
            }

            /// Creates a one-based integer from non-zero 1-based index value.
            ///
            /// Given the underlying value is guaranteed to be non-zero, this will always succeed.
            #[inline]
            pub const fn from_one_based_nonzero(v: $nonzerotype) -> Self {
                Self(v)
            }

            /// Creates a one-based integer from 0-based index value.
            /// Returns `None` if the given index is MAX value,
            /// as that would cause overflow when converted to 1-based.
            #[inline]
            pub const fn from_zero_based(v: $itype) -> Option<Self> {
                if v == <$nonzerotype>::MAX.get() {
                    return None;
                }
                // this won't overflow, and cannot be zero (note all $itype is unsigned).
                Some($name(unsafe { <$nonzerotype>::new_unchecked(v + 1) }))
            }

            /// Creates a one-based integer from 0-based index value without check.
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
            pub const fn as_zero_based(self) -> $itype {
                self.0.get() - 1
            }

            /// Returns 1-based index.
            pub const fn as_one_based(self) -> $nonzerotype {
                self.0
            }

            /// Adds an unsigned integer to a one-based integer value. Checks for overflow and returns `None` on overflow.
            ///
            #[doc = r" ```"]
            #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
            #[doc = concat!(r" let one = ", stringify!($name), "::from_zero_based(1).unwrap();")]
            #[doc = concat!(r" let two = ", stringify!($name), "::from_zero_based(2).unwrap();")]
            #[doc = concat!(r" let max = ", stringify!($name), "::MAX;")]
            #[doc = r""]
            #[doc = r" assert_eq!(Some(two), one.checked_add(1));"]
            #[doc = r" assert_eq!(None, max.checked_add(1));"]
            #[doc = r" ```"]
            pub const fn checked_add(self, other: $itype) -> Option<Self> {
                match self.0.checked_add(other) {
                    None => None,
                    Some(v) => Some(Self(v)),
                }
            }

            /// Adds an unsigned integer to a one-based integer value. Returns [`Self::MAX`] on overflow.
            ///
            #[doc = r" ```"]
            #[doc = concat!(r" # use one_based::", stringify!($name), r";")]
            #[doc = concat!(r" let one = ", stringify!($name), "::from_zero_based(1).unwrap();")]
            #[doc = concat!(r" let two = ", stringify!($name), "::from_zero_based(2).unwrap();")]
            #[doc = concat!(r" let max = ", stringify!($name), "::MAX;")]
            #[doc = r""]
            #[doc = r" assert_eq!(two, one.saturating_add(1));"]
            #[doc = r" assert_eq!(max, max.saturating_add(1));"]
            #[doc = r" ```"]
            pub const fn saturating_add(self, other: $itype) -> Self {
                Self(self.0.saturating_add(other))
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

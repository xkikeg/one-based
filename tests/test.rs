#![no_std]

use core::num::{IntErrorKind, NonZeroU16, NonZeroUsize};
use core::str::FromStr;

use arrayvec::ArrayString;
use one_based::{OneBasedU128, OneBasedU16, OneBasedU32, OneBasedU64, OneBasedU8, OneBasedUsize};

mod constness {
    use super::*;

    const ONE_BASED_ONE: OneBasedUsize =
        OneBasedUsize::from_one_based(1).expect("1 must be a valid one-based usize usize");
    const ZERO_BASED_ONE: OneBasedUsize =
        OneBasedUsize::from_zero_based(1).expect("1 must be a valid zero-based usize index");
    const ONE_BASED_MAX: OneBasedUsize = OneBasedUsize::from_one_based_nonzero(NonZeroUsize::MAX);

    const ONE_BASED_ONE_AS_ZERO_BASED: usize = ONE_BASED_ONE.as_zero_based();
    const ZERO_BASED_ONE_AS_ONE_BASED: NonZeroUsize = ZERO_BASED_ONE.as_one_based();

    const UNSAFE_ZERO: usize =
        unsafe { OneBasedUsize::from_one_based_unchecked(1) }.as_zero_based();
    const UNSAFE_ONE: NonZeroUsize =
        unsafe { OneBasedUsize::from_zero_based_unchecked(0) }.as_one_based();

    #[test]
    fn verify() {
        assert_eq!(8, OneBasedU8::BITS);
        assert_eq!(ONE_BASED_ONE, OneBasedUsize::MIN);
        assert_eq!(ONE_BASED_MAX, OneBasedUsize::MAX);
        assert_eq!(ONE_BASED_ONE_AS_ZERO_BASED, 0);
        assert_eq!(ZERO_BASED_ONE_AS_ONE_BASED.get(), 2);
        assert_eq!(UNSAFE_ZERO, 0);
        assert_eq!(UNSAFE_ONE, NonZeroUsize::new(1).unwrap());
    }
}

mod from_x_based {
    use super::*;

    #[test]
    fn valid_values() {
        assert_eq!(OneBasedU8::from_one_based(1).unwrap().as_zero_based(), 0);

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

        assert_eq!(
            unsafe { OneBasedU32::from_one_based_unchecked(2) }.as_zero_based(),
            1
        );

        assert_eq!(
            unsafe { OneBasedU32::from_zero_based_unchecked(2) }.as_zero_based(),
            2
        );
    }

    #[test]
    fn zero_fails_on_one_based() {
        assert_eq!(None, OneBasedU8::from_one_based(0));
        assert_eq!(None, OneBasedU16::from_one_based(0));
        assert_eq!(None, OneBasedU32::from_one_based(0));
        assert_eq!(None, OneBasedU64::from_one_based(0));
        assert_eq!(None, OneBasedU128::from_one_based(0));
    }

    #[test]
    fn overflow_fails_on_zero_based() {
        assert_eq!(None, OneBasedU8::from_zero_based(u8::MAX));
        assert_eq!(None, OneBasedU16::from_zero_based(u16::MAX));
        assert_eq!(None, OneBasedU32::from_zero_based(u32::MAX));
        assert_eq!(None, OneBasedU64::from_zero_based(u64::MAX));
        assert_eq!(None, OneBasedU128::from_zero_based(u128::MAX));
    }
}

mod add {
    use super::*;

    #[test]
    fn add_works_for_non_overflow_ops() {
        let zero = OneBasedU8::from_zero_based(0).unwrap();
        let one = OneBasedU8::from_zero_based(1).unwrap();
        let prev_max = OneBasedU8::from_one_based(u8::MAX - 1).unwrap();
        let max = OneBasedU8::from_one_based(u8::MAX).unwrap();

        // Non-overflow ops.
        assert_eq!(Some(zero), zero.checked_add(0));
        assert_eq!(Some(one), zero.checked_add(1));
        assert_eq!(Some(one), one.checked_add(0));
        assert_eq!(Some(max), zero.checked_add(u8::MAX - 1));
        assert_eq!(Some(max), prev_max.checked_add(1));
        assert_eq!(zero, zero.saturating_add(0));
        assert_eq!(one, zero.saturating_add(1));
        assert_eq!(one, one.saturating_add(0));
        assert_eq!(max, zero.saturating_add(u8::MAX - 1));
        assert_eq!(max, prev_max.saturating_add(1));

        // Overflow ops.
        assert_eq!(None, zero.checked_add(u8::MAX));
        assert_eq!(None, one.checked_add(u8::MAX));
        assert_eq!(None, max.checked_add(1));
        assert_eq!(None, max.checked_add(u8::MAX));
        assert_eq!(OneBasedU8::MAX, zero.saturating_add(u8::MAX));
        assert_eq!(OneBasedU8::MAX, one.saturating_add(u8::MAX));
        assert_eq!(OneBasedU8::MAX, max.saturating_add(1));
        assert_eq!(OneBasedU8::MAX, max.saturating_add(u8::MAX));
    }
}

mod from_str {
    use super::*;

    #[test]
    fn valid_input() {
        use core::fmt::Write as _;

        let v: OneBasedU16 = "12345".parse().unwrap();
        assert_eq!(v.as_zero_based(), 12344u16);
        let mut buf: ArrayString<10> = ArrayString::new();
        write!(&mut buf, "{}", v).unwrap();
        assert_eq!(&buf, "12345");
    }

    #[test]
    fn invalid_input() {
        let err = OneBasedU8::from_str("-5").unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::InvalidDigit);

        let err = OneBasedU8::from_str("0").unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::Zero);

        let err = OneBasedU8::from_str("256").unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::PosOverflow);
    }
}

mod conversion {
    use super::*;

    use core::convert::TryInto;

    #[test]
    fn into_works() {
        let v: OneBasedU16 = OneBasedU8::from_one_based(1).unwrap().into();
        assert_eq!(v.as_zero_based(), 0);
        let v: OneBasedU32 = v.into();
        assert_eq!(v.as_zero_based(), 0);
        let v: OneBasedU64 = v.into();
        assert_eq!(v.as_zero_based(), 0);
        let v: OneBasedU128 = v.into();
        assert_eq!(v.as_zero_based(), 0);
    }

    #[test]
    fn try_into_ok() {
        let v = OneBasedU128::from_one_based(1).unwrap();
        let v: OneBasedU64 = v.try_into().unwrap();
        let v: OneBasedU32 = v.try_into().unwrap();
        let v: OneBasedU16 = v.try_into().unwrap();
        let v: OneBasedU8 = v.try_into().unwrap();
        let v: OneBasedUsize = v.try_into().unwrap();
        let v: OneBasedU128 = v.try_into().unwrap();
        assert_eq!(v.as_zero_based(), 0);
    }

    #[test]
    fn try_into_fails() {
        let v = OneBasedU128::from_one_based(1u128.saturating_add(u64::MAX.into())).unwrap();
        let _ = <_ as TryInto<OneBasedU64>>::try_into(v).unwrap_err();

        let v = OneBasedU64::from_one_based(1u64.saturating_add(u32::MAX.into())).unwrap();
        let _ = <_ as TryInto<OneBasedU32>>::try_into(v).unwrap_err();

        let v = OneBasedU32::from_one_based(1u32.saturating_add(u16::MAX.into())).unwrap();
        let _ = <_ as TryInto<OneBasedU16>>::try_into(v).unwrap_err();

        let v = OneBasedU16::from_one_based(1u16.saturating_add(u8::MAX.into())).unwrap();
        let _ = <_ as TryInto<OneBasedU8>>::try_into(v).unwrap_err();
    }
}

#![no_std]

use core::num::{IntErrorKind, NonZeroU16, NonZeroUsize};
use core::str::FromStr;

use arrayvec::ArrayString;
use one_based::*;

mod constness {
    use super::*;

    const fn unwrap_const(v: Result<OneBasedUsize, OneBasedError>) -> OneBasedUsize {
        match v {
            Ok(v) => v,
            Err(_) => panic!("OneBased initialization failed"),
        }
    }

    const ONE_BASED_ONE: OneBasedUsize = unwrap_const(OneBasedUsize::from_one_based(1));
    const ZERO_BASED_ONE: OneBasedUsize = unwrap_const(OneBasedUsize::from_zero_based(1));

    const ONE_BASED_ONE_AS_ZERO_BASED: usize = ONE_BASED_ONE.as_zero_based();
    const ZERO_BASED_ONE_AS_ONE_BASED: NonZeroUsize = ZERO_BASED_ONE.as_one_based();

    #[test]
    fn verify() {
        assert_eq!(ONE_BASED_ONE_AS_ZERO_BASED, 0);
        assert_eq!(ZERO_BASED_ONE_AS_ONE_BASED.get(), 2);
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
    }

    #[test]
    fn zero_fails_on_one_based() {
        assert_eq!(Err(OneBasedError::ZeroIndex), OneBasedU8::from_one_based(0));
        assert_eq!(
            Err(OneBasedError::ZeroIndex),
            OneBasedU16::from_one_based(0)
        );
        assert_eq!(
            Err(OneBasedError::ZeroIndex),
            OneBasedU32::from_one_based(0)
        );
        assert_eq!(
            Err(OneBasedError::ZeroIndex),
            OneBasedU64::from_one_based(0)
        );
        assert_eq!(
            Err(OneBasedError::ZeroIndex),
            OneBasedU128::from_one_based(0)
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

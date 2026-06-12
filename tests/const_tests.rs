use kitness::{u8, u16, u32, u64, usize};

const _: () = {
    assert!(u8::non_empty::<2, 8>().within(u8::non_empty::<1, 10>()));
    assert!(!u8::non_empty::<2, 8>().within(u8::non_empty::<3, 10>()));
    assert!(!u8::non_empty::<2, 8>().within(u8::non_empty::<1, 7>()));

    assert!(u16::empty::<8>().within(u16::empty::<10>()));
    assert!(!u16::empty::<8>().within(u16::empty::<7>()));

    assert!(u8::non_empty::<2, 8>().fitsin() == 7);
    assert!(u16::empty::<8>().fitsin() == 9);
    assert!(u32::fixed::<8>().fitsin() == 1);

    assert!(u32::fixed::<8>().within(u32::non_empty::<1, 10>()));
    assert!(!u32::fixed::<8>().within(u32::non_empty::<9, 10>()));
    assert!(!u32::fixed::<8>().within(u32::non_empty::<1, 7>()));

    assert!(usize::non_empty::<2, 8>().within(usize::non_empty::<1, 10>()));
    assert!(u64::non_empty::<2, 8>().within(u64::empty::<10>()));

    assert!(u8::fixed::<8>().within(u8::fixed::<8>()));
    assert!(!u8::fixed::<8>().within(u8::fixed::<9>()));

    assert!(u16::fixed::<8>().within(u32::non_empty::<1, 10>()));
    assert!(!u16::fixed::<8>().within(u32::non_empty::<9, 10>()));
    assert!(!u16::fixed::<8>().within(u32::non_empty::<1, 7>()));

    assert!(u8::non_empty::<2, 8>().within(u64::empty::<10>()));
    assert!(!u8::empty::<8>().within(u16::fixed::<8>()));
    assert!(usize::fixed::<8>().within(u8::fixed::<8>()));
};

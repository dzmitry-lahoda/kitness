use kitness::witnesses;

const _: () = {
    assert!(witnesses::u8::non_empty::<2, 8>().within(witnesses::u8::non_empty::<1, 10>()));
    assert!(!witnesses::u8::non_empty::<2, 8>().within(witnesses::u8::non_empty::<3, 10>()));
    assert!(!witnesses::u8::non_empty::<2, 8>().within(witnesses::u8::non_empty::<1, 7>()));

    assert!(witnesses::u16::empty::<8>().within(witnesses::u16::empty::<10>()));
    assert!(!witnesses::u16::empty::<8>().within(witnesses::u16::empty::<7>()));

    assert!(witnesses::u32::fixed::<8>().within(witnesses::u32::non_empty::<1, 10>()));
    assert!(!witnesses::u32::fixed::<8>().within(witnesses::u32::non_empty::<9, 10>()));
    assert!(!witnesses::u32::fixed::<8>().within(witnesses::u32::non_empty::<1, 7>()));

    assert!(witnesses::usize::non_empty::<2, 8>().within(witnesses::usize::non_empty::<1, 10>()));
    assert!(witnesses::u64::non_empty::<2, 8>().within(witnesses::u64::empty::<10>()));

    assert!(witnesses::u8::fixed::<8>().within(witnesses::u8::fixed::<8>()));
    assert!(!witnesses::u8::fixed::<8>().within(witnesses::u8::fixed::<9>()));

    assert!(witnesses::u16::fixed::<8>().within(witnesses::u32::non_empty::<1, 10>()));
    assert!(!witnesses::u16::fixed::<8>().within(witnesses::u32::non_empty::<9, 10>()));
    assert!(!witnesses::u16::fixed::<8>().within(witnesses::u32::non_empty::<1, 7>()));

    assert!(witnesses::u8::non_empty::<2, 8>().within(witnesses::u64::empty::<10>()));
    assert!(!witnesses::u8::empty::<8>().within(witnesses::u16::fixed::<8>()));
    assert!(witnesses::usize::fixed::<8>().within(witnesses::u8::fixed::<8>()));
};

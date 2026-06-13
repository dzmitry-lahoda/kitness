#[test]
fn converts_witnesses_into_inclusive_ranges() {
    let non_empty: std::range::RangeInclusive<u8> = u8::non_empty::<2, 8>().into();
    assert_eq!(non_empty, std::range::RangeInclusive { start: 2, last: 8 });

    let empty: std::range::RangeInclusive<u16> = u16::empty::<8>().into();
    assert_eq!(empty, std::range::RangeInclusive { start: 0, last: 8 });

    let fixed: std::range::RangeInclusive<u32> = u32::fixed::<8>().into();
    assert_eq!(fixed, std::range::RangeInclusive { start: 8, last: 8 });
}

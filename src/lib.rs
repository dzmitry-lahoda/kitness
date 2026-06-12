//! Type witnesses used to prove vector bounds at compile time.

// NOTE:
// we can have prove if needed for some cases like 8/16/32/64 upper bound and operating range,
// and make memory layout more efficient:
// - decide stackalloc or smallvec or std::vec, depending on range * size_of at compile time
// - make some values of vec to be not usize, but other numbers

macro_rules! define_witnesses {
    ($module:ident, $integer:ty) => {
        pub mod $module {
            /// Compile-time proof of valid bounds.
            ///
            /// Must be constructed with same bounds to instantiate `BoundedVec`.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct NonEmpty<const L: $integer, const U: $integer>(());

            impl<const L: $integer, const U: $integer> NonEmpty<L, U> {
                /// Checks whether this witness is within the target bounds.
                pub const fn within<const WL: $integer, const WU: $integer>(self) -> bool {
                    konst::cmp::const_le!(WL, L) && konst::cmp::const_le!(U, WU)
                }
            }

            pub type OneOrMore = NonEmpty<1, { <$integer>::MAX }>;

            /// Possibly empty vector with upper bound.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct Empty<const U: $integer>(());

            impl<const U: $integer> Empty<U> {
                /// Checks whether this witness is within the target upper bound.
                pub const fn within<const WU: $integer>(self) -> bool {
                    konst::cmp::const_le!(U, WU)
                }
            }

            /// Fixed capacity vector. Cannot be resized.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct Fixed<const C: $integer>(());

            impl<const C: $integer> Fixed<C> {
                /// Checks whether this witness is within the target bounds.
                pub const fn within<const WL: $integer, const WU: $integer>(self) -> bool {
                    konst::cmp::const_le!(WL, C) && konst::cmp::const_le!(C, WU)
                }
            }

            /// Type a compile-time proof of valid bounds.
            pub const fn non_empty<const L: $integer, const U: $integer>() -> NonEmpty<L, U> {
                const {
                    if L == 0 {
                        panic!("L must be greater than 0")
                    }
                    if L > U {
                        panic!("L must be less than or equal to U")
                    }

                    serde::<U>();
                    NonEmpty::<L, U>(())
                }
            }

            const fn serde<const U: $integer>() {
                #[cfg(feature = "schemars12")]
                if U as u128 > u32::MAX as u128 {
                    // There is no const safe way to cast usize to u32, nor to other bigger number.
                    panic!(
                        "`schemars` encodes `maxLength` as u32, so `U` must be less than or equal to `u32::MAX`"
                    )
                }

                #[cfg(feature = "borsh15")]
                if U as u128 > u32::MAX as u128 {
                    panic!(
                        "`borsh` specifies size of dynamic containers as u32, so `U` must be less than or equal to `u32::MAX`"
                    )
                }
            }

            /// Type a compile-time proof for possibly empty vector with upper bound.
            pub const fn empty<const U: $integer>() -> Empty<U> {
                const {
                    serde::<U>();
                    Empty::<U>(())
                }
            }

            /// Type a compile-time proof for a fixed-capacity vector.
            pub const fn fixed<const C: $integer>() -> Fixed<C> {
                const {
                    serde::<C>();
                    Fixed::<C>(())
                }
            }
        }
    };
}

/// Witnesses grouped by the integer type used for their const bounds.
pub mod witnesses {
    define_witnesses!(u8, u8);
    define_witnesses!(u16, u16);
    define_witnesses!(u32, u32);
    define_witnesses!(usize, usize);
    define_witnesses!(u64, u64);
}

pub use witnesses::usize::{Empty, Fixed, NonEmpty, OneOrMore, empty, fixed, non_empty};

#[cfg(test)]
mod tests {
    use super::witnesses;

    const _: () = {
        assert!(witnesses::u8::non_empty::<2, 8>().within::<1, 10>());
        assert!(!witnesses::u8::non_empty::<2, 8>().within::<3, 10>());
        assert!(!witnesses::u8::non_empty::<2, 8>().within::<1, 7>());

        assert!(witnesses::u16::empty::<8>().within::<10>());
        assert!(!witnesses::u16::empty::<8>().within::<7>());

        assert!(witnesses::u32::fixed::<8>().within::<1, 10>());
        assert!(!witnesses::u32::fixed::<8>().within::<9, 10>());
        assert!(!witnesses::u32::fixed::<8>().within::<1, 7>());

        assert!(witnesses::usize::non_empty::<2, 8>().within::<1, 10>());
        assert!(witnesses::u64::non_empty::<2, 8>().within::<1, 10>());
    };
}

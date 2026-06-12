//! Type witnesses used to prove vector bounds at compile time.

// NOTE:
// we can have prove if needed for some cases like 8/16/32/64 upper bound and operating range,
// and make memory layout more efficient:
// - decide stackalloc or smallvec or std::vec, depending on range * size_of at compile time
// - make some values of vec to be not usize, but other numbers

/// Normalized bounds for any vector-bound witness.
pub trait WitnessBounds: Copy {
    const LOWER: u128;
    const UPPER: u128;
}

/// Checks whether one witness's bounds are within another witness's bounds.
pub const fn witness_within<Inner, Outer>(inner: Inner, outer: Outer) -> bool
where
    Inner: WitnessBounds,
    Outer: WitnessBounds,
{
    let _ = (inner, outer);
    konst::cmp::const_le!(Outer::LOWER, Inner::LOWER)
        && konst::cmp::const_le!(Inner::UPPER, Outer::UPPER)
}

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
                /// Checks whether this witness is within another witness.
                pub const fn within<Other>(self, other: Other) -> bool
                where
                    Other: crate::WitnessBounds,
                {
                    crate::witness_within(self, other)
                }
            }

            impl<const L: $integer, const U: $integer> crate::WitnessBounds for NonEmpty<L, U> {
                const LOWER: u128 = L as u128;
                const UPPER: u128 = U as u128;
            }

            pub type OneOrMore = NonEmpty<1, { <$integer>::MAX }>;

            /// Possibly empty vector with upper bound.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct Empty<const U: $integer>(());

            impl<const U: $integer> Empty<U> {
                /// Checks whether this witness is within another witness.
                pub const fn within<Other>(self, other: Other) -> bool
                where
                    Other: crate::WitnessBounds,
                {
                    crate::witness_within(self, other)
                }
            }

            impl<const U: $integer> crate::WitnessBounds for Empty<U> {
                const LOWER: u128 = 0;
                const UPPER: u128 = U as u128;
            }

            /// Fixed capacity vector. Cannot be resized.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct Fixed<const C: $integer>(());

            impl<const C: $integer> Fixed<C> {
                /// Checks whether this witness is within another witness.
                pub const fn within<Other>(self, other: Other) -> bool
                where
                    Other: crate::WitnessBounds,
                {
                    crate::witness_within(self, other)
                }
            }

            impl<const C: $integer> crate::WitnessBounds for Fixed<C> {
                const LOWER: u128 = C as u128;
                const UPPER: u128 = C as u128;
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


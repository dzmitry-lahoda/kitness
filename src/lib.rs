//! Type witnesses used to prove vector bounds at compile time.

// NOTE:
// we can have prove if needed for some cases like 8/16/32/64 max bound and operating range,
// and make memory layout more efficient:
// - decide stackalloc or smallvec or std::vec, depending on range * size_of at compile time
// - make some values of vec to be not usize, but other numbers

mod sealed {
    pub trait WitnessBounds {}
}

/// Normalized bounds for any vector-bound witness.
pub trait WitnessBounds: Copy + sealed::WitnessBounds {
    const MIN: u128;
    const MAX: u128;
}

/// Checks whether one witness's bounds are within another witness's bounds.
pub const fn witness_within<Inner, Outer>(inner: Inner, outer: Outer) -> bool
where
    Inner: WitnessBounds,
    Outer: WitnessBounds,
{
    let _ = (inner, outer);
    konst::cmp::const_le!(Outer::MIN, Inner::MIN) && konst::cmp::const_le!(Inner::MAX, Outer::MAX)
}

macro_rules! define_witnesses {
    ($module:ident, $integer:ty) => {
        pub mod $module {
            /// Compile-time proof of valid bounds.
            ///
            /// Must be constructed with same bounds to instantiate `BoundedVec`.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct NonEmpty<const MIN: $integer, const MAX: $integer>(());

            impl<const MIN: $integer, const MAX: $integer> NonEmpty<MIN, MAX> {
                /// Counts how many lengths fit in this witness.
                pub const fn fitsin(self) -> u128 {
                    let _ = self;
                    MAX as u128 - MIN as u128 + 1
                }

                /// Checks whether this witness is within another witness.
                pub const fn within<Other>(self, other: Other) -> bool
                where
                    Other: crate::WitnessBounds,
                {
                    crate::witness_within(self, other)
                }
            }

            impl<const MIN: $integer, const MAX: $integer> crate::WitnessBounds
                for NonEmpty<MIN, MAX>
            {
                const MIN: u128 = MIN as u128;
                const MAX: u128 = MAX as u128;
            }

            impl<const MIN: $integer, const MAX: $integer> crate::sealed::WitnessBounds
                for NonEmpty<MIN, MAX>
            {
            }

            pub type OneOrMore = NonEmpty<1, { <$integer>::MAX }>;

            /// Possibly empty vector with max bound.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct Empty<const MAX: $integer>(());

            impl<const MAX: $integer> Empty<MAX> {
                /// Counts how many lengths fit in this witness.
                pub const fn fitsin(self) -> u128 {
                    let _ = self;
                    MAX as u128 + 1
                }

                /// Checks whether this witness is within another witness.
                pub const fn within<Other>(self, other: Other) -> bool
                where
                    Other: crate::WitnessBounds,
                {
                    crate::witness_within(self, other)
                }
            }

            impl<const MAX: $integer> crate::WitnessBounds for Empty<MAX> {
                const MIN: u128 = 0;
                const MAX: u128 = MAX as u128;
            }

            impl<const MAX: $integer> crate::sealed::WitnessBounds for Empty<MAX> {}

            /// Fixed capacity vector. Cannot be resized.
            #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
            #[non_exhaustive]
            pub struct Fixed<const C: $integer>(());

            impl<const C: $integer> Fixed<C> {
                /// Counts how many lengths fit in this witness.
                pub const fn fitsin(self) -> u128 {
                    let _ = self;
                    1
                }

                /// Checks whether this witness is within another witness.
                pub const fn within<Other>(self, other: Other) -> bool
                where
                    Other: crate::WitnessBounds,
                {
                    crate::witness_within(self, other)
                }
            }

            impl<const C: $integer> crate::WitnessBounds for Fixed<C> {
                const MIN: u128 = C as u128;
                const MAX: u128 = C as u128;
            }

            impl<const C: $integer> crate::sealed::WitnessBounds for Fixed<C> {}

            /// Type a compile-time proof of valid bounds.
            pub const fn non_empty<const MIN: $integer, const MAX: $integer>() -> NonEmpty<MIN, MAX> {
                const {
                    if MIN == 0 {
                        panic!("MIN must be greater than 0")
                    }
                    if MIN > MAX {
                        panic!("MIN must be less than or equal to MAX")
                    }

                    serde::<MAX>();
                    NonEmpty::<MIN, MAX>(())
                }
            }

            const fn serde<const MAX: $integer>() {
                #[cfg(feature = "schemars12")]
                if MAX as u128 > u32::MAX as u128 {
                    // There is no const safe way to cast usize to u32, nor to other bigger number.
                    panic!(
                        "`schemars` encodes `maxLength` as u32, so `MAX` must be less than or equal to `u32::MAX`"
                    )
                }

                #[cfg(feature = "borsh15")]
                if MAX as u128 > u32::MAX as u128 {
                    panic!(
                        "`borsh` specifies size of dynamic containers as u32, so `MAX` must be less than or equal to `u32::MAX`"
                    )
                }
            }

            /// Type a compile-time proof for possibly empty vector with max bound.
            pub const fn empty<const MAX: $integer>() -> Empty<MAX> {
                const {
                    serde::<MAX>();
                    Empty::<MAX>(())
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

define_witnesses!(u8, u8);
define_witnesses!(u16, u16);
define_witnesses!(u32, u32);
define_witnesses!(usize, usize);
define_witnesses!(u64, u64);

pub use crate::usize::{Empty, Fixed, NonEmpty, OneOrMore, empty, fixed, non_empty};

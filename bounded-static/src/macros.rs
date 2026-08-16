//! Shared implementation macros.

/// Implements no-op conversions for a `Copy` type.
macro_rules! make_copy_impl {
    ($id:ty) => {
        /// No-op [`ToBoundedStatic`] impl for this `Copy` type.
        impl crate::ToBoundedStatic for $id {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                *self
            }
        }

        /// No-op [`IntoBoundedStatic`] impl for this `Copy` type.
        impl crate::IntoBoundedStatic for $id {
            type Static = Self;

            fn into_static(self) -> Self::Static {
                self
            }
        }
    };
}

pub(crate) use make_copy_impl;

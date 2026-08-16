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

/// Implements clone and no-op conversions for a `Clone` type.
#[allow(unused_macros)]
macro_rules! make_clone_impl {
    ($id:ty) => {
        /// [`ToBoundedStatic`] impl that clones this `Clone` type.
        impl ToBoundedStatic for $id {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                self.clone()
            }
        }

        /// No-op [`IntoBoundedStatic`] impl for this `Clone` type.
        impl IntoBoundedStatic for $id {
            type Static = Self;

            fn into_static(self) -> Self::Static {
                self
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use make_clone_impl;
pub(crate) use make_copy_impl;

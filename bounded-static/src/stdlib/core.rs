use crate::{macros, IntoBoundedStatic, ToBoundedStatic};
use ::core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

/// No-op [`ToBoundedStatic`] impl for converting `&'static str` to `&'static str`.
impl ToBoundedStatic for &'static str {
    type Static = &'static str;

    fn to_static(&self) -> Self::Static {
        self
    }
}

/// No-op [`IntoBoundedStatic`] impl for converting `&'static str` into `&'static str`.
impl IntoBoundedStatic for &'static str {
    type Static = &'static str;

    fn into_static(self) -> Self::Static {
        self
    }
}

macros::make_copy_impl!(bool);
macros::make_copy_impl!(char);
macros::make_copy_impl!(f32);
macros::make_copy_impl!(f64);
macros::make_copy_impl!(usize);
macros::make_copy_impl!(u8);
macros::make_copy_impl!(u16);
macros::make_copy_impl!(u32);
macros::make_copy_impl!(u64);
macros::make_copy_impl!(u128);
macros::make_copy_impl!(isize);
macros::make_copy_impl!(i8);
macros::make_copy_impl!(i16);
macros::make_copy_impl!(i32);
macros::make_copy_impl!(i64);
macros::make_copy_impl!(i128);
macros::make_copy_impl!(NonZeroUsize);
macros::make_copy_impl!(NonZeroU8);
macros::make_copy_impl!(NonZeroU16);
macros::make_copy_impl!(NonZeroU32);
macros::make_copy_impl!(NonZeroU64);
macros::make_copy_impl!(NonZeroU128);
macros::make_copy_impl!(NonZeroIsize);
macros::make_copy_impl!(NonZeroI8);
macros::make_copy_impl!(NonZeroI16);
macros::make_copy_impl!(NonZeroI32);
macros::make_copy_impl!(NonZeroI64);
macros::make_copy_impl!(NonZeroI128);

/// No-op [`ToBoundedStatic`] impl for unit type `()`.
impl ToBoundedStatic for () {
    type Static = ();

    fn to_static(&self) -> Self::Static {}
}

/// No-op [`IntoBoundedStatic`] impl for unit type `()`.
impl IntoBoundedStatic for () {
    type Static = ();

    fn into_static(self) -> Self::Static {}
}

/// Blanket [`ToBoundedStatic`] impl for converting `Option<T>` to `Option<T>: 'static`.
impl<T> ToBoundedStatic for Option<T>
where
    T: ToBoundedStatic,
{
    type Static = Option<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.as_ref().map(ToBoundedStatic::to_static)
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Option<T>` into `Option<T>: 'static`.
impl<T> IntoBoundedStatic for Option<T>
where
    T: IntoBoundedStatic,
{
    type Static = Option<T::Static>;

    fn into_static(self) -> Self::Static {
        self.map(IntoBoundedStatic::into_static)
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `Result<T, E>` to `Result<T, E>: 'static`.
impl<T, E> ToBoundedStatic for Result<T, E>
where
    T: ToBoundedStatic,
    E: ToBoundedStatic,
{
    type Static = Result<T::Static, E::Static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Ok(value) => Ok(value.to_static()),
            Err(err) => Err(err.to_static()),
        }
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Result<T, E>` into `Result<T, E>: 'static`.
impl<T, E> IntoBoundedStatic for Result<T, E>
where
    T: IntoBoundedStatic,
    E: IntoBoundedStatic,
{
    type Static = Result<T::Static, E::Static>;

    fn into_static(self) -> Self::Static {
        match self {
            Ok(value) => Ok(value.into_static()),
            Err(err) => Err(err.into_static()),
        }
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `[T; const N: usize]` to `[T; const N: usize]: 'static`.
impl<T, const N: usize> ToBoundedStatic for [T; N]
where
    T: ToBoundedStatic,
{
    type Static = [T::Static; N];

    fn to_static(&self) -> Self::Static {
        ::core::array::from_fn(|i| self[i].to_static())
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `[T; const N: usize]` into `[T; const N: usize]: 'static`.
impl<T, const N: usize> IntoBoundedStatic for [T; N]
where
    T: IntoBoundedStatic,
{
    type Static = [T::Static; N];

    fn into_static(self) -> Self::Static {
        self.map(IntoBoundedStatic::into_static)
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting tuples `(T1, T2, ...)` to `(T1, T2, ..): 'static`.
macro_rules! tuple_to_static {
    () => ();
    ($($name:ident,)+) => {
        tuple_to_static! (
            @gen $($name,)+,
            concat!(
                "Blanket [`ToBoundedStatic`] impl for converting tuple `",
                stringify!(($($name,)+)), "` to `", stringify!(($($name,)+)), ": 'static `"
            )
        );
    };
    (@gen $($name:ident,)+, $doc:expr) => {
        #[doc = $doc]
        impl<$($name: ToBoundedStatic),+> ToBoundedStatic for ($($name,)+) {
            type Static = ($($name::Static,)+);
            #[allow(non_snake_case)]
            fn to_static(&self) -> Self::Static {
                let ($(ref $name,)+) = *self;
                ($($name.to_static(),)+)
            }
        }
        tuple_to_static! {@peel $($name,)+ }
    };
    (@peel $name:ident, $($other:ident,)*) => {tuple_to_static! { $($other,)* }};
}

/// Blanket [`IntoBoundedStatic`] impl for converting tuples `(T1, T2, ...)` into `(T1, T2, ..): 'static`.
macro_rules! tuple_into_static {
    () => ();
    ($($name:ident,)+) => {
        tuple_into_static! (
            @gen $($name,)+,
            concat!(
                "Blanket [`IntoBoundedStatic`] impl for converting tuple `",
                stringify!(($($name,)+)), "` into `", stringify!(($($name,)+)), ": 'static `"
            )
        );
    };
    (@gen $($name:ident,)+, $doc:expr) => {
        #[doc = $doc]
        impl<$($name: IntoBoundedStatic),+> IntoBoundedStatic for ($($name,)+) {
            type Static = ($($name::Static,)+);
            #[allow(non_snake_case)]
            fn into_static(self) -> Self::Static {
                let ($($name,)+) = self;
                ($($name.into_static(),)+)
            }
        }
        tuple_into_static! {@peel $($name,)+ }
    };
    (@peel $name:ident, $($other:ident,)*) => {tuple_into_static! { $($other,)* }};
}

tuple_to_static! { T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1, T0, }
tuple_into_static! { T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1, T0, }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test_case(false; "bool")]
    #[test_case('a'; "char")]
    #[test_case(0.0f32; "f32")]
    #[test_case(0.0f64; "f64")]
    #[test_case(0usize; "usize")]
    #[test_case(0u8; "u8")]
    #[test_case(0u16; "u16")]
    #[test_case(0u32; "u32")]
    #[test_case(0u64; "u64")]
    #[test_case(0u128; "u128")]
    #[test_case(0isize; "isize")]
    #[test_case(0i8; "i8")]
    #[test_case(0i16; "i16")]
    #[test_case(0i32; "i32")]
    #[test_case(0i64; "i64")]
    #[test_case(0i128; "i128")]
    #[allow(clippy::needless_pass_by_value)]
    fn test_primitive<T: ToBoundedStatic>(t: T) {
        ensure_static(t.to_static());
    }

    #[test_case(NonZeroUsize::new(1); "usize")]
    #[test_case(NonZeroU8::new(1); "u8")]
    #[test_case(NonZeroU16::new(1); "u16")]
    #[test_case(NonZeroU32::new(1); "u32")]
    #[test_case(NonZeroU64::new(1); "u64")]
    #[test_case(NonZeroU128::new(1); "u128")]
    #[test_case(NonZeroIsize::new(1); "isize")]
    #[test_case(NonZeroI8::new(1); "i8")]
    #[test_case(NonZeroI16::new(1); "i16")]
    #[test_case(NonZeroI32::new(1); "i32")]
    #[test_case(NonZeroI64::new(1); "i64")]
    #[test_case(NonZeroI128::new(1); "i128")]
    #[allow(clippy::needless_pass_by_value)]
    fn test_non_zero<T: ToBoundedStatic>(t: T) {
        ensure_static(t.to_static());
    }

    #[test]
    fn test_unit() {
        #[allow(clippy::unit_arg)]
        ensure_static(().to_static());
    }

    #[test]
    fn test_str() {
        let s = "";
        let to_static = s.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_none() {
        let value: Option<u32> = None;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_some() {
        let value: Option<u32> = Some(32);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_result() {
        #[derive(Clone)]
        struct MyError;
        #[allow(clippy::unnecessary_wraps)]
        fn foo_ok() -> Result<(), MyError> {
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn foo_err() -> Result<(), MyError> {
            Err(MyError)
        }
        impl ToBoundedStatic for MyError {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                self.clone()
            }
        }
        let ok_result = foo_ok();
        ensure_static(ok_result.to_static());
        assert!(ok_result.is_ok());
        let err_result = foo_err();
        ensure_static(err_result.to_static());
        assert!(err_result.is_err());
    }

    #[test]
    fn test_array() {
        let arr = ["test"];
        ensure_static(arr.to_static());
    }

    #[test]
    fn test_tuple2() {
        let tuple = ("test", 32);
        ensure_static(tuple.to_static());
    }

    #[test]
    fn test_tuple11() {
        let tuple = (
            (),
            '1',
            "2",
            3_i32,
            4_usize,
            5_isize,
            6.0_f64,
            ["7"],
            Some(8),
            9,
            (10,),
            false,
        );
        ensure_static(tuple.to_static());
    }
}

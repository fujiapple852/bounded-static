#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_const_for_fn)]
#![forbid(unsafe_code)]

use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// A trait for converting `&T` to an owned `T` such that `T: 'static`.
///
/// As described in the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):
///
/// > `T: 'static'` should be read as "`T` is bounded by a `'static` lifetime" not "`T` has a `'static` lifetime".
pub trait ToBoundedStatic {
    /// The target type is bounded by the `'static` lifetime.
    type Static: 'static;

    /// Convert an `&T` to an owned `T` such that `T: 'static`.
    #[must_use = "converting is often expensive and is not expected to have side effects"]
    fn to_static(&self) -> Self::Static;
}

/// A trait for converting an owned `T` into an owned `T` such that `T: 'static`.
///
/// As described in the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):
///
/// > `T: 'static'` should be read as "`T` is bounded by a `'static` lifetime" not "`T` has a `'static` lifetime".
pub trait IntoBoundedStatic {
    /// The target type is bounded by the `'static` lifetime.
    type Static: 'static;

    /// Convert an owned `T` into an owned `T` such that `T: 'static`.
    #[must_use = "converting is often expensive and is not expected to have side effects"]
    fn into_static(self) -> Self::Static;
}

/// Blanket [`ToBoundedStatic`] impl for converting `Cow<'a, T: ?Sized>` to `Cow<'static, T: ?Sized>`.
impl<T> ToBoundedStatic for Cow<'_, T>
where
    T: 'static + ToOwned + ?Sized,
{
    type Static = Cow<'static, T>;

    fn to_static(&self) -> Self::Static {
        Cow::Owned(self.clone().into_owned())
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Cow<'a, T: ?Sized>` into `Cow<'static, T: ?Sized>`.
impl<T> IntoBoundedStatic for Cow<'_, T>
where
    T: 'static + ToOwned + ?Sized,
{
    type Static = Cow<'static, T>;

    fn into_static(self) -> Self::Static {
        Cow::Owned(self.into_owned())
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `Vec<T>` to `Vec<T>: 'static`.
impl<T> ToBoundedStatic for Vec<T>
where
    T: ToBoundedStatic,
{
    type Static = Vec<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Vec<T>` into `Vec<T>: 'static`.
impl<T> IntoBoundedStatic for Vec<T>
where
    T: IntoBoundedStatic,
{
    type Static = Vec<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `HashMap<K, V>` to `HashMap<K, V>: 'static`.
impl<K, V, S: BuildHasher> ToBoundedStatic for HashMap<K, V, S>
where
    K: ToBoundedStatic,
    K::Static: Eq + Hash,
    V: ToBoundedStatic,
{
    type Static = HashMap<K::Static, V::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter()
            .map(|(k, v)| (k.to_static(), v.to_static()))
            .collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for for converting `HashMap<K, V>` into `HashMap<K, V>: 'static`.
impl<K, V, S: BuildHasher> IntoBoundedStatic for HashMap<K, V, S>
where
    K: IntoBoundedStatic,
    K::Static: Eq + Hash,
    V: IntoBoundedStatic,
{
    type Static = HashMap<K::Static, V::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(|(k, v)| (k.into_static(), v.into_static()))
            .collect()
    }
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

/// Blanket [`ToBoundedStatic`] impl for converting `Box<T>` to `Box<T>: 'static`.
impl<T> ToBoundedStatic for Box<T>
where
    T: ToBoundedStatic,
{
    type Static = Box<T::Static>;

    fn to_static(&self) -> Self::Static {
        Box::new(self.as_ref().to_static())
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Box<T>` into `Box<T>: 'static`.
impl<T> IntoBoundedStatic for Box<T>
where
    T: IntoBoundedStatic,
{
    type Static = Box<T::Static>;

    fn into_static(self) -> Self::Static {
        Box::new((*self).into_static())
    }
}

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

/// [`ToBoundedStatic`] impl for `String`.
impl ToBoundedStatic for String {
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

/// No-op [`IntoBoundedStatic`] impl for `String`.
impl IntoBoundedStatic for String {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

/// No-op [`ToBoundedStatic`] and [`IntoBoundedStatic`] impls for primitive types.
macro_rules! make_primitive_impl {
    ($id:ident) => {
        impl ToBoundedStatic for $id {
            type Static = $id;

            fn to_static(&self) -> Self::Static {
                *self
            }
        }
        impl IntoBoundedStatic for $id {
            type Static = $id;

            fn into_static(self) -> Self::Static {
                self
            }
        }
    };
}

make_primitive_impl!(bool);
make_primitive_impl!(f32);
make_primitive_impl!(f64);
make_primitive_impl!(u8);
make_primitive_impl!(u16);
make_primitive_impl!(u32);
make_primitive_impl!(u64);
make_primitive_impl!(u128);
make_primitive_impl!(i8);
make_primitive_impl!(i16);
make_primitive_impl!(i32);
make_primitive_impl!(i64);
make_primitive_impl!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_static<T: 'static>(_: T) {}

    #[test]
    fn test_bool() {
        ensure_static(false.to_static());
    }

    #[test]
    fn test_f32() {
        ensure_static(0.0f32.to_static());
    }

    #[test]
    fn test_f64() {
        ensure_static(0.0f64.to_static());
    }

    #[test]
    fn test_u8() {
        ensure_static(0u8.to_static());
    }

    #[test]
    fn test_u16() {
        ensure_static(0u16.to_static());
    }

    #[test]
    fn test_u32() {
        ensure_static(0u32.to_static());
    }

    #[test]
    fn test_u64() {
        ensure_static(0u64.to_static());
    }

    #[test]
    fn test_u128() {
        ensure_static(0u128.to_static());
    }

    #[test]
    fn test_i8() {
        ensure_static(0i8.to_static());
    }

    #[test]
    fn test_i16() {
        ensure_static(0i16.to_static());
    }

    #[test]
    fn test_i32() {
        ensure_static(0i32.to_static());
    }

    #[test]
    fn test_i64() {
        ensure_static(0i64.to_static());
    }

    #[test]
    fn test_i128() {
        ensure_static(0i128.to_static());
    }

    #[test]
    fn test_str() {
        let s = "";
        let to_static = s.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_string() {
        let s = String::from("");
        let to_static = s.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_borrowed_str() {
        let s = String::from("");
        let to_static = Cow::from(&s).to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_owned_string() {
        let s = String::from("");
        let to_static = Cow::from(s).to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_to_static() {
        let s = String::from("");
        let s_cow: Cow<'_, str> = Cow::Borrowed(&s);
        let s1_cow_owned: Cow<'_, str> = s_cow.to_static();
        let s2_cow_owned: Cow<'_, str> = Cow::Owned(s_cow.into_owned());
        assert_eq!(s1_cow_owned, s2_cow_owned);
    }

    #[test]
    fn test_cow_into_static() {
        let s = String::from("");
        let s_cow: Cow<'_, str> = Cow::Borrowed(&s);
        let s1_cow_owned: Cow<'_, str> = s_cow.clone().into_static();
        let s2_cow_owned: Cow<'_, str> = Cow::Owned(s_cow.into_owned());
        assert_eq!(s1_cow_owned, s2_cow_owned);
    }

    #[test]
    fn test_vec1() {
        let s = String::from("");
        let value = vec![Cow::from(&s)];
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_vec2() {
        let s = String::from("");
        let value = vec![Cow::from(&s), Cow::from(s.as_str())];
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap1() {
        let k = String::from("key");
        let v = String::from("value");
        let value = HashMap::from([(Cow::from(&k), Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap2() {
        let k = "key";
        let v = String::from("value");
        let value = HashMap::from([(k, Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap3() {
        let k = String::from("key");
        let v = 0i16;
        let value = HashMap::from([(Cow::from(&k), v)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_none() {
        let value: Option<Cow<'_, str>> = None;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_some() {
        let s = String::from("");
        let value = Some(Cow::from(&s));
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_box() {
        let s = String::from("");
        let value = Box::new(s);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_box_cow() {
        let s = String::from("");
        let value = Box::new(Cow::from(&s));
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_box_vec_cow() {
        let s = String::from("");
        let value = Box::new(vec![Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_vec_box_cow() {
        let s = String::from("");
        let value = vec![Box::new(Cow::from(&s))];
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_box() {
        let s = String::from("");
        let boxed = Box::new(s);
        let value = Cow::Borrowed(&boxed);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_struct() {
        #[derive(Copy, Clone)]
        struct Foo {}
        impl ToBoundedStatic for Foo {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                *self
            }
        }
        let foo = Foo {};
        let value = Cow::Borrowed(&foo);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_struct_of_cow() {
        #[derive(Clone)]
        struct Foo<'a> {
            foo: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    foo: self.foo.to_static(),
                }
            }
        }
        let s = String::from("");
        let foo = Foo { foo: Cow::from(&s) };
        let value = Cow::Borrowed(&foo);
        // TODO need to `into_owned()` here
        let to_static = value.into_owned().to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_cow() {
        let s = String::from("");
        let value1: Cow<'_, str> = Cow::Borrowed(&s);
        let value2: Cow<'_, Cow<'_, str>> = Cow::Borrowed(&value1);
        // TODO need to `into_owned()` here
        let to_static = value2.into_owned().to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_cow_borrowed_str() {
        struct Foo<'a> {
            foo: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    foo: self.foo.to_static(),
                }
            }
        }
        let s = String::from("");
        let foo = Foo { foo: Cow::from(&s) };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_cow_owned_string() {
        struct Foo<'a> {
            foo: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    foo: self.foo.to_static(),
                }
            }
        }
        let s = String::from("");
        let foo = Foo { foo: Cow::from(s) };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_multi() {
        #[derive(Clone)]
        struct Foo<'a> {
            bar: Cow<'a, str>,
            baz: Vec<Cow<'a, str>>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    bar: self.bar.to_static(),
                    baz: self.baz.to_static(),
                }
            }
        }
        let s = String::from("");
        let foo = Foo {
            bar: Cow::from(&s),
            baz: vec![Cow::from(&s)],
        };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_mixed() {
        struct Foo<'a> {
            prim: u64,
            borrowed_str: &'static str,
            owned_str: String,
            cow_str: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    prim: self.prim.to_static(),
                    borrowed_str: self.borrowed_str.to_static(),
                    owned_str: self.owned_str.to_static(),
                    cow_str: self.cow_str.to_static(),
                }
            }
        }
        let s = String::from("");
        let foo = Foo {
            prim: 0,
            borrowed_str: "",
            owned_str: s.clone(),
            cow_str: Cow::from(&s),
        };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }
}

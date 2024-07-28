#![doc(html_root_url = "https://docs.rs/bounded-static/0.8.0")]
//! Provides the [`ToBoundedStatic`] and [`IntoBoundedStatic`] traits and [`ToStatic`] derive macro.
//!
//! As described in the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):
//!
//! > `T: 'static` should be read as "`T` is bounded by a `'static` lifetime" not "`T` has a `'static` lifetime".
//!
//! The traits [`ToBoundedStatic`] and [`IntoBoundedStatic`] can be used to convert any suitable `T` and `&T` to an
//! owned `T` such that `T: 'static`.  Both traits define an associated type which is bounded by `'static` and provide
//! a method to convert to that bounded type:
//!
//! ```rust
//! pub trait ToBoundedStatic {
//!     type Static: 'static;
//!
//!     fn to_static(&self) -> Self::Static;
//! }
//!
//! pub trait IntoBoundedStatic {
//!     type Static: 'static;
//!
//!     fn into_static(self) -> Self::Static;
//! }
//! ```
//!
//! Implementations of [`ToBoundedStatic`] and [`IntoBoundedStatic`] are provided for the following `core` types:
//!
//! - [`primitive`](core::primitive) (no-op conversions)
//! - [`array`](array)
//! - [`tuple`](tuple)
//! - [`Option`]
//! - [`Result`]
//!
//! Additional implementations are available by enabling the following features:
//!
//! - `alloc` for common types from the `alloc` crate:
//!   - [Cow](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html)
//!   - [String](https://doc.rust-lang.org/alloc/string/struct.String.html)
//!   - [Vec](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)
//!   - [Box](https://doc.rust-lang.org/alloc/boxed/struct.Box.html)
//!
//! - `collections` for all collection types in the `alloc` crate:
//!   - [BinaryHeap](https://doc.rust-lang.org/alloc/collections/binary_heap/struct.BinaryHeap.html)
//!   - [BTreeMap](https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html)
//!   - [BTreeSet](https://doc.rust-lang.org/alloc/collections/btree_set/struct.BTreeSet.html)
//!   - [LinkedList](https://doc.rust-lang.org/alloc/collections/linked_list/struct.LinkedList.html)
//!   - [VecDeque](https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html)
//!
//! - `std` for additional types from `std`:
//!   - [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
//!   - [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
//!   - [RandomState](https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html)
//!
//! Note that `collections`, `alloc` and `std` are enabled be default.
//!
//! Additional implementations for 3rd party types are available by enabling the following features:
//!
//! - `smol_str` for [`SmolStr`](https://docs.rs/smol_str/0.2.2/smol_str/struct.SmolStr.html)
//! - `smallvec` for [`SmallVec`](https://docs.rs/smallvec/1.13.2/smallvec/struct.SmallVec.html)
//! - `ahash` for:
//!     - [`RandomState`](https://docs.rs/ahash/0.8.6/ahash/random_state/struct.RandomState.html)
//!     - [`AHashMap`](https://docs.rs/ahash/0.8.6/ahash/struct.AHashMap.html)
//!     - [`AHashSet`](https://docs.rs/ahash/0.8.6/ahash/struct.AHashSet.html)
//! - `chrono` for:
//!     - [`DateTime`](https://docs.rs/chrono/0.4.38/chrono/struct.DateTime.html)
//!     - [`FixedOffset`](https://docs.rs/chrono/0.4.38/chrono/struct.FixedOffset.html)
//!     - [`Months`](https://docs.rs/chrono/0.4.38/chrono/struct.Months.html)
//!     - [`TimeDelta`](https://docs.rs/chrono/0.4.38/chrono/struct.TimeDelta.html)
//!     - [`Utc`](https://docs.rs/chrono/0.4.38/chrono/struct.Utc.html)
//!     - [`Month`](https://docs.rs/chrono/0.4.38/chrono/enum.Month.html)
//!     - [`Weekday`](https://docs.rs/chrono/0.4.38/chrono/enum.Weekday.html)
//!     - [`Days`](https://docs.rs/chrono/0.4.38/chrono/naive/struct.Days.html)
//!     - [`IsoWeek`](https://docs.rs/chrono/0.4.38/chrono/naive/struct.IsoWeek.html)
//!     - [`NaiveDate`](https://docs.rs/chrono/0.4.38/chrono/naive/struct.NaiveDate.html)
//!     - [`NaiveDateTime`](https://docs.rs/chrono/0.4.38/chrono/naive/struct.NaiveDateTime.html)
//!     - [`NaiveTime`](https://docs.rs/chrono/0.4.38/chrono/naive/struct.NaiveTime.html)
//! - `chrono-clock` for:
//!    - [`Local`](https://docs.rs/chrono/0.4.38/chrono/struct.Local.html)
//! - `jiff` for:
//!     - [`Zoned`](https://docs.rs/jiff/0.2.25/jiff/struct.Zoned.html)
//!     - [`Timestamp`](https://docs.rs/jiff/0.2.25/jiff/struct.Timestamp.html)
//!     - [`Span`](https://docs.rs/jiff/0.2.25/jiff/struct.Span.html)
//!     - [`SpanFieldwise`](https://docs.rs/jiff/0.2.25/jiff/struct.SpanFieldwise.html)
//!     - [`SignedDuration`](https://docs.rs/jiff/0.2.25/jiff/struct.SignedDuration.html)
//!     - [`DateTime`](https://docs.rs/jiff/0.2.25/jiff/civil/struct.DateTime.html)
//!     - [`Date`](https://docs.rs/jiff/0.2.25/jiff/civil/struct.Date.html)
//!     - [`Time`](https://docs.rs/jiff/0.2.25/jiff/civil/struct.Time.html)
//!     - [`ISOWeekDate`](https://docs.rs/jiff/0.2.25/jiff/civil/struct.ISOWeekDate.html)
//!     - [`Era`](https://docs.rs/jiff/0.2.25/jiff/civil/enum.Era.html)
//!     - [`Weekday`](https://docs.rs/jiff/0.2.25/jiff/civil/enum.Weekday.html)
//!     - [`TimeZone`](https://docs.rs/jiff/0.2.25/jiff/tz/struct.TimeZone.html)
//!     - [`Offset`](https://docs.rs/jiff/0.2.25/jiff/tz/struct.Offset.html)
//!     - [`Dst`](https://docs.rs/jiff/0.2.25/jiff/tz/enum.Dst.html)
//!     - [`AmbiguousZoned`](https://docs.rs/jiff/0.2.25/jiff/tz/struct.AmbiguousZoned.html)
//!     - [`AmbiguousTimestamp`](https://docs.rs/jiff/0.2.25/jiff/tz/struct.AmbiguousTimestamp.html)
//!     - [`AmbiguousOffset`](https://docs.rs/jiff/0.2.25/jiff/tz/enum.AmbiguousOffset.html)
//!
//! # Examples
//!
//! Given a structure which can be borrow or owned and a function which requires its argument is bounded by the
//! `'static` lifetime:
//!
//! ```rust
//! # use std::borrow::Cow;
//! struct Foo<'a> {
//!     bar: Cow<'a, str>,
//!     baz: Vec<Cow<'a, str>>,
//! }
//!
//! fn ensure_static<T: 'static>(_: T) {}
//! ```
//!
//! We can implement [`ToBoundedStatic`] (and [`IntoBoundedStatic`]) for `Foo<'_>`:
//!
//! ```rust
//! # use std::borrow::Cow;
//! # use bounded_static::ToBoundedStatic;
//! struct Foo<'a> {
//!     bar: Cow<'a, str>,
//!     baz: Vec<Cow<'a, str>>,
//! }
//! impl ToBoundedStatic for Foo<'_> {
//!     type Static = Foo<'static>;
//!
//!     fn to_static(&self) -> Self::Static {
//!         Foo { bar: self.bar.to_static(), baz: self.baz.to_static() }
//!     }
//! }
//! ```
//!
//! This allows it to be converted to an owned representation such that it is now bounded by `'static`:
//!
//! ```rust
//! # use std::borrow::Cow;
//! # use bounded_static::ToBoundedStatic;
//! # struct Foo<'a> {
//! #     bar: Cow<'a, str>,
//! #     baz: Vec<Cow<'a, str>>,
//! # }
//! # impl ToBoundedStatic for Foo<'_> {
//! #     type Static = Foo<'static>;
//! #
//! #     fn to_static(&self) -> Self::Static {
//! #         Foo { bar: self.bar.to_static(), baz: self.baz.to_static() }
//! #     }
//! # }
//! fn test() {
//!     # fn ensure_static<T: 'static>(_: T) {}
//!     let s = String::from("data");
//!     let foo = Foo { bar: Cow::from(&s), baz: vec![Cow::from(&s)] };
//!     let to_static = foo.to_static();
//!     ensure_static(to_static);
//! }
//! ```
//!
//! # Derive
//!
//! These traits may be automatically derived for any `struct` or `enum` that can be converted to a form that is
//! bounded by `'static` by using the [`ToStatic`] macro. It support all `struct` flavors (unit, named & unnamed),
//! all `enum` variant flavors (unit, named & unnamed).  It does not currently support `union`.
//!
//! To use the [`ToStatic`] macro you must enable the `derive` feature:
//!
//! ```yaml
//! bounded-static = { version = "0.8.0", features = [ "derive" ] }
//! ```
//!
//! # Examples
//!
//! ```rust
//! # use std::borrow::Cow;
//! # use std::collections::HashMap;
//! # use bounded_static::ToStatic;
//! /// Named field struct
//! #[derive(ToStatic)]
//! struct Foo<'a> {
//!     aaa: Cow<'a, str>,
//!     bbb: &'static str,
//!     ccc: Baz<'a>,
//! }
//!
//! /// Unnamed field struct
//! #[derive(ToStatic)]
//! struct Bar<'a, 'b>(u128, HashMap<Cow<'a, str>, Cow<'b, str>>);
//!
//! /// Unit struct
//! #[derive(ToStatic)]
//! struct Qux;
//!
//! #[derive(ToStatic)]
//! enum Baz<'a> {
//!     First(String, usize, Vec<Cow<'a, str>>),
//!     Second { fst: u32, snd: &'static str },
//!     Third,
//! }
//! ```
//!
//! [`ToStatic`]: https://docs.rs/bounded-static-derive/0.8.0/bounded_static_derive/derive.ToStatic.html
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_const_for_fn)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "derive")]
/// Re-export for the custom derive macro `ToStatic`.
pub use bounded_static_derive::ToStatic;

/// A trait for converting `&T` to an owned `T` such that `T: 'static`.
///
/// See the module level documentation for details.
pub trait ToBoundedStatic {
    /// The target type is bounded by the `'static` lifetime.
    type Static: 'static;

    /// Convert an `&T` to an owned `T` such that `T: 'static`.
    #[must_use = "converting is often expensive and is not expected to have side effects"]
    fn to_static(&self) -> Self::Static;
}

/// A trait for converting an owned `T` into an owned `T` such that `T: 'static`.
///
/// See the module level documentation for details.
pub trait IntoBoundedStatic {
    /// The target type is bounded by the `'static` lifetime.
    type Static: 'static;

    /// Convert an owned `T` into an owned `T` such that `T: 'static`.
    #[must_use = "converting is often expensive and is not expected to have side effects"]
    fn into_static(self) -> Self::Static;
}

mod macros;
mod stdlib;
mod third_party;

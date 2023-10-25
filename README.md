![ci](https://github.com/fujiapple852/bounded-static/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/bounded-static/badge.svg)](https://docs.rs/bounded-static/0.7.0)
[![Crate](https://img.shields.io/crates/v/bounded-static.svg)](https://crates.io/crates/bounded-static/0.7.0)

# Bounded Static
This crate defines the [`ToBoundedStatic`](https://docs.rs/bounded-static/0.7.0/bounded_static/trait.ToBoundedStatic.html) 
and [`IntoBoundedStatic`](https://docs.rs/bounded-static/0.7.0/bounded_static/trait.IntoBoundedStatic.html) traits, 
the [`ToStatic`](https://docs.rs/bounded-static/0.7.0/bounded_static/derive.ToStatic.html) macro and provides impls 
for common types.  This crate has zero-dependencies, is `no_std` friendly and 
forbids `unsafe` code.

As described in
the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):

> `T: 'static` should be read as _"`T` is bounded by a `'static` lifetime"_ not _"`T` has a `'static` lifetime"_.

The traits `ToBoundedStatic` and `IntoBoundedStatic` can be used to convert any suitable `T` and `&T` to an
owned `T` such that `T: 'static`.  Both traits define an associated type which is bounded by `'static` and provide a 
method to convert to that bounded type.

The macros `ToStatic` can be used to automatically derive `ToBoundedStatic` and `IntoBoundedStatic` for any `struct` 
or `enum` that can be converted to a form that is bounded by `'static`.

Refer to the crate [`documentation`](https://docs.rs/bounded-static/0.7.0/bounded_static) for details and examples.

## FAQ

### When is this useful?

This is useful for data structures which directly or indirectly contain `Cow<T>` types that must be supplied to 
a function which requires the `'static` bound (i.e. [`std::thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)):

```rust
#[derive(Debug, PartialEq, ToStatic)]
struct Foo<'a> {
    foo: Cow<'a, str>,
    bar: Vec<Bar<'a>>
}
#[derive(Debug, PartialEq, ToStatic)]
enum Bar<'a> {
    First,
    Second(Cow<'a, str>),
}

fn main() {
    let value = String::from("data");
    let foo = Foo {
        foo: Cow::from(&value),
        bar: vec![Bar::First, Bar::Second(Cow::from(&value))]
    };
    let foo_static = foo.into_static();
    std::thread::spawn(move || {
        assert_eq!(foo_static.foo, "data");
        assert_eq!(foo_static.bar, vec![Bar::First, Bar::Second("data".into())])
    }).join().unwrap();
}
```

### How does this differ from the `ToOwned` trait?

The [`ToOwned`](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) trait defines an associated type `Owned` which
is not bound by `'static` and therefore the follow will not compile:

```rust
use std::borrow::Cow;

fn main() {
    #[derive(Clone)]
    struct Foo<'a> {
        foo: Cow<'a, str>,
    }

    fn ensure_static<T: 'static>(_: T) {}

    let s = String::from("data");
    let foo = Foo { foo: Cow::from(&s) };
    ensure_static(foo.to_owned())
}
```

Results in the following error:

```
error[E0597]: `s` does not live long enough
  --> src/lib.rs:12:36
   |
12 |     let foo = Foo { foo: Cow::from(&s) };
   |                          ----------^^-
   |                          |         |
   |                          |         borrowed value does not live long enough
   |                          argument requires that `s` is borrowed for `'static`
13 |     ensure_static(foo.to_owned())
14 | }
   | - `s` dropped here while still borrowed
```

Replacing `Clone` with `ToStatic` and using `into_static()` (or `to_static()` as needed) allows the example to compile:

```rust
use std::borrow::Cow;

fn main() {

    #[derive(ToStatic)]
    struct Foo<'a> {
        foo: Cow<'a, str>,
    }

    fn ensure_static<T: 'static>(_: T) {}

    let s = String::from("data");
    let foo = Foo { foo: Cow::from(&s) };
    ensure_static(foo.into_static())
}
```

## License

`bounded-static` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2022
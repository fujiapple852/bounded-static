![ci](https://github.com/fujiapple852/bounded_static/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/bounded-static/badge.svg)](https://docs.rs/bounded-static)
[![Crate](https://img.shields.io/crates/v/bounded-static.svg)](https://crates.io/crates/bounded-static)

# Bounded Static
An experimental crate that defines the `ToBoundedStatic` and `IntoBoundedStatic` traits, the `ToStatic` macro and 
provides impls for common types.  This crate has zero-dependencies, is `no_std` friendly and forbids `unsafe` code.

As described in
the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):

> `T: 'static` should be read as _"`T` is bounded by a `'static` lifetime"_ not _"`T` has a `'static` lifetime"_.

The traits `ToBoundedStatic` and `IntoBoundedStatic` can be used to convert any suitable `T` and `&T` to an
owned `T` such that `T: 'static`.  Both traits define an associated type which is bounded by `'static` and provide a 
method to convert to that bounded type.

The macros `ToStatic` can be used to automatically derive `ToBoundedStatic` and `IntoBoundedStatic` for any `struct` 
or `enum` that can be converted to a form that is bounded by `'static`.

## Status

Experimental

## FAQ

### Why is this useful?

This is mainly useful when dealing with nested `Cow<T>` data structures.

### How does this differ from the `ToOwned` trait?

The [`ToOwned`](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) trait defines an associated type `Owned` which
is bound by [`Borrow<Self>`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) but not by `'static`.  Therefore,
the follow will not compile:

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

Results in:

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

## License

`bounded-static` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2022
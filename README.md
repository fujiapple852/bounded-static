# Bounded Static

An experimental crate that defines the `ToStaticBounded` and `IntoStaticBounded` traits and provides impls for common
types from the Rust standard library.

As described in
the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):

> `T: 'static` is some `T` that can be safely held indefinitely long, including up until the end of the program.
> `T: 'static` includes all `&'static T` however it also includes all owned types, like `String`, `Vec`, etc.  The
> owner of some data is guaranteed that data will never get invalidated as long as the owner holds onto it,
> therefore the owner can safely hold onto the data indefinitely long, including up until the end of the program.
>
> `T: 'static` should be read as _"`T` is bounded by a `'static` lifetime"_ not _"`T` has a `'static` lifetime"_.

The traits `ToStaticBounded` and `IntoStaticBounded` each define an associated type which is bounded by `'static` and 
provide a method to convert to that bounded type:

```rust
pub trait ToStaticBounded {
    type Static: 'static;
    
    fn to_static(&self) -> Self::Static;
}

pub trait IntoStaticBounded {
    type Static: 'static;

    fn into_static(self) -> Self::Static;
}
```

## Examples

### Converting a `Cow`

Converting a borrowed `Cow<str>` (`Cow::Borrowed`) to an owned `Cow<str>` (`Cow::Owned`):

```rust
fn main() {
    let s = String::from("text");
    let s_cow: Cow<str> = Cow::Borrowed(&s);
    let s_cow_owned: Cow<str> = s_cow.into_static();
}
```

This is equivalent to:

```rust
fn main() {
    let s = String::from("text");
    let s_cow: Cow<str> = Cow::Borrowed(&s);
    let s_cow_owned: Cow<str> = Cow::Owned(s_cow.into_owned());
}
```

If the `Cow` should not be consumed then `to_static()` can be used instead:

```rust
fn main() {
    let s = String::from("text");
    let s_cow: Cow<str> = Cow::Borrowed(&s);
    let s_cow_owned: Cow<str> = s_cow.to_static();
}
```

### Converting a `struct`

Given a structure which can borrow:

```rust
struct Foo<'a> {
    bar: Cow<'a, str>,
    baz: Vec<Cow<'a, str>>,
}
```

And a function which requires its argument is bounded by the `'static` lifetime:

```rust
fn ensure_static<T: 'static>(_: T) {}
```

We can impl `ToStaticBounded` for `Foo<'_>`:

```rust
impl ToStaticBounded for Foo<'_> {
    type Static = Foo<'static>;

    fn to_static(&self) -> Self::Static {
        Foo { bar: self.bar.to_static(), baz: self.baz.to_static() }
    }
}
```

Such that:

```rust
#[test]
fn test() {
    let s = String::from("data");
    let foo = Foo { bar: Cow::from(&s), baz: vec![Cow::from(&s)] };
    let to_static = foo.to_static();
    ensure_static(to_static);
}
```

### Deriving `ToStaticBounded` and `IntoStaticBounded`

Both the `ToStaticBounded` and the `IntoStaticBounded` traits may be automatically derived using the custom derive
marcos provided in the `bounded_static_derive` crate:

```rust
#[derive(bounded_static_derive::ToStaticBounded)]
struct Foo<'a> {
    bar: Cow<'a, str>,
    baz: Vec<Cow<'a, str>>,
}
```
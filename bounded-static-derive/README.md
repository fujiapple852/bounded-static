[![Documentation](https://docs.rs/bounded-static-derive/badge.svg)](https://docs.rs/bounded-static-derive/0.3.0)
[![Crate](https://img.shields.io/crates/v/bounded-static-derive.svg)](https://crates.io/crates/bounded-static-derive/0.3.0)

# Bounded Static Derive

This crate provides the `ToStatic` macro which can be used to derive implementations of
the [`ToBoundedStatic`](https://docs.rs/bounded-static/0.3.0/bounded_static/trait.ToBoundedStatic.html) and
[`IntoBoundedStatic`](https://docs.rs/bounded-static/0.3.0/bounded_static/trait.IntoBoundedStatic.html) traits for all `struct`and `enum`
that can be converted to a form that is bounded by `'static`.

The `ToStatic` macro should be used via the [`bounded-static`](https://docs.rs/bounded-static/0.3.0/bounded_static) crate rather
than using this crate directly.

```yaml
bounded-static = { version = "0.3.0", features = [ "derive" ] }
```

## License

`bounded-static-derive` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2022
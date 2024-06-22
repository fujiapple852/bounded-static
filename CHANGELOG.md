# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [bounded-static-0.8.0] & [bounded-static-derive-0.8.0] - 2024-06-23

### Added

- Added optional support for 3rd party `chrono` crate (by [@xkikeg](https://github.com/xkikeg))

This change ([#46](https://github.com/fujiapple852/bounded-static/pull/46)) adds support for types from
the `chrono` crate via the `chrono` and `chrono-clock` feature flags.

### Changed

- Increased MSRV to `1.64`
- Updated `ahash`, `smol_str` and `smallvec` to the latest versions

## [bounded-static-0.7.0] & [bounded-static-derive-0.7.0] - 2023-10-25

### Changed

- Increased MSRV to `1.61`

## [bounded-static-0.6.0] & [bounded-static-derive-0.6.0] - 2023-10-20

### Added

- Added support for custom `RandomState` and `aHash` (by [@zhu-he](https://github.com/zhu-he))

This change ([#62](https://github.com/fujiapple852/bounded-static/pull/62)) adds support for using a
custom `RandomState` with the stdlib `HashMap` and `HashSet` types.

It also adds support for the 3rd party `AHashMap`, `AHashSet` and `RandomState` types from the `ahash` crate via
the `ahash` feature flag.

## [bounded-static-0.5.0] & [bounded-static-derive-0.5.0] - 2023-04-29

### Changed

- Increased MSRV to `1.60` and updated all dependency versions

## [bounded-static-0.4.0] & [bounded-static-derive-0.4.0] - 2022-06-08

### Added

- Added support for non-zero integer types (by [@jakoschiko](https://github.com/jakoschiko))

## [bounded-static-0.3.0] & [bounded-static-derive-0.3.0] - 2022-03-10

### Added

- Added support for tuples of up to 12 elements
- Added optional support for 3rd party `smartstring::SmartString`
- Added optional support for 3rd party `smallvec::SmallVec`
- Added optional support for 3rd party `smol_str::SmolStr`
- Added `Result` and `array` to the list of documented blanket implementation

### Changed

- Refactored repo and crate directories to `bounded-static` and `bounded-static-derive` to match crate names

### Fixed

- Fixed broken crate and documentation links

## [bounded-static-0.2.1] & [bounded-static-derive-0.2.1] - 2022-02-22

### Fixed

- Fixed broken links to crate documentation
- Fixed broken link to LICENCE file

## [bounded-static-0.2.0] & [bounded-static-derive-0.2.0] - 2022-02-22

### Added

- Added support for complex generic bounds on struct and enum in the `ToStatic` derive macro

> For example, the following `struct` is now supported:
>
> ```rust
> #[derive(ToStatic)]
> struct Baz<'a, T: Foo>(T, Cow<'a, str>)
>     where
>         T: Into<String> + 'a + Bar;
> ```
>
> This produces (`ToBoundedStatic` shown, `IntoBoundedStatic` is also produced):
>
> ```rust
> impl<'a, T: Foo + ::bounded_static::ToBoundedStatic> ::bounded_static::ToBoundedStatic for Baz<'a, T>
> where
>     T: Into<String> + 'a + Bar + ::bounded_static::ToBoundedStatic,
>     T::Static: Foo + Into<String> + 'a + Bar,
> {
>     type Static = Baz<'static, T::Static>;
>     fn to_static(&self) -> Self::Static {
>         Baz(self.0.to_static(), self.1.to_static())
>     }
> }
> ```

- Added `ToBoundedStatic` and `IntoBoundedStatic` implementations for the `()` (unit) type
- Added `ToBoundedStatic` and `IntoBoundedStatic` implementations for the `Result<T, E>` type
- Added doc comments for `ToBoundedStatic` and `IntoBoundedStatic` impls for all primitive types

### Fixed

- Fixed broken links in documentation
- Fixed additional Clippy lints and [lib.rs](https://lib.rs) crates validation errors

## [bounded-static-0.1.0] & [bounded-static-derive-0.1.0] - 2022-02-18

### Added

- Initial release of `bounded-static` and `bounded-static-derive`

[bounded-static-0.8.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.7.0...bounded-static-0.8.0

[bounded-static-derive-0.8.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.7.0...bounded-static-derive-0.8.0

[bounded-static-0.7.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.6.0...bounded-static-0.7.0

[bounded-static-derive-0.7.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.6.0...bounded-static-derive-0.7.0

[bounded-static-0.6.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.5.0...bounded-static-0.6.0

[bounded-static-derive-0.6.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.5.0...bounded-static-derive-0.6.0

[bounded-static-0.5.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.4.0...bounded-static-0.5.0

[bounded-static-derive-0.5.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.4.0...bounded-static-derive-0.5.0

[bounded-static-0.4.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.3.0...bounded-static-0.4.0

[bounded-static-derive-0.4.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.3.0...bounded-static-derive-0.4.0

[bounded-static-0.3.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.2.1...bounded-static-0.3.0

[bounded-static-derive-0.3.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.2.1...bounded-static-derive-0.3.0

[bounded-static-0.2.1]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.2.0...bounded-static-0.2.1

[bounded-static-derive-0.2.1]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.2.0...bounded-static-derive-0.2.1

[bounded-static-0.2.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.1.0...bounded-static-0.2.0

[bounded-static-derive-0.2.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.1.0...bounded-static-derive-0.2.0

[bounded-static-0.1.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.0.0...bounded-static-0.1.0

[bounded-static-derive-0.1.0]: https://github.com/fujiapple852/bounded-static/compare/bounded-static-0.0.0...bounded-static-derive-0.1.0

//! Implementations for Rust standard library types.

mod core;

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "collections")]
mod collections;

#[cfg(feature = "std")]
mod std;

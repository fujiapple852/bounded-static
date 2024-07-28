//! Implementations for third-party types.

#[cfg(feature = "smol_str")]
mod smol_str;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "ahash")]
mod ahash;

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "jiff")]
mod jiff;

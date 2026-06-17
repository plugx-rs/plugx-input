#![doc = include_str!("../README.md")]

//! ## Modules
//!
//! | Module | Feature | Role |
//! |--------|---------|------|
//! | [`error`] | `serde` or `rkyv` | Deserialize failure types |
//! | [`position`] | `serde` or `rkyv` | Source paths in error messages |

mod impls;
mod input;

#[doc(inline)]
pub use input::{Input, InputType};

/// Deserialize errors and helpers.
///
/// Requires the `serde` or `rkyv` feature.
#[cfg(any(feature = "serde", feature = "rkyv"))]
pub mod error;
/// Location paths for deserialization errors (e.g. `config.toml:7:[server][host]`).
///
/// Requires the `serde` or `rkyv` feature.
#[cfg(any(feature = "serde", feature = "rkyv"))]
pub mod position;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rkyv")]
mod rkyv;

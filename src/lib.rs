#![doc = include_str!("../README.md")]

mod impls;
mod input;

#[doc(inline)]
pub use input::Input;

#[cfg(any(feature = "serde", feature = "rkyv"))]
pub mod error;
#[cfg(any(feature = "serde", feature = "rkyv"))]
pub mod position;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rkyv")]
mod rkyv;

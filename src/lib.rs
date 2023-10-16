#![doc = include_str!("../README.md")]

pub mod diff;
pub mod merge;
pub mod position;
#[doc(inline)]
pub use input::Input;
#[cfg(feature = "validation")]
pub mod validation;

pub mod ext {
    pub extern crate anyhow;
}

mod input;
mod input_from_impls;
mod logging;

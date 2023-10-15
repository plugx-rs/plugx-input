#![doc = include_str!("../README.md")]

pub mod definition;
pub mod diff;
pub mod merge;
pub mod position;
pub mod validation;
#[doc(inline)]
pub use input::Input;

pub mod ext {
    pub extern crate anyhow;
}

mod definition_from_impls;
mod input_from_impls;
mod logging;
mod input;
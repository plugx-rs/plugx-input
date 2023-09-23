#![doc = include_str!("../README.md")]

pub mod definition;
pub mod diff;
pub mod input;
pub mod merge;
pub mod position;
pub mod validation;
pub use input::Input;

pub extern crate anyhow;

mod definition_from_impls;
mod input_from_impls;
mod logging;

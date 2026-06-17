//! Errors returned when [`Input`] fails to deserialize.
//!
//! Enabled by the `serde` or `rkyv` Cargo feature. Serde maps these to custom
//! deserializer errors; rkyv uses [`DeserializeError::InvalidArchive`].

use crate::position::InputPath;
use std::convert::Infallible;
use thiserror::Error;

/// Allowed top-level [`Input`] kinds, used in type-mismatch messages.
pub const EXPECTED_INPUT_TYPES: &str = "boolean, integer, float, string, list, or map";

/// Failure while decoding external data into [`Input`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DeserializeError {
    /// Value at [`InputPath`] was not a supported [`Input`] variant.
    #[error("{}", invalid_type_message(.path, .expected, .found))]
    InvalidType {
        path: InputPath,
        expected: &'static str,
        found: String,
    },
    /// Integer literal does not fit in [`isize`] (serde `i128`/`u128` paths).
    #[error("{}", integer_out_of_range_message(.path, *.value))]
    IntegerOutOfRange { path: InputPath, value: i128 },
    /// Archived bytes could not be validated or decoded (`rkyv` feature).
    #[error("{path}: invalid archived data: {message}")]
    InvalidArchive { path: InputPath, message: String },
}

/// Build a type-mismatch error with [`EXPECTED_INPUT_TYPES`] (`serde` feature).
#[cfg(feature = "serde")]
pub fn invalid_type(path: InputPath, found: impl Into<String>) -> DeserializeError {
    DeserializeError::InvalidType {
        path,
        expected: EXPECTED_INPUT_TYPES,
        found: found.into(),
    }
}

fn invalid_type_message(path: &InputPath, expected: &str, found: &str) -> String {
    if path.is_empty() {
        format!("expected {expected}, found {found}")
    } else {
        format!("{path}: expected {expected}, found {found}")
    }
}

fn integer_out_of_range_message(path: &InputPath, value: i128) -> String {
    if path.is_empty() {
        format!("integer {value} is out of range for isize")
    } else {
        format!("{path}: integer {value} is out of range for isize")
    }
}

/// Convert a deserialized integer to [`isize`], if it fits (`serde` feature).
#[cfg(feature = "serde")]
pub fn i128_to_isize(value: i128) -> Option<isize> {
    value.try_into().ok()
}

/// [`Input`] serialization is infallible for supported serializers/archives.
pub type InputSerializeError = Infallible;

use crate::position::InputPath;
use std::convert::Infallible;
use thiserror::Error;

pub const EXPECTED_INPUT_TYPES: &str = "boolean, integer, float, string, list, or map";

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InputDeserializeError {
    #[error("{}", invalid_type_message(.path, .expected, .found))]
    InvalidType {
        path: InputPath,
        expected: &'static str,
        found: String,
    },
    #[error("{}", integer_out_of_range_message(.path, *.value))]
    IntegerOutOfRange { path: InputPath, value: i128 },
    #[error("{path}: invalid archived data: {message}")]
    InvalidArchive { path: InputPath, message: String },
}

#[cfg(feature = "serde")]
pub fn invalid_type(path: InputPath, found: impl Into<String>) -> InputDeserializeError {
    InputDeserializeError::InvalidType {
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

#[cfg(feature = "serde")]
pub fn i128_to_isize(value: i128) -> Result<isize, ()> {
    value.try_into().map_err(|_| ())
}

/// Marker for APIs that intentionally never fail during serialization.
pub type InputSerializeError = Infallible;

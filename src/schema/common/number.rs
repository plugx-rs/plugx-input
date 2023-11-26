use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum InputSchemaTypeNumberValue {
    Integer(isize),
    Float(f64),
}

impl Display for InputSchemaTypeNumberValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Integer(i) => i.to_string(),
                Self::Float(f) => f.to_string(),
            }
            .as_str(),
        )
    }
}

impl InputSchemaTypeNumberValue {
    pub fn from_integer<I: Into<isize>>(i: I) -> Self {
        Self::Integer(i.into())
    }

    pub fn from_float<F: Into<f64>>(f: F) -> Self {
        Self::Float(f.into())
    }

    pub fn round(&self) -> Self {
        match self {
            Self::Float(f) => Self::Integer(f.round() as isize),
            Self::Integer(i) => Self::Integer(*i),
        }
    }

    pub fn round_mut(&mut self) {
        *self = match self {
            Self::Float(f) => Self::Integer(f.round() as isize),
            Self::Integer(i) => Self::Integer(*i),
        };
    }

    pub fn trunc(&self) -> Self {
        match self {
            Self::Float(f) => Self::Integer(f.trunc() as isize),
            Self::Integer(i) => Self::Integer(*i),
        }
    }

    pub fn trunc_mut(&mut self) {
        *self = match self {
            Self::Float(f) => Self::Integer(f.trunc() as isize),
            Self::Integer(i) => Self::Integer(*i),
        };
    }

    pub fn integer(&self) -> Option<isize> {
        if let Self::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn float(&self) -> f64 {
        match self {
            Self::Float(f) => *f,
            Self::Integer(i) => *i as f64,
        }
    }
}

impl PartialEq for InputSchemaTypeNumberValue {
    fn eq(&self, other: &Self) -> bool {
        self.float() == other.float()
    }
}

impl PartialOrd for InputSchemaTypeNumberValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.float().partial_cmp(&other.float())
    }
}

impl From<InputSchemaTypeNumberValue> for f64 {
    fn from(value: InputSchemaTypeNumberValue) -> Self {
        value.float()
    }
}

impl From<i8> for InputSchemaTypeNumberValue {
    fn from(i: i8) -> Self {
        Self::Integer(i as isize)
    }
}

impl From<i16> for InputSchemaTypeNumberValue {
    fn from(i: i16) -> Self {
        Self::Integer(i as isize)
    }
}
impl From<i32> for InputSchemaTypeNumberValue {
    fn from(i: i32) -> Self {
        Self::Integer(i as isize)
    }
}

impl From<u8> for InputSchemaTypeNumberValue {
    fn from(u: u8) -> Self {
        Self::Integer(u as isize)
    }
}

impl From<u16> for InputSchemaTypeNumberValue {
    fn from(u: u16) -> Self {
        Self::Integer(u as isize)
    }
}
impl From<u32> for InputSchemaTypeNumberValue {
    fn from(u: u32) -> Self {
        Self::Integer(u as isize)
    }
}

impl From<isize> for InputSchemaTypeNumberValue {
    fn from(i: isize) -> Self {
        Self::Integer(i)
    }
}

impl From<f32> for InputSchemaTypeNumberValue {
    fn from(f: f32) -> Self {
        Self::Float(f as f64)
    }
}

impl From<f64> for InputSchemaTypeNumberValue {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn from() {
        assert_eq!(
            InputSchemaTypeNumberValue::from(3.14f64),
            InputSchemaTypeNumberValue::from_float(3.14)
        );
        assert_eq!(
            InputSchemaTypeNumberValue::from(3i8),
            InputSchemaTypeNumberValue::from_float(3.0)
        );
        assert_eq!(
            InputSchemaTypeNumberValue::from(-10i16),
            InputSchemaTypeNumberValue::from_float(-10.0)
        );
    }

    #[test]
    fn serde() {
        let json = serde_json::to_string_pretty(&json!(3.6)).unwrap();
        let mut n: InputSchemaTypeNumberValue = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(n.float(), 3.6);
        assert_eq!(n.round(), 4.into());
        assert_eq!(n.trunc(), 3.into());
        assert_eq!(n.integer(), None);
        n.trunc_mut();
        assert_eq!(n.integer(), Some(3isize.into()));
    }
}

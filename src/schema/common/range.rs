use crate::schema::common::number::InputSchemaTypeNumberValue;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", untagged, deny_unknown_fields)]
pub enum InputSchemaTypeRange {
    Max(InputSchemaTypeNumberValue),
    MinMax {
        #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
        maybe_max: Option<InputSchemaTypeNumberValue>,
        #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
        maybe_min: Option<InputSchemaTypeNumberValue>,
    },
}

impl Display for InputSchemaTypeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match (self.maybe_min(), self.maybe_max()) {
                (Some(min), Some(max)) => format!("range from {min} to {max}"),
                (Some(min), None) => format!("range that starts from {min}"),
                (None, Some(max)) => format!("range that ends in {max}"),
                (None, None) => "range".into(),
            }
            .as_str(),
        )
    }
}

impl InputSchemaTypeRange {
    pub fn new_with_max<M: Into<InputSchemaTypeNumberValue>>(max: M) -> Self {
        Self::Max(max.into())
    }

    pub fn new_with_min<M: Into<InputSchemaTypeNumberValue>>(min: M) -> Self {
        Self::MinMax {
            maybe_max: None,
            maybe_min: Some(min.into()),
        }
    }

    pub fn new_with_min_max<M: Into<InputSchemaTypeNumberValue>>(min: M, max: M) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }

    pub fn set_min<M: Into<InputSchemaTypeNumberValue>>(&mut self, min: M) {
        *self = match self {
            Self::Max(max) => Self::MinMax {
                maybe_max: Some(*max),
                maybe_min: Some(min.into()),
            },
            Self::MinMax { maybe_max, .. } => Self::MinMax {
                maybe_max: *maybe_max,
                maybe_min: Some(min.into()),
            },
        };
    }

    pub fn with_min<M: Into<InputSchemaTypeNumberValue>>(mut self, min: M) -> Self {
        self.set_min(min);
        self
    }

    pub fn set_max<M: Into<InputSchemaTypeNumberValue>>(&mut self, max: M) {
        *self = match self {
            Self::Max(..) => Self::Max(max.into()),
            Self::MinMax { maybe_min, .. } => Self::MinMax {
                maybe_max: Some(max.into()),
                maybe_min: *maybe_min,
            },
        };
    }

    pub fn with_max<M: Into<InputSchemaTypeNumberValue>>(mut self, max: M) -> Self {
        self.set_max(max);
        self
    }

    pub fn maybe_min(&self) -> Option<InputSchemaTypeNumberValue> {
        match self {
            Self::MinMax { maybe_min, .. } => *maybe_min,
            _ => None,
        }
    }

    pub fn maybe_max(&self) -> Option<InputSchemaTypeNumberValue> {
        match self {
            Self::MinMax { maybe_max, .. } => *maybe_max,
            Self::Max(max) => Some(*max),
        }
    }
}

impl PartialEq for InputSchemaTypeRange {
    fn eq(&self, other: &Self) -> bool {
        self.maybe_max() == other.maybe_max() && self.maybe_min() == other.maybe_min()
    }
}

impl<T: Into<InputSchemaTypeNumberValue>> From<Range<T>> for InputSchemaTypeRange {
    fn from(range: Range<T>) -> Self {
        Self::MinMax {
            maybe_max: Some(range.end.into()),
            maybe_min: Some(range.start.into()),
        }
    }
}

impl<T1: Into<InputSchemaTypeNumberValue>, T2: Into<InputSchemaTypeNumberValue>> From<(T1, T2)>
    for InputSchemaTypeRange
{
    fn from((min, max): (T1, T2)) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }
}

impl<T: Into<InputSchemaTypeNumberValue>> From<T> for InputSchemaTypeRange {
    fn from(max: T) -> Self {
        Self::Max(max.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(
            InputSchemaTypeRange::from(0u8).maybe_max(),
            Some(0u8.into())
        );
        assert_eq!(
            InputSchemaTypeRange::from((10isize, 20u16)).maybe_max(),
            Some(20u8.into())
        );
        assert_eq!(
            InputSchemaTypeRange::from((10isize, 20u16)).maybe_min(),
            Some(10i16.into())
        );
        assert_eq!(
            InputSchemaTypeRange::from(10..20).maybe_max(),
            Some(20u8.into())
        );
        assert_eq!(
            InputSchemaTypeRange::from(10..20).maybe_min(),
            Some(10u16.into())
        );
    }

    #[test]
    fn serde() {
        let json = serde_json::to_string_pretty(&serde_json::json!(3.14)).unwrap();
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeRange>(json.as_str())
                .unwrap()
                .maybe_min(),
            None
        );
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeRange>(json.as_str())
                .unwrap()
                .maybe_max()
                .map(|max| max.trunc()),
            Some(3.into())
        );

        let json = serde_json::to_string_pretty(&serde_json::json!({"min": 10})).unwrap();
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeRange>(json.as_str())
                .unwrap()
                .maybe_min(),
            Some(10.into())
        );
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeRange>(json.as_str())
                .unwrap()
                .maybe_max(),
            None
        );

        let json =
            serde_json::to_string_pretty(&serde_json::json!({"min": 10, "max": 3.6})).unwrap();
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeRange>(json.as_str())
                .unwrap()
                .maybe_min(),
            Some(10.into())
        );
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeRange>(json.as_str())
                .unwrap()
                .maybe_max()
                .map(|max| max.round()),
            Some(4.into())
        );
    }
}

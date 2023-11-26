use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
#[serde(
    rename_all = "snake_case",
    untagged,
    deny_unknown_fields,
    expecting = "Expecting an unsigned integer or an object containing `min` and `max` keys with unsigned integer values"
)]
pub enum InputSchemaTypeSize {
    Max(usize),
    MinMax {
        #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
        maybe_max: Option<usize>,
        #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
        maybe_min: Option<usize>,
    },
}

impl Display for InputSchemaTypeSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match (self.maybe_min(), self.maybe_max()) {
                (Some(min), Some(max)) => format!("size bigger than {min} and lower than {max}"),
                (Some(min), None) => format!("size bigger than {min}"),
                (None, Some(max)) => format!("size lower than {max}"),
                (None, None) => "size".into(),
            }
            .as_str(),
        )
    }
}

impl InputSchemaTypeSize {
    pub fn new_with_max<M: Into<usize>>(max: M) -> Self {
        Self::Max(max.into())
    }

    pub fn new_with_min<M: Into<usize>>(min: M) -> Self {
        Self::MinMax {
            maybe_max: None,
            maybe_min: Some(min.into()),
        }
    }

    pub fn new_with_min_max<M: Into<usize>>(min: M, max: M) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }

    pub fn set_min<M: Into<usize>>(&mut self, min: M) {
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

    pub fn with_min<M: Into<usize>>(mut self, min: M) -> Self {
        self.set_min(min);
        self
    }

    pub fn set_max<M: Into<usize>>(&mut self, max: M) {
        *self = match self {
            Self::Max(..) => Self::Max(max.into()),
            Self::MinMax { maybe_min, .. } => Self::MinMax {
                maybe_max: Some(max.into()),
                maybe_min: *maybe_min,
            },
        };
    }

    pub fn with_max<M: Into<usize>>(mut self, max: M) -> Self {
        self.set_max(max);
        self
    }

    pub fn maybe_min(&self) -> Option<usize> {
        match self {
            Self::MinMax { maybe_min, .. } => *maybe_min,
            _ => None,
        }
    }

    pub fn maybe_max(&self) -> Option<usize> {
        match self {
            Self::MinMax { maybe_max, .. } => *maybe_max,
            Self::Max(max) => Some(*max),
        }
    }
}

impl PartialEq for InputSchemaTypeSize {
    fn eq(&self, other: &Self) -> bool {
        self.maybe_max() == other.maybe_max() && self.maybe_min() == other.maybe_min()
    }
}

impl<T: Into<usize>> From<Range<T>> for InputSchemaTypeSize {
    fn from(range: Range<T>) -> Self {
        Self::MinMax {
            maybe_max: Some(range.end.into()),
            maybe_min: Some(range.start.into()),
        }
    }
}

impl<T1: Into<usize>, T2: Into<usize>> From<(T1, T2)> for InputSchemaTypeSize {
    fn from((min, max): (T1, T2)) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }
}

impl From<usize> for InputSchemaTypeSize {
    fn from(max: usize) -> Self {
        Self::Max(max)
    }
}

impl From<u32> for InputSchemaTypeSize {
    fn from(max: u32) -> Self {
        Self::Max(max as usize)
    }
}

impl From<u16> for InputSchemaTypeSize {
    fn from(max: u16) -> Self {
        Self::Max(max as usize)
    }
}

impl From<u8> for InputSchemaTypeSize {
    fn from(max: u8) -> Self {
        Self::Max(max as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(InputSchemaTypeSize::from(0u8).maybe_max(), Some(0u8.into()));
        assert_eq!(
            InputSchemaTypeSize::from((10usize, 20usize)).maybe_max(),
            Some(20)
        );
        assert_eq!(
            InputSchemaTypeSize::from((10usize, 20usize)).maybe_min(),
            Some(10)
        );
        assert_eq!(InputSchemaTypeSize::from(10usize..20).maybe_max(), Some(20));
        assert_eq!(InputSchemaTypeSize::from(10usize..20).maybe_min(), Some(10));
    }

    #[test]
    fn serde() {
        let json = serde_json::to_string_pretty(&serde_json::json!(3)).unwrap();
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeSize>(json.as_str())
                .unwrap()
                .maybe_min(),
            None
        );
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeSize>(json.as_str())
                .unwrap()
                .maybe_max(),
            Some(3)
        );

        let json = serde_json::to_string_pretty(&serde_json::json!({"min": 10})).unwrap();
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeSize>(json.as_str())
                .unwrap()
                .maybe_min(),
            Some(10)
        );
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeSize>(json.as_str())
                .unwrap()
                .maybe_max(),
            None
        );

        let json = serde_json::to_string_pretty(&serde_json::json!({"min": 10, "max": 4})).unwrap();
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeSize>(json.as_str())
                .unwrap()
                .maybe_min(),
            Some(10)
        );
        assert_eq!(
            serde_json::from_str::<InputSchemaTypeSize>(json.as_str())
                .unwrap()
                .maybe_max(),
            Some(4)
        );

        let json =
            serde_json::to_string_pretty(&serde_json::json!({"min": 10, "max": -4})).unwrap();
        assert!(serde_json::from_str::<InputSchemaTypeSize>(json.as_str()).is_err());
    }
}

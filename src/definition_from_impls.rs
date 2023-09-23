use crate::definition::*;
use crate::Input;
use std::ops::Range;

impl<T: Into<InputDefinitionType>> From<T> for InputDefinition {
    fn from(value: T) -> Self {
        Self {
            definition_type: Box::new(value.into()),
            maybe_default: None,
        }
    }
}

impl<T: Into<InputDefinitionType>, D: Into<Input>> From<(T, Option<D>)> for InputDefinition {
    fn from(value: (T, Option<D>)) -> Self {
        Self {
            definition_type: Box::new(value.0.into()),
            maybe_default: value.1.map(|default| default.into()),
        }
    }
}

impl From<usize> for InputDefinitionSize {
    fn from(value: usize) -> Self {
        Self::Max(value)
    }
}

impl From<u32> for InputDefinitionSize {
    fn from(value: u32) -> Self {
        Self::from(value as usize)
    }
}

impl From<u16> for InputDefinitionSize {
    fn from(value: u16) -> Self {
        Self::from(value as usize)
    }
}

impl From<u8> for InputDefinitionSize {
    fn from(value: u8) -> Self {
        Self::from(value as usize)
    }
}

impl From<Range<usize>> for InputDefinitionSize {
    fn from(value: Range<usize>) -> Self {
        let Range { start, end, .. } = value;
        Self::MinMax {
            maybe_min: Some(start),
            maybe_max: Some(end),
        }
    }
}

impl From<Range<u32>> for InputDefinitionSize {
    fn from(value: Range<u32>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as usize)..(end as usize))
    }
}

impl From<Range<u16>> for InputDefinitionSize {
    fn from(value: Range<u16>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as usize)..(end as usize))
    }
}

impl From<Range<u8>> for InputDefinitionSize {
    fn from(value: Range<u8>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as usize)..(end as usize))
    }
}

impl From<isize> for InputDefinitionRangeInteger {
    fn from(value: isize) -> Self {
        Self::Max(value)
    }
}

impl From<i32> for InputDefinitionRangeInteger {
    fn from(value: i32) -> Self {
        Self::from(value as isize)
    }
}

impl From<u32> for InputDefinitionRangeInteger {
    fn from(value: u32) -> Self {
        Self::from(value as isize)
    }
}

impl From<i16> for InputDefinitionRangeInteger {
    fn from(value: i16) -> Self {
        Self::from(value as isize)
    }
}

impl From<u16> for InputDefinitionRangeInteger {
    fn from(value: u16) -> Self {
        Self::from(value as isize)
    }
}

impl From<i8> for InputDefinitionRangeInteger {
    fn from(value: i8) -> Self {
        Self::from(value as isize)
    }
}

impl From<u8> for InputDefinitionRangeInteger {
    fn from(value: u8) -> Self {
        Self::from(value as isize)
    }
}

impl From<Range<isize>> for InputDefinitionRangeInteger {
    fn from(value: Range<isize>) -> Self {
        let Range { start, end, .. } = value;
        Self::MinMax {
            maybe_min: Some(start),
            maybe_max: Some(end),
        }
    }
}

impl From<Range<i32>> for InputDefinitionRangeInteger {
    fn from(value: Range<i32>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as isize)..(end as isize))
    }
}

impl From<Range<u32>> for InputDefinitionRangeInteger {
    fn from(value: Range<u32>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as isize)..(end as isize))
    }
}

impl From<Range<i16>> for InputDefinitionRangeInteger {
    fn from(value: Range<i16>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as isize)..(end as isize))
    }
}

impl From<Range<u16>> for InputDefinitionRangeInteger {
    fn from(value: Range<u16>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as isize)..(end as isize))
    }
}

impl From<Range<i8>> for InputDefinitionRangeInteger {
    fn from(value: Range<i8>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as isize)..(end as isize))
    }
}

impl From<Range<u8>> for InputDefinitionRangeInteger {
    fn from(value: Range<u8>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as isize)..(end as isize))
    }
}

impl From<f64> for InputDefinitionRangeFloat {
    fn from(value: f64) -> Self {
        Self::Max(value)
    }
}

impl From<f32> for InputDefinitionRangeFloat {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl From<i32> for InputDefinitionRangeFloat {
    fn from(value: i32) -> Self {
        Self::from(value as f64)
    }
}

impl From<u32> for InputDefinitionRangeFloat {
    fn from(value: u32) -> Self {
        Self::from(value as f64)
    }
}

impl From<i16> for InputDefinitionRangeFloat {
    fn from(value: i16) -> Self {
        Self::from(value as f64)
    }
}

impl From<u16> for InputDefinitionRangeFloat {
    fn from(value: u16) -> Self {
        Self::from(value as f64)
    }
}

impl From<i8> for InputDefinitionRangeFloat {
    fn from(value: i8) -> Self {
        Self::from(value as f64)
    }
}

impl From<u8> for InputDefinitionRangeFloat {
    fn from(value: u8) -> Self {
        Self::from(value as f64)
    }
}

impl From<Range<f64>> for InputDefinitionRangeFloat {
    fn from(value: Range<f64>) -> Self {
        let Range { start, end, .. } = value;
        Self::MinMax {
            maybe_min: Some(start),
            maybe_max: Some(end),
        }
    }
}

impl From<Range<f32>> for InputDefinitionRangeFloat {
    fn from(value: Range<f32>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

impl From<Range<i32>> for InputDefinitionRangeFloat {
    fn from(value: Range<i32>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

impl From<Range<u32>> for InputDefinitionRangeFloat {
    fn from(value: Range<u32>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

impl From<Range<i16>> for InputDefinitionRangeFloat {
    fn from(value: Range<i16>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

impl From<Range<u16>> for InputDefinitionRangeFloat {
    fn from(value: Range<u16>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

impl From<Range<i8>> for InputDefinitionRangeFloat {
    fn from(value: Range<i8>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

impl From<Range<u8>> for InputDefinitionRangeFloat {
    fn from(value: Range<u8>) -> Self {
        let Range { start, end, .. } = value;
        Self::from((start as f64)..(end as f64))
    }
}

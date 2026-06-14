use crate::Input;
use std::collections::HashMap;

impl From<&Input> for Input {
    fn from(value: &Input) -> Self {
        value.clone()
    }
}

impl From<bool> for Input {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Option<bool>> for Input {
    fn from(value: Option<bool>) -> Self {
        Self::Bool(value.unwrap_or_default())
    }
}

impl From<isize> for Input {
    fn from(value: isize) -> Self {
        Self::Int(value)
    }
}

impl From<Option<isize>> for Input {
    fn from(value: Option<isize>) -> Self {
        Self::Int(value.unwrap_or_default())
    }
}

impl From<i8> for Input {
    fn from(value: i8) -> Self {
        Self::Int(value as isize)
    }
}

impl From<i16> for Input {
    fn from(value: i16) -> Self {
        Self::Int(value as isize)
    }
}

impl From<i32> for Input {
    fn from(value: i32) -> Self {
        Self::Int(value as isize)
    }
}

impl From<u8> for Input {
    fn from(value: u8) -> Self {
        Self::Int(value as isize)
    }
}

impl From<u16> for Input {
    fn from(value: u16) -> Self {
        Self::Int(value as isize)
    }
}

impl From<u32> for Input {
    fn from(value: u32) -> Self {
        Self::Int(value as isize)
    }
}

impl From<f64> for Input {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<Option<f64>> for Input {
    fn from(value: Option<f64>) -> Self {
        Self::Float(value.unwrap_or_default())
    }
}

impl From<f32> for Input {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self::Str(value.to_string())
    }
}

impl From<Option<&str>> for Input {
    fn from(value: Option<&str>) -> Self {
        Self::Str(value.unwrap_or_default().to_string())
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<Option<String>> for Input {
    fn from(value: Option<String>) -> Self {
        Self::Str(value.unwrap_or_default())
    }
}

impl From<&String> for Input {
    fn from(value: &String) -> Self {
        Self::Str(value.to_string())
    }
}

impl From<Option<&String>> for Input {
    fn from(value: Option<&String>) -> Self {
        Self::Str(value.cloned().unwrap_or_default())
    }
}

impl<T: Into<Input>> From<Vec<T>> for Input {
    fn from(value: Vec<T>) -> Self {
        Self::List(value.into_iter().map(|i| i.into()).collect())
    }
}

impl<T: Into<Input>> From<Option<Vec<T>>> for Input {
    fn from(value: Option<Vec<T>>) -> Self {
        Self::List(
            value
                .unwrap_or_default()
                .into_iter()
                .map(|i| i.into())
                .collect(),
        )
    }
}

impl<T: Into<Input>, const SIZE: usize> From<[T; SIZE]> for Input {
    fn from(value: [T; SIZE]) -> Self {
        Self::List(value.into_iter().map(|i| i.into()).collect())
    }
}

impl<T: Into<Input> + Clone> From<&[T]> for Input {
    fn from(value: &[T]) -> Self {
        Self::List(value.iter().cloned().map(|i| i.into()).collect())
    }
}

impl<T: Into<Input> + Clone> From<Option<&[T]>> for Input {
    fn from(value: Option<&[T]>) -> Self {
        Self::List(
            value
                .unwrap_or_default()
                .iter()
                .cloned()
                .map(|i| i.into())
                .collect(),
        )
    }
}

impl<I: Into<Input>> FromIterator<I> for Input {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self::List(iter.into_iter().map(|i| i.into()).collect())
    }
}

impl<K: Into<String>, V: Into<Input>> From<HashMap<K, V>> for Input {
    fn from(value: HashMap<K, V>) -> Self {
        Self::Map(
            value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<K: Into<String>, V: Into<Input>> From<Option<HashMap<K, V>>> for Input {
    fn from(value: Option<HashMap<K, V>>) -> Self {
        Self::Map(
            value
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<K: Into<String>, V: Into<Input>> FromIterator<(K, V)> for Input {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::Map(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

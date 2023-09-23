use serde::Serialize;
use std::{
    fmt::{Display, Formatter, Result},
    ops::Index,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputPosition {
    inner: Vec<InputPositionType>,
}

impl InputPosition {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn new_with_index(&self, index: usize) -> Self {
        let mut new = self.clone();
        new.add_index(index);
        new
    }

    pub fn new_with_key(&self, key: &str) -> Self {
        let mut new = self.clone();
        new.add_key(key);
        new
    }

    pub fn add_index(&mut self, index: usize) {
        self.inner.push(InputPositionType::Index(index))
    }

    pub fn add_key(&mut self, key: &str) {
        self.inner.push(InputPositionType::Key(key.to_string()))
    }

    pub fn add<T: Into<InputPositionType>>(&mut self, key_or_index: T) {
        self.inner.push(key_or_index.into())
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub fn new() -> InputPosition {
    InputPosition::new()
}

impl Default for InputPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for InputPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(
            if self.inner.len() == 1 {
                self.inner[0].to_string()
            } else {
                self.inner
                    .iter()
                    .map(|position_type| format!("[{position_type}]"))
                    .collect::<String>()
            }
            .as_str(),
        )
    }
}

impl Index<usize> for InputPosition {
    type Output = InputPositionType;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum InputPositionType {
    Key(String),
    Index(usize),
}

impl InputPositionType {
    pub fn new<T: Into<Self>>(value: T) -> Self {
        value.into()
    }
}

impl From<usize> for InputPositionType {
    fn from(index: usize) -> Self {
        Self::Index(index)
    }
}

impl From<&str> for InputPositionType {
    fn from(key: &str) -> Self {
        Self::Key(key.to_string())
    }
}

impl From<String> for InputPositionType {
    fn from(key: String) -> Self {
        Self::Key(key)
    }
}

impl Display for InputPositionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(
            match self {
                Self::Key(key) => key.to_string(),
                Self::Index(index) => format!("{index}"),
            }
            .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn done() {
        let mut pos = new();
        pos.add_index(0);
        assert_eq!(pos[0], InputPositionType::new(0));
        assert_eq!(format!("{pos}"), "0".to_string());
        pos.add_key("foo");
        assert_eq!(pos[0], InputPositionType::new(0));
        assert_eq!(pos[1], InputPositionType::new("foo"));
        assert_eq!(format!("{pos}"), "[0][foo]".to_string());
    }
}

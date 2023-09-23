use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    untagged,
    expecting = "expecting boolean, integer, float, string, list, or map"
)]
pub enum Input {
    Bool(bool),
    Int(isize),
    Float(f64),
    Str(String),
    List(Vec<Input>),
    Map(HashMap<String, Input>),
}

impl Input {
    pub fn new_map() -> Self {
        Self::Map(HashMap::new())
    }

    pub fn new_list() -> Self {
        Self::List(Vec::new())
    }

    pub fn new_str() -> Self {
        Self::Str(String::new())
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn bool_ref(&self) -> Option<&bool> {
        if let Self::Bool(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        if let Self::Bool(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn bool_mut(&mut self) -> Option<&mut bool> {
        if let Self::Bool(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub fn int_ref(&self) -> Option<&isize> {
        if let Self::Int(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_int(self) -> Option<isize> {
        if let Self::Int(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn int_mut(&mut self) -> Option<&mut isize> {
        if let Self::Int(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn float_ref(&self) -> Option<&f64> {
        if let Self::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_float(self) -> Option<f64> {
        if let Self::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn float_mut(&mut self) -> Option<&mut f64> {
        if let Self::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    pub fn str_ref(&self) -> Option<&String> {
        if let Self::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_str(self) -> Option<String> {
        if let Self::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn str_mut(&mut self) -> Option<&mut String> {
        if let Self::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub fn list_ref(&self) -> Option<&Vec<Input>> {
        if let Self::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_list(self) -> Option<Vec<Input>> {
        if let Self::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn list_mut(&mut self) -> Option<&mut Vec<Input>> {
        if let Self::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }

    pub fn map_ref(&self) -> Option<&HashMap<String, Input>> {
        if let Self::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_map(self) -> Option<HashMap<String, Input>> {
        if let Self::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn map_mut(&mut self) -> Option<&mut HashMap<String, Input>> {
        if let Self::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Self::Bool(_) => Self::bool_type_name(),
            Self::Int(_) => Self::int_type_name(),
            Self::Float(_) => Self::float_type_name(),
            Self::Str(_) => Self::str_type_name(),
            Self::List(_) => Self::list_type_name(),
            Self::Map(_) => Self::map_type_name(),
        }
    }

    pub fn empty_type_name() -> String {
        "empty".to_string()
    }

    pub fn map_type_name() -> String {
        "map".to_string()
    }

    pub fn list_type_name() -> String {
        "list".to_string()
    }

    pub fn str_type_name() -> String {
        "string".to_string()
    }

    pub fn int_type_name() -> String {
        "integer".to_string()
    }

    pub fn float_type_name() -> String {
        "float".to_string()
    }

    pub fn bool_type_name() -> String {
        "boolean".to_string()
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(value) => write!(f, "{value}"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Str(value) => write!(f, "{value:?}"),
            Self::List(value) => {
                write!(f, "[")?;
                let length = value.len();
                for (index, inner_value) in value.iter().enumerate() {
                    write!(f, "{inner_value}")?;
                    if index < length - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Self::Map(value) => {
                write!(f, "{{")?;
                for (key, value) in value {
                    write!(f, "{key:?}:{value}")?
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn serde() {
        let de_result = serde_json::from_str::<Input>("true");
        assert!(de_result.is_ok());
        assert_eq!(Input::Bool(true), de_result.unwrap());

        let de_result = serde_json::from_str::<Input>("0");
        assert!(de_result.is_ok());
        assert_eq!(Input::Int(0), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("1234567890");
        assert!(de_result.is_ok());
        assert_eq!(Input::Int(1234567890), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("-1234567890");
        assert!(de_result.is_ok());
        assert_eq!(Input::Int(-1234567890), de_result.unwrap());

        let de_result = serde_json::from_str::<Input>("0.0");
        assert!(de_result.is_ok());
        assert_eq!(Input::Float(0.0), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("1234567890.0");
        assert!(de_result.is_ok());
        assert_eq!(Input::Float(1234567890.0), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("-1234567890.0");
        assert!(de_result.is_ok());
        assert_eq!(Input::Float(-1234567890.0), de_result.unwrap());

        let de_result = serde_json::from_str::<Input>("\"hello\"");
        assert!(de_result.is_ok());
        assert_eq!(Input::Str("hello".to_string()), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("\"false\"");
        assert!(de_result.is_ok());
        assert_eq!(Input::Str("false".to_string()), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("\"1\"");
        assert!(de_result.is_ok());
        assert_eq!(Input::Str("1".to_string()), de_result.unwrap());
        let de_result = serde_json::from_str::<Input>("\"3.14\"");
        assert!(de_result.is_ok());
        assert_eq!(Input::Str("3.14".to_string()), de_result.unwrap());

        let de_result = serde_json::from_str::<Input>("[false, 0, 0.0, \"hello\", [[]], {}]");
        assert!(de_result.is_ok());
        let list = de_result.unwrap();
        assert!(list.is_list());
        assert_eq!(
            Input::List(
                [
                    Input::from(false),
                    Input::from(0),
                    Input::from(0.0),
                    Input::from("hello".to_string()),
                    Input::from([Input::List([].to_vec())].to_vec()),
                    Input::from(Input::new_map())
                ]
                .to_vec()
            ),
            list
        );

        let de_result = serde_json::from_str::<Input>(
            "{\"foo\": 0, \"bar\": 0.0, \"baz\": false, \"qux\": {\"hello\": \"world\"}}",
        );
        assert!(de_result.is_ok());
        let map = de_result.unwrap();
        assert!(map.is_map());
        assert_eq!(
            Input::Map(HashMap::from([
                ("foo".to_string(), Input::Int(0)),
                ("bar".to_string(), Input::Float(0.0)),
                ("baz".to_string(), Input::Bool(false)),
                (
                    "qux".to_string(),
                    Input::Map(HashMap::from([(
                        "hello".to_string(),
                        Input::Str("world".to_string())
                    )]))
                ),
            ])),
            map
        );
    }
}

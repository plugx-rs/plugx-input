use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    rkyv(derive(Debug)),
    rkyv(serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator,
        __S::Error: rkyv::rancor::Source,
    )),
    rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source)),
    rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext))),
)]
pub enum Input {
    Bool(bool),
    Int(isize),
    Float(f64),
    Str(String),
    List(#[cfg_attr(feature = "rkyv", rkyv(omit_bounds))] Vec<Input>),
    Map(#[cfg_attr(feature = "rkyv", rkyv(omit_bounds))] HashMap<String, Input>),
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

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Self::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub fn as_int(&self) -> Option<&isize> {
        match self {
            Self::Int(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_int(self) -> Option<isize> {
        match self {
            Self::Int(value) => Some(value),
            _ => None,
        }
    }

    pub fn int_mut(&mut self) -> Option<&mut isize> {
        match self {
            Self::Int(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn as_float(&self) -> Option<&f64> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_float(self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn float_mut(&mut self) -> Option<&mut f64> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }

    pub fn as_str(&self) -> Option<&String> {
        match self {
            Self::Str(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_str(self) -> Option<String> {
        match self {
            Self::Str(value) => Some(value),
            _ => None,
        }
    }

    pub fn str_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Str(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub fn as_list(&self) -> Option<&Vec<Input>> {
        match self {
            Self::List(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<Vec<Input>> {
        match self {
            Self::List(value) => Some(value),
            _ => None,
        }
    }

    pub fn list_mut(&mut self) -> Option<&mut Vec<Input>> {
        match self {
            Self::List(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Input>> {
        match self {
            Self::Map(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_map(self) -> Option<HashMap<String, Input>> {
        match self {
            Self::Map(value) => Some(value),
            _ => None,
        }
    }

    pub fn map_mut(&mut self) -> Option<&mut HashMap<String, Input>> {
        match self {
            Self::Map(value) => Some(value),
            _ => None,
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
                let length = value.len();
                for (index, (key, value)) in value.iter().enumerate() {
                    write!(f, "{key:?}: {value}")?;
                    if index < length - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

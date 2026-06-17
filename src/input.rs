use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

/// Kind of value stored in [`Input`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    Bool,
    Int,
    Str,
    Float,
    Map,
    List,
}

impl Display for InputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bool => "boolean",
            Self::Int => "integer",
            Self::Str => "string",
            Self::Float => "float",
            Self::Map => "map",
            Self::List => "list",
        })
    }
}

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
/// Dynamically typed value for plugin configuration and runtime state.
///
/// Six variants: boolean, integer ([`isize`]), float ([`f64`]), string, list, and map.
/// Build values with [`From`] (scalars, slices, [`Vec`], [`HashMap`], etc.); inspect them
/// with `is_*`, `as_*`, `into_*`, and `*_mut` accessors. Use [`Input::type_name`] for the
/// active [`InputType`].
///
/// # Cargo features
///
/// - **`serde`**: [`Serialize`], [`Deserialize`], and [`Input::serialize`].
/// - **`rkyv`**: archive derives plus [`Input::to_rkyv_bytes`] and [`Input::from_rkyv_bytes`].
pub enum Input {
    Bool(bool),
    Int(isize),
    Float(f64),
    Str(String),
    List(#[cfg_attr(feature = "rkyv", rkyv(omit_bounds))] Vec<Input>),
    Map(#[cfg_attr(feature = "rkyv", rkyv(omit_bounds))] HashMap<String, Input>),
}

impl Input {
    /// Empty map (`Input::Map`).
    pub fn new_map() -> Self {
        Self::Map(HashMap::new())
    }

    /// Empty list (`Input::List`).
    pub fn new_list() -> Self {
        Self::List(Vec::new())
    }

    /// Empty string (`Input::Str`).
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

    /// Kind of the active variant.
    pub fn type_name(&self) -> InputType {
        match self {
            Self::Bool(_) => InputType::Bool,
            Self::Int(_) => InputType::Int,
            Self::Float(_) => InputType::Float,
            Self::Str(_) => InputType::Str,
            Self::List(_) => InputType::List,
            Self::Map(_) => InputType::Map,
        }
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

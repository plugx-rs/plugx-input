pub mod common;
mod types;

pub use common::{
    number::InputSchemaTypeNumberValue, range::InputSchemaTypeRange, regex::InputSchemaTypeRegex,
    size::InputSchemaTypeSize,
};
pub use types::{
    any::InputSchemaTypeAny, boolean::InputSchemaTypeBoolean,
    dynamic_map::InputSchemaTypeDynamicMap, either::InputSchemaTypeEither,
    float::InputSchemaTypeFloat, fs::InputSchemaTypeFs, integer::InputSchemaTypeInteger,
    ip::InputSchemaTypeIp, list::InputSchemaTypeList, log_level::InputSchemaTypeLogLevel,
    log_level_filter::InputSchemaTypeLogLevelFilter, number::InputSchemaTypeNumber,
    port::InputSchemaTypePort, r#enum::InputSchemaTypeEnum,
    socket_address::InputSchemaTypeSocketAddress, static_map::InputSchemaTypeStaticMap,
    string::InputSchemaTypeString,
};

use crate::{position::InputPosition, Input};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(
    rename_all = "snake_case",
    deny_unknown_fields,
    expecting = "Expecting an object with key `schema` and optionally a `default` key containing default value"
)]
pub struct InputSchema {
    #[serde(rename = "schema")]
    pub(crate) schema_type: Box<InputSchemaType>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub(crate) maybe_default: Option<Input>,
}

impl Display for InputSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref default) = self.maybe_default {
            f.write_str(format!("{} with default value {default}", self.schema_type).as_str())
        } else {
            f.write_str(format!("{}", self.schema_type).as_str())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type", deny_unknown_fields)]
pub enum InputSchemaType {
    Any(InputSchemaTypeAny),
    Boolean(InputSchemaTypeBoolean),
    Number(InputSchemaTypeNumber),
    Integer(InputSchemaTypeInteger),
    Float(InputSchemaTypeFloat),
    String(InputSchemaTypeString),
    List(InputSchemaTypeList),
    StaticMap(InputSchemaTypeStaticMap),
    DynamicMap(InputSchemaTypeDynamicMap),
    Enum(InputSchemaTypeEnum),
    Either(InputSchemaTypeEither),
    Fs(InputSchemaTypeFs),
    LogLevel(InputSchemaTypeLogLevel),
    LogLevelFilter(InputSchemaTypeLogLevelFilter),
    Ip(InputSchemaTypeIp),
    Port(InputSchemaTypePort),
    SocketAddress(InputSchemaTypeSocketAddress),
}

impl Display for InputSchemaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Any(any) => format!("{any}"),
            Self::Boolean(boolean) => format!("{boolean}"),
            Self::Number(number) => format!("{number}"),
            Self::Integer(integer) => format!("{integer}"),
            Self::Float(float) => format!("{float}"),
            Self::String(string) => format!("{string}"),
            Self::List(list) => format!("{list}"),
            Self::StaticMap(static_map) => format!("{static_map}"),
            Self::DynamicMap(dynamic_map) => format!("{dynamic_map}"),
            Self::Enum(enum_) => format!("{enum_}"),
            Self::Either(either) => format!("{either}"),
            Self::Fs(fs) => format!("{fs}"),
            Self::LogLevel(log_level) => format!("{log_level}"),
            Self::LogLevelFilter(log_level_filter) => format!("{log_level_filter}"),
            Self::Ip(ip) => format!("{ip}"),
            Self::Port(port) => format!("{port}"),
            Self::SocketAddress(socket_address) => format!("{socket_address}"),
        };
        f.write_str(text.as_str())
    }
}

impl InputSchemaType {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        match self {
            Self::Any(any) => any.validate(input, maybe_position),
            Self::Boolean(boolean) => boolean.validate(input, maybe_position),
            Self::Number(number) => number.validate(input, maybe_position),
            Self::Integer(integer) => integer.validate(input, maybe_position),
            Self::Float(float) => float.validate(input, maybe_position),
            Self::String(string) => string.validate(input, maybe_position),
            Self::List(list) => list.validate(input, maybe_position),
            Self::StaticMap(static_map) => static_map.validate(input, maybe_position),
            Self::DynamicMap(dynamic_map) => dynamic_map.validate(input, maybe_position),
            Self::Enum(enum_) => enum_.validate(input, maybe_position),
            Self::Either(either) => either.validate(input, maybe_position),
            Self::Fs(fs) => fs.validate(input, maybe_position),
            Self::LogLevel(log_level) => log_level.validate(input, maybe_position),
            Self::LogLevelFilter(log_level_filter) => {
                log_level_filter.validate(input, maybe_position)
            }
            Self::Ip(ip) => ip.validate(input, maybe_position),
            Self::Port(port) => port.validate(input, maybe_position),
            Self::SocketAddress(socket_address) => socket_address.validate(input, maybe_position),
        }
    }
}

impl InputSchemaType {
    pub fn new_any() -> Self {
        Self::Any(Default::default())
    }

    pub fn new_boolean() -> Self {
        Self::Boolean(Default::default())
    }

    pub fn new_number() -> Self {
        Self::Number(Default::default())
    }

    pub fn new_integer() -> Self {
        Self::Integer(Default::default())
    }

    pub fn new_float() -> Self {
        Self::Float(Default::default())
    }

    pub fn new_string() -> Self {
        Self::String(Default::default())
    }

    pub fn new_list() -> Self {
        Self::List(Default::default())
    }

    pub fn new_static_map() -> Self {
        Self::StaticMap(Default::default())
    }

    pub fn new_dynamic_map() -> Self {
        Self::DynamicMap(Default::default())
    }

    pub fn new_enum() -> Self {
        Self::Enum(Default::default())
    }

    pub fn new_either() -> Self {
        Self::Either(Default::default())
    }

    pub fn new_fs() -> Self {
        Self::Fs(Default::default())
    }

    pub fn new_log_level() -> Self {
        Self::LogLevel(Default::default())
    }

    pub fn new_log_level_filter() -> Self {
        Self::LogLevelFilter(Default::default())
    }

    pub fn new_ip() -> Self {
        Self::Ip(Default::default())
    }

    pub fn new_port() -> Self {
        Self::Port(Default::default())
    }

    pub fn new_socket_address() -> Self {
        Self::SocketAddress(Default::default())
    }
}

impl InputSchemaType {
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub fn is_static_map(&self) -> bool {
        matches!(self, Self::StaticMap(_))
    }

    pub fn is_dynamic_map(&self) -> bool {
        matches!(self, Self::DynamicMap(_))
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    pub fn is_either(&self) -> bool {
        matches!(self, Self::Either(_))
    }

    pub fn is_fs(&self) -> bool {
        matches!(self, Self::Fs(_))
    }

    pub fn is_log_level(&self) -> bool {
        matches!(self, Self::LogLevel(_))
    }

    pub fn is_log_level_filter(&self) -> bool {
        matches!(self, Self::LogLevelFilter(_))
    }

    pub fn is_ip(&self) -> bool {
        matches!(self, Self::Ip(_))
    }

    pub fn is_port(&self) -> bool {
        matches!(self, Self::Port(_))
    }

    pub fn is_socket_address(&self) -> bool {
        matches!(self, Self::SocketAddress(_))
    }
}

impl InputSchemaType {
    pub fn as_any(&self) -> &InputSchemaTypeAny {
        if let Self::Any(any) = self {
            any
        } else {
            panic!(
                "`&self` is not `Any`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_boolean(&self) -> &InputSchemaTypeBoolean {
        if let Self::Boolean(boolean) = self {
            boolean
        } else {
            panic!(
                "`&self` is not `Boolean`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_number(&self) -> &InputSchemaTypeNumber {
        if let Self::Number(number) = self {
            number
        } else {
            panic!(
                "`&self` is not `Number`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_integer(&self) -> &InputSchemaTypeInteger {
        if let Self::Integer(integer) = self {
            integer
        } else {
            panic!(
                "`&self` is not `Integer`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_float(&self) -> &InputSchemaTypeFloat {
        if let Self::Float(float) = self {
            float
        } else {
            panic!(
                "`&self` is not `Float`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_string(&self) -> &InputSchemaTypeString {
        if let Self::String(string) = self {
            string
        } else {
            panic!(
                "`&self` is not `String`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_list(&self) -> &InputSchemaTypeList {
        if let Self::List(list) = self {
            list
        } else {
            panic!(
                "`&self` is not `List`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_static_map(&self) -> &InputSchemaTypeStaticMap {
        if let Self::StaticMap(static_map) = self {
            static_map
        } else {
            panic!(
                "`&self` is not `StaticMap`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_dynamic_map(&self) -> &InputSchemaTypeDynamicMap {
        if let Self::DynamicMap(dynamic_map) = self {
            dynamic_map
        } else {
            panic!(
                "`&self` is not `DynamicMap`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_enum(&self) -> &InputSchemaTypeEnum {
        if let Self::Enum(enum_) = self {
            enum_
        } else {
            panic!(
                "`&self` is not `Enum`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_either(&self) -> &InputSchemaTypeEither {
        if let Self::Either(either) = self {
            either
        } else {
            panic!(
                "`&self` is not `Either`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_fs(&self) -> &InputSchemaTypeFs {
        if let Self::Fs(fs) = self {
            fs
        } else {
            panic!(
                "`&self` is not `Fs`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_log_level(&self) -> &InputSchemaTypeLogLevel {
        if let Self::LogLevel(log_level) = self {
            log_level
        } else {
            panic!(
                "`&self` is not `LogLevel`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_log_level_filter(&self) -> &InputSchemaTypeLogLevelFilter {
        if let Self::LogLevelFilter(log_level_filter) = self {
            log_level_filter
        } else {
            panic!(
                "`&self` is not `LogLevelFilter`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_ip(&self) -> &InputSchemaTypeIp {
        if let Self::Ip(ip) = self {
            ip
        } else {
            panic!(
                "`&self` is not `Ip`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_port(&self) -> &InputSchemaTypePort {
        if let Self::Port(port) = self {
            port
        } else {
            panic!(
                "`&self` is not `Port`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }

    pub fn as_socket_address(&self) -> &InputSchemaTypeSocketAddress {
        if let Self::SocketAddress(socket_address) = self {
            socket_address
        } else {
            panic!(
                "`&self` is not `SocketAddress`. You should call `is_<TYPE>()` method before using any `as_<TYPE>()` method."
            )
        }
    }
}

impl InputSchemaType {
    pub fn mut_any(&mut self) -> &mut InputSchemaTypeAny {
        if let Self::Any(any) = self {
            any
        } else {
            panic!(
                "`&self` is not `Any`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_boolean(&mut self) -> &mut InputSchemaTypeBoolean {
        if let Self::Boolean(boolean) = self {
            boolean
        } else {
            panic!(
                "`&self` is not `Boolean`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_number(&mut self) -> &mut InputSchemaTypeNumber {
        if let Self::Number(number) = self {
            number
        } else {
            panic!(
                "`&self` is not `Number`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_integer(&mut self) -> &mut InputSchemaTypeInteger {
        if let Self::Integer(integer) = self {
            integer
        } else {
            panic!(
                "`&self` is not `Integer`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_float(&mut self) -> &mut InputSchemaTypeFloat {
        if let Self::Float(float) = self {
            float
        } else {
            panic!(
                "`&self` is not `Float`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_string(&mut self) -> &mut InputSchemaTypeString {
        if let Self::String(string) = self {
            string
        } else {
            panic!(
                "`&self` is not `String`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_list(&mut self) -> &mut InputSchemaTypeList {
        if let Self::List(list) = self {
            list
        } else {
            panic!(
                "`&self` is not `List`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_static_map(&mut self) -> &mut InputSchemaTypeStaticMap {
        if let Self::StaticMap(static_map) = self {
            static_map
        } else {
            panic!(
                "`&self` is not `StaticMap`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_dynamic_map(&mut self) -> &mut InputSchemaTypeDynamicMap {
        if let Self::DynamicMap(dynamic_map) = self {
            dynamic_map
        } else {
            panic!(
                "`&self` is not `DynamicMap`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_enum(&mut self) -> &mut InputSchemaTypeEnum {
        if let Self::Enum(enum_) = self {
            enum_
        } else {
            panic!(
                "`&self` is not `Enum`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_either(&mut self) -> &mut InputSchemaTypeEither {
        if let Self::Either(either) = self {
            either
        } else {
            panic!(
                "`&self` is not `Either`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_fs(&mut self) -> &mut InputSchemaTypeFs {
        if let Self::Fs(fs) = self {
            fs
        } else {
            panic!(
                "`&self` is not `Fs`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_log_level(&mut self) -> &mut InputSchemaTypeLogLevel {
        if let Self::LogLevel(log_level) = self {
            log_level
        } else {
            panic!(
                "`&self` is not `LogLevel`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_log_level_filter(&mut self) -> &mut InputSchemaTypeLogLevelFilter {
        if let Self::LogLevelFilter(log_level_filter) = self {
            log_level_filter
        } else {
            panic!(
                "`&self` is not `LogLevelFilter`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_ip(&mut self) -> &mut InputSchemaTypeIp {
        if let Self::Ip(ip) = self {
            ip
        } else {
            panic!(
                "`&self` is not `Ip`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_port(&mut self) -> &mut InputSchemaTypePort {
        if let Self::Port(port) = self {
            port
        } else {
            panic!(
                "`&self` is not `Port`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }

    pub fn mut_socket_address(&mut self) -> &mut InputSchemaTypeSocketAddress {
        if let Self::SocketAddress(socket_address) = self {
            socket_address
        } else {
            panic!(
                "`&self` is not `SocketAddress`. You should call `is_<TYPE>()` method before using any `mut_<TYPE>()` method."
            )
        }
    }
}

impl Default for InputSchemaType {
    fn default() -> Self {
        Self::new_any()
    }
}

impl From<InputSchemaType> for InputSchema {
    fn from(schema_type: InputSchemaType) -> Self {
        Self {
            schema_type: Box::new(schema_type),
            maybe_default: None,
        }
    }
}

impl From<InputSchema> for InputSchemaType {
    fn from(schema: InputSchema) -> Self {
        *schema.schema_type
    }
}

impl From<&InputSchema> for InputSchemaType {
    fn from(schema: &InputSchema) -> Self {
        *schema.schema_type.clone()
    }
}

impl AsRef<InputSchemaType> for InputSchema {
    fn as_ref(&self) -> &InputSchemaType {
        &self.schema_type
    }
}

impl AsMut<InputSchemaType> for InputSchema {
    fn as_mut(&mut self) -> &mut InputSchemaType {
        &mut self.schema_type
    }
}

impl From<InputSchemaTypeAny> for InputSchemaType {
    fn from(any: InputSchemaTypeAny) -> Self {
        Self::Any(any)
    }
}

impl From<InputSchemaTypeEither> for InputSchemaType {
    fn from(either: InputSchemaTypeEither) -> Self {
        Self::Either(either)
    }
}

impl From<InputSchemaTypeEnum> for InputSchemaType {
    fn from(enum_: InputSchemaTypeEnum) -> Self {
        Self::Enum(enum_)
    }
}

pub(crate) mod default {
    #[inline(always)]
    pub fn default_true() -> bool {
        true
    }

    #[inline(always)]
    pub fn default_port_zero() -> u16 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum InputSchemaError {
    #[error("Expected `{expected_type}` type, got `{input_type}`")]
    Type {
        position: InputPosition,
        expected_type: String,
        input_type: String,
    },
    #[error("Expected {schema_type:?}, got `{input}`")]
    Schema {
        position: InputPosition,
        schema_type: InputSchemaType,
        input: Input,
    },
    #[error(
        "{position} Expected {schema_type} with {expected_size}, but the input size is {size}"
    )]
    Size {
        position: InputPosition,
        schema_type: InputSchemaType,
        expected_size: InputSchemaTypeSize,
        size: usize,
    },
    #[error("{position} Expected {schema_type} with {expected_range}, but the input is {input}")]
    Range {
        position: InputPosition,
        schema_type: InputSchemaType,
        expected_range: InputSchemaTypeRange,
        input: Input,
    },
    #[error("{position} is not set (expected {schema_type:?})")]
    NotFound {
        position: InputPosition,
        schema_type: InputSchemaType,
    },
    #[error("{position} {description}")]
    Invalid {
        description: String,
        position: InputPosition,
    },
}

impl InputSchema {
    pub fn new() -> Self {
        Self {
            schema_type: Default::default(),
            maybe_default: Default::default(),
        }
    }

    pub fn schema_type(&self) -> &InputSchemaType {
        &self.schema_type
    }

    pub fn schema_type_mut(&mut self) -> &mut InputSchemaType {
        self.schema_type.as_mut()
    }

    pub fn set_schema_type<T: Into<InputSchemaType>>(&mut self, schema_type: T) {
        self.schema_type = Box::new(schema_type.into());
    }

    pub fn with_schema_type<T: Into<InputSchemaType>>(mut self, schema_type: T) -> Self {
        self.set_schema_type(schema_type);
        self
    }

    pub fn maybe_default(&self) -> Option<&Input> {
        self.maybe_default.as_ref()
    }

    pub fn maybe_default_mut(&mut self) -> Option<&mut Input> {
        self.maybe_default.as_mut()
    }

    pub fn set_default<T: Into<Input>>(&mut self, default: T) {
        self.maybe_default = Some(default.into())
    }

    pub fn with_default<T: Into<Input>>(mut self, default: T) -> Self {
        self.set_default(default);
        self
    }

    pub fn set_maybe_default<T: Into<Input>>(&mut self, default: Option<T>) {
        self.maybe_default = default.map(|input| input.into());
    }

    pub fn with_maybe_default<T: Into<Input>>(mut self, default: Option<T>) -> Self {
        self.set_maybe_default(default);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serde() {
        let json = json!({
            "schema": {
                "type": "fs", "access": "rw"
            }
        });
        let decoded: InputSchema =
            serde_json::from_str(&serde_json::to_string_pretty(&json).unwrap()).unwrap();
        println!("\n\n\n{decoded}\n\n\n");
    }
}

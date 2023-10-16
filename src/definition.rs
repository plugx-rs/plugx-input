use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    path::Path,
};

use crate::Input;

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub struct InputDefinition {
    #[serde(alias = "definition", default)]
    pub(crate) definition_type: Box<InputDefinitionType>,
    #[serde(alias = "default", skip_serializing_if = "Option::is_none")]
    pub(crate) maybe_default: Option<Input>,
}

impl InputDefinition {
    pub fn new() -> Self {
        Self {
            definition_type: Box::new(InputDefinitionType::Any),
            maybe_default: None,
        }
    }

    pub fn definition_type(&self) -> &InputDefinitionType {
        &self.definition_type
    }

    pub fn definition_type_mut(&mut self) -> &mut InputDefinitionType {
        self.definition_type.as_mut()
    }

    pub fn set_definition_type<T: Into<InputDefinitionType>>(&mut self, definition_type: T) {
        self.definition_type = Box::new(definition_type.into());
    }

    pub fn with_definition_type<T: Into<InputDefinitionType>>(
        mut self,
        definition_type: T,
    ) -> Self {
        self.set_definition_type(definition_type);
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

impl Display for InputDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let definition_type = &self.definition_type;
        f.write_str(
            if self.maybe_default.is_some()
                && !(definition_type.is_list()
                    || definition_type.is_dynamic_map()
                    || definition_type.is_static_map()
                    || definition_type.is_either())
            {
                format!(
                    "{definition_type} with default value `{}`",
                    self.maybe_default.as_ref().unwrap()
                )
            } else {
                format!("{definition_type}")
            }
            .as_str(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type", deny_unknown_fields)]
pub enum InputDefinitionType {
    Any,
    Boolean,
    Integer {
        #[serde(rename = "range", skip_serializing_if = "Option::is_none", default)]
        maybe_range: Option<InputDefinitionRangeInteger>,
    },
    Float {
        #[serde(rename = "range", skip_serializing_if = "Option::is_none", default)]
        maybe_range: Option<InputDefinitionRangeFloat>,
    },
    String {
        #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
        maybe_size: Option<InputDefinitionSize>,
    },
    List {
        #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
        maybe_size: Option<InputDefinitionSize>,
        #[serde(rename = "definition", default)]
        item_definition: Box<InputDefinitionType>,
    },
    StaticMap {
        #[serde(rename = "definitions", default)]
        item_definitions: HashMap<String, InputDefinition>,
    },
    DynamicMap {
        #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
        maybe_size: Option<InputDefinitionSize>,
        #[serde(rename = "definition", default)]
        item_definition: Box<InputDefinitionType>,
    },
    Enum {
        #[serde(rename = "items")]
        item_list: Vec<Input>,
    },
    Either {
        #[serde(rename = "definitions", default)]
        item_definition_list: Vec<InputDefinitionType>,
    },
    Path {
        file_type: Option<InputDefinitionPathType>,
        #[serde(default)]
        error_if_not_found: bool,
        #[serde(default)]
        access: Vec<InputDefinitionAccessFlag>,
        absolute: Option<bool>,
    },
}

impl Default for InputDefinitionType {
    fn default() -> Self {
        Self::Any
    }
}

impl InputDefinitionType {
    pub fn any() -> Self {
        Self::Any
    }

    pub fn is_any(&self) -> bool {
        self == &Self::Any
    }
}

impl InputDefinitionType {
    pub fn boolean() -> Self {
        Self::Boolean
    }

    pub fn is_boolean(&self) -> bool {
        self == &Self::Boolean
    }
}

impl InputDefinitionType {
    pub fn integer() -> Self {
        Self::Integer { maybe_range: None }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer { .. })
    }

    pub fn maybe_integer_range(&self) -> Option<&InputDefinitionRangeInteger> {
        assert!(self.is_integer(), "definition should be integer");
        if let Self::Integer { maybe_range } = self {
            maybe_range.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn maybe_integer_range_mut(&mut self) -> &mut Option<InputDefinitionRangeInteger> {
        assert!(self.is_integer(), "definition should be integer");
        if let Self::Integer { maybe_range } = self {
            maybe_range
        } else {
            unreachable!()
        }
    }

    pub fn set_integer_range<T: Into<InputDefinitionRangeInteger>>(&mut self, range: T) {
        assert!(self.is_integer(), "definition should be integer");
        *self.maybe_integer_range_mut() = Some(range.into());
    }

    pub fn with_integer_range<T: Into<InputDefinitionRangeInteger>>(mut self, range: T) -> Self {
        self.set_integer_range(range);
        self
    }
}

impl InputDefinitionType {
    pub fn float() -> Self {
        Self::Float { maybe_range: None }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float { .. })
    }

    pub fn maybe_float_range(&self) -> Option<&InputDefinitionRangeFloat> {
        assert!(self.is_float(), "definition should be float");
        if let Self::Float { maybe_range } = self {
            maybe_range.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn maybe_float_range_mut(&mut self) -> &mut Option<InputDefinitionRangeFloat> {
        assert!(self.is_float(), "definition should be float");
        if let Self::Float { maybe_range } = self {
            maybe_range
        } else {
            unreachable!()
        }
    }

    pub fn set_float_range<T: Into<InputDefinitionRangeFloat>>(&mut self, range: T) {
        assert!(self.is_float(), "definition should be float");
        *self.maybe_float_range_mut() = Some(range.into());
    }

    pub fn with_float_range<T: Into<InputDefinitionRangeFloat>>(mut self, range: T) -> Self {
        self.set_float_range(range);
        self
    }
}

impl InputDefinitionType {
    pub fn string() -> Self {
        Self::String { maybe_size: None }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    pub fn maybe_string_size(&self) -> Option<&InputDefinitionSize> {
        assert!(self.is_string(), "definition should be string");
        if let Self::String { maybe_size } = self {
            maybe_size.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn maybe_string_size_mut(&mut self) -> &mut Option<InputDefinitionSize> {
        assert!(self.is_string(), "definition should be string");
        if let Self::String { maybe_size } = self {
            maybe_size
        } else {
            unreachable!()
        }
    }

    pub fn set_string_size<T: Into<InputDefinitionSize>>(&mut self, size: T) {
        assert!(self.is_string(), "definition should be string");
        *self.maybe_string_size_mut() = Some(size.into());
    }

    pub fn with_string_size<T: Into<InputDefinitionSize>>(mut self, size: T) -> Self {
        self.set_string_size(size);
        self
    }
}

impl InputDefinitionType {
    pub fn list() -> Self {
        Self::List {
            maybe_size: None,
            item_definition: Box::new(Self::any()),
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List { .. })
    }

    pub fn list_item_definition(&self) -> &InputDefinitionType {
        assert!(self.is_list(), "definition should be list");
        if let Self::List {
            item_definition, ..
        } = self
        {
            item_definition
        } else {
            unreachable!()
        }
    }

    pub fn list_item_definition_mut(&mut self) -> &mut InputDefinitionType {
        assert!(self.is_list(), "definition should be list");
        if let Self::List {
            item_definition, ..
        } = self
        {
            item_definition
        } else {
            unreachable!()
        }
    }

    pub fn set_list_item_definition<T: Into<InputDefinitionType>>(&mut self, definition: T) {
        assert!(self.is_list(), "definition should be list");
        *self.list_item_definition_mut() = definition.into();
    }

    pub fn with_list_item_definition<T: Into<InputDefinitionType>>(
        mut self,
        definition: T,
    ) -> Self {
        self.set_list_item_definition(definition);
        self
    }

    pub fn maybe_list_size(&self) -> Option<&InputDefinitionSize> {
        assert!(self.is_list(), "definition should be list");
        if let Self::List { maybe_size, .. } = self {
            maybe_size.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn maybe_list_size_mut(&mut self) -> &mut Option<InputDefinitionSize> {
        assert!(self.is_list(), "definition should be list");
        if let Self::List { maybe_size, .. } = self {
            maybe_size
        } else {
            unreachable!()
        }
    }

    pub fn set_list_size<T: Into<InputDefinitionSize>>(&mut self, size: T) {
        assert!(self.is_list(), "definition should be list");
        *self.maybe_list_size_mut() = Some(size.into());
    }

    pub fn with_list_size<T: Into<InputDefinitionSize>>(mut self, size: T) -> Self {
        self.set_list_size(size);
        self
    }
}

impl InputDefinitionType {
    pub fn static_map() -> Self {
        Self::StaticMap {
            item_definitions: HashMap::new(),
        }
    }

    pub fn is_static_map(&self) -> bool {
        matches!(self, Self::StaticMap { .. })
    }

    pub fn static_map_definitions(&self) -> &HashMap<String, InputDefinition> {
        assert!(self.is_static_map(), "definition should be static map");
        if let Self::StaticMap {
            item_definitions, ..
        } = self
        {
            item_definitions
        } else {
            unreachable!()
        }
    }

    pub fn static_map_definitions_mut(&mut self) -> &mut HashMap<String, InputDefinition> {
        assert!(self.is_static_map(), "definition should be static map");
        if let Self::StaticMap {
            item_definitions, ..
        } = self
        {
            item_definitions
        } else {
            unreachable!()
        }
    }

    pub fn set_static_map_definition<K: Into<String>, V: Into<InputDefinition>>(
        &mut self,
        key: K,
        definition: V,
    ) {
        self.static_map_definitions_mut()
            .insert(key.into(), definition.into());
    }

    pub fn with_static_map_definition<K: Into<String>, V: Into<InputDefinition>>(
        mut self,
        key: K,
        definition: V,
    ) -> Self {
        self.set_static_map_definition(key, definition);
        self
    }
}

impl InputDefinitionType {
    pub fn dynamic_map() -> Self {
        Self::DynamicMap {
            maybe_size: None,
            item_definition: Box::new(Self::any()),
        }
    }

    pub fn is_dynamic_map(&self) -> bool {
        matches!(self, Self::DynamicMap { .. })
    }

    pub fn dynamic_map_item_definition(&self) -> &InputDefinitionType {
        assert!(self.is_dynamic_map(), "definition should be dynamic map");
        if let Self::DynamicMap {
            item_definition, ..
        } = self
        {
            item_definition
        } else {
            unreachable!()
        }
    }

    pub fn dynamic_map_item_definition_mut(&mut self) -> &mut InputDefinitionType {
        assert!(self.is_dynamic_map(), "definition should be dynamic map");
        if let Self::DynamicMap {
            item_definition, ..
        } = self
        {
            item_definition
        } else {
            unreachable!()
        }
    }

    pub fn set_dynamic_map_item_definition<T: Into<InputDefinitionType>>(&mut self, definition: T) {
        assert!(self.is_dynamic_map(), "definition should be dynamic map");
        *self.dynamic_map_item_definition_mut() = definition.into();
    }

    pub fn with_dynamic_map_item_definition<T: Into<InputDefinitionType>>(
        mut self,
        definition: T,
    ) -> Self {
        self.set_dynamic_map_item_definition(definition);
        self
    }

    pub fn maybe_dynamic_map_size(&self) -> Option<&InputDefinitionSize> {
        assert!(self.is_dynamic_map(), "definition should be dynamic map");
        if let Self::DynamicMap { maybe_size, .. } = self {
            maybe_size.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn maybe_dynamic_map_size_mut(&mut self) -> Option<&mut InputDefinitionSize> {
        assert!(self.is_dynamic_map(), "definition should be dynamic map");
        if let Self::DynamicMap { maybe_size, .. } = self {
            maybe_size.as_mut()
        } else {
            unreachable!()
        }
    }

    pub fn set_dynamic_map_size<T: Into<InputDefinitionSize>>(&mut self, size: T) {
        assert!(self.is_dynamic_map(), "definition should be dynamic map");
        self.maybe_dynamic_map_size_mut().replace(&mut size.into());
    }

    pub fn with_dynamic_map_size<T: Into<InputDefinitionSize>>(mut self, size: T) -> Self {
        self.set_dynamic_map_size(size);
        self
    }
}

impl InputDefinitionType {
    pub fn enum_() -> Self {
        Self::Enum {
            item_list: Vec::new(),
        }
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum { .. })
    }

    pub fn enum_item_list(&self) -> &Vec<Input> {
        assert!(self.is_enum(), "definition should be enum");
        if let Self::Enum { item_list, .. } = self {
            item_list
        } else {
            unreachable!()
        }
    }

    pub fn enum_item_list_mut(&mut self) -> &mut Vec<Input> {
        assert!(self.is_enum(), "definition should be enum");
        if let Self::Enum { item_list, .. } = self {
            item_list
        } else {
            unreachable!()
        }
    }

    pub fn set_enum_item_list<T: Into<Input>>(&mut self, item_list: Vec<T>) {
        assert!(self.is_enum(), "definition should be enum");
        *self.enum_item_list_mut() = item_list.into_iter().map(|input| input.into()).collect();
    }

    pub fn with_enum_item_list<T: Into<Input>>(mut self, item_list: Vec<T>) -> Self {
        self.set_enum_item_list(item_list);
        self
    }

    pub fn set_enum_item<T: Into<Input>>(&mut self, item: T) {
        assert!(self.is_enum(), "definition should be enum");
        self.enum_item_list_mut().push(item.into());
    }

    pub fn with_enum_item<T: Into<Input>>(mut self, item: T) -> Self {
        self.set_enum_item(item);
        self
    }
}

impl InputDefinitionType {
    pub fn either() -> Self {
        Self::Either {
            item_definition_list: Vec::new(),
        }
    }

    pub fn is_either(&self) -> bool {
        matches!(self, Self::Either { .. })
    }

    pub fn either_definition_list(&self) -> &Vec<InputDefinitionType> {
        assert!(self.is_either(), "definition should be `either`");
        if let Self::Either {
            item_definition_list,
            ..
        } = self
        {
            item_definition_list
        } else {
            unreachable!()
        }
    }

    pub fn either_definition_list_mut(&mut self) -> &mut Vec<InputDefinitionType> {
        assert!(self.is_either(), "definition should be `either`");
        if let Self::Either {
            item_definition_list,
            ..
        } = self
        {
            item_definition_list
        } else {
            unreachable!()
        }
    }

    pub fn set_either_definition_list<T: Into<InputDefinitionType>>(
        &mut self,
        definition_list: Vec<T>,
    ) {
        assert!(self.is_either(), "definition should be `either`");
        *self.either_definition_list_mut() = definition_list
            .into_iter()
            .map(|definition| definition.into())
            .collect();
    }

    pub fn with_either_definition_list<T: Into<InputDefinitionType>>(
        mut self,
        definition_list: Vec<T>,
    ) -> Self {
        self.set_either_definition_list(definition_list);
        self
    }

    pub fn set_either_definition<T: Into<InputDefinitionType>>(&mut self, item: T) {
        assert!(self.is_either(), "definition should be `either`");
        self.either_definition_list_mut().push(item.into());
    }

    pub fn with_either_definition<T: Into<InputDefinitionType>>(mut self, item: T) -> Self {
        self.set_either_definition(item);
        self
    }
}

impl InputDefinitionType {
    pub fn path() -> Self {
        Self::Path {
            file_type: Default::default(),
            error_if_not_found: Default::default(),
            access: Default::default(),
            absolute: Default::default(),
        }
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path { .. })
    }

    pub fn path_type(&self) -> Option<&InputDefinitionPathType> {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path { file_type, .. } = self {
            file_type.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn path_type_mut(&mut self) -> &mut Option<InputDefinitionPathType> {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path { file_type, .. } = self {
            file_type
        } else {
            unreachable!()
        }
    }

    pub fn path_error_if_not_found(&self) -> bool {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path {
            error_if_not_found, ..
        } = self
        {
            *error_if_not_found
        } else {
            unreachable!()
        }
    }

    pub fn path_error_if_not_found_mut(&mut self) -> &mut bool {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path {
            error_if_not_found, ..
        } = self
        {
            error_if_not_found
        } else {
            unreachable!()
        }
    }

    pub fn path_access(&self) -> &Vec<InputDefinitionAccessFlag> {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path { access, .. } = self {
            access
        } else {
            unreachable!()
        }
    }

    pub fn path_access_mut(&mut self) -> &mut Vec<InputDefinitionAccessFlag> {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path { access, .. } = self {
            access
        } else {
            unreachable!()
        }
    }

    pub fn path_absolute(&self) -> Option<&bool> {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path { absolute, .. } = self {
            absolute.as_ref()
        } else {
            unreachable!()
        }
    }

    pub fn path_absolute_mut(&mut self) -> &mut Option<bool> {
        assert!(self.is_path(), "definition should be `path`");
        if let Self::Path { absolute, .. } = self {
            absolute
        } else {
            unreachable!()
        }
    }

    pub fn set_path_error_if_not_found(&mut self, flag: bool) {
        *self.path_error_if_not_found_mut() = flag;
    }

    pub fn with_path_error_if_not_found<T: AsRef<Path>>(mut self, flag: bool) -> Self {
        self.set_path_error_if_not_found(flag);
        self
    }

    pub fn add_path_access(&mut self, access: InputDefinitionAccessFlag) {
        self.path_access_mut().push(access);
    }

    pub fn with_path_access(mut self, access: InputDefinitionAccessFlag) -> Self {
        self.add_path_access(access);
        self
    }
}

impl Display for InputDefinitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => f.write_str("anything"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer { maybe_range } => {
                if let Some(range) = maybe_range {
                    f.write_str(format!("integer {range}").as_str())
                } else {
                    f.write_str("integer")
                }
            }
            Self::Float { maybe_range } => {
                if let Some(range) = maybe_range {
                    f.write_str(format!("float {range}").as_str())
                } else {
                    f.write_str("float")
                }
            }
            Self::String { maybe_size } => {
                if let Some(size) = maybe_size {
                    f.write_str(format!("string with length {size}").as_str())
                } else {
                    f.write_str("string")
                }
            }
            Self::List {
                maybe_size,
                item_definition,
            } => {
                if let Some(size) = maybe_size {
                    f.write_str(
                        format!(
                            "list with length {size} where each item should be {item_definition}"
                        )
                        .as_str(),
                    )
                } else {
                    f.write_str(
                        format!("list where each item should be {item_definition}").as_str(),
                    )
                }
            }
            Self::StaticMap {
                item_definitions, ..
            } => {
                if item_definitions.is_empty() {
                    f.write_str("static map")
                } else {
                    let mut key_values_text = String::new();
                    for (key, inner_definition) in item_definitions {
                        if !key_values_text.is_empty() {
                            key_values_text += ", "
                        }
                        key_values_text +=
                            format!("key `{key}` which should be {inner_definition}").as_str();
                    }
                    f.write_str(format!("static map with {}", key_values_text).as_str())
                }
            }
            Self::DynamicMap {
                item_definition, ..
            } => f.write_str(
                format!("dynamic map which each key should be {item_definition}").as_str(),
            ),
            Self::Enum { item_list } => f.write_str(
                format!(
                    "enum with possible values [{}]",
                    item_list
                        .iter()
                        .map(|input| format!("{input}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .as_str(),
            ),
            Self::Either {
                item_definition_list,
                ..
            } => f.write_str(
                format!(
                    "a value that must be {}",
                    item_definition_list
                        .iter()
                        .map(|definition| format!("{definition}"))
                        .collect::<Vec<_>>()
                        .join(" or ")
                )
                .as_str(),
            ),
            Self::Path {
                file_type,
                absolute,
                error_if_not_found,
                access,
            } => {
                let file_type = if let Some(file_type) = file_type {
                    if file_type.is_file() {
                        "file"
                    } else if file_type.is_directory() {
                        "directory"
                    } else if file_type.is_symlink() {
                        "symlink"
                    } else {
                        unreachable!()
                    }
                } else {
                    "path"
                }
                .to_string();
                let access = if access.is_empty() {
                    String::new()
                } else {
                    let mut read = if access.contains(&InputDefinitionAccessFlag::Read) {
                        " with read".to_string()
                    } else {
                        String::new()
                    };
                    let write = if access.contains(&InputDefinitionAccessFlag::Write) {
                        if read.is_empty() {
                            " with write access"
                        } else {
                            " and write access"
                        }
                        .to_string()
                    } else {
                        String::new()
                    };
                    if write.is_empty() {
                        read.push_str(" access")
                    }
                    format!("{read}{write}")
                };
                let absolute = if let Some(absolute) = absolute {
                    if *absolute {
                        " that should be absolute path"
                    } else {
                        " that should be relative path"
                    }
                    .to_string()
                } else {
                    String::new()
                };
                let error_if_not_found = if *error_if_not_found {
                    if absolute.is_empty() {
                        " that should exists"
                    } else {
                        " and should exists"
                    }
                    .to_string()
                } else {
                    String::new()
                };
                f.write_str(format!("{file_type}{access}{absolute}{error_if_not_found}").as_str())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum InputDefinitionSize {
    Max(usize),
    MinMax {
        #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
        maybe_max: Option<usize>,
        #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
        maybe_min: Option<usize>,
    },
}

impl Default for InputDefinitionSize {
    fn default() -> Self {
        Self::MinMax {
            maybe_min: None,
            maybe_max: None,
        }
    }
}

impl Display for InputDefinitionSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Max(max) => f.write_str(format!("less than `{max}`").as_str()),
            Self::MinMax {
                maybe_min,
                maybe_max,
            } => f.write_str(
                {
                    let mut text = if let Some(min) = maybe_min {
                        format!("bigger than `{min}`")
                    } else {
                        String::new()
                    };
                    if let Some(max) = maybe_max {
                        if maybe_min.is_some() {
                            text += " and "
                        };
                        text += format!("less than `{max}`").as_str()
                    };
                    text
                }
                .as_str(),
            ),
        }
    }
}

impl InputDefinitionSize {
    pub fn new_with_max<T: Into<usize>>(max: T) -> Self {
        Self::Max(max.into())
    }

    pub fn new_with_min<T: Into<usize>>(min: T) -> Self {
        Self::MinMax {
            maybe_max: None,
            maybe_min: Some(min.into()),
        }
    }

    pub fn new<T1: Into<usize>, T2: Into<usize>>(min: T1, max: T2) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }

    pub fn maybe_with_max<T: Into<usize>>(mut self, maybe_max: Option<T>) -> Self {
        let maybe_max = maybe_max.map(|max| max.into());
        self = match self {
            Self::Max(_) => Self::MinMax {
                maybe_max,
                maybe_min: None,
            },
            Self::MinMax { maybe_min, .. } => Self::MinMax {
                maybe_max,
                maybe_min,
            },
        };
        self
    }

    pub fn with_max<T: Into<usize>>(self, max: T) -> Self {
        self.maybe_with_max(Some(max))
    }

    pub fn maybe_set_max<T: Into<usize>>(&mut self, maybe_max: Option<T>) {
        *self = self.maybe_with_max(maybe_max);
    }

    pub fn set_max<T: Into<usize>>(&mut self, max: T) {
        self.maybe_set_max(Some(max));
    }

    pub fn maybe_with_min<T: Into<usize>>(mut self, maybe_min: Option<T>) -> Self {
        let maybe_min = maybe_min.map(|min| min.into());
        self = match self {
            Self::Max(max) => Self::MinMax {
                maybe_max: Some(max),
                maybe_min,
            },
            Self::MinMax { maybe_max, .. } => Self::MinMax {
                maybe_max,
                maybe_min,
            },
        };
        self
    }

    pub fn with_min<T: Into<usize>>(self, min: T) -> Self {
        self.maybe_with_min(Some(min))
    }

    pub fn maybe_set_min<T: Into<usize>>(&mut self, maybe_min: Option<T>) {
        *self = self.maybe_with_min(maybe_min);
    }

    pub fn set_min<T: Into<usize>>(&mut self, min: T) {
        self.maybe_set_min(Some(min));
    }

    pub fn maybe_min(&self) -> Option<usize> {
        if let Self::MinMax { maybe_min, .. } = self {
            *maybe_min
        } else {
            None
        }
    }

    pub fn maybe_min_mut(&mut self) -> &mut Option<usize> {
        match self {
            Self::Max(max) => {
                *self = Self::MinMax {
                    maybe_max: Some(*max),
                    maybe_min: None,
                };
                self.maybe_min_mut()
            }
            Self::MinMax { maybe_min, .. } => maybe_min,
        }
    }

    pub fn maybe_max(&self) -> Option<usize> {
        if let Self::MinMax { maybe_max, .. } = self {
            *maybe_max
        } else if let Self::Max(max) = self {
            Some(*max)
        } else {
            None
        }
    }

    pub fn maybe_max_mut(&mut self) -> &mut Option<usize> {
        match self {
            Self::Max(max) => {
                *self = Self::MinMax {
                    maybe_max: Some(*max),
                    maybe_min: None,
                };
                self.maybe_max_mut()
            }
            Self::MinMax { maybe_max, .. } => maybe_max,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum InputDefinitionRangeInteger {
    Max(isize),
    MinMax {
        #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
        maybe_max: Option<isize>,
        #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
        maybe_min: Option<isize>,
    },
}

impl Display for InputDefinitionRangeInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Max(max) => f.write_str(format!("less than `{max}`").as_str()),
            Self::MinMax {
                maybe_min,
                maybe_max,
            } => f.write_str(
                {
                    let mut text = if let Some(min) = maybe_min {
                        format!("bigger than `{min}`")
                    } else {
                        String::new()
                    };
                    if let Some(max) = maybe_max {
                        if maybe_min.is_some() {
                            text += " and "
                        };
                        text += format!("less than `{max}`").as_str()
                    };
                    text
                }
                .as_str(),
            ),
        }
    }
}

impl InputDefinitionRangeInteger {
    pub fn new_with_max<T: Into<isize>>(max: T) -> Self {
        Self::Max(max.into())
    }

    pub fn new_with_min<T: Into<isize>>(min: T) -> Self {
        Self::MinMax {
            maybe_max: None,
            maybe_min: Some(min.into()),
        }
    }

    pub fn new<T1: Into<isize>, T2: Into<isize>>(min: T1, max: T2) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }

    pub fn maybe_with_max<T: Into<isize>>(mut self, maybe_max: Option<T>) -> Self {
        let maybe_max = maybe_max.map(|max| max.into());
        self = match self {
            Self::Max(_) => Self::MinMax {
                maybe_max,
                maybe_min: None,
            },
            Self::MinMax { maybe_min, .. } => Self::MinMax {
                maybe_max,
                maybe_min,
            },
        };
        self
    }

    pub fn with_max<T: Into<isize>>(self, max: T) -> Self {
        self.maybe_with_max(Some(max))
    }

    pub fn maybe_set_max<T: Into<isize>>(&mut self, maybe_max: Option<T>) {
        *self = self.maybe_with_max(maybe_max);
    }

    pub fn set_max<T: Into<isize>>(&mut self, max: T) {
        self.maybe_set_max(Some(max));
    }

    pub fn maybe_with_min<T: Into<isize>>(mut self, maybe_min: Option<T>) -> Self {
        let maybe_min = maybe_min.map(|min| min.into());
        self = match self {
            Self::Max(max) => Self::MinMax {
                maybe_max: Some(max),
                maybe_min,
            },
            Self::MinMax { maybe_max, .. } => Self::MinMax {
                maybe_max,
                maybe_min,
            },
        };
        self
    }

    pub fn with_min<T: Into<isize>>(self, min: T) -> Self {
        self.maybe_with_min(Some(min))
    }

    pub fn maybe_set_min<T: Into<isize>>(&mut self, maybe_min: Option<T>) {
        *self = self.maybe_with_min(maybe_min);
    }

    pub fn set_min<T: Into<isize>>(&mut self, min: T) {
        self.maybe_set_min(Some(min));
    }

    pub fn maybe_min(&self) -> Option<isize> {
        if let Self::MinMax { maybe_min, .. } = self {
            *maybe_min
        } else {
            None
        }
    }

    pub fn maybe_min_mut(&mut self) -> &mut Option<isize> {
        match self {
            Self::Max(max) => {
                *self = Self::MinMax {
                    maybe_max: Some(*max),
                    maybe_min: None,
                };
                self.maybe_min_mut()
            }
            Self::MinMax { maybe_min, .. } => maybe_min,
        }
    }

    pub fn maybe_max(&self) -> Option<isize> {
        if let Self::MinMax { maybe_max, .. } = self {
            *maybe_max
        } else if let Self::Max(max) = self {
            Some(*max)
        } else {
            None
        }
    }

    pub fn maybe_max_mut(&mut self) -> &mut Option<isize> {
        match self {
            Self::Max(max) => {
                *self = Self::MinMax {
                    maybe_max: Some(*max),
                    maybe_min: None,
                };
                self.maybe_max_mut()
            }
            Self::MinMax { maybe_max, .. } => maybe_max,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged, deny_unknown_fields)]
pub enum InputDefinitionRangeFloat {
    Max(f64),
    MinMax {
        #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
        maybe_max: Option<f64>,
        #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
        maybe_min: Option<f64>,
    },
}

impl Display for InputDefinitionRangeFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Max(max) => f.write_str(format!("less than `{max}`").as_str()),
            Self::MinMax {
                maybe_min,
                maybe_max,
            } => f.write_str(
                {
                    let mut text = if let Some(min) = maybe_min {
                        format!("bigger than `{min}`")
                    } else {
                        String::new()
                    };
                    if let Some(max) = maybe_max {
                        if maybe_min.is_some() {
                            text += " and "
                        };
                        text += format!("less than `{max}`").as_str()
                    };
                    text
                }
                .as_str(),
            ),
        }
    }
}

impl InputDefinitionRangeFloat {
    pub fn new_with_max<T: Into<f64>>(max: T) -> Self {
        Self::Max(max.into())
    }

    pub fn new_with_min<T: Into<f64>>(min: T) -> Self {
        Self::MinMax {
            maybe_max: None,
            maybe_min: Some(min.into()),
        }
    }

    pub fn new<T1: Into<f64>, T2: Into<f64>>(min: T1, max: T2) -> Self {
        Self::MinMax {
            maybe_max: Some(max.into()),
            maybe_min: Some(min.into()),
        }
    }

    pub fn maybe_with_max<T: Into<f64>>(mut self, maybe_max: Option<T>) -> Self {
        let maybe_max = maybe_max.map(|max| max.into());
        self = match self {
            Self::Max(_) => Self::MinMax {
                maybe_max,
                maybe_min: None,
            },
            Self::MinMax { maybe_min, .. } => Self::MinMax {
                maybe_max,
                maybe_min,
            },
        };
        self
    }

    pub fn with_max<T: Into<f64>>(self, max: T) -> Self {
        self.maybe_with_max(Some(max))
    }

    pub fn maybe_set_max<T: Into<f64>>(&mut self, maybe_max: Option<T>) {
        *self = self.maybe_with_max(maybe_max);
    }

    pub fn set_max<T: Into<f64>>(&mut self, max: T) {
        self.maybe_set_max(Some(max));
    }

    pub fn maybe_with_min<T: Into<f64>>(mut self, maybe_min: Option<T>) -> Self {
        let maybe_min = maybe_min.map(|min| min.into());
        self = match self {
            Self::Max(max) => Self::MinMax {
                maybe_max: Some(max),
                maybe_min,
            },
            Self::MinMax { maybe_max, .. } => Self::MinMax {
                maybe_max,
                maybe_min,
            },
        };
        self
    }

    pub fn with_min<T: Into<f64>>(self, min: T) -> Self {
        self.maybe_with_min(Some(min))
    }

    pub fn maybe_set_min<T: Into<f64>>(&mut self, maybe_min: Option<T>) {
        *self = self.maybe_with_min(maybe_min);
    }

    pub fn set_min<T: Into<f64>>(&mut self, min: T) {
        self.maybe_set_min(Some(min));
    }

    pub fn maybe_min(&self) -> Option<f64> {
        if let Self::MinMax { maybe_min, .. } = self {
            *maybe_min
        } else {
            None
        }
    }

    pub fn maybe_min_mut(&mut self) -> &mut Option<f64> {
        match self {
            Self::Max(max) => {
                *self = Self::MinMax {
                    maybe_max: Some(*max),
                    maybe_min: None,
                };
                self.maybe_min_mut()
            }
            Self::MinMax { maybe_min, .. } => maybe_min,
        }
    }

    pub fn maybe_max(&self) -> Option<f64> {
        if let Self::MinMax { maybe_max, .. } = self {
            *maybe_max
        } else if let Self::Max(max) = self {
            Some(*max)
        } else {
            None
        }
    }

    pub fn maybe_max_mut(&mut self) -> &mut Option<f64> {
        match self {
            Self::Max(max) => {
                *self = Self::MinMax {
                    maybe_max: Some(*max),
                    maybe_min: None,
                };
                self.maybe_max_mut()
            }
            Self::MinMax { maybe_max, .. } => maybe_max,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InputDefinitionAccessFlag {
    #[serde(alias = "r")]
    Read,
    #[serde(alias = "w")]
    Write,
}

impl InputDefinitionAccessFlag {
    pub fn read() -> Self {
        Self::Read
    }

    pub fn write() -> Self {
        Self::Write
    }

    pub fn is_read_flag(&self) -> bool {
        matches!(self, Self::Read)
    }

    pub fn is_write_flag(&self) -> bool {
        matches!(self, Self::Write)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InputDefinitionPathType {
    #[serde(alias = "f")]
    File,
    #[serde(alias = "d")]
    Directory,
    #[serde(alias = "s")]
    Symlink,
}

impl InputDefinitionPathType {
    pub fn file() -> Self {
        Self::File
    }

    pub fn directory() -> Self {
        Self::Directory
    }

    pub fn symlink() -> Self {
        Self::Symlink
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File)
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory)
    }

    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink)
    }
}

// #[derive(Default, Clone, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "lowercase")]
// pub struct InputDefinitionRegex(String);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::{enable_logging, info};

    #[test]
    fn serde() {
        enable_logging();

        let json = serde_json::json!(
            [
                {"definition": {"type": "any"}},

                {"definition": {"type": "boolean"}, "default": false},

                {
                    "definition": {"type": "integer", "range": 10},
                    "default": 11
                },

                {
                    "definition": {"type": "float", "range": {"min": 3.14}},
                    "default": 0.0
                },

                {
                    "definition": {"type": "string", "size": {"max": 10, "min": 1}},
                    "default": "one_default"
                },

                {
                    "definition": {"type": "list", "size": 5},
                    "default": []
                },

                {
                    "definition": {"type": "list", "definition": {"type": "boolean"}},
                    "default": ["hello"]
                },

                {
                    "definition": {
                        "type": "static_map",
                        "definitions": {
                            "foo": {"definition": {"type": "boolean"}},
                            "bar": {"definition": {"type": "string"}, "default": "hello world"},
                        }
                    }
                },

                {
                    "definition": {
                        "type": "dynamic_map",
                        "definition": {"type": "integer", "range": 10}
                    }
                },

                {
                    "definition": {
                        "type": "enum",
                        "items": [true, 1, 2.0, "three", ["f", "o", "u", "r"], {"five": 6}]
                    }
                },

                {
                    "definition": {
                        "type": "either",
                        "definitions": [
                            {"type": "boolean"},
                            {"type": "enum", "items": ["yes", "no"]},
                            {"type": "integer", "range": {"min": 0, "max": 1}},
                        ]
                    }
                },
            ]
        );
        let json_str = json.to_string();
        info(&json_str);
        let definition_list = serde_json::from_str::<Vec<InputDefinition>>(&json_str).unwrap();
        definition_list
            .iter()
            .for_each(|definition| info(format!("{definition}")));

        let any = &definition_list[0];
        assert_eq!(
            any,
            &InputDefinition::new().with_definition_type(InputDefinitionType::any())
        );

        let boolean = &definition_list[1];
        assert_eq!(
            boolean,
            &InputDefinition::new()
                .with_definition_type(InputDefinitionType::boolean())
                .with_default(false)
        );

        let integer = &definition_list[2];
        assert_eq!(
            integer,
            &InputDefinition::new()
                .with_definition_type(
                    InputDefinitionType::integer()
                        .with_integer_range(InputDefinitionRangeInteger::new_with_max(10isize))
                )
                .with_default(11)
        );

        let float = &definition_list[3];
        assert_eq!(
            float,
            &InputDefinition::new()
                .with_definition_type(
                    InputDefinitionType::float()
                        .with_float_range(InputDefinitionRangeFloat::new_with_min(3.14))
                )
                .with_default(0.0)
        );

        let string = &definition_list[4];
        assert_eq!(
            string,
            &InputDefinition::new()
                .with_definition_type(
                    InputDefinitionType::string()
                        .with_string_size(InputDefinitionSize::new(1usize, 10usize))
                )
                .with_default("one_default"),
        );

        let list = &definition_list[5];
        assert_eq!(
            list,
            &InputDefinition::new()
                .with_definition_type(
                    InputDefinitionType::list()
                        .with_list_size(InputDefinitionSize::new_with_max(5usize))
                )
                .with_default(Vec::<String>::new()),
        );

        let list = &definition_list[6];
        assert_eq!(
            list,
            &InputDefinition::new()
                .with_definition_type(
                    InputDefinitionType::list()
                        .with_list_item_definition(InputDefinitionType::boolean())
                )
                .with_default(["hello"]),
        );

        let static_map = &definition_list[7];
        assert_eq!(
            static_map,
            &InputDefinition::new().with_definition_type(
                InputDefinitionType::static_map()
                    .with_static_map_definition(
                        "foo",
                        InputDefinition::new().with_definition_type(InputDefinitionType::boolean())
                    )
                    .with_static_map_definition(
                        "bar",
                        InputDefinition::new()
                            .with_definition_type(InputDefinitionType::string())
                            .with_default("hello world")
                    )
            ),
        );

        let dynamic_map = &definition_list[8];
        assert_eq!(
            dynamic_map,
            &InputDefinition::new().with_definition_type(
                InputDefinitionType::dynamic_map().with_dynamic_map_item_definition(
                    InputDefinitionType::integer()
                        .with_integer_range(InputDefinitionRangeInteger::new_with_max(10isize))
                )
            ),
        );

        let enum_ = &definition_list[9];
        assert_eq!(
            enum_,
            &InputDefinition::new().with_definition_type(
                InputDefinitionType::enum_()
                    .with_enum_item(true)
                    .with_enum_item(1)
                    .with_enum_item(2.0)
                    .with_enum_item("three")
                    .with_enum_item(["f", "o", "u", "r"])
                    .with_enum_item(HashMap::from([("five", 6)]))
            ),
        );

        let either = &definition_list[10];
        assert_eq!(
            either,
            &InputDefinition::new().with_definition_type(
                InputDefinitionType::either()
                    .with_either_definition(InputDefinitionType::boolean())
                    .with_either_definition(
                        InputDefinitionType::enum_()
                            .with_enum_item("yes")
                            .with_enum_item("no")
                    )
                    .with_either_definition(
                        InputDefinitionType::integer()
                            .with_integer_range(InputDefinitionRangeInteger::new(0isize, 1isize))
                    )
            ),
        );
    }
}

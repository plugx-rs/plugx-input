use crate::{
    position::InputPosition,
    schema::{InputSchema, InputSchemaError},
    Input,
};
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeStaticMap {
    items: HashMap<String, InputSchema>,
}

impl InputSchemaTypeStaticMap {
    pub fn items(&self) -> &HashMap<String, InputSchema> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut HashMap<String, InputSchema> {
        &mut self.items
    }

    pub fn set_item<K: ToString, S: Into<InputSchema>>(&mut self, key: K, schema: S) {
        self.items_mut().insert(key.to_string(), schema.into());
    }

    pub fn with_item<K: ToString, S: Into<InputSchema>>(mut self, key: K, schema: S) -> Self {
        self.set_item(key, schema);
        self
    }
}

impl InputSchemaTypeStaticMap {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if !input.is_map() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::map_type_name(),
                input_type: input.type_name(),
            });
        };
        let mut new_input = input.clone();
        let map = new_input.map_mut();
        let position = maybe_position.unwrap_or_default();
        for (key, inner_schema) in self.items.iter() {
            let inner_position = position.new_with_key(key);
            if let Some(inner_input) = map.get_mut(key) {
                inner_schema
                    .schema_type()
                    .validate(inner_input, Some(inner_position))?;
            } else if let Some(default) = inner_schema.maybe_default() {
                cfg_if! {
                    if #[cfg(feature = "tracing")] {
                        tracing::trace!(
                            position = %inner_position,
                            default = %default,
                            "using default value"
                        );
                    } else if #[cfg(feature = "logging")] {
                        log::trace!(
                            "position={:?} default={:?} message=\"using default value\"",
                            inner_position.to_string(),
                            default.to_string(),
                        );
                    }
                }
                map.insert(key.clone(), default.clone());
            } else {
                return Err(InputSchemaError::NotFound {
                    position: inner_position,
                    schema_type: inner_schema.schema_type().clone(),
                });
            }
        }
        *input = new_input;
        Ok(())
    }
}

impl Display for InputSchemaTypeStaticMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let length = self.items.len();
        if length == 1 {
            let (key, schema) = self.items.iter().next().unwrap();
            f.write_str(
                format!("static map with key `{key}` and value that should be {schema}").as_str(),
            )
        } else if length == 2 {
            let mut iter = self.items.iter();
            let (key1, schema1) = iter.next().unwrap();
            let (key2, schema2) = iter.next().unwrap();
            f.write_str(format!("static map with key `{key1}` and its value that should be {schema1} and another key `{key2}` and its value that should be {schema2}").as_str())
        } else if length == 0 {
            f.write_str("static map that should contain nothing")
        } else {
            let mut text = format!("static map with {length} different keys;");
            self.items
                .iter()
                .enumerate()
                .for_each(|(index, (key, schema))| {
                    text += if index + 1 == length {
                        format!("and finally key `{key}` and its value that should be {schema}")
                    } else {
                        format!("key `{key}` and its value that should be {schema}, ")
                    }
                    .as_str()
                });
            f.write_str(text.as_str())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

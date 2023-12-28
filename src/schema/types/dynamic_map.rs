use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType, InputSchemaTypeSize},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeDynamicMap {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
    maybe_size: Option<InputSchemaTypeSize>,
    item_schema: Box<InputSchemaType>,
}

impl InputSchemaTypeDynamicMap {
    pub fn item_schema(&self) -> &InputSchemaType {
        &self.item_schema
    }

    pub fn item_schema_mut(&mut self) -> &mut InputSchemaType {
        &mut self.item_schema
    }

    pub fn set_item_schema<S: Into<InputSchemaType>>(&mut self, schema_type: S) {
        *self.item_schema_mut() = schema_type.into();
    }

    pub fn with_item_schema<S: Into<InputSchemaType>>(mut self, schema_type: S) -> Self {
        self.set_item_schema(schema_type);
        self
    }

    pub fn maybe_size(&self) -> Option<&InputSchemaTypeSize> {
        self.maybe_size.as_ref()
    }

    pub fn maybe_size_mut(&mut self) -> &mut Option<InputSchemaTypeSize> {
        &mut self.maybe_size
    }

    pub fn set_size<S: Into<InputSchemaTypeSize>>(&mut self, size: S) {
        self.maybe_size_mut().replace(size.into());
    }

    pub fn with_size<S: Into<InputSchemaTypeSize>>(mut self, size: S) -> Self {
        self.set_size(size);
        self
    }
}

// impl InputSchemaTypeDynamicMap {
//     fn validate_input_type(&self, input: &Input) -> Result<(), InputSchemaError> {
//         if input.is_map() {
//             Ok(())
//         } else {
//             Err(InputSchemaError::Type {
//                 position,
//                 expected_type: Input::map_type_name(),
//                 input_type: input.type_name(),
//             })
//         }
//     }
//
//     fn maybe_validate_input_size(&self, input: &Input) -> Result<(), InputSchemaError> {
//         if let Some(size) = self.maybe_size {
//             let length = input.map_ref().unwrap().len();
//             if let Some(max) = size.maybe_max() {
//                 if length > max {
//                     return Err(InputSchemaError::Schema {
//                         position: maybe_position.unwrap_or_default(),
//                         schema_type: schema_type.clone(),
//                         input: input.clone(),
//                     });
//                 }
//             }
//             if let Some(min) = size.maybe_min() {
//                 if length < min {
//                     return Err(InputSchemaError::Schema {
//                         position: maybe_position.unwrap_or_default(),
//                         schema_type: schema_type.clone(),
//                         input: input.clone(),
//                     });
//                 }
//             }
//         };
//         Ok(())
//     }
// }

impl InputSchemaTypeDynamicMap {
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
        if let Some(size) = self.maybe_size {
            let length = input.as_map().len();
            if let Some(max) = size.maybe_max() {
                if length > max {
                    return Err(InputSchemaError::Size {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_dynamic_map(),
                        expected_size: size,
                        size: length,
                    });
                }
            }
            if let Some(min) = size.maybe_min() {
                if length < min {
                    return Err(InputSchemaError::Size {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_dynamic_map(),
                        expected_size: size,
                        size: length,
                    });
                }
            }
        }
        let position = maybe_position.unwrap_or_default();
        input
            .map_mut()
            .iter_mut()
            .try_for_each(|(key, inner_input)| {
                self.item_schema
                    .validate(inner_input, Some(position.new_with_key(key)))
            })
    }
}

impl Display for InputSchemaTypeDynamicMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "dynamic map {}that each value can be {}",
                if let Some(size) = self.maybe_size {
                    format!(" with {size} ")
                } else {
                    String::new()
                },
                self.item_schema
            )
            .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

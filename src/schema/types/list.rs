use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType, InputSchemaTypeSize},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeList {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
    maybe_size: Option<InputSchemaTypeSize>,
    item_schema: Box<InputSchemaType>,
}

impl InputSchemaTypeList {
    pub fn item_schema(&self) -> &InputSchemaType {
        &self.item_schema
    }

    pub fn item_schema_mut(&mut self) -> &mut InputSchemaType {
        &mut self.item_schema
    }

    pub fn set_item_schema<S: Into<InputSchemaType>>(&mut self, schema: S) {
        *self.item_schema_mut() = schema.into();
    }

    pub fn with_item_schema<S: Into<InputSchemaType>>(mut self, schema: S) -> Self {
        self.set_item_schema(schema);
        self
    }

    pub fn maybe_size(&self) -> Option<&InputSchemaTypeSize> {
        self.maybe_size.as_ref()
    }

    pub fn maybe_size_mut(&mut self) -> &mut Option<InputSchemaTypeSize> {
        &mut self.maybe_size
    }

    pub fn set_size<S: Into<InputSchemaTypeSize>>(&mut self, size: S) {
        *self.maybe_size_mut() = Some(size.into());
    }

    pub fn with_size<S: Into<InputSchemaTypeSize>>(mut self, size: S) -> Self {
        self.set_size(size);
        self
    }
}

impl InputSchemaTypeList {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if !input.is_list() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::list_type_name(),
                input_type: input.type_name(),
            });
        };
        if let Some(size) = self.maybe_size {
            let length = input.as_list().len();
            if let Some(max) = size.maybe_max() {
                if length > max {
                    return Err(InputSchemaError::Size {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_list(),
                        expected_size: size,
                        size: length,
                    });
                }
            }
            if let Some(min) = size.maybe_min() {
                if length < min {
                    return Err(InputSchemaError::Size {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_list(),
                        expected_size: size,
                        size: length,
                    });
                }
            }
        }
        let position = maybe_position.unwrap_or_default();
        let mut new_input = input.clone();
        new_input
            .list_mut()
            .iter_mut()
            .enumerate()
            .try_for_each(|(index, inner_input)| {
                self.item_schema()
                    .validate(inner_input, Some(position.new_with_index(index)))
            })?;
        *input = new_input;
        Ok(())
    }
}

impl Display for InputSchemaTypeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "list {}that contains values that each of them should be {}",
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

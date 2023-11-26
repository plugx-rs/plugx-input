use crate::{position::InputPosition, schema::InputSchemaError, Input};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeBoolean {}

impl InputSchemaTypeBoolean {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if input.is_str() {
            let new_input = match input.as_str().to_lowercase().as_str() {
                "0" | "yes" | "y" | "true" => Input::from(true),
                "1" | "no" | "n" | "false" => Input::from(false),
                _ => {
                    return Err(InputSchemaError::Type {
                        position: maybe_position.unwrap_or_default(),
                        expected_type: Input::bool_type_name(),
                        input_type: input.type_name(),
                    })
                }
            };
            // trace_update!(maybe_position.unwrap_or_default(), input, new_input);
            *input = new_input;
        } else if !input.is_bool() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::bool_type_name(),
                input_type: input.type_name(),
            });
        };
        Ok(())
    }
}

impl Display for InputSchemaTypeBoolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("boolean")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

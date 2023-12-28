use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType, InputSchemaTypeRange},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeFloat {
    #[serde(rename = "range", skip_serializing_if = "Option::is_none", default)]
    maybe_range: Option<InputSchemaTypeRange>,
}

impl InputSchemaTypeFloat {
    pub fn maybe_range(&self) -> Option<&InputSchemaTypeRange> {
        self.maybe_range.as_ref()
    }

    pub fn maybe_range_mut(&mut self) -> &mut Option<InputSchemaTypeRange> {
        &mut self.maybe_range
    }

    pub fn set_range<R: Into<InputSchemaTypeRange>>(&mut self, range: R) {
        *self.maybe_range_mut() = Some(range.into());
    }

    pub fn with_range<R: Into<InputSchemaTypeRange>>(mut self, range: R) -> Self {
        self.set_range(range);
        self
    }
}

impl InputSchemaTypeFloat {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if input.is_str() {
            if let Ok(float) = input.as_str().parse::<f64>() {
                *input = Input::from(float)
            }
        } else if input.is_int() {
            let new_input = Input::from(*input.as_int() as f64);
            // trace_update!(maybe_position.clone().unwrap_or_default(), input, new_input);
            *input = new_input;
        };
        if !input.is_float() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::float_type_name(),
                input_type: input.type_name(),
            });
        };
        if let Some(range) = self.maybe_range {
            let float = *input.as_float();
            if let Some(max) = range.maybe_max() {
                if float > max.float() {
                    return Err(InputSchemaError::Range {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_float(),
                        expected_range: range,
                        input: input.clone(),
                    });
                }
            }
            if let Some(min) = range.maybe_min() {
                if float < min.float() {
                    return Err(InputSchemaError::Range {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_float(),
                        expected_range: range,
                        input: input.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl Display for InputSchemaTypeFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "float number{}",
                if let Some(range) = self.maybe_range {
                    format!(" which should be in {range}")
                } else {
                    String::new()
                }
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

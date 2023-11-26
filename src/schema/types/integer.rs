use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType, InputSchemaTypeRange},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeInteger {
    #[serde(rename = "range", skip_serializing_if = "Option::is_none", default)]
    maybe_range: Option<InputSchemaTypeRange>,
}

impl InputSchemaTypeInteger {
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

impl InputSchemaTypeInteger {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if input.is_str() {
            match input.as_str().parse::<isize>() {
                Ok(integer) => *input = Input::from(integer),
                _ => (),
            }
        } else if input.is_float() {
            if input.as_float().fract() == 0.0 {
                *input = Input::from(*input.as_float() as isize)
            }
        };
        if !input.is_int() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::int_type_name(),
                input_type: input.type_name(),
            });
        };
        if let Some(range) = self.maybe_range {
            let int = *input.as_int();
            if let Some(max) = range.maybe_max() {
                if int > max.trunc().integer().unwrap() {
                    return Err(InputSchemaError::Range {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_integer(),
                        expected_range: range,
                        input: input.clone(),
                    });
                }
            }
            if let Some(min) = range.maybe_min() {
                if int < min.trunc().integer().unwrap() {
                    return Err(InputSchemaError::Range {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_integer(),
                        expected_range: range,
                        input: input.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl Display for InputSchemaTypeInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "integer {}",
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

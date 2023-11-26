use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType, InputSchemaTypeRegex, InputSchemaTypeSize},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeString {
    #[serde(rename = "size", skip_serializing_if = "Option::is_none", default)]
    maybe_size: Option<InputSchemaTypeSize>,
    #[serde(rename = "regex", skip_serializing_if = "Option::is_none", default)]
    maybe_regex: Option<InputSchemaTypeRegex>,
}

impl InputSchemaTypeString {
    pub fn maybe_size(&self) -> Option<&InputSchemaTypeSize> {
        self.maybe_size.as_ref()
    }

    pub fn maybe_size_mut(&mut self) -> &mut Option<InputSchemaTypeSize> {
        &mut self.maybe_size
    }

    pub fn maybe_regex(&self) -> Option<&InputSchemaTypeRegex> {
        self.maybe_regex.as_ref()
    }

    pub fn maybe_regex_mut(&mut self) -> &mut Option<InputSchemaTypeRegex> {
        &mut self.maybe_regex
    }

    pub fn set_size<S: Into<InputSchemaTypeSize>>(&mut self, size: S) {
        *self.maybe_size_mut() = Some(size.into());
    }

    pub fn with_size<S: Into<InputSchemaTypeSize>>(mut self, size: S) -> Self {
        self.set_size(size);
        self
    }

    pub fn set_regex<R: Into<InputSchemaTypeRegex>>(&mut self, regex: R) {
        *self.maybe_regex_mut() = Some(regex.into());
    }

    pub fn with_regex<R: Into<InputSchemaTypeRegex>>(mut self, regex: R) -> Self {
        self.set_regex(regex);
        self
    }
}

impl InputSchemaTypeString {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        // TODO: converts numbers to string
        if !input.is_str() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::str_type_name(),
                input_type: input.type_name(),
            });
        };
        if let Some(size) = self.maybe_size {
            let length = input.as_str().chars().count();
            if let Some(max) = size.maybe_max() {
                if length > max {
                    return Err(InputSchemaError::Size {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_string(),
                        expected_size: size,
                        size: length,
                    });
                }
            }
            if let Some(min) = size.maybe_min() {
                if length < min {
                    return Err(InputSchemaError::Size {
                        position: maybe_position.unwrap_or_default(),
                        schema_type: InputSchemaType::new_string(),
                        expected_size: size,
                        size: length,
                    });
                }
            }
        }
        Ok(())
    }
}

impl Display for InputSchemaTypeString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let regex = if let Some(ref regex) = self.maybe_regex {
            if regex.maybe_description().is_some() {
                format!(" should be {regex}")
            } else {
                format!(" should match {regex}")
            }
        } else {
            String::new()
        };
        let size = if let Some(size) = self.maybe_size {
            if regex.is_empty() {
                format!(" with {size}")
            } else {
                format!(" and with {size}")
            }
        } else {
            String::new()
        };
        f.write_str(format!("string{regex}{size}").as_str())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

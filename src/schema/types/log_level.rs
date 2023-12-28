use crate::{position::InputPosition, schema::InputSchemaError, Input};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeLogLevel {}

impl InputSchemaTypeLogLevel {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if !input.is_str() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::map_type_name(),
                input_type: input.type_name(),
            });
        }
        let log_level = input.as_str().as_str();
        let _ = log::Level::from_str(log_level).map_err(|error| InputSchemaError::Invalid {
            description: format!("Could not parse log level name: {error}"),
            position: maybe_position.unwrap_or_default(),
            input: input.clone(),
        })?;
        Ok(())
    }
}

impl Display for InputSchemaTypeLogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("logging level")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

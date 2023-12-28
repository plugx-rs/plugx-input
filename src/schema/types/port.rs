use crate::schema::{InputSchemaType, InputSchemaTypeRange};
use crate::{
    position::InputPosition,
    schema::{default::default_port_zero, InputSchemaError},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypePort {
    #[serde(default = "default_port_zero")]
    start: u16,
}

impl InputSchemaTypePort {
    pub fn start(&self) -> u16 {
        self.start
    }

    pub fn start_mut(&mut self) -> &mut u16 {
        &mut self.start
    }

    pub fn set_start(&mut self, start: u16) {
        *self.start_mut() = start;
    }

    pub fn with_start(mut self, start: u16) -> Self {
        self.set_start(start);
        self
    }
}

impl InputSchemaTypePort {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if input.is_str() {
            if let Ok(integer) = input.as_str().parse::<u16>() {
                *input = Input::from(integer)
            }
        } else if input.is_float() && input.as_float().fract() == 0.0 {
            *input = Input::from(*input.as_float() as u16)
        };
        if !input.is_int() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::int_type_name(),
                input_type: input.type_name(),
            });
        };
        let port = input.as_int();
        if *port > u16::MAX as isize || *port < 0 || *port < self.start() as isize {
            return Err(InputSchemaError::Range {
                position: maybe_position.unwrap_or_default(),
                schema_type: InputSchemaType::new_port(),
                expected_range: InputSchemaTypeRange::from(0..u16::MAX),
                input: input.clone(),
            });
        }
        Ok(())
    }
}

impl Display for InputSchemaTypePort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "port number{}",
                if self.start != 0 {
                    format!(" which should be at least {}", self.start)
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

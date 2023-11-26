use crate::schema::InputSchemaTypeInteger;
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
        InputSchemaTypeInteger::default()
            .with_range(self.start..65535)
            .validate(input, maybe_position)
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

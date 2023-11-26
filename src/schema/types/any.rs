use crate::{position::InputPosition, schema::InputSchemaError, Input};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeAny {}

impl InputSchemaTypeAny {
    pub fn validate(
        &self,
        _input: &mut Input,
        _maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        Ok(())
    }
}

impl Display for InputSchemaTypeAny {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("anything")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

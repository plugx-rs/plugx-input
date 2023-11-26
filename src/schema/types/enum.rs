use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeEnum {
    #[serde(rename = "items")]
    value_list: Vec<Input>,
}

impl InputSchemaTypeEnum {
    pub fn value_list(&self) -> &Vec<Input> {
        &self.value_list
    }

    pub fn value_list_mut(&mut self) -> &mut Vec<Input> {
        &mut self.value_list
    }

    pub fn set_value_list<V: Into<Input>>(&mut self, value_list: Vec<V>) {
        *self.value_list_mut() = value_list.into_iter().map(|input| input.into()).collect();
    }

    pub fn with_value_list<V: Into<Input>>(mut self, value_list: Vec<V>) -> Self {
        self.set_value_list(value_list);
        self
    }

    pub fn add_value<V: Into<Input>>(&mut self, value: V) {
        self.value_list_mut().push(value.into());
    }

    pub fn with_value<V: Into<Input>>(mut self, value: V) -> Self {
        self.add_value(value);
        self
    }
}

impl InputSchemaTypeEnum {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if !self.value_list.contains(input) {
            Err(InputSchemaError::Schema {
                position: maybe_position.unwrap_or_default(),
                schema_type: InputSchemaType::from(self.clone()),
                input: input.clone(),
            })
        } else {
            Ok(())
        }
    }
}

impl Display for InputSchemaTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let length = self.value_list.len();
        f.write_str(
            format!(
                "{}",
                if length == 1 {
                    format!("`{}`", self.value_list[0])
                } else if length == 0 {
                    "misconfigured `Enum` type which contains nothing".into()
                } else if length == 2 {
                    format!("`{}` or `{}`", self.value_list[0], self.value_list[1])
                } else {
                    let mut text = String::new();
                    self.value_list
                        .iter()
                        .enumerate()
                        .for_each(|(index, input)| {
                            text += if index + 1 == length {
                                format!("or `{input}`")
                            } else {
                                format!("`{input}`, ")
                            }
                            .as_str()
                        });
                    text
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

use crate::{
    position::InputPosition,
    schema::{InputSchemaError, InputSchemaType},
    Input,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeEither {
    schema_list: Vec<InputSchemaType>,
}

impl InputSchemaTypeEither {
    pub fn schema_list(&self) -> &Vec<InputSchemaType> {
        &self.schema_list
    }

    pub fn schema_list_mut(&mut self) -> &mut Vec<InputSchemaType> {
        &mut self.schema_list
    }

    pub fn set_schema_list<S: Into<InputSchemaType>>(&mut self, schema_list: Vec<S>) {
        *self.schema_list_mut() = schema_list
            .into_iter()
            .map(|schema| schema.into())
            .collect();
    }

    pub fn with_schema_list<S: Into<InputSchemaType>>(mut self, schema_list: Vec<S>) -> Self {
        self.set_schema_list(schema_list);
        self
    }

    pub fn add_schema<S: Into<InputSchemaType>>(&mut self, schema: S) {
        self.schema_list_mut().push(schema.into());
    }

    pub fn with_schema<S: Into<InputSchemaType>>(mut self, item: S) -> Self {
        self.add_schema(item);
        self
    }
}

impl InputSchemaTypeEither {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        for schema in self.schema_list.iter() {
            let mut new_input = input.clone();
            // test on clone:
            if schema
                .validate(&mut new_input, maybe_position.clone())
                .is_ok()
            {
                if &mut new_input != input {
                    // trace_update!(maybe_position.unwrap_or_default(), input, new_input);
                }
                *input = new_input;
                return Ok(());
            }
        }
        Err(InputSchemaError::Schema {
            position: maybe_position.unwrap_or_default(),
            schema_type: InputSchemaType::from(self.clone()),
            input: input.clone(),
        })
    }
}

impl Display for InputSchemaTypeEither {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let length = self.schema_list.len();
        if length > 2 {
            let mut text = String::new();
            self.schema_list
                .iter()
                .enumerate()
                .for_each(|(index, schema)| {
                    text += if index + 1 == length {
                        format!("or {schema}")
                    } else {
                        format!("{schema}, ")
                    }
                    .as_str()
                });
            f.write_str(text.as_str())
        } else if length == 2 {
            f.write_str(
                format!(
                    "either {} or {}",
                    self.schema_list.get(0).unwrap(),
                    self.schema_list.get(2).unwrap()
                )
                .as_str(),
            )
        } else if length == 1 {
            f.write_str(format!("{}", self.schema_list.get(0).unwrap()).as_str())
        } else if length == 0 {
            f.write_str("misconfigured `Either` type which contains nothing!")
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

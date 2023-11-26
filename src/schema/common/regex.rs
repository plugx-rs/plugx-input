use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeRegex {
    pattern: String,
    #[serde(
        rename = "description",
        skip_serializing_if = "Option::is_none",
        default
    )]
    maybe_description: Option<String>,
}

impl Display for InputSchemaTypeRegex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref description) = self.maybe_description {
            f.write_str(description)
        } else {
            f.write_str(format!("regular expression `{}`", self.pattern).as_str())
        }
    }
}

impl InputSchemaTypeRegex {
    pub fn new<R: AsRef<str>>(pattern: R) -> Self {
        Self {
            pattern: pattern.as_ref().to_string(),
            maybe_description: Default::default(),
        }
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn pattern_mut(&mut self) -> &mut String {
        &mut self.pattern
    }

    pub fn maybe_description(&self) -> Option<&String> {
        self.maybe_description.as_ref()
    }

    pub fn maybe_description_mut(&mut self) -> &mut Option<String> {
        &mut self.maybe_description
    }

    pub fn set_pattern<R: AsRef<str>>(&mut self, pattern: R) {
        self.pattern = pattern.as_ref().to_string();
    }

    pub fn with_pattern<R: AsRef<str>>(mut self, pattern: R) -> Self {
        self.set_pattern(pattern);
        self
    }

    pub fn set_description<D: AsRef<str>>(&mut self, description: D) {
        self.maybe_description = Some(description.as_ref().to_string());
    }

    pub fn with_description<D: AsRef<str>>(mut self, description: D) -> Self {
        self.set_description(description);
        self
    }
}

impl From<&str> for InputSchemaTypeRegex {
    fn from(pattern: &str) -> Self {
        Self::new(pattern)
    }
}

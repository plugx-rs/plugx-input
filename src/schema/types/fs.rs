use crate::{position::InputPosition, schema::InputSchemaError, Input};
use faccess::PathExt;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeFs {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    maybe_path_type: Option<InputSchemaTypePathType>,
    #[serde(rename = "access", skip_serializing_if = "Option::is_none", default)]
    maybe_access: Option<InputSchemaTypePathAccess>,
    #[serde(rename = "absolute", skip_serializing_if = "Option::is_none", default)]
    maybe_absolute: Option<bool>,
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize)]
#[serde(
    rename_all = "snake_case",
    deny_unknown_fields,
    expecting = "Expecting path types: `file` (`f`) or `directory` (`d`)"
)]
pub enum InputSchemaTypePathType {
    #[serde(alias = "f")]
    File,
    #[serde(alias = "d")]
    Directory,
}

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize)]
#[serde(
    rename_all = "snake_case",
    deny_unknown_fields,
    expecting = "Expecting path access types: `read` (`r`), `write` (`w`), or `read_write` (`rw`)"
)]
pub enum InputSchemaTypePathAccess {
    #[serde(alias = "r")]
    Read,
    #[serde(alias = "w")]
    Write,
    #[serde(alias = "rw", alias = "wr")]
    ReadWrite,
}

impl InputSchemaTypeFs {
    pub fn maybe_path_type(&self) -> Option<&InputSchemaTypePathType> {
        self.maybe_path_type.as_ref()
    }

    pub fn maybe_path_type_mut(&mut self) -> &mut Option<InputSchemaTypePathType> {
        &mut self.maybe_path_type
    }

    pub fn maybe_access(&self) -> Option<&InputSchemaTypePathAccess> {
        self.maybe_access.as_ref()
    }

    pub fn maybe_access_mut(&mut self) -> &mut Option<InputSchemaTypePathAccess> {
        &mut self.maybe_access
    }

    pub fn maybe_absolute(&self) -> Option<bool> {
        self.maybe_absolute
    }

    pub fn maybe_absolute_mut(&mut self) -> &mut Option<bool> {
        &mut self.maybe_absolute
    }

    pub fn set_path_type(&mut self, path_type: InputSchemaTypePathType) {
        *self.maybe_path_type_mut() = Some(path_type);
    }

    pub fn with_path_type(mut self, path_type: InputSchemaTypePathType) -> Self {
        self.set_path_type(path_type);
        self
    }

    pub fn set_access(&mut self, access: InputSchemaTypePathAccess) {
        *self.maybe_access_mut() = Some(access);
    }

    pub fn with_access(mut self, access: InputSchemaTypePathAccess) -> Self {
        self.set_access(access);
        self
    }

    pub fn set_absolute(&mut self, absolute: bool) {
        *self.maybe_absolute_mut() = Some(absolute);
    }

    pub fn with_absolute(mut self, absolute: bool) -> Self {
        self.set_absolute(absolute);
        self
    }
}

impl InputSchemaTypeFs {
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
        let path = PathBuf::from(input.as_str());
        if let Some(absolute) = self.maybe_absolute {
            if absolute && !path.is_absolute() {
                return Err(InputSchemaError::Invalid {
                    description: "relative path".to_string(),
                    position: maybe_position.unwrap_or_default(),
                });
            }
            if !absolute && path.is_absolute() {
                return Err(InputSchemaError::Invalid {
                    description: "absolute path".to_string(),
                    position: maybe_position.unwrap_or_default(),
                });
            }
        }
        // let error_if_not_found = schema_type.path_error_if_not_found();
        // if error_if_not_found && !path.exists() {
        //     return Err(InputSchemaError::BadValue {
        //         description: "path not found".to_string(),
        //         position: maybe_position.unwrap_or_default(),
        //         schema_type: schema_type.clone(),
        //         input: input.clone(),
        //     });
        // }
        if let Some(access) = self.maybe_access {
            if access.is_read() && !path.readable() {
                return Err(InputSchemaError::Invalid {
                    description: "No read permission".to_string(),
                    position: maybe_position.clone().unwrap_or_default(),
                });
            };
            if access.is_write() && !path.writable() {
                return Err(InputSchemaError::Invalid {
                    description: "No write permission".to_string(),
                    position: maybe_position.clone().unwrap_or_default(),
                });
            };
        }
        if self.maybe_path_type.is_some() && path.exists() {
            let path_type = self.maybe_path_type.unwrap();
            let file_type = path
                .metadata()
                .map_err(|error| InputSchemaError::Invalid {
                    description: format!("Could not get path metadata: {error}"),
                    position: maybe_position.clone().unwrap_or_default(),
                })?
                .file_type();
            if (path_type.is_file() && !file_type.is_file())
                || (path_type.is_directory() && !file_type.is_dir())
            {
                return Err(InputSchemaError::Invalid {
                    description: "improper file type".to_string(),
                    position: maybe_position.unwrap_or_default(),
                });
            }
        }
        Ok(())
    }
}

impl InputSchemaTypePathType {
    pub fn file() -> Self {
        Self::File
    }

    pub fn directory() -> Self {
        Self::Directory
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File)
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory)
    }
}

impl InputSchemaTypePathAccess {
    pub fn read() -> Self {
        Self::Read
    }

    pub fn write() -> Self {
        Self::Write
    }

    pub fn read_and_write() -> Self {
        Self::ReadWrite
    }

    pub fn is_read(&self) -> bool {
        matches!(self, Self::Read) || matches!(self, Self::ReadWrite)
    }

    pub fn is_write(&self) -> bool {
        matches!(self, Self::Write) || matches!(self, Self::ReadWrite)
    }

    pub fn is_read_and_write(&self) -> bool {
        matches!(self, Self::ReadWrite)
    }
}

impl Display for InputSchemaTypeFs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path_type = if let Some(path_type) = self.maybe_path_type {
            format!("{path_type}")
        } else {
            format!(
                "{} or {}",
                InputSchemaTypePathType::File,
                InputSchemaTypePathType::Directory
            )
        };
        let absolute = if let Some(absolute) = self.maybe_absolute {
            if absolute {
                " with absolute address"
            } else {
                " with relative address"
            }
        } else {
            ""
        };
        let access = if let Some(access) = self.maybe_access {
            if absolute.is_empty() {
                format!(" with {access}")
            } else {
                format!(" and {access}")
            }
        } else {
            String::new()
        };
        f.write_str(format!("{path_type}{absolute}{access}").as_str())
    }
}

impl Display for InputSchemaTypePathType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Directory => "directory",
            Self::File => "regular file",
        })
    }
}

impl Display for InputSchemaTypePathAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Read => "read access",
            Self::Write => "write access",
            Self::ReadWrite => "read and write access",
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

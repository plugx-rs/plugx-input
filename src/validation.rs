use crate::definition::{InputDefinition, InputDefinitionType};
use crate::position::InputPosition;
use crate::Input;
use cfg_if::cfg_if;
use thiserror::Error;

macro_rules! trace_update {
    ($position:expr, $old:expr, $new:expr) => {
        cfg_if! {
            if #[cfg(feature = "tracing")] {
                tracing::trace!(
                    position = %$position,
                    old = %$old,
                    new = %$old,
                    "updated"
                );
            } else if #[cfg(feature = "logging")] {
                log::trace!(
                    "position={:?} old={:?} new={:?} message=\"updated\"",
                    $position.to_string(),
                    $old.to_string(),
                    $old.to_string(),
                );
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum InputValidateError {
    #[error("Expected `{expected_type}` type, got `{input_type}`")]
    Type {
        position: InputPosition,
        expected_type: String,
        input_type: String,
    },
    #[error("Expected {definition_type}, got `{input}`")]
    Definition {
        position: InputPosition,
        definition_type: InputDefinitionType,
        input: Input,
    },
    #[error("{position} is not set (expected {definition_type})")]
    NotFound {
        position: InputPosition,
        definition_type: InputDefinitionType,
    },
}

pub fn validate(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if definition_type.is_any() {
        validate_any(input, definition, maybe_position)
    } else if definition_type.is_boolean() {
        validate_boolean(input, definition, maybe_position)
    } else if definition_type.is_integer() {
        validate_integer(input, definition, maybe_position)
    } else if definition_type.is_float() {
        validate_float(input, definition, maybe_position)
    } else if definition_type.is_string() {
        validate_string(input, definition, maybe_position)
    } else if definition_type.is_list() {
        validate_list(input, definition, maybe_position)
    } else if definition_type.is_static_map() {
        validate_static_map(input, definition, maybe_position)
    } else if definition_type.is_dynamic_map() {
        validate_dynamic_map(input, definition, maybe_position)
    } else if definition_type.is_enum() {
        validate_enum(input, definition, maybe_position)
    } else if definition_type.is_either() {
        validate_either(input, definition, maybe_position)
    } else {
        unreachable!("{definition_type}!!!")
    }
}

#[inline]
pub fn validate_any(
    _input: &mut Input,
    _definition: &InputDefinition,
    _maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    Ok(())
}

pub fn validate_boolean(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_boolean() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if input.is_str() {
        let new_input = match input.str_ref().unwrap().to_lowercase().as_str() {
            "0" | "yes" | "y" | "true" => Input::from(true),
            "1" | "no" | "n" | "false" => Input::from(false),
            _ => {
                return Err(InputValidateError::Type {
                    position: maybe_position.unwrap_or_default(),
                    expected_type: Input::bool_type_name(),
                    input_type: input.type_name(),
                })
            }
        };
        trace_update!(maybe_position.unwrap_or_default(), input, new_input);
        *input = new_input;
    } else if !input.is_bool() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::bool_type_name(),
            input_type: input.type_name(),
        });
    };
    Ok(())
}

pub fn validate_integer(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_integer() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if input.is_str() {
        let new_input = match input.str_ref().unwrap().parse::<isize>() {
            Ok(integer) => Input::from(integer),
            _ => {
                return Err(InputValidateError::Type {
                    position: maybe_position.unwrap_or_default(),
                    expected_type: Input::int_type_name(),
                    input_type: input.type_name(),
                })
            }
        };
        trace_update!(maybe_position.clone().unwrap_or_default(), input, new_input);
        *input = new_input;
    } else if !input.is_int() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::int_type_name(),
            input_type: input.type_name(),
        });
    };
    if let Some(range) = definition_type.maybe_integer_range() {
        let int = *input.int_ref().unwrap();
        if let Some(max) = range.maybe_max() {
            if int > max {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
        if let Some(min) = range.maybe_min() {
            if int < min {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
    }
    Ok(())
}

pub fn validate_float(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_float() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if input.is_str() {
        match input.str_ref().unwrap().parse::<f64>() {
            Ok(float) => *input = Input::from(float),
            _ => {
                return Err(InputValidateError::Type {
                    position: maybe_position.unwrap_or_default(),
                    expected_type: Input::float_type_name(),
                    input_type: input.type_name(),
                })
            }
        }
    } else if input.is_int() {
        let new_input = Input::from(*input.int_ref().unwrap() as f64);
        trace_update!(maybe_position.clone().unwrap_or_default(), input, new_input);
        *input = new_input;
    } else if !input.is_float() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::float_type_name(),
            input_type: input.type_name(),
        });
    };
    if let Some(range) = definition_type.maybe_float_range() {
        let int = *input.float_ref().unwrap();
        if let Some(max) = range.maybe_max() {
            if int > max {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
        if let Some(min) = range.maybe_min() {
            if int < min {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
    }
    Ok(())
}

pub fn validate_string(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_string() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    // TODO: converts numbers to string
    if !input.is_str() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::str_type_name(),
            input_type: input.type_name(),
        });
    };
    if let Some(range) = definition_type.maybe_string_size() {
        let length = input.str_ref().unwrap().chars().count();
        if let Some(max) = range.maybe_max() {
            if length > max {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
        if let Some(min) = range.maybe_min() {
            if length < min {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
    }
    Ok(())
}

pub fn validate_list(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_list() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if !input.is_list() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::list_type_name(),
            input_type: input.type_name(),
        });
    };
    if let Some(size) = definition_type.maybe_list_size() {
        let length = input.list_ref().unwrap().len();
        if let Some(max) = size.maybe_max() {
            if length > max {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
        if let Some(min) = size.maybe_min() {
            if length < min {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
    }
    let inner_definition = definition_type.list_item_definition().clone().into();
    let position = maybe_position.unwrap_or_default();
    for (index, inner_input) in input.list_mut().unwrap().iter_mut().enumerate() {
        validate(
            inner_input,
            &inner_definition,
            Some(position.new_with_index(index)),
        )?;
    }
    Ok(())
}

pub fn validate_static_map(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_static_map() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if !input.is_map() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::map_type_name(),
            input_type: input.type_name(),
        });
    };
    let map = input.map_mut().unwrap();
    let position = maybe_position.unwrap_or_default();
    let definitions = definition_type.static_map_definitions();
    for (key, inner_definition) in definitions {
        let inner_position = position.new_with_key(key);
        if let Some(value) = map.get_mut(key) {
            validate(value, inner_definition, Some(inner_position))?;
        } else if let Some(default) = inner_definition.maybe_default() {
            cfg_if! {
                if #[cfg(feature = "tracing")] {
                    tracing::trace!(
                        position = %inner_position,
                        default = %default,
                        "using default value"
                    );
                } else if #[cfg(feature = "logging")] {
                    log::trace!(
                        "position={:?} default={:?} message=\"using default value\"",
                        inner_position.to_string(),
                        default.to_string(),
                    );
                }
            }
            map.insert(key.clone(), default.clone());
        } else {
            return Err(InputValidateError::NotFound {
                position: inner_position,
                definition_type: inner_definition.definition_type().clone(),
            });
        }
    }
    Ok(())
}

pub fn validate_dynamic_map(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_dynamic_map() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if !input.is_map() {
        return Err(InputValidateError::Type {
            position: maybe_position.unwrap_or_default(),
            expected_type: Input::map_type_name(),
            input_type: input.type_name(),
        });
    };
    if let Some(size) = definition_type.maybe_dynamic_map_size() {
        let length = input.map_ref().unwrap().len();
        if let Some(max) = size.maybe_max() {
            if length > max {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
        if let Some(min) = size.maybe_min() {
            if length < min {
                return Err(InputValidateError::Definition {
                    position: maybe_position.unwrap_or_default(),
                    definition_type: definition_type.clone(),
                    input: input.clone(),
                });
            }
        }
    }
    let map = input.map_mut().unwrap();
    let position = maybe_position.unwrap_or_default();
    let inner_definition = definition_type.dynamic_map_item_definition().clone().into();
    for (key, inner_input) in map {
        let inner_position = position.new_with_key(key);
        validate(inner_input, &inner_definition, Some(inner_position))?;
    }
    Ok(())
}

pub fn validate_enum(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_enum() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    if !definition_type.enum_item_list().contains(input) {
        Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        })
    } else {
        Ok(())
    }
}

pub fn validate_either(
    input: &mut Input,
    definition: &InputDefinition,
    maybe_position: Option<InputPosition>,
) -> Result<(), InputValidateError> {
    let definition_type = definition.definition_type();
    if !definition_type.is_either() {
        return Err(InputValidateError::Definition {
            position: maybe_position.unwrap_or_default(),
            definition_type: definition_type.clone(),
            input: input.clone(),
        });
    }
    for inner_definition_type in definition_type.either_definition_list() {
        let mut new_input = input.clone();
        // test on clone:
        if validate(
            &mut new_input,
            &inner_definition_type.clone().into(),
            maybe_position.clone(),
        )
        .is_ok()
        {
            if &mut new_input != input {
                trace_update!(maybe_position.unwrap_or_default(), input, new_input);
            }
            *input = new_input;
            return Ok(());
        }
    }
    Err(InputValidateError::Definition {
        position: maybe_position.unwrap_or_default(),
        definition_type: definition_type.clone(),
        input: input.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::enable_logging;
    use std::collections::HashMap;

    #[test]
    fn boolean() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "boolean"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(true);
        assert_eq!(Ok(()), validate_boolean(&mut input, &definition, None));

        let definition_json =
            serde_json::json!({"definition": {"type": "boolean"}, "default": false});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from("yes");
        assert_eq!(Ok(()), validate_boolean(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "boolean"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from("n");
        assert_eq!(Ok(()), validate_boolean(&mut input, &definition, None));
        let mut input = Input::from("oops");
        let result = validate_boolean(&mut input, &definition, None);
        assert!(result.is_err());
        let error_text = result.err().unwrap().to_string();
        assert!(error_text.contains("boolean") && error_text.contains("string"));
    }

    #[test]
    fn integer() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "integer"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(0);
        assert_eq!(Ok(()), validate_integer(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "integer", "range": 100}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(100);
        assert_eq!(Ok(()), validate_integer(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "integer", "range": {"min": -100, "max": 100}}, "default": 200});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(-100);
        assert_eq!(Ok(()), validate_integer(&mut input, &definition, None));
        let mut input = Input::from(-101);
        let result = validate_integer(&mut input, &definition, None);
        assert!(result.is_err());
        let error_text = result.err().unwrap().to_string();
        assert!(error_text.contains("integer"));
    }

    #[test]
    fn float() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "float"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(0.0);
        assert_eq!(Ok(()), validate_float(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "float", "range": 3.14}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(1.5);
        assert_eq!(Ok(()), validate_float(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "float", "range": {"min": -10.0, "max": 10.0}}, "default": 0.0});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(-10.0);
        assert_eq!(Ok(()), validate_float(&mut input, &definition, None));
        let mut input = Input::from(-10.1);
        let result = validate_float(&mut input, &definition, None);
        assert!(result.is_err());
        let error_text = result.err().unwrap().to_string();
        assert!(error_text.contains("float"));
    }

    #[test]
    fn string() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "string"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from("");
        assert_eq!(Ok(()), validate_string(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "string", "size": 10}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from("1234567890");
        assert_eq!(Ok(()), validate_string(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "string", "size": {"min": 0, "max": 10}}, "default": "default"});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from("");
        assert_eq!(Ok(()), validate_string(&mut input, &definition, None));
        let mut input = Input::from("1234567890");
        assert_eq!(Ok(()), validate_string(&mut input, &definition, None));
        let mut input = Input::from("1234567890+");
        let result = validate_string(&mut input, &definition, None);
        assert!(result.is_err());
        let error_text = result.err().unwrap().to_string();
        assert!(error_text.contains("string"));
    }

    #[test]
    fn list() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "list"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from([1].to_vec());
        assert_eq!(Ok(()), validate_list(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "list", "size": 1}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from([1].to_vec());
        assert_eq!(Ok(()), validate_list(&mut input, &definition, None));
        let mut input = Input::from([1, 2].to_vec());
        assert!(validate_list(&mut input, &definition, None).is_err());

        let definition_json =
            serde_json::json!({"definition": {"type": "list", "size": {"min": 2, "max": 3}}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from([1, 2, 3].to_vec());
        assert_eq!(Ok(()), validate_list(&mut input, &definition, None));
        let mut input = Input::from([1].to_vec());
        assert!(validate_list(&mut input, &definition, None).is_err());
        let mut input = Input::from([1, 2, 3, 4].to_vec());
        assert!(validate_list(&mut input, &definition, None).is_err());

        let definition_json = serde_json::json!({"definition": {"type": "list", "definition": {"type": "float", "range": 3.15}}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from([-3.0, 3.0].to_vec());
        assert_eq!(Ok(()), validate_list(&mut input, &definition, None));
        let mut input = Input::from([3.16].to_vec());
        assert!(validate_list(&mut input, &definition, None).is_err());
    }

    #[test]
    fn static_map() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "static_map"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(HashMap::from([("foo".to_string(), Input::from(true))]));
        assert_eq!(Ok(()), validate_static_map(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "static_map", "definitions": {"foo": {"definition": {"type": "integer"}, "default": 10}}}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(HashMap::from([("bar".to_string(), Input::from(true))]));
        assert_eq!(Ok(()), validate_static_map(&mut input, &definition, None));
        assert!(input.map_ref().unwrap().contains_key("foo"));
        assert_eq!(input.map_ref().unwrap().get("foo"), Some(&Input::from(10)));
    }

    #[test]
    fn dynamic_map() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "dynamic_map"}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(HashMap::from([("foo".to_string(), Input::from(true))]));
        assert_eq!(Ok(()), validate_dynamic_map(&mut input, &definition, None));

        let definition_json = serde_json::json!({"definition": {"type": "dynamic_map", "definition": {"type": "boolean"}, "size": 1}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(HashMap::from([("foo".to_string(), Input::from(true))]));
        assert_eq!(Ok(()), validate_dynamic_map(&mut input, &definition, None));
        input
            .map_mut()
            .unwrap()
            .insert("bar".to_string(), Input::from(false));
        assert!(validate_dynamic_map(&mut input, &definition, None).is_err());
        let mut input = Input::from(HashMap::from([("foo".to_string(), Input::from(0.0))]));
        assert!(validate_dynamic_map(&mut input, &definition, None).is_err());

        // let validation_rules = InputDefinition::new().with_definition_type(
        //     InputDefinitionType::static_map()
        //         .with_static_map_definition(
        //             "foo",
        //             InputDefinition::new()
        //                 .with_definition_type(
        //                     InputDefinitionType::string()
        //                         .with_string_size(InputDefinitionSize::new_with_max(10)),
        //                 )
        //                 .with_default("hello world"),
        //         )
        //         .with_static_map_definition(
        //             "bar",
        //             InputDefinition::new()
        //                 .with_definition_type(
        //                     InputDefinitionType::enum_()
        //                         .with_enum_item("x")
        //                         .with_enum_item("y")
        //                         .with_enum_item("z"),
        //                 )
        //                 .with_default("y"),
        //         )
        //         .with_static_map_definition(
        //             "baz",
        //             InputDefinition::new().with_definition_type(
        //                 InputDefinitionType::either()
        //                     .with_either_definition(
        //                         InputDefinitionType::integer().with_integer_range(
        //                             InputDefinitionRangeInteger::new_with_min(10),
        //                         ),
        //                     )
        //                     .with_either_definition(
        //                         InputDefinitionType::float().with_float_range(
        //                             InputDefinitionRangeFloat::new_with_min(10.0),
        //                         ),
        //                     ),
        //             ),
        //         ),
        // );
    }

    #[test]
    fn enum_() {
        enable_logging();
        let definition_json =
            serde_json::json!({"definition": {"type": "enum", "items": ["foo", true, 10, 3.14]}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(true);
        assert_eq!(Ok(()), validate_enum(&mut input, &definition, None));
        let mut input = Input::from("foo");
        assert_eq!(Ok(()), validate_enum(&mut input, &definition, None));
        let mut input = Input::from(10);
        assert_eq!(Ok(()), validate_enum(&mut input, &definition, None));
        let mut input = Input::from(3.14);
        assert_eq!(Ok(()), validate_enum(&mut input, &definition, None));
        let mut input = Input::from(false);
        assert!(validate_enum(&mut input, &definition, None).is_err());
    }

    #[test]
    fn either() {
        enable_logging();
        let definition_json = serde_json::json!({"definition": {"type": "either", "definitions": [{"type": "boolean"}, {"type": "string"}, {"type": "float"}]}});
        let definition: InputDefinition = serde_json::from_value(definition_json).unwrap();
        let mut input = Input::from(1);
        assert_eq!(Ok(()), validate_either(&mut input, &definition, None));
        assert_eq!(input.float_ref(), Some(&1.0));
    }
}

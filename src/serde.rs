use crate::{
    error::{i128_to_isize, invalid_type, InputDeserializeError, EXPECTED_INPUT_TYPES},
    position::InputPath,
    Input,
};
use serde::{
    de::{self, Deserialize, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, SerializeMap, SerializeSeq},
    Deserializer, Serializer,
};
use std::{collections::HashMap, fmt};

impl Input {
    /// Serializes `self`. This cannot fail for any supported [`Serializer`].
    pub fn serialize<S: Serializer>(&self, serializer: S) -> S::Ok {
        serialize_infallible(self, serializer)
    }
}

impl Serialize for Input {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Ok(serialize_infallible(self, serializer))
    }
}

fn serialize_infallible<S: Serializer>(input: &Input, serializer: S) -> S::Ok {
    match input {
        Input::Bool(value) => serializer.serialize_bool(*value).unwrap_infallible(),
        Input::Int(value) => serializer.serialize_i64(*value as i64).unwrap_infallible(),
        Input::Float(value) => serializer.serialize_f64(*value).unwrap_infallible(),
        Input::Str(value) => serializer.serialize_str(value).unwrap_infallible(),
        Input::List(values) => {
            let mut seq = serializer
                .serialize_seq(Some(values.len()))
                .unwrap_infallible();
            for value in values {
                seq.serialize_element(value).unwrap_infallible();
            }
            seq.end().unwrap_infallible()
        }
        Input::Map(values) => {
            let mut map = serializer
                .serialize_map(Some(values.len()))
                .unwrap_infallible();
            for (key, value) in values {
                map.serialize_key(key).unwrap_infallible();
                map.serialize_value(value).unwrap_infallible();
            }
            map.end().unwrap_infallible()
        }
    }
}

trait UnwrapInfallible<T> {
    fn unwrap_infallible(self) -> T;
}

impl<T, E: fmt::Debug> UnwrapInfallible<T> for Result<T, E> {
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|error| panic!("Input serialization cannot fail: {error:?}"))
    }
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(InputSeed {
            path: InputPath::root(),
        })
    }
}

struct InputSeed {
    path: InputPath,
}

impl Clone for InputSeed {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
        }
    }
}

impl<'de> DeserializeSeed<'de> for InputSeed {
    type Value = Input;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl InputSeed {
    fn invalid_type<E: de::Error>(&self, found: impl Into<String>) -> E {
        E::custom(invalid_type(self.path.clone(), found).to_string())
    }

    fn integer_out_of_range<E: de::Error>(&self, value: i128) -> E {
        E::custom(
            InputDeserializeError::IntegerOutOfRange {
                path: self.path.clone(),
                value,
            }
            .to_string(),
        )
    }

    fn int_from_i128<E: de::Error>(&self, value: i128) -> Result<Input, E> {
        match i128_to_isize(value) {
            Ok(value) => Ok(Input::Int(value)),
            Err(()) => Err(self.integer_out_of_range(value)),
        }
    }
}

impl<'de> Visitor<'de> for InputSeed {
    type Value = Input;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a {EXPECTED_INPUT_TYPES}")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Input::Bool(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        self.int_from_i128(value as i128)
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.int_from_i128(value as i128)
    }

    fn visit_i128<E: de::Error>(self, value: i128) -> Result<Self::Value, E> {
        self.int_from_i128(value)
    }

    fn visit_u128<E: de::Error>(self, value: u128) -> Result<Self::Value, E> {
        self.int_from_i128(value as i128)
    }

    fn visit_f32<E: de::Error>(self, value: f32) -> Result<Self::Value, E> {
        Ok(Input::Float(value as f64))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(Input::Float(value))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Input::Str(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(Input::Str(value))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut values = Vec::new();
        let mut index = 0;
        while let Some(value) = seq.next_element_seed(InputSeed {
            path: self.path.clone().with_index(index),
        })? {
            values.push(value);
            index += 1;
        }
        Ok(Input::List(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut values = HashMap::new();
        while let Some(key) = map.next_key::<String>()? {
            let value = map.next_value_seed(InputSeed {
                path: self.path.clone().with_key(&key),
            })?;
            values.insert(key, value);
        }
        Ok(Input::Map(values))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Err(self.invalid_type("null"))
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Err(self.invalid_type("null"))
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_enum<A: de::EnumAccess<'de>>(self, _data: A) -> Result<Self::Value, A::Error> {
        Err(self.invalid_type("enum"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::InputSerializeError;
    use serde_json::{json, Value};

    #[test]
    fn serialize_is_infallible() {
        let input = Input::Map(HashMap::from([
            ("list".to_string(), Input::from([1, 2, 3])),
            ("flag".to_string(), Input::from(true)),
        ]));
        let _: Result<Value, InputSerializeError> =
            Ok(input.serialize(serde_json::value::Serializer));
    }

    #[test]
    fn rejects_null_with_clean_error() {
        let error = serde_json::from_str::<Input>("null").unwrap_err();
        assert!(error
            .to_string()
            .starts_with("expected boolean, integer, float, string, list, or map, found null"));
    }

    #[test]
    fn rejects_invalid_type_at_path() {
        let error = serde_json::from_str::<Input>(r#"{"foo": null}"#).unwrap_err();
        assert!(error.to_string().starts_with(
            "[foo]: expected boolean, integer, float, string, list, or map, found null"
        ));
    }

    #[test]
    fn rejects_out_of_range_integer_at_path() {
        let value = if cfg!(target_pointer_width = "64") {
            json!(i64::MAX as i128 + 1)
        } else {
            json!(i64::MAX)
        };
        let json = format!(r#"{{"count": {value}}}"#);
        let error = serde_json::from_str::<Input>(&json).unwrap_err();
        assert!(error.to_string().starts_with(&format!(
            "[count]: integer {value} is out of range for isize"
        )));
    }

    #[test]
    fn roundtrip() {
        let input = Input::from(HashMap::from([
            ("name".to_string(), Input::from("plugx")),
            ("enabled".to_string(), Input::from(true)),
            ("count".to_string(), Input::from(3)),
            ("ratio".to_string(), Input::from(0.5)),
        ]));
        let json = serde_json::to_string(&input).unwrap();
        let decoded: Input = serde_json::from_str(&json).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn deep_json_roundtrip() {
        let json = r#"{
            "app": {
                "server": {
                    "host": "localhost",
                    "port": 8080,
                    "routes": [
                        {
                            "path": "/api",
                            "handler": {
                                "type": "proxy",
                                "target": {
                                    "url": "http://example.com",
                                    "headers": ["accept", "authorization"]
                                }
                            }
                        }
                    ]
                },
                "logging": {
                    "level": "debug",
                    "filters": {
                        "modules": {
                            "plugx": {
                                "input": {
                                    "serde": true
                                }
                            }
                        }
                    }
                }
            }
        }"#;

        let decoded: Input = serde_json::from_str(json).unwrap();
        let encoded = serde_json::to_string(&decoded).unwrap();
        let roundtripped: Input = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, roundtripped);

        assert_eq!(
            decoded
                .as_map()
                .unwrap()
                .get("app")
                .unwrap()
                .as_map()
                .unwrap()
                .get("logging")
                .unwrap()
                .as_map()
                .unwrap()
                .get("filters")
                .unwrap()
                .as_map()
                .unwrap()
                .get("modules")
                .unwrap()
                .as_map()
                .unwrap()
                .get("plugx")
                .unwrap()
                .as_map()
                .unwrap()
                .get("input")
                .unwrap()
                .as_map()
                .unwrap()
                .get("serde")
                .unwrap(),
            &Input::Bool(true)
        );
    }

    #[test]
    fn rejects_invalid_type_at_deep_path() {
        let error = serde_json::from_str::<Input>(
            r#"{"app": {"logging": {"filters": {"modules": {"plugx": null}}}}}"#,
        )
        .unwrap_err();
        assert!(
            error.to_string().starts_with(
                "[app][logging][filters][modules][plugx]: expected boolean, integer, float, string, list, or map, found null"
            )
        );
    }
}

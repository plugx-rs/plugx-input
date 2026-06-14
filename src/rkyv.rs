use crate::{Input, error::InputDeserializeError, position::InputPath};
use rkyv::{from_bytes, rancor, to_bytes, util::AlignedVec};

impl Input {
    /// Serializes `self` to rkyv bytes. This cannot fail.
    pub fn to_rkyv_bytes(&self) -> AlignedVec {
        to_bytes::<rancor::Error>(self).expect("Input rkyv serialization cannot fail")
    }

    /// Deserializes `self` from rkyv bytes with structured errors.
    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self, InputDeserializeError> {
        from_bytes::<Input, rancor::Error>(bytes).map_err(|error| {
            InputDeserializeError::InvalidArchive {
                path: InputPath::root(),
                message: error.to_string(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::InputSerializeError;
    use std::collections::HashMap;

    #[test]
    fn serialize_is_infallible() {
        let input = Input::from("hello");
        let _: Result<AlignedVec, InputSerializeError> = Ok(input.to_rkyv_bytes());
    }

    #[test]
    fn roundtrip() {
        let input = Input::from(HashMap::from([
            ("name".to_string(), Input::from("plugx")),
            ("enabled".to_string(), Input::from(true)),
            ("count".to_string(), Input::from(3)),
            ("ratio".to_string(), Input::from(0.5)),
        ]));
        let bytes = input.to_rkyv_bytes();
        let decoded = Input::from_rkyv_bytes(&bytes).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn deep_roundtrip() {
        let input = Input::from(HashMap::from([(
            "app".to_string(),
            Input::from(HashMap::from([(
                "logging".to_string(),
                Input::from(HashMap::from([(
                    "filters".to_string(),
                    Input::from(HashMap::from([(
                        "modules".to_string(),
                        Input::from(HashMap::from([(
                            "plugx".to_string(),
                            Input::from(HashMap::from([(
                                "input".to_string(),
                                Input::from(HashMap::from([(
                                    "rkyv".to_string(),
                                    Input::from(true),
                                )])),
                            )])),
                        )])),
                    )])),
                )])),
            )])),
        )]));

        let bytes = input.to_rkyv_bytes();
        let decoded = Input::from_rkyv_bytes(&bytes).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn rejects_invalid_bytes() {
        let error = Input::from_rkyv_bytes(&[0u8, 1, 2, 3]).unwrap_err();
        assert!(matches!(
            error,
            InputDeserializeError::InvalidArchive { .. }
        ));
    }
}

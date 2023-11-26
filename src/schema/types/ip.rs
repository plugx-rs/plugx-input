use crate::{
    position::InputPosition,
    schema::{default::default_true, InputSchemaError},
    Input,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeIp {
    #[serde(default = "default_true")]
    v4: bool,
    #[serde(default = "default_true")]
    v6: bool,
}

impl InputSchemaTypeIp {
    pub fn v4(&self) -> bool {
        self.v4
    }

    pub fn v4_mut(&mut self) -> &mut bool {
        &mut self.v4
    }

    pub fn v6(&self) -> bool {
        self.v6
    }

    pub fn v6_mut(&mut self) -> &mut bool {
        &mut self.v6
    }

    pub fn set_v4(&mut self, flag: bool) {
        *self.v4_mut() = flag;
    }

    pub fn with_v4(mut self, flag: bool) -> Self {
        self.set_v4(flag);
        self
    }

    pub fn set_v6(&mut self, flag: bool) {
        *self.v6_mut() = flag;
    }

    pub fn with_v6(mut self, flag: bool) -> Self {
        self.set_v6(flag);
        self
    }
}

impl InputSchemaTypeIp {
    pub fn validate(
        &self,
        input: &mut Input,
        maybe_position: Option<InputPosition>,
    ) -> Result<(), InputSchemaError> {
        if !input.is_str() {
            return Err(InputSchemaError::Type {
                position: maybe_position.unwrap_or_default(),
                expected_type: Input::str_type_name(),
                input_type: input.type_name(),
            });
        }
        let ip = input.as_str().as_str();
        if self.v4 && !self.v6 {
            Ipv4Addr::from_str(ip).map(|_| ())
        } else if self.v6 && !self.v4 {
            Ipv6Addr::from_str(ip).map(|_| ())
        } else {
            IpAddr::from_str(ip).map(|_| ())
        }
        .map_err(|error| InputSchemaError::Invalid {
            description: format!("Invalid IP address ({error})"),
            position: maybe_position.unwrap_or_default(),
        })?;
        Ok(())
    }
}

impl Display for InputSchemaTypeIp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(if !self.v4 && self.v6 {
            "IPv6 address"
        } else if self.v4 && !self.v6 {
            "IPv4 address"
        } else {
            "IP address"
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

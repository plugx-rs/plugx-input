use crate::{
    position::InputPosition,
    schema::{
        default::{default_port_zero, default_true},
        InputSchemaError,
    },
    Input,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::SocketAddr,
};

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct InputSchemaTypeSocketAddress {
    #[serde(default = "default_true")]
    v4: bool,
    #[serde(default = "default_true")]
    v6: bool,
    #[serde(default = "default_port_zero")]
    port_start: u16,
}

impl InputSchemaTypeSocketAddress {
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

    pub fn port_start(&self) -> u16 {
        self.port_start
    }

    pub fn port_start_mut(&mut self) -> &mut u16 {
        &mut self.port_start
    }

    pub fn set_port_start(&mut self, start: u16) {
        *self.port_start_mut() = start
    }

    pub fn with_port_start(mut self, start: u16) -> Self {
        self.set_port_start(start);
        self
    }
}

impl InputSchemaTypeSocketAddress {
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
        let socket_address_str = input.as_str();
        let socket_address = match socket_address_str.parse::<SocketAddr>() {
            Ok(socket_address) => socket_address,
            Err(error) => {
                return Err(InputSchemaError::Invalid {
                    description: format!("Could not parse socket address: {error}"),
                    position: maybe_position.unwrap_or_default(),
                    input: input.clone(),
                });
            }
        };
        let ip = socket_address.ip();
        let maybe_error = if self.v4 && !self.v6 && ip.is_ipv6() {
            Some("Only IPv4 is supported".to_string())
        } else if self.v6 && !self.v4 && ip.is_ipv4() {
            Some("Only IPv6 is supported".to_string())
        } else {
            None
        };
        if let Some(error) = maybe_error {
            return Err(InputSchemaError::Invalid {
                description: error,
                position: maybe_position.unwrap_or_default(),
                input: input.clone(),
            });
        }
        // TODO: port
        Ok(())
    }
}

impl Display for InputSchemaTypeSocketAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ip = if !self.v4 && self.v6 {
            "IPv6 address"
        } else if self.v4 && !self.v6 {
            "IPv4 address"
        } else {
            "IP address"
        };
        let port = if self.port_start != 0 {
            format!(" which port number should be at least {}", self.port_start)
        } else {
            String::new()
        };
        f.write_str(format!("`<{ip}>:<Port>`{port}").as_str())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serde() {}
}

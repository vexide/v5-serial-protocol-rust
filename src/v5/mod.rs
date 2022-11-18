use std::io::{Read, Write};

use crate::devices::SocketInfoPairs;

/// The representation of a V5 device
pub struct Device<S: Read + Write, U: Read+Write> {
    system_port: S,
    user_port: Option<U>
}

impl<S: Read + Write, U: Read+Write> Device<S, U> {
    pub fn new(system: S, user: Option<U>) -> Self {
        
        Device {
            system_port: system,
            user_port: user
        }
    }

    /// Sends a command and recieves its response
    pub fn send_request<C: crate::commands::Command + Copy>(&mut self, command: C) -> Result<C::Response, crate::errors::DecodeError> {
        // Send the command over the system port
        self.send_command(command)?;

        // Wait for the response
        self.response_for::<C>()
    }

    /// Sends a command
    pub fn send_command<C: crate::commands::Command + Copy>(&mut self, command: C) -> Result<(), crate::errors::DecodeError> {

        // Encode the command
        let encoded = command.encode_request();
        
        // Write the command to the serial port
        match self.system_port.write_all(&encoded) {
            Ok(_) => (),
            Err(e) => return Err(crate::errors::DecodeError::IoError(e))
        };

        match self.system_port.flush() {
            Ok(_) => (),
            Err(e) => return Err(crate::errors::DecodeError::IoError(e))
        };

        Ok(())
    }

    /// Recieves a response for a command
    pub fn response_for<C: crate::commands::Command + Copy>(&mut self) -> Result<C::Response, crate::errors::DecodeError> {
        C::decode_stream(&mut self.system_port, std::time::Duration::from_secs(10))
    }
}
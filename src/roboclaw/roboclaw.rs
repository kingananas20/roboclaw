use super::connection::Connection;
use super::commands::Commands;
use std::time::Duration;
use pyo3::prelude::*;
use anyhow::{anyhow, Context, Result};

#[pyclass]
pub struct RoboClaw {
    connection: Connection,
    address: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Motor {
    M1 = 1,
    M2 = 2,
}

#[pymethods]
impl RoboClaw {
    #[new]
    fn new(port_name: &str, baud_rate: u32, timeout: Duration, retries: u8, address: u8) -> Result<Self> {
        let connection: Connection = Connection::new(port_name, baud_rate, timeout, retries)
            .context("couldn't make a new connection")?;

        Ok(Self {
            connection,
            address
        })
    }

    #[pyo3(signature = (motor, speed, address=None))]
    fn set_speed(&mut self, motor: Motor, speed: i8, address: Option<u8>) -> Result<bool> {
        let command: Commands = match (motor, speed) {
            (Motor::M1, 0..=127) => Commands::M1Forward,
            (Motor::M1, -127..=-1) => Commands::M1Backward,
            (Motor::M2, 0..=127) => Commands::M2Forward,
            (Motor::M2, -127..=-1) => Commands::M2Backward,
            _ => return Ok(false),
        };
        let address_value: u8 = address.unwrap_or(self.address);
        self.connection.write(address_value, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (speed, address=None))]
    fn drive(&mut self, speed: i8, address: Option<u8>) -> Result<bool> {
        let command: Commands = match speed {
            0..=127 => Commands::MixDriveForward,
            -127..=-1 => Commands::MixDriveBackward,
            _ => return Ok(false),
        };
        let address_value: u8 = address.unwrap_or(self.address);
        self.connection.write(address_value, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }
}
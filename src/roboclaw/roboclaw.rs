use super::connection::Connection;
use super::commands::Commands;
use super::common::calculate_encoder;
use std::time::Duration;
use pyo3::prelude::*;
use anyhow::{Context, Result};

#[pyclass]
pub struct RoboClaw {
    connection: Connection,
    address: u8,
    encoder_value_m1: i64,
    encoder_value_m2: i64,
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
    fn new(port_name: &str, baud_rate: u32, timeout: u32, retries: u8, address: u8) -> Result<Self> {
        let connection: Connection = Connection::new(port_name, baud_rate, Duration::new(0, timeout * 1_000_000), retries)
            .context("couldn't make a new connection")?;

        Ok(Self {
            connection,
            address,
            encoder_value_m1: 0,
            encoder_value_m2: 0,
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
        let address: u8 = address.unwrap_or(self.address);
        self.connection.write(address, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (speed, address=None))]
    fn drive(&mut self, speed: i8, address: Option<u8>) -> Result<bool> {
        let command: Commands = match speed {
            0..=127 => Commands::MixDriveForward,
            -127..=-1 => Commands::MixDriveBackward,
            _ => return Ok(false),
        };
        let address: u8 = address.unwrap_or(self.address);
        self.connection.write(address, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (speed, address=None))]
    fn turn(&mut self, speed: i8, address: Option<u8>) -> Result<bool> {
        let command: Commands = match speed {
            0..=127 => Commands::MixTurnRight,
            -127..=-1 => Commands::MixTurnLeft,
            _ => return Ok(false)
        };
        let address: u8 = address.unwrap_or(self.address);
        self.connection.write(address, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (motor, address=None))]
    fn read_encoder(&mut self, motor: Motor, address: Option<u8>) -> Result<i64> {
        let command: Commands = match motor {
            Motor::M1 => Commands::M1ReadEncoder,
            Motor::M2 => Commands::M2ReadEncoder,
        };
        let address: u8 = address.unwrap_or(self.address);
        let read_result: Vec<u32> = self.connection.read(address, command, vec![4, 1])?;

        match motor {
            Motor::M1 => {
                let encoder_value: i64 = calculate_encoder(self.encoder_value_m1, read_result);
                self.encoder_value_m1 = encoder_value;
                return Ok(encoder_value);
            },
            Motor::M2 => {
                let encoder_value: i64 = calculate_encoder(self.encoder_value_m2, read_result);
                self.encoder_value_m2 = encoder_value;
                return Ok(encoder_value);
            },
        }
    }
}
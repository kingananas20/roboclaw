use super::commands::Commands;
use super::common::calculate_encoder;
use super::connection::Connection;
use anyhow::{Context, Ok, Result};
use pyo3::prelude::*;
use std::time::Duration;

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
    fn new(
        port_name: &str,
        baud_rate: u32,
        timeout: u32,
        retries: u8,
        address: u8,
    ) -> Result<Self> {
        let connection: Connection = Connection::new(
            port_name,
            baud_rate,
            Duration::new(0, timeout * 1_000_000),
            retries,
        )
        .context("couldn't make a new connection")?;

        Ok(Self {
            connection,
            address,
            encoder_value_m1: 0,
            encoder_value_m2: 0,
        })
    }

    //--------------------------------[Simple Commands]--------------------------------//

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
        self.connection
            .write(address, command, &[speed.unsigned_abs() as u32])?;
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
        self.connection
            .write(address, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (speed, address=None))]
    fn turn(&mut self, speed: i8, address: Option<u8>) -> Result<bool> {
        let command: Commands = match speed {
            0..=127 => Commands::MixTurnRight,
            -127..=-1 => Commands::MixTurnLeft,
            _ => return Ok(false),
        };
        let address: u8 = address.unwrap_or(self.address);
        self.connection
            .write(address, command, &[speed.unsigned_abs() as u32])?;
        Ok(true)
    }

    //--------------------------------[Encoders]--------------------------------//

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
                Ok(encoder_value)
            }
            Motor::M2 => {
                let encoder_value: i64 = calculate_encoder(self.encoder_value_m2, read_result);
                self.encoder_value_m2 = encoder_value;
                Ok(encoder_value)
            }
        }
    }

    #[pyo3(signature = (address=None))]
    fn reset_encoders(&mut self, address: Option<u8>) -> Result<bool> {
        let address: u8 = address.unwrap_or(self.address);
        self.connection
            .write(address, Commands::ResetEncoders, &[])?;
        Ok(true)
    }

    #[pyo3(signature = (motor, encoder_value, address=None))]
    fn set_encoder(
        &mut self,
        motor: Motor,
        encoder_value: i32,
        address: Option<u8>,
    ) -> Result<bool> {
        let command: Commands = match motor {
            Motor::M1 => Commands::M1SetEncoder,
            Motor::M2 => Commands::M2SetEncoder,
        };
        let address: u8 = address.unwrap_or(self.address);
        self.connection
            .write(address, command, &[encoder_value as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (motor, address=None))]
    fn read_encoder_speed(&mut self, motor: Motor, address: Option<u8>) -> Result<i64> {
        let command: Commands = match motor {
            Motor::M1 => Commands::M1ReadSpeedCPS,
            Motor::M2 => Commands::M2ReadSpeedCPS,
        };
        let address: u8 = address.unwrap_or(self.address);
        let result: Vec<u32> = self.connection.read(address, command, vec![4, 1])?;

        let mut speed: i64 = result[0] as i64;
        if result[1] == 1 {
            speed *= -1;
        }

        Ok(speed)
    }

    #[pyo3(signature = (motor, address=None))]
    fn read_raw_speed(&mut self, motor: Motor, address: Option<u8>) -> Result<i64> {
        let command: Commands = match motor {
            Motor::M1 => Commands::M1ReadRawSpeed,
            Motor::M2 => Commands::M2ReadRawSpeed,
        };
        let address: u8 = address.unwrap_or(self.address);
        let result: Vec<u32> = self.connection.read(address, command, vec![4, 1])?;

        let mut speed: i64 = result[0] as i64;
        if result[1] == 1 {
            speed += -1;
        }

        Ok(speed)
    }

    #[pyo3(signature = (motor, address=None))]
    fn read_avg_speed(&mut self, motor: Motor, address: Option<u8>) -> Result<i64> {
        let address: u8 = address.unwrap_or(self.address);
        let read_result: Vec<u32> =
            self.connection
                .read(address, Commands::ReadMotorAvgSpeed, vec![4, 4])?;

        Ok(match motor {
            Motor::M1 => read_result[0] as i32 as i64,
            Motor::M2 => read_result[1] as i32 as i64,
        })
    }

    #[pyo3(signature = (motor, address=None))]
    fn read_speed_error(&mut self, motor: Motor, address: Option<u8>) -> Result<i64> {
        let address: u8 = address.unwrap_or(self.address);
        let read_result: Vec<u32> =
            self.connection
                .read(address, Commands::ReadSpeedErrors, vec![4, 4])?;

        Ok(match motor {
            Motor::M1 => read_result[0] as i32 as i64,
            Motor::M2 => read_result[1] as i32 as i64,
        })
    }

    #[pyo3(signature = (motor, address=None))]
    fn read_position_error(&mut self, motor: Motor, address: Option<u8>) -> Result<i64> {
        let address: u8 = address.unwrap_or(self.address);
        let read_result: Vec<u32> =
            self.connection
                .read(address, Commands::ReadPositionErrors, vec![4, 4])?;

        Ok(match motor {
            Motor::M1 => read_result[0] as i32 as i64,
            Motor::M2 => read_result[1] as i32 as i64,
        })
    }

    //-----------------------------[Advanced Motor Controls]--------------------------------//

    #[pyo3(signature = (motor, qpps, proportional, integral, derivative, address=None))]
    fn set_velocity_pid(
        &mut self,
        motor: Motor,
        qpps: i32,
        proportional: i32,
        integral: i32,
        derivative: i32,
        address: Option<u8>,
    ) -> Result<bool> {
        let command: Commands = match motor {
            Motor::M1 => Commands::M1SetVelocityPIDConst,
            Motor::M2 => Commands::M2SetVelocityPIDConst,
        };
        let address: u8 = address.unwrap_or(self.address);
        self.connection.write(
            address,
            command,
            &[
                derivative as u32,
                proportional as u32,
                integral as u32,
                qpps as u32,
            ],
        )?;
        Ok(true)
    }

    #[pyo3(signature = (motor, duty, address=None))]
    fn set_speed_duty(&mut self, motor: Motor, duty: i16, address: Option<u8>) -> Result<bool> {
        let command: Commands = match motor {
            Motor::M1 => Commands::M1DriveSignedDutyCycle,
            Motor::M2 => Commands::M2DriveSignedDutyCycle,
        };
        let address: u8 = address.unwrap_or(self.address);
        self.connection
            .write(address, command, &[duty as i32 as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (duty, address=None))]
    fn drive_duty(&mut self, duty: i16, address: Option<u8>) -> Result<bool> {
        let address: u8 = address.unwrap_or(self.address);
        self.connection.write(
            address,
            Commands::MixDriveSignedDutyCycle,
            &[duty as i32 as u32],
        )?;
        Ok(true)
    }

    //--------------------------------[Advanced Commands]--------------------------------//

    #[pyo3(signature = (timeout, address=None))]
    fn set_serial_timeout(&mut self, timeout: u8, address: Option<u8>) -> Result<bool> {
        let address: u8 = address.unwrap_or(self.address);
        self.connection
            .write(address, Commands::SetSerialTimeout, &[timeout as u32])?;
        Ok(true)
    }

    #[pyo3(signature = (address=None))]
    fn read_serial_timeout(&mut self, address: Option<u8>) -> Result<u8> {
        let address: u8 = address.unwrap_or(self.address);
        let result: Vec<u32> =
            self.connection
                .read(address, Commands::ReadSerialTimeout, vec![1])?;
        Ok(result[0] as u8)
    }
}

use serialport::{SerialPort, ClearBuffer};
use std::time::Duration;
use thiserror::Error;
use super::{commands::Commands, Crc16};

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Serial port error: {0}")]
    Serial(#[from] serialport::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error), 
    #[error("Timeout after {0} retries")]
    Timeout(u8),
    #[error("CRC mismatch")]
    CrcMismatch,
    #[error("Invalid ACK received")]
    InvalidAck,
    #[error("Invalid value: {0}")]
    InvalidValue(u32),
}

pub struct Connection {
    port: Box<dyn SerialPort>,
    baud_rate: u32,
    timeout: Duration,
    retries: u8,
    crc: Crc16
}

impl Connection {
    pub fn new(port_name: &str, baud_rate: u32, timeout: Duration, retries: u8) -> Result<Self, ConnectionError> {
        let port: Box<dyn SerialPort> = serialport::new(port_name, baud_rate)
            .timeout(timeout)
            .open()?;

        Ok(Self {
            port,
            baud_rate,
            timeout,
            retries,
            crc: Crc16::new(),
        })
    }

    fn reset_connection(&mut self) -> Result<(), ConnectionError> {
        self.port.clear(ClearBuffer::Input)?;
        self.crc.clear();
        Ok(())
    }

    fn send_command(&mut self, address: u8, command: Commands) -> Result<(), ConnectionError> {
        self.crc.clear();
        self.crc.update(address);
        self.port.write_all(&[address])?;
        self.crc.update(command as u8);
        self.port.write_all(&[command as u8])?;
        Ok(())
    }

    //-----------------------------------------------------------------------------------------------------------------------------------------------//
    //----------------------------------------------------------------[Write Methods]----------------------------------------------------------------//
    //-----------------------------------------------------------------------------------------------------------------------------------------------//

    pub fn write(&mut self, address: u8, command: Commands, values: &[u32]) -> Result<(), ConnectionError> {
        for _ in 0..self.retries {
            self.reset_connection()?;
            
            self.send_command(address, command)?;
            
            for &val in values {
                match val {
                    0..=0xFF => self.write_u8(val as u8)?,
                    0x100..=0xFFFF => self.write_u16(val as u16)?,
                    _ => self.write_u32(val)?,
                }
            }

            if self.verify_write_checksum()? {
                return Ok(());
            }
        }

        Err(ConnectionError::Timeout(self.retries))
    }

    fn write_u8(&mut self, byte: u8) -> Result<(), ConnectionError> {
        self.crc.update(byte);
        self.port.write_all(&[byte])?;
        Ok(())
    }

    fn write_u16(&mut self, value: u16) -> Result<(), ConnectionError> {
        let bytes: [u8; 2] = value.to_be_bytes();
        self.write_u8(bytes[0])?;
        self.write_u8(bytes[1])?;
        Ok(())
    }

    fn write_u32(&mut self, value: u32) -> Result<(), ConnectionError> {
        let bytes: [u8; 4] = value.to_be_bytes();
        self.write_u8(bytes[0])?;
        self.write_u8(bytes[1])?;
        self.write_u8(bytes[2])?;
        self.write_u8(bytes[3])?;
        Ok(())
    }

    fn verify_write_checksum(&mut self) -> Result<bool, ConnectionError> {
        let crc_bytes: [u8; 2] = self.crc.get().to_be_bytes();
        self.port.write_all(&crc_bytes)?;

        let mut ack: [u8; 1] = [0u8; 1];
        match self.port.read_exact(&mut ack) {
            Ok(_) => Ok(ack[0] == 0xFF),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    //----------------------------------------------------------------------------------------------------------------------------------------------//
    //----------------------------------------------------------------[Read Methods]----------------------------------------------------------------//
    //----------------------------------------------------------------------------------------------------------------------------------------------//
}
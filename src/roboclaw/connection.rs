use serialport::{SerialPort, ClearBuffer};
use std::time::Duration;
use anyhow::{anyhow, Context, Result};
use super::{commands::Commands, Crc16};

pub struct Connection {
    port: Box<dyn SerialPort>,
    retries: u8,
    crc: Crc16
}

impl Connection {
    pub fn new(port_name: &str, baud_rate: u32, timeout: Duration, retries: u8) -> Result<Self> {
        let port: Box<dyn SerialPort> = serialport::new(port_name, baud_rate)
            .timeout(timeout)
            .open()
            .with_context(|| format!("error while while creating a new serialport using following values:\nport_name: {}\nbaud_rate: {}\ntimout: {:?}\nretries: {}", port_name, baud_rate, timeout, retries))?;

        Ok(Self {
            port,
            retries,
            crc: Crc16::new(),
        })
    }

    fn reset_connection(&mut self) -> Result<()> {
        self.port.clear(ClearBuffer::Input)?;
        self.crc.clear();
        Ok(())
    }

    fn send_command(&mut self, address: u8, command: Commands) -> Result<()> {
        self.crc.update(address);
        self.port.write_all(&[address])?;
        self.crc.update(command as u8);
        self.port.write_all(&[command as u8])?;
        Ok(())
    }

    //-----------------------------------------------------------------------------------------------------------------------------------------------//
    //----------------------------------------------------------------[Write Methods]----------------------------------------------------------------//
    //-----------------------------------------------------------------------------------------------------------------------------------------------//

    pub fn write(&mut self, address: u8, command: Commands, values: &[u32]) -> Result<()> {
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

        Err(anyhow!("timeout after {}", self.retries))
    }

    fn write_u8(&mut self, byte: u8) -> Result<()> {
        self.crc.update(byte);
        self.port.write_all(&[byte])?;
        Ok(())
    }

    fn write_u16(&mut self, value: u16) -> Result<()> {
        let bytes: [u8; 2] = value.to_be_bytes();
        self.crc.update_bytes(&bytes);
        self.port.write_all(&bytes)?;
        Ok(())
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        let bytes: [u8; 4] = value.to_be_bytes();
        self.crc.update_bytes(&bytes);
        self.port.write_all(&bytes)?;
        Ok(())
    }

    fn verify_write_checksum(&mut self) -> Result<bool> {
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

    pub fn read(&mut self, address: u8, command: Commands, num_reads: usize, byte_size: usize) -> Result<Vec<u32>> {
        for _ in 0..self.retries {
            self.reset_connection()?;
            self.send_command(address, command)?;

            let mut data: Vec<_> = Vec::new();
            for _ in 0..num_reads {
                let bytes: Vec<u8> = self.read_bytes(byte_size)?;
                let value: u32 = match byte_size {
                    1 => bytes[0] as u32,
                    2 => u16::from_be_bytes([bytes[0], bytes[1]]) as u32,
                    4 => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    _ => return Err(anyhow!("invalid value: {} needs to be 1, 2 or 4", &byte_size)),
                };
                data.push(value);
            }

            if self.read_checksum()? {
                return Ok(data);
            }
        }

        Err(anyhow!("timeout after {}", self.retries))
    }

    fn read_checksum(&mut self) -> Result<bool> {
        let crc: Vec<u8> = self.read_bytes(2)?;
        if self.crc.get() & 0xFFFF == u16::from_be_bytes([crc[0], crc[1]]) {
            return Ok(true);
        }
        Err(anyhow!("crc mismatch during reading"))
    }

    fn read_bytes(&mut self, byte_size: usize) -> Result<Vec<u8>>{
        let mut buf: Vec<u8> = vec![0u8; byte_size];
        self.port.read_exact(&mut buf)?;
        for b in &buf {
            self.crc.update(*b);
        }
        Ok(buf)
    }   
}
use std::time::Duration;
use super::connection::{Connection, ConnectionError};

pub struct RoboClaw {
    connection: Connection,
    address: Option<u8>,
}

impl RoboClaw {
    pub fn new(port_name: &str, baud_rate: u32, timeout: Duration, retries: u8, address: Option<u8>) -> Self {
        let connection: Connection = Connection::new(port_name, baud_rate, timeout, retries)
            .expect("couldn't make a new connection");

        Self {
            connection,
            address
        }
    }
}
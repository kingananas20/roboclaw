pub struct Crc16 {
    crc: u16,
}

impl Crc16 {
    pub fn new() -> Self {
        Crc16 { crc: 0 }
    }

    pub fn update(&mut self, data: u8) {
        self.crc ^= (data as u16) << 8;
        for _ in 0..8 {
            if self.crc & 0x8000 != 0 {
                self.crc = (self.crc << 1) ^ 0x1021;
            } else {
                self.crc <<= 1;
            }
        }
    }

    pub fn clear(&mut self) {
        self.crc = 0;
    }

    pub fn get(&self) -> u16 {
        self.crc
    }
}
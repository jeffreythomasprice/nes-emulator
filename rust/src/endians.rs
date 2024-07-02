pub struct Word {
    pub low: u8,
    pub high: u8,
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Self {
            low: (value & 0xff) as u8,
            high: (value >> 8) as u8,
        }
    }
}

impl Into<u16> for Word {
    fn into(self) -> u16 {
        (self.low as u16) | ((self.high as u16) << 8)
    }
}

use std::{error::Error, fmt::Display};

const PRG_BLOCK_SIZE: usize = 1024 * 16;
const CHR_BLOCK_SIZE: usize = 1024 * 8;

#[derive(Debug)]
pub enum CartridgeError {
    ParseError(String),
}

impl Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CartridgeError {}

pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, CartridgeError> {
        if &data[0..=3] != &['N' as u8, 'E' as u8, 'S' as u8, 0x1a] {
            return Err(CartridgeError::ParseError(
                "doesn't start with expected header".to_string(),
            ));
        }

        if data.len() < 16 {
            return Err(CartridgeError::ParseError("missing header".to_string()));
        }

        let result = Self { data };

        Ok(result)
    }

    pub fn prg_size_in_blocks(&self) -> u8 {
        self.data[4]
    }

    pub fn chr_size_in_blocks(&self) -> u8 {
        self.data[5]
    }
}

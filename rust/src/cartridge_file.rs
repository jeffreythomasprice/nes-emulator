// see "INES - NESdev Wiki.html"

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CartridgeError {
    BadHeader,
    MissingPRGROM,
}

impl Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CartridgeError {}

pub struct PRGROMBlocks(u8);

impl PRGROMBlocks {
    pub fn in_bytes(&self) -> usize {
        (self.0 as usize) * 1024 * 16
    }
}

pub struct CHRROMBlocks(u8);

impl CHRROMBlocks {
    pub fn in_bytes(&self) -> usize {
        (self.0 as usize) * 1024 * 8
    }
}

pub struct PRGRAMBlocks(u8);

impl PRGRAMBlocks {
    pub fn in_bytes(&self) -> usize {
        (self.0 as usize) * 1024 * 8
    }
}

pub enum NametableArrangement {
    Vertical,
    Horizontal,
}

pub enum TVSystem {
    NTSC,
    PAL,
    Both,
}

const PRG_ROM_INDEX: usize = 4;

pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, CartridgeError> {
        if data.len() < 16 {
            return Err(CartridgeError::BadHeader);
        }

        if &data[0..=3] != &['N' as u8, 'E' as u8, 'S' as u8, 0x1a] {
            return Err(CartridgeError::BadHeader);
        }

        if data[PRG_ROM_INDEX] == 0 {
            return Err(CartridgeError::MissingPRGROM);
        }

        /*
        TODO in order, the rest of the bytes should be:
            Trainer, if present (0 or 512 bytes)
            PRG ROM data (16384 * x bytes)
            CHR ROM data, if present (8192 * y bytes)
            PlayChoice INST-ROM, if present (0 or 8192 bytes)
            PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing; see PC10 ROM-Images for details)
        */

        let result = Self { data };

        Ok(result)
    }

    pub fn prg_rom_size_in_blocks(&self) -> PRGROMBlocks {
        PRGROMBlocks(self.data[PRG_ROM_INDEX])
    }

    pub fn chr_rom_size_in_blocks(&self) -> CHRROMBlocks {
        CHRROMBlocks(self.data[5])
    }

    pub fn nametable_arrangement(&self) -> NametableArrangement {
        if (self.data[6] & 0b0000_0001) == 0 {
            NametableArrangement::Vertical
        } else {
            NametableArrangement::Horizontal
        }
    }

    pub fn has_battery_backed_prg_ram(&self) -> bool {
        (self.data[6] & 0b0000_0010) != 0
    }

    pub fn has_trainer(&self) -> bool {
        (self.data[6] & 0b0000_0100) != 0
    }

    pub fn has_alternative_nametable_layout(&self) -> bool {
        (self.data[6] & 0b0000_1000) != 0
    }

    pub fn mapper_number(&self) -> u8 {
        let low = (self.data[6] & 0b1111_0000) >> 4;
        let high = self.data[7] & 0b1111_0000;
        low | high
    }

    pub fn is_vs_unisystem(&self) -> bool {
        (self.data[7] & 0b0000_0001) != 0
    }

    pub fn is_playchoice_10(&self) -> bool {
        (self.data[7] & 0b0000_0010) != 0
    }

    /// flags from byte 8 through 15 are in NES 2.0 foramt
    pub fn is_nes_2_0(&self) -> bool {
        (self.data[7] & 0b0000_1100) == 0b0000_1100
    }

    pub fn prg_ram_size_in_blocks(&self) -> PRGRAMBlocks {
        let x = self.data[8];
        PRGRAMBlocks(if x == 0 { 1 } else { x })
    }

    pub fn tv_system(&self) -> TVSystem {
        match self.data[10] & 0b0000_0011 {
            0 => TVSystem::NTSC,
            2 => TVSystem::PAL,
            _ => TVSystem::Both,
        }
    }

    pub fn has_prg_ram(&self) -> bool {
        (self.data[10] & 0b0001_0000) == 0
    }

    pub fn has_bus_conflicts(&self) -> bool {
        (self.data[10] & 0b0010_0000) != 0
    }
}

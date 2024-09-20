// see "INES - NESdev Wiki.html"

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CartridgeError {
    BadHeader,
    MissingTrainer,
    MissingPRGROM,
    MissingCHRROM,
}

impl Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CartridgeError {}

pub mod PrgRom {
    pub const BLOCK_SIZE: usize = 1024 * 16;

    pub struct Size(pub u8);

    impl Size {
        pub fn in_blocks(&self) -> u8 {
            self.0
        }

        pub fn in_bytes(&self) -> usize {
            (self.in_blocks() as usize) * BLOCK_SIZE
        }
    }

    pub type Block = [u8; BLOCK_SIZE];
}

pub mod ChrRom {
    pub const BLOCK_SIZE: usize = 1024 * 8;

    pub struct Size(pub u8);

    impl Size {
        pub fn in_blocks(&self) -> u8 {
            self.0
        }

        pub fn in_bytes(&self) -> usize {
            (self.in_blocks() as usize) * BLOCK_SIZE
        }
    }

    pub type Block = [u8; BLOCK_SIZE];
}

pub mod PgrRam {
    pub const BLOCK_SIZE: usize = 1024 * 8;

    pub struct Size(pub u8);

    impl Size {
        pub fn in_blocks(&self) -> u8 {
            self.0
        }

        pub fn in_bytes(&self) -> usize {
            (self.in_blocks() as usize) * BLOCK_SIZE
        }
    }

    pub type Block = [u8; BLOCK_SIZE];
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

pub struct Header([u8; 16]);

pub struct Trainer([u8; 512]);

impl Header {
    pub fn new(data: [u8; 16]) -> Result<Self, CartridgeError> {
        if &data[0..=3] != &['N' as u8, 'E' as u8, 'S' as u8, 0x1a] {
            return Err(CartridgeError::BadHeader);
        }

        if data[PRG_ROM_INDEX] == 0 {
            return Err(CartridgeError::MissingPRGROM);
        }

        Ok(Self(data))
    }

    pub fn prg_rom_size(&self) -> PrgRom::Size {
        PrgRom::Size(self.0[PRG_ROM_INDEX])
    }

    pub fn chr_rom_size(&self) -> ChrRom::Size {
        ChrRom::Size(self.0[5])
    }

    pub fn nametable_arrangement(&self) -> NametableArrangement {
        if (self.0[6] & 0b0000_0001) == 0 {
            NametableArrangement::Vertical
        } else {
            NametableArrangement::Horizontal
        }
    }

    pub fn has_battery_backed_prg_ram(&self) -> bool {
        (self.0[6] & 0b0000_0010) != 0
    }

    pub fn has_trainer(&self) -> bool {
        (self.0[6] & 0b0000_0100) != 0
    }

    pub fn has_alternative_nametable_layout(&self) -> bool {
        (self.0[6] & 0b0000_1000) != 0
    }

    pub fn mapper_number(&self) -> u8 {
        let low = (self.0[6] & 0b1111_0000) >> 4;
        let high = self.0[7] & 0b1111_0000;
        low | high
    }

    pub fn is_vs_unisystem(&self) -> bool {
        (self.0[7] & 0b0000_0001) != 0
    }

    pub fn is_playchoice_10(&self) -> bool {
        (self.0[7] & 0b0000_0010) != 0
    }

    /// flags from byte 8 through 15 are in NES 2.0 foramt
    pub fn is_nes_2_0(&self) -> bool {
        (self.0[7] & 0b0000_1100) == 0b0000_1100
    }

    pub fn prg_ram_size(&self) -> PgrRam::Size {
        let x = self.0[8];
        PgrRam::Size(if x == 0 { 1 } else { x })
    }

    pub fn tv_system(&self) -> TVSystem {
        match self.0[10] & 0b0000_0011 {
            0 => TVSystem::NTSC,
            2 => TVSystem::PAL,
            _ => TVSystem::Both,
        }
    }

    pub fn has_prg_ram(&self) -> bool {
        (self.0[10] & 0b0001_0000) == 0
    }

    pub fn has_bus_conflicts(&self) -> bool {
        (self.0[10] & 0b0010_0000) != 0
    }
}

pub struct Cartridge {
    header: Header,
    trainer: Option<Trainer>,
    prg_rom_data: Vec<PrgRom::Block>,
    chr_rom_data: Vec<ChrRom::Block>,
}

impl Cartridge {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, CartridgeError> {
        // let data = data.as_slice();

        let header = Header::new(
            data[0..16]
                .try_into()
                .map_err(|_| CartridgeError::BadHeader)?,
        )?;
        let mut remaining_data = &data[16..];

        let trainer = if header.has_trainer() {
            let len = 512;
            if remaining_data.len() < len {
                Err(CartridgeError::MissingTrainer)
            } else {
                let result = Trainer(
                    remaining_data[0..len]
                        .try_into()
                        .map_err(|_| CartridgeError::MissingTrainer)?,
                );
                remaining_data = &remaining_data[len..];
                Ok(Some(result))
            }
        } else {
            Ok(None)
        }?;

        let prg_rom_data = {
            let len = header.prg_rom_size().in_bytes();
            if remaining_data.len() < len {
                Err(CartridgeError::MissingPRGROM)
            } else {
                let size = header.prg_rom_size();
                let mut results = Vec::with_capacity(size.in_blocks() as usize);
                for _ in 0..size.in_blocks() {
                    results.push(
                        remaining_data[0..len]
                            .try_into()
                            .map_err(|_| CartridgeError::MissingPRGROM)?,
                    );
                    remaining_data = &remaining_data[len..];
                }
                Ok(results)
            }
        }?;

        let chr_rom_data = {
            let len = header.prg_rom_size().in_bytes();
            if remaining_data.len() < len {
                Err(CartridgeError::MissingPRGROM)
            } else {
                let size = header.chr_rom_size();
                let mut results = Vec::with_capacity(size.in_blocks() as usize);
                for _ in 0..size.in_blocks() {
                    results.push(
                        remaining_data[0..len]
                            .try_into()
                            .map_err(|_| CartridgeError::MissingPRGROM)?,
                    );
                    remaining_data = &remaining_data[len..];
                }
                Ok(results)
            }
        }?;

        /*
        TODO remaining data is playchoice stuff
            PlayChoice INST-ROM, if present (0 or 8192 bytes)
            PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing; see PC10 ROM-Images for details)
        */

        let result = Self {
            header,
            trainer,
            prg_rom_data,
            chr_rom_data,
        };

        Ok(result)
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn pgr_rom(&self) -> &[PrgRom::Block] {
        self.prg_rom_data.as_slice()
    }

    pub fn chr_rom(&self) -> &[ChrRom::Block] {
        self.chr_rom_data.as_slice()
    }
}

// see "INES - NESdev Wiki.html"

use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum CartridgeError {
    BadHeader,
    UnrecognizedMemoryMapper(u8),
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

pub mod pgr_rom {
    pub const BLOCK_SIZE: usize = 1024 * 16;

    #[derive(Debug, Clone)]
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

pub mod chr_rom {
    pub const BLOCK_SIZE: usize = 1024 * 8;

    #[derive(Debug, Clone)]
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

pub mod pgr_ram {
    pub const BLOCK_SIZE: usize = 1024 * 8;

    #[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum NametableArrangement {
    Vertical,
    Horizontal,
    SingleScreenMirroring,
    FourScreenMirroring,
}

#[derive(Debug, Clone, Copy)]
pub enum TVSystem {
    NTSC,
    PAL,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryMapper {
    NROM,
}

const PRG_ROM_INDEX: usize = 4;

pub struct Header {
    data: [u8; 16],
    mapper: MemoryMapper,
}

pub struct Trainer([u8; 512]);

impl Header {
    pub fn new(data: [u8; 16]) -> Result<Self, CartridgeError> {
        if &data[0..=3] != &['N' as u8, 'E' as u8, 'S' as u8, 0x1a] {
            return Err(CartridgeError::BadHeader);
        }

        if data[PRG_ROM_INDEX] == 0 {
            return Err(CartridgeError::MissingPRGROM);
        }

        let mapper = {
            let low = (data[6] & 0b1111_0000) >> 4;
            let high = data[7] & 0b1111_0000;
            let mapper = low | high;
            match mapper {
                0 => MemoryMapper::NROM,
                _ => Err(CartridgeError::UnrecognizedMemoryMapper(mapper))?,
            }
        };

        Ok(Self { data, mapper })
    }

    pub fn prg_rom_size(&self) -> pgr_rom::Size {
        pgr_rom::Size(self.data[PRG_ROM_INDEX])
    }

    pub fn chr_rom_size(&self) -> chr_rom::Size {
        chr_rom::Size(self.data[5])
    }

    pub fn nametable_arrangement(&self) -> NametableArrangement {
        let alt_layout = (self.data[6] & 0b0000_1000) != 0;
        let layout = (self.data[6] & 0b0000_0001) != 0;
        match (self.memory_mapper(), layout, alt_layout) {
            (MemoryMapper::NROM, _, _) => NametableArrangement::Vertical,
        }
    }

    pub fn has_battery_backed_prg_ram(&self) -> bool {
        (self.data[6] & 0b0000_0010) != 0
    }

    pub fn has_trainer(&self) -> bool {
        (self.data[6] & 0b0000_0100) != 0
    }

    pub fn memory_mapper(&self) -> MemoryMapper {
        self.mapper
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

    pub fn prg_ram_size(&self) -> pgr_ram::Size {
        let x = self.data[8];
        pgr_ram::Size(if x == 0 { 1 } else { x })
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

pub struct Cartridge {
    header: Header,
    trainer: Option<Trainer>,
    prg_rom_data: Vec<pgr_rom::Block>,
    chr_rom_data: Vec<chr_rom::Block>,
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
            let size = header.prg_rom_size();
            if remaining_data.len() < size.in_bytes() {
                Err(CartridgeError::MissingPRGROM)
            } else {
                let mut results = Vec::with_capacity(size.in_blocks() as usize);
                for _ in 0..size.in_blocks() {
                    const LEN: usize = pgr_rom::BLOCK_SIZE;
                    results.push(
                        remaining_data[0..LEN]
                            .try_into()
                            .map_err(|_| CartridgeError::MissingPRGROM)?,
                    );
                    remaining_data = &remaining_data[LEN..];
                }
                Ok(results)
            }
        }?;

        let chr_rom_data = {
            let size = header.chr_rom_size();
            if remaining_data.len() < size.in_bytes() {
                Err(CartridgeError::MissingCHRROM)
            } else {
                let size = header.chr_rom_size();
                let mut results = Vec::with_capacity(size.in_blocks() as usize);
                for _ in 0..size.in_blocks() {
                    const LEN: usize = chr_rom::BLOCK_SIZE;
                    results.push(
                        remaining_data[0..LEN]
                            .try_into()
                            .map_err(|_| CartridgeError::MissingCHRROM)?,
                    );
                    remaining_data = &remaining_data[LEN..];
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

    pub fn pgr_rom(&self) -> &[pgr_rom::Block] {
        self.prg_rom_data.as_slice()
    }

    pub fn chr_rom(&self) -> &[chr_rom::Block] {
        self.chr_rom_data.as_slice()
    }
}

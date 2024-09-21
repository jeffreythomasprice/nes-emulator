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
    /// NROM, no mapper
    NROMNoMapper,
    /// Nintendo MMC1
    NintendoMMC1,
    /// UNROM switch
    UNROMSwitch,
    /// CNROM switch
    CNROMSwitch,
    /// Nintendo MMC3
    NintendoMMC3,
    /// Nintendo MMC5
    NintendoMMC5,
    /// FFE F4xxx
    FFEF4xxx,
    /// AOROM switch
    AOROMSwitch,
    /// FFE F3xxx
    FFEF3xxx,
    /// Nintendo MMC2
    NintendoMMC2,
    /// Nintendo MMC4
    NintendoMMC4,
    /// ColorDreams chip
    ColorDreamsChip,
    /// FFE F6xxx
    FFEF6xxx,
    /// 100-in-1 switch
    Switch100In1,
    /// Bandai chip
    BandaiChip,
    /// FFE F8xxx
    FFEF8xxx,
    /// Jaleco SS8806 chip
    JalecoSS8806Chip,
    /// Namcot 106 chip
    Namcot106Chip,
    /// Nintendo DiskSystem
    NintendoDiskSystem,
    /// Konami VRC4a
    KonamiVRC4a,
    /// Konami VRC2a
    KonamiVRC2a,
    /// Konami VRC6
    KonamiVRC6,
    /// Konami VRC4b
    KonamiVRC4b,
    /// Irem G-101 chip
    IremG101Chip,
    /// Taito TC0190/TC0350
    TaitoTC0190TC0350,
    /// 32 KB ROM switch
    SwitchROM32KB,
    /// Tengen RAMBO-1 chip
    TengenRAMBO1Chip,
    /// Irem H-3001 chip
    IremH3001Chip,
    /// GNROM switch
    GNROMSwitch,
    /// SunSoft3 chip
    SunSoft3Chip,
    /// SunSoft4 chip
    SunSoft4Chip,
    /// SunSoft5 FME-7 chip
    SunSoft5FME7Chip,
    /// Camerica chip
    CamericaChip,
    /// Irem 74HC161/32-based
    Irem74HC161_32Based,
    /// Pirate HK-SF3 chip
    PirateHKSF3Chip,
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
                0 => MemoryMapper::NROMNoMapper,
                1 => MemoryMapper::NintendoMMC1,
                2 => MemoryMapper::UNROMSwitch,
                3 => MemoryMapper::CNROMSwitch,
                4 => MemoryMapper::NintendoMMC3,
                5 => MemoryMapper::NintendoMMC5,
                6 => MemoryMapper::FFEF4xxx,
                7 => MemoryMapper::AOROMSwitch,
                8 => MemoryMapper::FFEF3xxx,
                9 => MemoryMapper::NintendoMMC2,
                10 => MemoryMapper::NintendoMMC4,
                11 => MemoryMapper::ColorDreamsChip,
                12 => MemoryMapper::FFEF6xxx,
                15 => MemoryMapper::Switch100In1,
                16 => MemoryMapper::BandaiChip,
                17 => MemoryMapper::FFEF8xxx,
                18 => MemoryMapper::JalecoSS8806Chip,
                19 => MemoryMapper::Namcot106Chip,
                20 => MemoryMapper::NintendoDiskSystem,
                21 => MemoryMapper::KonamiVRC4a,
                22 | 23 => MemoryMapper::KonamiVRC2a,
                24 => MemoryMapper::KonamiVRC6,
                25 => MemoryMapper::KonamiVRC4b,
                32 => MemoryMapper::IremG101Chip,
                33 => MemoryMapper::TaitoTC0190TC0350,
                34 => MemoryMapper::SwitchROM32KB,
                64 => MemoryMapper::TengenRAMBO1Chip,
                65 => MemoryMapper::IremH3001Chip,
                66 => MemoryMapper::GNROMSwitch,
                67 => MemoryMapper::SunSoft3Chip,
                68 => MemoryMapper::SunSoft4Chip,
                69 => MemoryMapper::SunSoft5FME7Chip,
                71 => MemoryMapper::CamericaChip,
                78 => MemoryMapper::Irem74HC161_32Based,
                91 => MemoryMapper::PirateHKSF3Chip,
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
        if (self.data[6] & 0b0000_1000) != 0 {
            NametableArrangement::FourScreenMirroring
        } else {
            /*
            sources differ
            INES - NESdev Wiki.html
                0 = vertical
                1 = horizontal
            NESDoc.pdf
                0 = horizontal
                1 = vertical
            */
            if (self.data[6] & 0b0000_0001) == 0 {
                NametableArrangement::Vertical
            } else {
                NametableArrangement::Horizontal
            }
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

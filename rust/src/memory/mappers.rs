use crate::cartridge_file::{self, pgr_rom, Cartridge};

pub trait MemoryMapper {
    /// Reads from whatever bank is set as the lower bank.
    /// Address will be restricted to 0..PRG_BANK_SIZE. So only the low 14 bits will be set; the high 2 bits will be 0.
    fn read8_pgr_lower_bank(&self, address: u16) -> u8;

    /// As lower bank, but for whatever bank is set on the upper bank.
    fn read8_pgr_upper_bank(&self, address: u16) -> u8;

    /// Since we're talking about program ROM we don't really write here, but the mappers all use this address space for memory mapper
    /// registers. Since they all behave differently and don't really have a banking mode just expose the whole range.
    /// Address will be restricted to 0..(PRG_BANK_SIZE*2). So only the low 15 bits will be set; the high bit will be 0.
    fn write8_pgr(&mut self, address: u16, value: u8);

    // TODO chr read and write
}

pub struct NROMNoMapper {
    lower: pgr_rom::Block,
    upper: pgr_rom::Block,
}

impl NROMNoMapper {
    pub fn new(cartridge: &Cartridge) -> Self {
        let size = cartridge.header().prg_rom_size().in_blocks() as usize;
        let lower = cartridge.pgr_rom()[0];
        let upper = cartridge.pgr_rom()[size - 1];
        Self { lower, upper }
    }
}

impl MemoryMapper for NROMNoMapper {
    fn read8_pgr_lower_bank(&self, address: u16) -> u8 {
        self.lower[address as usize]
    }

    fn read8_pgr_upper_bank(&self, address: u16) -> u8 {
        self.upper[address as usize]
    }

    fn write8_pgr(&mut self, _address: u16, _value: u8) {
        // mapper doesn't have any input registers
    }
}

pub fn new_memory_mapper(cartridge: &Cartridge) -> Box<dyn MemoryMapper> {
    Box::new(match cartridge.header().memory_mapper() {
        cartridge_file::MemoryMapper::NROMNoMapper => NROMNoMapper::new(cartridge),
        cartridge_file::MemoryMapper::NintendoMMC1 => todo!(),
        cartridge_file::MemoryMapper::UNROMSwitch => todo!(),
        cartridge_file::MemoryMapper::CNROMSwitch => todo!(),
        cartridge_file::MemoryMapper::NintendoMMC3 => todo!(),
        cartridge_file::MemoryMapper::NintendoMMC5 => todo!(),
        cartridge_file::MemoryMapper::FFEF4xxx => todo!(),
        cartridge_file::MemoryMapper::AOROMSwitch => todo!(),
        cartridge_file::MemoryMapper::FFEF3xxx => todo!(),
        cartridge_file::MemoryMapper::NintendoMMC2 => todo!(),
        cartridge_file::MemoryMapper::NintendoMMC4 => todo!(),
        cartridge_file::MemoryMapper::ColorDreamsChip => todo!(),
        cartridge_file::MemoryMapper::FFEF6xxx => todo!(),
        cartridge_file::MemoryMapper::Switch100In1 => todo!(),
        cartridge_file::MemoryMapper::BandaiChip => todo!(),
        cartridge_file::MemoryMapper::FFEF8xxx => todo!(),
        cartridge_file::MemoryMapper::JalecoSS8806Chip => todo!(),
        cartridge_file::MemoryMapper::Namcot106Chip => todo!(),
        cartridge_file::MemoryMapper::NintendoDiskSystem => todo!(),
        cartridge_file::MemoryMapper::KonamiVRC4a => todo!(),
        cartridge_file::MemoryMapper::KonamiVRC2a => todo!(),
        cartridge_file::MemoryMapper::KonamiVRC6 => todo!(),
        cartridge_file::MemoryMapper::KonamiVRC4b => todo!(),
        cartridge_file::MemoryMapper::IremG101Chip => todo!(),
        cartridge_file::MemoryMapper::TaitoTC0190TC0350 => todo!(),
        cartridge_file::MemoryMapper::SwitchROM32KB => todo!(),
        cartridge_file::MemoryMapper::TengenRAMBO1Chip => todo!(),
        cartridge_file::MemoryMapper::IremH3001Chip => todo!(),
        cartridge_file::MemoryMapper::GNROMSwitch => todo!(),
        cartridge_file::MemoryMapper::SunSoft3Chip => todo!(),
        cartridge_file::MemoryMapper::SunSoft4Chip => todo!(),
        cartridge_file::MemoryMapper::SunSoft5FME7Chip => todo!(),
        cartridge_file::MemoryMapper::CamericaChip => todo!(),
        cartridge_file::MemoryMapper::Irem74HC161_32Based => todo!(),
        cartridge_file::MemoryMapper::PirateHKSF3Chip => todo!(),
    })
}

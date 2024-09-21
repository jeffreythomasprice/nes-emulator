use crate::{
    cartridge_file::{pgr_rom, Cartridge, MemoryMapper},
    endians::Word,
};

const ZERO_PAGE_SIZE: u16 = 0x0100;
const STACK_SIZE: u16 = 0x0100;
const RAM_SIZE: u16 = 0x0600;
const TOTAL_RAM_SIZE: u16 = ZERO_PAGE_SIZE + STACK_SIZE + RAM_SIZE;

const RAM_MIRROR_END: u16 = 0x2000;

const IO_REGISTER_LOWER_START: u16 = 0x2000;
const IO_REGISTER_LOWER_END: u16 = 0x2008;
const IO_REGISTER_MIRROR_END: u16 = 0x4000;
const IO_REGISTER_UPPER_END: u16 = 0x4020;

const EXPANSION_ROM_END: u16 = 0x6000;

const SRAM_START: u16 = EXPANSION_ROM_END;
const SRAM_END: u16 = 0x8000;
const SRAM_SIZE: u16 = SRAM_END - SRAM_START;

const PRG_BANK_SIZE: u16 = 1024 * 16;
const PRG_LOWER_BANK_START: u16 = SRAM_END;
const PRG_LOWER_BANK_END: u16 = PRG_LOWER_BANK_START + PRG_BANK_SIZE;

pub trait Memory {
    fn read8(&self, address: u16) -> u8;
    fn write8(&mut self, address: u16, value: u8);

    fn read16(&self, address: u16) -> u16 {
        let low = self.read8(address);
        let high = self.read8(address.wrapping_add(1));
        Word { low, high }.into()
    }

    fn write16(&mut self, address: u16, value: u16) {
        let value: Word = value.into();
        self.write8(address, value.low);
        self.write8(address.wrapping_add(1), value.high);
    }
}

pub trait PgrMemoryMapper {
    /// Reads from whatever bank is set as the lower bank.
    /// Address will be restricted to 0..PRG_BANK_SIZE. So only the low 14 bits will be set; the high 2 bits will be 0.
    fn read8_pgr_lower_bank(&self, address: u16) -> u8;

    /// As lower bank, but for whatever bank is set on the upper bank.
    fn read8_pgr_upper_bank(&self, address: u16) -> u8;

    /// Since we're talking about program ROM we don't really write here, but the mappers all use this address space for memory mapper
    /// registers. Since they all behave differently and don't really have a banking mode just expose the whole range.
    /// Address will be restricted to 0..(PRG_BANK_SIZE*2). So only the low 15 bits will be set; the high bit will be 0.
    fn write8_pgr(&mut self, address: u16, value: u8);
}

struct NROMNoMapper {
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

impl PgrMemoryMapper for NROMNoMapper {
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

pub struct MainMemory {
    ram: [u8; TOTAL_RAM_SIZE as usize],
    sram: [u8; SRAM_SIZE as usize],
    mapper: Box<dyn PgrMemoryMapper>,
}

impl MainMemory {
    pub fn new(mapper: Box<dyn PgrMemoryMapper>) -> Self {
        Self {
            ram: [0; TOTAL_RAM_SIZE as usize],
            sram: [0; SRAM_SIZE as usize],
            mapper,
        }
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let mapper = Box::new(match cartridge.header().memory_mapper() {
            MemoryMapper::NROMNoMapper => NROMNoMapper::new(cartridge),
            MemoryMapper::NintendoMMC1 => todo!(),
            MemoryMapper::UNROMSwitch => todo!(),
            MemoryMapper::CNROMSwitch => todo!(),
            MemoryMapper::NintendoMMC3 => todo!(),
            MemoryMapper::NintendoMMC5 => todo!(),
            MemoryMapper::FFEF4xxx => todo!(),
            MemoryMapper::AOROMSwitch => todo!(),
            MemoryMapper::FFEF3xxx => todo!(),
            MemoryMapper::NintendoMMC2 => todo!(),
            MemoryMapper::NintendoMMC4 => todo!(),
            MemoryMapper::ColorDreamsChip => todo!(),
            MemoryMapper::FFEF6xxx => todo!(),
            MemoryMapper::Switch100In1 => todo!(),
            MemoryMapper::BandaiChip => todo!(),
            MemoryMapper::FFEF8xxx => todo!(),
            MemoryMapper::JalecoSS8806Chip => todo!(),
            MemoryMapper::Namcot106Chip => todo!(),
            MemoryMapper::NintendoDiskSystem => todo!(),
            MemoryMapper::KonamiVRC4a => todo!(),
            MemoryMapper::KonamiVRC2a => todo!(),
            MemoryMapper::KonamiVRC6 => todo!(),
            MemoryMapper::KonamiVRC4b => todo!(),
            MemoryMapper::IremG101Chip => todo!(),
            MemoryMapper::TaitoTC0190TC0350 => todo!(),
            MemoryMapper::SwitchROM32KB => todo!(),
            MemoryMapper::TengenRAMBO1Chip => todo!(),
            MemoryMapper::IremH3001Chip => todo!(),
            MemoryMapper::GNROMSwitch => todo!(),
            MemoryMapper::SunSoft3Chip => todo!(),
            MemoryMapper::SunSoft4Chip => todo!(),
            MemoryMapper::SunSoft5FME7Chip => todo!(),
            MemoryMapper::CamericaChip => todo!(),
            MemoryMapper::Irem74HC161_32Based => todo!(),
            MemoryMapper::PirateHKSF3Chip => todo!(),
        });
        Self::new(mapper)
    }
}

impl Memory for MainMemory {
    fn read8(&self, address: u16) -> u8 {
        match address {
            // zero page, stack, ram
            ..TOTAL_RAM_SIZE => self.ram[address as usize],
            // mirrors ram
            ..RAM_MIRROR_END => self.read8(address % TOTAL_RAM_SIZE),
            // io registers
            ..IO_REGISTER_LOWER_END => todo!(),
            // mirrors io registers
            ..IO_REGISTER_MIRROR_END => self.read8(IO_REGISTER_LOWER_START + (address & 8)),
            // io registers
            ..IO_REGISTER_UPPER_END => todo!(),
            // expansion rom
            ..EXPANSION_ROM_END => todo!(),
            // sram = persistent ram for save games
            ..SRAM_END => self.sram[(address - SRAM_START) as usize],
            // prg rom lower bank
            ..PRG_LOWER_BANK_END => self
                .mapper
                .read8_pgr_lower_bank(address & 0b0011_1111_1111_1111),
            // prg rom upper bank
            _ => self
                .mapper
                .read8_pgr_upper_bank(address & 0b0011_1111_1111_1111),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            // zero page, stack, ram
            ..TOTAL_RAM_SIZE => self.ram[address as usize] = value,
            // mirrors ram
            ..RAM_MIRROR_END => self.write8(address % TOTAL_RAM_SIZE, value),
            // io registers
            ..IO_REGISTER_LOWER_END => todo!(),
            // mirrors io registers
            ..IO_REGISTER_MIRROR_END => self.write8(IO_REGISTER_LOWER_START + (address & 8), value),
            // io registers
            ..IO_REGISTER_UPPER_END => todo!(),
            // expansion rom
            ..EXPANSION_ROM_END => todo!(),
            // sram = persistent ram for save games
            ..SRAM_END => self.sram[(address - SRAM_START) as usize] = value,
            // prg rom lower and upper banks
            _ => self
                .mapper
                .write8_pgr(address & 0b0111_1111_1111_1111, value),
        }
    }
}

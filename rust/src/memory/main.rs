use crate::cartridge_file::pgr_rom;

use super::main_mapper::MainMemoryMapper;

const ZERO_PAGE_SIZE: u16 = 0x0100;
const STACK_SIZE: u16 = 0x0100;
const RAM_SIZE: u16 = 0x0600;
const TOTAL_RAM_SIZE: u16 = ZERO_PAGE_SIZE + STACK_SIZE + RAM_SIZE;

const RAM_MIRROR_END: u16 = 0x2000;

const IO_REGISTER_LOWER_START: u16 = 0x2000;
const IO_REGISTER_LOWER_END: u16 = 0x2008;
const IO_REGISTER_LOWER_SIZE: u16 = IO_REGISTER_LOWER_END - IO_REGISTER_LOWER_START;
const IO_REGISTER_MIRROR_END: u16 = 0x4000;
const IO_REGISTER_UPPER_END: u16 = 0x4020;

const EXPANSION_ROM_END: u16 = 0x6000;

const SRAM_START: u16 = EXPANSION_ROM_END;
const SRAM_END: u16 = 0x8000;
const SRAM_SIZE: u16 = SRAM_END - SRAM_START;

pub const PRG_BANK_SIZE: u16 = pgr_rom::BLOCK_SIZE as u16;
const PRG_LOWER_BANK_START: u16 = SRAM_END;
const PRG_LOWER_BANK_END: u16 = PRG_LOWER_BANK_START + PRG_BANK_SIZE;

pub struct Memory {
    ram: [u8; TOTAL_RAM_SIZE as usize],
    sram: [u8; SRAM_SIZE as usize],
    mapper: Box<dyn MainMemoryMapper>,
}

impl Memory {
    pub fn new(mapper: Box<dyn MainMemoryMapper>) -> Self {
        Self {
            ram: [0; TOTAL_RAM_SIZE as usize],
            sram: [0; SRAM_SIZE as usize],
            mapper,
        }
    }
}

impl super::Memory for Memory {
    fn read8(&self, address: u16) -> u8 {
        match address {
            // zero page, stack, ram
            ..TOTAL_RAM_SIZE => self.ram[address as usize],
            // mirrors ram
            ..RAM_MIRROR_END => self.read8(address % TOTAL_RAM_SIZE),
            // io registers
            ..IO_REGISTER_LOWER_END => todo!(),
            // mirrors io registers
            ..IO_REGISTER_MIRROR_END => self.read8(
                (address - IO_REGISTER_LOWER_START) % IO_REGISTER_LOWER_SIZE
                    + IO_REGISTER_LOWER_START,
            ),
            // io registers
            ..IO_REGISTER_UPPER_END => todo!(),
            // expansion rom
            ..EXPANSION_ROM_END => todo!(),
            // sram = persistent ram for save games
            ..SRAM_END => self.sram[(address - SRAM_START) as usize],
            // prg rom lower bank
            ..PRG_LOWER_BANK_END => self
                .mapper
                .read8_main_lower_bank(address & 0b0011_1111_1111_1111),
            // prg rom upper bank
            _ => self
                .mapper
                .read8_main_upper_bank(address & 0b0011_1111_1111_1111),
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
            ..IO_REGISTER_MIRROR_END => self.write8(
                (address - IO_REGISTER_LOWER_START) % IO_REGISTER_LOWER_SIZE
                    + IO_REGISTER_LOWER_START,
                value,
            ),
            // io registers
            ..IO_REGISTER_UPPER_END => todo!(),
            // expansion rom
            ..EXPANSION_ROM_END => todo!(),
            // sram = persistent ram for save games
            ..SRAM_END => self.sram[(address - SRAM_START) as usize] = value,
            // prg rom lower and upper banks
            _ => self
                .mapper
                .write8_main(address & 0b0111_1111_1111_1111, value),
        }
    }
}

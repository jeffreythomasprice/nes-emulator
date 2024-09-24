use crate::{
    cartridge_file::{chr_rom, pgr_rom, Cartridge},
    memory::{main_mapper::MainMemoryMapper, pattern_tables_mapper::PatternTableMemoryMapper},
};

pub struct Main {
    main_lower: pgr_rom::Block,
    main_upper: pgr_rom::Block,
}

impl Main {
    pub fn new(cartridge: &Cartridge) -> Self {
        let size = cartridge.header().prg_rom_size().in_blocks() as usize;
        let lower = cartridge.pgr_rom()[0];
        let upper = cartridge.pgr_rom()[size - 1];
        Self {
            main_lower: lower,
            main_upper: upper,
        }
    }
}

impl MainMemoryMapper for Main {
    fn read8_main_lower_bank(&self, address: u16) -> u8 {
        self.main_lower[address as usize]
    }

    fn read8_main_upper_bank(&self, address: u16) -> u8 {
        self.main_upper[address as usize]
    }

    fn write8_main(&mut self, _address: u16, _value: u8) {
        // mapper doesn't have any input registers
    }
}

pub struct PatternTable {
    pattern_table_0: chr_rom::Block,
    pattern_table_1: Option<chr_rom::Block>,
}

impl PatternTable {
    pub fn new(cartridge: &Cartridge) -> Self {
        Self {
            pattern_table_0: [0; chr_rom::BLOCK_SIZE],
            pattern_table_1: if cartridge.header().chr_rom_size().in_blocks() >= 2 {
                Some([0; chr_rom::BLOCK_SIZE])
            } else {
                None
            },
        }
    }
}

impl PatternTableMemoryMapper for PatternTable {
    fn read8_pattern_table_0(&self, address: u16) -> u8 {
        self.pattern_table_0[address as usize]
    }

    fn write8_pattern_table_0(&mut self, address: u16, value: u8) {
        self.pattern_table_0[address as usize] = value;
    }

    fn read8_pattern_table_1(&self, address: u16) -> u8 {
        (match self.pattern_table_1 {
            Some(x) => x,
            None => self.pattern_table_0,
        })[address as usize]
    }

    fn write8_pattern_table_1(&mut self, address: u16, value: u8) {
        (match self.pattern_table_1 {
            Some(x) => x,
            None => self.pattern_table_0,
        })[address as usize] = value
    }
}

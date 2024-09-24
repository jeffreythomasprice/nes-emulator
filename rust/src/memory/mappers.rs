use crate::cartridge_file::{self, chr_rom, pgr_rom, Cartridge, NametableArrangement};

use super::video_memory;

pub trait MainMemoryMapper {
    /// Reads from whatever bank is set as the lower bank.
    /// Address will be already adjusted to be in 0..PRG_BANK_SIZE.
    fn read8_main_lower_bank(&self, address: u16) -> u8;

    /// As lower bank, but for whatever bank is set on the upper bank.
    fn read8_main_upper_bank(&self, address: u16) -> u8;

    /// Since we're talking about program ROM we don't really write here, but the mappers all use this address space for memory mapper
    /// registers. Since they all behave differently and don't really have a banking mode just expose the whole range.
    /// Address will be already adjusted to be in 0..(PRG_BANK_SIZE*2).
    fn write8_main(&mut self, address: u16, value: u8);
}

pub trait PatternTableMemoryMapper {
    /// Reads from pattern table 0.
    /// Address will be already adjusted to be in 0..PATTERN_TABLE_SIZE.
    fn read8_pattern_table_0(&self, address: u16) -> u8;

    /// As read.
    fn write8_pattern_table_0(&mut self, address: u16, value: u8);

    /// Reads from pattern table 0.
    /// Address will be already adjusted to be in 0..PATTERN_TABLE_SIZE.
    fn read8_pattern_table_1(&self, address: u16) -> u8;

    /// As read.
    fn write8_pattern_table_1(&mut self, address: u16, value: u8);
}

pub trait NameAndAttributeTablesMemoryMapper {
    /// Reads from name table 0.
    /// Address will be already adjusted to be in 0..NAME_TABLE_SIZE.
    fn read8_name_table_0(&self, address: u16) -> u8;

    /// As read.
    fn write8_name_table_0(&mut self, address: u16, value: u8);

    /// Reads from attribute table 0.
    /// Address will be already adjusted to be in 0..ATTRIBUTE_TABLE_SIZE.
    fn read8_attribute_table_0(&self, address: u16) -> u8;

    /// As read.
    fn write8_attribute_table_0(&mut self, address: u16, value: u8);

    /// Reads from name table 0.
    /// Address will be already adjusted to be in 0..NAME_TABLE_SIZE.
    fn read8_name_table_1(&self, address: u16) -> u8;

    /// As read.
    fn write8_name_table_1(&mut self, address: u16, value: u8);

    /// Reads from attribute table 0.
    /// Address will be already adjusted to be in 0..ATTRIBUTE_TABLE_SIZE.
    fn read8_attribute_table_1(&self, address: u16) -> u8;

    /// As read.
    fn write8_attribute_table_1(&mut self, address: u16, value: u8);

    /// Reads from name table 0.
    /// Address will be already adjusted to be in 0..NAME_TABLE_SIZE.
    fn read8_name_table_2(&self, address: u16) -> u8;

    /// As read.
    fn write8_name_table_2(&mut self, address: u16, value: u8);

    /// Reads from attribute table 0.
    /// Address will be already adjusted to be in 0..ATTRIBUTE_TABLE_SIZE.
    fn read8_attribute_table_2(&self, address: u16) -> u8;

    /// As read.
    fn write8_attribute_table_2(&mut self, address: u16, value: u8);

    /// Reads from name table 0.
    /// Address will be already adjusted to be in 0..NAME_TABLE_SIZE.
    fn read8_name_table_3(&self, address: u16) -> u8;

    /// As read.
    fn write8_name_table_3(&mut self, address: u16, value: u8);

    /// Reads from attribute table 0.
    /// Address will be already adjusted to be in 0..ATTRIBUTE_TABLE_SIZE.
    fn read8_attribute_table_3(&self, address: u16) -> u8;

    /// As read.
    fn write8_attribute_table_3(&mut self, address: u16, value: u8);
}

pub struct NROMMain {
    main_lower: pgr_rom::Block,
    main_upper: pgr_rom::Block,
}

impl NROMMain {
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

impl MainMemoryMapper for NROMMain {
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

pub struct HorizontalMirroringNameAndAttributeTable {
    name_table_0: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_1: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_1: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
}

impl HorizontalMirroringNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_1: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_1: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
        }
    }
}

impl NameAndAttributeTablesMemoryMapper for HorizontalMirroringNameAndAttributeTable {
    fn read8_name_table_0(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_0(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_0(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_0(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_1(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_1(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_1(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_1(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_2(&self, address: u16) -> u8 {
        self.name_table_1[address as usize]
    }

    fn write8_name_table_2(&mut self, address: u16, value: u8) {
        self.name_table_1[address as usize] = value;
    }

    fn read8_attribute_table_2(&self, address: u16) -> u8 {
        self.attribute_table_1[address as usize]
    }

    fn write8_attribute_table_2(&mut self, address: u16, value: u8) {
        self.attribute_table_1[address as usize] = value;
    }

    fn read8_name_table_3(&self, address: u16) -> u8 {
        self.name_table_1[address as usize]
    }

    fn write8_name_table_3(&mut self, address: u16, value: u8) {
        self.name_table_1[address as usize] = value;
    }

    fn read8_attribute_table_3(&self, address: u16) -> u8 {
        self.attribute_table_1[address as usize]
    }

    fn write8_attribute_table_3(&mut self, address: u16, value: u8) {
        self.attribute_table_1[address as usize] = value;
    }
}

pub struct VerticalMirroringNameAndAttributeTable {
    name_table_0: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_1: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_1: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
}

impl VerticalMirroringNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_1: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_1: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
        }
    }
}

impl NameAndAttributeTablesMemoryMapper for VerticalMirroringNameAndAttributeTable {
    fn read8_name_table_0(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_0(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_0(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_0(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_1(&self, address: u16) -> u8 {
        self.name_table_1[address as usize]
    }

    fn write8_name_table_1(&mut self, address: u16, value: u8) {
        self.name_table_1[address as usize] = value;
    }

    fn read8_attribute_table_1(&self, address: u16) -> u8 {
        self.attribute_table_1[address as usize]
    }

    fn write8_attribute_table_1(&mut self, address: u16, value: u8) {
        self.attribute_table_1[address as usize] = value;
    }

    fn read8_name_table_2(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_2(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_2(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_2(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_3(&self, address: u16) -> u8 {
        self.name_table_1[address as usize]
    }

    fn write8_name_table_3(&mut self, address: u16, value: u8) {
        self.name_table_1[address as usize] = value;
    }

    fn read8_attribute_table_3(&self, address: u16) -> u8 {
        self.attribute_table_1[address as usize]
    }

    fn write8_attribute_table_3(&mut self, address: u16, value: u8) {
        self.attribute_table_1[address as usize] = value;
    }
}

pub struct SingleNameAndAttributeTable {
    name_table_0: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
}

impl SingleNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
        }
    }
}

impl NameAndAttributeTablesMemoryMapper for SingleNameAndAttributeTable {
    fn read8_name_table_0(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_0(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_0(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_0(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_1(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_1(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_1(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_1(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_2(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_2(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_2(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_2(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_3(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_3(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_3(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_3(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }
}

pub struct FourWayMirroringNameAndAttributeTable {
    name_table_0: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_1: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_1: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_2: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_2: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_3: [u8; video_memory::NAME_TABLE_SIZE as usize],
    attribute_table_3: [u8; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
}

impl FourWayMirroringNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_1: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_1: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_2: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_2: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_3: [0; video_memory::NAME_TABLE_SIZE as usize],
            attribute_table_3: [0; video_memory::ATTRIBUTE_TABLE_SIZE as usize],
        }
    }
}

impl NameAndAttributeTablesMemoryMapper for FourWayMirroringNameAndAttributeTable {
    fn read8_name_table_0(&self, address: u16) -> u8 {
        self.name_table_0[address as usize]
    }

    fn write8_name_table_0(&mut self, address: u16, value: u8) {
        self.name_table_0[address as usize] = value;
    }

    fn read8_attribute_table_0(&self, address: u16) -> u8 {
        self.attribute_table_0[address as usize]
    }

    fn write8_attribute_table_0(&mut self, address: u16, value: u8) {
        self.attribute_table_0[address as usize] = value;
    }

    fn read8_name_table_1(&self, address: u16) -> u8 {
        self.name_table_1[address as usize]
    }

    fn write8_name_table_1(&mut self, address: u16, value: u8) {
        self.name_table_1[address as usize] = value;
    }

    fn read8_attribute_table_1(&self, address: u16) -> u8 {
        self.attribute_table_1[address as usize]
    }

    fn write8_attribute_table_1(&mut self, address: u16, value: u8) {
        self.attribute_table_1[address as usize] = value;
    }

    fn read8_name_table_2(&self, address: u16) -> u8 {
        self.name_table_2[address as usize]
    }

    fn write8_name_table_2(&mut self, address: u16, value: u8) {
        self.name_table_2[address as usize] = value;
    }

    fn read8_attribute_table_2(&self, address: u16) -> u8 {
        self.attribute_table_2[address as usize]
    }

    fn write8_attribute_table_2(&mut self, address: u16, value: u8) {
        self.attribute_table_2[address as usize] = value;
    }

    fn read8_name_table_3(&self, address: u16) -> u8 {
        self.name_table_3[address as usize]
    }

    fn write8_name_table_3(&mut self, address: u16, value: u8) {
        self.name_table_3[address as usize] = value;
    }

    fn read8_attribute_table_3(&self, address: u16) -> u8 {
        self.attribute_table_3[address as usize]
    }

    fn write8_attribute_table_3(&mut self, address: u16, value: u8) {
        self.attribute_table_3[address as usize] = value;
    }
}

pub struct NROMVideo {
    pattern_table_0: chr_rom::Block,
    pattern_table_1: Option<chr_rom::Block>,
}

impl NROMVideo {
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

impl PatternTableMemoryMapper for NROMVideo {
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

pub fn new_memory_mapper(
    cartridge: &Cartridge,
) -> (
    Box<dyn MainMemoryMapper>,
    Box<dyn PatternTableMemoryMapper>,
    Box<dyn NameAndAttributeTablesMemoryMapper>,
) {
    let name_and_attributes: Box<dyn NameAndAttributeTablesMemoryMapper> = match cartridge
        .header()
        .nametable_arrangement()
    {
        NametableArrangement::Vertical => Box::new(VerticalMirroringNameAndAttributeTable::new()),
        NametableArrangement::Horizontal => {
            Box::new(HorizontalMirroringNameAndAttributeTable::new())
        }
        NametableArrangement::SingleScreenMirroring => Box::new(SingleNameAndAttributeTable::new()),
        NametableArrangement::FourScreenMirroring => {
            Box::new(FourWayMirroringNameAndAttributeTable::new())
        }
    };

    let (main, pattern_table) = match cartridge.header().memory_mapper() {
        cartridge_file::MemoryMapper::NROM => (
            Box::new(NROMMain::new(cartridge)),
            Box::new(NROMVideo::new(cartridge)),
        ),
    };

    (main, pattern_table, name_and_attributes)
}

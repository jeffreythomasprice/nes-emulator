pub mod main;
pub mod main_mapper;
pub mod mappers;
pub mod name_attr_tables_mapper;
pub mod pattern_tables_mapper;
pub mod video;

use main_mapper::MainMemoryMapper;
use name_attr_tables_mapper::{
    FourWayMirroringNameAndAttributeTable, HorizontalMirroringNameAndAttributeTable,
    NameAndAttributeTablesMemoryMapper, SingleNameAndAttributeTable,
    VerticalMirroringNameAndAttributeTable,
};
use pattern_tables_mapper::PatternTableMemoryMapper;

use crate::{
    cartridge_file::{Cartridge, NametableArrangement},
    endians::Word,
};

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

pub fn new(cartridge: &Cartridge) -> (main::Memory, video::Memory) {
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

    let (main, pattern_table) = mappers::new(cartridge);

    (
        main::Memory::new(main),
        video::Memory::new(pattern_table, name_and_attributes),
    )
}

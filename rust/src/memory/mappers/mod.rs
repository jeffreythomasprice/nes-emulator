use crate::cartridge_file::{self, Cartridge};

use super::{main_mapper::MainMemoryMapper, pattern_tables_mapper::PatternTableMemoryMapper};

mod nrom;

pub fn new(
    cartridge: &Cartridge,
) -> (Box<dyn MainMemoryMapper>, Box<dyn PatternTableMemoryMapper>) {
    match cartridge.header().memory_mapper() {
        cartridge_file::MemoryMapper::NROM => (
            Box::new(nrom::Main::new(cartridge)),
            Box::new(nrom::PatternTable::new(cartridge)),
        ),
    }
}

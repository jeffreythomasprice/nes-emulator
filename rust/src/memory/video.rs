use super::{
    name_attr_tables_mapper::NameAndAttributeTablesMemoryMapper,
    pattern_tables_mapper::PatternTableMemoryMapper,
};

const PATTERN_TABLE_0_START: u16 = 0x0000;
const PATTERN_TABLE_0_END: u16 = 0x1000;
const PATTERN_TABLE_1_START: u16 = PATTERN_TABLE_0_END;
const PATTERN_TABLE_1_END: u16 = 0x2000;

const PATTERN_TABLE_SIZE: u16 = PATTERN_TABLE_0_END;

const NAME_TABLE_0_START: u16 = PATTERN_TABLE_1_END;
const NAME_TABLE_0_END: u16 = 0x23c0;
const ATTRIBUTE_TABLE_0_START: u16 = NAME_TABLE_0_END;
const ATTRIBUTE_TABLE_0_END: u16 = 0x2400;
const NAME_TABLE_1_START: u16 = ATTRIBUTE_TABLE_0_END;
const NAME_TABLE_1_END: u16 = 0x27c0;
const ATTRIBUTE_TABLE_1_START: u16 = NAME_TABLE_1_END;
const ATTRIBUTE_TABLE_1_END: u16 = 0x2800;
const NAME_TABLE_2_START: u16 = ATTRIBUTE_TABLE_1_END;
const NAME_TABLE_2_END: u16 = 0x2bc0;
const ATTRIBUTE_TABLE_2_START: u16 = NAME_TABLE_2_END;
const ATTRIBUTE_TABLE_2_END: u16 = 0x2c00;
const NAME_TABLE_3_START: u16 = ATTRIBUTE_TABLE_2_END;
const NAME_TABLE_3_END: u16 = 0x2fc0;
const ATTRIBUTE_TABLE_3_START: u16 = NAME_TABLE_3_END;
const ATTRIBUTE_TABLE_3_END: u16 = 0x3000;

pub const NAME_TABLE_SIZE: u16 = NAME_TABLE_0_END - NAME_TABLE_0_START;
pub const ATTRIBUTE_TABLE_SIZE: u16 = ATTRIBUTE_TABLE_0_END - ATTRIBUTE_TABLE_0_START;
const NAME_AND_ATTRIBUTE_TABLES_TOTAL_SIZE: u16 = ATTRIBUTE_TABLE_3_END - NAME_TABLE_0_START;

const NAME_AND_ATTRIBUTE_TABLE_MIRRORS_START: u16 = ATTRIBUTE_TABLE_3_END;
const NAME_AND_ATTRIBUTE_TABLE_MIRRORS_END: u16 = 0x3f00;

const IMAGE_PALETTE_START: u16 = NAME_AND_ATTRIBUTE_TABLE_MIRRORS_END;
const IMAGE_PALETTE_END: u16 = 0x3f10;
const SPRITE_PALETTE_END: u16 = 0x3f20;
const IMAGE_AND_SPRITE_PALETTE_TOTAL_SIZE: u16 = SPRITE_PALETTE_END - IMAGE_PALETTE_START;

const IMAGE_AND_SPRITE_PALETTE_MIRRORS_START: u16 = SPRITE_PALETTE_END;
const IMAGE_AND_SPRITE_PALETTE_MIRRORS_END: u16 = 0x4000;

const MIRRORS_START: u16 = IMAGE_AND_SPRITE_PALETTE_MIRRORS_END;
const MIRRORED_CONTENT_SIZE: u16 = IMAGE_AND_SPRITE_PALETTE_MIRRORS_END;

pub struct Memory {
    pattern_table_mapper: Box<dyn PatternTableMemoryMapper>,
    name_and_attribute_table_mapper: Box<dyn NameAndAttributeTablesMemoryMapper>,
}

impl Memory {
    pub fn new(
        pattern_table_mapper: Box<dyn PatternTableMemoryMapper>,
        name_and_attribute_table_mapper: Box<dyn NameAndAttributeTablesMemoryMapper>,
    ) -> Self {
        Self {
            pattern_table_mapper,
            name_and_attribute_table_mapper,
        }
    }
}

impl super::Memory for Memory {
    /*
    TODO nametable mirroring

    horizontal
        0 and 1 are both the 1st physical ram
        2 and 3 are both the 2nd physical ram

    vertical
        0 and 2 are both the 1st physical ram
        1 and 3 are both the 2nd physical ram

    1 screen
        all 4 go to the same physical ram

    4 screen
        there is actually a full 4 kb of ram backing these, they're all distinct
    */

    fn read8(&self, address: u16) -> u8 {
        match address {
            ..PATTERN_TABLE_0_END => self
                .pattern_table_mapper
                .read8_pattern_table_0(address - PATTERN_TABLE_0_START),
            ..PATTERN_TABLE_1_END => self
                .pattern_table_mapper
                .read8_pattern_table_1(address - PATTERN_TABLE_1_START),
            ..NAME_TABLE_0_END => self
                .name_and_attribute_table_mapper
                .read8_name_table_0(address - NAME_TABLE_0_START),
            ..ATTRIBUTE_TABLE_0_END => self
                .name_and_attribute_table_mapper
                .read8_attribute_table_0(address - ATTRIBUTE_TABLE_0_START),
            ..NAME_TABLE_1_END => self
                .name_and_attribute_table_mapper
                .read8_name_table_1(address - NAME_TABLE_1_START),
            ..ATTRIBUTE_TABLE_1_END => self
                .name_and_attribute_table_mapper
                .read8_attribute_table_1(address - ATTRIBUTE_TABLE_1_START),
            ..NAME_TABLE_2_END => self
                .name_and_attribute_table_mapper
                .read8_name_table_2(address - NAME_TABLE_2_START),
            ..ATTRIBUTE_TABLE_2_END => self
                .name_and_attribute_table_mapper
                .read8_attribute_table_2(address - ATTRIBUTE_TABLE_2_START),
            ..NAME_TABLE_3_END => self
                .name_and_attribute_table_mapper
                .read8_name_table_3(address - NAME_TABLE_3_START),
            ..ATTRIBUTE_TABLE_3_END => self
                .name_and_attribute_table_mapper
                .read8_attribute_table_3(address - ATTRIBUTE_TABLE_3_START),
            ..NAME_AND_ATTRIBUTE_TABLE_MIRRORS_END => self.read8(
                (address - NAME_TABLE_0_START) % NAME_AND_ATTRIBUTE_TABLES_TOTAL_SIZE
                    + NAME_TABLE_0_START,
            ),
            ..IMAGE_PALETTE_END => todo!(),
            ..SPRITE_PALETTE_END => todo!(),
            ..IMAGE_AND_SPRITE_PALETTE_MIRRORS_END => self.read8(
                (address - IMAGE_PALETTE_START) % IMAGE_AND_SPRITE_PALETTE_TOTAL_SIZE
                    + IMAGE_PALETTE_START,
            ),
            _ => self.read8(address % MIRRORED_CONTENT_SIZE),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            ..PATTERN_TABLE_0_END => self
                .pattern_table_mapper
                .write8_pattern_table_0(address - PATTERN_TABLE_0_START, value),
            ..PATTERN_TABLE_1_END => self
                .pattern_table_mapper
                .write8_pattern_table_1(address - PATTERN_TABLE_1_START, value),
            ..NAME_TABLE_0_END => self
                .name_and_attribute_table_mapper
                .write8_name_table_0(address - NAME_TABLE_0_START, value),
            ..ATTRIBUTE_TABLE_0_END => self
                .name_and_attribute_table_mapper
                .write8_attribute_table_0(address - ATTRIBUTE_TABLE_0_START, value),
            ..NAME_TABLE_1_END => self
                .name_and_attribute_table_mapper
                .write8_name_table_1(address - NAME_TABLE_1_START, value),
            ..ATTRIBUTE_TABLE_1_END => self
                .name_and_attribute_table_mapper
                .write8_attribute_table_1(address - ATTRIBUTE_TABLE_1_START, value),
            ..NAME_TABLE_2_END => self
                .name_and_attribute_table_mapper
                .write8_name_table_2(address - NAME_TABLE_2_START, value),
            ..ATTRIBUTE_TABLE_2_END => self
                .name_and_attribute_table_mapper
                .write8_attribute_table_2(address - ATTRIBUTE_TABLE_2_START, value),
            ..NAME_TABLE_3_END => self
                .name_and_attribute_table_mapper
                .write8_name_table_3(address - NAME_TABLE_3_START, value),
            ..ATTRIBUTE_TABLE_3_END => self
                .name_and_attribute_table_mapper
                .write8_attribute_table_3(address - ATTRIBUTE_TABLE_3_START, value),
            ..NAME_AND_ATTRIBUTE_TABLE_MIRRORS_END => self.write8(
                (address - NAME_TABLE_0_START) % NAME_AND_ATTRIBUTE_TABLES_TOTAL_SIZE
                    + NAME_TABLE_0_START,
                value,
            ),
            ..IMAGE_PALETTE_END => todo!(),
            ..SPRITE_PALETTE_END => todo!(),
            ..IMAGE_AND_SPRITE_PALETTE_MIRRORS_END => self.write8(
                (address - IMAGE_PALETTE_START) % IMAGE_AND_SPRITE_PALETTE_TOTAL_SIZE
                    + IMAGE_PALETTE_START,
                value,
            ),
            _ => self.write8(address % MIRRORED_CONTENT_SIZE, value),
        }
    }
}

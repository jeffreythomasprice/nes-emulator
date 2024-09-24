pub mod mappers;

use crate::endians::Word;

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

pub mod main_memory {
    use crate::cartridge_file::pgr_rom;

    use super::mappers::MainMemoryMapper;

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
}

pub mod video_memory {
    use super::mappers::{
        MainMemoryMapper, NameAndAttributeTablesMemoryMapper, PatternTableMemoryMapper,
    };

    const PATTERN_TABLE_0_END: u16 = 0x1000;
    const PATTERN_TABLE_1_END: u16 = 0x2000;

    const PATTERN_TABLE_SIZE: u16 = PATTERN_TABLE_0_END;

    const NAME_TABLE_0_START: u16 = PATTERN_TABLE_1_END;
    const NAME_TABLE_0_END: u16 = 0x23c0;
    const ATTRIBUTE_TABLE_0_START: u16 = NAME_TABLE_0_END;
    const ATTRIBUTE_TABLE_0_END: u16 = 0x2400;
    const NAME_TABLE_1_END: u16 = 0x27c0;
    const ATTRIBUTE_TABLE_1_END: u16 = 0x2800;
    const NAME_TABLE_2_END: u16 = 0x2bc0;
    const ATTRIBUTE_TABLE_2_END: u16 = 0x2c00;
    const NAME_TABLE_3_END: u16 = 0x2fc0;
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
                ..PATTERN_TABLE_0_END => todo!(),
                ..PATTERN_TABLE_1_END => todo!(),
                ..NAME_TABLE_0_END => todo!(),
                ..ATTRIBUTE_TABLE_0_END => todo!(),
                ..NAME_TABLE_1_END => todo!(),
                ..ATTRIBUTE_TABLE_1_END => todo!(),
                ..NAME_TABLE_2_END => todo!(),
                ..ATTRIBUTE_TABLE_2_END => todo!(),
                ..NAME_TABLE_3_END => todo!(),
                ..ATTRIBUTE_TABLE_3_END => todo!(),
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
                ..PATTERN_TABLE_0_END => todo!(),
                ..PATTERN_TABLE_1_END => todo!(),
                ..NAME_TABLE_0_END => todo!(),
                ..ATTRIBUTE_TABLE_0_END => todo!(),
                ..NAME_TABLE_1_END => todo!(),
                ..ATTRIBUTE_TABLE_1_END => todo!(),
                ..NAME_TABLE_2_END => todo!(),
                ..ATTRIBUTE_TABLE_2_END => todo!(),
                ..NAME_TABLE_3_END => todo!(),
                ..ATTRIBUTE_TABLE_3_END => todo!(),
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
}

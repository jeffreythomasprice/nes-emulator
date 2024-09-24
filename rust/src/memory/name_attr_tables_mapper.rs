use super::video;

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

pub struct HorizontalMirroringNameAndAttributeTable {
    name_table_0: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_1: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_1: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
}

impl HorizontalMirroringNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_1: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_1: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
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
    name_table_0: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_1: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_1: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
}

impl VerticalMirroringNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_1: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_1: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
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
    name_table_0: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
}

impl SingleNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
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
    name_table_0: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_0: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_1: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_1: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_2: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_2: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
    name_table_3: [u8; video::NAME_TABLE_SIZE as usize],
    attribute_table_3: [u8; video::ATTRIBUTE_TABLE_SIZE as usize],
}

impl FourWayMirroringNameAndAttributeTable {
    pub fn new() -> Self {
        Self {
            name_table_0: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_0: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_1: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_1: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_2: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_2: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
            name_table_3: [0; video::NAME_TABLE_SIZE as usize],
            attribute_table_3: [0; video::ATTRIBUTE_TABLE_SIZE as usize],
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

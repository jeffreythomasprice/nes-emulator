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

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

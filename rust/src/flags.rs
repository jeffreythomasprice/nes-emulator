use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags: u8 {
        const CARRY_MASK = 0b0000_0001;
        const ZERO_MASK = 0b0000_0010;
        const INTERRUPT_DISABLE_MASK = 0b0000_0100;
        const DECIMAL_MODE_MASK = 0b0000_1000;
        const BREAK_COMMAND_MASK = 0b0001_0000;
        const UNUSED_MASK = 0b0010_0000;
        const OVERFLOW_MASK = 0b0100_0000;
        const NEGATIVE_MASK = 0b1000_0000;
    }
}

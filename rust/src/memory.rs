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

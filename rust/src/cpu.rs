use crate::{endians::Word, flags::Flags, memory::Memory};

const STACK_ADDRESS: u16 = 0x0100;
const NON_MASKABLE_INTERRUPT_ADDRESS: u16 = 0xfffa;
const RESET_INTERRUPT_ADDRESS: u16 = 0xfffc;
const INTERRUPT_REQUEST_INTERRUPT_ADDRESS: u16 = 0xfffe;

pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub flags: Flags,
    pub clock: u64,
}

struct AddrValue {
    address: u16,
    value: u8,
}

struct AddrValueClock {
    address: u16,
    value: u8,
    extra_clock: u64,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            flags: Flags::from_bits_truncate(0),
            clock: 0,
        }
    }

    pub fn step<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        match self.read_next_u8(m) {
            0x00 => self.brk(m),
            0x01 => self.ora_zero_page_indirect_x(m),
            0x02 => self.nop(-1, 3),
            0x03 => self.slo_zero_page_indirect_x(m),
            0x04 => self.nop(1, 3),
            0x05 => self.ora_zero_page_fixed(m),
            0x06 => self.asl_zero_page_fixed(m),
            0x07 => self.slo_zero_page_immediate(m),
            0x08 => self.php(m),
            0x09 => self.ora_immediate(m),
            0x0a => self.asl(),
            0x0b => self.anc_immediate(m),
            0x0c => self.nop(2, 4),
            0x0d => self.ora_absolute(m),
            0x0e => self.asl_absolute(m),
            0x0f => self.slo_absolute(m),
            0x10 => self.bpl(m),
            0x11 => self.ora_zero_page_indirect_y(m),
            0x12 => self.nop(-1, 3),
            0x13 => self.slo_zero_page_indirect_y(m),
            0x14 => self.nop(1, 4),
            0x15 => self.ora_zero_page_x(m),
            0x16 => self.asl_zero_page_x(m),
            0x17 => self.slo_zero_page_x(m),
            0x18 => self.clc(),
            0x19 => self.ora_absolute_y(m),
            0x1a => self.nop(0, 2),
            0x1b => self.slo_absolute_y(m),
            0x1c => self.nop_absolute_x(m),
            0x1d => self.ora_absolute_x(m),
            0x1e => self.asl_absolute_x(m),
            0x1f => self.slo_absolute_x(m),
            // 	case 0x20:
            // 		c.jsr(m)
            // 	case 0x21:
            // 		c.andZeroPageIndirectX(m)
            // 	case 0x22:
            // 		c.nop(0, 3)
            // 	case 0x23:
            // 		c.rlaZeroPageIndirectX(m)
            // 	case 0x24:
            // 		c.bitZeroPageImmediate(m)
            // 	case 0x25:
            // 		c.andZeroPage(m)
            // 	case 0x26:
            // 		c.rolZeroPage(m)
            // 	case 0x27:
            // 		c.rlaZeroPage(m)
            // 	case 0x28:
            // 		c.plp(m)
            // 	case 0x29:
            // 		c.andImmediate(m)
            // 	case 0x2a:
            // 		c.rol()
            // 	case 0x2b:
            // 		c.ancImmediate(m)
            // 	case 0x2c:
            // 		c.bitAbsolute(m)
            // 	case 0x2d:
            // 		c.andAbsolute(m)
            // 	case 0x2e:
            // 		c.rolAbsolute(m)
            // 	case 0x2f:
            // 		c.rlaAbsolute(m)
            // 	case 0x30:
            // 		c.bmi(m)
            // 	case 0x31:
            // 		c.andZeroPageIndirectY(m)
            // 	case 0x32:
            // 		c.nop(0, 3)
            // 	case 0x33:
            // 		c.rlaZeroPageIndirectY(m)
            // 	case 0x34:
            // 		c.nop(2, 4)
            // 	case 0x35:
            // 		c.andZeroPageX(m)
            // 	case 0x36:
            // 		c.rolZeroPageX(m)
            // 	case 0x37:
            // 		c.rlaZeroPageX(m)
            // 	case 0x38:
            // 		c.sec()
            // 	case 0x39:
            // 		c.andAbsoluteY(m)
            // 	case 0x3a:
            // 		c.nop(1, 2)
            // 	case 0x3b:
            // 		c.rlaAbsoluteY(m)
            // 	case 0x3c:
            // 		c.nopAbsoluteX(m)
            // 	case 0x3d:
            // 		c.andAbsoluteX(m)
            // 	case 0x3e:
            // 		c.rolAbsoluteX(m)
            // 	case 0x3f:
            // 		c.rlaAbsoluteX(m)
            // 	case 0x40:
            // 		c.rti(m)
            // 	case 0x41:
            // 		c.eorZeroPageIndirectX(m)
            // 	case 0x42:
            // 		c.nop(0, 3)
            // 	case 0x43:
            // 		c.sreZeroPageIndirectX(m)
            // 	case 0x44:
            // 		c.nop(2, 3)
            // 	case 0x45:
            // 		c.eorZeroPage(m)
            // 	case 0x46:
            // 		c.lsrZeroPage(m)
            // 	case 0x47:
            // 		c.sreZeroPage(m)
            // 	case 0x48:
            // 		c.pha(m)
            // 	case 0x49:
            // 		c.eorImmediate(m)
            // 	case 0x4a:
            // 		c.lsr()
            // 	case 0x4b:
            // 		c.alr(m)
            // 	case 0x4c:
            // 		c.jmpAbsolute(m)
            // 	case 0x4d:
            // 		c.eorAbsolute(m)
            // 	case 0x4e:
            // 		c.lsrAbsolute(m)
            // 	case 0x4f:
            // 		c.sreAbsolute(m)
            // 	case 0x50:
            // 		c.bvc(m)
            // 	case 0x51:
            // 		c.eorZeroPageIndirectY(m)
            // 	case 0x52:
            // 		c.nop(0, 3)
            // 	case 0x53:
            // 		c.sreZeroPageIndirectY(m)
            // 	case 0x54:
            // 		c.nop(2, 4)
            // 	case 0x55:
            // 		c.eorZeroPageX(m)
            // 	case 0x56:
            // 		c.lsrZeroPageX(m)
            // 	case 0x57:
            // 		c.sreZeroPageX(m)
            // 	case 0x58:
            // 		c.cli()
            // 	case 0x59:
            // 		c.eorAbsoluteY(m)
            // 	case 0x5a:
            // 		c.nop(1, 2)
            // 	case 0x5b:
            // 		c.sreAbsoluteY(m)
            // 	case 0x5c:
            // 		c.nopAbsoluteX(m)
            // 	case 0x5d:
            // 		c.eorAbsoluteX(m)
            // 	case 0x5e:
            // 		c.lsrAbsoluteX(m)
            // 	case 0x5f:
            // 		c.sreAbsoluteX(m)
            // 	case 0x60:
            // 		c.rta(m)
            // 	case 0x61:
            // 		c.adcZeroPageIndirectX(m)
            // 	case 0x62:
            // 		c.nop(0, 3)
            // 	case 0x63:
            // 		c.rraZeroPageIndirectX(m)
            // 	case 0x64:
            // 		c.nop(2, 3)
            // 	case 0x65:
            // 		c.adcZeroPage(m)
            // 	case 0x66:
            // 		c.rorZeroPage(m)
            // 	case 0x67:
            // 		c.rraZeroPage(m)
            // 	case 0x68:
            // 		c.pla(m)
            // 	case 0x69:
            // 		c.adcImmediate(m)
            // 	case 0x6a:
            // 		c.ror()
            // 	case 0x6b:
            // 		c.arrImmediate(m)
            // 	case 0x6c:
            // 		c.jmpIndirect(m)
            // 	case 0x6d:
            // 		c.adcAbsolute(m)
            // 	case 0x6e:
            // 		c.rorAbsolute(m)
            // 	case 0x6f:
            // 		c.rraAbsolute(m)
            // 	case 0x70:
            // 		c.bvs(m)
            // 	case 0x71:
            // 		c.adcZeroPageIndirectY(m)
            // 	case 0x72:
            // 		c.nop(0, 3)
            // 	case 0x73:
            // 		c.rraZeroPageIndirectY(m)
            // 	case 0x74:
            // 		c.nop(2, 4)
            // 	case 0x75:
            // 		c.adcZeroPageX(m)
            // 	case 0x76:
            // 		c.rorZeroPageX(m)
            // 	case 0x77:
            // 		c.rraZeroPageX(m)
            // 	case 0x78:
            // 		c.sei()
            // 	case 0x79:
            // 		c.adcAbsoluteY(m)
            // 	case 0x7a:
            // 		c.nop(1, 2)
            // 	case 0x7b:
            // 		c.rraAbsoluteY(m)
            // 	case 0x7c:
            // 		c.nopAbsoluteX(m)
            // 	case 0x7d:
            // 		c.adcAbsoluteX(m)
            // 	case 0x7e:
            // 		c.rorAbsoluteX(m)
            // 	case 0x7f:
            // 		c.rraAbsoluteX(m)
            // 	case 0x80:
            // 		c.nop(2, 2)
            // 	case 0x81:
            // 		c.staZeroPageIndirectX(m)
            // 	case 0x82:
            // 		c.nop(2, 2)
            // 	case 0x83:
            // 		c.saxZeroPageIndirectX(m)
            // 	case 0x84:
            // 		c.styZeroPage(m)
            // 	case 0x85:
            // 		c.staZeroPage(m)
            // 	case 0x86:
            // 		c.stxZeroPage(m)
            // 	case 0x87:
            // 		c.saxZeroPage(m)
            // 	case 0x88:
            // 		c.dey()
            // 	case 0x89:
            // 		c.nop(2, 2)
            // 	case 0x8a:
            // 		c.txa()
            // 	case 0x8b:
            // 		c.xaaImmediate(m)
            // 	case 0x8c:
            // 		c.styAbsolute(m)
            // 	case 0x8d:
            // 		c.staAbsolute(m)
            // 	case 0x8e:
            // 		c.stxAbsolute(m)
            // 	case 0x8f:
            // 		c.saxAbsolute(m)
            // 	case 0x90:
            // 		c.bcc(m)
            // 	case 0x91:
            // 		c.staZeroPageIndirectY(m)
            // 	case 0x92:
            // 		c.nop(0, 3)
            // 	case 0x93:
            // 		c.ahxZeroPageIndirectY(m)
            // 	case 0x94:
            // 		// TODO impl
            // 	case 0x95:
            // 		// TODO impl
            // 	case 0x96:
            // 		// TODO impl
            // 	case 0x97:
            // 		// TODO impl
            // 	case 0x98:
            // 		// TODO impl
            // 	case 0x99:
            // 		// TODO impl
            // 	case 0x9a:
            // 		// TODO impl
            // 	case 0x9b:
            // 		// TODO impl
            // 	case 0x9c:
            // 		// TODO impl
            // 	case 0x9d:
            // 		// TODO impl
            // 	case 0x9e:
            // 		// TODO impl
            // 	case 0x9f:
            // 		// TODO impl
            // 	case 0xa0:
            // 		// TODO impl
            // 	case 0xa1:
            // 		// TODO impl
            // 	case 0xa2:
            // 		// TODO impl
            // 	case 0xa3:
            // 		// TODO impl
            // 	case 0xa4:
            // 		// TODO impl
            // 	case 0xa5:
            // 		// TODO impl
            // 	case 0xa6:
            // 		// TODO impl
            // 	case 0xa7:
            // 		// TODO impl
            // 	case 0xa8:
            // 		// TODO impl
            // 	case 0xa9:
            // 		// TODO impl
            // 	case 0xaa:
            // 		// TODO impl
            // 	case 0xab:
            // 		// TODO impl
            // 	case 0xac:
            // 		// TODO impl
            // 	case 0xad:
            // 		// TODO impl
            // 	case 0xae:
            // 		// TODO impl
            // 	case 0xaf:
            // 		// TODO impl
            // 	case 0xb0:
            // 		// TODO impl
            // 	case 0xb1:
            // 		// TODO impl
            // 	case 0xb2:
            // 		// TODO impl
            // 	case 0xb3:
            // 		// TODO impl
            // 	case 0xb4:
            // 		// TODO impl
            // 	case 0xb5:
            // 		// TODO impl
            // 	case 0xb6:
            // 		// TODO impl
            // 	case 0xb7:
            // 		// TODO impl
            // 	case 0xb8:
            // 		// TODO impl
            // 	case 0xb9:
            // 		// TODO impl
            // 	case 0xba:
            // 		// TODO impl
            // 	case 0xbb:
            // 		// TODO impl
            // 	case 0xbc:
            // 		// TODO impl
            // 	case 0xbd:
            // 		// TODO impl
            // 	case 0xbe:
            // 		// TODO impl
            // 	case 0xbf:
            // 		// TODO impl
            // 	case 0xc0:
            // 		// TODO impl
            // 	case 0xc1:
            // 		// TODO impl
            // 	case 0xc2:
            // 		// TODO impl
            // 	case 0xc3:
            // 		// TODO impl
            // 	case 0xc4:
            // 		// TODO impl
            // 	case 0xc5:
            // 		// TODO impl
            // 	case 0xc6:
            // 		// TODO impl
            // 	case 0xc7:
            // 		// TODO impl
            // 	case 0xc8:
            // 		// TODO impl
            // 	case 0xc9:
            // 		// TODO impl
            // 	case 0xca:
            // 		// TODO impl
            // 	case 0xcb:
            // 		// TODO impl
            // 	case 0xcc:
            // 		// TODO impl
            // 	case 0xcd:
            // 		// TODO impl
            // 	case 0xce:
            // 		// TODO impl
            // 	case 0xcf:
            // 		// TODO impl
            // 	case 0xd0:
            // 		// TODO impl
            // 	case 0xd1:
            // 		// TODO impl
            // 	case 0xd2:
            // 		// TODO impl
            // 	case 0xd3:
            // 		// TODO impl
            // 	case 0xd4:
            // 		// TODO impl
            // 	case 0xd5:
            // 		// TODO impl
            // 	case 0xd6:
            // 		// TODO impl
            // 	case 0xd7:
            // 		// TODO impl
            // 	case 0xd8:
            // 		// TODO impl
            // 	case 0xd9:
            // 		// TODO impl
            // 	case 0xda:
            // 		// TODO impl
            // 	case 0xdb:
            // 		// TODO impl
            // 	case 0xdc:
            // 		// TODO impl
            // 	case 0xdd:
            // 		// TODO impl
            // 	case 0xde:
            // 		// TODO impl
            // 	case 0xdf:
            // 		// TODO impl
            // 	case 0xe0:
            // 		// TODO impl
            // 	case 0xe1:
            // 		// TODO impl
            // 	case 0xe2:
            // 		// TODO impl
            // 	case 0xe3:
            // 		// TODO impl
            // 	case 0xe4:
            // 		// TODO impl
            // 	case 0xe5:
            // 		// TODO impl
            // 	case 0xe6:
            // 		// TODO impl
            // 	case 0xe7:
            // 		// TODO impl
            // 	case 0xe8:
            // 		// TODO impl
            // 	case 0xe9:
            // 		// TODO impl
            // 	case 0xea:
            // 		// TODO impl
            // 	case 0xeb:
            // 		// TODO impl
            // 	case 0xec:
            // 		// TODO impl
            // 	case 0xed:
            // 		// TODO impl
            // 	case 0xee:
            // 		// TODO impl
            // 	case 0xef:
            // 		// TODO impl
            // 	case 0xf0:
            // 		// TODO impl
            // 	case 0xf1:
            // 		// TODO impl
            // 	case 0xf2:
            // 		// TODO impl
            // 	case 0xf3:
            // 		// TODO impl
            // 	case 0xf4:
            // 		// TODO impl
            // 	case 0xf5:
            // 		// TODO impl
            // 	case 0xf6:
            // 		// TODO impl
            // 	case 0xf7:
            // 		// TODO impl
            // 	case 0xf8:
            // 		// TODO impl
            // 	case 0xf9:
            // 		// TODO impl
            // 	case 0xfa:
            // 		// TODO impl
            // 	case 0xfb:
            // 		// TODO impl
            // 	case 0xfc:
            // 		// TODO impl
            // 	case 0xfd:
            // 		// TODO impl
            // 	case 0xfe:
            // 		// TODO impl
            // 	case 0xff:
            // 		// TODO impl
            // 	}
            _ => todo!("TODO instruction not implemented"),
        }
    }

    fn brk<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.push16(m, self.pc + 1);
        self.push8(m, (self.flags | Flags::BREAK_COMMAND_MASK).bits());
        self.flags.set(Flags::INTERRUPT_DISABLE_MASK, true);
        self.pc = m.read16(INTERRUPT_REQUEST_INTERRUPT_ADDRESS);
        self.clock += 7;
    }

    fn php<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.push8(m, (self.flags | Flags::BREAK_COMMAND_MASK).bits());
        self.clock += 3;
    }

    // TODO here

    // func (c *CPU) plp(m Memory) {
    // 	c.Flags = c.pop8(m)
    // 	c.clearFlags(BreakCommandFlagMask)
    // 	c.setFlags(UnusedFlagMask)
    // 	c.PC += 1
    // 	c.ClockTime += 4
    // }

    // func (c *CPU) rti(m Memory) {
    // 	c.Flags = c.pop8(m)
    // 	c.PC = c.pop16(m)
    // 	c.clearFlags(BreakCommandFlagMask)
    // 	c.setFlags(UnusedFlagMask)
    // 	c.ClockTime += 6
    // }

    // func (c *CPU) rta(m Memory) {
    // 	c.PC = c.pop16(m) + 1
    // 	c.ClockTime += 6
    // }

    // func (c *CPU) pha(m Memory) {
    // 	c.push8(m, c.A)
    // 	c.PC += 1
    // 	c.ClockTime += 3
    // }

    // func (c *CPU) pla(m Memory) {
    // 	c.A = c.pop8(m)
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.PC += 1
    // 	c.ClockTime += 4
    // }

    fn ora_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.ora_common(value, 2);
    }

    fn ora_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.ora_common(value, 4);
    }

    fn ora_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.ora_common(value, 6);
    }

    fn ora_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.ora_common(value, 5 + extra_clock);
    }

    fn ora_zero_page_fixed<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.ora_common(value, 3)
    }

    fn ora_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.ora_common(value, 4);
    }

    fn ora_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.ora_common(value, 4 + extra_clock);
    }

    fn ora_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.ora_common(value, 4 + extra_clock);
    }

    fn ora_common(&mut self, new_value: u8, clock: u64) {
        self.a |= new_value;
        self.flags.set(Flags::NEGATIVE_MASK, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO_MASK, self.a == 0);
        self.clock += clock;
    }

    fn slo_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        self.slo_common(m, address, value, 6);
    }

    fn slo_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        self.slo_common(m, address, value, 7);
    }

    fn slo_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_y(m);
        self.slo_common(m, address, value, 7);
    }

    fn slo_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        self.slo_common(m, address, value, 8);
    }

    fn slo_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        self.slo_common(m, address, value, 8);
    }

    fn slo_zero_page_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        // 	address, value := c.zeroPageFixed(m)
        self.slo_common(m, address, value, 5)
    }

    fn slo_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        // 	address, value := c.zeroPageFixed(m)
        self.slo_common(m, address, value, 6)
    }

    fn slo_common<M>(&mut self, m: &mut M, address: u16, value: u8, clock: u64)
    where
        M: Memory,
    {
        let new_value = value << 1;
        m.write8(address, new_value);
        self.a |= new_value;
        self.flags.set(Flags::NEGATIVE_MASK, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO_MASK, self.a == 0);
        self.flags.set(Flags::CARRY_MASK, new_value < value);
        self.clock += clock;
    }

    fn asl(&mut self) {
        let value = self.a;
        let new_value = value << 1;
        self.a = new_value;
        self.flags.set(Flags::NEGATIVE_MASK, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO_MASK, new_value == 0);
        self.flags.set(Flags::CARRY_MASK, new_value < value);
        self.clock += 2;
    }

    fn asl_zero_page_fixed<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        self.asl_common(m, address, value, 5);
    }

    fn asl_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        self.asl_common(m, address, value, 6);
    }

    fn asl_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        self.asl_common(m, address, value, 6);
    }

    fn asl_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        self.asl_common(m, address, value, 7);
    }

    fn asl_common<M>(&mut self, m: &mut M, address: u16, value: u8, clock: u64)
    where
        M: Memory,
    {
        let new_value = value << 1;
        m.write8(address, new_value);
        self.flags.set(Flags::NEGATIVE_MASK, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO_MASK, new_value == 0);
        self.flags.set(Flags::CARRY_MASK, new_value < value);
        self.clock += clock;
    }

    fn anc_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        let new_value = self.a & value;
        self.a = new_value;
        self.flags.set(Flags::NEGATIVE_MASK, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO_MASK, new_value == 0);
        self.flags
            .set(Flags::CARRY_MASK, (new_value & 0b1000_0000) != 0);
        self.clock += 2;
    }

    // TODO here

    // func (c *CPU) andZeroPage(m Memory) {
    // 	_, value := c.zeroPageFixed(m)
    // 	c.andCommon(value, 2, 3)
    // }

    // func (c *CPU) andZeroPageX(m Memory) {
    // 	_, value := c.zeroPageX(m)
    // 	c.andCommon(value, 2, 4)
    // }

    // func (c *CPU) andZeroPageIndirectX(m Memory) {
    // 	_, value := c.zeroPageIndirectX(m)
    // 	c.andCommon(value, 2, 6)
    // }

    // func (c *CPU) andZeroPageIndirectY(m Memory) {
    // 	_, value, extraClock := c.zeroPageIndirectY(m)
    // 	c.andCommon(value, 2, 5+extraClock)
    // }

    // func (c *CPU) andImmediate(m Memory) {
    // 	value := m.Read(c.PC + 1)
    // 	c.andCommon(value, 2, 2)
    // }

    // func (c *CPU) andAbsolute(m Memory) {
    // 	_, value := c.absolute(m)
    // 	c.andCommon(value, 3, 4)
    // }

    // func (c *CPU) andAbsoluteX(m Memory) {
    // 	_, value, extraClock := c.absoluteX(m)
    // 	c.andCommon(value, 3, 4+extraClock)
    // }

    // func (c *CPU) andAbsoluteY(m Memory) {
    // 	_, value, extraClock := c.absoluteY(m)
    // 	c.andCommon(value, 3, 4+extraClock)
    // }

    // func (c *CPU) andCommon(value uint8, pcOffset uint16, clock uint64) {
    // 	c.A &= value
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // }

    // func (c *CPU) rlaZeroPage(m Memory) {
    // 	address, value := c.zeroPageFixed(m)
    // 	c.rlaCommon(m, address, value, 2, 5)
    // }

    // func (c *CPU) rlaZeroPageX(m Memory) {
    // 	address, value := c.zeroPageX(m)
    // 	c.rlaCommon(m, address, value, 2, 6)
    // }

    // func (c *CPU) rlaZeroPageIndirectX(m Memory) {
    // 	address, value := c.zeroPageIndirectX(m)
    // 	c.rlaCommon(m, address, value, 2, 8)
    // }

    // func (c *CPU) rlaZeroPageIndirectY(m Memory) {
    // 	address, value, _ := c.zeroPageIndirectY(m)
    // 	c.rlaCommon(m, address, value, 2, 8)
    // }

    // func (c *CPU) rlaAbsolute(m Memory) {
    // 	address, value := c.absolute(m)
    // 	c.rlaCommon(m, address, value, 3, 6)
    // }

    // func (c *CPU) rlaAbsoluteX(m Memory) {
    // 	address, value, _ := c.absoluteX(m)
    // 	c.rlaCommon(m, address, value, 3, 7)
    // }

    // func (c *CPU) rlaAbsoluteY(m Memory) {
    // 	address, value, _ := c.absoluteY(m)
    // 	c.rlaCommon(m, address, value, 3, 7)
    // }

    // func (c *CPU) rlaCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
    // 	newValue := value << 1
    // 	if (c.Flags & CarryFlagMask) != 0 {
    // 		newValue += 1
    // 	}
    // 	m.Write(address, newValue)
    // 	c.A &= newValue
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.setFlagsTo(CarryFlagMask, (value&0b1000_0000) != 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // }

    // func (c *CPU) rol() {
    // 	c.A = c.rolCommon(c.A, 1, 2)
    // }

    // func (c *CPU) rolZeroPage(m Memory) {
    // 	address, value := c.zeroPageFixed(m)
    // 	newValue := c.rolCommon(value, 2, 5)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rolZeroPageX(m Memory) {
    // 	address, value := c.zeroPageX(m)
    // 	newValue := c.rolCommon(value, 2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rolAbsolute(m Memory) {
    // 	address, value := c.absolute(m)
    // 	newValue := c.rolCommon(value, 3, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rolAbsoluteX(m Memory) {
    // 	address, value, _ := c.absoluteX(m)
    // 	newValue := c.rolCommon(value, 3, 7)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rolCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
    // 	newValue := value << 1
    // 	if (c.Flags & CarryFlagMask) != 0 {
    // 		newValue += 1
    // 	}
    // 	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, newValue == 0)
    // 	c.setFlagsTo(CarryFlagMask, (value&0b1000_0000) != 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return newValue
    // }

    fn bpl<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, !self.flags.contains(Flags::NEGATIVE_MASK));
    }

    // TODO here

    // func (c *CPU) bmi(m Memory) {
    // 	c.branchCommon(m, (c.Flags&NegativeFlagMask) != 0)
    // }

    // func (c *CPU) bvc(m Memory) {
    // 	c.branchCommon(m, (c.Flags&OverflowFlagMask) == 0)
    // }

    // func (c *CPU) bvs(m Memory) {
    // 	c.branchCommon(m, (c.Flags&OverflowFlagMask) != 0)
    // }

    // func (c *CPU) bcc(m Memory) {
    // 	c.branchCommon(m, (c.Flags&CarryFlagMask) == 0)
    // }

    fn branch_common<M>(&mut self, m: &mut M, condition: bool)
    where
        M: Memory,
    {
        if condition {
            let offset = self.read_next_u8(m);
            // high byte of address after the branch instruction
            let high1 = self.pc & 0xff00;
            // do the jump
            self.pc = self.pc.wrapping_add(offset as i8 as u16);
            // high byte of address of branch destination
            let high2 = self.pc & 0xff00;
            self.clock += if high1 == high2 { 3 } else { 4 }
        } else {
            self.pc += 1;
            self.clock += 2;
        }
    }

    // TODO here

    // func (c *CPU) jmpAbsolute(m Memory) {
    // 	c.PC = Read16(m, c.PC+1)
    // 	c.ClockTime += 3
    // }

    // func (c *CPU) jmpIndirect(m Memory) {
    // 	address := Read16(m, c.PC+1)
    // 	if address&0xff == 0x0ff {
    // 		low := m.Read(address)
    // 		high := m.Read(address & 0xff00)
    // 		c.PC = uint16(low) | (uint16(high) << 8)
    // 	} else {
    // 		c.PC = Read16(m, address)
    // 	}
    // 	c.ClockTime += 5
    // }

    fn clc(&mut self) {
        self.flags -= Flags::CARRY_MASK;
        self.clock += 2
    }

    // TODO here

    // func (c *CPU) sec() {
    // 	c.setFlags(CarryFlagMask)
    // 	c.PC += 1
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) cli() {
    // 	c.clearFlags(InterruptDisableFlagMask)
    // 	c.PC += 1
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) sei() {
    // 	c.setFlags(InterruptDisableFlagMask)
    // 	c.PC += 1
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) dey() {
    // 	c.Y--
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.Y) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.Y == 0)
    // 	c.PC += 1
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) jsr(m Memory) {
    // 	address := Read16(m, c.PC+1)
    // 	c.push16(m, c.PC+2)
    // 	c.PC = address
    // 	c.ClockTime += 6
    // }

    // func (c *CPU) bitZeroPageImmediate(m Memory) {
    // 	_, value := c.zeroPageFixed(m)
    // 	c.bitCommon(value, 2, 3)
    // }

    // func (c *CPU) bitAbsolute(m Memory) {
    // 	_, value := c.absolute(m)
    // 	c.bitCommon(value, 3, 4)
    // }

    // func (c *CPU) bitCommon(value uint8, pcOffset uint16, clock uint64) {
    // 	c.setFlagsTo(OverflowFlagMask, (value&OverflowFlagMask) != 0)
    // 	c.setFlagsTo(NegativeFlagMask, (NegativeFlagMask&value) != 0)
    // 	c.setFlagsTo(ZeroFlagMask, (value&c.A) == 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // }

    // func (c *CPU) eorZeroPage(m Memory) {
    // 	_, value := c.zeroPageFixed(m)
    // 	c.eorCommon(value, 2, 3)
    // }

    // func (c *CPU) eorZeroPageX(m Memory) {
    // 	_, value := c.zeroPageX(m)
    // 	c.eorCommon(value, 2, 4)
    // }

    // func (c *CPU) eorZeroPageIndirectX(m Memory) {
    // 	_, value := c.zeroPageIndirectX(m)
    // 	c.eorCommon(value, 2, 6)
    // }

    // func (c *CPU) eorZeroPageIndirectY(m Memory) {
    // 	_, value, extraClock := c.zeroPageIndirectY(m)
    // 	c.eorCommon(value, 2, 5+extraClock)
    // }

    // func (c *CPU) eorImmediate(m Memory) {
    // 	value := m.Read(c.PC + 1)
    // 	c.eorCommon(value, 2, 2)
    // }

    // func (c *CPU) eorAbsolute(m Memory) {
    // 	_, value := c.absolute(m)
    // 	c.eorCommon(value, 3, 4)
    // }

    // func (c *CPU) eorAbsoluteX(m Memory) {
    // 	_, value, extraClock := c.absoluteX(m)
    // 	c.eorCommon(value, 3, 4+extraClock)
    // }

    // func (c *CPU) eorAbsoluteY(m Memory) {
    // 	_, value, extraClock := c.absoluteY(m)
    // 	c.eorCommon(value, 3, 4+extraClock)
    // }

    // func (c *CPU) eorCommon(value uint8, pcOffset uint16, clock uint64) {
    // 	c.A ^= value
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // }

    // func (c *CPU) sreZeroPage(m Memory) {
    // 	address, value := c.zeroPageFixed(m)
    // 	c.sreCommon(m, address, value, 2, 5)
    // }

    // func (c *CPU) sreZeroPageX(m Memory) {
    // 	address, value := c.zeroPageX(m)
    // 	c.sreCommon(m, address, value, 2, 6)
    // }

    // func (c *CPU) sreZeroPageIndirectX(m Memory) {
    // 	address, value := c.zeroPageIndirectX(m)
    // 	c.sreCommon(m, address, value, 2, 8)
    // }

    // func (c *CPU) sreZeroPageIndirectY(m Memory) {
    // 	address, value, _ := c.zeroPageIndirectY(m)
    // 	c.sreCommon(m, address, value, 2, 8)
    // }

    // func (c *CPU) sreAbsolute(m Memory) {
    // 	address, value := c.absolute(m)
    // 	c.sreCommon(m, address, value, 3, 6)
    // }

    // func (c *CPU) sreAbsoluteX(m Memory) {
    // 	address, value, _ := c.absoluteX(m)
    // 	c.sreCommon(m, address, value, 3, 7)
    // }

    // func (c *CPU) sreAbsoluteY(m Memory) {
    // 	address, value, _ := c.absoluteY(m)
    // 	c.sreCommon(m, address, value, 3, 7)
    // }

    // func (c *CPU) sreCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
    // 	newValue := value >> 1
    // 	m.Write(address, newValue)
    // 	c.A ^= newValue
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // }

    // func (c *CPU) lsr() {
    // 	c.A = c.lsrCommon(c.A, 1, 2)
    // }

    // func (c *CPU) lsrZeroPage(m Memory) {
    // 	address, value := c.zeroPageFixed(m)
    // 	newValue := c.lsrCommon(value, 2, 5)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) lsrZeroPageX(m Memory) {
    // 	address, value := c.zeroPageX(m)
    // 	newValue := c.lsrCommon(value, 2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) lsrAbsolute(m Memory) {
    // 	address, value := c.absolute(m)
    // 	newValue := c.lsrCommon(value, 3, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) lsrAbsoluteX(m Memory) {
    // 	address, value, _ := c.absoluteX(m)
    // 	newValue := c.lsrCommon(value, 3, 7)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) lsrCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
    // 	newValue := value >> 1
    // 	c.clearFlags(NegativeFlagMask)
    // 	c.setFlagsTo(ZeroFlagMask, newValue == 0)
    // 	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return newValue
    // }

    // func (c *CPU) alr(m Memory) {
    // 	value := c.A & m.Read(c.PC+1)
    // 	c.A = value >> 1
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
    // 	c.PC += 2
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) adcImmediate(m Memory) {
    // 	value := m.Read(c.PC + 1)
    // 	c.adcCommon(value, 2, 2)
    // }

    // func (c *CPU) adcAbsolute(m Memory) {
    // 	_, value := c.absolute(m)
    // 	c.adcCommon(value, 3, 4)
    // }

    // func (c *CPU) adcAbsoluteX(m Memory) {
    // 	_, value, extraClock := c.absoluteX(m)
    // 	c.adcCommon(value, 3, 4+extraClock)
    // }

    // func (c *CPU) adcAbsoluteY(m Memory) {
    // 	_, value, extraClock := c.absoluteY(m)
    // 	c.adcCommon(value, 3, 4+extraClock)
    // }

    // func (c *CPU) adcZeroPage(m Memory) {
    // 	_, value := c.zeroPageFixed(m)
    // 	c.adcCommon(value, 2, 3)
    // }

    // func (c *CPU) adcZeroPageX(m Memory) {
    // 	_, value := c.zeroPageX(m)
    // 	c.adcCommon(value, 2, 4)
    // }

    // func (c *CPU) adcZeroPageIndirectX(m Memory) {
    // 	_, value := c.zeroPageIndirectX(m)
    // 	c.adcCommon(value, 2, 6)
    // }

    // func (c *CPU) adcZeroPageIndirectY(m Memory) {
    // 	_, value, extraClock := c.zeroPageIndirectY(m)
    // 	c.adcCommon(value, 2, 5+extraClock)
    // }

    // func (c *CPU) adcCommon(value uint8, pcOffset uint16, clock uint64) {
    // 	newValue := uint16(c.A) + uint16(value)
    // 	if (c.Flags & CarryFlagMask) != 0 {
    // 		newValue++
    // 	}
    // 	oldA := c.A
    // 	c.A = uint8(newValue)
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.setFlagsTo(CarryFlagMask, (newValue&0b1_0000_0000) != 0)
    // 	valueSignBit := value & 0b1000_0000
    // 	oldASignBit := oldA & 0b1000_0000
    // 	newSignBit := uint8(newValue & 0b1000_0000)
    // 	c.setFlagsTo(OverflowFlagMask, valueSignBit == oldASignBit && valueSignBit != newSignBit)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // }

    // func (c *CPU) rraAbsolute(m Memory) {
    // 	address, value := c.absolute(m)
    // 	newValue := c.rraCommon(value, 3, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraAbsoluteX(m Memory) {
    // 	address, value, _ := c.absoluteX(m)
    // 	newValue := c.rraCommon(value, 3, 7)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraAbsoluteY(m Memory) {
    // 	address, value, _ := c.absoluteY(m)
    // 	newValue := c.rraCommon(value, 3, 7)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraZeroPage(m Memory) {
    // 	address, value := c.zeroPageFixed(m)
    // 	newValue := c.rraCommon(value, 2, 5)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraZeroPageX(m Memory) {
    // 	address, value := c.zeroPageX(m)
    // 	newValue := c.rraCommon(value, 2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraZeroPageIndirectX(m Memory) {
    // 	address, value := c.zeroPageIndirectX(m)
    // 	newValue := c.rraCommon(value, 2, 8)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraZeroPageIndirectY(m Memory) {
    // 	address, value, _ := c.zeroPageIndirectY(m)
    // 	newValue := c.rraCommon(value, 2, 8)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rraCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
    // 	rorNewValue := value >> 1
    // 	if (c.Flags & CarryFlagMask) != 0 {
    // 		rorNewValue |= 0b1000_0000
    // 	}
    // 	adcNewValue := uint16(c.A) + uint16(rorNewValue)
    // 	// carry flag check, but what the carry flag should be after the previous ROR
    // 	if (value & 1) != 0 {
    // 		adcNewValue++
    // 	}
    // 	oldA := c.A
    // 	c.A = uint8(adcNewValue)
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.setFlagsTo(CarryFlagMask, (adcNewValue&0b1_0000_0000) != 0)
    // 	valueSignBit := rorNewValue & 0b1000_0000
    // 	oldASignBit := oldA & 0b1000_0000
    // 	newSignBit := uint8(adcNewValue & 0b1000_0000)
    // 	c.setFlagsTo(OverflowFlagMask, valueSignBit == oldASignBit && valueSignBit != newSignBit)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return rorNewValue
    // }

    // func (c *CPU) ror() {
    // 	c.A = c.rorCommon(c.A, 1, 2)
    // }

    // func (c *CPU) rorAbsolute(m Memory) {
    // 	address, value := c.absolute(m)
    // 	newValue := c.rorCommon(value, 3, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rorAbsoluteX(m Memory) {
    // 	address, value, _ := c.absoluteX(m)
    // 	newValue := c.rorCommon(value, 3, 7)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rorZeroPage(m Memory) {
    // 	address, value := c.zeroPageFixed(m)
    // 	newValue := c.rorCommon(value, 2, 5)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rorZeroPageX(m Memory) {
    // 	address, value := c.zeroPageX(m)
    // 	newValue := c.rorCommon(value, 2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) rorCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
    // 	newValue := value >> 1
    // 	if (c.Flags & CarryFlagMask) != 0 {
    // 		newValue |= 0b1000_0000
    // 	}
    // 	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, newValue == 0)
    // 	c.setFlagsTo(CarryFlagMask, (value&1) != 0)
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return newValue
    // }

    // func (c *CPU) arrImmediate(m Memory) {
    // 	immValue := m.Read(c.PC + 1)
    // 	newValue := c.A & immValue
    // 	c.setFlagsTo(OverflowFlagMask, (newValue^(newValue>>1))&0x40 != 0)
    // 	newCarry := newValue & 0b1000_0000
    // 	newValue >>= 1
    // 	if (c.Flags & CarryFlagMask) != 0 {
    // 		newValue |= 0b1000_0000
    // 	}
    // 	c.A = newValue
    // 	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, newValue == 0)
    // 	c.setFlagsTo(CarryFlagMask, newCarry != 0)
    // 	c.PC += 2
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) staAbsolute(m Memory) {
    // 	address, _ := c.absolute(m)
    // 	newValue := c.staCommon(3, 4)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) staZeroPage(m Memory) {
    // 	address, _ := c.zeroPageFixed(m)
    // 	newValue := c.staCommon(2, 3)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) staZeroPageIndirectX(m Memory) {
    // 	address, _ := c.zeroPageIndirectX(m)
    // 	newValue := c.staCommon(2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) staZeroPageIndirectY(m Memory) {
    // 	address, _, _ := c.zeroPageIndirectY(m)
    // 	newValue := c.staCommon(2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) staCommon(pcOffset uint16, clock uint64) uint8 {
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return c.A
    // }

    // func (c *CPU) saxAbsolute(m Memory) {
    // 	address, _ := c.absolute(m)
    // 	newValue := c.saxCommon(3, 4)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) saxZeroPage(m Memory) {
    // 	address, _ := c.zeroPageFixed(m)
    // 	newValue := c.saxCommon(2, 3)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) saxZeroPageIndirectX(m Memory) {
    // 	address, _ := c.zeroPageIndirectX(m)
    // 	newValue := c.saxCommon(2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) saxCommon(pcOffset uint16, clock uint64) uint8 {
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return c.A & c.X
    // }

    // func (c *CPU) stxAbsolute(m Memory) {
    // 	address, _ := c.absolute(m)
    // 	newValue := c.stxCommon(3, 4)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) stxZeroPage(m Memory) {
    // 	address, _ := c.zeroPageFixed(m)
    // 	newValue := c.stxCommon(2, 3)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) stxCommon(pcOffset uint16, clock uint64) uint8 {
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return c.X
    // }

    // func (c *CPU) styAbsolute(m Memory) {
    // 	address, _ := c.absolute(m)
    // 	newValue := c.styCommon(3, 4)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) styZeroPage(m Memory) {
    // 	address, _ := c.zeroPageFixed(m)
    // 	newValue := c.styCommon(2, 3)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) styCommon(pcOffset uint16, clock uint64) uint8 {
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return c.Y
    // }

    // func (c *CPU) xaaImmediate(m Memory) {
    // 	value := m.Read(c.PC + 1)
    // 	c.A |= 0xee
    // 	c.A &= c.X
    // 	c.A &= value
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.PC += 2
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) txa() {
    // 	c.A = c.X
    // 	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
    // 	c.setFlagsTo(ZeroFlagMask, c.A == 0)
    // 	c.PC += 1
    // 	c.ClockTime += 2
    // }

    // func (c *CPU) ahxZeroPageIndirectY(m Memory) {
    // 	address, value, _ := c.zeroPageIndirectY(m)
    // 	newValue := c.ahxCommon(m, address, value, 2, 6)
    // 	m.Write(address, newValue)
    // }

    // func (c *CPU) ahxCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) uint8 {
    // 	fmt.Printf("TODO address = %04x\n", address)
    // 	fmt.Printf("TODO value = %02x\n", value)
    // 	fmt.Printf("TODO a = %02x\n", c.A)
    // 	fmt.Printf("TODO x = %02x\n", c.X)
    // 	// newValue := c.A & c.X & uint8((address+1)>>8)
    // 	newValue := c.A & c.X & (uint8(address >> 8))
    // 	fmt.Printf("TODO newValue = %02x\n", newValue)
    // 	// newValue := c.A & c.X & ((value - c.Y) + 1)
    // 	// newValue := c.A & c.X & (value + 1)
    // 	// newValue := c.A & c.X
    // 	// newValue := c.A & c.X & m.Read(uint16(uint8(address>>8)+1))
    // 	c.PC += pcOffset
    // 	c.ClockTime += clock
    // 	return newValue
    // }

    fn nop(&mut self, pc_offset: i8, clock: u64) {
        self.pc = self.pc.wrapping_add(pc_offset as u16);
        self.clock += clock;
    }

    fn nop_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value: _,
            extra_clock,
        } = self.absolute_x(m);
        self.clock += 4 + extra_clock;
    }

    fn push8<M>(&mut self, m: &mut M, value: u8)
    where
        M: Memory,
    {
        m.write8(STACK_ADDRESS + (self.sp as u16), value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn push16<M>(&mut self, m: &mut M, value: u16)
    where
        M: Memory,
    {
        let value: Word = value.into();
        self.push8(m, value.high);
        self.push8(m, value.low);
    }

    fn pop8<M>(&mut self, m: &mut M) -> u8
    where
        M: Memory,
    {
        self.sp = self.sp.wrapping_add(1);
        m.read8(STACK_ADDRESS + (self.sp as u16))
    }

    fn pop16<M>(&mut self, m: &mut M) -> u16
    where
        M: Memory,
    {
        let low = self.pop8(m);
        let high = self.pop8(m);
        Word { low, high }.into()
    }

    fn absolute<M>(&mut self, m: &mut M) -> AddrValue
    where
        M: Memory,
    {
        let address = self.read_next_u16(m);
        let value = m.read8(address);
        AddrValue { address, value }
    }

    fn absolute_x<M>(&mut self, m: &mut M) -> AddrValueClock
    where
        M: Memory,
    {
        self.absolute_common(m, self.x)
    }

    fn absolute_y<M>(&mut self, m: &mut M) -> AddrValueClock
    where
        M: Memory,
    {
        self.absolute_common(m, self.y)
    }

    fn absolute_common<M>(&mut self, m: &mut M, offset: u8) -> AddrValueClock
    where
        M: Memory,
    {
        let address = self.read_next_u16(m);
        let high1 = address & 0xff00;
        let address = address.wrapping_add(offset as u16);
        let value = m.read8(address);
        let high2 = address & 0xff00;
        // if adding Y pushes us into a new page it will take an extra clock cycle to resolve
        let extra_clock = if high1 == high2 { 0 } else { 1 };
        AddrValueClock {
            address,
            value,
            extra_clock,
        }
    }

    fn zero_page_fixed<M>(&mut self, m: &mut M) -> AddrValue
    where
        M: Memory,
    {
        let address = self.read_next_u8(m) as u16;
        let value = m.read8(address);
        AddrValue { address, value }
    }

    fn zero_page_x<M>(&mut self, m: &mut M) -> AddrValue
    where
        M: Memory,
    {
        let address = ((self.read_next_u8(m) as u16) + (self.x as u16)) & 0xff;
        let value = m.read8(address);
        AddrValue { address, value }
    }

    fn zero_page_indirect_x<M>(&mut self, m: &mut M) -> AddrValue
    where
        M: Memory,
    {
        let offset = self.read_next_u8(m);
        let address = self.zero_page_indirect(m, offset, self.x);
        let value = m.read8(address);
        AddrValue { address, value }
    }

    fn zero_page_indirect_y<M>(&mut self, m: &mut M) -> AddrValueClock
    where
        M: Memory,
    {
        let offset = self.read_next_u8(m);
        let address = self.zero_page_indirect(m, offset, 0);
        let high1 = address & 0xff00;
        let address = address.wrapping_add(self.y as u16);
        let value = m.read8(address);
        let high2 = address & 0xff00;
        // if adding Y pushes us into a new page it will take an extra clock cycle to resolve
        let extra_clock = if high1 == high2 { 0 } else { 1 };
        AddrValueClock {
            address,
            value,
            extra_clock,
        }
    }

    fn zero_page_indirect<M>(&mut self, m: &mut M, offset1: u8, offset2: u8) -> u16
    where
        M: Memory,
    {
        let new_offset = (offset1 as u16).wrapping_add(offset2 as u16);
        // address must be on the zero page
        let new_offset_low = new_offset & 0xff;
        let result_low = m.read8(new_offset_low);
        // address must be on the zero page
        let result_high = m.read8(new_offset_low.wrapping_add(1) & 0xff);
        Word {
            low: result_low,
            high: result_high,
        }
        .into()
    }

    fn read_next_u8<M>(&mut self, m: &mut M) -> u8
    where
        M: Memory,
    {
        let result = m.read8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        result
    }

    fn read_next_u16<M>(&mut self, m: &mut M) -> u16
    where
        M: Memory,
    {
        let low = self.read_next_u8(m);
        let high = self.read_next_u8(m);
        Word { low, high }.into()
    }
}

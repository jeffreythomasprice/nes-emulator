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
            0x20 => self.jsr(m),
            0x21 => self.and_zero_page_indirect_x(m),
            0x22 => self.nop(-1, 3),
            0x23 => self.rla_zero_page_indirect_x(m),
            0x24 => self.bit_zero_page_immediate(m),
            0x25 => self.and_zero_page(m),
            0x26 => self.rol_zero_page(m),
            0x27 => self.rla_zero_page(m),
            0x28 => self.plp(m),
            0x29 => self.and_immediate(m),
            0x2a => self.rol(),
            0x2b => self.anc_immediate(m),
            0x2c => self.bit_absolute(m),
            0x2d => self.and_absolute(m),
            0x2e => self.rol_absolute(m),
            0x2f => self.rla_absolute(m),
            0x30 => self.bmi(m),
            0x31 => self.and_zero_page_indirect_y(m),
            0x32 => self.nop(-1, 3),
            0x33 => self.rla_zero_page_indirect_y(m),
            0x34 => self.nop(1, 4),
            0x35 => self.and_zero_page_x(m),
            0x36 => self.rol_zero_page_x(m),
            0x37 => self.rla_zero_page_x(m),
            0x38 => self.sec(),
            0x39 => self.and_absolute_y(m),
            0x3a => self.nop(0, 2),
            0x3b => self.rla_absolute_y(m),
            0x3c => self.nop_absolute_x(m),
            0x3d => self.and_absolute_x(m),
            0x3e => self.rol_absolute_x(m),
            0x3f => self.rla_absolute_x(m),
            0x40 => self.rti(m),
            0x41 => self.eor_zero_page_indirect_x(m),
            0x42 => self.nop(-1, 3),
            0x43 => self.sre_zero_page_indirect_x(m),
            0x44 => self.nop(1, 3),
            0x45 => self.eor_zero_page(m),
            0x46 => self.lsr_zero_page(m),
            0x47 => self.sre_zero_page(m),
            0x48 => self.pha(m),
            0x49 => self.eor_immediate(m),
            0x4a => self.lsr(),
            0x4b => self.alr(m),
            0x4c => self.jmp_absolute(m),
            0x4d => self.eor_absolute(m),
            0x4e => self.lsr_absolute(m),
            0x4f => self.sre_absolute(m),
            0x50 => self.bvc(m),
            0x51 => self.eor_zero_page_indirect_y(m),
            0x52 => self.nop(-1, 3),
            0x53 => self.sre_zero_page_indirect_y(m),
            0x54 => self.nop(1, 4),
            0x55 => self.eor_zero_page_x(m),
            0x56 => self.lsr_zero_page_x(m),
            0x57 => self.sre_zero_page_x(m),
            0x58 => self.cli(),
            0x59 => self.eor_absolute_y(m),
            0x5a => self.nop(0, 2),
            0x5b => self.sre_absolute_y(m),
            0x5c => self.nop_absolute_x(m),
            0x5d => self.eor_absolute_x(m),
            0x5e => self.lsr_absolute_x(m),
            0x5f => self.sre_absolute_x(m),
            0x60 => self.rta(m),
            0x61 => self.adc_zero_page_indirect_x(m),
            0x62 => self.nop(-1, 3),
            0x63 => self.rra_zero_page_indirect_x(m),
            0x64 => self.nop(1, 3),
            0x65 => self.adc_zero_page(m),
            0x66 => self.ror_zero_page(m),
            0x67 => self.rra_zero_page(m),
            0x68 => self.pla(m),
            0x69 => self.adc_immediate(m),
            0x6a => self.ror(),
            0x6b => self.arr_immediate(m),
            0x6c => self.jmp_indirect(m),
            0x6d => self.adc_absolute(m),
            0x6e => self.ror_absolute(m),
            0x6f => self.rra_absolute(m),
            0x70 => self.bvs(m),
            0x71 => self.adc_zero_page_indirect_y(m),
            0x72 => self.nop(-1, 3),
            0x73 => self.rra_zero_page_indirect_y(m),
            0x74 => self.nop(1, 4),
            0x75 => self.adc_zero_page_x(m),
            0x76 => self.ror_zero_page_x(m),
            0x77 => self.rra_zero_page_x(m),
            0x78 => self.sei(),
            0x79 => self.adc_absolute_y(m),
            0x7a => self.nop(0, 2),
            0x7b => self.rra_absolute_y(m),
            0x7c => self.nop_absolute_x(m),
            0x7d => self.adc_absolute_x(m),
            0x7e => self.ror_absolute_x(m),
            0x7f => self.rra_absolute_x(m),
            0x80 => self.nop(1, 2),
            0x81 => self.sta_zero_page_indirect_x(m),
            0x82 => self.nop(1, 2),
            0x83 => self.sax_zero_page_indirect_x(m),
            0x84 => self.sty_zero_page(m),
            0x85 => self.sta_zero_page(m),
            0x86 => self.stx_zero_page(m),
            0x87 => self.sax_zero_page(m),
            0x88 => self.dey(),
            0x89 => self.nop(1, 2),
            0x8a => self.txa(),
            0x8b => self.xaa_immediate(m),
            0x8c => self.sty_absolute(m),
            0x8d => self.sta_absolute(m),
            0x8e => self.stx_absolute(m),
            0x8f => self.sax_aboslute(m),
            0x90 => self.bcc(m),
            0x91 => self.sta_zero_page_indirect_y(m),
            0x92 => self.nop(-1, 3),
            0x93 => self.ahx_zero_page_indirect_y(m),
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
        self.push8(m, (self.flags | Flags::BREAK_COMMAND).bits());
        self.flags.set(Flags::INTERRUPT_DISABLE, true);
        self.pc = m.read16(INTERRUPT_REQUEST_INTERRUPT_ADDRESS);
        self.clock += 7;
    }

    fn php<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.push8(m, (self.flags | Flags::BREAK_COMMAND).bits());
        self.clock += 3;
    }

    fn plp<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.flags = (Flags::from_bits_retain(self.pop8(m)) - Flags::BREAK_COMMAND) | Flags::UNUSED;
        self.clock += 4;
    }

    fn rti<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.flags = (Flags::from_bits_retain(self.pop8(m)) - Flags::BREAK_COMMAND) | Flags::UNUSED;
        self.pc = self.pop16(m);
        self.clock += 6;
    }

    fn rta<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.pc = self.pop16(m) + 1;
        self.clock += 6;
    }

    fn pha<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.push8(m, self.a);
        self.clock += 3;
    }

    fn pla<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.a = self.pop8(m);
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += 4;
    }

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
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
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
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.flags.set(Flags::CARRY, new_value < value);
        self.clock += clock;
    }

    fn asl(&mut self) {
        let value = self.a;
        let new_value = value << 1;
        self.a = new_value;
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, new_value < value);
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
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, new_value < value);
        self.clock += clock;
    }

    fn anc_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        let new_value = self.a & value;
        self.a = new_value;
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, (new_value & 0b1000_0000) != 0);
        self.clock += 2;
    }

    fn and_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.and_common(value, 3);
    }

    fn and_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.and_common(value, 4);
    }

    fn and_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.and_common(value, 6);
    }

    fn and_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.and_common(value, 5 + extra_clock);
    }

    fn and_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.and_common(value, 2);
    }

    fn and_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.and_common(value, 4);
    }

    fn and_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.and_common(value, 4 + extra_clock);
    }

    fn and_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.and_common(value, 4 + extra_clock);
    }

    fn and_common(&mut self, value: u8, clock: u64) {
        self.a &= value;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += clock;
    }

    fn rla_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        self.rla_common(m, address, value, 5);
    }

    fn rla_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        self.rla_common(m, address, value, 6);
    }

    fn rla_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        self.rla_common(m, address, value, 8);
    }

    fn rla_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        self.rla_common(m, address, value, 8);
    }

    fn rla_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        self.rla_common(m, address, value, 6);
    }

    fn rla_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        self.rla_common(m, address, value, 7);
    }

    fn rla_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_y(m);
        self.rla_common(m, address, value, 7);
    }

    fn rla_common<M>(&mut self, m: &mut M, address: u16, value: u8, clock: u64)
    where
        M: Memory,
    {
        let new_value = (value << 1)
            + if self.flags.contains(Flags::CARRY) {
                1
            } else {
                0
            };
        m.write8(address, new_value);
        self.a &= new_value;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.flags.set(Flags::CARRY, (value & 0b1000_0000) != 0);
        self.clock += clock;
    }

    fn rol(&mut self) {
        self.a = self.rol_common(self.a, 2);
    }

    fn rol_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.rol_common(value, 5);
        m.write8(address, new_value);
    }

    fn rol_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.rol_common(value, 6);
        m.write8(address, new_value);
    }

    fn rol_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.rol_common(value, 6);
        m.write8(address, new_value);
    }

    fn rol_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.rol_common(value, 7);
        m.write8(address, new_value);
    }

    fn rol_common(&mut self, value: u8, clock: u64) -> u8 {
        let new_value = (value << 1)
            + if self.flags.contains(Flags::CARRY) {
                1
            } else {
                0
            };
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, (value & 0b1000_0000) != 0);
        self.clock += clock;
        new_value
    }

    fn bpl<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, !self.flags.contains(Flags::NEGATIVE));
    }

    fn bmi<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, self.flags.contains(Flags::NEGATIVE));
    }

    fn bvc<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, !self.flags.contains(Flags::OVERFLOW));
    }

    fn bvs<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, self.flags.contains(Flags::OVERFLOW));
    }

    fn bcc<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, !self.flags.contains(Flags::CARRY));
    }

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

    fn jmp_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.pc = self.read_next_u16(m);
        self.clock += 3;
    }

    fn jmp_indirect<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let address = self.read_next_u16(m);
        self.pc = if address & 0xff == 0x0ff {
            let low = m.read8(address);
            let high = m.read8(address & 0xff00);
            Word { low, high }.into()
        } else {
            m.read16(address)
        };
        self.clock += 5;
    }

    fn clc(&mut self) {
        self.flags -= Flags::CARRY;
        self.clock += 2
    }

    fn sec(&mut self) {
        self.flags |= Flags::CARRY;
        self.clock += 2;
    }

    fn cli(&mut self) {
        self.flags -= Flags::INTERRUPT_DISABLE;
        self.clock += 2;
    }

    fn sei(&mut self) {
        self.flags |= Flags::INTERRUPT_DISABLE;
        self.clock += 2;
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.flags.set(Flags::NEGATIVE, (self.y as i8) < 0);
        self.flags.set(Flags::ZERO, self.y == 0);
        self.clock += 2;
    }

    fn jsr<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let address = self.read_next_u16(m);
        self.push16(m, self.pc.wrapping_sub(1));
        self.pc = address;
        self.clock += 6;
    }

    fn bit_zero_page_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.bit_common(value, 3);
    }

    fn bit_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.bit_common(value, 4);
    }

    fn bit_common(&mut self, value: u8, clock: u64) {
        self.flags
            .set(Flags::OVERFLOW, (value & Flags::OVERFLOW.bits()) != 0);
        self.flags
            .set(Flags::NEGATIVE, (value & Flags::NEGATIVE.bits()) != 0);
        self.flags.set(Flags::ZERO, (value & self.a) == 0);
        self.clock += clock;
    }

    fn eor_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.eor_common(value, 3);
    }

    fn eor_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.eor_common(value, 4);
    }

    fn eor_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.eor_common(value, 6);
    }

    fn eor_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.eor_common(value, 5 + extra_clock);
    }

    fn eor_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.eor_common(value, 2);
    }

    fn eor_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.eor_common(value, 4);
    }

    fn eor_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.eor_common(value, 4 + extra_clock);
    }

    fn eor_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.eor_common(value, 4 + extra_clock);
    }

    fn eor_common(&mut self, value: u8, clock: u64) {
        self.a ^= value;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += clock;
    }

    fn sre_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        self.sre_common(m, address, value, 5);
    }

    fn sre_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        self.sre_common(m, address, value, 6);
    }

    fn sre_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        self.sre_common(m, address, value, 8);
    }

    fn sre_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        self.sre_common(m, address, value, 8);
    }

    fn sre_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        self.sre_common(m, address, value, 6);
    }

    fn sre_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        self.sre_common(m, address, value, 7);
    }

    fn sre_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_y(m);
        self.sre_common(m, address, value, 7);
    }

    fn sre_common<M>(&mut self, m: &mut M, address: u16, value: u8, clock: u64)
    where
        M: Memory,
    {
        let new_value = value >> 1;
        m.write8(address, new_value);
        self.a ^= new_value;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.flags.set(Flags::CARRY, (value & 0b0000_0001) != 0);
        self.clock += clock;
    }

    fn lsr(&mut self) {
        self.a = self.lsr_common(self.a, 2);
    }

    fn lsr_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.lsr_common(value, 5);
        m.write8(address, new_value);
    }

    fn lsr_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.lsr_common(value, 6);
        m.write8(address, new_value);
    }

    fn lsr_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.lsr_common(value, 6);
        m.write8(address, new_value);
    }

    fn lsr_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.lsr_common(value, 7);
        m.write8(address, new_value);
    }

    fn lsr_common(&mut self, value: u8, clock: u64) -> u8 {
        let new_value = value >> 1;
        self.flags -= Flags::NEGATIVE;
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, (value & 0b0000_0001) != 0);
        self.clock += clock;
        new_value
    }

    fn alr<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.a & self.read_next_u8(m);
        self.a = value >> 1;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.flags.set(Flags::CARRY, (value & 0b0000_0001) != 0);
        self.clock += 2;
    }

    fn adc_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.adc_common(value, 2);
    }

    fn adc_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.adc_common(value, 4);
    }

    fn adc_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.adc_common(value, 4 + extra_clock);
    }

    fn adc_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.adc_common(value, 4 + extra_clock);
    }

    fn adc_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.adc_common(value, 3);
    }

    fn adc_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.adc_common(value, 4);
    }

    fn adc_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.adc_common(value, 6);
    }

    fn adc_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.adc_common(value, 5 + extra_clock);
    }

    fn adc_common(&mut self, value: u8, clock: u64) {
        let new_value = (self.a as u16).wrapping_add(value as u16).wrapping_add(
            if self.flags.contains(Flags::CARRY) {
                1
            } else {
                0
            },
        );
        let old_a = self.a;
        self.a = new_value as u8;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.flags
            .set(Flags::CARRY, (new_value & 0b1_0000_0000) != 0);
        let value_sign_bit = value & 0b1000_0000;
        let old_a_sign_bit = old_a & 0b1000_0000;
        let new_sign_bit = (new_value & 0b1000_0000) as u8;
        self.flags.set(
            Flags::OVERFLOW,
            value_sign_bit == old_a_sign_bit && value_sign_bit != new_sign_bit,
        );
        self.clock += clock;
    }

    fn rra_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.rra_common(value, 6);
        m.write8(address, new_value);
    }

    fn rra_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.rra_common(value, 7);
        m.write8(address, new_value);
    }

    fn rra_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_y(m);
        let new_value = self.rra_common(value, 7);
        m.write8(address, new_value);
    }

    fn rra_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.rra_common(value, 5);
        m.write8(address, new_value);
    }

    fn rra_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.rra_common(value, 6);
        m.write8(address, new_value);
    }

    fn rra_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        let new_value = self.rra_common(value, 8);
        m.write8(address, new_value);
    }

    fn rra_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        let new_value = self.rra_common(value, 8);
        m.write8(address, new_value);
    }

    fn rra_common(&mut self, value: u8, clock: u64) -> u8 {
        let ror_new_value = (value >> 1)
            | if self.flags.contains(Flags::CARRY) {
                0b1000_0000
            } else {
                0
            };
        let adc_new_value = (self.a as u16)
            .wrapping_add(ror_new_value as u16)
            // carry flag check, but what the carry flag should be after the previous ROR
            .wrapping_add(if (value & 1) != 0 { 1 } else { 0 });
        let old_a = self.a;
        self.a = adc_new_value as u8;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.flags
            .set(Flags::CARRY, (adc_new_value & 0b1_0000_0000) != 0);
        let value_sign_bit = ror_new_value & 0b1000_0000;
        let old_a_sign_bit = old_a & 0b1000_0000;
        let new_sign_bit = (adc_new_value & 0b1000_0000) as u8;
        self.flags.set(
            Flags::OVERFLOW,
            value_sign_bit == old_a_sign_bit && value_sign_bit != new_sign_bit,
        );
        self.clock += clock;
        ror_new_value
    }

    fn ror(&mut self) {
        self.a = self.ror_common(self.a, 2);
    }

    fn ror_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.ror_common(value, 6);
        m.write8(address, new_value);
    }

    fn ror_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.ror_common(value, 7);
        m.write8(address, new_value);
    }

    fn ror_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.ror_common(value, 5);
        m.write8(address, new_value);
    }

    fn ror_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.ror_common(value, 6);
        m.write8(address, new_value);
    }

    fn ror_common(&mut self, value: u8, clock: u64) -> u8 {
        let new_value = (value >> 1)
            | if self.flags.contains(Flags::CARRY) {
                0b1000_0000
            } else {
                0
            };
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, (value & 1) != 0);
        self.clock += clock;
        new_value
    }

    fn arr_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let imm_value = self.read_next_u8(m);
        let new_value = self.a & imm_value;
        self.flags
            .set(Flags::OVERFLOW, (new_value ^ (new_value >> 1)) & 0x40 != 0);
        let new_carry = new_value & 0b1000_0000;
        let new_value = (new_value >> 1)
            | if self.flags.contains(Flags::CARRY) {
                0b1000_0000
            } else {
                0
            };
        self.a = new_value;
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, new_carry != 0);
        self.clock += 2;
    }

    fn sta_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.absolute(m);
        let new_value = self.sta_common(4);
        m.write8(address, new_value);
    }

    fn sta_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_fixed(m);
        let new_value = self.sta_common(3);
        m.write8(address, new_value);
    }

    fn sta_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_indirect_x(m);
        let new_value = self.sta_common(6);
        m.write8(address, new_value);
    }

    fn sta_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value: _,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        let new_value = self.sta_common(6);
        m.write8(address, new_value);
    }

    fn sta_common(&mut self, clock: u64) -> u8 {
        self.clock += clock;
        self.a
    }

    fn sax_aboslute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.absolute(m);
        let new_value = self.sax_common(4);
        m.write8(address, new_value);
    }

    fn sax_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_fixed(m);
        let new_value = self.sax_common(3);
        m.write8(address, new_value);
    }

    fn sax_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_indirect_x(m);
        let new_value = self.sax_common(6);
        m.write8(address, new_value);
    }

    fn sax_common(&mut self, clock: u64) -> u8 {
        self.clock += clock;
        self.a & self.x
    }

    fn stx_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.absolute(m);
        let new_value = self.stx_common(4);
        m.write8(address, new_value);
    }

    fn stx_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_fixed(m);
        let new_value = self.stx_common(3);
        m.write8(address, new_value);
    }

    fn stx_common(&mut self, clock: u64) -> u8 {
        self.clock += clock;
        self.x
    }

    fn sty_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.absolute(m);
        let new_value = self.sty_common(4);
        m.write8(address, new_value);
    }

    fn sty_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_fixed(m);
        let new_value = self.sty_common(3);
        m.write8(address, new_value);
    }

    fn sty_common(&mut self, clock: u64) -> u8 {
        self.clock += clock;
        self.y
    }

    fn xaa_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.a |= 0xee;
        self.a &= self.x;
        self.a &= value;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += 2;
    }

    fn txa(&mut self) {
        self.a = self.x;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += 2;
    }

    fn ahx_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        let new_value = self.ahx_common(m, address, value, 6, extra_clock != 0);
        let mut address: Word = address.into();
        if extra_clock != 0 {
            address.high = new_value;
        }
        let address: u16 = address.into();
        m.write8(address, new_value);
    }

    fn ahx_common<M>(
        &mut self,
        m: &mut M,
        address: u16,
        value: u8,
        clock: u64,
        page_crossing: bool,
    ) -> u8 {
        let address: Word = address.into();
        let value = self.a
            & self.x
            & if !page_crossing {
                address.high.wrapping_add(1)
            } else {
                address.high
            };
        self.clock += clock;
        value
    }

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

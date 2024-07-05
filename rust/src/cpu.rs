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

impl AddrValueClock {
    pub fn is_page_crossing(&self) -> bool {
        self.extra_clock != 0
    }
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
            0x02 => self.kil(),
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
            0x12 => self.kil(),
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
            0x22 => self.kil(),
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
            0x32 => self.kil(),
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
            0x42 => self.kil(),
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
            0x52 => self.kil(),
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
            0x62 => self.kil(),
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
            0x72 => self.kil(),
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
            0x92 => self.kil(),
            0x93 => self.ahx_zero_page_indirect_y(m),
            0x94 => self.sty_zero_page_x(m),
            0x95 => self.sta_zero_page_x(m),
            0x96 => self.stx_zero_page_y(m),
            0x97 => self.sax_zero_page_y(m),
            0x98 => self.tya(),
            0x99 => self.sta_absolute_y(m),
            0x9a => self.txs(),
            0x9b => self.tas_absolute_y(m),
            0x9c => self.shy_absolute_x(m),
            0x9d => self.sta_absolute_x(m),
            0x9e => self.shx_absolute_y(m),
            0x9f => self.ahx_absolute_y(m),
            0xa0 => self.ldy_immediate(m),
            0xa1 => self.lda_zero_page_indirect_x(m),
            0xa2 => self.ldx_immediate(m),
            0xa3 => self.lax_zero_page_indirect_x(m),
            0xa4 => self.ldy_zero_page(m),
            0xa5 => self.lda_zero_page(m),
            0xa6 => self.ldx_zero_page(m),
            0xa7 => self.lax_zero_page(m),
            0xa8 => self.tay(),
            0xa9 => self.lda_immediate(m),
            0xaa => self.tax(),
            0xab => self.lax_immediate(m),
            0xac => self.ldy_absolute(m),
            0xad => self.lda_absolute(m),
            0xae => self.ldx_absolute(m),
            0xaf => self.lax_absolute(m),
            0xb0 => self.bcs(m),
            0xb1 => self.lda_zero_page_indirect_y(m),
            0xb2 => self.kil(),
            0xb3 => self.lax_zero_page_indirect_y(m),
            0xb4 => self.ldy_zero_page_x(m),
            0xb5 => self.lda_zero_page_x(m),
            0xb6 => self.ldx_zero_page_y(m),
            0xb7 => self.lax_zero_page_y(m),
            0xb8 => self.clv(),
            0xb9 => self.lda_absolute_y(m),
            0xba => self.tsx(),
            0xbb => self.las_absolute_y(m),
            0xbc => self.ldy_absolute_x(m),
            0xbd => self.lda_absolute_x(m),
            0xbe => self.ldx_absolute_y(m),
            0xbf => self.lax_absolute_y(m),
            0xc0 => self.cpy_immediate(m),
            0xc1 => self.cmp_zero_page_indirect_x(m),
            0xc2 => self.nop(1, 2),
            0xc3 => self.dcp_zero_page_indirect_x(m),
            0xc4 => self.cpy_zero_page(m),
            0xc5 => self.cmp_zero_page(m),
            0xc6 => self.dec_zero_page(m),
            0xc7 => self.dcp_zero_page(m),
            0xc8 => self.iny(),
            0xc9 => self.cmp_immediate(m),
            0xca => self.dex(),
            0xcb => self.axs_immediate(m),
            0xcc => self.cpy_absolute(m),
            0xcd => self.cmp_absolute(m),
            0xce => self.dec_absolute(m),
            0xcf => self.dcp_absolute(m),
            0xd0 => self.bne(m),
            0xd1 => self.cmp_zero_page_indirect_y(m),
            0xd2 => self.kil(),
            0xd3 => self.dcp_zero_page_indirect_y(m),
            0xd4 => self.nop(1, 4),
            0xd5 => self.cmp_zero_page_x(m),
            0xd6 => self.dec_zero_page_x(m),
            0xd7 => self.dcp_zero_page_x(m),
            0xd8 => self.cld(),
            0xd9 => self.cmp_absolute_y(m),
            0xda => self.nop(0, 2),
            0xdb => self.dcp_absolute_y(m),
            0xdc => self.nop_absolute_x(m),
            0xdd => self.cmp_absolute_x(m),
            0xde => self.dec_absolute_x(m),
            0xdf => self.dcp_absolute_x(m),
            0xe0 => self.cpx_immediate(m),
            0xe1 => self.sbc_zero_page_indirect_x(m),
            0xe2 => self.nop(1, 2),
            0xe3 => self.isc_zero_page_indirect_x(m),
            0xe4 => self.cpx_zerp_page(m),
            0xe5 => self.sbc_zero_page(m),
            0xe6 => self.inc_zero_page(m),
            0xe7 => self.isc_zero_page(m),
            0xe8 => self.inx(),
            0xe9 => self.sbc_immediate(m),
            0xea => self.nop(0, 2),
            0xeb => self.sbc_immediate(m),
            0xec => self.cpx_absolute(m),
            0xed => self.sbc_absolute(m),
            0xee => self.inc_absolute(m),
            0xef => self.isc_absolute(m),
            0xf0 => self.beq(m),
            0xf1 => self.sbc_zero_page_indirect_y(m),
            0xf2 => self.kil(),
            0xf3 => self.isc_zero_page_indirect_y(m),
            0xf4 => self.nop(1, 4),
            0xf5 => self.sbc_zero_page_x(m),
            0xf6 => self.inc_zero_page_x(m),
            0xf7 => self.isc_zero_page_x(m),
            0xf8 => self.sed(),
            0xf9 => self.sbc_absolute_y(m),
            0xfa => self.nop(0, 2),
            0xfb => self.isc_absolute_y(m),
            0xfc => self.nop_absolute_x(m),
            0xfd => self.sbc_absolute_x(m),
            0xfe => self.inc_absolute_x(m),
            0xff => self.isc_absolute_x(m),
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

    fn bcs<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, self.flags.contains(Flags::CARRY));
    }

    fn bne<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, !self.flags.contains(Flags::ZERO));
    }

    fn beq<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        self.branch_common(m, self.flags.contains(Flags::ZERO));
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

    fn tya(&mut self) {
        self.a = self.y;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += 2;
    }

    fn tay(&mut self) {
        self.y = self.a;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += 2;
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.flags.set(Flags::NEGATIVE, (self.a as i8) < 0);
        self.flags.set(Flags::ZERO, self.a == 0);
        self.clock += 2;
    }

    fn clv(&mut self) {
        self.flags -= Flags::OVERFLOW;
        self.clock += 2;
    }

    fn tsx(&mut self) {
        self.x = self.sp;
        self.flags.set(Flags::NEGATIVE, (self.x as i8) < 0);
        self.flags.set(Flags::ZERO, self.x == 0);
        self.clock += 2;
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.flags.set(Flags::NEGATIVE, (self.y as i8) < 0);
        self.flags.set(Flags::ZERO, self.y == 0);
        self.clock += 2;
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.flags.set(Flags::NEGATIVE, (self.x as i8) < 0);
        self.flags.set(Flags::ZERO, self.x == 0);
        self.clock += 2;
    }

    fn cld(&mut self) {
        self.flags -= Flags::DECIMAL_MODE;
        self.clock += 2;
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.flags.set(Flags::NEGATIVE, (self.x as i8) < 0);
        self.flags.set(Flags::ZERO, self.x == 0);
        self.clock += 2;
    }

    fn sed(&mut self) {
        self.flags |= Flags::DECIMAL_MODE;
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

    fn sta_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value: _,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.sta_common(5);
        m.write8(address, new_value);
    }

    fn sta_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value: _,
            extra_clock: _,
        } = self.absolute_y(m);
        let new_value = self.sta_common(5);
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

    fn sta_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_x(m);
        let new_value = self.sta_common(4);
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

    fn sax_zero_page_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_y(m);
        let new_value = self.sax_common(4);
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

    fn stx_zero_page_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_y(m);
        let new_value = self.stx_common(4);
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

    fn sty_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value: _ } = self.zero_page_x(m);
        let new_value = self.sty_common(4);
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

    fn txs(&mut self) {
        self.sp = self.x;
        self.clock += 2;
    }

    fn ahx_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let input = self.absolute_y(m);
        self.ahx_common(m, input, 5);
    }

    fn ahx_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let input = self.zero_page_indirect_y(m);
        self.ahx_common(m, input, 6);
    }

    fn ahx_common<M>(&mut self, m: &mut M, input: AddrValueClock, clock: u64)
    where
        M: Memory,
    {
        let address: Word = input.address.into();
        let value = self.a
            & self.x
            & if !input.is_page_crossing() {
                address.high.wrapping_add(1)
            } else {
                address.high
            };
        let address = if input.is_page_crossing() {
            Word {
                high: value,
                low: address.low,
            }
        } else {
            address
        };
        m.write8(address.into(), value);
        self.clock += clock;
    }

    fn tas_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let input = self.absolute_y(m);
        let address: Word = input.address.into();
        let value = self.a
            & self.x
            & if !input.is_page_crossing() {
                address.high.wrapping_add(1)
            } else {
                address.high
            };
        let address = if input.is_page_crossing() {
            Word {
                high: value,
                low: address.low,
            }
        } else {
            address
        };
        self.sp = self.a & self.x;
        m.write8(address.into(), value);
        self.clock += 5;
    }

    fn shy_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let input = self.absolute_x(m);
        let address: Word = input.address.into();
        let value = self.y
            & if !input.is_page_crossing() {
                address.high.wrapping_add(1)
            } else {
                address.high
            };
        let address = if input.is_page_crossing() {
            Word {
                high: value,
                low: address.low,
            }
        } else {
            address
        };
        m.write8(address.into(), value);
        self.clock += 5;
    }

    fn shx_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let input = self.absolute_y(m);
        let address: Word = input.address.into();
        let value = self.x
            & if !input.is_page_crossing() {
                address.high.wrapping_add(1)
            } else {
                address.high
            };
        let address = if input.is_page_crossing() {
            Word {
                high: value,
                low: address.low,
            }
        } else {
            address
        };
        m.write8(address.into(), value);
        self.clock += 5;
    }

    fn ldy_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.ldy_common(value, 2);
    }

    fn ldy_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.ldy_common(value, 4);
    }

    fn ldy_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.ldy_common(value, 4 + extra_clock);
    }

    fn ldy_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.ldy_common(value, 3);
    }

    fn ldy_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.ldy_common(value, 4);
    }

    fn ldy_common(&mut self, value: u8, clock: u64) {
        self.y = value;
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.clock += clock;
    }

    fn lda_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.lda_common(value, 2);
    }

    fn lda_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.lda_common(value, 4);
    }

    fn lda_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.lda_common(value, 4 + extra_clock);
    }

    fn lda_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.lda_common(value, 4 + extra_clock);
    }

    fn lda_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.lda_common(value, 3);
    }

    fn lda_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.lda_common(value, 4);
    }

    fn lda_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.lda_common(value, 6);
    }

    fn lda_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.lda_common(value, 5 + extra_clock);
    }

    fn lda_common(&mut self, value: u8, clock: u64) {
        self.a = value;
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.clock += clock;
    }

    fn ldx_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.ldx_common(value, 2);
    }

    fn ldx_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.ldx_common(value, 4);
    }

    fn ldx_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.ldx_common(value, 4 + extra_clock);
    }

    fn ldx_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.ldx_common(value, 3);
    }

    fn ldx_zero_page_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_y(m);
        self.ldx_common(value, 4);
    }

    fn ldx_common(&mut self, value: u8, clock: u64) {
        self.x = value;
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.clock += clock;
    }

    fn lax_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        let value = (self.a | 0xee) & value;
        self.lax_common(value, 2);
    }

    fn lax_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.lax_common(value, 4);
    }

    fn lax_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.lax_common(value, 4 + extra_clock);
    }

    fn lax_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.lax_common(value, 3);
    }

    fn lax_zero_page_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_y(m);
        self.lax_common(value, 4);
    }

    fn lax_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        self.lax_common(value, 6);
    }

    fn lax_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.lax_common(value, 5 + extra_clock);
    }

    fn lax_common(&mut self, value: u8, clock: u64) {
        self.a = value;
        self.x = value;
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.clock += clock;
    }

    fn las_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock,
        } = self.absolute_y(m);
        let value = value & self.sp;
        self.a = value;
        self.x = value;
        self.sp = value;
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.clock += 4 + extra_clock;
    }

    fn cpy_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.cpy_common(value, 2);
    }

    fn cpy_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.cpy_common(value, 4);
    }

    fn cpy_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        self.cpy_common(value, 3);
    }

    fn cpy_common(&mut self, value: u8, clock: u64) {
        let value = self.y.wrapping_sub(value);
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.flags.set(Flags::CARRY, self.y >= value);
        self.clock += clock;
    }

    fn cmp_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.cmp_common(value, 2);
    }

    fn cmp_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.cmp_common(value, 4);
    }

    fn cmp_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.cmp_common(value, 4 + extra_clock);
    }

    fn cmp_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.cmp_common(value, 4 + extra_clock);
    }

    fn cmp_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.cmp_common(value, 3);
    }

    fn cmp_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.cmp_common(value, 4);
    }

    fn cmp_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.cmp_common(value, 6);
    }

    fn cmp_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.cmp_common(value, 5 + extra_clock);
    }

    fn cmp_common(&mut self, value: u8, clock: u64) {
        let value = self.a.wrapping_sub(value);
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.flags.set(Flags::CARRY, self.a >= value);
        self.clock += clock;
    }

    fn dcp_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.dcp_common(value, 6);
        m.write8(address, new_value);
    }

    fn dcp_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.dcp_common(value, 7);
        m.write8(address, new_value);
    }

    fn dcp_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_y(m);
        let new_value = self.dcp_common(value, 7);
        m.write8(address, new_value);
    }

    fn dcp_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.dcp_common(value, 5);
        m.write8(address, new_value);
    }

    fn dcp_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.dcp_common(value, 6);
        m.write8(address, new_value);
    }

    fn dcp_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        let new_value = self.dcp_common(value, 8);
        m.write8(address, new_value);
    }

    fn dcp_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        let new_value = self.dcp_common(value, 8);
        m.write8(address, new_value);
    }

    fn dcp_common(&mut self, value: u8, clock: u64) -> u8 {
        let dec_value = value.wrapping_sub(1);
        let cmp_value = self.a.wrapping_sub(dec_value);
        self.flags.set(Flags::NEGATIVE, (cmp_value as i8) < 0);
        self.flags.set(Flags::ZERO, cmp_value == 0);
        self.flags.set(Flags::CARRY, self.a >= cmp_value);
        self.clock += clock;
        dec_value
    }

    fn dec_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.dec_common(value, 6);
        m.write8(address, new_value);
    }

    fn dec_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.dec_common(value, 7);
        m.write8(address, new_value);
    }

    fn dec_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.dec_common(value, 5);
        m.write8(address, new_value);
    }

    fn dec_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.dec_common(value, 6);
        m.write8(address, new_value);
    }

    fn dec_common(&mut self, value: u8, clock: u64) -> u8 {
        let value = value.wrapping_sub(1);
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.clock += clock;
        value
    }

    fn axs_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let left = self.a & self.x;
        let right = self.read_next_u8(m);
        self.x = left.wrapping_sub(right);
        self.flags.set(Flags::NEGATIVE, (self.x as i8) < 0);
        self.flags.set(Flags::ZERO, self.x == 0);
        self.flags.set(Flags::CARRY, left >= right);
        self.clock += 2;
    }

    fn cpx_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.cpx_common(value, 2);
    }

    fn cpx_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.cpx_common(value, 4);
    }

    fn cpx_zerp_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.cpx_common(value, 3);
    }

    fn cpx_common(&mut self, value: u8, clock: u64) {
        let value = self.x.wrapping_sub(value);
        self.flags.set(Flags::NEGATIVE, (value as i8) < 0);
        self.flags.set(Flags::ZERO, value == 0);
        self.flags.set(Flags::CARRY, self.x >= value);
        self.clock += clock;
    }

    fn sbc_immediate<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let value = self.read_next_u8(m);
        self.sbc_common(value, 2);
    }

    fn sbc_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.absolute(m);
        self.sbc_common(value, 4);
    }

    fn sbc_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_x(m);
        self.sbc_common(value, 4 + extra_clock);
    }

    fn sbc_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.absolute_y(m);
        self.sbc_common(value, 4 + extra_clock);
    }

    fn sbc_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_fixed(m);
        self.sbc_common(value, 3);
    }

    fn sbc_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_x(m);
        self.sbc_common(value, 4);
    }

    fn sbc_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address: _, value } = self.zero_page_indirect_x(m);
        self.sbc_common(value, 6);
    }

    fn sbc_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address: _,
            value,
            extra_clock,
        } = self.zero_page_indirect_y(m);
        self.sbc_common(value, 5 + extra_clock);
    }

    fn sbc_common(&mut self, value: u8, clock: u64) {
        let old_a = self.a;
        let (new_value, overflow1) = self.a.overflowing_sub(value);
        let (new_value, overflow2) =
            new_value.overflowing_sub(if self.flags.contains(Flags::CARRY) {
                0
            } else {
                1
            });
        self.a = new_value;
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::CARRY, !overflow1 && !overflow2);
        self.flags.set(
            Flags::OVERFLOW,
            (old_a ^ value) & 0x80 != 0 && (old_a ^ new_value) & 0x80 != 0,
        );
        self.clock += clock;
    }

    fn isc_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.isc_common(value, 6);
        m.write8(address, new_value);
    }

    fn isc_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.isc_common(value, 7);
        m.write8(address, new_value);
    }

    fn isc_absolute_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_y(m);
        let new_value = self.isc_common(value, 7);
        m.write8(address, new_value);
    }

    fn isc_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.isc_common(value, 5);
        m.write8(address, new_value);
    }

    fn isc_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.isc_common(value, 6);
        m.write8(address, new_value);
    }

    fn isc_zero_page_indirect_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_indirect_x(m);
        let new_value = self.isc_common(value, 8);
        m.write8(address, new_value);
    }

    fn isc_zero_page_indirect_y<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.zero_page_indirect_y(m);
        let new_value = self.isc_common(value, 8);
        m.write8(address, new_value);
    }

    fn isc_common(&mut self, value: u8, clock: u64) -> u8 {
        let inc_value = value.wrapping_add(1);
        let (sub_value, overflow1) = self.a.overflowing_sub(inc_value);
        let (sub_value, overflow2) =
            sub_value.overflowing_sub(if self.flags.contains(Flags::CARRY) {
                0
            } else {
                1
            });
        let old_a = self.a;
        self.a = sub_value;
        self.flags.set(Flags::NEGATIVE, (sub_value as i8) < 0);
        self.flags.set(Flags::ZERO, sub_value == 0);
        self.flags.set(Flags::CARRY, !overflow1 && !overflow2);
        self.flags.set(
            Flags::OVERFLOW,
            (old_a ^ inc_value) & 0x80 != 0 && (old_a ^ sub_value) & 0x80 != 0,
        );
        self.clock += clock;
        inc_value
    }

    fn inc_absolute<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.absolute(m);
        let new_value = self.inc_common(value, 6);
        m.write8(address, new_value);
    }

    fn inc_absolute_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValueClock {
            address,
            value,
            extra_clock: _,
        } = self.absolute_x(m);
        let new_value = self.inc_common(value, 7);
        m.write8(address, new_value);
    }

    fn inc_zero_page<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_fixed(m);
        let new_value = self.inc_common(value, 5);
        m.write8(address, new_value);
    }

    fn inc_zero_page_x<M>(&mut self, m: &mut M)
    where
        M: Memory,
    {
        let AddrValue { address, value } = self.zero_page_x(m);
        let new_value = self.inc_common(value, 6);
        m.write8(address, new_value);
    }

    fn inc_common(&mut self, value: u8, clock: u64) -> u8 {
        let new_value = value.wrapping_add(1);
        self.flags.set(Flags::NEGATIVE, (new_value as i8) < 0);
        self.flags.set(Flags::ZERO, new_value == 0);
        self.clock += clock;
        new_value
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

    fn kil(&mut self) {
        self.nop(-1, 3);
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

    fn zero_page_y<M>(&mut self, m: &mut M) -> AddrValue
    where
        M: Memory,
    {
        let address = ((self.read_next_u8(m) as u16) + (self.y as u16)) & 0xff;
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

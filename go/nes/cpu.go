package nes

const (
	CarryFlagMask            uint8 = 0b0000_0001
	ZeroFlagMask             uint8 = 0b0000_0010
	InterruptDisableFlagMask uint8 = 0b0000_0100
	DecimalModeFlagMask      uint8 = 0b0000_1000
	BreakCommandFlagMask     uint8 = 0b0001_0000
	UnusedFlagMask           uint8 = 0b0010_0000
	OverflowFlagMask         uint8 = 0b0100_0000
	NegativeFlagMask         uint8 = 0b1000_0000
)

const (
	StackAddress                     Word = 0x0100
	NonMaskableInterruptAddress      Word = 0xfffa
	ResetInterruptAddress            Word = 0xfffc
	InterruptRequestInterruptAddress Word = 0xfffe
)

type CPU struct {
	PC        Word
	SP        uint8
	A         uint8
	X         uint8
	Y         uint8
	Flags     uint8
	ClockTime uint64
}

type addrValue struct {
	address Word
	value   uint8
}

type addrValueClock struct {
	address    Word
	value      uint8
	extraClock uint64
}

func (a addrValueClock) IsPageCrossing() bool {
	return a.extraClock > 0
}

func (c *CPU) Step(m Memory) {
	instruction := m.Read(c.PC)
	switch instruction {
	case 0x00:
		c.brk(m)
	case 0x01:
		c.oraZeroPageIndirectX(m)
	case 0x02:
		c.nop(0, 3)
	case 0x03:
		c.sloZeroPageIndirectX(m)
	case 0x04:
		c.nop(2, 3)
	case 0x05:
		c.oraZeroPageFixed(m)
	case 0x06:
		c.aslZeroPageFixed(m)
	case 0x07:
		c.sloZeroPageImmediate(m)
	case 0x08:
		c.php(m)
	case 0x09:
		c.oraImmediate(m)
	case 0x0a:
		c.asl()
	case 0x0b:
		c.ancImmediate(m)
	case 0x0c:
		c.nop(3, 4)
	case 0x0d:
		c.oraAbsolute(m)
	case 0x0e:
		c.aslAbsolute(m)
	case 0x0f:
		c.sloAbsolute(m)
	case 0x10:
		c.bpl(m)
	case 0x11:
		c.oraZeroPageIndirectY(m)
	case 0x12:
		c.nop(0, 3)
	case 0x13:
		c.sloZeroPageIndirectY(m)
	case 0x14:
		c.nop(2, 4)
	case 0x15:
		c.oraZeroPageX(m)
	case 0x16:
		c.aslZeroPageX(m)
	case 0x17:
		c.sloZeroPageX(m)
	case 0x18:
		c.clc()
	case 0x19:
		c.oraAbsoluteY(m)
	case 0x1a:
		c.nop(1, 2)
	case 0x1b:
		c.sloAbsoluteY(m)
	case 0x1c:
		c.nopAbsoluteX(m)
	case 0x1d:
		c.oraAbsoluteX(m)
	case 0x1e:
		c.aslAbsoluteX(m)
	case 0x1f:
		c.sloAbsoluteX(m)
	case 0x20:
		c.jsr(m)
	case 0x21:
		c.andZeroPageIndirectX(m)
	case 0x22:
		c.nop(0, 3)
	case 0x23:
		c.rlaZeroPageIndirectX(m)
	case 0x24:
		c.bitZeroPageImmediate(m)
	case 0x25:
		c.andZeroPage(m)
	case 0x26:
		c.rolZeroPage(m)
	case 0x27:
		c.rlaZeroPage(m)
	case 0x28:
		c.plp(m)
	case 0x29:
		c.andImmediate(m)
	case 0x2a:
		c.rol()
	case 0x2b:
		c.ancImmediate(m)
	case 0x2c:
		c.bitAbsolute(m)
	case 0x2d:
		c.andAbsolute(m)
	case 0x2e:
		c.rolAbsolute(m)
	case 0x2f:
		c.rlaAbsolute(m)
	case 0x30:
		c.bmi(m)
	case 0x31:
		c.andZeroPageIndirectY(m)
	case 0x32:
		c.nop(0, 3)
	case 0x33:
		c.rlaZeroPageIndirectY(m)
	case 0x34:
		c.nop(2, 4)
	case 0x35:
		c.andZeroPageX(m)
	case 0x36:
		c.rolZeroPageX(m)
	case 0x37:
		c.rlaZeroPageX(m)
	case 0x38:
		c.sec()
	case 0x39:
		c.andAbsoluteY(m)
	case 0x3a:
		c.nop(1, 2)
	case 0x3b:
		c.rlaAbsoluteY(m)
	case 0x3c:
		c.nopAbsoluteX(m)
	case 0x3d:
		c.andAbsoluteX(m)
	case 0x3e:
		c.rolAbsoluteX(m)
	case 0x3f:
		c.rlaAbsoluteX(m)
	case 0x40:
		c.rti(m)
	case 0x41:
		c.eorZeroPageIndirectX(m)
	case 0x42:
		c.nop(0, 3)
	case 0x43:
		c.sreZeroPageIndirectX(m)
	case 0x44:
		c.nop(2, 3)
	case 0x45:
		c.eorZeroPage(m)
	case 0x46:
		c.lsrZeroPage(m)
	case 0x47:
		c.sreZeroPage(m)
	case 0x48:
		c.pha(m)
	case 0x49:
		c.eorImmediate(m)
	case 0x4a:
		c.lsr()
	case 0x4b:
		c.alr(m)
	case 0x4c:
		c.jmpAbsolute(m)
	case 0x4d:
		c.eorAbsolute(m)
	case 0x4e:
		c.lsrAbsolute(m)
	case 0x4f:
		c.sreAbsolute(m)
	case 0x50:
		c.bvc(m)
	case 0x51:
		c.eorZeroPageIndirectY(m)
	case 0x52:
		c.nop(0, 3)
	case 0x53:
		c.sreZeroPageIndirectY(m)
	case 0x54:
		c.nop(2, 4)
	case 0x55:
		c.eorZeroPageX(m)
	case 0x56:
		c.lsrZeroPageX(m)
	case 0x57:
		c.sreZeroPageX(m)
	case 0x58:
		c.cli()
	case 0x59:
		c.eorAbsoluteY(m)
	case 0x5a:
		c.nop(1, 2)
	case 0x5b:
		c.sreAbsoluteY(m)
	case 0x5c:
		c.nopAbsoluteX(m)
	case 0x5d:
		c.eorAbsoluteX(m)
	case 0x5e:
		c.lsrAbsoluteX(m)
	case 0x5f:
		c.sreAbsoluteX(m)
	case 0x60:
		c.rta(m)
	case 0x61:
		c.adcZeroPageIndirectX(m)
	case 0x62:
		c.nop(0, 3)
	case 0x63:
		c.rraZeroPageIndirectX(m)
	case 0x64:
		c.nop(2, 3)
	case 0x65:
		c.adcZeroPage(m)
	case 0x66:
		c.rorZeroPage(m)
	case 0x67:
		c.rraZeroPage(m)
	case 0x68:
		c.pla(m)
	case 0x69:
		c.adcImmediate(m)
	case 0x6a:
		c.ror()
	case 0x6b:
		c.arrImmediate(m)
	case 0x6c:
		c.jmpIndirect(m)
	case 0x6d:
		c.adcAbsolute(m)
	case 0x6e:
		c.rorAbsolute(m)
	case 0x6f:
		c.rraAbsolute(m)
	case 0x70:
		c.bvs(m)
	case 0x71:
		c.adcZeroPageIndirectY(m)
	case 0x72:
		c.nop(0, 3)
	case 0x73:
		c.rraZeroPageIndirectY(m)
	case 0x74:
		c.nop(2, 4)
	case 0x75:
		c.adcZeroPageX(m)
	case 0x76:
		c.rorZeroPageX(m)
	case 0x77:
		c.rraZeroPageX(m)
	case 0x78:
		c.sei()
	case 0x79:
		c.adcAbsoluteY(m)
	case 0x7a:
		c.nop(1, 2)
	case 0x7b:
		c.rraAbsoluteY(m)
	case 0x7c:
		c.nopAbsoluteX(m)
	case 0x7d:
		c.adcAbsoluteX(m)
	case 0x7e:
		c.rorAbsoluteX(m)
	case 0x7f:
		c.rraAbsoluteX(m)
	case 0x80:
		c.nop(2, 2)
	case 0x81:
		c.staZeroPageIndirectX(m)
	case 0x82:
		c.nop(2, 2)
	case 0x83:
		c.saxZeroPageIndirectX(m)
	case 0x84:
		c.styZeroPage(m)
	case 0x85:
		c.staZeroPage(m)
	case 0x86:
		c.stxZeroPage(m)
	case 0x87:
		c.saxZeroPage(m)
	case 0x88:
		c.dey()
	case 0x89:
		c.nop(2, 2)
	case 0x8a:
		c.txa()
	case 0x8b:
		c.xaaImmediate(m)
	case 0x8c:
		c.styAbsolute(m)
	case 0x8d:
		c.staAbsolute(m)
	case 0x8e:
		c.stxAbsolute(m)
	case 0x8f:
		c.saxAbsolute(m)
	case 0x90:
		c.bcc(m)
	case 0x91:
		c.staZeroPageIndirectY(m)
	case 0x92:
		c.nop(0, 3)
	case 0x93:
		c.ahxZeroPageIndirectY(m)
	case 0x94:
		// TODO impl
	case 0x95:
		// TODO impl
	case 0x96:
		// TODO impl
	case 0x97:
		// TODO impl
	case 0x98:
		// TODO impl
	case 0x99:
		// TODO impl
	case 0x9a:
		// TODO impl
	case 0x9b:
		// TODO impl
	case 0x9c:
		// TODO impl
	case 0x9d:
		// TODO impl
	case 0x9e:
		// TODO impl
	case 0x9f:
		// TODO impl
	case 0xa0:
		// TODO impl
	case 0xa1:
		// TODO impl
	case 0xa2:
		// TODO impl
	case 0xa3:
		// TODO impl
	case 0xa4:
		// TODO impl
	case 0xa5:
		// TODO impl
	case 0xa6:
		// TODO impl
	case 0xa7:
		// TODO impl
	case 0xa8:
		// TODO impl
	case 0xa9:
		// TODO impl
	case 0xaa:
		// TODO impl
	case 0xab:
		// TODO impl
	case 0xac:
		// TODO impl
	case 0xad:
		// TODO impl
	case 0xae:
		// TODO impl
	case 0xaf:
		// TODO impl
	case 0xb0:
		// TODO impl
	case 0xb1:
		// TODO impl
	case 0xb2:
		// TODO impl
	case 0xb3:
		// TODO impl
	case 0xb4:
		// TODO impl
	case 0xb5:
		// TODO impl
	case 0xb6:
		// TODO impl
	case 0xb7:
		// TODO impl
	case 0xb8:
		// TODO impl
	case 0xb9:
		// TODO impl
	case 0xba:
		// TODO impl
	case 0xbb:
		// TODO impl
	case 0xbc:
		// TODO impl
	case 0xbd:
		// TODO impl
	case 0xbe:
		// TODO impl
	case 0xbf:
		// TODO impl
	case 0xc0:
		// TODO impl
	case 0xc1:
		// TODO impl
	case 0xc2:
		// TODO impl
	case 0xc3:
		// TODO impl
	case 0xc4:
		// TODO impl
	case 0xc5:
		// TODO impl
	case 0xc6:
		// TODO impl
	case 0xc7:
		// TODO impl
	case 0xc8:
		// TODO impl
	case 0xc9:
		// TODO impl
	case 0xca:
		// TODO impl
	case 0xcb:
		// TODO impl
	case 0xcc:
		// TODO impl
	case 0xcd:
		// TODO impl
	case 0xce:
		// TODO impl
	case 0xcf:
		// TODO impl
	case 0xd0:
		// TODO impl
	case 0xd1:
		// TODO impl
	case 0xd2:
		// TODO impl
	case 0xd3:
		// TODO impl
	case 0xd4:
		// TODO impl
	case 0xd5:
		// TODO impl
	case 0xd6:
		// TODO impl
	case 0xd7:
		// TODO impl
	case 0xd8:
		// TODO impl
	case 0xd9:
		// TODO impl
	case 0xda:
		// TODO impl
	case 0xdb:
		// TODO impl
	case 0xdc:
		// TODO impl
	case 0xdd:
		// TODO impl
	case 0xde:
		// TODO impl
	case 0xdf:
		// TODO impl
	case 0xe0:
		// TODO impl
	case 0xe1:
		// TODO impl
	case 0xe2:
		// TODO impl
	case 0xe3:
		// TODO impl
	case 0xe4:
		// TODO impl
	case 0xe5:
		// TODO impl
	case 0xe6:
		// TODO impl
	case 0xe7:
		// TODO impl
	case 0xe8:
		// TODO impl
	case 0xe9:
		// TODO impl
	case 0xea:
		// TODO impl
	case 0xeb:
		// TODO impl
	case 0xec:
		// TODO impl
	case 0xed:
		// TODO impl
	case 0xee:
		// TODO impl
	case 0xef:
		// TODO impl
	case 0xf0:
		// TODO impl
	case 0xf1:
		// TODO impl
	case 0xf2:
		// TODO impl
	case 0xf3:
		// TODO impl
	case 0xf4:
		// TODO impl
	case 0xf5:
		// TODO impl
	case 0xf6:
		// TODO impl
	case 0xf7:
		// TODO impl
	case 0xf8:
		// TODO impl
	case 0xf9:
		// TODO impl
	case 0xfa:
		// TODO impl
	case 0xfb:
		// TODO impl
	case 0xfc:
		// TODO impl
	case 0xfd:
		// TODO impl
	case 0xfe:
		// TODO impl
	case 0xff:
		// TODO impl
	}
}

func (c *CPU) brk(m Memory) {
	c.push16(m, c.PC+2)
	c.setFlags(BreakCommandFlagMask)
	c.push8(m, c.Flags)
	c.clearFlags(BreakCommandFlagMask)
	c.setFlags(InterruptDisableFlagMask)
	c.PC = Read16(m, InterruptRequestInterruptAddress)
	c.ClockTime += 7
}

func (c *CPU) php(m Memory) {
	c.setFlags(BreakCommandFlagMask)
	c.push8(m, c.Flags)
	c.clearFlags(BreakCommandFlagMask)
	c.PC += 1
	c.ClockTime += 3
}

func (c *CPU) plp(m Memory) {
	c.Flags = c.pop8(m)
	c.clearFlags(BreakCommandFlagMask)
	c.setFlags(UnusedFlagMask)
	c.PC += 1
	c.ClockTime += 4
}

func (c *CPU) rti(m Memory) {
	c.Flags = c.pop8(m)
	c.PC = c.pop16(m)
	c.clearFlags(BreakCommandFlagMask)
	c.setFlags(UnusedFlagMask)
	c.ClockTime += 6
}

func (c *CPU) rta(m Memory) {
	c.PC = c.pop16(m) + 1
	c.ClockTime += 6
}

func (c *CPU) pha(m Memory) {
	c.push8(m, c.A)
	c.PC += 1
	c.ClockTime += 3
}

func (c *CPU) pla(m Memory) {
	c.A = c.pop8(m)
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += 1
	c.ClockTime += 4
}

func (c *CPU) oraImmediate(m Memory) {
	value := m.Read((c.PC + 1))
	c.oraCommon(value, 2, 2)
}

func (c *CPU) oraAbsolute(m Memory) {
	data := c.absolute(m)
	c.oraCommon(data.value, 3, 4)
}

func (c *CPU) oraZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.oraCommon(data.value, 2, 6)
}

func (c *CPU) oraZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.oraCommon(data.value, 2, 5+data.extraClock)
}

func (c *CPU) oraZeroPageFixed(m Memory) {
	data := c.zeroPageFixed(m)
	c.oraCommon(data.value, 2, 3)
}

func (c *CPU) oraZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.oraCommon(data.value, 2, 4)
}

func (c *CPU) oraAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.oraCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) oraAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.oraCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) oraCommon(newValue uint8, pcOffset Word, clock uint64) {
	c.A |= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) sloAbsolute(m Memory) {
	data := c.absolute(m)
	c.sloCommon(m, data.address, data.value, 3, 6)
}

func (c *CPU) sloAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.sloCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) sloAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.sloCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) sloZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.sloCommon(m, data.address, data.value, 2, 8)
}

func (c *CPU) sloZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.sloCommon(m, data.address, data.value, 2, 8)
}

func (c *CPU) sloZeroPageImmediate(m Memory) {
	data := c.zeroPageFixed(m)
	c.sloCommon(m, data.address, data.value, 2, 5)
}

func (c *CPU) sloZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.sloCommon(m, data.address, data.value, 2, 6)
}

func (c *CPU) sloCommon(m Memory, address Word, value uint8, pcOffset Word, clock uint64) {
	newValue := value << 1
	m.Write(address, newValue)
	c.A |= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, newValue < value)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) asl() {
	value := c.A
	newValue := value << 1
	c.A = newValue
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, newValue < value)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) aslZeroPageFixed(m Memory) {
	data := c.zeroPageFixed(m)
	c.aslCommon(m, data.address, data.value, 2, 5)
}

func (c *CPU) aslAbsolute(m Memory) {
	data := c.absolute(m)
	c.aslCommon(m, data.address, data.value, 3, 6)
}

func (c *CPU) aslZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.aslCommon(m, data.address, data.value, 2, 6)
}

func (c *CPU) aslAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.aslCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) aslCommon(m Memory, address Word, value uint8, pcOffset Word, clock uint64) {
	newValue := value << 1
	m.Write(address, newValue)
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, newValue < value)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) ancImmediate(m Memory) {
	value := m.Read((c.PC + 1))
	newValue := c.A & value
	c.A = newValue
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, (newValue&0b1000_0000) != 0)
	c.PC += 2
	c.ClockTime += 2
}

func (c *CPU) andZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	c.andCommon(data.value, 2, 3)
}

func (c *CPU) andZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.andCommon(data.value, 2, 4)
}

func (c *CPU) andZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.andCommon(data.value, 2, 6)
}

func (c *CPU) andZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.andCommon(data.value, 2, 5+data.extraClock)
}

func (c *CPU) andImmediate(m Memory) {
	value := m.Read((c.PC + 1))
	c.andCommon(value, 2, 2)
}

func (c *CPU) andAbsolute(m Memory) {
	data := c.absolute(m)
	c.andCommon(data.value, 3, 4)
}

func (c *CPU) andAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.andCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) andAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.andCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) andCommon(value uint8, pcOffset Word, clock uint64) {
	c.A &= value
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) rlaZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	c.rlaCommon(m, data.address, data.value, 2, 5)
}

func (c *CPU) rlaZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.rlaCommon(m, data.address, data.value, 2, 6)
}

func (c *CPU) rlaZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.rlaCommon(m, data.address, data.value, 2, 8)
}

func (c *CPU) rlaZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.rlaCommon(m, data.address, data.value, 2, 8)
}

func (c *CPU) rlaAbsolute(m Memory) {
	data := c.absolute(m)
	c.rlaCommon(m, data.address, data.value, 3, 6)
}

func (c *CPU) rlaAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.rlaCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) rlaAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.rlaCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) rlaCommon(m Memory, address Word, value uint8, pcOffset Word, clock uint64) {
	newValue := value << 1
	if (c.Flags & CarryFlagMask) != 0 {
		newValue += 1
	}
	m.Write(address, newValue)
	c.A &= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b1000_0000) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) rol() {
	c.A = c.rolCommon(c.A, 1, 2)
}

func (c *CPU) rolZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.rolCommon(data.value, 2, 5)
	m.Write(data.address, newValue)
}

func (c *CPU) rolZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	newValue := c.rolCommon(data.value, 2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) rolAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.rolCommon(data.value, 3, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) rolAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	newValue := c.rolCommon(data.value, 3, 7)
	m.Write(data.address, newValue)
}

func (c *CPU) rolCommon(value uint8, pcOffset Word, clock uint64) uint8 {
	newValue := value << 1
	if (c.Flags & CarryFlagMask) != 0 {
		newValue += 1
	}
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b1000_0000) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
	return newValue
}

func (c *CPU) bpl(m Memory) {
	c.branchCommon(m, (c.Flags&NegativeFlagMask) == 0)
}

func (c *CPU) bmi(m Memory) {
	c.branchCommon(m, (c.Flags&NegativeFlagMask) != 0)
}

func (c *CPU) bvc(m Memory) {
	c.branchCommon(m, (c.Flags&OverflowFlagMask) == 0)
}

func (c *CPU) bvs(m Memory) {
	c.branchCommon(m, (c.Flags&OverflowFlagMask) != 0)
}

func (c *CPU) bcc(m Memory) {
	c.branchCommon(m, (c.Flags&CarryFlagMask) == 0)
}

func (c *CPU) branchCommon(m Memory, condition bool) {
	if condition {
		// high byte of address after the branch instruction
		high1 := (c.PC + 2) & 0xff00
		// do the jump
		c.PC = Word(int32(c.PC) + 2 + int32(int8(m.Read((c.PC + 1)))))
		// high byte of address of branch destination
		high2 := c.PC & 0xff00
		if high1 == high2 {
			c.ClockTime += 3
		} else {
			c.ClockTime += 4
		}
	} else {
		c.PC += 2
		c.ClockTime += 2
	}
}

func (c *CPU) jmpAbsolute(m Memory) {
	c.PC = Read16(m, (c.PC + 1))
	c.ClockTime += 3
}

func (c *CPU) jmpIndirect(m Memory) {
	address := Read16(m, (c.PC + 1))
	if address&0xff == 0x0ff {
		low := m.Read(address)
		high := m.Read(address & 0xff00)
		c.PC = NewWord(low, high)
	} else {
		c.PC = Read16(m, address)
	}
	c.ClockTime += 5
}

func (c *CPU) clc() {
	c.clearFlags(CarryFlagMask)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) sec() {
	c.setFlags(CarryFlagMask)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) cli() {
	c.clearFlags(InterruptDisableFlagMask)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) sei() {
	c.setFlags(InterruptDisableFlagMask)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) dey() {
	c.Y--
	c.setFlagsTo(NegativeFlagMask, int8(c.Y) < 0)
	c.setFlagsTo(ZeroFlagMask, c.Y == 0)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) jsr(m Memory) {
	address := Read16(m, (c.PC + 1))
	c.push16(m, c.PC+2)
	c.PC = address
	c.ClockTime += 6
}

func (c *CPU) bitZeroPageImmediate(m Memory) {
	data := c.zeroPageFixed(m)
	c.bitCommon(data.value, 2, 3)
}

func (c *CPU) bitAbsolute(m Memory) {
	data := c.absolute(m)
	c.bitCommon(data.value, 3, 4)
}

func (c *CPU) bitCommon(value uint8, pcOffset Word, clock uint64) {
	c.setFlagsTo(OverflowFlagMask, (value&OverflowFlagMask) != 0)
	c.setFlagsTo(NegativeFlagMask, (NegativeFlagMask&value) != 0)
	c.setFlagsTo(ZeroFlagMask, (value&c.A) == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) eorZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	c.eorCommon(data.value, 2, 3)
}

func (c *CPU) eorZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.eorCommon(data.value, 2, 4)
}

func (c *CPU) eorZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.eorCommon(data.value, 2, 6)
}

func (c *CPU) eorZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.eorCommon(data.value, 2, 5+data.extraClock)
}

func (c *CPU) eorImmediate(m Memory) {
	value := m.Read((c.PC + 1))
	c.eorCommon(value, 2, 2)
}

func (c *CPU) eorAbsolute(m Memory) {
	data := c.absolute(m)
	c.eorCommon(data.value, 3, 4)
}

func (c *CPU) eorAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.eorCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) eorAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.eorCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) eorCommon(value uint8, pcOffset Word, clock uint64) {
	c.A ^= value
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) sreZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	c.sreCommon(m, data.address, data.value, 2, 5)
}

func (c *CPU) sreZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.sreCommon(m, data.address, data.value, 2, 6)
}

func (c *CPU) sreZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.sreCommon(m, data.address, data.value, 2, 8)
}

func (c *CPU) sreZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.sreCommon(m, data.address, data.value, 2, 8)
}

func (c *CPU) sreAbsolute(m Memory) {
	data := c.absolute(m)
	c.sreCommon(m, data.address, data.value, 3, 6)
}

func (c *CPU) sreAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.sreCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) sreAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.sreCommon(m, data.address, data.value, 3, 7)
}

func (c *CPU) sreCommon(m Memory, address Word, value uint8, pcOffset Word, clock uint64) {
	newValue := value >> 1
	m.Write(address, newValue)
	c.A ^= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) lsr() {
	c.A = c.lsrCommon(c.A, 1, 2)
}

func (c *CPU) lsrZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.lsrCommon(data.value, 2, 5)
	m.Write(data.address, newValue)
}

func (c *CPU) lsrZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	newValue := c.lsrCommon(data.value, 2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) lsrAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.lsrCommon(data.value, 3, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) lsrAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	newValue := c.lsrCommon(data.value, 3, 7)
	m.Write(data.address, newValue)
}

func (c *CPU) lsrCommon(value uint8, pcOffset Word, clock uint64) uint8 {
	newValue := value >> 1
	c.clearFlags(NegativeFlagMask)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
	return newValue
}

func (c *CPU) alr(m Memory) {
	value := c.A & m.Read((c.PC + 1))
	c.A = value >> 1
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
	c.PC += 2
	c.ClockTime += 2
}

func (c *CPU) adcImmediate(m Memory) {
	value := m.Read((c.PC + 1))
	c.adcCommon(value, 2, 2)
}

func (c *CPU) adcAbsolute(m Memory) {
	data := c.absolute(m)
	c.adcCommon(data.value, 3, 4)
}

func (c *CPU) adcAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.adcCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) adcAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	c.adcCommon(data.value, 3, 4+data.extraClock)
}

func (c *CPU) adcZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	c.adcCommon(data.value, 2, 3)
}

func (c *CPU) adcZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	c.adcCommon(data.value, 2, 4)
}

func (c *CPU) adcZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	c.adcCommon(data.value, 2, 6)
}

func (c *CPU) adcZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.adcCommon(data.value, 2, 5+data.extraClock)
}

func (c *CPU) adcCommon(value uint8, pcOffset Word, clock uint64) {
	newValue := uint16(c.A) + uint16(value)
	if (c.Flags & CarryFlagMask) != 0 {
		newValue++
	}
	oldA := c.A
	c.A = uint8(newValue)
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (newValue&0b1_0000_0000) != 0)
	valueSignBit := value & 0b1000_0000
	oldASignBit := oldA & 0b1000_0000
	newSignBit := uint8(newValue & 0b1000_0000)
	c.setFlagsTo(OverflowFlagMask, valueSignBit == oldASignBit && valueSignBit != newSignBit)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) rraAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.rraCommon(data.value, 3, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) rraAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	newValue := c.rraCommon(data.value, 3, 7)
	m.Write(data.address, newValue)
}

func (c *CPU) rraAbsoluteY(m Memory) {
	data := c.absoluteY(m)
	newValue := c.rraCommon(data.value, 3, 7)
	m.Write(data.address, newValue)
}

func (c *CPU) rraZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.rraCommon(data.value, 2, 5)
	m.Write(data.address, newValue)
}

func (c *CPU) rraZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	newValue := c.rraCommon(data.value, 2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) rraZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	newValue := c.rraCommon(data.value, 2, 8)
	m.Write(data.address, newValue)
}

func (c *CPU) rraZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	newValue := c.rraCommon(data.value, 2, 8)
	m.Write(data.address, newValue)
}

func (c *CPU) rraCommon(value uint8, pcOffset Word, clock uint64) uint8 {
	rorNewValue := value >> 1
	if (c.Flags & CarryFlagMask) != 0 {
		rorNewValue |= 0b1000_0000
	}
	adcNewValue := uint16(c.A) + uint16(rorNewValue)
	// carry flag check, but what the carry flag should be after the previous ROR
	if (value & 1) != 0 {
		adcNewValue++
	}
	oldA := c.A
	c.A = uint8(adcNewValue)
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (adcNewValue&0b1_0000_0000) != 0)
	valueSignBit := rorNewValue & 0b1000_0000
	oldASignBit := oldA & 0b1000_0000
	newSignBit := uint8(adcNewValue & 0b1000_0000)
	c.setFlagsTo(OverflowFlagMask, valueSignBit == oldASignBit && valueSignBit != newSignBit)
	c.PC += pcOffset
	c.ClockTime += clock
	return rorNewValue
}

func (c *CPU) ror() {
	c.A = c.rorCommon(c.A, 1, 2)
}

func (c *CPU) rorAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.rorCommon(data.value, 3, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) rorAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	newValue := c.rorCommon(data.value, 3, 7)
	m.Write(data.address, newValue)
}

func (c *CPU) rorZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.rorCommon(data.value, 2, 5)
	m.Write(data.address, newValue)
}

func (c *CPU) rorZeroPageX(m Memory) {
	data := c.zeroPageX(m)
	newValue := c.rorCommon(data.value, 2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) rorCommon(value uint8, pcOffset Word, clock uint64) uint8 {
	newValue := value >> 1
	if (c.Flags & CarryFlagMask) != 0 {
		newValue |= 0b1000_0000
	}
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, (value&1) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
	return newValue
}

func (c *CPU) arrImmediate(m Memory) {
	immValue := m.Read((c.PC + 1))
	newValue := c.A & immValue
	c.setFlagsTo(OverflowFlagMask, (newValue^(newValue>>1))&0x40 != 0)
	newCarry := newValue & 0b1000_0000
	newValue >>= 1
	if (c.Flags & CarryFlagMask) != 0 {
		newValue |= 0b1000_0000
	}
	c.A = newValue
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, newCarry != 0)
	c.PC += 2
	c.ClockTime += 2
}

func (c *CPU) staAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.staCommon(3, 4)
	m.Write(data.address, newValue)
}

func (c *CPU) staZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.staCommon(2, 3)
	m.Write(data.address, newValue)
}

func (c *CPU) staZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	newValue := c.staCommon(2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) staZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	newValue := c.staCommon(2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) staCommon(pcOffset Word, clock uint64) uint8 {
	c.PC += pcOffset
	c.ClockTime += clock
	return c.A
}

func (c *CPU) saxAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.saxCommon(3, 4)
	m.Write(data.address, newValue)
}

func (c *CPU) saxZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.saxCommon(2, 3)
	m.Write(data.address, newValue)
}

func (c *CPU) saxZeroPageIndirectX(m Memory) {
	data := c.zeroPageIndirectX(m)
	newValue := c.saxCommon(2, 6)
	m.Write(data.address, newValue)
}

func (c *CPU) saxCommon(pcOffset Word, clock uint64) uint8 {
	c.PC += pcOffset
	c.ClockTime += clock
	return c.A & c.X
}

func (c *CPU) stxAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.stxCommon(3, 4)
	m.Write(data.address, newValue)
}

func (c *CPU) stxZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.stxCommon(2, 3)
	m.Write(data.address, newValue)
}

func (c *CPU) stxCommon(pcOffset Word, clock uint64) uint8 {
	c.PC += pcOffset
	c.ClockTime += clock
	return c.X
}

func (c *CPU) styAbsolute(m Memory) {
	data := c.absolute(m)
	newValue := c.styCommon(3, 4)
	m.Write(data.address, newValue)
}

func (c *CPU) styZeroPage(m Memory) {
	data := c.zeroPageFixed(m)
	newValue := c.styCommon(2, 3)
	m.Write(data.address, newValue)
}

func (c *CPU) styCommon(pcOffset Word, clock uint64) uint8 {
	c.PC += pcOffset
	c.ClockTime += clock
	return c.Y
}

func (c *CPU) xaaImmediate(m Memory) {
	value := m.Read((c.PC + 1))
	c.A |= 0xee
	c.A &= c.X
	c.A &= value
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += 2
	c.ClockTime += 2
}

func (c *CPU) txa() {
	c.A = c.X
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += 1
	c.ClockTime += 2
}

func (c *CPU) ahxZeroPageIndirectY(m Memory) {
	data := c.zeroPageIndirectY(m)
	c.ahxCommon(m, data, 2, 6)
}

func (c *CPU) ahxCommon(m Memory, data addrValueClock, pcOffset Word, clock uint64) {
	newValue := c.A & c.X
	var address Word
	if data.IsPageCrossing() {
		newValue &= data.address.High()
		address = NewWord(data.address.Low(), newValue)
	} else {
		newValue &= data.address.High() + 1
		address = data.address
	}
	m.Write(address, newValue)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) nop(pcOffset Word, clock uint64) {
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) nopAbsoluteX(m Memory) {
	data := c.absoluteX(m)
	c.PC += 3
	c.ClockTime += 4 + data.extraClock
}

func (c *CPU) clearFlags(mask uint8) {
	c.Flags &= ^mask
}

func (c *CPU) setFlags(mask uint8) {
	c.Flags |= mask
}

func (c *CPU) setFlagsTo(mask uint8, value bool) {
	if value {
		c.setFlags(mask)
	} else {
		c.clearFlags(mask)
	}
}

func (c *CPU) push8(m Memory, value uint8) {
	m.Write(StackAddress+Word(c.SP), value)
	c.SP--
}

func (c *CPU) pop8(m Memory) uint8 {
	c.SP++
	return m.Read(StackAddress + Word(c.SP))
}

func (c *CPU) push16(m Memory, value Word) {
	c.push8(m, value.High())
	c.push8(m, value.Low())
}

func (c *CPU) pop16(m Memory) Word {
	low := c.pop8(m)
	high := c.pop8(m)
	return NewWord(low, high)
}

func (c *CPU) absolute(m Memory) addrValue {
	address := Word(Read16(m, (c.PC + 1)))
	value := m.Read(address)
	return addrValue{address, value}
}

func (c *CPU) absoluteX(m Memory) addrValueClock {
	return c.absoluteCommon(m, c.X)
}

func (c *CPU) absoluteY(m Memory) addrValueClock {
	return c.absoluteCommon(m, c.Y)
}

func (c *CPU) absoluteCommon(m Memory, offset uint8) addrValueClock {
	address := Read16(m, (c.PC + 1))
	high1 := address & 0xff00
	address += Word(offset)
	value := m.Read(address)
	high2 := address & 0xff00
	// if adding Y pushes us into a new page it will take an extra clock cycle to resolve
	var extraClock uint64
	if high1 == high2 {
		extraClock = 0
	} else {
		extraClock = 1
	}
	return addrValueClock{address, value, extraClock}
}

func (c *CPU) zeroPageFixed(m Memory) addrValue {
	address := Word(m.Read((c.PC + 1)))
	value := m.Read(address)
	return addrValue{address, value}
}

func (c *CPU) zeroPageX(m Memory) addrValue {
	address := (Word(m.Read((c.PC + 1))) + Word(c.X)) & 0xff
	value := m.Read(address)
	return addrValue{address, value}
}

func (c *CPU) zeroPageIndirectX(m Memory) addrValue {
	address := zeroPageIndirect(m, m.Read((c.PC + 1)), c.X)
	value := m.Read(address)
	return addrValue{address, value}
}

func (c *CPU) zeroPageIndirectY(m Memory) addrValueClock {
	address := zeroPageIndirect(m, m.Read((c.PC + 1)), 0)
	high1 := address & 0xff00
	address += Word(c.Y)
	value := m.Read(address)
	high2 := address & 0xff00
	// if adding Y pushes us into a new page it will take an extra clock cycle to resolve
	var extraClock uint64
	if high1 == high2 {
		extraClock = 0
	} else {
		extraClock = 1
	}
	return addrValueClock{address, value, extraClock}
}

func zeroPageIndirect(m Memory, offset1, offset2 uint8) Word {
	newOffset := Word(uint16(offset1) + uint16(offset2))
	// address must be on the zero page
	newOffsetLow := newOffset.Low()
	resultLow := m.Read(Word(newOffsetLow))
	// address must be on the zero page
	resultHigh := m.Read(Word(newOffsetLow + 1))
	return NewWord(resultLow, resultHigh)
}

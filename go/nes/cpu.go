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
	StackAddress                     uint16 = 0x0100
	NonMaskableInterruptAddress      uint16 = 0xfffa
	ResetInterruptAddress            uint16 = 0xfffc
	InterruptRequestInterruptAddress uint16 = 0xfffe
)

type CPU struct {
	PC        uint16
	SP        uint8
	A         uint8
	X         uint8
	Y         uint8
	Flags     uint8
	ClockTime uint64
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
		c.asl(m)
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
		c.rol(m)
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
		c.lsr(m)
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
		// TODO impl
	case 0x6d:
		// TODO impl
	case 0x6e:
		// TODO impl
	case 0x6f:
		// TODO impl
	case 0x70:
		// TODO impl
	case 0x71:
		// TODO impl
	case 0x72:
		// TODO impl
	case 0x73:
		// TODO impl
	case 0x74:
		// TODO impl
	case 0x75:
		// TODO impl
	case 0x76:
		// TODO impl
	case 0x77:
		// TODO impl
	case 0x78:
		// TODO impl
	case 0x79:
		// TODO impl
	case 0x7a:
		// TODO impl
	case 0x7b:
		// TODO impl
	case 0x7c:
		// TODO impl
	case 0x7d:
		// TODO impl
	case 0x7e:
		// TODO impl
	case 0x7f:
		// TODO impl
	case 0x80:
		// TODO impl
	case 0x81:
		// TODO impl
	case 0x82:
		// TODO impl
	case 0x83:
		// TODO impl
	case 0x84:
		// TODO impl
	case 0x85:
		// TODO impl
	case 0x86:
		// TODO impl
	case 0x87:
		// TODO impl
	case 0x88:
		// TODO impl
	case 0x89:
		// TODO impl
	case 0x8a:
		// TODO impl
	case 0x8b:
		// TODO impl
	case 0x8c:
		// TODO impl
	case 0x8d:
		// TODO impl
	case 0x8e:
		// TODO impl
	case 0x8f:
		// TODO impl
	case 0x90:
		// TODO impl
	case 0x91:
		// TODO impl
	case 0x92:
		// TODO impl
	case 0x93:
		// TODO impl
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
	value := m.Read(c.PC + 1)
	c.oraCommon(value, 2, 2)
}

func (c *CPU) oraAbsolute(m Memory) {
	_, value := c.absolute(m)
	c.oraCommon(value, 3, 4)
}

func (c *CPU) oraZeroPageIndirectX(m Memory) {
	_, value := c.zeroPageIndirectX(m)
	c.oraCommon(value, 2, 6)
}

func (c *CPU) oraZeroPageIndirectY(m Memory) {
	_, value, extraClock := c.zeroPageIndirectY(m)
	c.oraCommon(value, 2, 5+extraClock)
}

func (c *CPU) oraZeroPageFixed(m Memory) {
	_, value := c.zeroPageFixed(m)
	c.oraCommon(value, 2, 3)
}

func (c *CPU) oraZeroPageX(m Memory) {
	_, value := c.zeroPageX(m)
	c.oraCommon(value, 2, 4)
}

func (c *CPU) oraAbsoluteX(m Memory) {
	_, value, extraClock := c.absoluteX(m)
	c.oraCommon(value, 3, 4+extraClock)
}

func (c *CPU) oraAbsoluteY(m Memory) {
	_, value, extraClock := c.absoluteY(m)
	c.oraCommon(value, 3, 4+extraClock)
}

func (c *CPU) oraCommon(newValue uint8, pcOffset uint16, clock uint64) {
	c.A |= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) sloAbsolute(m Memory) {
	address, value := c.absolute(m)
	c.sloCommon(m, address, value, 3, 6)
}

func (c *CPU) sloAbsoluteX(m Memory) {
	address, value, _ := c.absoluteX(m)
	c.sloCommon(m, address, value, 3, 7)
}

func (c *CPU) sloAbsoluteY(m Memory) {
	address, value, _ := c.absoluteY(m)
	c.sloCommon(m, address, value, 3, 7)
}

func (c *CPU) sloZeroPageIndirectX(m Memory) {
	address, value := c.zeroPageIndirectX(m)
	c.sloCommon(m, address, value, 2, 8)
}

func (c *CPU) sloZeroPageIndirectY(m Memory) {
	address, value, _ := c.zeroPageIndirectY(m)
	c.sloCommon(m, address, value, 2, 8)
}

func (c *CPU) sloZeroPageImmediate(m Memory) {
	address, value := c.zeroPageFixed(m)
	c.sloCommon(m, address, value, 2, 5)
}

func (c *CPU) sloZeroPageX(m Memory) {
	address, value := c.zeroPageX(m)
	c.sloCommon(m, address, value, 2, 6)
}

func (c *CPU) sloCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
	newValue := value << 1
	m.Write(address, newValue)
	c.A |= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, newValue < value)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) asl(m Memory) {
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
	address, value := c.zeroPageFixed(m)
	c.aslCommon(m, address, value, 2, 5)
}

func (c *CPU) aslAbsolute(m Memory) {
	address, value := c.absolute(m)
	c.aslCommon(m, address, value, 3, 6)
}

func (c *CPU) aslZeroPageX(m Memory) {
	address, value := c.zeroPageX(m)
	c.aslCommon(m, address, value, 2, 6)
}

func (c *CPU) aslAbsoluteX(m Memory) {
	address, value, _ := c.absoluteX(m)
	c.aslCommon(m, address, value, 3, 7)
}

func (c *CPU) aslCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
	newValue := value << 1
	m.Write(address, newValue)
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, newValue < value)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) ancImmediate(m Memory) {
	value := m.Read(c.PC + 1)
	newValue := c.A & value
	c.A = newValue
	c.setFlagsTo(NegativeFlagMask, int8(newValue) < 0)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, (newValue&0b1000_0000) != 0)
	c.PC += 2
	c.ClockTime += 2
}

func (c *CPU) andZeroPage(m Memory) {
	_, value := c.zeroPageFixed(m)
	c.andCommon(value, 2, 3)
}

func (c *CPU) andZeroPageX(m Memory) {
	_, value := c.zeroPageX(m)
	c.andCommon(value, 2, 4)
}

func (c *CPU) andZeroPageIndirectX(m Memory) {
	_, value := c.zeroPageIndirectX(m)
	c.andCommon(value, 2, 6)
}

func (c *CPU) andZeroPageIndirectY(m Memory) {
	_, value, extraClock := c.zeroPageIndirectY(m)
	c.andCommon(value, 2, 5+extraClock)
}

func (c *CPU) andImmediate(m Memory) {
	value := m.Read(c.PC + 1)
	c.andCommon(value, 2, 2)
}

func (c *CPU) andAbsolute(m Memory) {
	_, value := c.absolute(m)
	c.andCommon(value, 3, 4)
}

func (c *CPU) andAbsoluteX(m Memory) {
	_, value, extraClock := c.absoluteX(m)
	c.andCommon(value, 3, 4+extraClock)
}

func (c *CPU) andAbsoluteY(m Memory) {
	_, value, extraClock := c.absoluteY(m)
	c.andCommon(value, 3, 4+extraClock)
}

func (c *CPU) andCommon(value uint8, pcOffset uint16, clock uint64) {
	c.A &= value
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) rlaZeroPage(m Memory) {
	address, value := c.zeroPageFixed(m)
	c.rlaCommon(m, address, value, 2, 5)
}

func (c *CPU) rlaZeroPageX(m Memory) {
	address, value := c.zeroPageX(m)
	c.rlaCommon(m, address, value, 2, 6)
}

func (c *CPU) rlaZeroPageIndirectX(m Memory) {
	address, value := c.zeroPageIndirectX(m)
	c.rlaCommon(m, address, value, 2, 8)
}

func (c *CPU) rlaZeroPageIndirectY(m Memory) {
	address, value, _ := c.zeroPageIndirectY(m)
	c.rlaCommon(m, address, value, 2, 8)
}

func (c *CPU) rlaAbsolute(m Memory) {
	address, value := c.absolute(m)
	c.rlaCommon(m, address, value, 3, 6)
}

func (c *CPU) rlaAbsoluteX(m Memory) {
	address, value, _ := c.absoluteX(m)
	c.rlaCommon(m, address, value, 3, 7)
}

func (c *CPU) rlaAbsoluteY(m Memory) {
	address, value, _ := c.absoluteY(m)
	c.rlaCommon(m, address, value, 3, 7)
}

func (c *CPU) rlaCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
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

func (c *CPU) rol(m Memory) {
	c.A = c.rolCommon(c.A, 1, 2)
}

func (c *CPU) rolZeroPage(m Memory) {
	address, value := c.zeroPageFixed(m)
	newValue := c.rolCommon(value, 2, 5)
	m.Write(address, newValue)
}

func (c *CPU) rolZeroPageX(m Memory) {
	address, value := c.zeroPageX(m)
	newValue := c.rolCommon(value, 2, 6)
	m.Write(address, newValue)
}

func (c *CPU) rolAbsolute(m Memory) {
	address, value := c.absolute(m)
	newValue := c.rolCommon(value, 3, 6)
	m.Write(address, newValue)
}

func (c *CPU) rolAbsoluteX(m Memory) {
	address, value, _ := c.absoluteX(m)
	newValue := c.rolCommon(value, 3, 7)
	m.Write(address, newValue)
}

func (c *CPU) rolCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
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

func (c *CPU) branchCommon(m Memory, condition bool) {
	if condition {
		// high byte of address after the branch instruction
		high1 := (c.PC + 2) & 0xff00
		// do the jump
		c.PC = uint16(int32(c.PC) + 2 + int32(int8(m.Read(c.PC+1))))
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
	c.PC = Read16(m, c.PC+1)
	c.ClockTime += 3
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

func (c *CPU) jsr(m Memory) {
	address := Read16(m, c.PC+1)
	c.push16(m, c.PC+2)
	c.PC = address
	c.ClockTime += 6
}

func (c *CPU) bitZeroPageImmediate(m Memory) {
	_, value := c.zeroPageFixed(m)
	c.bitCommon(value, 2, 3)
}

func (c *CPU) bitAbsolute(m Memory) {
	_, value := c.absolute(m)
	c.bitCommon(value, 3, 4)
}

func (c *CPU) bitCommon(value uint8, pcOffset uint16, clock uint64) {
	c.setFlagsTo(OverflowFlagMask, (value&OverflowFlagMask) != 0)
	c.setFlagsTo(NegativeFlagMask, (NegativeFlagMask&value) != 0)
	c.setFlagsTo(ZeroFlagMask, (value&c.A) == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) eorZeroPage(m Memory) {
	_, value := c.zeroPageFixed(m)
	c.eorCommon(value, 2, 3)
}

func (c *CPU) eorZeroPageX(m Memory) {
	_, value := c.zeroPageX(m)
	c.eorCommon(value, 2, 4)
}

func (c *CPU) eorZeroPageIndirectX(m Memory) {
	_, value := c.zeroPageIndirectX(m)
	c.eorCommon(value, 2, 6)
}

func (c *CPU) eorZeroPageIndirectY(m Memory) {
	_, value, extraClock := c.zeroPageIndirectY(m)
	c.eorCommon(value, 2, 5+extraClock)
}

func (c *CPU) eorImmediate(m Memory) {
	value := m.Read(c.PC + 1)
	c.eorCommon(value, 2, 2)
}

func (c *CPU) eorAbsolute(m Memory) {
	_, value := c.absolute(m)
	c.eorCommon(value, 3, 4)
}

func (c *CPU) eorAbsoluteX(m Memory) {
	_, value, extraClock := c.absoluteX(m)
	c.eorCommon(value, 3, 4+extraClock)
}

func (c *CPU) eorAbsoluteY(m Memory) {
	_, value, extraClock := c.absoluteY(m)
	c.eorCommon(value, 3, 4+extraClock)
}

func (c *CPU) eorCommon(value uint8, pcOffset uint16, clock uint64) {
	c.A ^= value
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) sreZeroPage(m Memory) {
	address, value := c.zeroPageFixed(m)
	c.sreCommon(m, address, value, 2, 5)
}

func (c *CPU) sreZeroPageX(m Memory) {
	address, value := c.zeroPageX(m)
	c.sreCommon(m, address, value, 2, 6)
}

func (c *CPU) sreZeroPageIndirectX(m Memory) {
	address, value := c.zeroPageIndirectX(m)
	c.sreCommon(m, address, value, 2, 8)
}

func (c *CPU) sreZeroPageIndirectY(m Memory) {
	address, value, _ := c.zeroPageIndirectY(m)
	c.sreCommon(m, address, value, 2, 8)
}

func (c *CPU) sreAbsolute(m Memory) {
	address, value := c.absolute(m)
	c.sreCommon(m, address, value, 3, 6)
}

func (c *CPU) sreAbsoluteX(m Memory) {
	address, value, _ := c.absoluteX(m)
	c.sreCommon(m, address, value, 3, 7)
}

func (c *CPU) sreAbsoluteY(m Memory) {
	address, value, _ := c.absoluteY(m)
	c.sreCommon(m, address, value, 3, 7)
}

func (c *CPU) sreCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
	newValue := value >> 1
	m.Write(address, newValue)
	c.A ^= newValue
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) lsr(m Memory) {
	c.A = c.lsrCommon(c.A, 1, 2)
}

func (c *CPU) lsrZeroPage(m Memory) {
	address, value := c.zeroPageFixed(m)
	newValue := c.lsrCommon(value, 2, 5)
	m.Write(address, newValue)
}

func (c *CPU) lsrZeroPageX(m Memory) {
	address, value := c.zeroPageX(m)
	newValue := c.lsrCommon(value, 2, 6)
	m.Write(address, newValue)
}

func (c *CPU) lsrAbsolute(m Memory) {
	address, value := c.absolute(m)
	newValue := c.lsrCommon(value, 3, 6)
	m.Write(address, newValue)
}

func (c *CPU) lsrAbsoluteX(m Memory) {
	address, value, _ := c.absoluteX(m)
	newValue := c.lsrCommon(value, 3, 7)
	m.Write(address, newValue)
}

func (c *CPU) lsrCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
	newValue := value >> 1
	c.clearFlags(NegativeFlagMask)
	c.setFlagsTo(ZeroFlagMask, newValue == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
	c.PC += pcOffset
	c.ClockTime += clock
	return newValue
}

func (c *CPU) alr(m Memory) {
	value := c.A & m.Read(c.PC+1)
	c.A = value >> 1
	c.setFlagsTo(NegativeFlagMask, int8(c.A) < 0)
	c.setFlagsTo(ZeroFlagMask, c.A == 0)
	c.setFlagsTo(CarryFlagMask, (value&0b0000_0001) != 0)
	c.PC += 2
	c.ClockTime += 2
}

func (c *CPU) adcImmediate(m Memory) {
	value := m.Read(c.PC + 1)
	c.adcCommon(value, 2, 2)
}

func (c *CPU) adcZeroPage(m Memory) {
	_, value := c.zeroPageFixed(m)
	c.adcCommon(value, 2, 3)
}

func (c *CPU) adcZeroPageIndirectX(m Memory) {
	_, value := c.zeroPageIndirectX(m)
	c.adcCommon(value, 2, 6)
}

func (c *CPU) adcCommon(value uint8, pcOffset uint16, clock uint64) {
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

func (c *CPU) rraZeroPage(m Memory) {
	address, value := c.zeroPageFixed(m)
	newValue := c.rraCommon(value, 2, 5)
	m.Write(address, newValue)
}

func (c *CPU) rraZeroPageIndirectX(m Memory) {
	address, value := c.zeroPageIndirectX(m)
	newValue := c.rraCommon(value, 2, 8)
	m.Write(address, newValue)
}

func (c *CPU) rraCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
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

func (c *CPU) rorZeroPage(m Memory) {
	address, value := c.zeroPageFixed(m)
	newValue := c.rorCommon(value, 2, 5)
	m.Write(address, newValue)
}

func (c *CPU) rorCommon(value uint8, pcOffset uint16, clock uint64) uint8 {
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
	// TODO AND oper + ROR
}

func (c *CPU) nop(pcOffset uint16, clock uint64) {
	c.PC += pcOffset
	c.ClockTime += clock
}

func (c *CPU) nopAbsoluteX(m Memory) {
	_, _, extraClock := c.absoluteX(m)
	c.PC += 3
	c.ClockTime += 4 + extraClock
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
	m.Write(StackAddress+uint16(c.SP), value)
	c.SP--
}

func (c *CPU) pop8(m Memory) uint8 {
	c.SP++
	return m.Read(StackAddress + uint16(c.SP))
}

func (c *CPU) push16(m Memory, value uint16) {
	c.push8(m, uint8(value>>8))
	c.push8(m, uint8(value))
}

func (c *CPU) pop16(m Memory) uint16 {
	low := c.pop8(m)
	high := c.pop8(m)
	return (uint16(high) << 8) | uint16(low)
}

func (c *CPU) absolute(m Memory) (address uint16, value uint8) {
	address = Read16(m, c.PC+1)
	value = m.Read(address)
	return
}

func (c *CPU) absoluteX(m Memory) (address uint16, value uint8, extraClock uint64) {
	return c.absoluteCommon(m, c.X)
}

func (c *CPU) absoluteY(m Memory) (address uint16, value uint8, extraClock uint64) {
	return c.absoluteCommon(m, c.Y)
}

func (c *CPU) absoluteCommon(m Memory, offset uint8) (address uint16, value uint8, extraClock uint64) {
	address = Read16(m, c.PC+1)
	high1 := address & 0xff00
	address += uint16(offset)
	value = m.Read(address)
	high2 := address & 0xff00
	// if adding Y pushes us into a new page it will take an extra clock cycle to resolve
	if high1 == high2 {
		extraClock = 0
	} else {
		extraClock = 1
	}
	return
}

func (c *CPU) zeroPageFixed(m Memory) (address uint16, value uint8) {
	address = uint16(m.Read(c.PC + 1))
	value = m.Read(address)
	return
}

func (c *CPU) zeroPageX(m Memory) (address uint16, value uint8) {
	address = (uint16(m.Read(c.PC+1)) + uint16(c.X)) & 0xff
	value = m.Read(address)
	return
}

func (c *CPU) zeroPageIndirectX(m Memory) (address uint16, value uint8) {
	address = zeroPageIndirect(m, m.Read(c.PC+1), c.X)
	value = m.Read(address)
	return
}

func (c *CPU) zeroPageIndirectY(m Memory) (address uint16, value uint8, extraClock uint64) {
	address = zeroPageIndirect(m, m.Read(c.PC+1), 0)
	high1 := address & 0xff00
	address += uint16(c.Y)
	value = m.Read(address)
	high2 := address & 0xff00
	// if adding Y pushes us into a new page it will take an extra clock cycle to resolve
	if high1 == high2 {
		extraClock = 0
	} else {
		extraClock = 1
	}
	return
}

func zeroPageIndirect(m Memory, offset1, offset2 uint8) (address uint16) {
	newOffset := uint16(offset1) + uint16(offset2)
	// address must be on the zero page
	newOffsetLow := newOffset & 0xff
	resultLow := m.Read(newOffsetLow)
	// address must be on the zero page
	resultHigh := m.Read((newOffsetLow + 1) & 0xff)
	address = uint16(resultLow) | (uint16(resultHigh) << 8)
	return
}

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
		// TODO impl
	case 0x07:
		// TODO impl
	case 0x08:
		// TODO impl
	case 0x09:
		// TODO impl
	case 0x0a:
		// TODO impl
	case 0x0b:
		// TODO impl
	case 0x0c:
		// TODO impl
	case 0x0d:
		// TODO impl
	case 0x0e:
		// TODO impl
	case 0x0f:
		// TODO impl
	case 0x10:
		// TODO impl
	case 0x11:
		// TODO impl
	case 0x12:
		// TODO impl
	case 0x13:
		// TODO impl
	case 0x14:
		// TODO impl
	case 0x15:
		// TODO impl
	case 0x16:
		// TODO impl
	case 0x17:
		// TODO impl
	case 0x18:
		// TODO impl
	case 0x19:
		// TODO impl
	case 0x1a:
		// TODO impl
	case 0x1b:
		// TODO impl
	case 0x1c:
		// TODO impl
	case 0x1d:
		// TODO impl
	case 0x1e:
		// TODO impl
	case 0x1f:
		// TODO impl
	case 0x20:
		// TODO impl
	case 0x21:
		// TODO impl
	case 0x22:
		// TODO impl
	case 0x23:
		// TODO impl
	case 0x24:
		// TODO impl
	case 0x25:
		// TODO impl
	case 0x26:
		// TODO impl
	case 0x27:
		// TODO impl
	case 0x28:
		// TODO impl
	case 0x29:
		// TODO impl
	case 0x2a:
		// TODO impl
	case 0x2b:
		// TODO impl
	case 0x2c:
		// TODO impl
	case 0x2d:
		// TODO impl
	case 0x2e:
		// TODO impl
	case 0x2f:
		// TODO impl
	case 0x30:
		// TODO impl
	case 0x31:
		// TODO impl
	case 0x32:
		// TODO impl
	case 0x33:
		// TODO impl
	case 0x34:
		// TODO impl
	case 0x35:
		// TODO impl
	case 0x36:
		// TODO impl
	case 0x37:
		// TODO impl
	case 0x38:
		// TODO impl
	case 0x39:
		// TODO impl
	case 0x3a:
		// TODO impl
	case 0x3b:
		// TODO impl
	case 0x3c:
		// TODO impl
	case 0x3d:
		// TODO impl
	case 0x3e:
		// TODO impl
	case 0x3f:
		// TODO impl
	case 0x40:
		// TODO impl
	case 0x41:
		// TODO impl
	case 0x42:
		// TODO impl
	case 0x43:
		// TODO impl
	case 0x44:
		// TODO impl
	case 0x45:
		// TODO impl
	case 0x46:
		// TODO impl
	case 0x47:
		// TODO impl
	case 0x48:
		// TODO impl
	case 0x49:
		// TODO impl
	case 0x4a:
		// TODO impl
	case 0x4b:
		// TODO impl
	case 0x4c:
		// TODO impl
	case 0x4d:
		// TODO impl
	case 0x4e:
		// TODO impl
	case 0x4f:
		// TODO impl
	case 0x50:
		// TODO impl
	case 0x51:
		// TODO impl
	case 0x52:
		// TODO impl
	case 0x53:
		// TODO impl
	case 0x54:
		// TODO impl
	case 0x55:
		// TODO impl
	case 0x56:
		// TODO impl
	case 0x57:
		// TODO impl
	case 0x58:
		// TODO impl
	case 0x59:
		// TODO impl
	case 0x5a:
		// TODO impl
	case 0x5b:
		// TODO impl
	case 0x5c:
		// TODO impl
	case 0x5d:
		// TODO impl
	case 0x5e:
		// TODO impl
	case 0x5f:
		// TODO impl
	case 0x60:
		// TODO impl
	case 0x61:
		// TODO impl
	case 0x62:
		// TODO impl
	case 0x63:
		// TODO impl
	case 0x64:
		// TODO impl
	case 0x65:
		// TODO impl
	case 0x66:
		// TODO impl
	case 0x67:
		// TODO impl
	case 0x68:
		// TODO impl
	case 0x69:
		// TODO impl
	case 0x6a:
		// TODO impl
	case 0x6b:
		// TODO impl
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

/*
TODO impl

private void PHP(IMemory memory)
	{
		BreakCommandFlag = true;
		Push8(memory, Flags);
		BreakCommandFlag = false;
		PC += 1;
		ClockCycles += 3;
	}

	private void PLP(IMemory memory)
	{
		Flags = Pop8(memory);
		BreakCommandFlag = false;
		UnusedFlag = true;
		PC += 1;
		ClockCycles += 4;
	}

	private void RTI(IMemory memory)
	{
		Flags = Pop8(memory);
		PC = Pop16(memory);
		BreakCommandFlag = false;
		UnusedFlag = true;
		ClockCycles += 6;
	}

	private void RTS(IMemory memory)
	{
		PC = (ushort)(Pop16(memory) + 1);
		ClockCycles += 6;
	}

	private void PHA(IMemory memory)
	{
		Push8(memory, A);
		PC += 1;
		ClockCycles += 3;
	}
*/

func (c *CPU) oraZeroPageIndirectX(m Memory) {
	_, value := c.zeroPageIndirectX(m)
	c.oraCommon(value, 2, 6)
}

/*
TODO impl

	private void ORA_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (_, value, extraClock) = ZeroPageIndirectY(memory);
		ORA_Common(value, 2, 5 + extraClock);
	}
*/

func (c *CPU) oraZeroPageFixed(m Memory) {
	_, value := c.zeroPageFixed(m)
	c.oraCommon(value, 2, 3)
}

/*
TODO impl

	private void ORA_ZeroPage_Fixed(IMemory memory)
	{
		var (_, value) = ZeroPageFixed(memory);
		ORA_Common(value, 2, 3);
	}

	private void ORA_Immediate(IMemory memory)
	{
		var value = memory.Read8((ushort)(PC + 1));
		ORA_Common(value, 2, 2);
	}

	private void ORA_Absolute(IMemory memory)
	{
		var (_, value) = Absolute(memory);
		ORA_Common(value, 3, 4);
	}

	private void ORA_ZeroPage_X(IMemory memory)
	{
		var (_, value) = ZeroPageX(memory);
		ORA_Common(value, 2, 4);
	}

	private void ORA_Absolute_X(IMemory memory)
	{
		var (_, value, extraClock) = AbsoluteX(memory);
		ORA_Common(value, 3, 4 + extraClock);
	}

	private void ORA_Absolute_Y(IMemory memory)
	{
		var (_, value, extraClock) = AbsoluteY(memory);
		ORA_Common(value, 3, 4 + extraClock);
	}
*/

func (c *CPU) oraCommon(newValue uint8, pcOffset uint16, clock uint64) {
	c.A |= newValue
	if int8(c.A) < 0 {
		c.setFlags(NegativeFlagMask)
	} else {
		c.clearFlags(NegativeFlagMask)
	}
	if c.A == 0 {
		c.setFlags(ZeroFlagMask)
	} else {
		c.clearFlags(ZeroFlagMask)
	}
	c.PC += pcOffset
	c.ClockTime += clock
}

/*
TODO impl

	private void ORA_Common(byte newValue, UInt16 pcOffset, UInt64 clock)
	{
		A |= newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		PC += pcOffset;
		ClockCycles += clock;
	}
*/

func (c *CPU) sloZeroPageIndirectX(m Memory) {
	address, value := c.zeroPageIndirectX(m)
	c.sloCommon(m, address, value, 2, 8)
}

/*
	private void SLO_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (address, value, _) = ZeroPageIndirectY(memory);
		SLO_Common(memory, address, value, 2, 8);
	}

	private void SLO_ZeroPage_Immediate(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		SLO_Common(memory, address, value, 2, 5);
	}

	private void SLO_Absolute(IMemory memory)
	{
		var (address, value) = Absolute(memory);
		SLO_Common(memory, address, value, 3, 6);
	}

	private void SLO_ZeroPage_X(IMemory memory)
	{
		var (address, value) = ZeroPageX(memory);
		SLO_Common(memory, address, value, 2, 6);
	}

	private void SLO_Absolute_X(IMemory memory)
	{
		var (address, value, _) = AbsoluteX(memory);
		SLO_Common(memory, address, value, 3, 7);
	}

	private void SLO_Absolute_Y(IMemory memory)
	{
		var (address, value, _) = AbsoluteY(memory);
		SLO_Common(memory, address, value, 3, 7);
	}
*/

func (c *CPU) sloCommon(m Memory, address uint16, value uint8, pcOffset uint16, clock uint64) {
	newValue := value << 1
	m.Write(address, newValue)
	c.A |= newValue
	if int8(c.A) < 0 {
		c.setFlags(NegativeFlagMask)
	} else {
		c.clearFlags(NegativeFlagMask)
	}
	if c.A == 0 {
		c.setFlags(ZeroFlagMask)
	} else {
		c.clearFlags(ZeroFlagMask)
	}
	if newValue < value {
		c.setFlags(CarryFlagMask)
	} else {
		c.clearFlags(CarryFlagMask)
	}
	c.PC += pcOffset
	c.ClockTime += clock
}

/*
TODO impl

	private void SLO_Common(IMemory memory, UInt16 address, byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = (byte)(value << 1);
		memory.Write8(address, newValue);
		A |= newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		CarryFlag = newValue < value;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void ASL(IMemory memory)
	{
		var value = A;
		var newValue = (byte)(value << 1);
		A = newValue;
		NegativeFlag = (sbyte)newValue < 0;
		ZeroFlag = newValue == 0;
		CarryFlag = newValue < value;
		PC += 1;
		ClockCycles += 2;
	}

	private void ASL_ZeroPage_Fixed(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		ASL_Common(memory, address, value, 2, 5);
	}

	private void ASL_Absolute(IMemory memory)
	{
		var (address, value) = Absolute(memory);
		ASL_Common(memory, address, value, 3, 6);
	}

	private void ASL_ZeroPage_X(IMemory memory)
	{
		var (address, value) = ZeroPageX(memory);
		ASL_Common(memory, address, value, 2, 6);
	}

	private void ASL_Absolute_X(IMemory memory)
	{
		var (address, value, _) = AbsoluteX(memory);
		ASL_Common(memory, address, value, 3, 7);
	}

	private void ASL_Common(IMemory memory, UInt16 address, byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = (byte)(value << 1);
		memory.Write8(address, newValue);
		NegativeFlag = (sbyte)newValue < 0;
		ZeroFlag = newValue == 0;
		CarryFlag = newValue < value;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void ANC_Immediate(IMemory memory)
	{
		var value = memory.Read8((ushort)(PC + 1));
		var newValue = (byte)(A & value);
		A = newValue;
		NegativeFlag = (sbyte)newValue < 0;
		ZeroFlag = newValue == 0;
		CarryFlag = (newValue & 0b1000_0000) != 0;
		PC += 2;
		ClockCycles += 2;
	}

	private void AND_ZeroPage(IMemory memory)
	{
		var (_, value) = ZeroPageFixed(memory);
		AND_Common(value, 2, 3);
	}

	private void AND_ZeroPage_X(IMemory memory)
	{
		var (_, value) = ZeroPageX(memory);
		AND_Common(value, 2, 4);
	}

	private void AND_ZeroPage_Indirect_X(IMemory memory)
	{
		var (_, value) = ZeroPageIndirectX(memory);
		AND_Common(value, 2, 6);
	}

	private void AND_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (_, value, extraClock) = ZeroPageIndirectY(memory);
		AND_Common(value, 2, 5 + extraClock);
	}

	private void AND_Immediate(IMemory memory)
	{
		var value = memory.Read8((ushort)(PC + 1));
		AND_Common(value, 2, 2);
	}

	private void AND_Absolute(IMemory memory)
	{
		var (_, value) = Absolute(memory);
		AND_Common(value, 3, 4);
	}

	private void AND_Absolute_X(IMemory memory)
	{
		var (_, value, extraClock) = AbsoluteX(memory);
		AND_Common(value, 3, 4 + extraClock);
	}

	private void AND_Absolute_Y(IMemory memory)
	{
		var (_, value, extraClock) = AbsoluteY(memory);
		AND_Common(value, 3, 4 + extraClock);
	}

	private void AND_Common(byte value, UInt16 pcOffset, UInt64 clock)
	{
		A &= value;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void RLA_ZeroPage(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		RLA_Common(memory, address, value, 2, 5);
	}

	private void RLA_ZeroPage_X(IMemory memory)
	{
		var (address, value) = ZeroPageX(memory);
		RLA_Common(memory, address, value, 2, 6);
	}

	private void RLA_ZeroPage_Indirect_X(IMemory memory)
	{
		var (address, value) = ZeroPageIndirectX(memory);
		RLA_Common(memory, address, value, 2, 8);
	}

	private void RLA_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (address, value, _) = ZeroPageIndirectY(memory);
		RLA_Common(memory, address, value, 2, 8);
	}

	private void RLA_Absolute(IMemory memory)
	{
		var (address, value) = Absolute(memory);
		RLA_Common(memory, address, value, 3, 6);
	}

	private void RLA_Absolute_X(IMemory memory)
	{
		var (address, value, _) = AbsoluteX(memory);
		RLA_Common(memory, address, value, 3, 7);
	}

	private void RLA_Absolute_Y(IMemory memory)
	{
		var (address, value, _) = AbsoluteY(memory);
		RLA_Common(memory, address, value, 3, 7);
	}

	private void RLA_Common(IMemory memory, UInt16 address, byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = (byte)((byte)(value << 1) | (CarryFlag ? 1 : 0));
		memory.Write8(address, newValue);
		A &= newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		CarryFlag = (value & 0b1000_0000) != 0;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void ROL()
	{
		A = ROL_Common(A, 1, 2);
	}

	private void ROL_ZeroPage(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		var newValue = ROL_Common(value, 2, 5);
		memory.Write8(address, newValue);
	}

	private void ROL_ZeroPage_X(IMemory memory)
	{
		var (address, value) = ZeroPageX(memory);
		var newValue = ROL_Common(value, 2, 6);
		memory.Write8(address, newValue);
	}

	private void ROL_Absolute(IMemory memory)
	{
		var (address, value) = Absolute(memory);
		var newValue = ROL_Common(value, 3, 6);
		memory.Write8(address, newValue);
	}

	private void ROL_Absolute_X(IMemory memory)
	{
		var (address, value, _) = AbsoluteX(memory);
		var newValue = ROL_Common(value, 3, 7);
		memory.Write8(address, newValue);
	}

	private byte ROL_Common(byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = (byte)((byte)(value << 1) | (CarryFlag ? 1 : 0));
		NegativeFlag = (sbyte)newValue < 0;
		ZeroFlag = newValue == 0;
		CarryFlag = (value & 0b1000_0000) != 0;
		PC += pcOffset;
		ClockCycles += clock;
		return newValue;
	}

	private void BPL(IMemory memory)
	{
		Branch_Common(memory, !NegativeFlag);
	}

	private void BMI(IMemory memory)
	{
		Branch_Common(memory, NegativeFlag);
	}

	private void BVC(IMemory memory)
	{
		Branch_Common(memory, !OverflowFlag);
	}

	private void Branch_Common(IMemory memory, bool condition)
	{
		if (condition)
		{
			// high byte of address after the branch instruction
			var high1 = (PC + 2) & 0xff00;
			// do the jump
			PC = (ushort)(PC + 2 + (sbyte)memory.Read8((ushort)(PC + 1)));
			// high byte of address of branch destination
			var high2 = PC & 0xff00;
			if (high1 == high2)
			{
				ClockCycles += 3;
			}
			else
			{
				ClockCycles += 4;
			}
		}
		else
		{
			PC += 2;
			ClockCycles += 2;
		}
	}

	private void JMP_Absolute(IMemory memory)
	{
		PC = memory.Read16((ushort)(PC + 1));
		ClockCycles += 3;
	}

	private void CLC()
	{
		CarryFlag = false;
		PC += 1;
		ClockCycles += 2;
	}

	private void SEC()
	{
		CarryFlag = true;
		PC += 1;
		ClockCycles += 2;
	}

	private void CLI()
	{
		InterruptDisableFlag = false;
		PC += 1;
		ClockCycles += 2;
	}

	private void JSR(IMemory memory)
	{
		var address = memory.Read16((ushort)(PC + 1));
		Push16(memory, (ushort)(PC + 2));
		PC = address;
		ClockCycles += 6;
	}

	private void BIT_ZeroPage_Immediate(IMemory memory)
	{
		var (_, value) = ZeroPageFixed(memory);
		BIT_Common(value, 2, 3);
	}

	private void BIT_Absolute(IMemory memory)
	{
		var (_, value) = Absolute(memory);
		BIT_Common(value, 3, 4);
	}

	private void BIT_Common(byte value, UInt16 pcOffset, UInt64 clock)
	{
		OverflowFlag = (OverflowFlagMask & value) != 0;
		NegativeFlag = (NegativeFlagMask & value) != 0;
		ZeroFlag = (value & A) == 0;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void EOR_ZeroPage(IMemory memory)
	{
		var (_, value) = ZeroPageFixed(memory);
		EOR_Common(value, 2, 3);
	}

	private void EOR_ZeroPage_X(IMemory memory)
	{
		var (_, value) = ZeroPageX(memory);
		EOR_Common(value, 2, 4);
	}

	private void EOR_ZeroPage_Indirect_X(IMemory memory)
	{
		var (_, value) = ZeroPageIndirectX(memory);
		EOR_Common(value, 2, 6);
	}

	private void EOR_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (_, value, extraClock) = ZeroPageIndirectY(memory);
		EOR_Common(value, 2, 5 + extraClock);
	}

	private void EOR_Immediate(IMemory memory)
	{
		var value = memory.Read8((ushort)(PC + 1));
		EOR_Common(value, 2, 2);
	}

	private void EOR_Absolute(IMemory memory)
	{
		var (_, value) = Absolute(memory);
		EOR_Common(value, 3, 4);
	}

	private void EOR_Absolute_X(IMemory memory)
	{
		var (_, value, extraClock) = AbsoluteX(memory);
		EOR_Common(value, 3, 4 + extraClock);
	}

	private void EOR_Absolute_Y(IMemory memory)
	{
		var (_, value, extraClock) = AbsoluteY(memory);
		EOR_Common(value, 3, 4 + extraClock);
	}

	private void EOR_Common(byte value, UInt16 pcOffset, UInt64 clock)
	{
		A ^= value;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void SRE_ZeroPage(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		SRE_Common(memory, address, value, 2, 5);
	}

	private void SRE_ZeroPage_X(IMemory memory)
	{
		var (address, value) = ZeroPageX(memory);
		SRE_Common(memory, address, value, 2, 6);
	}

	private void SRE_ZeroPage_Indirect_X(IMemory memory)
	{
		var (address, value) = ZeroPageIndirectX(memory);
		SRE_Common(memory, address, value, 2, 8);
	}

	private void SRE_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (address, value, _) = ZeroPageIndirectY(memory);
		SRE_Common(memory, address, value, 2, 8);
	}

	private void SRE_Absolute(IMemory memory)
	{
		var (address, value) = Absolute(memory);
		SRE_Common(memory, address, value, 3, 6);
	}

	private void SRE_Absolute_X(IMemory memory)
	{
		var (address, value, _) = AbsoluteX(memory);
		SRE_Common(memory, address, value, 3, 7);
	}

	private void SRE_Absolute_Y(IMemory memory)
	{
		var (address, value, _) = AbsoluteY(memory);
		SRE_Common(memory, address, value, 3, 7);
	}

	private void SRE_Common(IMemory memory, UInt16 address, byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = (byte)(value >> 1);
		memory.Write8(address, newValue);
		A ^= newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		CarryFlag = (value & 0b0000_0001) != 0;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void LSR()
	{
		A = LSR_Common(A, 1, 2);
	}

	private void LSR_ZeroPage(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		var newValue = LSR_Common(value, 2, 5);
		memory.Write8(address, newValue);
	}

	private void LSR_ZeroPage_X(IMemory memory)
	{
		var (address, value) = ZeroPageX(memory);
		var newValue = LSR_Common(value, 2, 6);
		memory.Write8(address, newValue);
	}

	private void LSR_Absolute(IMemory memory)
	{
		var (address, value) = Absolute(memory);
		var newValue = LSR_Common(value, 3, 6);
		memory.Write8(address, newValue);
	}

	private void LSR_Absolute_X(IMemory memory)
	{
		var (address, value, _) = AbsoluteX(memory);
		var newValue = LSR_Common(value, 3, 7);
		memory.Write8(address, newValue);
	}

	private byte LSR_Common(byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = (byte)(value >> 1);
		NegativeFlag = false;
		ZeroFlag = newValue == 0;
		CarryFlag = (value & 0b0000_0001) != 0;
		PC += pcOffset;
		ClockCycles += clock;
		return newValue;
	}

	private void ALR(IMemory memory)
	{
		var value = (byte)(A & memory.Read8((ushort)(PC + 1)));
		A = (byte)(value >> 1);
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		CarryFlag = (value & 0b0000_0001) != 0;
		PC += 2;
		ClockCycles += 2;
	}

	private void ADC_ZeroPage_Indirect_X(IMemory memory)
	{
		var (_, value) = ZeroPageIndirectX(memory);
		ADC_Common(value, 2, 6);
	}

	private void ADC_Common(byte value, UInt16 pcOffset, UInt64 clock)
	{
		var newValue = A + value + (CarryFlag ? 1 : 0);
		var oldA = A;
		A = (byte)newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		CarryFlag = (newValue & 0b1_0000_0000) != 0;
		var valueSignBit = value & 0b1000_0000;
		var oldASignBit = oldA & 0b1000_0000;
		var newSignBit = newValue & 0b1000_0000;
		OverflowFlag = valueSignBit == oldASignBit && valueSignBit != newSignBit;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void RRA_ZeroPage_Indirect_X(IMemory memory)
	{
		var (address, value) = ZeroPageIndirectX(memory);
		RRA_Common(address, value, 2, 8);
	}

	private void RRA_Common(UInt16 address, byte value, UInt16 pcOffset, UInt64 clock)
	{
		var rorNewValue = (byte)((value >> 1) | (CarryFlag ? 0b1000_0000 : 0));
		var adcNewValue = A + value + (CarryFlag ? 1 : 0);
		// TODO JEFF but what now?
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void NOP(UInt16 pcOffset, UInt16 clock)
	{
		PC += pcOffset;
		ClockCycles += clock;
	}
*/

func (c *CPU) nop(pcOffset uint16, clock uint64) {
	c.PC += pcOffset
	c.ClockTime += clock
}

/*
TODO impl

	private void NOP_Absolute_X(IMemory memory)
	{
		var (_, _, extraClock) = AbsoluteX(memory);
		PC += 3;
		ClockCycles += 4 + extraClock;
	}
*/

func (c *CPU) clearFlags(mask uint8) {
	c.Flags &= ^mask
}

func (c *CPU) setFlags(mask uint8) {
	c.Flags |= mask
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
	address = uint16(m.Read(c.PC+1)) + uint16(c.X)&0xff
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

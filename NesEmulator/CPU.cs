using System.Runtime.InteropServices;

namespace NesEmulator;

public class CPU
{
	private const byte CarryFlagMask = 0b0000_0001;
	private const byte ZeroFlagMask = 0b0000_0010;
	private const byte InterruptDisableFlagMask = 0b0000_0100;
	private const byte DecimalModeFlagMask = 0b0000_1000;
	private const byte BreakCommandFlagMask = 0b0001_0000;
	private const byte UnusedFlagMask = 0b0010_0000;
	private const byte OverflowFlagMask = 0b0100_0000;
	private const byte NegativeFlagMask = 0b1000_0000;

	private const UInt16 StackAddress = 0x0100;

	private const UInt16 NonMaskableInterruptAddress = 0xfffa;
	private const UInt16 ResetInterruptAddress = 0xfffc;
	private const UInt16 InterruptRequestInterruptAddress = 0xfffe;

	public UInt16 PC;
	public byte SP;
	public byte A;
	public byte X;
	public byte Y;
	public byte Flags;

	public UInt64 ClockCycles = 0;

	public bool CarryFlag
	{
		get => (Flags & CarryFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= CarryFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~CarryFlagMask);
				}
			}
		}
	}

	public bool ZeroFlag
	{
		get => (Flags & ZeroFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= ZeroFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~ZeroFlagMask);
				}
			}
		}
	}

	public bool InterruptDisableFlag
	{
		get => (Flags & InterruptDisableFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= InterruptDisableFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~InterruptDisableFlagMask);
				}
			}
		}
	}

	public bool DecimalModeFlag
	{
		get => (Flags & DecimalModeFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= DecimalModeFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~DecimalModeFlagMask);
				}
			}
		}
	}

	public bool BreakCommandFlag
	{
		get => (Flags & BreakCommandFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= BreakCommandFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~BreakCommandFlagMask);
				}
			}
		}
	}

	public bool UnusedFlag
	{
		get => (Flags & UnusedFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= UnusedFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~UnusedFlagMask);
				}
			}
		}
	}

	public bool OverflowFlag
	{
		get => (Flags & OverflowFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= OverflowFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~OverflowFlagMask);
				}
			}
		}
	}

	public bool NegativeFlag
	{
		get => (Flags & NegativeFlagMask) != 0;
		set
		{
			if (value)
			{
				Flags |= NegativeFlagMask;
			}
			else
			{
				unchecked
				{
					Flags &= (byte)(~NegativeFlagMask);
				}
			}
		}
	}

	public void Step(IMemory memory)
	{
		var instruction = memory.Read8(PC);
		switch (instruction)
		{
			case 0x00: BRK(memory); break;
			case 0x01: ORA_ZeroPage_Indirect_X(memory); break;
			case 0x02: NOP(0, 3); break;
			case 0x03: SLO_ZeroPage_Indirect_X(memory); break;
			case 0x04: NOP(2, 3); break;
			case 0x05: ORA_ZeroPage_Fixed(memory); break;
			case 0x06: ASL_ZeroPage_Fixed(memory); break;
			case 0x07: SLO_ZeroPage_Immediate(memory); break;
			case 0x08: PHP(memory); break;
			case 0x09: ORA_Immediate(memory); break;
			case 0x0a: ASL(memory); break;
			case 0x0b: ANC_Immediate(memory); break;
			case 0x0c: NOP(3, 4); break;
			case 0x0d: ORA_Absolute(memory); break;
			case 0x0e: ASL_Absolute(memory); break;
			case 0x0f: SLO_Absolute(memory); break;
			case 0x10: BPL(memory); break;
			case 0x11: ORA_ZeroPage_Indirect_Y(memory); break;
			case 0x12: NOP(0, 3); break;
			case 0x13: SLO_ZeroPage_Indirect_Y(memory); break;
			case 0x14: NOP(2, 4); break;
			case 0x15: ORA_ZeroPage_X(memory); break;
			case 0x16: ASL_ZeroPage_X(memory); break;
			case 0x17: SLO_ZeroPage_X(memory); break;
			case 0x18: CLC(); break;
			case 0x19: ORA_Absolute_Y(memory); break;
			case 0x1a: NOP(1, 2); break;
			case 0x1b: SLO_Absolute_Y(memory); break;
			case 0x1c: NOP_Absolute_X(memory); break;
			case 0x1d: ORA_Absolute_X(memory); break;
			case 0x1e: ASL_Absolute_X(memory); break;
			case 0x1f: SLO_Absolute_X(memory); break;
			case 0x20: JSR(memory); break;
			case 0x21: AND_ZeroPage_Indirect_X(memory); break;
			case 0x22: NOP(0, 3); break;
			case 0x23: RLA_ZeroPage_Indirect_X(memory); break;
			case 0x24: BIT_ZeroPage_Immediate(memory); break;
			case 0x25: AND_ZeroPage(memory); break;
			case 0x26: ROL_ZeroPage(memory); break;
			case 0x27: RLA_ZeroPage(memory); break;
			case 0x28: PLP(memory); break;
			case 0x29: AND_Immediate(memory); break;
			case 0x2a: ROL(); break;
			case 0x2b: ANC_Immediate(memory); break;
			case 0x2c: BIT_Absolute(memory); break;
			case 0x2d: AND_Absolute(memory); break;
			case 0x2e: ROL_Absolute(memory); break;
			case 0x2f: RLA_Absolute(memory); break;
			case 0x30: BMI(memory); break;
			case 0x31: AND_ZeroPage_Indirect_Y(memory); break;
			case 0x32: NOP(0, 3); break;
			case 0x33: RLA_ZeroPage_Indirect_Y(memory); break;
			case 0x34: NOP(2, 4); break;
			case 0x35: AND_ZeroPage_X(memory); break;
			case 0x36: ROL_ZeroPage_X(memory); break;
			case 0x37: RLA_ZeroPage_X(memory); break;
			case 0x38: SEC(memory); break;
			case 0x39: AND_Absolute_Y(memory); break;
			case 0x3a: NOP(1, 2); break;
			case 0x3b: RLA_Absolute_Y(memory); break;
			case 0x3c: NOP_Absolute_X(memory); break;
			case 0x3d: AND_Absolute_X(memory); break;
			case 0x3e: ROL_Absolute_X(memory); break;
			case 0x3f: RLA_Absolute_X(memory); break;
			case 0x40: RTI(memory); break;
			case 0x41: EOR_ZeroPage_Indirect_X(memory); break;
			case 0x42: NOP(0, 3); break;
			case 0x43: SRE_ZeroPage_Indirect_X(memory); break;
			case 0x44: NOP(2, 3); break;
			case 0x45: EOR_ZeroPage(memory); break;
			case 0x46: LSR_ZeroPage(memory); break;
			case 0x47: SRE_ZeroPage(memory); break;
				// TODO remaining instructions
		}
	}

	private void BRK(IMemory memory)
	{
		Push16(memory, (ushort)(PC + 2));
		BreakCommandFlag = true;
		Push8(memory, Flags);
		BreakCommandFlag = false;
		InterruptDisableFlag = true;
		PC = memory.Read16(InterruptRequestInterruptAddress);
		ClockCycles += 7;
	}

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

	private void ORA_ZeroPage_Indirect_X(IMemory memory)
	{
		var (_, value) = ZeroPageIndirectX(memory);
		ORA_Common(value, 2, 6);
	}

	private void ORA_ZeroPage_Indirect_Y(IMemory memory)
	{
		var (_, value, extraClock) = ZeroPageIndirectY(memory);
		ORA_Common(value, 2, 5 + extraClock);
	}

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

	private void ORA_Common(byte newValue, UInt16 pcOffset, UInt64 clock)
	{
		A |= newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void SLO_ZeroPage_Indirect_X(IMemory memory)
	{
		var (address, value) = ZeroPageIndirectX(memory);
		SLO_Common(memory, address, value, 2, 8);
	}

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

	private void CLC()
	{
		CarryFlag = false;
		PC += 1;
		ClockCycles += 2;
	}

	private void SEC(IMemory memory)
	{
		CarryFlag = true;
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

	private void EOR_ZeroPage_Indirect_X(IMemory memory)
	{
		var (_, value) = ZeroPageIndirectX(memory);
		EOR_Common(value, 2, 6);
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

	private void SRE_ZeroPage_Indirect_X(IMemory memory)
	{
		var (address, value) = ZeroPageIndirectX(memory);
		SRE_Common(memory, address, value, 2, 8);
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

	private void LSR_ZeroPage(IMemory memory)
	{
		var (address, value) = ZeroPageFixed(memory);
		var newValue = LSR_Common(value, 2, 5);
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

	private void NOP(UInt16 pcOffset, UInt16 clock)
	{
		PC += pcOffset;
		ClockCycles += clock;
	}

	private void NOP_Absolute_X(IMemory memory)
	{
		var (_, _, extraClock) = AbsoluteX(memory);
		PC += 3;
		ClockCycles += 4 + extraClock;
	}

	private void Push8(IMemory memory, byte value)
	{
		memory.Write8((ushort)(StackAddress + SP), value);
		SP--;
	}

	private byte Pop8(IMemory memory)
	{
		SP++;
		return memory.Read8((ushort)(StackAddress + SP));
	}

	private void Push16(IMemory memory, UInt16 value)
	{
		Push8(memory, (byte)((value & 0xff00) >> 8));
		Push8(memory, (byte)(value & 0xff));
	}

	private UInt16 Pop16(IMemory memory)
	{
		var low = Pop8(memory);
		var high = Pop8(memory);
		return (ushort)((high << 8) | low);
	}

	private (UInt16, byte) Absolute(IMemory memory)
	{
		var address = memory.Read16((ushort)(PC + 1));
		return (address, memory.Read8(address));
	}

	private (UInt16, byte, UInt64) AbsoluteX(IMemory memory) => Absolute_Common(memory, X);

	private (UInt16, byte, UInt64) AbsoluteY(IMemory memory) => Absolute_Common(memory, Y);

	private (UInt16, byte, UInt64) Absolute_Common(IMemory memory, byte offset)
	{
		var address = memory.Read16((ushort)(PC + 1));
		var high1 = address & 0xff00;
		address += offset;
		var high2 = address & 0xff00;
		UInt64 extraClock;
		// if adding Y pushes us into a new page it will take an extra clock cycle to resolve
		if (high1 == high2)
		{
			extraClock = 0;
		}
		else
		{
			extraClock = 1;
		}
		return (address, memory.Read8(address), extraClock);
	}

	private (UInt16, byte) ZeroPageFixed(IMemory memory)
	{
		var address = (UInt16)memory.Read8((ushort)(PC + 1));
		return (address, memory.Read8(address));
	}

	private (UInt16, byte) ZeroPageX(IMemory memory)
	{
		var address = (UInt16)((memory.Read8((ushort)(PC + 1)) + X) & 0xff);
		return (address, memory.Read8(address));
	}

	private (UInt16, byte) ZeroPageIndirectX(IMemory memory)
	{
		var address = ZeroPageIndirect(memory, memory.Read8((ushort)(PC + 1)), X);
		return (address, memory.Read8(address));
	}

	private (UInt16, byte, UInt64) ZeroPageIndirectY(IMemory memory)
	{
		var address = ZeroPageIndirect(memory, memory.Read8((ushort)(PC + 1)), 0);
		var high1 = address & 0xff00;
		address = (UInt16)(address + Y);
		var high2 = address & 0xff00;
		UInt64 extraClock;
		// if adding Y pushes us into a new page it will take an extra clock cycle to resolve
		if (high1 == high2)
		{
			extraClock = 0;
		}
		else
		{
			extraClock = 1;
		}
		return (address, memory.Read8(address), extraClock);
	}

	// TODO extension method on memory?
	private UInt16 ZeroPageIndirect(IMemory memory, byte offset1, byte offset2)
	{
		var newOffset = (UInt16)((offset1) + offset2);
		// address must be on the zero page
		var newOffsetLow = (byte)(newOffset & 0xff);
		var resultLow = memory.Read8(newOffsetLow);
		// address must be on the zero page
		var resultHigh = memory.Read8((ushort)((newOffsetLow + 1) & 0xff));
		return (ushort)(resultLow | (resultHigh << 8));
	}
}
using System.Runtime.InteropServices;

namespace NesEmulator;

public class CPU
{
	private const byte CarryFlagMask = 0b0000_0001;
	private const byte ZeroFlagMask = 0b0000_0010;
	private const byte InterruptDisableFlagMask = 0b0000_0100;
	private const byte DecimalModeFlagMask = 0b0000_1000;
	private const byte BreakCommandFlagMask = 0b0001_0000;
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
			case 0x00: Break(memory); break;
			case 0x01: ORA_X_Indirect(memory); break;
			case 0x02: NOP(0, 3); break;
			case 0x03: SLO_ZeroPage_Indirect_X(memory); break;
			case 0x04: NOP(2, 3); break;
			case 0x05: ORA_ZeroPage_Fixed(memory); break;
			case 0x06: ASL_ZeroPage_Fixed(memory); break;
			case 0x07: SLO_ZeroPageImmediate(memory); break;
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
				// TODO remaining instructions
		}
	}

	private void Break(IMemory memory)
	{
		Push16(memory, (ushort)(PC + 2));
		BreakCommandFlag = true;
		Push8(memory, Flags);
		BreakCommandFlag = false;
		InterruptDisableFlag = true;
		PC = memory.Read16(InterruptRequestInterruptAddress);
		ClockCycles += 7;
	}

	private void ORA_X_Indirect(IMemory memory)
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

	private void SLO_ZeroPageImmediate(IMemory memory)
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

	private void PHP(IMemory memory)
	{
		BreakCommandFlag = true;
		Push8(memory, Flags);
		BreakCommandFlag = false;
		PC += 1;
		ClockCycles += 3;
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

	private void BPL(IMemory memory)
	{
		// TODO common jump?
		if (!NegativeFlag)
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

	private void NOP(UInt16 pcOffset, UInt16 cycleCount)
	{
		PC += pcOffset;
		ClockCycles += cycleCount;
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
		var high = Pop8(memory);
		var low = Pop8(memory);
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
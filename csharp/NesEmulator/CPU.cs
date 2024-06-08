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

	public UInt16 ClockCycles = 0;

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
			case 0x02: Illegal(0, 3); break;
			case 0x03: SLO_X_Indirect(memory); break;
			case 0x04: Illegal(2, 3); break;
			case 0x05: ORA_FixedZeroPage(memory); break;
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
		var address = ZeroPageIndirect(memory, memory.Read8((ushort)(PC + 1)), X);
		A |= memory.Read8(address);
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		PC += 2;
		ClockCycles += 6;
	}

	private void ORA_FixedZeroPage(IMemory memory)
	{
		var address = (UInt16)memory.Read8((ushort)(PC + 1));
		A |= memory.Read8(address);
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		PC += 2;
		ClockCycles += 3;
	}

	private void SLO_X_Indirect(IMemory memory)
	{
		var address = ZeroPageIndirect(memory, memory.Read8((ushort)(PC + 1)), X);
		var value = memory.Read8(address);
		var newValue = (byte)(value << 1);
		memory.Write8(address, newValue);
		A |= newValue;
		NegativeFlag = (sbyte)A < 0;
		ZeroFlag = A == 0;
		CarryFlag = newValue < value;
		PC += 2;
		ClockCycles += 8;
	}

	private void Illegal(UInt16 pcOffset, UInt16 cycleCount)
	{
		PC += pcOffset;
		ClockCycles += cycleCount;
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
using NesEmulator;

namespace NesEmulatorTests;

public class CPUTests
{
	[Fact]
	public void FlagsTest()
	{
		var cpu = new CPU();
		cpu.Flags = 0;

		Assert.Equal(0b0000_0000, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.CarryFlag = true;

		Assert.Equal(0b0000_0001, cpu.Flags);
		Assert.True(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.CarryFlag = false;
		cpu.ZeroFlag = true;

		Assert.Equal(0b0000_0010, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.True(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.ZeroFlag = false;
		cpu.InterruptDisableFlag = true;

		Assert.Equal(0b0000_0100, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.True(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.InterruptDisableFlag = false;
		cpu.DecimalModeFlag = true;

		Assert.Equal(0b0000_1000, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.True(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.DecimalModeFlag = false;
		cpu.BreakCommandFlag = true;

		Assert.Equal(0b0001_0000, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.True(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.BreakCommandFlag = false;
		cpu.OverflowFlag = true;

		Assert.Equal(0b0100_0000, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.True(cpu.OverflowFlag);
		Assert.False(cpu.NegativeFlag);

		cpu.OverflowFlag = false;
		cpu.NegativeFlag = true;

		Assert.Equal(0b1000_0000, cpu.Flags);
		Assert.False(cpu.CarryFlag);
		Assert.False(cpu.ZeroFlag);
		Assert.False(cpu.InterruptDisableFlag);
		Assert.False(cpu.DecimalModeFlag);
		Assert.False(cpu.BreakCommandFlag);
		Assert.False(cpu.OverflowFlag);
		Assert.True(cpu.NegativeFlag);
	}
}
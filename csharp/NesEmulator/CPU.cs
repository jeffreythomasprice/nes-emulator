namespace NesEmulator;

public class CPU
{
	public UInt16 PC { get; set; }
	public byte SP { get; set; }
	public byte A { get; set; }
	public byte X { get; set; }
	public byte Y { get; set; }
	public byte Flags { get; set; }

	public void Step(IMemory memory)
	{
		var instruction = memory.Read8(PC);
		// TODO implement instructions
	}
}
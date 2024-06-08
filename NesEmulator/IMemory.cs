namespace NesEmulator;

public interface IMemory
{
	public byte Read8(UInt16 address);
	public void Write8(UInt16 address, byte value);

	public UInt16 Read16(UInt16 address)
	{
		var low = Read8(address);
		var high = Read8((UInt16)(address + 1));
		return (UInt16)(low | (high << 8));
	}

	public void Write16(UInt16 address)
	{
		var low = (byte)(address & 0xff);
		var high = (byte)((address & 0xff00) >> 8);
	}
}
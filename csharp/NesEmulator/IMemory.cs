namespace NesEmulator;

public interface IMemory
{
	public byte Read8(UInt16 address);
	public void Write8(UInt16 address, byte value);

	public UInt16 Read16(UInt16 address)
	{
		AssertAddress(address);
		var low = Read8(address);
		var high = Read8((UInt16)(address + 1));
		return (UInt16)(((UInt16)low) | (((UInt16)high) << (UInt16)8));
	}

	public void Write16(UInt16 address)
	{
		AssertAddress(address);
		var low = (byte)(address & 0xff);
		var high = (byte)((address & 0xff00) >> 8);
	}

	private static void AssertAddress(UInt16 address)
	{
		if (address == 0xffff)
		{
			throw new IndexOutOfRangeException("address is the last byte of addressable memory, can't get a 2-byte value here");
		}
	}
}
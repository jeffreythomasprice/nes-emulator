using NesEmulator;

namespace NesEmulatorTests;

public partial class CPUInstructionSetTests
{
	public class TestMemory : IMemory
	{
		private readonly byte[] data = new byte[0x10000];

		public byte Read8(ushort address)
		{
			return data[address];
		}

		public void Write8(ushort address, byte value)
		{
			data[address] = value;
		}
	}
}

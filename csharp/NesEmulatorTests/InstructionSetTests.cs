using System.Reflection;
using System.Text.Json;
using System.Text.Json.Serialization;
using NesEmulator;

namespace NesEmulatorTests;

public class InstructionSetTests
{
    public record class TestCase
    {
        [JsonPropertyName("name")]
        public string Name { get; set; }

        [JsonPropertyName("initial")]
        public CPUAndMemoryState Initial { get; set; }

        [JsonPropertyName("final")]
        public CPUAndMemoryState Final { get; set; }

        // TODO convert to a sensible record
        [JsonPropertyName("cycles")]
        public object[] Cycles { get; set; }
    }

    public record class CPUAndMemoryState
    {
        [JsonPropertyName("pc")]
        public UInt16 PC { get; set; }

        [JsonPropertyName("s")]
        public byte SP { get; set; }

        [JsonPropertyName("a")]
        public byte A { get; set; }

        [JsonPropertyName("x")]
        public byte X { get; set; }

        [JsonPropertyName("y")]
        public byte Y { get; set; }

        [JsonPropertyName("p")]
        public byte Flags { get; set; }

        // TODO convert to a sensible record
        [JsonPropertyName("ram")]
        public UInt16[][] RAM { get; set; }
    }

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

    [Theory]
    [MemberData(nameof(TestData))]
    public void Tests(TestCase testCase)
    {
        var cpu = new CPU();
        cpu.PC = testCase.Initial.PC;
        cpu.SP = testCase.Initial.SP;
        cpu.A = testCase.Initial.A;
        cpu.X = testCase.Initial.X;
        cpu.Y = testCase.Initial.Y;
        cpu.Flags = testCase.Initial.Flags;

        // var memory = new TestMemory();
        // foreach (var data in testCase.Initial.RAM)
        // {
        //     memory.Write8(data[0], data[1]);
        // }
    }

    public static IEnumerable<object[]> TestData
    {
        get
        {
            foreach (var path in Directory.GetFiles("../../../../../submodules/ProcessorTests/nes6502/v1", "*.json", SearchOption.AllDirectories))
            {
                using var file = File.Open(path, FileMode.Open);
                var results = JsonSerializer.Deserialize<TestCase[]>(file);
                if (results == null)
                {
                    throw new NullReferenceException($"failed to load test data for file: {path}");
                }
                foreach (var result in results)
                {
                    yield return [result];
                }
            }
        }
    }
}

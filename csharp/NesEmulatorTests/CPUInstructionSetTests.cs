using System.Text.Json;
using System.Text.Json.Serialization;
using NesEmulator;

namespace NesEmulatorTests;

public partial class CPUInstructionSetTests
{
    public record class TestCase
    {
        [JsonPropertyName("name")]
        public required string Name { get; set; }

        [JsonPropertyName("initial")]
        public required CPUAndMemoryState Initial { get; set; }

        [JsonPropertyName("final")]
        public required CPUAndMemoryState Final { get; set; }

        [JsonPropertyName("cycles")]
        public required Cycle[] Cycles { get; set; }
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

        [JsonPropertyName("ram")]
        public required MemoryState[] RAM { get; set; }
    }

    public class MemoryStateConverter : JsonConverter<MemoryState>
    {
        public override MemoryState? Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            if (reader.TokenType != JsonTokenType.StartArray)
            {
                throw new JsonException("expected start of array");
            }
            reader.Read();
            if (reader.TokenType != JsonTokenType.Number)
            {
                throw new JsonException("expected number");
            }
            var address = reader.GetUInt16();
            reader.Read();
            if (reader.TokenType != JsonTokenType.Number)
            {
                throw new JsonException("expected number");
            }
            var value = reader.GetByte();
            reader.Read();
            if (reader.TokenType != JsonTokenType.EndArray)
            {
                throw new JsonException("expected end of array");
            }
            return new MemoryState
            {
                Address = address,
                Value = value,
            };
        }

        public override void Write(Utf8JsonWriter writer, MemoryState value, JsonSerializerOptions options)
        {
            throw new NotImplementedException();
        }
    }

    [JsonConverter(typeof(MemoryStateConverter))]
    public record class MemoryState
    {
        public UInt16 Address { get; set; }

        public byte Value { get; set; }
    }

    public class CycleConverter : JsonConverter<Cycle>
    {
        public override Cycle? Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            if (reader.TokenType != JsonTokenType.StartArray)
            {
                throw new JsonException("expected start of array");
            }
            reader.Read();
            if (reader.TokenType != JsonTokenType.Number)
            {
                throw new JsonException("expected number");
            }
            var address = reader.GetUInt16();
            reader.Read();
            if (reader.TokenType != JsonTokenType.Number)
            {
                throw new JsonException("expected number");
            }
            var value = reader.GetByte();
            reader.Read();
            if (reader.TokenType != JsonTokenType.String)
            {
                throw new JsonException("expected string");
            }
            var mode = reader.GetString();
            reader.Read();
            if (reader.TokenType != JsonTokenType.EndArray)
            {
                throw new JsonException("expected end of array");
            }
            return new Cycle
            {
                Address = address,
                Value = value,
                Mode = mode switch
                {
                    "read" => CycleMode.Read,
                    "write" => CycleMode.Write,
                    _ => throw new JsonException($"expected the exact string \"read\" or \"write\", got \"{mode}\"")
                }
            };
        }

        public override void Write(Utf8JsonWriter writer, Cycle value, JsonSerializerOptions options)
        {
            throw new NotImplementedException();
        }
    }

    public enum CycleMode
    {
        Read,
        Write
    }

    [JsonConverter(typeof(CycleConverter))]
    public record class Cycle
    {
        public UInt16 Address { get; set; }

        public byte Value { get; set; }

        public CycleMode Mode { get; set; }
    }

    [Theory]
    [MemberData(nameof(TestData))]
    public void Tests(string path, TestCase testCase)
    {
        Console.WriteLine($"{path} - {testCase.Name}");

        var cpu = new CPU
        {
            PC = testCase.Initial.PC,
            SP = testCase.Initial.SP,
            A = testCase.Initial.A,
            X = testCase.Initial.X,
            Y = testCase.Initial.Y,
            Flags = testCase.Initial.Flags
        };

        var memory = new TestMemory();
        foreach (var data in testCase.Initial.RAM)
        {
            memory.Write8(data.Address, data.Value);
        }

        cpu.Step(memory);

        Assert.Equal(testCase.Final.PC, cpu.PC);
        Assert.Equal(testCase.Final.SP, cpu.SP);
        Assert.Equal(testCase.Final.A, cpu.A);
        Assert.Equal(testCase.Final.X, cpu.X);
        Assert.Equal(testCase.Final.Y, cpu.Y);
        Assert.Equal(testCase.Final.Flags, cpu.Flags);

        foreach (var data in testCase.Final.RAM)
        {
            Assert.Equal(data.Value, memory.Read8(data.Address));
        }

        Assert.Equal((UInt64)testCase.Cycles.Length, cpu.ClockCycles);
        // TODO test exact per-cycle memory access
    }

    public static IEnumerable<object[]> TestData
    {
        get
        {
            foreach (var path in Directory.GetFiles("../../../../../submodules/ProcessorTests/nes6502/v1", "*.json", SearchOption.AllDirectories))
            {
                // TODO do all tests
                var instruction = byte.Parse(Path.GetFileNameWithoutExtension(path), System.Globalization.NumberStyles.HexNumber);
                if (instruction != 0x12)
                // if (instruction > 0x12)
                {
                    continue;
                }

                using var file = File.Open(path, FileMode.Open);
                var results = JsonSerializer.Deserialize<TestCase[]>(file)
                    ?? throw new NullReferenceException($"failed to load test data for file: {path}");
                foreach (var result in results)
                {
                    yield return [path, result];
                }
            }
        }
    }
}

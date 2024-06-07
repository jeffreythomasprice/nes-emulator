using System.Reflection;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Serialization;
using NesEmulator;

namespace NesEmulatorTests;

public class InstructionSetTests
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
    public void Tests(string path, TestCase testCase)
    {
        Console.WriteLine($"{path} - {testCase.Name}");

        var cpu = new CPU();
        cpu.PC = testCase.Initial.PC;
        cpu.SP = testCase.Initial.SP;
        cpu.A = testCase.Initial.A;
        cpu.X = testCase.Initial.X;
        cpu.Y = testCase.Initial.Y;
        cpu.Flags = testCase.Initial.Flags;

        var memory = new TestMemory();
        foreach (var data in testCase.Initial.RAM)
        {
            memory.Write8(data.Address, data.Value);
        }

        // TODO do one cpu step

        // TODO check results
        Console.WriteLine(string.Join(", ", testCase.Cycles.Select(x => $"{x.Address} - {x.Value} - {x.Mode}")));
    }

    public static IEnumerable<object[]> TestData
    {
        get
        {
            foreach (var path in Directory.GetFiles("../../../../../submodules/ProcessorTests/nes6502/v1", "*.json", SearchOption.AllDirectories))
            {
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

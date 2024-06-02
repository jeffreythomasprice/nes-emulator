package nes

import (
	"encoding/json"
	"fmt"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type testMemory struct {
	t    *testing.T
	data [65536]uint8
}

func newTestMemory(t *testing.T) *testMemory {
	return &testMemory{
		t: t,
	}
}

var _ Memory = (*testMemory)(nil)

// Read8 implements Memory.
func (t *testMemory) Read8(addr uint16) uint8 {
	result := t.data[addr]
	t.t.Logf("reading memory from %04x = %02x", addr, result)
	return result
}

// Write8 implements Memory.
func (t *testMemory) Write8(addr uint16, value uint8) {
	t.t.Logf("writing memory to %04x = %02x", addr, value)
	t.data[addr] = value
}

type ramState struct {
	Address uint16
	Value   uint8
}

type cpuState struct {
	PC  uint16      `json:"pc"`
	SP  uint8       `json:"s"`
	A   uint8       `json:"a"`
	X   uint8       `json:"x"`
	Y   uint8       `json:"y"`
	P   uint8       `json:"p"`
	RAM []*ramState `json:"ram"`
}

type Mode int

const (
	ModeRead Mode = iota
	ModeWrite
)

type cycle struct {
	Address uint16
	Value   uint8
	Mode    Mode
}

type instructionTest struct {
	Name    string   `json:"name"`
	Initial cpuState `json:"initial"`
	Final   cpuState `json:"final"`
	Cycles  []*cycle `json:"cycles"`
}

func (r *ramState) UnmarshalJSON(data []byte) error {
	temp := make([]json.RawMessage, 0, 2)
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}
	if len(temp) != 2 {
		return fmt.Errorf("expected exactly 2 elements in array for ram state, got %v, %v\n", len(temp), temp)
	}
	if err := json.Unmarshal(temp[0], &r.Address); err != nil {
		return err
	}
	if err := json.Unmarshal(temp[1], &r.Value); err != nil {
		return err
	}
	return nil
}

func (c *cycle) UnmarshalJSON(data []byte) error {
	temp := make([]json.RawMessage, 0, 3)
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}
	if len(temp) != 3 {
		return fmt.Errorf("expected exactly 3 elements in array for cycle, got %v, %v\n", len(temp), temp)
	}
	if err := json.Unmarshal(temp[0], &c.Address); err != nil {
		return err
	}
	if err := json.Unmarshal(temp[1], &c.Value); err != nil {
		return err
	}
	var mode string
	if err := json.Unmarshal(temp[2], &mode); err != nil {
		return err
	}
	switch mode {
	case "read":
		c.Mode = ModeRead
	case "write":
		c.Mode = ModeWrite
	default:
		return fmt.Errorf("invalid mode: %v", mode)
	}
	return nil
}

func getInstructionTestFile(instruction uint8) ([]*instructionTest, error) {
	bytes, err := os.ReadFile(fmt.Sprintf("../submodules/ProcessorTests/6502/v1/%02x.json", instruction))
	if err != nil {
		return nil, fmt.Errorf("failed to read file for instruction %02x: %w", instruction, err)
	}
	result := make([]*instructionTest, 0)
	if err := json.Unmarshal(bytes, &result); err != nil {
		return nil, fmt.Errorf("failed to parse file for instruction %02x: %w", instruction, err)
	}
	return result, nil
}

func runTestForInstruction(t *testing.T, instruction uint8) {
	tests, err := getInstructionTestFile(instruction)
	require.NoError(t, err)
	for _, test := range tests {
		t.Run(
			fmt.Sprintf("instruction=%02x, test name=%v", instruction, test.Name),
			func(t *testing.T) {
				var cpu CPU
				cpu.PC = test.Initial.PC
				cpu.SP = test.Initial.SP
				cpu.A = test.Initial.A
				cpu.X = test.Initial.X
				cpu.Y = test.Initial.Y
				cpu.P = test.Initial.P
				memory := newTestMemory(t)
				for _, ram := range test.Initial.RAM {
					memory.Write8(ram.Address, ram.Value)
				}

				cpu.Tick(memory)

				assert.Equal(t, cpu.PC, test.Final.PC, "program counter")
				assert.Equal(t, cpu.SP, test.Final.SP, "stack pointer")
				assert.Equal(t, cpu.A, test.Final.A, "accumulator")
				assert.Equal(t, cpu.X, test.Final.X, "X index")
				assert.Equal(t, cpu.Y, test.Final.Y, "Y index")
				assert.Equal(t, cpu.P, test.Final.P, "flags register")
				for _, ram := range test.Final.RAM {
					assert.Equal(t, memory.Read8(ram.Address), ram.Value, "ram at %04x should be %02x", ram.Address, ram.Value)
				}

				// TODO validate per-cycle stuff?
			},
		)
	}
}

func TestFoo(t *testing.T) {
	// TODO should do all files in dir
	runTestForInstruction(t, 0x00)
}

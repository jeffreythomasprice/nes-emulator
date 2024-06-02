package nes

import (
	"encoding/json"
	"fmt"
	"os"
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type ramState struct {
	Address uint16
	Value   uint8
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

type systemState struct {
	PC  uint16      `json:"pc"`
	SP  uint8       `json:"s"`
	A   uint8       `json:"a"`
	X   uint8       `json:"x"`
	Y   uint8       `json:"y"`
	P   uint8       `json:"p"`
	RAM []*ramState `json:"ram"`
}

func (state systemState) String() string {
	memoryParts := make([]string, 0, len(state.RAM))
	sort.Slice(state.RAM, func(i, j int) bool {
		return state.RAM[i].Address < state.RAM[j].Address
	})
	for _, r := range state.RAM {
		memoryParts = append(memoryParts, fmt.Sprintf("[%04x]=%02x", r.Address, r.Value))
	}
	return fmt.Sprintf(
		"PC=%04x, SP=%02x, A=%02x, X=%02x, Y=%02x, P=%08b\n    %v",
		state.PC,
		state.SP,
		state.A,
		state.X,
		state.Y,
		state.P,
		strings.Join(memoryParts, ", "),
	)
}

type Mode int

const (
	ModeRead Mode = iota
	ModeWrite
)

type memoryOperation struct {
	Address uint16
	Value   uint8
	Mode    Mode
}

func (op *memoryOperation) UnmarshalJSON(data []byte) error {
	temp := make([]json.RawMessage, 0, 3)
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}
	if len(temp) != 3 {
		return fmt.Errorf("expected exactly 3 elements in array for cycle, got %v, %v\n", len(temp), temp)
	}
	if err := json.Unmarshal(temp[0], &op.Address); err != nil {
		return err
	}
	if err := json.Unmarshal(temp[1], &op.Value); err != nil {
		return err
	}
	var mode string
	if err := json.Unmarshal(temp[2], &mode); err != nil {
		return err
	}
	switch mode {
	case "read":
		op.Mode = ModeRead
	case "write":
		op.Mode = ModeWrite
	default:
		return fmt.Errorf("invalid mode: %v", mode)
	}
	return nil
}

type instructionTest struct {
	Name    string             `json:"name"`
	Initial systemState        `json:"initial"`
	Final   systemState        `json:"final"`
	Cycles  []*memoryOperation `json:"cycles"`
}

type testMemory struct {
	t      *testing.T
	data   [65536]uint8
	cycles []*memoryOperation
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
	t.cycles = append(t.cycles, &memoryOperation{
		Address: addr,
		Value:   result,
		Mode:    ModeRead,
	})
	return result
}

// Write8 implements Memory.
func (t *testMemory) Write8(addr uint16, value uint8) {
	t.t.Logf("writing memory to %04x = %02x", addr, value)
	t.cycles = append(t.cycles, &memoryOperation{
		Address: addr,
		Value:   value,
		Mode:    ModeWrite,
	})
	t.data[addr] = value
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
				t.Logf("initial state  = %v\n", test.Initial)
				t.Logf("expected state = %v\n", test.Final)

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

				assert.Equal(t, test.Final.PC, cpu.PC, "program counter")
				assert.Equal(t, test.Final.SP, cpu.SP, "stack pointer")
				assert.Equal(t, test.Final.A, cpu.A, "accumulator")
				assert.Equal(t, test.Final.X, cpu.X, "X index")
				assert.Equal(t, test.Final.Y, cpu.Y, "Y index")
				assert.Equal(t, test.Final.P, cpu.P, "flags register")
				for _, ram := range test.Final.RAM {
					assert.Equal(t, ram.Value, memory.Read8(ram.Address), "ram at %04x should be %02x", ram.Address, ram.Value)
				}
				assert.Equal(t, uint64(len(test.Cycles)), cpu.ClockCycles, "clock")
				// TODO test exact per-cycle memory access
				// assert.Equal(t, test.Cycles, memory.cycles, "memory access operations")
			},
		)
	}
}

func TestFoo(t *testing.T) {
	// TODO should do all files in dir
	runTestForInstruction(t, 0x00)
	runTestForInstruction(t, 0x01)
}

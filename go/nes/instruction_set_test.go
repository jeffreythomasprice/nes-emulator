package nes

import (
	"encoding/json"
	"fmt"
	"os"
	"path"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
)

type ramState struct {
	Address uint16
	Value   uint8
}

func (r *ramState) UnmarshalJSON(data []byte) error {
	var v []interface{}
	if err := json.Unmarshal(data, &v); err != nil {
		return err
	}
	if len(v) != 2 {
		return fmt.Errorf("wrong number of arguments, expected 2 got %v", len(v))
	}
	address, ok := v[0].(float64)
	if !ok {
		return fmt.Errorf("failed to get expected type for address, got %v", v[0])
	}
	r.Address = uint16(address)
	value, ok := v[1].(float64)
	if !ok {
		return fmt.Errorf("failed to get expected type for value, got %v", v[1])
	}
	r.Value = uint8(value)
	return nil
}

type state struct {
	PC    uint16     `json:"pc"`
	SP    uint8      `json:"s"`
	A     uint8      `json:"a"`
	X     uint8      `json:"x"`
	Y     uint8      `json:"y"`
	Flags uint8      `json:"p"`
	RAM   []ramState `json:"ram"`
}

type cycleMode uint8

const (
	cycleModeRead cycleMode = iota
	cycleModeWrite
)

type cycle struct {
	Address uint16
	Value   uint8
	Mode    cycleMode
}

func (s *cycle) UnmarshalJSON(data []byte) error {
	var v []interface{}
	if err := json.Unmarshal(data, &v); err != nil {
		return err
	}
	if len(v) != 3 {
		return fmt.Errorf("wrong number of arguments, expected 3 got %v", len(v))
	}
	address, ok := v[0].(float64)
	if !ok {
		return fmt.Errorf("failed to get expected type for address, got %v", v[0])
	}
	s.Address = uint16(address)
	value, ok := v[1].(float64)
	if !ok {
		return fmt.Errorf("failed to get expected type for value, got %v", v[1])
	}
	s.Value = uint8(value)
	mode, ok := v[2].(string)
	if !ok {
		return fmt.Errorf("failed to get expected type for mode, got %v", v[2])
	}
	switch mode {
	case "read":
		s.Mode = cycleModeRead
	case "write":
		s.Mode = cycleModeWrite
	default:
		return fmt.Errorf("invalid read/write mode %v", mode)
	}
	return nil
}

type testCase struct {
	Name    string  `json:"name"`
	Initial state   `json:"initial"`
	Final   state   `json:"final"`
	Cycles  []cycle `json:"cycles"`
}

func TestInstructionSet(t *testing.T) {
	paths, err := filepath.Glob("../../submodules/ProcessorTests/nes6502/v1/*.json")
	assert.NoError(t, err)

	t.Logf("num test files = %v", len(paths))
	for _, p := range paths {
		// TODO do all the tests
		name := path.Base(p)
		if name[0:len(name)-len(path.Ext(p))] != "05" {
			continue
		}

		bytes, err := os.ReadFile(p)
		assert.NoError(t, err)

		var testCases []*testCase
		err = json.Unmarshal(bytes, &testCases)
		assert.NoError(t, err)

		t.Logf("test file = %v, num test cases in file = %v", path.Base(p), len(testCases))
		for i, testCase := range testCases {
			t.Run(fmt.Sprintf("path = %v, test case # = %v, name = %v", p, i, testCase.Name), func(t *testing.T) {
				c := CPU{
					PC:    testCase.Initial.PC,
					SP:    testCase.Initial.SP,
					A:     testCase.Initial.A,
					X:     testCase.Initial.X,
					Y:     testCase.Initial.Y,
					Flags: testCase.Initial.Flags,
				}

				m := &testMemory{}
				for _, x := range testCase.Initial.RAM {
					m.Write(x.Address, x.Value)
				}

				c.Step(m)

				assert.Equal(t, testCase.Final.PC, c.PC, "PC")
				assert.Equal(t, testCase.Final.SP, c.SP, "SP")
				assert.Equal(t, testCase.Final.A, c.A, "A")
				assert.Equal(t, testCase.Final.X, c.X, "X")
				assert.Equal(t, testCase.Final.Y, c.Y, "Y")
				assert.Equal(t, testCase.Final.Flags, c.Flags, "flags")
				for _, x := range testCase.Final.RAM {
					assert.Equal(t, x.Value, m.Read(x.Address), "address at %v", x.Address)
				}
				assert.Equal(t, uint64(len(testCase.Cycles)), c.ClockTime, "clock time")
				// TODO assert actual cycle contents
			})
		}
	}
}

package nes

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// TODO write tests for read and write 16 bits at a time

func TestSplit16(t *testing.T) {
	for _, testCase := range []struct {
		input                     uint16
		expectedLow, expectedHigh uint8
	}{
		{0x1234, 0x34, 0x12},
		{0x1122, 0x22, 0x11},
	} {
		low, high := Split16(testCase.input)
		assert.Equal(t, testCase.expectedLow, low)
		assert.Equal(t, testCase.expectedHigh, high)
	}
}

func TestCombine16(t *testing.T) {
	for _, testCase := range []struct {
		inputLow, inputHigh uint8
		expected            uint16
	}{
		{0x11, 0x22, 0x2211},
		{0x34, 0x12, 0x1234},
	} {
		result := Combine16(testCase.inputLow, testCase.inputHigh)
		assert.Equal(t, testCase.expected, result)
	}
}

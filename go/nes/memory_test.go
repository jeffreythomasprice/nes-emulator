package nes

type testMemory struct {
	data [0x10000]uint8
}

var _ Memory = (*testMemory)(nil)

// Read implements Memory.
func (t *testMemory) Read(address Word) uint8 {
	return t.data[address]
}

// Write implements Memory.
func (t *testMemory) Write(address Word, value uint8) {
	t.data[address] = value
}

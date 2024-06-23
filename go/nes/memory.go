package nes

type Memory interface {
	Read(address uint16) uint8
	Write(address uint16, value uint8)
}

func Read16(m Memory, address uint16) uint16 {
	low := m.Read(address)
	high := m.Read(address + 1)
	return uint16(low) | (uint16(high) << 8)
}

func Write16(m Memory, address uint16, value uint16) {
	low := uint8(value)
	high := uint8(value >> 8)
	m.Write(address, low)
	m.Write(address+1, high)
}

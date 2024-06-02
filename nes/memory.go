package nes

type Memory interface {
	Read8(addr uint16) uint8
	Write8(addr uint16, value uint8)
}

func Read16(memory Memory, addr uint16) uint16 {
	return Combine16(memory.Read8(addr), memory.Read8(addr+1))
}

func Write16(memory Memory, addr uint16, value uint16) {
	low, high := Split16(value)
	memory.Write8(addr, low)
	memory.Write8(addr+1, high)
}

func Split16(value uint16) (low uint8, high uint8) {
	low = uint8(value & 0xff)
	high = uint8((value & 0xff00) >> 8)
	return
}

func Combine16(low uint8, high uint8) uint16 {
	return uint16(low) | (uint16(high) << 8)
}

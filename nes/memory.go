package nes

type Memory interface {
	Read8(addr uint16) uint8
	Write8(addr uint16, value uint8)
}

func Read16(memory Memory, addr uint16) uint16 {
	low := uint16(memory.Read8(addr))
	high := uint16(memory.Read8(addr + 1))
	return (high << 8) | low
}

func Write16(memory Memory, addr uint16, value uint16) {
	memory.Write8(addr, uint8(value&0xff))
	memory.Write8(addr+1, uint8((value&0xff00)>>8))
}

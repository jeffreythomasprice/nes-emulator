package nes

type Word uint16

func NewWord(low, high uint8) Word {
	return Word(uint16(low) | (uint16(high) << 8))
}

func (w Word) Low() uint8 {
	return uint8(w & 0xff)
}

func (w Word) High() uint8 {
	return uint8(w >> 8)
}

type Memory interface {
	Read(address Word) uint8
	Write(address Word, value uint8)
}

func Read16(m Memory, address Word) Word {
	low := m.Read(address)
	high := m.Read(address + 1)
	return NewWord(low, high)
}

func Write16(m Memory, address Word, value Word) {
	m.Write(address, value.Low())
	m.Write(address+1, value.High())
}

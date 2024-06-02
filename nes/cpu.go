package nes

const (
	carryFlagMask               uint8 = 0b0000_0001
	carryFlagMaskInverse        uint8 = 0b1111_1110
	zeroFlagMask                uint8 = 0b0000_0010
	zeroFlagMaskInverse         uint8 = 0b1111_1101
	interruptDisableMask        uint8 = 0b0000_0100
	interruptDisableMaskInverse uint8 = 0b1111_1011
	decimalModeMask             uint8 = 0b0000_1000
	decimalModeMaskInverse      uint8 = 0b1111_0111
	breakCommandMask            uint8 = 0b0001_0000
	breakCommandMaskInverse     uint8 = 0b1110_1111
	overflowFlagMask            uint8 = 0b0100_0000
	overflowFlagMaskInverse     uint8 = 0b1011_1111
	negativeFlagMask            uint8 = 0b1000_0000
	negativeFlagMaskInverse     uint8 = 0b0111_1111
)

const (
	stackAddress uint16 = 0x0100

	nonMaskableInterruptAddress      uint16 = 0xfffa
	resetInterruptAddress            uint16 = 0xfffc
	interruptRequestInterruptAddress uint16 = 0xfffe
)

type CPU struct {
	// program counter
	PC uint16
	// stack pointer
	SP uint8
	// accumulator
	A uint8
	// index register
	X uint8
	// index register
	Y uint8
	// processor status
	P uint8
	// total clock cycles
	ClockCycles uint64
}

func (cpu *CPU) Tick(memory Memory) {
	/*
		TODO interrupt handlers

		if an interrupt has occurred {
			push PC
			set interrupt disable flag
			load next addr from interrupt table, priority: reset, NMI, IRQ
		}
	*/
	cpu.executeInstruction(memory, memory.Read8(cpu.PC))
}

func (cpu *CPU) executeInstruction(memory Memory, instruction uint8) {
	switch instruction {
	// BRK impl
	case 0b000_000_00:
		cpu._break(memory)
	// ORA X, ind
	case 0b000_000_01:
		cpu.oraXIndirect(memory)
	// illegal
	case 0b000_000_10:
		// TODO
	// illegal
	case 0b000_000_11:
		// TODO

	// illegal
	case 0b000_001_00:
		// TODO
	// ORA zpg
	case 0b000_001_01:
		// TODO
	// ASL zpg
	case 0b000_001_10:
		// TODO
	// illegal
	case 0b000_001_11:
		// TODO

	// PHP impl
	case 0b000_010_00:
		// TODO
	// ORA #
	case 0b000_010_01:
		// TODO
	// ASL A
	case 0b000_010_10:
		// TODO
	// illegal
	case 0b000_010_11:
		// TODO

	// illegal
	case 0b000_011_00:
		// TODO
	// ORA abs
	case 0b000_011_01:
		// TODO
	// ASL abs
	case 0b000_011_10:
		// TODO
	// illegal
	case 0b000_011_11:
		// TODO

	// BPL rel
	case 0b000_100_00:
		// TODO
	// ORA ind,Y
	case 0b000_100_01:
		// TODO
	// illegal
	case 0b000_100_10:
		// TODO
	// illegal
	case 0b000_100_11:
		// TODO

	// illegal
	case 0b000_101_00:
		// TODO
	// ORA zpg,X
	case 0b000_101_01:
		// TODO
	// ASL zpg,X
	case 0b000_101_10:
		// TODO
	// illegal
	case 0b000_101_11:
		// TODO

	// CLC impl
	case 0b000_110_00:
		// TODO
	// ORA abs,Y
	case 0b000_110_01:
		// TODO
	// illegal
	case 0b000_110_10:
		// TODO
	// illegal
	case 0b000_110_11:
		// TODO

	// illegal
	case 0b000_111_00:
		// TODO
	// ORA abs,X
	case 0b000_111_01:
		// TODO
	// ASL abs,X
	case 0b000_111_10:
		// TODO
	// illegal
	case 0b000_111_11:
		// TODO

	// JSR abs
	case 0b001_000_00:
		// TODO
	// AND X,ind
	case 0b001_000_01:
		// TODO
	// illegal
	case 0b001_000_10:
		// TODO
	// illegal
	case 0b001_000_11:
		// TODO

	// BIT zpg
	case 0b001_001_00:
		// TODO
	// AND zpg
	case 0b001_001_01:
		// TODO
	// ROL zpg
	case 0b001_001_10:
		// TODO
	// illegal
	case 0b001_001_11:
		// TODO

	// PLP impl
	case 0b001_010_00:
		// TODO
	// AND #
	case 0b001_010_01:
		// TODO
	// ROL A
	case 0b001_010_10:
		// TODO
	// illegal
	case 0b001_010_11:
		// TODO

	// BIT abs
	case 0b001_011_00:
		// TODO
	// AND abs
	case 0b001_011_01:
		// TODO
	// ROL abs
	case 0b001_011_10:
		// TODO
	// illegal
	case 0b001_011_11:
		// TODO

	// BMI rel
	case 0b001_100_00:
		// TODO
	// AND ind,Y
	case 0b001_100_01:
		// TODO
	// illegal
	case 0b001_100_10:
		// TODO
	// illegal
	case 0b001_100_11:
		// TODO

	// illegal
	case 0b001_101_00:
		// TODO
	// AND zpg,X
	case 0b001_101_01:
		// TODO
	// ROL zpg,X
	case 0b001_101_10:
		// TODO
	// illegal
	case 0b001_101_11:
		// TODO

	// SEC impl
	case 0b001_110_00:
		// TODO
	// AND abs,Y
	case 0b001_110_01:
		// TODO
	// illegal
	case 0b001_110_10:
		// TODO
	// illegal
	case 0b001_110_11:
		// TODO

	// illegal
	case 0b001_111_00:
		// TODO
	// AND abs,X
	case 0b001_111_01:
		// TODO
	// ROL abs,X
	case 0b001_111_10:
		// TODO
	// illegal
	case 0b001_111_11:
		// TODO

	// RTI impl
	case 0b010_000_00:
		// TODO
	// EOR X,ind
	case 0b010_000_01:
		// TODO
	// illegal
	case 0b010_000_10:
		// TODO
	// illegal
	case 0b010_000_11:
		// TODO

	// illegal
	case 0b010_001_00:
		// TODO
	// EOR zpg
	case 0b010_001_01:
		// TODO
	// LSR zpg
	case 0b010_001_10:
		// TODO
	// illegal
	case 0b010_001_11:
		// TODO

	// PHA impl
	case 0b010_010_00:
		// TODO
	// EOR #
	case 0b010_010_01:
		// TODO
	// LSR A
	case 0b010_010_10:
		// TODO
	// illegal
	case 0b010_010_11:
		// TODO

	// JMP abs
	case 0b010_011_00:
		// TODO
	// EOR abs
	case 0b010_011_01:
		// TODO
	// LSR abs
	case 0b010_011_10:
		// TODO
	// illegal
	case 0b010_011_11:
		// TODO

	// BVC rel
	case 0b010_100_00:
		// TODO
	// EOR ind,Y
	case 0b010_100_01:
		// TODO
	// illegal
	case 0b010_100_10:
		// TODO
	// illegal
	case 0b010_100_11:
		// TODO

	// illegal
	case 0b010_101_00:
		// TODO
	// EOR zpg,X
	case 0b010_101_01:
		// TODO
	// LSR zpg,X
	case 0b010_101_10:
		// TODO
	// illegal
	case 0b010_101_11:
		// TODO

	// CLI impl
	case 0b010_110_00:
		// TODO
	// EOR abs,Y
	case 0b010_110_01:
		// TODO
	// illegal
	case 0b010_110_10:
		// TODO
	// illegal
	case 0b010_110_11:
		// TODO

	// illegal
	case 0b010_111_00:
		// TODO
	// EOR abs,X
	case 0b010_111_01:
		// TODO
	// LSR abs,X
	case 0b010_111_10:
		// TODO
	// illegal
	case 0b010_111_11:
		// TODO

	// RTS impl
	case 0b011_000_00:
		// TODO
	// ADC X,ind
	case 0b011_000_01:
		// TODO
	// illegal
	case 0b011_000_10:
		// TODO
	// illegal
	case 0b011_000_11:
		// TODO

	// illegal
	case 0b011_001_00:
		// TODO
	// ADC zpg
	case 0b011_001_01:
		// TODO
	// ROR zpg
	case 0b011_001_10:
		// TODO
	// illegal
	case 0b011_001_11:
		// TODO

	// PLA impl
	case 0b011_010_00:
		// TODO
	// ADC #
	case 0b011_010_01:
		// TODO
	// ROR A
	case 0b011_010_10:
		// TODO
	// illegal
	case 0b011_010_11:
		// TODO

	// JMP ind
	case 0b011_011_00:
		// TODO
	// ADC abs
	case 0b011_011_01:
		// TODO
	// ROR abs
	case 0b011_011_10:
		// TODO
	// illegal
	case 0b011_011_11:
		// TODO

	// BVS rel
	case 0b011_100_00:
		// TODO
	// ADC ind,Y
	case 0b011_100_01:
		// TODO
	// illegal
	case 0b011_100_10:
		// TODO
	// illegal
	case 0b011_100_11:
		// TODO

	// illegal
	case 0b011_101_00:
		// TODO
	// ADC zpg,X
	case 0b011_101_01:
		// TODO
	// ROR zpg,X
	case 0b011_101_10:
		// TODO
	// illegal
	case 0b011_101_11:
		// TODO

	// SEI impl
	case 0b011_110_00:
		// TODO
	// ADC abs,Y
	case 0b011_110_01:
		// TODO
	// illegal
	case 0b011_110_10:
		// TODO
	// illegal
	case 0b011_110_11:
		// TODO

	// illegal
	case 0b011_111_00:
		// TODO
	// ADC abs,X
	case 0b011_111_01:
		// TODO
	// ROR abs,X
	case 0b011_111_10:
		// TODO
	// illegal
	case 0b011_111_11:
		// TODO

	// illegal
	case 0b100_000_00:
		// TODO
	// STA X,ind
	case 0b100_000_01:
		// TODO
	// illegal
	case 0b100_000_10:
		// TODO
	// illegal
	case 0b100_000_11:
		// TODO

	// STY zpg
	case 0b100_001_00:
		// TODO
	// STA zpg
	case 0b100_001_01:
		// TODO
	// STX zpg
	case 0b100_001_10:
		// TODO
	// illegal
	case 0b100_001_11:
		// TODO

	// DEY impl
	case 0b100_010_00:
		// TODO
	// illegal
	case 0b100_010_01:
		// TODO
	// TXA impl
	case 0b100_010_10:
		// TODO
	// illegal
	case 0b100_010_11:
		// TODO

	// STY abs
	case 0b100_011_00:
		// TODO
	// STA abs
	case 0b100_011_01:
		// TODO
	// STX abs
	case 0b100_011_10:
		// TODO
	// illegal
	case 0b100_011_11:
		// TODO

	// BCC rel
	case 0b100_100_00:
		// TODO
	// STA ind,Y
	case 0b100_100_01:
		// TODO
	// illegal
	case 0b100_100_10:
		// TODO
	// illegal
	case 0b100_100_11:
		// TODO

	// STY zpg,X
	case 0b100_101_00:
		// TODO
	// STA zpg,X
	case 0b100_101_01:
		// TODO
	// STX zpg,Y
	case 0b100_101_10:
		// TODO
	// illegal
	case 0b100_101_11:
		// TODO

	// TYA impl
	case 0b100_110_00:
		// TODO
	// STA abs,Y
	case 0b100_110_01:
		// TODO
	// TXS impl
	case 0b100_110_10:
		// TODO
	// illegal
	case 0b100_110_11:
		// TODO

	// illegal
	case 0b100_111_00:
		// TODO
	// STA abs,X
	case 0b100_111_01:
		// TODO
	// illegal
	case 0b100_111_10:
		// TODO
	// illegal
	case 0b100_111_11:
		// TODO

	// LDY #
	case 0b101_000_00:
		// TODO
	// LDA X,ind
	case 0b101_000_01:
		// TODO
	// LDX #
	case 0b101_000_10:
		// TODO
	// illegal
	case 0b101_000_11:
		// TODO

	// LDY zpg
	case 0b101_001_00:
		// TODO
	// LDA zpg
	case 0b101_001_01:
		// TODO
	// LDX zpg
	case 0b101_001_10:
		// TODO
	// illegal
	case 0b101_001_11:
		// TODO

	// TAY impl
	case 0b101_010_00:
		// TODO
	// LDA #
	case 0b101_010_01:
		// TODO
	// TAX impl
	case 0b101_010_10:
		// TODO
	// illegal
	case 0b101_010_11:
		// TODO

	// LDY abs
	case 0b101_011_00:
		// TODO
	// LDA abs
	case 0b101_011_01:
		// TODO
	// LDX abs
	case 0b101_011_10:
		// TODO
	// illegal
	case 0b101_011_11:
		// TODO

	// BCS rel
	case 0b101_100_00:
		// TODO
	// LDA ind,Y
	case 0b101_100_01:
		// TODO
	// illegal
	case 0b101_100_10:
		// TODO
	// illegal
	case 0b101_100_11:
		// TODO

	// LDY zpg,X
	case 0b101_101_00:
		// TODO
	// LDA zpg,X
	case 0b101_101_01:
		// TODO
	// LDX zpg,Y
	case 0b101_101_10:
		// TODO
	// illegal
	case 0b101_101_11:
		// TODO

	// CLV impl
	case 0b101_110_00:
		// TODO
	// LDA abs,Y
	case 0b101_110_01:
		// TODO
	// TSX impl
	case 0b101_110_10:
		// TODO
	// illegal
	case 0b101_110_11:
		// TODO

	// LDY abs,X
	case 0b101_111_00:
		// TODO
	// LDA abs,X
	case 0b101_111_01:
		// TODO
	// LDX abs,Y
	case 0b101_111_10:
		// TODO
	// illegal
	case 0b101_111_11:
		// TODO

	// CPY #
	case 0b110_000_00:
		// TODO
	// CMP X,ind
	case 0b110_000_01:
		// TODO
	// illegal
	case 0b110_000_10:
		// TODO
	// illegal
	case 0b110_000_11:
		// TODO

	// CPY zpg
	case 0b110_001_00:
		// TODO
	// CMP zpg
	case 0b110_001_01:
		// TODO
	// DEC zpg
	case 0b110_001_10:
		// TODO
	// illegal
	case 0b110_001_11:
		// TODO

	// INY impl
	case 0b110_010_00:
		// TODO
	// CMP #
	case 0b110_010_01:
		// TODO
	// DEX impl
	case 0b110_010_10:
		// TODO
	// illegal
	case 0b110_010_11:
		// TODO

	// CPY abs
	case 0b110_011_00:
		// TODO
	// CMP abs
	case 0b110_011_01:
		// TODO
	// DEC abs
	case 0b110_011_10:
		// TODO
	// illegal
	case 0b110_011_11:
		// TODO

	// BNE rel
	case 0b110_100_00:
		// TODO
	// CMP ind,Y
	case 0b110_100_01:
		// TODO
	// illegal
	case 0b110_100_10:
		// TODO
	// illegal
	case 0b110_100_11:
		// TODO

	// illegal
	case 0b110_101_00:
		// TODO
	// CMP zpg,X
	case 0b110_101_01:
		// TODO
	// DEC zpg,X
	case 0b110_101_10:
		// TODO
	// illegal
	case 0b110_101_11:
		// TODO

	// CLD impl
	case 0b110_110_00:
		// TODO
	// CMP abs,Y
	case 0b110_110_01:
		// TODO
	// illegal
	case 0b110_110_10:
		// TODO
	// illegal
	case 0b110_110_11:
		// TODO

	// illegal
	case 0b110_111_00:
		// TODO
	// CMP abs,X
	case 0b110_111_01:
		// TODO
	// DEC abs,X
	case 0b110_111_10:
		// TODO
	// illegal
	case 0b110_111_11:
		// TODO

	// CPX #
	case 0b111_000_00:
		// TODO
	// SBC X,ind
	case 0b111_000_01:
		// TODO
	// illegal
	case 0b111_000_10:
		// TODO
	// illegal
	case 0b111_000_11:
		// TODO

	// CPX zpg
	case 0b111_001_00:
		// TODO
	// SBC zpg
	case 0b111_001_01:
		// TODO
	// INC zpg
	case 0b111_001_10:
		// TODO
	// illegal
	case 0b111_001_11:
		// TODO

	// INX impl
	case 0b111_010_00:
		// TODO
	// SBC #
	case 0b111_010_01:
		// TODO
	// NOP impl
	case 0b111_010_10:
		// TODO
	// illegal
	case 0b111_010_11:
		// TODO

	// CPX abs
	case 0b111_011_00:
		// TODO
	// SBC abs
	case 0b111_011_01:
		// TODO
	// INC abs
	case 0b111_011_10:
		// TODO
	// illegal
	case 0b111_011_11:
		// TODO

	// BEQ rel
	case 0b111_100_00:
		// TODO
	// SBC ind,Y
	case 0b111_100_01:
		// TODO
	// illegal
	case 0b111_100_10:
		// TODO
	// illegal
	case 0b111_100_11:
		// TODO

	// illegal
	case 0b111_101_00:
		// TODO
	// SBC zpg,X
	case 0b111_101_01:
		// TODO
	// INC zpg,X
	case 0b111_101_10:
		// TODO
	// illegal
	case 0b111_101_11:
		// TODO

	// SED impl
	case 0b111_110_00:
		// TODO
	// SBC abs,Y
	case 0b111_110_01:
		// TODO
	// illegal
	case 0b111_110_10:
		// TODO
	// illegal
	case 0b111_110_11:
		// TODO

	// illegal
	case 0b111_111_00:
		// TODO
	// SBC abs,X
	case 0b111_111_01:
		// TODO
	// INC abs,X
	case 0b111_111_10:
		// TODO
	// illegal
	case 0b111_111_11:
		// TODO
	}
}

func (cpu *CPU) _break(memory Memory) {
	cpu.push16(memory, cpu.PC+2)
	cpu.push8(memory, cpu.P|breakCommandMask)
	cpu.P |= interruptDisableMask
	cpu.PC = Read16(memory, interruptRequestInterruptAddress)
	cpu.ClockCycles += 7
}

func (cpu *CPU) oraXIndirect(memory Memory) {
	addr := cpu.zeroPageIndirectXAddress(memory, memory.Read8(cpu.PC+1))
	cpu.A |= memory.Read8(addr)
	cpu.PC += 2
	cpu.updateNegativeFlag(cpu.A)
	cpu.updateZeroFlag(cpu.A)
	cpu.ClockCycles += 6
}

func (cpu *CPU) push8(memory Memory, value uint8) {
	memory.Write8(stackAddress+uint16(cpu.SP), value)
	cpu.SP--
}

func (cpu *CPU) push16(memory Memory, value uint16) {
	cpu.push8(memory, uint8((value&0xff00)>>8))
	cpu.push8(memory, uint8(value&0xff))
}

func (cpu *CPU) pop8(memory Memory) uint8 {
	cpu.SP++
	return memory.Read8(stackAddress + uint16(cpu.SP))
}

func (cpu *CPU) pop16(memory Memory) uint16 {
	high := cpu.pop8(memory)
	low := cpu.pop8(memory)
	return (uint16(high) << 8) | uint16(low)
}

func (cpu *CPU) zeroPageIndirectXAddress(memory Memory, offset uint8) uint16 {
	// using 8-bit math to keep overflows in the 0 page
	newOffset := uint16(offset) + uint16(cpu.X)
	newOffsetLow, _ := Split16(newOffset)
	low := memory.Read8(uint16(newOffsetLow))
	high := memory.Read8(uint16(newOffsetLow + 1))
	return Combine16(low, high)
}

func (cpu *CPU) updateNegativeFlag(value uint8) {
	if (value & 0b1000_0000) != 0 {
		cpu.P |= negativeFlagMask
	} else {
		cpu.P &= negativeFlagMaskInverse
	}
}

func (cpu *CPU) updateZeroFlag(value uint8) {
	if value == 0 {
		cpu.P |= zeroFlagMask
	} else {
		cpu.P &= zeroFlagMaskInverse
	}
}

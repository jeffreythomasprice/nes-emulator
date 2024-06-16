export enum MemoryModel {
	CPU_REGISTERS = 0x0000,
	CPU_REGISTER_PC = CPU_REGISTERS + 0,
	CPU_REGISTER_SP = CPU_REGISTERS + 2,
	CPU_REGISTER_A = CPU_REGISTERS + 3,
	CPU_REGISTER_X = CPU_REGISTERS + 4,
	CPU_REGISTER_Y = CPU_REGISTERS + 5,
	CPU_REGISTER_FLAGS = CPU_REGISTERS + 6,
	CPU_REGISTER_LAST = CPU_REGISTER_FLAGS,
	MEMORY_BANK_START = CPU_REGISTER_LAST + 1,
}

export enum CPUFlagsRegister {
	CARRY = 0b0000_0001,
	ZERO = 0b0000_0010,
	INTERRUPT_DISABLE = 0b0000_0100,
	DECIMAL_MODE = 0b0000_1000,
	BREAK_COMMAND = 0b0001_0000,
	// UNUSED = 0b0010_0000,
	OVERFLOW = 0b0100_0000,
	NEGATIVE = 0b1000_0000,
}
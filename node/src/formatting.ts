export function toHexStringU8(value: number): string {
	if (value !== Math.floor(value)) {
		throw new Error(`not an integer: ${value}`);
	}
	if (value < 0 || value > 0xff) {
		throw new Error(`out of bounds, expected u8: ${value}`);
	}
	return "0x" + value.toString(16).padStart(2, '0');
}

export function toHexStringU16(value: number): string {
	if (value !== Math.floor(value)) {
		throw new Error(`not an integer: ${value}`);
	}
	if (value < 0 || value > 0xffff) {
		throw new Error(`out of bounds, expected u8: ${value}`);
	}
	return "0x" + value.toString(16).padStart(4, '0');
}
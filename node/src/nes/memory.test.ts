import { Memory, NES_MAX_ADDRESS, WEBASSEMBLY_PAGE_SIZE } from "./memory";
import { Node } from "./node";

class TestMemory extends Memory {
	readonly memory: WebAssembly.Memory;
	private readonly u8: Uint8Array;

	constructor(initialContents?: Buffer) {
		super();

		const size = NES_MAX_ADDRESS + 1;
		this.memory = new WebAssembly.Memory({
			initial: Math.ceil(size / WEBASSEMBLY_PAGE_SIZE),
		});
		this.u8 = new Uint8Array(this.memory.buffer);
		if (initialContents) {
			if (initialContents.byteLength > size) {
				throw new Error(`can't initialze flat memory object with more than ${size} bytes, ${initialContents.byteLength} provided`);
			}
			initialContents.copy(this.u8);
		}
	}

	readU8(address: number): number {
		return this.u8[fixAddress(address)];
	}

	readU16(address: number): number {
		return this.readU8(address) | (this.readU8(address + 1) << 8);
	}

	writeU8(address: number, value: number) {
		this.u8[fixAddress(address)] = value;
	}

	writeU16(address: number, value: number) {
		this.writeU8(fixAddress(address), value & 0xff);
		this.writeU8(fixAddress(address + 1), (value & 0xff00) >> 8);
	}

	createReadU8Node(address: Node): Node {
		throw new Error("Method not implemented.");
	}

	createReadU16Node(address: Node): Node {
		throw new Error("Method not implemented.");
	}

	createWriteU8Node(address: Node, value: Node): Node {
		throw new Error("Method not implemented.");
	}

	createWriteU16Node(address: Node, value: Node): Node {
		throw new Error("Method not implemented.");
	}
}

function fixAddress(address: number): number {
	return (address | 0) % (NES_MAX_ADDRESS + 1);
}
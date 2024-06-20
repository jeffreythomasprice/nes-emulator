import { Function, I32LoadU8, LocalGet, Module, Statement, Node as WatNode } from "../wat";
import { Memory, NES_MAX_ADDRESS, WEBASSEMBLY_PAGE_SIZE } from "./memory";

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

	createReadU8Node(address: WatNode): Statement {
		return new I32LoadU8(fixAddressNode(address));
	}

	createReadU16Node(address: WatNode): Statement {
		// TODO implement me
	}

	createWriteU8Node(address: WatNode, value: WatNode): Statement {
		// TODO implement me
	}

	createWriteU16Node(address: WatNode, value: WatNode): Statement {
		// TODO implement me
	}
}

function fixAddress(address: number): number {
	return (address | 0) % (NES_MAX_ADDRESS + 1);
}

function fixAddressNode(address: WatNode): WatNode {
	// TODO wrap to max address
	return address;
}

describe(__filename, () => {
	it("TODO rename me", () => {
		const memory = new TestMemory();
		const module = new Module(
			new Function({
				name: "readU8",
				export: "readU8",
				params: [
					{
						name: "address",
						type: "i32",
					}
				],
				returnType: "i32",
				statements: [
					memory.createReadU8Node(new LocalGet("address")),
				],
			}),
			new Function({
				name: "readU16",
				export: "readU16",
				params: [
					{
						name: "address",
						type: "i32",
					}
				],
				returnType: "i32",
				statements: [
					memory.createReadU16Node(new LocalGet("address")),
				],
			}),
			new Function({
				name: "writeU8",
				export: "writeU8",
				params: [
					{
						name: "address",
						type: "i32",
					}, {
						name: "value",
						type: "i32",
					},
				],
				statements: [
					memory.createWriteU8Node(new LocalGet("address"), new LocalGet("value")),
				],
			}),
			new Function({
				name: "writeU16",
				export: "writeU16",
				params: [
					{
						name: "address",
						type: "i32",
					}, {
						name: "value",
						type: "i32",
					},
				],
				statements: [
					memory.createWriteU16Node(new LocalGet("address"), new LocalGet("value")),
				],
			}),
		);

		// TODO write some tests, proving that bytes copy back and forth both using memory directly and calling the wasm functions
	});
});
import { Const, Function, I32Add, I32LoadU8, I32Store8, LeftShift, LocalGet, Module, Or, Remainder, RightShift, Statement, Node as WatNode } from "../wat";
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

	createReadU8Nodes(address: WatNode): Statement[] {
		return [
			new I32LoadU8(fixAddressNode(address)),
		];
	}

	createReadU16Nodes(address: WatNode): Statement[] {
		return [
			new Or(
				this.createReadU8Nodes(address)[0],
				new LeftShift(
					this.createReadU8Nodes(new I32Add(address, new Const(1)))[0],
					new Const(8),
				),
			),
		];
	}

	createWriteU8Nodes(address: WatNode, value: WatNode): Statement[] {
		return [
			new I32Store8(fixAddressNode(address), value),
		];
	}

	createWriteU16Nodes(address: WatNode, value: WatNode): Statement[] {
		return [
			this.createWriteU8Nodes(address, value)[0],
			this.createWriteU8Nodes(
				new I32Add(address, new Const(1)),
				new RightShift(value, new Const(8)),
			)[0],
		];
	}
}

function fixAddress(address: number): number {
	return (address | 0) % (NES_MAX_ADDRESS + 1);
}

function fixAddressNode(address: WatNode): WatNode {
	return new Remainder(address, new Const(NES_MAX_ADDRESS + 1));
}

describe(__filename, () => {
	it("TODO rename me", () => {
		const memory = new TestMemory();
		const module = new Module(
			// TODO import memory
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
					...memory.createReadU8Nodes(new LocalGet("address")),
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
					...memory.createReadU16Nodes(new LocalGet("address")),
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
					...memory.createWriteU8Nodes(new LocalGet("address"), new LocalGet("value")),
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
					...memory.createWriteU16Nodes(new LocalGet("address"), new LocalGet("value")),
				],
			}),
		);

		// TODO write some tests, proving that bytes copy back and forth both using memory directly and calling the wasm functions
	});
});
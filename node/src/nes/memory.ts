import * as wat from "../wat";

export const WEBASSEMBLY_PAGE_SIZE = 65536;
export const NES_MAX_ADDRESS = 0xffff;

export abstract class Memory {
	abstract readU8(address: number): number;
	abstract readU16(address: number): number;
	abstract writeU8(address: number, value: number): void;
	abstract writeU16(address: number, value: number): void;

	abstract createReadU8Node(address: wat.Node): wat.Statement;
	abstract createReadU16Node(address: wat.Node): wat.Statement;
	abstract createWriteU8Node(address: wat.Node, value: wat.Node): wat.Statement;
	abstract createWriteU16Node(address: wat.Node, value: wat.Node): wat.Statement;
}
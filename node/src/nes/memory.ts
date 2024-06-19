import { Node } from "./node";

export const WEBASSEMBLY_PAGE_SIZE = 65536;
export const NES_MAX_ADDRESS = 0xffff;

export abstract class Memory {
	abstract readU8(address: number): number;
	abstract readU16(address: number): number;
	abstract writeU8(address: number, value: number): void;
	abstract writeU16(address: number, value: number): void;

	abstract createReadU8Node(address: Node): Node;
	abstract createReadU16Node(address: Node): Node;
	abstract createWriteU8Node(address: Node, value: Node): Node;
	abstract createWriteU16Node(address: Node, value: Node): Node;
}
import { SExpr } from "../sexpr";
import { step } from "./step";

export const BLOCK_SIZE_IN_BYTES = 64 * 1024;

export interface ImportObject {
	imports: {
		memory: WebAssembly.Memory;
	};
}

export function module(importObject: ImportObject): SExpr {
	return new SExpr([
		"module",
		[
			"import",
			"\"imports\"",
			"\"memory\"",
			[
				"memory",
				"$memory",
				getMemorySizeInBlocks(importObject.imports.memory.buffer.byteLength).toString(),
			]
		],
		step(),
	]);
}

export function importObject(initialMemory: Buffer): ImportObject {
	const memory = new WebAssembly.Memory({
		initial: getMemorySizeInBlocks(initialMemory.byteLength),
	});
	return {
		imports: {
			memory,
		},
	};
}

function getMemorySizeInBlocks(byteLength: number): number {
	return Math.max(Math.ceil(byteLength / BLOCK_SIZE_IN_BYTES), 1);
}
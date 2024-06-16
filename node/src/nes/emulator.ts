import wabt from "wabt";
import { Logger } from "../logger";
import { SExpr } from "../sexpr";
import { CPURegisterNames } from "./cpu";

const BLOCK_SIZE_IN_BYTES = 64 * 1024;

export class Emulator {
	private _step: () => void;

	static async createEmulator(
		{ initialMemory, logger }: {
			initialMemory: Buffer;
			logger: Logger;
		}
	): Promise<Emulator> {
		const memorySizeInBlocks = Math.max(Math.ceil(initialMemory.byteLength / BLOCK_SIZE_IN_BYTES), 1);

		const memory = new WebAssembly.Memory({
			initial: memorySizeInBlocks,
		});
		initialMemory.copy(new Uint8Array(memory.buffer));

		const moduleExpr = new SExpr([
			"module",

			// memory
			[
				"import",
				"\"imports\"",
				"\"memory\"",
				[
					"memory",
					"$memory",
					memorySizeInBlocks.toString(),
				]
			],

			// registers
			...[
				CPURegisterNames.PC,
				CPURegisterNames.SP,
				CPURegisterNames.A,
				CPURegisterNames.X,
				CPURegisterNames.Y,
				CPURegisterNames.FLAGS,
			].map(name => [
				"global",
				name,
				"i32",
				[
					"i32.const",
					"0"
				]
			]),

			// functions
			[
				"func",
				"$step",
				[
					"export",
					"\"step\""
				],
				/*
				TODO implement me
		
				read at PC
				inc PC
				switch on that byte
				each value is some instruction step
				*/
				"unreachable"
			],
		]);
		const moduleExprStr = moduleExpr.toString();
		logger.debug(`wasm module:\n${moduleExprStr}`);

		const wabtModule = (await wabt()).parseWat("", moduleExprStr);

		const { instance } = await WebAssembly.instantiate(
			wabtModule.toBinary({}).buffer,
			{
				imports: {
					memory,
				},
			}
		);

		return new Emulator(instance);
	}

	private constructor(private readonly instance: WebAssembly.Instance
	) {
		this._step = this.instance.exports.step as typeof this._step;
	}

	step() {
		this._step();
	}

	/*
	TODO various accessors, read registers and flags out of memory
	*/
}

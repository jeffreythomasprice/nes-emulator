import wabt from "wabt";
import { toBinaryStringU8, toHexStringU16, toHexStringU8 } from "../formatting";
import { Logger } from "../logger";
import { SExpr } from "../sexpr";
import { CPURegisterNames } from "./cpu";
import { MemoryController } from "./memory";

const BLOCK_SIZE_IN_BYTES = 64 * 1024;

export class Emulator {
	private exports: {
		[CPURegisterNames.PC]: WebAssembly.Global<"i32">;
		[CPURegisterNames.SP]: WebAssembly.Global<"i32">;
		[CPURegisterNames.A]: WebAssembly.Global<"i32">;
		[CPURegisterNames.X]: WebAssembly.Global<"i32">;
		[CPURegisterNames.Y]: WebAssembly.Global<"i32">;
		[CPURegisterNames.FLAGS]: WebAssembly.Global<"i32">;
		step: () => void;
	};
	private readonly memory: Uint8Array;

	static async createEmulator(
		{
			initialMemory,
			memoryController,
			logger,
		}: {
			initialMemory: Buffer;
			memoryController: MemoryController;
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
				[
					"export",
					`"${name}"`,
				],
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

		return new Emulator(logger, instance, memory, memoryController);
	}

	private constructor(
		private readonly logger: Logger,
		private readonly instance: WebAssembly.Instance,
		memory: WebAssembly.Memory,
		private readonly memoryController: MemoryController,
	) {
		this.exports = this.instance.exports as typeof this.exports;
		this.memory = new Uint8Array(memory.buffer);
	}

	step() {
		try {
			this.exports.step();
		} catch (err) {
			this.logger.error([
				`pc=${toHexStringU16(this.pc)} (memory[pc]=${toHexStringU8(this.getMemoryU8(this.pc))})`,
				`sp=${toHexStringU8(this.sp)}`,
				`sa=${toHexStringU8(this.a)}`,
				`x=${toHexStringU8(this.x)}`,
				` y=${toHexStringU8(this.y)}`,
				`flags=${toBinaryStringU8(this.flags)}`,
				// TODO parse out individual flags
			].join(", "));
			throw err;
		}
	}

	get pc(): number {
		return this.exports.$pc.value;
	}

	// TODO setter

	get sp(): number {
		return this.exports.$sp.value;
	}

	// TODO setter

	get a(): number {
		return this.exports.$a.value;
	}

	// TODO setter

	get x(): number {
		return this.exports.$x.value;
	}

	// TODO setter

	get y(): number {
		return this.exports.$y.value;
	}

	// TODO setter

	get flags(): number {
		return this.exports.$flags.value;
	}

	// TODO setter

	// TODO getters and setters for individual flags

	getMemoryU8(address: number): number {
		return this.memoryController.readU8FromMemory(this.memory, address);
	}

	// TODO getMemoryU16
	// TODO setters for memory
}

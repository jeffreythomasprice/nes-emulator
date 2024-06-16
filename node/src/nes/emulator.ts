import wabt from "wabt";
import * as module from "./module";

export class Emulator {
	private _step: () => void;

	static async createEmulator(initialMemory: Buffer): Promise<Emulator> {
		const importObject = module.importObject(initialMemory);
		const moduleExpr = module.module(importObject);
		const wabtInstance = await wabt();
		const wabtModule = wabtInstance.parseWat(
			"",
			moduleExpr.toString()
		);
		const { instance } = await WebAssembly.instantiate(
			wabtModule.toBinary({}).buffer,
			{
				...importObject,
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
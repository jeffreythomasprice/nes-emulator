import { Function, I32Add, LocalGet, Module } from ".";

describe(__filename, () => {
	it("empty module", () => {
		expect(new Module()
			.createSexpr()
			.toString("", "    ")
		)
			.toEqual("(module)");
	});

	it("add example", async () => {
		// https://github.com/eliben/wasm-wat-samples/blob/main/add/add.wat
		const module = new Module(
			new Function({
				name: "add",
				export: "add",
				params: [
					{
						name: "a",
						type: "i32",
					}, {
						name: "b",
						type: "i32",
					}
				],
				returnType: "i32",
				statements: [
					new I32Add(
						new LocalGet("a"),
						new LocalGet("b"),
					),
				],
			})
		);
		expect(module
			.createSexpr()
			.toString("", "\t")
		)
			.toEqual(`(module
	(func
		$add
		(export "add")
		(param $a i32)
		(param $b i32)
		(result i32)
		(i32.add
			(local.get $a)
			(local.get $b)
		)
	)
)`);

		const instance = await module.instantiate();
		const add = instance.exports["add"] as (a: number, b: number) => number;
		expect(add(1, 2)).toBe(3);
	});
});
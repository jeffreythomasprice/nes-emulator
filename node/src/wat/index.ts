/*
Useful references:
https://developer.mozilla.org/en-US/docs/WebAssembly/Reference
*/

import wabt from "wabt";
import * as sexpr from "../sexpr";

export type PrimitiveType = "i32" | "i64" | "f32" | "f64" | "v128" | "funcref" | "externref";

export abstract class Node {
	abstract get type(): string;
	abstract createSexpr(): sexpr.Node;
}

export abstract class Statement extends Node {
	get type(): string {
		return `Statement-${this.statementType}`;
	}

	abstract get statementType(): string;
}

abstract class BinaryOp extends Statement {
	constructor(
		readonly statementType: string,
		readonly left: Node,
		readonly right: Node,
	) {
		super();
	}

	createSexpr(): sexpr.Node {
		return new sexpr.List(
			this.statementType,
			this.left.createSexpr(),
			this.right.createSexpr(),
		);
	}
}

export class Module extends Node {
	readonly nodes: ReadonlyArray<Node>;

	constructor(...nodes: ReadonlyArray<Node>) {
		super();

		this.nodes = [...nodes];
	}

	get type(): string {
		return "Module";
	}

	createSexpr(): sexpr.Node {
		return new sexpr.List(
			"module",
			...this.nodes.map(n => n.createSexpr()),
		);
	}

	async parse(): Promise<{
		buffer: Uint8Array;
		log: string;
	}> {
		return (await wabt())
			.parseWat("", this.createSexpr().toString("", "    "))
			.toBinary({});;
	}

	async compile(): Promise<WebAssembly.Module> {
		return await WebAssembly.compile((await this.parse()).buffer);
	}

	async instantiate(imports?: WebAssembly.Imports): Promise<WebAssembly.Instance> {
		return await WebAssembly.instantiate(await this.compile(), imports);
	}
}

export class Function extends Node {
	readonly name: string;
	readonly export?: string;
	readonly params: ReadonlyArray<{
		name: string;
		type: string;
	}>;
	readonly returnType?: PrimitiveType;
	readonly statements: ReadonlyArray<Statement>;

	constructor(args: {
		readonly name: string;
		readonly export?: string;
		readonly params: ReadonlyArray<{
			name: string;
			type: PrimitiveType;
		}>;
		readonly returnType?: PrimitiveType;
		readonly statements: ReadonlyArray<Statement>;
	}) {
		super();

		this.name = args.name;
		this.export = args.export;
		this.params = [...args.params];
		this.returnType = args.returnType;
		this.statements = [...args.statements];
	}

	get type(): string {
		return "Function";
	}

	createSexpr(): sexpr.Node {
		const results: sexpr.NodeInitializer[] = [
			"func",
			identifier(this.name),
		];
		if (this.export) {
			results.push([
				"export",
				quotedString(this.export),
			]);
		}
		results.push(...this.params.map(p => [
			"param",
			identifier(p.name),
			p.type,
		]));
		if (this.returnType) {
			results.push([
				"result",
				this.returnType,
			]);
		}
		results.push(...this.statements.map(p => p.createSexpr()));
		return new sexpr.List(...results);
	}
}

/*
TODO numeric statements

const
equal
not equal
greater than
less than
greater than or equal
less or equal
*/

export class I32Add extends BinaryOp {
	constructor(left: Node, right: Node) {
		super("i32.add", left, right);
	}
}

export class I64Add extends BinaryOp {
	constructor(left: Node, right: Node) {
		super("i64.add", left, right);
	}
}

export class F32Add extends BinaryOp {
	constructor(left: Node, right: Node) {
		super("f32.add", left, right);
	}
}

export class F64Add extends BinaryOp {
	constructor(left: Node, right: Node) {
		super("f64.add", left, right);
	}
}

/*
TODO numeric statements

subtraction
multiplication
division
remainder
extend
wrap
promote
demote
convert
truncate
reinterpret
min
max
nearest
ceil
floor
truncate
absolute
negate
square root
copy sign
and
or
xor
left shift
right shift
left rotate
right rotate
count leading zeros
count trailing zeros
population count
*/

/*
TODO variable statements

declare local
*/

export class LocalGet extends Statement {
	constructor(readonly name: string) {
		super();
	}

	get statementType(): string {
		return "local.get";
	}

	createSexpr(): sexpr.Node {
		return new sexpr.List(
			"local.get",
			identifier(this.name),
		);
	}
}

/*
TODO variable statements

set local
tee local
declare globa
get global
set global
*/

/*
TODO memory statements

grow
size
lead
store
copy
fill
*/

/*
TODO control flow statements

block
br
call
drop
end
if else
loop
nop
return
select
unreachable
*/

function identifier(value: string): sexpr.Value {
	return new sexpr.Value(`$${value}`);
}

function quotedString(value: string): sexpr.Value {
	return new sexpr.Value(`"${value.replace('"', '\\"')}"`);
}
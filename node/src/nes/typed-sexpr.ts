import { SExpr } from "../sexpr";

export class TypedSExpr extends SExpr {
	constructor(
		readonly type: string,
		value: SExpr
	) {
		super(value);
	}
}

export class U8SExpr extends TypedSExpr {
	constructor(value: SExpr) {
		super("u8", value);
	}

	readonly type = "u8";
}

export class U16SExpr extends TypedSExpr {
	constructor(value: SExpr) {
		super("u16", value);
	}

	readonly type = "u16";
}

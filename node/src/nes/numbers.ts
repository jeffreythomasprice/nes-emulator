import { toHexStringU16 } from "../formatting";
import { SExpr } from "../sexpr";
import { MemoryModel } from "./memory-model";
import { U16SExpr, U8SExpr } from "./typed-sexpr";

export function wrapToU16(value: SExpr): U16SExpr {
	if (value instanceof U16SExpr) {
		return value;
	}
	if (value instanceof U8SExpr) {
		return new U16SExpr(value);
	}
	return new U16SExpr(new SExpr([
		"i32.rem_u",
		value,
		"0xffff",
	]));
}

export function fixAddress(value: SExpr): SExpr {
	return new SExpr([
		"i32.add",
		wrapToU16(value),
		toHexStringU16(MemoryModel.MEMORY_BANK_START),
	]);
}
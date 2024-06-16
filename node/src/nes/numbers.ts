import { SExpr } from "../sexpr";
import { U16SExpr, U8SExpr } from "./typed-sexpr";

// TODO needed?
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

import { toHexStringU16, toHexStringU8 } from "../formatting";
import { SExpr } from "../sexpr";
import { MemoryModel } from "./memory-model";
import { U16SExpr, U8SExpr } from "./typed-sexpr";

export function getPC(): U16SExpr {
	return new U16SExpr(new SExpr([
		"i32.load16_u",
		toHexStringU16(MemoryModel.CPU_REGISTER_PC)
	]));
}

export function setPC(value: SExpr): SExpr {
	return new SExpr([
		"i32.store16",
		toHexStringU16(MemoryModel.CPU_REGISTER_PC),
		value
	]);
}

export function getSP(): U8SExpr {
	return new U8SExpr(new SExpr([
		"i32.load8_u",
		toHexStringU16(MemoryModel.CPU_REGISTER_SP)
	]));
}

export function setSP(value: SExpr): SExpr {
	return new SExpr([
		"i32.store8",
		toHexStringU8(MemoryModel.CPU_REGISTER_SP),
		value
	]);
}

export function getA(): U8SExpr {
	return new U8SExpr(new SExpr([
		"i32.load8_u",
		toHexStringU16(MemoryModel.CPU_REGISTER_A)
	]));
}

export function setA(value: SExpr): SExpr {
	return new SExpr([
		"i32.store8",
		toHexStringU8(MemoryModel.CPU_REGISTER_A),
		value
	]);
}

export function getX(): U8SExpr {
	return new U8SExpr(new SExpr([
		"i32.load8_u",
		toHexStringU16(MemoryModel.CPU_REGISTER_X)
	]));
}

export function setX(value: SExpr): SExpr {
	return new SExpr([
		"i32.store8",
		toHexStringU8(MemoryModel.CPU_REGISTER_X),
		value
	]);
}

export function getY(): U8SExpr {
	return new U8SExpr(new SExpr([
		"i32.load8_u",
		toHexStringU16(MemoryModel.CPU_REGISTER_Y)
	]));
}

export function setY(value: SExpr): SExpr {
	return new SExpr([
		"i32.store8",
		toHexStringU8(MemoryModel.CPU_REGISTER_Y),
		value
	]);
}

export function getFlags(): U8SExpr {
	return new U8SExpr(new SExpr([
		"i32.load8_u",
		toHexStringU16(MemoryModel.CPU_REGISTER_FLAGS)
	]));
}

export function setFlags(value: SExpr): SExpr {
	return new SExpr([
		"i32.store8",
		toHexStringU8(MemoryModel.CPU_REGISTER_FLAGS),
		value
	]);
}

/*
TODO get and set individual flags
*/
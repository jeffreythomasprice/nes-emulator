import { SExpr } from "../sexpr";
import { U16SExpr, U8SExpr } from "./typed-sexpr";

export enum CPURegisterNames {
	PC = "$pc",
	SP = "$sp",
	A = "$a",
	X = "$x",
	Y = "$y",
	FLAGS = "$flags",
}

export enum CPUFlagsRegister {
	CARRY = 0b0000_0001,
	ZERO = 0b0000_0010,
	INTERRUPT_DISABLE = 0b0000_0100,
	DECIMAL_MODE = 0b0000_1000,
	BREAK_COMMAND = 0b0001_0000,
	// UNUSED = 0b0010_0000,
	OVERFLOW = 0b0100_0000,
	NEGATIVE = 0b1000_0000,
}

export function getPC(): U16SExpr {
	return new U16SExpr(new SExpr([
		"global.get",
		CPURegisterNames.PC,
	]));
}

export function setPC(value: SExpr): SExpr {
	return new SExpr([
		"global.set",
		CPURegisterNames.PC,
		value
	]);
}

export function getSP(): U8SExpr {
	return new U8SExpr(new SExpr([
		"global.get",
		CPURegisterNames.SP,
	]));
}

export function setSP(value: SExpr): SExpr {
	return new SExpr([
		"global.set",
		CPURegisterNames.SP,
		value
	]);
}

export function getA(): U8SExpr {
	return new U8SExpr(new SExpr([
		"global.get",
		CPURegisterNames.A,
	]));
}

export function setA(value: SExpr): SExpr {
	return new SExpr([
		"global.set",
		CPURegisterNames.A,
		value
	]);
}

export function getX(): U8SExpr {
	return new U8SExpr(new SExpr([
		"global.get",
		CPURegisterNames.X,
	]));
}

export function setX(value: SExpr): SExpr {
	return new SExpr([
		"global.set",
		CPURegisterNames.X,
		value
	]);
}

export function getY(): U8SExpr {
	return new U8SExpr(new SExpr([
		"global.get",
		CPURegisterNames.Y,
	]));
}

export function setY(value: SExpr): SExpr {
	return new SExpr([
		"global.set",
		CPURegisterNames.Y,
		value
	]);
}

export function getFlags(): U8SExpr {
	return new U8SExpr(new SExpr([
		"global.get",
		CPURegisterNames.FLAGS,
	]));
}

export function setFlags(value: SExpr): SExpr {
	return new SExpr([
		"global.set",
		CPURegisterNames.FLAGS,
		value
	]);
}

/*
TODO get and set individual flags
*/
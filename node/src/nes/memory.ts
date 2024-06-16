import { SExpr } from "../sexpr";
import { wrapToU16 } from "./numbers";

export interface MemoryController {
	readU8Expr(address: SExpr): SExpr;
	// TODO writeU8Expr
	// TODO readU16Expr
	// TODO writeU16Expr

	readU8FromMemory(memory: Uint8Array, address: number): number;
	// TODO writeU8ToMemory
	// TODO readU16ToMemory
	// TODO writeU16ToMemory
}

/**
 * No bank switching. The whole 16-bit address is considered RAM with no special case memory mapping. Intended for unit tests.
 */
export class SingleBankAllRAMMemoryController implements MemoryController {
	readU8Expr(address: SExpr): SExpr {
		return new SExpr(["i32.load8_u", wrapToU16(address)]);
	}

	readU8FromMemory(memory: Uint8Array, address: number): number {
		// TODO wrap address to u16
		return memory[address];
	}
}

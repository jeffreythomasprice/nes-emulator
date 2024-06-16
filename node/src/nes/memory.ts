import { SExpr } from "../sexpr";
import { fixAddress } from "./numbers";

export interface MemoryController {
	readU8(address: SExpr): SExpr;
	// TODO writeU8
	// TODO readU16
	// TODO writeU16
}

/**
 * No bank switching. The whole 16-bit address is considered RAM with no special case memory mapping. Intended for unit tests.
 */
export class SingleBankAllRAMMemoryController implements MemoryController {
	readU8(address: SExpr): SExpr {
		return new SExpr(["i32.load8_u", fixAddress(address)]);
	}
}
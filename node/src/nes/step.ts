import { SExpr } from "../sexpr";

export function step(): SExpr {
	return new SExpr([
		"func",
		"$step",
		[
			"export",
			"\"step\""
		],
		/*
		TODO implement me

		read at PC
		inc PC
		switch on that byte
		each value is some instruction step
		*/
	]);
}
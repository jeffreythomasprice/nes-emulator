type SExprConstructorArg = string | string[] | SExpr | SExpr[] | SExprConstructorArg[];

export class SExpr {
	readonly value: string | ReadonlyArray<SExpr>;
	readonly comment?: string;

	constructor(value: SExprConstructorArg);
	constructor(expr: {
		value: SExprConstructorArg;
		comment?: string;
	});
	constructor(arg: SExprConstructorArg | {
		value: SExprConstructorArg;
		comment?: string;
	}) {
		if (typeof arg === "string") {
			this.value = arg;
			return;
		}

		if (Array.isArray(arg)) {
			this.value = arg.map(a => new SExpr(a));
			return;
		}

		if (arg instanceof SExpr) {
			this.value = arg.value;
			this.comment = arg.comment;
			return;
		}

		this.value = new SExpr(arg.value).value;
		this.comment = arg.comment;
	}

	toString(indent: string = "\t"): string {
		return this.toStringHelper(indent, "").trimEnd();
	}

	private toStringHelper(eachIndent: string, currentIndent: string): string {
		let result = "";

		if (this.comment) {
			for (const line of this.comment.split("\n")) {
				result += currentIndent;
				result += ";; ";
				result += line;
				result += "\n";
			}
		}

		if (typeof this.value === "string") {
			result += currentIndent;
			result += this.value;
			result += "\n";
		} else {
			// empty list
			if (this.value.length === 0) {
				result += currentIndent;
				result += "()";
				result += "\n";
			}
			// list starts with a single string
			else if (this.value.length >= 1 && !this.value[0].comment && typeof this.value[0].value === "string") {
				result += currentIndent;
				result += "(";
				result += this.value[0].value;
				// every remaining value is a single string, no comments
				const remaining = this.value.slice(1);
				if (all(
					remaining,
					(item) => typeof item.value === "string" && !item.comment
				)) {
					for (const item of remaining) {
						result += " ";
						result += item.value;
					}
					result += ")";
					result += "\n";
				}
				// otherwise, each element on it's own line
				else {
					result += "\n";
					const nextIndent = currentIndent + eachIndent;
					for (const item of remaining) {
						result += item.toStringHelper(eachIndent, nextIndent);
					}
					result += ")";
					result += "\n";
				}
			}
			// default case, every element on it's own line
			else {
				result += currentIndent;
				result += "(";
				result += "\n";
				const nextIndent = currentIndent + eachIndent;
				for (const item of this.value) {
					result += item.toStringHelper(eachIndent, nextIndent);
				}
				result += currentIndent;
				result += ")";
				result += "\n";
			}
		}

		return result;
	}
}

// TODO move me
function all<T, S extends T>(items: T[], predicate: (item: T) => item is S): items is S[];
function all<T>(items: T[], predicate: (item: T) => boolean): boolean;
function all<T>(items: T[], predicate: (item: T) => boolean): boolean {
	for (const item of items) {
		if (!predicate(item)) {
			return false;
		}
	}
	return true;
}
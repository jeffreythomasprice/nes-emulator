import { all } from "../array-utils";

export type NodeInitializer = string | Node | (string | Node)[];

export abstract class Node {
	abstract get type(): string;
	abstract toString(currentIndent: string, indentIncrement: string): string;
}

export class Value extends Node {
	constructor(readonly value: string) {
		super();
	}

	get type(): string {
		return "Value";
	}

	toString(currentIndent: string, _indentIncrement: string): string {
		return `${currentIndent}${this.value}`;
	}
}

export class Comment extends Node {
	constructor(readonly value: string) {
		super();
	}

	get type(): string {
		return "Comment";
	}

	toString(currentIndent: string, _indentIncrement: string): string {
		return this.value
			.split("\n")
			.map(line => `${currentIndent};; ${line}`)
			.join("\n");
	}
}

export class List extends Node {
	readonly items: ReadonlyArray<Node>;

	constructor(...items: ReadonlyArray<NodeInitializer>) {
		super();

		this.items = items
			.map(node)
			.filter((x): x is Node => !!x);
	}

	get type(): string {
		return "List";
	}

	toString(currentIndent: string, indentIncrement: string): string {
		let result = `${currentIndent}(`;
		if (this.items.length === 0) {
			result += ")";
			return result;
		}

		if (all(this.items, item => item instanceof Value)) {
			result += this.items.map(item => item.toString("", "")).join(" ");
			result += ")";
			return result;
		}

		if (this.items[0] instanceof Value) {
			result += this.items[0].toString("", "");
			result += "\n";
			for (const item of this.items.slice(1)) {
				result += item.toString(currentIndent + indentIncrement, indentIncrement);
				result += "\n";
			}
			result += currentIndent;
			result += ")";
			return result;
		}

		result += "\n";
		for (const item of this.items) {
			result += item.toString(currentIndent + indentIncrement, indentIncrement);
			result += "\n";
		}
		result += currentIndent;
		result += ")";
		return result;
	}
}

export function node(x: NodeInitializer): Node | undefined {
	if (Array.isArray(x)) {
		return new List(...x);
	}
	if (x instanceof Node) {
		return x;
	}
	const s = x.trim();
	if (s.startsWith(";;")) {
		return new Comment(s.substring(2).trim());
	}
	if (s.length > 0) {
		return new Value(s);
	}
}
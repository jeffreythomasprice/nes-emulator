import * as wat from "../wat";

export abstract class Node {
	abstract get type(): string;
	abstract createWat(): wat.Node;
}

export class WatWrapperNode extends Node {
	constructor(private readonly node: wat.Node) {
		super();
	}

	get type(): string {
		return `wat-wrapper(${this.node.type})`;
	}

	createWat(): wat.Node {
		return this.node;
	}
}
import * as wat from "../wat";

export abstract class Node {
	abstract get type(): string;
	abstract createWat(): wat.Node;
}

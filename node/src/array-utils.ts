export function all<T, S extends T>(items: ReadonlyArray<T>, predicate: (item: T) => item is S): items is S[];
export function all<T>(items: ReadonlyArray<T>, predicate: (item: T) => boolean): boolean;
export function all<T>(items: ReadonlyArray<T>, predicate: (item: T) => boolean): boolean {
	for (const item of items) {
		if (!predicate(item)) {
			return false;
		}
	}
	return true;
}
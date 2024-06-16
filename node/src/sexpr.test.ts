import { SExpr } from "./sexpr";

describe(__filename, () => {
	it("single string", () => {
		expect(new SExpr("singleStringValue").toString())
			.toBe("singleStringValue");
	});

	it("multiple single string values", () => {
		expect(new SExpr(["multiple", "single", "string", "values"]).toString())
			.toBe("(multiple single string values)");
	});

	it("list with nested list", () => {
		expect(new SExpr([
			"single",
			"string",
			"or",
			[
				"some",
				"nested",
				"stuff"
			]
		]).toString())
			.toBe(`(single
	string
	or
	(some nested stuff)
)`);
	});

	it("first element is a list", () => {
		expect(new SExpr([
			[
				"first",
				"thing",
				"is",
				"a",
				"list",
			],
			"more",
			"stuff",
		]).toString())
			.toBe(`(
	(first thing is a list)
	more
	stuff
)`);
	});

	it("single string with a comment", () => {
		expect(new SExpr({
			value: "singleStringValue",
			comment: "comment",
		}).toString())
			.toBe(`;; comment
singleStringValue`);
	});

	it("multiple strings with a comment", () => {
		expect(new SExpr({
			value: ["multiple", "string", "values"],
			comment: "comment",
		}).toString())
			.toBe(`;; comment
(multiple string values)`);
	});

	it("inner list and multiple comments", () => {
		expect(new SExpr({
			value: [
				"single",
				"string",
				"or",
				new SExpr({
					comment: "another comment",
					value: ["some", "nested", "stuff"]
				})
			],
			comment: "comment",
		}).toString())
			.toBe(`;; comment
(single
	string
	or
	;; another comment
	(some nested stuff)
)`);
	});

	it("multi-line comment before the first element", () => {
		expect(new SExpr({
			value: [
				new SExpr({
					comment: `a multi-line
comment before the first element`,
					value: "single",
				}),
				"string",
				"or",
				new SExpr({
					comment: "another comment",
					value: ["some", "nested", "stuff"]
				})
			],
			comment: "comment",
		}).toString())
			.toBe(`;; comment
(
	;; a multi-line
	;; comment before the first element
	single
	string
	or
	;; another comment
	(some nested stuff)
)`);
	});

	it("comment before the first element, which is a list", () => {
		expect(new SExpr({
			value: [
				new SExpr({
					comment: "comment",
					value: ["first", "thing", "is", "a", "list"],
				}),
				"more",
				new SExpr({
					comment: "more comments",
					value: "stuff"
				})
			],
			comment: "comment",
		}).toString())
			.toBe(`;; comment
(
	;; comment
	(first thing is a list)
	more
	;; more comments
	stuff
)`);
	});

	it("multiple nested lists", () => {
		expect(new SExpr([
			"a",
			[
				"b",
				[
					"c",
					"d"
				]
			]
		]).toString())
			.toBe(`(a
	(b
		(c d)
	)
)`);
	});
});
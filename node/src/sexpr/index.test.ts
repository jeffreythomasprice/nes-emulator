import { Comment, List, Value, node } from ".";

describe(__filename, () => {
	it("single value", () => {
		expect(new Value("foobar").toString("    ", "  "))
			.toEqual("    foobar");
	});

	it("empty value", () => {
		expect(new Value("").toString("    ", "  "))
			.toEqual("    ");
	});

	it("comment", () => {
		expect(new Comment("foobar").toString("    ", "  "))
			.toEqual("    ;; foobar");
	});

	it("empty comment", () => {
		expect(new Comment("").toString("    ", "  "))
			.toEqual("    ;; ");
	});

	it("multi-line comment", () => {
		expect(new Comment("foo\n bar\nbaz").toString("    ", "  "))
			.toEqual("    ;; foo\n    ;;  bar\n    ;; baz");
	});

	it("empty list", () => {
		expect(new List().toString("    ", "  "))
			.toEqual("    ()");
	});

	it("list with single value", () => {
		expect(new List(
			new Value("foo"),
		).toString("    ", "  "))
			.toEqual("    (foo)");
	});

	it("list with multiple value", () => {
		expect(new List(
			new Value("foo"),
			new Value("bar"),
			new Value("baz"),
		).toString("    ", "  "))
			.toEqual("    (foo bar baz)");
	});

	it("list with nested list", () => {
		expect(new List(
			new Value("foo"),
			new List(
				new Value("bar"),
				new Value("baz"),
			),
			new Value("asdf"),
		).toString("    ", "  "))
			.toEqual("    (foo\n      (bar baz)\n      asdf\n    )");
	});

	it("list with leading nested list", () => {
		expect(new List(
			new List(
				new Value("foo"),
				new Value("bar"),
			),
			new Value("baz"),
			new Value("asdf"),
		).toString("    ", "  "))
			.toEqual("    (\n      (foo bar)\n      baz\n      asdf\n    )");
	});

	it("list with nested comment", () => {
		expect(new List(
			new Value("foo"),
			new Comment("comment"),
			new Value("bar"),
			new Value("baz"),
		).toString("    ", "  "))
			.toEqual("    (foo\n      ;; comment\n      bar\n      baz\n    )");
	});

	it("list with leading nested comment", () => {
		expect(new List(
			new Comment("comment"),
			new Value("foo"),
			new Value("bar"),
			new Value("baz"),
		).toString("    ", "  "))
			.toEqual("    (\n      ;; comment\n      foo\n      bar\n      baz\n    )");
	});

	describe(node.name, () => {
		it("already a node", () => {
			expect(node(new Value("foo")))
				.toEqual(new Value("foo"));
		});

		it("empty", () => {
			expect(node(""))
				.toBeUndefined();
		});

		it("value", () => {
			expect(node("  foo  "))
				.toEqual(new Value("foo"));
		});

		it("comment", () => {
			expect(node("  ;; foo     "))
				.toEqual(new Comment("foo"));
		});
	});
});
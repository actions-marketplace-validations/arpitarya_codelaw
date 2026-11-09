use super::block::{Block, inline_tags, is_jsdoc};
use super::types::{self, Printer};

fn block(src: &str) -> Block<'_> {
    Block::parse(src, 0, src.len()).expect("a JSDoc block")
}

fn canonical(t: &str) -> String {
    let ty = types::parse(t).unwrap_or_else(|e| panic!("`{t}` does not parse: {e}"));
    Printer { src: t, shallow: false }.print(&ty)
}

#[test]
fn only_double_asterisk_blocks_are_jsdoc() {
    assert!(is_jsdoc("/** a */"));
    assert!(!is_jsdoc("/*** a */"));
    assert!(!is_jsdoc("/**/"));
    assert!(!is_jsdoc("/* a */"));
}

#[test]
fn a_block_has_description_tags_types_and_names() {
    let src = "/**\n * Adds.\n *\n * More.\n * @param {Array<string>} [list=[]] the list,\n *   one per line\n * @returns {number}\n */";
    let b = block(src);
    assert_eq!(b.tags.len(), 2);
    assert_eq!(b.description(), "Adds.\n\nMore.");
    let p = &b.tags[0];
    assert_eq!(p.tag, "param");
    assert_eq!(p.type_text(src), Some("Array<string>"));
    assert_eq!(p.name, "list");
    assert!(p.optional);
    assert_eq!(p.default.as_deref(), Some("[]"));
    assert_eq!(b.tag_description(p), "the list,\none per line");
    assert_eq!(b.tags[1].tag, "returns");
    assert!(b.tags[1].name.is_empty(), "@returns has no name");
}

#[test]
fn a_type_may_span_lines_and_braces_nest() {
    let src = "/**\n * @param {{\n *   a: string,\n *   b: {c: number}\n * }} opts options\n */";
    let b = block(src);
    let t = &b.tags[0];
    assert!(t.type_text(src).unwrap().contains("b: {c: number}"));
    assert_eq!(t.name, "opts");
}

#[test]
fn a_fenced_line_is_not_a_tag() {
    let src = "/**\n * Example:\n * ```\n * @foo\n * ```\n * @param a\n */";
    let b = block(src);
    assert_eq!(b.tags.len(), 1);
    assert_eq!(b.tags[0].tag, "param");
}

#[test]
fn a_one_line_block_and_template_lists() {
    let b = block("/** @template T, [U=T[]] the types */");
    let t = &b.tags[0];
    assert_eq!(t.tag, "template");
    assert_eq!(t.name, "T, [U=T[]]");
    let b = block("/** @param {string} a */");
    assert_eq!(b.tags[0].name, "a");
    assert_eq!(b.lines.len(), 1);
}

#[test]
fn inline_tags_are_found() {
    let v = inline_tags("see {@link x|y} and {@code z}", 10);
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].tag, "link");
    assert_eq!(v[0].text, "x|y");
    assert_eq!(v[0].start, 14);
    assert_eq!(v[1].tag, "code");
}

#[test]
fn types_parse_and_print_in_the_default_format() {
    for t in [
        "string",
        "string[]",
        "string | number",
        "Map<string, Set<number>>",
        "?string",
        "!Object",
        "string=",
        "...number",
        "{a: string, b?: number}",
        "{}",
        "[string, number]",
        "function(string, number): boolean",
        "(a: string) => void",
        "new (a: string) => Foo",
        "<T>(x: T) => T",
        "typeof import('x')",
        "import('x').Y",
        "T extends U ? X : Y",
        "keyof T",
        "T[K]",
        "`a${string}b`",
        "\"a\" | \"b\"",
        "module:foo/bar.baz",
        "Foo#bar",
        "*",
        "{[key: string]: number}",
        "{(a: string): void}",
        "{readonly a: string}",
        "(string | number)[]",
    ] {
        assert_eq!(canonical(t), t, "`{t}` is already in the default format");
    }
}

#[test]
fn types_are_rewritten_to_the_default_format() {
    for (from, to) in [
        ("string|number", "string | number"),
        ("Array<string>", "string[]"),
        ("Array<string|number>", "(string | number)[]"),
        ("Array.<string>", "string[]"),
        ("Map.<string,number>", "Map<string, number>"),
        ("{a:string;b:number}", "{a: string, b: number}"),
        ("{ a: string }", "{a: string}"),
        ("'a'", "\"a\""),
        ("{\"a\": string}", "{a: string}"),
        ("function(string,number):boolean", "function(string, number): boolean"),
        ("(a:string)=>void", "(a: string) => void"),
        ("[string,number]", "[string, number]"),
        ("<T,U>(x:T)=>U", "<T, U>(x: T) => U"),
    ] {
        assert_eq!(canonical(from), to);
    }
}

#[test]
fn bad_types_do_not_parse() {
    for t in ["Array<string", "string[", "string|", "{a: string", "(string", "a b", "<>", "Array<>", "{a: }", "[string,"] {
        assert!(types::parse(t).is_err(), "`{t}` should not parse");
    }
}

#[test]
fn a_template_literal_type_keeps_its_parts() {
    let ty = types::parse("`a${'b'}c${number}`").unwrap();
    let types::Kind::Template { parts, interps } = &ty.kind else { panic!("a template") };
    assert_eq!(parts, &["a", "c", ""]);
    assert_eq!(interps.len(), 2);
}

//! W-195: TSLint's `comment-type`, written again. A comment is one of three
//! types — `singleline` (`//`), `multiline` (`/* */`) or `doc` (`/** */`) —
//! and the rule's `types` option lists the ones a project writes (by default
//! `singleline` and `doc`). A comment of another type is rewritten as the
//! first allowed type that can hold it: a block as `//` lines, a run of `//`
//! lines as one block, a doc comment as a plain block.
//!
//! What is left alone, so the rewrite never changes what a tool reads: a
//! directive or pragma (`/// <reference>`, `// @ts-…`, `/* eslint-… */`,
//! `/*! licence */`, `/*#__PURE__*/`, `//# sourceMappingURL`, …), a comment in
//! JSX braces, a block with code on both sides of it on one line
//! (`f(/* x */ 1)`), and a `//` comment whose text holds `*/`.

use super::{JS, span};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::TextEdit;
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Single,
    Multi,
    Doc,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Single => "a `//` comment",
            Kind::Multi => "a `/* */` comment",
            Kind::Doc => "a `/** */` doc comment",
        }
    }
    /// The types a comment of this type is rewritten as, best first.
    fn prefers(self) -> [Kind; 2] {
        match self {
            Kind::Single => [Kind::Multi, Kind::Doc],
            Kind::Multi => [Kind::Single, Kind::Doc],
            Kind::Doc => [Kind::Multi, Kind::Single],
        }
    }
}

fn allowed(ctx: &NativeContext<'_>) -> Vec<Kind> {
    let Some(list) = ctx.options.get("types").and_then(serde_json::Value::as_array) else {
        return vec![Kind::Single, Kind::Doc];
    };
    list.iter()
        .filter_map(serde_json::Value::as_str)
        .filter_map(|s| match s {
            "singleline" => Some(Kind::Single),
            "multiline" => Some(Kind::Multi),
            "doc" | "jsdoc" => Some(Kind::Doc),
            _ => None,
        })
        .collect()
}

fn kind_of(t: &str) -> Kind {
    if t.starts_with("//") {
        Kind::Single
    } else if t.starts_with("/**") && t != "/**/" && !t.starts_with("/***") {
        Kind::Doc
    } else {
        Kind::Multi
    }
}

const DIRECTIVES: &[&str] = &[
    "eslint", "prettier", "istanbul", "c8 ", "v8 ", "jshint", "jscs", "global ", "globals ", "exported ", "tslint", "biome", "oxlint", "codelaw", "noinspection", "vim:", "webpack", "region", "endregion", "@ts-", "@jsx",
    "@flow", "@vitest", "@jest", "@license", "@preserve", "@cc_on",
];

/// A comment tools read: its text starts with a marker or a known directive.
fn directive(t: &str) -> bool {
    let body = if let Some(b) = t.strip_prefix("//") { b } else { t.trim_start_matches("/*").trim_end_matches("*/") };
    let body = body.trim_start();
    body.starts_with(['/', '#', '!']) || (t.starts_with("/*") && !t.starts_with("/**") && body.starts_with('@')) || DIRECTIVES.iter().any(|d| body.starts_with(d))
}

fn comments<'t>(root: Node<'t>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        if x.kind() == "comment" {
            out.push(x);
            continue;
        }
        let mut c = named_children(x);
        c.reverse();
        stack.extend(c);
    }
    out.sort_by_key(|c| c.start_byte());
    out
}

struct Place<'s> {
    indent: &'s str,
    before_blank: bool,
    /// The text after the comment on its last line.
    after: &'s str,
}

fn place(src: &str, start: usize, end: usize) -> Place<'_> {
    let line_start = src[..start].rfind('\n').map_or(0, |i| i + 1);
    let before = &src[line_start..start];
    let line = &src[line_start..];
    let indent_len = line.len() - line.trim_start_matches([' ', '\t']).len();
    let after = src[end..].split('\n').next().unwrap_or("").trim_end_matches('\r');
    Place { indent: &line[..indent_len.min(start - line_start)], before_blank: before.trim().is_empty(), after }
}

/// The lines a block comment says, without its markers and stars.
fn block_lines(t: &str) -> Vec<String> {
    let inner = t.strip_prefix("/**").or_else(|| t.strip_prefix("/*")).unwrap_or(t);
    let inner = inner.strip_suffix("*/").unwrap_or(inner);
    let mut lines: Vec<String> = inner
        .split('\n')
        .enumerate()
        .map(|(i, l)| {
            let l = l.trim_end_matches('\r');
            let l = if i == 0 {
                l.trim()
            } else {
                let l = l.trim_start();
                let l = l.strip_prefix('*').unwrap_or(l);
                l.strip_prefix(' ').unwrap_or(l).trim_end()
            };
            l.trim_end().to_owned()
        })
        .collect();
    while lines.first().is_some_and(String::is_empty) {
        lines.remove(0);
    }
    while lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    lines
}

fn as_single(lines: &[String], indent: &str) -> String {
    if lines.is_empty() {
        return "//".to_owned();
    }
    lines.iter().map(|l| if l.is_empty() { "//".to_owned() } else { format!("// {l}") }).collect::<Vec<_>>().join(&format!("\n{indent}"))
}

fn as_block(lines: &[String], doc: bool, indent: &str) -> Option<String> {
    if lines.iter().any(|l| l.contains("*/")) {
        return None;
    }
    let open = if doc { "/**" } else { "/*" };
    Some(match lines {
        [] => format!("{open} */"),
        [one] => format!("{open} {one} */"),
        many => {
            let body: Vec<String> = many.iter().map(|l| if l.is_empty() { format!("{indent} *") } else { format!("{indent} * {l}") }).collect();
            format!("{open}\n{}\n{indent} */", body.join("\n"))
        }
    })
}

fn line_text(t: &str) -> String {
    let b = &t[2..];
    b.strip_prefix(' ').unwrap_or(b).trim_end().to_owned()
}

fn describe(from: Kind, to: Kind) -> String {
    format!("{} is not an allowed comment type here; write this as {}", from.name(), to.name())
}

pub struct CommentType;

impl NativeRule for CommentType {
    fn id(&self) -> &'static str {
        "js/comment-type"
    }
    fn languages(&self) -> &'static [codelaw_core::Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let allow = allowed(ctx);
        let cs = comments(node.get_inner_node());
        let mut i = 0;
        while i < cs.len() {
            let c = cs[i];
            i += 1;
            let t = text(c, src);
            let kind = kind_of(t);
            if allow.contains(&kind) || directive(t) || c.parent().is_some_and(|p| p.kind() == "jsx_expression") {
                continue;
            }
            let Some(to) = kind.prefers().into_iter().find(|k| allow.contains(k)) else { continue };
            let p = place(src, c.start_byte(), c.end_byte());
            match kind {
                Kind::Single => {
                    // A run of `//` lines, each alone on its line and in one column, is one block.
                    let mut lines = vec![line_text(t)];
                    let mut last = c;
                    if p.before_blank {
                        while let Some(&n) = cs.get(i) {
                            let nt = text(n, src);
                            let np = place(src, n.start_byte(), n.end_byte());
                            if !nt.starts_with("//") || directive(nt) || !np.before_blank || n.start_position().row != last.start_position().row + 1 || n.start_position().column != c.start_position().column {
                                break;
                            }
                            lines.push(line_text(nt));
                            last = n;
                            i += 1;
                        }
                    }
                    let Some(block) = as_block(&lines, to == Kind::Doc, p.indent) else { continue };
                    out.push(span(c.start_byte(), last.end_byte(), describe(kind, to), vec![TextEdit::new(c.start_byte(), last.end_byte(), block)]));
                }
                Kind::Multi | Kind::Doc => {
                    let after_blank = p.after.trim().is_empty();
                    if !p.before_blank && !after_blank {
                        continue;
                    }
                    let (end, tail) = if after_blank { (c.end_byte(), String::new()) } else { (c.end_byte() + (p.after.len() - p.after.trim_start().len()), format!("\n{}", p.indent)) };
                    let new = match to {
                        Kind::Single => format!("{}{tail}", as_single(&block_lines(t), p.indent)),
                        Kind::Multi => format!("/*{}", &t[3..]),
                        Kind::Doc => format!("/**{}", &t[2..]),
                    };
                    out.push(span(c.start_byte(), c.end_byte(), describe(kind, to), vec![TextEdit::new(c.start_byte(), end, new)]));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DetachedFile, NativeContext};
    use codelaw_core::Language;
    use std::collections::BTreeMap;

    fn fixed(src: &str, types: &[&str]) -> String {
        let language = Language::Javascript;
        let root = codelaw_lang::parse(language, src);
        let options: BTreeMap<String, serde_json::Value> = [("types".to_owned(), serde_json::json!(types))].into_iter().collect();
        let data = BTreeMap::new();
        let file = DetachedFile::new("example.js", src);
        let ctx = NativeContext { source: src, language, semantic: None, options: &options, data: &data, file: &file };
        let mut found = Vec::new();
        for node in root.root().dfs() {
            if CommentType.kinds().contains(&node.kind().as_ref()) {
                CommentType.check(node, &ctx, &mut found);
            }
        }
        let mut out = src.to_owned();
        for finding in found.iter().rev() {
            for e in finding.edits.iter().rev() {
                out.replace_range(e.start..e.end, &e.text);
            }
        }
        out
    }

    #[test]
    fn a_block_becomes_line_comments() {
        assert_eq!(fixed("/* a note */\nf();\n", &["singleline", "doc"]), "// a note\nf();\n");
        assert_eq!(fixed("function g() {\n  /*\n   * one\n   * two\n   */\n  f();\n}\n", &["singleline"]), "function g() {\n  // one\n  // two\n  f();\n}\n");
        assert_eq!(fixed("f(); /* tail */\n", &["singleline"]), "f(); // tail\n");
        assert_eq!(fixed("/* head */ f();\n", &["singleline"]), "// head\nf();\n");
    }

    #[test]
    fn line_comments_become_a_block() {
        assert_eq!(fixed("// one\n// two\nf();\n", &["multiline"]), "/*\n * one\n * two\n */\nf();\n");
        assert_eq!(fixed("f(); // tail\n", &["multiline"]), "f(); /* tail */\n");
        assert_eq!(fixed("// a\n// b\n", &["doc"]), "/**\n * a\n * b\n */\n");
    }

    #[test]
    fn a_doc_comment_becomes_a_block() {
        assert_eq!(fixed("/** note */\nf();\n", &["multiline", "singleline"]), "/* note */\nf();\n");
        assert_eq!(fixed("/** note */\nf();\n", &["singleline"]), "// note\nf();\n");
        assert_eq!(fixed("/* note */\nf();\n", &["doc"]), "/** note */\nf();\n");
    }

    #[test]
    fn what_tools_read_is_left_alone() {
        for src in ["/* eslint-disable */\nf();\n", "/*! licence */\nf();\n", "f(/* x */ 1);\n", "const x = /*#__PURE__*/ f();\n"] {
            assert_eq!(fixed(src, &["singleline"]), src, "{src}");
        }
        for src in ["/// <reference path=\"a\" />\n", "// @ts-ignore\nf();\n", "// a */ b\n", "//# sourceMappingURL=a.map\n"] {
            assert_eq!(fixed(src, &["multiline"]), src, "{src}");
        }
        assert_eq!(fixed("/* a */\n", &["singleline", "multiline", "doc"]), "/* a */\n");
        assert_eq!(fixed("/* a */\n", &[]), "/* a */\n");
    }
}

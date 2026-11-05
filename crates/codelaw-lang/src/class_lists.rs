//! Where a file writes class lists (SR-FORMATTER decision 9): the
//! `class`/`className` attributes, and the strings passed to the class
//! functions (`clsx`, `cn`, …) — the places prettier-plugin-tailwindcss
//! sorts, found the way it finds them, so the formatter and the `tailwind`
//! rules look at the same text.
//!
//! - **HTML:** the value of each attribute named in `attributes`.
//! - **JS · JSX · TS · TSX:** a JSX attribute named in `attributes` — its
//!   string, or every string and template literal inside its `{…}`; every
//!   string and template literal in the arguments of a call whose callee
//!   (through member accesses and calls) is an identifier in `functions`;
//!   and a template tagged by one of them.

use crate::{Root, SyntaxNode};
use ast_grep_core::Doc;
use codelaw_core::Language;
use codelaw_core::class_table::SortOptions;

/// Where class lists are looked for.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassListOptions {
    pub attributes: Vec<String>,
    pub functions: Vec<String>,
}

impl Default for ClassListOptions {
    fn default() -> Self {
        ClassListOptions {
            attributes: vec!["class".into(), "className".into()],
            functions: vec!["clsx".into(), "cn".into(), "cva".into(), "tw".into()],
        }
    }
}

/// One class list: the bytes `start..end` of the file, and how the sort
/// treats its ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassList {
    pub start: usize,
    pub end: usize,
    pub sort: SortOptions,
}

/// Every class list of a parsed file, in source order, each once.
pub fn class_lists(language: Language, root: &Root, options: &ClassListOptions) -> Vec<ClassList> {
    class_lists_in(language, &root.root(), options)
}

/// [`class_lists`] under one node: the file's root node, for a native rule.
pub fn class_lists_in(language: Language, top: &SyntaxNode<'_>, options: &ClassListOptions) -> Vec<ClassList> {
    let mut out = Vec::new();
    match language {
        Language::Html => html(top, options, &mut out),
        Language::Javascript | Language::Jsx | Language::Typescript | Language::Tsx => js(top, options, &mut out),
        _ => {}
    }
    out.sort_by_key(|l| (l.start, l.end));
    out.dedup_by_key(|l| (l.start, l.end));
    out
}

fn html(top: &SyntaxNode<'_>, options: &ClassListOptions, out: &mut Vec<ClassList>) {
    for node in top.dfs() {
        if node.kind() != "attribute" {
            continue;
        }
        let Some(name) = node.children().find(|c| c.kind() == "attribute_name") else { continue };
        if !options.attributes.iter().any(|a| *a == name.text()) {
            continue;
        }
        let value = node.children().find_map(|c| match c.kind().as_ref() {
            "attribute_value" => Some(c),
            "quoted_attribute_value" => c.children().find(|v| v.kind() == "attribute_value"),
            _ => None,
        });
        if let Some(v) = value {
            let r = v.range();
            out.push(ClassList { start: r.start, end: r.end, sort: SortOptions::default() });
        }
    }
}

fn js(top: &SyntaxNode<'_>, options: &ClassListOptions, out: &mut Vec<ClassList>) {
    for node in top.dfs() {
        match node.kind().as_ref() {
            "jsx_attribute" => {
                let mut children = node.children().filter(|c| c.is_named());
                let Some(name) = children.next() else { continue };
                if name.kind() != "property_identifier" || !options.attributes.iter().any(|a| *a == name.text()) {
                    continue;
                }
                match children.next() {
                    Some(v) if v.kind() == "string" => string(&v, SortOptions::default(), out),
                    Some(v) if v.kind() == "jsx_expression" => inside(&v, out),
                    _ => {}
                }
            }
            "call_expression" => {
                let (Some(callee), Some(args)) = (node.field("function"), node.field("arguments")) else { continue };
                if !sortable(&callee, options) {
                    continue;
                }
                if args.kind() == "template_string" {
                    let (start, end) = collapsible(&args, None);
                    template(&args, start, end, out);
                } else if args.children().any(|a| a.is_named() && a.kind() != "comment") {
                    for a in args.children().filter(|a| a.is_named()) {
                        inside(&a, out);
                    }
                }
            }
            _ => {}
        }
    }
}

/// prettier-plugin-tailwindcss's `isSortableExpression`: the callee, through
/// calls and member accesses, is a named function.
fn sortable(callee: &SyntaxNode<'_>, options: &ClassListOptions) -> bool {
    let mut n = callee.clone();
    loop {
        match n.kind().as_ref() {
            "call_expression" => match n.field("function") {
                Some(f) => n = f,
                None => return false,
            },
            "member_expression" => match n.field("object") {
                Some(o) => n = o,
                None => return false,
            },
            "identifier" => return options.functions.iter().any(|f| *f == n.text()),
            _ => return false,
        }
    }
}

/// Every string and template literal in `subtree`, itself included.
fn inside(subtree: &SyntaxNode<'_>, out: &mut Vec<ClassList>) {
    for d in subtree.dfs() {
        match d.kind().as_ref() {
            "string" => {
                let (start, end) = collapsible(&d, Some(subtree));
                string(&d, SortOptions { collapse_start: start, collapse_end: end, ..SortOptions::default() }, out);
            }
            "template_string" => {
                let (start, end) = collapsible(&d, Some(subtree));
                template(&d, start, end, out);
            }
            _ => {}
        }
    }
}

fn string(node: &SyntaxNode<'_>, sort: SortOptions, out: &mut Vec<ClassList>) {
    let r = node.range();
    if r.end >= r.start + 2 {
        out.push(ClassList { start: r.start + 1, end: r.end - 1, sort });
    }
}

/// The parts of a template literal between its `${…}`, as byte ranges.
fn quasis(node: &SyntaxNode<'_>) -> (Vec<(usize, usize)>, usize) {
    let r = node.range();
    if r.end < r.start + 2 {
        return (Vec::new(), 0);
    }
    let mut parts = Vec::new();
    let mut at = r.start + 1;
    let mut expressions = 0;
    for c in node.children().filter(|c| c.kind() == "template_substitution") {
        let s = c.range();
        parts.push((at, s.start));
        at = s.end;
        expressions += 1;
    }
    parts.push((at, r.end - 1));
    (parts, expressions)
}

fn template(node: &SyntaxNode<'_>, collapse_start: bool, collapse_end: bool, out: &mut Vec<ClassList>) {
    let text = node.get_doc().get_source();
    let (parts, expressions) = quasis(node);
    for (i, &(a, b)) in parts.iter().enumerate() {
        let raw = &text[a..b];
        out.push(ClassList {
            start: a,
            end: b,
            sort: SortOptions {
                ignore_first: i > 0 && !raw.starts_with(char::is_whitespace),
                ignore_last: i < expressions && !raw.ends_with(char::is_whitespace),
                collapse: true,
                collapse_start: collapse_start && i == 0,
                collapse_end: collapse_end && i >= expressions,
                remove_duplicates: true,
            },
        });
    }
}

/// prettier-plugin-tailwindcss's `canCollapseWhitespaceIn`: whether the
/// list's start and end may lose their whitespace, given what it is joined to
/// by `+` or inside a template, up to `stop` (the subtree the search started
/// from; `None`: the file).
fn collapsible(node: &SyntaxNode<'_>, stop: Option<&SyntaxNode<'_>>) -> (bool, bool) {
    let source = node.get_doc().get_source();
    let (mut start, mut end) = (true, true);
    let mut child = node.clone();
    loop {
        if stop.is_some_and(|s| s.range() == child.range() && s.kind() == child.kind()) {
            break;
        }
        let Some(parent) = child.parent() else { break };
        match parent.kind().as_ref() {
            "binary_expression" if parent.field("operator").is_some_and(|o| o.text() == "+") => {
                let is = |f: &str| parent.field(f).is_some_and(|n| n.range() == child.range());
                start = start && !is("right");
                end = end && !is("left");
            }
            "template_substitution" => {
                if let Some(tpl) = parent.parent().filter(|t| t.kind() == "template_string") {
                    let (cs, ce) = (child.range().start as i64, child.range().end as i64);
                    for (a, b) in quasis(&tpl).0 {
                        let raw = &source[a..b];
                        if cs - b as i64 <= 2 {
                            start = start && raw.starts_with(char::is_whitespace);
                        }
                        if ce - a as i64 <= 2 {
                            end = end && raw.ends_with(char::is_whitespace);
                        }
                    }
                }
            }
            _ => {}
        }
        child = parent;
    }
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    fn lists(language: Language, src: &str) -> Vec<String> {
        let root = parse(language, src);
        class_lists(language, &root, &ClassListOptions::default()).iter().map(|l| src[l.start..l.end].to_owned()).collect()
    }

    #[test]
    fn jsx_attributes_and_their_expressions() {
        let src = r#"const a = <div className="p-4 flex" title="x y" class={cond ? "a b" : `c ${d} e`} />;"#;
        assert_eq!(lists(Language::Tsx, src), ["p-4 flex", "a b", "c ", " e"]);
    }

    #[test]
    fn class_functions_through_members_and_calls() {
        let src = r#"cn("a b", { "c d": x }); other("no"); tw.div`e f`; cva("g")("h"); clsx();"#;
        assert_eq!(lists(Language::Typescript, src), ["a b", "c d", "e f", "g", "h"]);
    }

    #[test]
    fn html_class_attributes() {
        let src = "<div class=\"p-4 flex\" id=\"a b\"><span class=plain></span></div>\n";
        assert_eq!(lists(Language::Html, src), ["p-4 flex", "plain"]);
    }

    #[test]
    fn concatenation_keeps_the_joined_ends() {
        let src = r#"cn("a b " + x + " c d")"#;
        let root = parse(Language::Javascript, src);
        let l = class_lists(Language::Javascript, &root, &ClassListOptions::default());
        assert_eq!((l[0].sort.collapse_start, l[0].sort.collapse_end), (true, false));
        assert_eq!((l[1].sort.collapse_start, l[1].sort.collapse_end), (false, true));
    }
}

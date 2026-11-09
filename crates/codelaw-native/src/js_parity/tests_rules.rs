//! W-139 strand W-139-js: eslint-plugin-sonarjs 4.2.2's
//! `no-duplicate-parameterized-test-case` (behaviour only: SonarJS is
//! source-available, nothing here is copied), written again.

use super::{JS, finding};
use crate::js_ast::{arguments, is_function};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

/// The code tokens of a node, comments left out: two nodes with the same
/// tokens are the same code.
fn tokens<'s>(n: Node<'_>, src: &'s str, out: &mut Vec<&'s str>) {
    let kids = children(n);
    if kids.is_empty() {
        if n.kind() != "comment" {
            out.push(text(n, src));
        }
        return;
    }
    for c in kids {
        tokens(c, src, out);
    }
}

fn has_comment(n: Node<'_>) -> bool {
    n.kind() == "comment" || children(n).iter().any(|c| has_comment(*c))
}

/// `it.each([…])`, `test.only.each([…])`, `describe.skip.each([…])`: the
/// table.
fn table<'t>(call: Node<'t>, src: &str) -> Option<Node<'t>> {
    let inner = call.child_by_field_name("function").filter(|f| f.kind() == "call_expression")?;
    let f = inner.child_by_field_name("function").filter(|f| f.kind() == "member_expression")?;
    if f.child_by_field_name("property").is_none_or(|p| text(p, src) != "each") {
        return None;
    }
    // The object: `it`, `test`, `describe`, optionally with `.only` / `.skip` / `.concurrent`.
    let mut obj = f.child_by_field_name("object")?;
    while obj.kind() == "member_expression" && obj.child_by_field_name("property").is_some_and(|p| matches!(text(p, src), "only" | "skip" | "concurrent" | "sequential" | "fails")) {
        obj = obj.child_by_field_name("object")?;
    }
    if obj.kind() != "identifier" || !matches!(text(obj, src), "it" | "test" | "describe") {
        return None;
    }
    let first = arguments(inner).into_iter().next()?;
    let first = crate::js_ast::unparen(first);
    (first.kind() == "array").then_some(first)
}

/// The edit that removes elements `first..=last` of an array, with the
/// comma that joined them to the row before, or their lines.
fn remove_rows(first: Node<'_>, last: Node<'_>, src: &str) -> TextEdit {
    let line_start = src[..first.start_byte()].rfind('\n').map_or(0, |i| i + 1);
    let alone_before = src[line_start..first.start_byte()].trim().is_empty() && line_start > 0;
    let after = &src[last.end_byte()..];
    let tail = after.trim_start_matches([' ', '\t']);
    let tail = tail.strip_prefix(',').unwrap_or(tail).trim_start_matches([' ', '\t']);
    if alone_before && tail.starts_with('\n') || alone_before && tail.starts_with("\r\n") {
        let consumed = after.len() - tail.len() + if tail.starts_with("\r\n") { 2 } else { 1 };
        return TextEdit::delete(line_start, last.end_byte() + consumed);
    }
    let before = src[..first.start_byte()].trim_end();
    if before.ends_with(',') {
        return TextEdit::delete(before.len() - 1, last.end_byte());
    }
    TextEdit::delete(first.start_byte(), last.end_byte())
}

pub struct NoDuplicateParameterizedTestCase;
impl NativeRule for NoDuplicateParameterizedTestCase {
    fn id(&self) -> &'static str {
        "js/no-duplicate-parameterized-test-case"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let call = node.get_inner_node();
        let src = ctx.source;
        let Some(rows) = table(call, src) else { return };
        let args = arguments(call);
        if args.len() < 2 || !args.iter().any(|a| is_function(crate::js_ast::unparen(*a))) {
            return;
        }
        let items: Vec<Node<'_>> = named_children(rows).into_iter().filter(|c| c.kind() != "comment").collect();
        let mut groups: Vec<(Vec<&str>, Vec<usize>)> = Vec::new();
        for (i, it) in items.iter().enumerate() {
            if it.kind() == "spread_element" {
                continue;
            }
            let mut t = Vec::new();
            tokens(*it, src, &mut t);
            match groups.iter_mut().find(|(k, _)| *k == t) {
                Some((_, dups)) => dups.push(i),
                None => groups.push((t, vec![i])),
            }
        }
        for (_, idxs) in groups.iter().filter(|(_, v)| v.len() > 1) {
            let dups = &idxs[1..];
            let mut edits = Vec::new();
            let commented = dups.iter().any(|i| has_comment(items[*i]));
            if !commented {
                let mut k = 0;
                while k < dups.len() {
                    let first = dups[k];
                    let mut last = first;
                    k += 1;
                    while k < dups.len() && dups[k] == last + 1 {
                        last = dups[k];
                        k += 1;
                    }
                    edits.push(remove_rows(items[first], items[last], src));
                }
            }
            out.push(finding(items[dups[0]], "This test case repeats an earlier one: it checks nothing new", edits));
        }
    }
}

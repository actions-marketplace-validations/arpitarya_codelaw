//! W-139 strand W-139-js, native rows that compare or rewrite code a pattern
//! can't: whole branches against each other, a chain of operands, members
//! against an order, statements re-indented into a new block. Upstream
//! behaviour read from eslint-plugin-unicorn v79.0.0's, ESLint 10.13.0's and
//! eslint-plugin-sonarjs 4.2.2's source and tests (SonarJS: behaviour only).

use super::scope::{ancestors, fn_like};
use super::{JS, finding, span};
use crate::js_ast::{arguments, unparen};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule, as_suggestion};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tree_sitter::Node;

macro_rules! rule {
    ($name:ident, $id:literal, $kinds:expr, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                JS
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}

fn has_comment(n: Node<'_>) -> bool {
    n.kind() == "comment" || children(n).iter().any(|c| has_comment(*c))
}

fn indent_of(src: &str, at: usize) -> &str {
    let ls = src[..at].rfind('\n').map_or(0, |i| i + 1);
    let line = &src[ls..];
    &line[..line.len() - line.trim_start_matches([' ', '\t']).len()]
}

fn unit(src: &str) -> &'static str {
    if src.lines().any(|l| l.starts_with('\t')) { "\t" } else { "  " }
}

fn stmts(block: Node<'_>) -> Vec<Node<'_>> {
    named_children(block).into_iter().filter(|c| c.kind() != "comment" && c.kind() != "empty_statement").collect()
}

/// The node's leaf tokens, comments left out: structural equality that
/// ignores layout.
fn tokens<'s>(n: Node<'_>, src: &'s str, out: &mut Vec<&'s str>) {
    if n.kind() == "comment" {
        return;
    }
    if n.child_count() == 0 {
        out.push(text(n, src));
        return;
    }
    for c in children(n) {
        tokens(c, src, out);
    }
}

fn same_code(a: Node<'_>, b: Node<'_>, src: &str) -> bool {
    let (mut x, mut y) = (Vec::new(), Vec::new());
    tokens(a, src, &mut x);
    tokens(b, src, &mut y);
    x == y
}

fn multiline_literal(n: Node<'_>) -> bool {
    (matches!(n.kind(), "template_string" | "string" | "jsx_text") && n.start_position().row != n.end_position().row) || children(n).iter().any(|c| multiline_literal(*c))
}

/// Text from `start` to `end`, each line after the first moved by `delta`
/// columns (positive: more indented).
fn reindent(src: &str, start: usize, end: usize, from: &str, to: &str) -> String {
    let body = &src[start..end];
    let mut out = String::new();
    for (i, line) in body.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
            let l = line.strip_prefix(from).unwrap_or(line.trim_start_matches([' ', '\t']));
            if !l.is_empty() {
                out.push_str(to);
            }
            out.push_str(l);
        } else {
            out.push_str(line);
        }
    }
    out
}

// ---- unicorn: consistent-arrow-return-style --------------------------------

rule!(ConsistentArrowReturnStyle, "js/consistent-arrow-return-style", &["arrow_function"], |n, ctx, out| {
    let src = ctx.source;
    let Some(body) = n.child_by_field_name("body") else { return };
    let after = src[n.end_byte()..].trim_start();
    if body.kind() != "statement_block" {
        if body.start_position().row == body.end_position().row {
            return;
        }
        let mut edits = Vec::new();
        if !has_comment(body) && !multiline_literal(body) {
            let ind = indent_of(src, n.start_byte());
            let u = unit(src);
            let inner = if body.start_position().row == n.start_position().row {
                reindent(src, body.start_byte(), body.end_byte(), ind, &format!("{ind}{u}"))
            } else {
                let bind = indent_of(src, body.start_byte()).to_owned();
                reindent(src, body.start_byte(), body.end_byte(), &bind, &format!("{ind}{u}"))
            };
            let arrow = children(n).into_iter().find(|c| c.kind() == "=>");
            if let Some(a) = arrow {
                edits.push(TextEdit::new(a.end_byte(), body.end_byte(), format!(" {{\n{ind}{u}return {inner};\n{ind}}}")));
            }
        }
        out.push(finding(body, "This arrow's body spans lines: write it as a block with `return`", edits));
        return;
    }
    let ss = named_children(body);
    let [ret] = ss[..] else { return };
    if ret.kind() != "return_statement" {
        return;
    }
    let Some(arg) = ret.named_child(0) else { return };
    if arg.start_position().row != arg.end_position().row || body.start_position().row == body.end_position().row && false {
        return;
    }
    let mut edits = Vec::new();
    let risky = after.starts_with(['(', '[', '/', '`', '+', '-', '*', '.', '<']);
    if !has_comment(body) && !risky {
        let t = text(arg, src);
        let wrap = t.starts_with('{') || arg.kind() == "sequence_expression" || arg.kind() == "object";
        edits.push(TextEdit::new(body.start_byte(), body.end_byte(), if wrap { format!("({t})") } else { t.to_owned() }));
    }
    out.push(finding(body, "This block only returns a one-line value: return it directly, `=> value`", edits));
});

// ---- unicorn: no-duplicate-loops -------------------------------------------

fn iterator_source(n: Node<'_>, src: &str, depth: usize) -> bool {
    let n = unparen(n);
    if depth > 6 || n.kind() != "call_expression" {
        return false;
    }
    let Some(f) = n.child_by_field_name("function") else { return false };
    let t = text(f, src);
    if matches!(t, "Iterator.from" | "Iterator.concat" | "Iterator.zip" | "Iterator.zipKeyed" | "globalThis.Iterator.from") {
        return true;
    }
    if f.kind() != "member_expression" {
        return false;
    }
    let p = f.child_by_field_name("property").map_or("", |p| text(p, src));
    let o = f.child_by_field_name("object");
    match p {
        "entries" | "keys" | "values" => arguments(n).is_empty(),
        "matchAll" => true,
        "map" | "filter" | "flatMap" | "take" | "drop" => o.is_some_and(|o| iterator_source(o, src, depth + 1)),
        _ => false,
    }
}

rule!(NoDuplicateLoops, "js/no-duplicate-loops", &["for_in_statement"], |n, ctx, out| {
    let src = ctx.source;
    if !children(n).iter().any(|c| c.kind() == "of") {
        return;
    }
    let Some(right) = n.child_by_field_name("right").map(unparen) else { return };
    if right.kind() != "call_expression" || arguments(right).is_empty() {
        return;
    }
    let Some(f) = right.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && f.child_by_field_name("optional_chain").is_none()) else { return };
    let Some(p) = f.child_by_field_name("property").filter(|p| matches!(text(*p, src), "map" | "filter")) else { return };
    if f.child_by_field_name("object").is_some_and(|o| iterator_source(o, src, 0)) {
        return;
    }
    let m = text(p, src);
    out.push(finding(p, format!("`.{m}()` builds an array only for this loop to go over it again: do the `{m}` work inside the loop"), vec![]));
});

// ---- unicorn: no-redundant-comparison ---------------------------------------

fn reference(n: Node<'_>) -> bool {
    match n.kind() {
        "identifier" | "this" | "super" => true,
        "member_expression" => n.child_by_field_name("optional_chain").is_none() && n.child_by_field_name("object").is_some_and(reference),
        _ => false,
    }
}

fn number(n: Node<'_>, src: &str) -> Option<f64> {
    let n = unparen(n);
    match n.kind() {
        "number" => text(n, src).replace('_', "").parse().ok(),
        "unary_expression" if text(n, src).starts_with('-') => n.child_by_field_name("argument").and_then(|a| number(a, src)).map(|v| -v),
        _ => None,
    }
}

fn static_text(n: Node<'_>, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "string" | "number" | "true" | "false" | "null" => Some(text(n, src).to_owned()),
        "unary_expression" if number(n, src).is_some() => Some(text(n, src).replace(' ', "")),
        _ => None,
    }
}

fn flatten_and<'t>(n: Node<'t>, src: &str, out: &mut Vec<Node<'t>>) {
    if n.kind() == "binary_expression" && n.child_by_field_name("operator").is_some_and(|o| text(o, src) == "&&") {
        if let (Some(l), Some(r)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) {
            flatten_and(l, src, out);
            flatten_and(r, src, out);
        }
    } else {
        out.push(n);
    }
}

fn effects(n: Node<'_>) -> bool {
    matches!(n.kind(), "call_expression" | "assignment_expression" | "augmented_assignment_expression" | "update_expression" | "new_expression") || (n.kind() == "unary_expression" && n.child(0).is_some_and(|c| c.kind() == "delete")) || named_children(n).iter().any(|c| !fn_like(c.kind()) && effects(*c))
}

#[derive(Clone)]
struct Cmp {
    lhs: String,
    op: String,
    rhs: String,
    plain: bool,
}

fn comparison(n: Node<'_>, src: &str) -> Option<Cmp> {
    let n = unparen(n);
    if n.kind() != "binary_expression" {
        return None;
    }
    let op = text(n.child_by_field_name("operator")?, src);
    if !matches!(op, "<" | "<=" | ">" | ">=" | "===" | "!==") {
        return None;
    }
    let (l, r) = (unparen(n.child_by_field_name("left")?), unparen(n.child_by_field_name("right")?));
    let flip = |o: &str| match o {
        "<" => ">",
        "<=" => ">=",
        ">" => "<",
        ">=" => "<=",
        o => o,
    }
    .to_owned();
    if reference(l) {
        Some(Cmp { lhs: text(l, src).to_owned(), op: op.to_owned(), rhs: text(r, src).to_owned(), plain: l.kind() == "identifier" && (r.kind() == "identifier" || static_text(r, src).is_some()) })
    } else if reference(r) {
        Some(Cmp { lhs: text(r, src).to_owned(), op: flip(op), rhs: text(l, src).to_owned(), plain: r.kind() == "identifier" && static_text(l, src).is_some() })
    } else {
        None
    }
}

/// Whether `a` (holding) makes `b` always hold, both ordering comparisons
/// of the same thing against numbers.
fn implies(a: &Cmp, b: &Cmp) -> bool {
    let (Ok(x), Ok(y)) = (a.rhs.parse::<f64>(), b.rhs.parse::<f64>()) else { return false };
    match (a.op.as_str(), b.op.as_str()) {
        (">", ">") | (">=", ">=") => x >= y,
        (">", ">=") => x >= y,
        (">=", ">") => x > y,
        ("<", "<") | ("<=", "<=") => x <= y,
        ("<", "<=") => x <= y,
        ("<=", "<") => x < y,
        _ => false,
    }
}

rule!(NoRedundantComparison, "js/no-redundant-comparison", &["binary_expression"], |n, ctx, out| {
    let src = ctx.source;
    if !n.child_by_field_name("operator").is_some_and(|o| text(o, src) == "&&") || n.parent().is_some_and(|p| p.kind() == "binary_expression" && p.child_by_field_name("operator").is_some_and(|o| text(o, src) == "&&")) {
        return;
    }
    let mut ops = Vec::new();
    flatten_and(n, src, &mut ops);
    let cmps: Vec<Option<Cmp>> = ops.iter().map(|o| comparison(*o, src)).collect();
    let (Some(first), Some(last)) = (cmps.iter().position(Option::is_some), cmps.iter().rposition(Option::is_some)) else { return };
    if ops[first..=last].iter().any(|o| effects(*o)) {
        return;
    }
    // Classes of references known equal (`a === b`).
    let mut class: HashMap<String, String> = HashMap::new();
    fn find(c: &HashMap<String, String>, x: &str) -> String {
        let mut cur = x.to_owned();
        while let Some(p) = c.get(&cur).filter(|p| **p != cur) {
            cur = p.clone();
        }
        cur
    }
    for c in cmps.iter().flatten() {
        if c.op == "===" && ops.iter().any(|_| true) && !c.rhs.is_empty() && static_text_str(&c.rhs).is_none() {
            let (a, b) = (find(&class, &c.lhs), find(&class, &c.rhs));
            if a != b {
                class.insert(a, b);
            }
        }
    }
    let mut seen: Vec<(usize, Cmp)> = Vec::new();
    for (i, c) in cmps.iter().enumerate() {
        let Some(c) = c else { continue };
        if c.op == "===" && static_text_str(&c.rhs).is_none() {
            continue;
        }
        let key = find(&class, &c.lhs);
        let redundant = seen.iter().any(|(_, p)| find(&class, &p.lhs) == key && ((p.op == c.op && p.rhs == c.rhs) || implies(p, c)))
            || (c.op == "!==" && seen.iter().any(|(_, p)| p.op == "===" && p.rhs == c.rhs && cmps.iter().flatten().any(|q| q.op == "!==" && ((q.lhs == p.lhs && q.rhs == c.lhs) || (q.rhs == p.lhs && q.lhs == c.lhs)))));
        if redundant {
            let at = ops[i];
            let mut edits = Vec::new();
            if !has_comment(n) {
                let (s, e) = if i == 0 { (at.start_byte(), ops[1].start_byte()) } else { (ops[i - 1].end_byte(), at.end_byte()) };
                edits.push(TextEdit::delete(s, e));
                if !c.plain || !matches!(c.op.as_str(), "===" | "!==") {
                    edits = as_suggestion(edits);
                }
            }
            out.push(finding(at, format!("`{}` is already decided by the comparisons before it: remove it", text(at, src)), edits));
            return;
        }
        // An implied earlier comparison is the redundant one.
        if let Some((j, _)) = seen.iter().find(|(_, p)| find(&class, &p.lhs) == key && implies(c, p)) {
            let at = ops[*j];
            out.push(finding(at, format!("`{}` is implied by `{}` later in the same condition: remove it", text(at, src), text(ops[i], src)), vec![]));
            return;
        }
        seen.push((i, c.clone()));
    }
});

fn static_text_str(s: &str) -> Option<()> {
    let t = s.trim();
    (t.starts_with(['"', '\'']) || t.parse::<f64>().is_ok() || matches!(t, "true" | "false" | "null") || (t.starts_with('-') && t[1..].trim().parse::<f64>().is_ok())).then_some(())
}

// ---- unicorn: prefer-continue / prefer-early-return ----------------------------

fn ends_in_exit(block: Node<'_>) -> bool {
    stmts(block).last().is_some_and(|s| matches!(s.kind(), "return_statement" | "break_statement" | "continue_statement" | "throw_statement"))
}

fn negate(cond: Node<'_>, src: &str) -> String {
    let c = unparen(cond);
    let t = text(c, src);
    if c.kind() == "unary_expression" && c.child_by_field_name("operator").is_some_and(|o| text(o, src) == "!") {
        let a = c.child_by_field_name("argument").map(unparen).unwrap_or(c);
        return text(a, src).to_owned();
    }
    if c.kind() == "binary_expression"
        && let Some(op) = c.child_by_field_name("operator")
    {
        let flipped = match text(op, src) {
            "===" => Some("!=="),
            "!==" => Some("==="),
            "==" => Some("!="),
            "!=" => Some("=="),
            _ => None,
        };
        if let Some(f) = flipped {
            return format!("{}{f}{}", &src[c.start_byte()..op.start_byte()], &src[op.end_byte()..c.end_byte()]);
        }
    }
    if matches!(c.kind(), "identifier" | "member_expression" | "call_expression" | "subscript_expression" | "this") { format!("!{t}") } else { format!("!({t})") }
}

/// The guard rewrite of a final `if` with no `else`: `if (!cond) { <exit>; }`
/// then the `if`'s statements, one level out.
fn guard_edit(stmt: Node<'_>, exit: &str, src: &str) -> Option<TextEdit> {
    let cond = stmt.child_by_field_name("condition")?;
    let block = stmt.child_by_field_name("consequence").filter(|b| b.kind() == "statement_block")?;
    let inner = stmts(block);
    if has_comment(block) || multiline_literal(block) || inner.iter().any(|s| matches!(s.kind(), "function_declaration" | "class_declaration" | "lexical_declaration" | "generator_function_declaration")) {
        return None;
    }
    let (first, last) = (inner.first()?, inner.last()?);
    let ind = indent_of(src, stmt.start_byte()).to_owned();
    let u = unit(src);
    let inner_ind = indent_of(src, first.start_byte()).to_owned();
    let body = if first.start_position().row == stmt.start_position().row { text_between(src, first.start_byte(), last.end_byte()) } else { reindent(src, first.start_byte(), last.end_byte(), &inner_ind, &ind) };
    let c = negate(cond, src);
    let c = c.trim_start_matches('(').trim_end_matches(')');
    let c = if negate(cond, src).starts_with('(') { negate(cond, src) } else { c.to_owned() };
    Some(TextEdit::new(stmt.start_byte(), stmt.end_byte(), format!("if ({c}) {{\n{ind}{u}{exit};\n{ind}}}\n\n{ind}{body}")))
}

fn text_between(src: &str, a: usize, b: usize) -> String {
    src[a..b].to_owned()
}

fn max_statements(o: &BTreeMap<String, Value>) -> usize {
    o.get("maximumStatements").and_then(Value::as_u64).map_or(1, |v| v as usize)
}

rule!(PreferContinue, "js/prefer-continue", &["for_statement", "for_in_statement", "while_statement", "do_statement"], |n, ctx, out| {
    let src = ctx.source;
    let Some(body) = n.child_by_field_name("body").filter(|b| b.kind() == "statement_block") else { return };
    let ss = stmts(body);
    let Some(last) = ss.last().copied().filter(|s| s.kind() == "if_statement" && s.child_by_field_name("alternative").is_none()) else { return };
    let Some(cons) = last.child_by_field_name("consequence").filter(|b| b.kind() == "statement_block") else { return };
    if stmts(cons).len() <= max_statements(ctx.options) || ends_in_exit(cons) {
        return;
    }
    let preceding_lexical = ss.len() > 1 && stmts(cons).iter().any(|s| s.kind() == "lexical_declaration");
    let trailing_comment = last.next_sibling().is_some_and(|c| c.kind() == "comment");
    let edits = if preceding_lexical || trailing_comment || text(body, src).contains("eval(") { vec![] } else { guard_edit(last, "continue", src).into_iter().collect() };
    let k = last.child(0).unwrap_or(last);
    out.push(span(k.start_byte(), k.end_byte(), "The loop's last statement is an `if` wrapping its work: invert it into a `continue` guard", edits));
});

rule!(PreferEarlyReturn, "js/prefer-early-return", &["function_declaration", "function_expression", "function", "arrow_function"], |n, ctx, out| {
    let src = ctx.source;
    let Some(body) = n.child_by_field_name("body").filter(|b| b.kind() == "statement_block") else { return };
    let ss = stmts(body);
    let Some(last) = ss.last().copied().filter(|s| s.kind() == "if_statement" && s.child_by_field_name("alternative").is_none()) else { return };
    let Some(cons) = last.child_by_field_name("consequence").filter(|b| b.kind() == "statement_block") else { return };
    if stmts(cons).len() <= max_statements(ctx.options) {
        return;
    }
    let preceding_lexical = ss.len() > 1 && stmts(cons).iter().any(|s| s.kind() == "lexical_declaration");
    let trailing_comment = last.next_sibling().is_some_and(|c| c.kind() == "comment");
    let mut edits = if preceding_lexical || text(body, src).contains("eval(") { vec![] } else { guard_edit(last, "return", src).into_iter().collect() };
    if trailing_comment {
        edits = as_suggestion(edits);
    }
    let k = last.child(0).unwrap_or(last);
    out.push(span(k.start_byte(), k.end_byte(), "The function's last statement is an `if` wrapping its work: invert it into an early `return`", edits));
});

// ---- unicorn: prefer-single-replace ---------------------------------------------

/// One link of the chain: its receiver, the character it replaces, and the
/// replacement's text.
fn replace_link<'t>(call: Node<'t>, src: &str) -> Option<(Node<'t>, char, String)> {
    if call.kind() != "call_expression" {
        return None;
    }
    let f = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && f.child_by_field_name("optional_chain").is_none())?;
    let method = text(f.child_by_field_name("property")?, src);
    let args = arguments(call);
    let [search, repl] = args[..] else { return None };
    if repl.kind() != "string" {
        return None;
    }
    let ch = match search.kind() {
        "string" if method == "replaceAll" => {
            let v = crate::util::unquote(text(search, src));
            let mut cs = v.chars();
            let c = cs.next()?;
            (cs.next().is_none() && c.len_utf16() == 1 && c != '\\').then_some(c)?
        }
        "regex" if matches!(method, "replace" | "replaceAll") => {
            let flags = search.child_by_field_name("flags").map_or("", |f| text(f, src));
            if !flags.contains('g') || flags.chars().any(|c| !matches!(c, 'g' | 'm' | 's')) {
                return None;
            }
            let pat = text(search.child_by_field_name("pattern")?, src);
            let inner = pat.strip_prefix('[').and_then(|p| p.strip_suffix(']')).filter(|p| !p.starts_with('^')).unwrap_or(pat);
            let mut cs = inner.chars();
            match (cs.next()?, cs.next(), cs.next()) {
                ('\\', Some(c), None) if !c.is_ascii_alphanumeric() => c,
                (c, None, None) if !".*+?()[]{}|^$\\/".contains(c) => c,
                _ => return None,
            }
        }
        _ => return None,
    };
    Some((f.child_by_field_name("object")?, ch, text(repl, src).to_owned()))
}

fn class_escape(c: char) -> String {
    match c {
        '\\' | ']' | '^' | '-' | '/' => format!("\\{c}"),
        '\n' => "\\n".into(),
        '\r' => "\\r".into(),
        '\t' => "\\t".into(),
        c if c.is_control() => format!("\\x{:02X}", c as u32),
        c => c.to_string(),
    }
}

rule!(PreferSingleReplace, "js/prefer-single-replace", &["call_expression"], |n, ctx, out| {
    let src = ctx.source;
    let Some((_, _, repl)) = replace_link(n, src) else { return };
    // Only the outermost call of a chain.
    if let Some(p) = n.parent().filter(|p| p.kind() == "member_expression").and_then(|m| m.parent()).filter(|c| c.kind() == "call_expression")
        && replace_link(p, src).is_some_and(|(_, _, r)| r == repl)
    {
        return;
    }
    let mut chars = Vec::new();
    let mut base = n;
    let mut cur = n;
    while let Some((recv, ch, r)) = replace_link(cur, src) {
        if r != repl {
            break;
        }
        chars.push(ch);
        base = recv;
        cur = unparen(recv);
    }
    if chars.len() < 2 {
        return;
    }
    let value = crate::util::unquote(&repl);
    let unique: BTreeSet<char> = chars.iter().copied().collect();
    if value.contains("$`") || value.contains("$'") || unique.iter().any(|c| value.contains(*c)) || (value.contains("$&") && unique.len() != chars.len()) {
        return;
    }
    chars.reverse();
    let mut ordered = Vec::new();
    for c in chars {
        if !ordered.contains(&c) {
            ordered.push(c);
        }
    }
    let class: String = ordered.iter().map(|c| class_escape(*c)).collect();
    let edits = if has_comment(n) { vec![] } else { vec![TextEdit::new(n.start_byte(), n.end_byte(), format!("{}.replaceAll(/[{class}]/g, {repl})", text(base, src)))] };
    out.push(finding(n, format!("These replacements all write {repl}: do them in one `replaceAll()` with a character class"), edits));
});

// ---- unicorn: prefer-hoisting-branch-code ------------------------------------------

fn chain_branches(head: Node<'_>) -> Option<Vec<Node<'_>>> {
    let mut out = Vec::new();
    let mut cur = head;
    loop {
        out.push(cur.child_by_field_name("consequence").filter(|b| b.kind() == "statement_block")?);
        let alt = cur.child_by_field_name("alternative")?;
        let next = named_children(alt).into_iter().find(|c| c.kind() != "comment")?;
        match next.kind() {
            "if_statement" => cur = next,
            "statement_block" => {
                out.push(next);
                return Some(out);
            }
            _ => return None,
        }
    }
}

fn effect_free(n: Node<'_>) -> bool {
    !effects(n) && !named_children(n).iter().any(|c| matches!(c.kind(), "member_expression" | "subscript_expression"))
}

rule!(PreferHoistingBranchCode, "js/prefer-hoisting-branch-code", &["if_statement"], |n, ctx, out| {
    let src = ctx.source;
    if n.parent().is_some_and(|p| p.kind() == "else_clause") {
        return;
    }
    let Some(branches) = chain_branches(n) else { return };
    let lists: Vec<Vec<Node<'_>>> = branches.iter().map(|b| stmts(*b)).collect();
    let shortest = lists.iter().map(Vec::len).min().unwrap_or(0);
    let lead = (0..shortest).take_while(|&i| lists.iter().all(|l| same_code(l[i], lists[0][i], src))).count();
    let trail = (0..shortest).take_while(|&i| lists.iter().all(|l| same_code(l[l.len() - 1 - i], lists[0][lists[0].len() - 1 - i], src))).count();
    if (lead == 0 && trail == 0) || lead + trail >= shortest {
        return;
    }
    let in_list = n.parent().is_some_and(|p| matches!(p.kind(), "program" | "statement_block" | "class_static_block" | "switch_case"));
    let declares = |s: &Node<'_>| matches!(s.kind(), "lexical_declaration" | "class_declaration");
    let ind = indent_of(src, n.start_byte()).to_owned();
    let emit = |shared: &[Node<'_>], leading: bool, out: &mut Vec<NativeFinding>| {
        let (first, last) = (shared[0], shared[shared.len() - 1]);
        let ok = in_list && !shared.iter().any(declares) && !shared.iter().any(|s| multiline_literal(*s)) && !branches.iter().any(|b| has_comment(*b));
        let mut edits = Vec::new();
        if ok {
            let sind = indent_of(src, first.start_byte()).to_owned();
            let moved = if first.start_position().row == n.start_position().row { src[first.start_byte()..last.end_byte()].to_owned() } else { reindent(src, first.start_byte(), last.end_byte(), &sind, &ind) };
            for l in &lists {
                let k = shared.len();
                if leading {
                    let next = l[k];
                    edits.push(TextEdit::delete(l[0].start_byte(), next.start_byte()));
                } else {
                    let keep = l[l.len() - k - 1];
                    edits.push(TextEdit::delete(keep.end_byte(), l[l.len() - 1].end_byte()));
                }
            }
            if leading {
                edits.push(TextEdit::insert(n.start_byte(), format!("{moved}\n{ind}")));
                let tests_pure = { let mut c = n; let mut pure = true; loop { pure &= c.child_by_field_name("condition").is_some_and(effect_free); match c.child_by_field_name("alternative").and_then(|a| named_children(a).into_iter().find(|x| x.kind() == "if_statement")) { Some(x) => c = x, None => break } } pure };
                if !tests_pure || !shared.iter().all(|s| s.kind() == "expression_statement" && effect_free(*s)) {
                    edits = as_suggestion(edits);
                }
            } else {
                edits.push(TextEdit::insert(n.end_byte(), format!("\n{ind}{moved}")));
            }
        }
        let where_ = if leading { "start" } else { "end" };
        out.push(span(first.start_byte(), last.end_byte(), format!("Every branch {}s with the same code: move it {} the `if`", where_, if leading { "before" } else { "after" }), edits));
    };
    if lead > 0 {
        emit(&lists[0][..lead], true, out);
    }
    if trail > 0 {
        let l = &lists[0];
        emit(&l[l.len() - trail..], false, out);
    }
});

// ---- ESLint: one-var ("always") ---------------------------------------------------------

rule!(OneVar, "js/one-var", &["lexical_declaration", "variable_declaration"], |n, ctx, out| {
    let src = ctx.source;
    let kind = n.child(0).map_or("", |k| text(k, src));
    let Some(p) = n.parent() else { return };
    if !matches!(p.kind(), "program" | "statement_block" | "switch_case" | "class_static_block") || crate::util::in_ambient(n) {
        return;
    }
    // The scope the declaration belongs to: its block for `let`/`const`,
    // its function for `var`.
    let scope = if kind == "var" { ancestors(n).find(|a| fn_like(a.kind()) || a.kind() == "program").unwrap_or(p) } else { p };
    let mut earlier = None;
    let mut stack = vec![scope];
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if c.start_byte() >= n.start_byte() {
                continue;
            }
            if c.kind() == n.kind() && c.child(0).is_some_and(|k| text(k, src) == kind) && c.parent().is_some_and(|q| matches!(q.kind(), "program" | "statement_block" | "switch_case" | "class_static_block")) {
                let same_scope = if kind == "var" { ancestors(c).find(|a| fn_like(a.kind()) || a.kind() == "program").is_some_and(|s| s.id() == scope.id()) } else { c.parent().is_some_and(|q| q.id() == p.id()) };
                if same_scope && earlier.is_none_or(|e: Node<'_>| e.start_byte() < c.start_byte()) {
                    earlier = Some(c);
                }
            }
            if kind == "var" && !fn_like(c.kind()) {
                stack.push(c);
            }
        }
    }
    let Some(prev) = earlier else { return };
    let mut edits = Vec::new();
    let adjacent = n.prev_named_sibling().is_some_and(|s| s.id() == prev.id());
    if adjacent && !has_comment(prev) && !has_comment(n) && !src[prev.end_byte()..n.start_byte()].contains("//") {
        let prev_end = if text(prev, src).ends_with(';') { prev.end_byte() - 1 } else { prev.end_byte() };
        let first = named_children(n).into_iter().find(|c| c.kind() == "variable_declarator");
        if let Some(f) = first {
            edits.push(TextEdit::new(prev_end, f.start_byte(), ", "));
        }
    }
    let k = n.child(0).unwrap_or(n);
    out.push(span(k.start_byte(), k.end_byte(), format!("Combine this with the previous `{kind}` declaration"), edits));
});

// ---- sonarjs: no-same-line-conditional (S3972) -------------------------------------------

rule!(NoSameLineConditional, "js/no-same-line-conditional", &["if_statement"], |n, ctx, out| {
    let src = ctx.source;
    let Some(prev) = n.prev_named_sibling().filter(|p| p.kind() == "if_statement") else { return };
    if !n.parent().is_some_and(|p| matches!(p.kind(), "program" | "statement_block" | "switch_case")) {
        return;
    }
    if prev.end_position().row != n.start_position().row || n.end_position().row == prev.start_position().row {
        return;
    }
    let ind = indent_of(src, prev.start_byte());
    let choices = vec![
        crate::choice("make it an else if", true, vec![TextEdit::insert(n.start_byte(), "else ")]),
        crate::choice("start it on its own line", false, vec![TextEdit::new(prev.end_byte(), n.start_byte(), format!("\n{ind}"))]),
    ];
    let k = n.child(0).unwrap_or(n);
    out.push(span(k.start_byte(), k.end_byte(), "This `if` starts on the line the previous `if` ends: write `else if`, or start it on a new line", crate::choice_edits(choices)));
});

// ---- unicorn: no-conflicting-constraints and no-invalid-integrity (JSX) -------------------

fn jsx_attrs<'t>(el: Node<'t>, src: &str) -> Option<(String, Vec<(String, Node<'t>)>)> {
    let open = match el.kind() {
        "jsx_self_closing_element" => el,
        "jsx_opening_element" => el,
        _ => return None,
    };
    let name = open.child_by_field_name("name").map(|x| text(x, src).to_owned())?;
    let mut attrs = Vec::new();
    for a in named_children(open) {
        match a.kind() {
            "jsx_attribute" => {
                let Some(nm) = a.named_child(0) else { continue };
                attrs.push((text(nm, src).to_owned(), a));
            }
            "jsx_expression" => attrs.push(("...".to_owned(), a)),
            _ => {}
        }
    }
    Some((name, attrs))
}

fn jsx_value(a: Node<'_>, src: &str) -> Option<String> {
    let v = a.named_child(1)?;
    match v.kind() {
        "string" => Some(crate::util::unquote(text(v, src)).to_owned()),
        "jsx_expression" => {
            let e = v.named_child(0)?;
            match e.kind() {
                "number" => Some(text(e, src).to_owned()),
                "string" => Some(crate::util::unquote(text(e, src)).to_owned()),
                "template_string" if !named_children(e).iter().any(|c| c.kind() == "template_substitution") => Some(text(e, src).trim_matches('`').to_owned()),
                _ => None,
            }
        }
        _ => None,
    }
}

rule!(NoConflictingConstraints, "js/no-conflicting-constraints", &["jsx_self_closing_element", "jsx_opening_element"], |n, ctx, out| {
    let src = ctx.source;
    let Some((tag, attrs)) = jsx_attrs(n, src) else { return };
    if !matches!(tag.as_str(), "input" | "textarea") || attrs.iter().any(|(k, _)| k == "...") {
        return;
    }
    let get = |k: &str| {
        let found: Vec<&(String, Node<'_>)> = attrs.iter().filter(|(x, _)| x == k).collect();
        match found[..] {
            [one] => Some((jsx_value(one.1, src), one.1)),
            _ => None,
        }
    };
    let ty = match get("type") {
        None if attrs.iter().all(|(k, _)| k != "type") => "text".to_owned(),
        Some((Some(t), _)) => if t.is_empty() { "text".to_owned() } else { t.to_ascii_lowercase() },
        _ => return,
    };
    let pairs: &[(&str, &str)] = match (tag.as_str(), ty.as_str()) {
        ("input", "number" | "range") => &[("min", "max")],
        ("textarea", _) | ("input", "text" | "search" | "url" | "tel" | "email" | "password") => &[("minLength", "maxLength")],
        _ => &[],
    };
    for (lo, hi) in pairs {
        let (Some((Some(a), _)), Some((Some(b), at))) = (get(lo), get(hi)) else { continue };
        let (Ok(x), Ok(y)) = (a.trim().parse::<f64>(), b.trim().parse::<f64>()) else { continue };
        if x > y {
            out.push(finding(at, format!("`{hi}` ({b}) is below `{lo}` ({a}): no value can satisfy both"), vec![]));
        }
    }
});

fn integrity_token(t: &str) -> Option<bool> {
    let (algo, len) = [("sha256", 43), ("sha384", 64), ("sha512", 86), ("ed25519", 43)].into_iter().find(|(a, _)| t.starts_with(a) && t[a.len()..].starts_with(['-', '?']))?;
    let rest = &t[algo.len()..];
    let Some(body) = rest.strip_prefix('-') else { return Some(false) };
    let (hash, opts) = body.split_once('?').map_or((body, None), |(h, o)| (h, Some(o)));
    let pad = hash.len() - hash.trim_end_matches('=').len();
    let core = hash.trim_end_matches('=');
    let ok = pad <= 2 && !core.is_empty() && core.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '/' | '-')) && core.len() == len && opts.is_none_or(|o| o.chars().all(|c| ('!'..='~').contains(&c)));
    Some(ok)
}

rule!(NoInvalidIntegrity, "js/no-invalid-integrity", &["jsx_self_closing_element", "jsx_opening_element"], |n, ctx, out| {
    let src = ctx.source;
    let Some((tag, attrs)) = jsx_attrs(n, src) else { return };
    if !matches!(tag.as_str(), "script" | "link") {
        return;
    }
    let Some(i) = attrs.iter().rposition(|(k, _)| k == "integrity") else { return };
    if attrs[i + 1..].iter().any(|(k, _)| k == "...") {
        return;
    }
    let at = attrs[i].1;
    let Some(value) = jsx_value(at, src) else { return };
    if value.trim().is_empty() {
        return;
    }
    let mut known = false;
    for tok in value.split_ascii_whitespace() {
        match integrity_token(tok) {
            Some(true) => known = true,
            Some(false) => {
                known = true;
                out.push(finding(at, format!("`{tok}` is not a valid integrity hash: the browser will refuse the resource"), vec![]));
            }
            None => {}
        }
    }
    if !known {
        out.push(finding(at, "This `integrity` names no hash the browser knows (sha256, sha384, sha512): it blocks the resource", vec![]));
    }
});

// ---- unicorn: consistent-class-member-order ------------------------------------------------

const GROUPS: &[&str] = &["static-field", "static-block", "static-method", "private-field", "public-field", "constructor", "private-method", "public-method"];

fn group_of(m: Node<'_>, src: &str) -> Option<&'static str> {
    let st = children(m).iter().any(|c| c.kind() == "static");
    let key = m.child_by_field_name("name").or_else(|| m.child_by_field_name("property"));
    let private = key.is_some_and(|k| k.kind() == "private_property_identifier") || children(m).iter().any(|c| c.kind() == "accessibility_modifier" && text(*c, src) == "private");
    match m.kind() {
        "class_static_block" => Some("static-block"),
        "field_definition" | "public_field_definition" => Some(if st { "static-field" } else if private { "private-field" } else { "public-field" }),
        "method_definition" | "abstract_method_signature" | "method_signature" => {
            if key.is_some_and(|k| text(k, src) == "constructor") && !st {
                Some("constructor")
            } else if st {
                Some("static-method")
            } else if private {
                Some("private-method")
            } else {
                Some("public-method")
            }
        }
        _ => None,
    }
}

rule!(ConsistentClassMemberOrder, "js/consistent-class-member-order", &["class_body"], |n, ctx, out| {
    let src = ctx.source;
    let order: Vec<String> = ctx.options.get("order").and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_str).map(str::to_owned).collect()).filter(|v: &Vec<String>| v.len() == GROUPS.len()).unwrap_or_else(|| GROUPS.iter().map(|s| (*s).to_owned()).collect());
    let rank = |g: &str| order.iter().position(|x| x == g).unwrap_or(0);
    let members: Vec<(Node<'_>, &'static str)> = named_children(n).into_iter().filter_map(|m| group_of(m, src).map(|g| (m, g))).collect();
    let mut max: Option<(Node<'_>, &str)> = None;
    for (m, g) in &members {
        if let Some((pm, pg)) = max
            && rank(g) < rank(pg)
        {
            let label = |x: Node<'_>, g: &str| {
                let nm = x.child_by_field_name("name").or_else(|| x.child_by_field_name("property")).map_or(String::new(), |k| format!(" `{}`", text(k, src)));
                format!("{}{nm}", g.replace('-', " "))
            };
            // A sorted body, when every member starts its own line and nothing would be lost.
            let mut edits = Vec::new();
            let lines_ok = members.iter().all(|(x, _)| src[..x.start_byte()].rsplit('\n').next().is_some_and(|l| l.trim().is_empty()));
            let odd = members.iter().any(|(x, _)| children(*x).iter().any(|c| c.kind() == "decorator") || x.child_by_field_name("name").is_some_and(|k| k.kind() == "computed_property_name"));
            if lines_ok && !odd && !has_comment(n) && named_children(n).iter().all(|c| group_of(*c, src).is_some()) {
                let mut sorted: Vec<(usize, &(Node<'_>, &str))> = members.iter().enumerate().collect();
                sorted.sort_by_key(|(i, (_, g))| (rank(g), *i));
                let statics_cross = sorted.iter().map(|(i, _)| *i).collect::<Vec<_>>() != (0..members.len()).collect::<Vec<_>>() && members.iter().any(|(_, g)| *g == "static-block") && members.iter().any(|(_, g)| *g == "static-field");
                if !statics_cross {
                    let ind = indent_of(src, members[0].0.start_byte()).to_owned();
                    let piece = |x: Node<'_>| {
                        let mut e = x.end_byte();
                        if src[e..].trim_start_matches([' ', '\t']).starts_with(';') {
                            e = e + (src[e..].len() - src[e..].trim_start_matches([' ', '\t']).len()) + 1;
                        }
                        src[x.start_byte()..e].to_owned()
                    };
                    let first = members[0].0;
                    let last = members[members.len() - 1].0;
                    let mut end = last.end_byte();
                    if src[end..].trim_start_matches([' ', '\t']).starts_with(';') {
                        end = end + (src[end..].len() - src[end..].trim_start_matches([' ', '\t']).len()) + 1;
                    }
                    let body = sorted.iter().map(|(_, (x, _))| piece(*x)).collect::<Vec<_>>().join(&format!("\n\n{ind}"));
                    edits = vec![TextEdit::new(first.start_byte(), end, body)];
                }
            }
            out.push(finding(*m, format!("Expected {} to come before {}", label(*m, g), label(pm, pg)), edits));
            return;
        }
        if max.is_none_or(|(_, pg)| rank(g) >= rank(pg)) {
            max = Some((*m, g));
        }
    }
});


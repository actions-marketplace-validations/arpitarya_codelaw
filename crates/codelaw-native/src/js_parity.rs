//! W-139 strand W-139-js, native rows: rules from ESLint, eslint-plugin-unicorn
//! and eslint-plugin-sonarjs that count, sort, compare code across a file or
//! compute a value — each its upstream behaviour, read from the upstream
//! rule's source and tests at the pinned version, and reimplemented.

use crate::util::{children, is_function, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use tree_sitter::Node;

mod scope;
mod scope_fix;
mod sonar_scope;
mod files;
mod polyfills;
mod shape;
mod values;
mod coll_rules;
mod iter_rules;
mod text_rules;
mod temporal;
mod arity;
mod builtin_props;
mod commented;
mod guards;
mod tables;
mod tests_rules;
mod maps;
mod objects;
mod style;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const TS: &[Language] = &[Language::Typescript, Language::Tsx];

fn finding(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

fn span(start: usize, end: usize, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

fn uint(o: &BTreeMap<String, Value>, key: &str, default: usize) -> usize {
    o.get(key).and_then(Value::as_u64).map_or(default, |v| v as usize)
}

macro_rules! rule {
    ($name:ident, $id:literal, $langs:expr, $kinds:expr, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                $langs
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

/// A function boundary for counting: functions, methods, classes and arrows.
fn is_boundary(kind: &str) -> bool {
    is_function(kind) || kind == "arrow_function" || kind == "class_static_block"
}

fn is_loop(kind: &str) -> bool {
    matches!(kind, "for_statement" | "for_in_statement" | "while_statement" | "do_statement")
}

/// The keyword at the start of a statement, as a report span.
fn keyword(n: Node<'_>) -> (usize, usize) {
    n.child(0).map_or((n.start_byte(), n.end_byte()), |k| (k.start_byte(), k.end_byte()))
}

fn logical_op<'s>(n: Node<'_>, src: &'s str) -> Option<&'s str> {
    if n.kind() != "binary_expression" {
        return None;
    }
    let op = n.child_by_field_name("operator").map(|o| text(o, src))?;
    matches!(op, "&&" | "||" | "??").then_some(op)
}

// ---- unicorn: max-nested-calls -------------------------------------------

rule!(MaxNestedCalls, "js/max-nested-calls", JS, &["call_expression", "new_expression"], |n, ctx, out| {
    let max = uint(ctx.options, "max", 3);
    let mut depth = 1;
    let mut child = n;
    let mut cur = n.parent();
    while let Some(p) = cur {
        if is_boundary(p.kind()) || matches!(p.kind(), "jsx_element" | "jsx_self_closing_element" | "jsx_fragment") {
            break;
        }
        if p.kind() == "arguments" && child.kind() != "spread_element" && p.parent().is_some_and(|g| matches!(g.kind(), "call_expression" | "new_expression")) {
            depth += 1;
        }
        child = p;
        cur = p.parent();
    }
    if depth > max {
        out.push(finding(n, format!("This call is nested {depth} deep (at most {max}): give an intermediate result a name"), vec![]));
    }
});

// ---- unicorn: try-complexity ---------------------------------------------

/// Decision points under `n`, not entering nested functions: the signals
/// ESLint's `complexity` counts.
fn decisions(n: Node<'_>, src: &str) -> usize {
    let mut count = 0;
    for c in children(n) {
        if is_boundary(c.kind()) {
            continue;
        }
        count += match c.kind() {
            "if_statement" | "ternary_expression" | "for_statement" | "for_in_statement" | "while_statement" | "do_statement" | "catch_clause" => 1,
            "switch_case" => 1,
            "binary_expression" => usize::from(logical_op(c, src).is_some()),
            "augmented_assignment_expression" => usize::from(c.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "&&=" | "||=" | "??="))),
            _ => 0,
        };
        count += decisions(c, src);
    }
    count
}

rule!(TryComplexity, "js/try-complexity", JS, &["try_statement"], |n, ctx, out| {
    let max = uint(ctx.options, "max", 1);
    let Some(body) = n.child_by_field_name("body") else { return };
    let complexity = 1 + decisions(body, ctx.source);
    if complexity > max {
        let (s, e) = keyword(n);
        out.push(span(s, e, format!("This `try` block has a complexity of {complexity} (at most {max}): move the code that can't throw out of it"), vec![]));
    }
});

// ---- sonarjs: too-many-break-or-continue-in-loop -------------------------

/// `break` and `continue` statements under `n` that leave the loop `n` is the
/// body of: not inside a nested function or loop, and a `break` not inside a
/// nested `switch`.
fn jumps(n: Node<'_>, in_switch: bool, out: &mut usize) {
    for c in children(n) {
        let k = c.kind();
        if is_boundary(k) || is_loop(k) {
            continue;
        }
        match k {
            "break_statement" if !in_switch || c.named_child_count() > 0 => *out += 1,
            "continue_statement" => *out += 1,
            "switch_statement" => jumps(c, true, out),
            _ => jumps(c, in_switch, out),
        }
    }
}

rule!(TooManyBreakOrContinueInLoop, "js/too-many-break-or-continue-in-loop", JS, &["for_statement", "for_in_statement", "while_statement", "do_statement"], |n, _ctx, out| {
    let Some(body) = n.child_by_field_name("body") else { return };
    let mut count = 0;
    jumps(body, false, &mut count);
    if count > 1 {
        let (s, e) = keyword(n);
        out.push(span(s, e, format!("This loop is left by {count} `break`/`continue` statements: restructure it to leave in one place"), vec![]));
    }
});

// ---- sonarjs: expression-complexity --------------------------------------

fn operators(n: Node<'_>, src: &str) -> usize {
    if is_boundary(n.kind()) {
        return 0;
    }
    let own = match n.kind() {
        "ternary_expression" => 1,
        "binary_expression" => usize::from(logical_op(n, src).is_some()),
        _ => 0,
    };
    own + children(n).into_iter().map(|c| operators(c, src)).sum::<usize>()
}

fn is_expression_root(n: Node<'_>, src: &str) -> bool {
    let mut p = n.parent();
    while let Some(q) = p {
        match q.kind() {
            "parenthesized_expression" | "unary_expression" => p = q.parent(),
            "ternary_expression" => return false,
            "binary_expression" if logical_op(q, src).is_some() => return false,
            "binary_expression" => p = q.parent(),
            _ => return true,
        }
    }
    true
}

rule!(ExpressionComplexity, "js/expression-complexity", JS, &["binary_expression", "ternary_expression"], |n, ctx, out| {
    let src = ctx.source;
    if n.kind() == "binary_expression" && logical_op(n, src).is_none() {
        return;
    }
    if !is_expression_root(n, src) {
        return;
    }
    let max = uint(ctx.options, "max", 3);
    let count = operators(n, src);
    if count > max {
        out.push(finding(n, format!("This expression has {count} conditional operators (at most {max}): split it with named intermediate values"), vec![]));
    }
});

// ---- sonarjs: cognitive-complexity ---------------------------------------

/// Cognitive complexity of a function body (SonarSource's definition): +1
/// for each break in the linear flow — `if`, `else if`, `else`, `switch`,
/// loops, `catch`, `?:`, a labelled jump, each run of like logical operators —
/// plus the nesting depth for the structures that nest. Nested functions
/// raise the nesting of what they hold.
fn cognitive(n: Node<'_>, src: &str, nesting: usize, total: &mut usize) {
    for c in children(n) {
        let k = c.kind();
        match k {
            "if_statement" => {
                let is_else_if = c.parent().is_some_and(|p| p.kind() == "else_clause");
                *total += if is_else_if { 1 } else { 1 + nesting };
                if let Some(cond) = c.child_by_field_name("condition") {
                    cognitive(cond, src, nesting, total);
                }
                if let Some(cons) = c.child_by_field_name("consequence") {
                    cognitive(cons, src, nesting + 1, total);
                }
                if let Some(alt) = c.child_by_field_name("alternative") {
                    let inner = named_children(alt).into_iter().next();
                    match inner {
                        Some(i) if i.kind() == "if_statement" => cognitive(alt, src, nesting, total),
                        _ => {
                            *total += 1;
                            cognitive(alt, src, nesting + 1, total);
                        }
                    }
                }
            }
            "switch_statement" | "for_statement" | "for_in_statement" | "while_statement" | "do_statement" | "catch_clause" | "ternary_expression" => {
                *total += 1 + nesting;
                cognitive(c, src, nesting + 1, total);
            }
            "break_statement" | "continue_statement" if c.named_child_count() > 0 => {
                *total += 1;
            }
            "binary_expression" if logical_op(c, src).is_some() => {
                let op = logical_op(c, src);
                let parent_same = c.parent().and_then(|p| logical_op(p, src)) == op && op.is_some();
                if !parent_same {
                    // one run of like operators, plus one per change of operator inside it
                    *total += 1 + operator_changes(c, src);
                }
                cognitive(c, src, nesting, total);
            }
            _ if is_boundary(k) => cognitive(c, src, nesting + 1, total),
            _ => cognitive(c, src, nesting, total),
        }
    }
}

/// The operators of a logical chain, left to right, through parentheses.
fn chain_ops<'s>(n: Node<'_>, src: &'s str, out: &mut Vec<&'s str>) {
    if let Some(op) = logical_op(n, src) {
        if let Some(l) = n.child_by_field_name("left") {
            chain_ops(l, src, out);
        }
        out.push(op);
        if let Some(r) = n.child_by_field_name("right") {
            chain_ops(r, src, out);
        }
    }
}

fn operator_changes(n: Node<'_>, src: &str) -> usize {
    let mut ops = Vec::new();
    chain_ops(n, src, &mut ops);
    ops.windows(2).filter(|w| w[0] != w[1]).count()
}

fn function_name_span(n: Node<'_>) -> (usize, usize) {
    if let Some(name) = n.child_by_field_name("name") {
        return (name.start_byte(), name.end_byte());
    }
    if let Some(p) = n.parent()
        && p.kind() == "variable_declarator"
        && let Some(name) = p.child_by_field_name("name")
    {
        return (name.start_byte(), name.end_byte());
    }
    keyword(n)
}

rule!(CognitiveComplexity, "js/cognitive-complexity", JS, &["function_declaration", "function_expression", "arrow_function", "method_definition", "generator_function_declaration", "generator_function"], |n, ctx, out| {
    // a function nested in another is counted with it
    let mut p = n.parent();
    while let Some(q) = p {
        if is_boundary(q.kind()) && !matches!(q.kind(), "class_declaration" | "class" | "abstract_class_declaration") {
            return;
        }
        p = q.parent();
    }
    let Some(body) = n.child_by_field_name("body") else { return };
    let threshold = uint(ctx.options, "threshold", 15);
    let mut total = 0;
    cognitive(body, ctx.source, 0, &mut total);
    if total > threshold {
        let (s, e) = function_name_span(n);
        out.push(span(s, e, format!("This function's cognitive complexity is {total} (at most {threshold}): extract parts of it, or flatten its nesting"), vec![]));
    }
});

// ---- ESLint: sort-keys, sort-vars, sort-imports --------------------------

/// Reorder `items` (nodes of one list) into `order` by swapping their texts
/// in place: the separators between them stay where they are.
fn permute(items: &[Node<'_>], order: &[usize], src: &str) -> Vec<TextEdit> {
    items
        .iter()
        .zip(order)
        .filter_map(|(slot, &j)| {
            let t = text(items[j], src);
            (text(*slot, src) != t).then(|| TextEdit::new(slot.start_byte(), slot.end_byte(), t.to_owned()))
        })
        .collect()
}

fn has_comment(n: Node<'_>) -> bool {
    children(n).into_iter().any(|c| c.kind() == "comment" || has_comment(c))
}

/// A value whose evaluation can't be observed: moving it is safe.
fn is_quiet(n: Node<'_>) -> bool {
    matches!(
        n.kind(),
        "string" | "number" | "true" | "false" | "null" | "undefined" | "identifier" | "regex" | "arrow_function" | "function_expression" | "template_string"
    ) && !(n.kind() == "template_string" && children(n).iter().any(|c| c.kind() == "template_substitution"))
}

fn static_key(prop: Node<'_>, src: &str) -> Option<String> {
    match prop.kind() {
        "shorthand_property_identifier" => Some(text(prop, src).to_owned()),
        "pair" | "method_definition" => {
            let key = prop.child_by_field_name("key").or_else(|| prop.child_by_field_name("name"))?;
            match key.kind() {
                "property_identifier" | "private_property_identifier" | "number" => Some(text(key, src).to_owned()),
                "string" => Some(crate::util::string_value(key, src)),
                _ => None,
            }
        }
        _ => None,
    }
}

rule!(SortKeys, "js/sort-keys", JS, &["object"], |n, ctx, out| {
    let src = ctx.source;
    let props: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    if props.len() < 2 {
        return;
    }
    // a spread starts a new group, as upstream
    let mut first_bad: Option<(Node<'_>, String, String)> = None;
    let mut prev: Option<String> = None;
    for p in &props {
        if p.kind() == "spread_element" {
            prev = None;
            continue;
        }
        let Some(k) = static_key(*p, src) else {
            prev = None;
            continue;
        };
        if let Some(pk) = &prev
            && k < *pk
            && first_bad.is_none()
        {
            first_bad = Some((*p, k.clone(), pk.clone()));
        }
        prev = Some(k);
    }
    let Some((at, k, pk)) = first_bad else { return };
    // the fix sorts the whole object when every key is static, nothing is
    // spread, no comment sits inside and no value's evaluation can be seen
    let simple = !has_comment(n)
        && props.iter().all(|p| match p.kind() {
            "shorthand_property_identifier" | "method_definition" => true,
            "pair" => static_key(*p, src).is_some() && p.child_by_field_name("value").is_some_and(is_quiet),
            _ => false,
        });
    let edits = if simple {
        let keys: Vec<String> = props.iter().map(|p| static_key(*p, src).unwrap_or_default()).collect();
        let mut order: Vec<usize> = (0..props.len()).collect();
        order.sort_by(|&a, &b| keys[a].cmp(&keys[b]));
        crate::as_suggestion(permute(&props, &order, src))
    } else {
        vec![]
    };
    out.push(finding(at, format!("Keys should be sorted: `{k}` before `{pk}`"), edits));
});

rule!(SortVars, "js/sort-vars", JS, &["lexical_declaration", "variable_declaration"], |n, ctx, out| {
    let src = ctx.source;
    let decls: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
    if decls.len() < 2 {
        return;
    }
    let names: Vec<Option<String>> = decls.iter().map(|d| d.child_by_field_name("name").filter(|x| x.kind() == "identifier").map(|x| text(x, src).to_owned())).collect();
    let mut prev: Option<&String> = None;
    let mut bad = None;
    for (i, nm) in names.iter().enumerate() {
        let Some(nm) = nm else {
            prev = None;
            continue;
        };
        if let Some(p) = prev
            && nm < p
        {
            bad = Some((i, nm.clone(), p.clone()));
            break;
        }
        prev = Some(nm);
    }
    let Some((i, nm, p)) = bad else { return };
    // fixed only when every declarator is a name with no value or a literal one
    let fixable = !has_comment(n)
        && names.iter().all(Option::is_some)
        && decls.iter().all(|d| d.child_by_field_name("value").is_none_or(|v| matches!(v.kind(), "string" | "number" | "true" | "false" | "null")));
    let edits = if fixable {
        let mut order: Vec<usize> = (0..decls.len()).collect();
        order.sort_by(|&a, &b| names[a].cmp(&names[b]));
        permute(&decls, &order, src)
    } else {
        vec![]
    };
    out.push(finding(decls[i], format!("Variables within a declaration should be sorted: `{nm}` before `{p}`"), edits));
});

rule!(SortImports, "js/sort-imports", JS, &["named_imports"], |n, ctx, out| {
    let src = ctx.source;
    let specs: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() == "import_specifier").collect();
    if specs.len() < 2 || has_comment(n) {
        return;
    }
    let local = |s: &Node<'_>| s.child_by_field_name("alias").or_else(|| s.child_by_field_name("name")).map(|x| text(x, src).to_owned()).unwrap_or_default();
    let names: Vec<String> = specs.iter().map(local).collect();
    let Some(i) = (1..names.len()).find(|&i| names[i] < names[i - 1]) else { return };
    let mut order: Vec<usize> = (0..specs.len()).collect();
    order.sort_by(|&a, &b| names[a].cmp(&names[b]));
    out.push(finding(specs[i], format!("Import members should be sorted: `{}` before `{}`", names[i], names[i - 1]), permute(&specs, &order, src)));
});

// ---- sonarjs: no-duplicate-string ----------------------------------------

fn string_excluded(s: Node<'_>) -> bool {
    let Some(p) = s.parent() else { return true };
    match p.kind() {
        "import_statement" | "export_statement" | "jsx_attribute" | "literal_type" | "import_attribute" => true,
        "pair" => p.child_by_field_name("key") == Some(s),
        "expression_statement" => true, // a directive such as "use strict"
        _ => false,
    }
}

fn collect_strings<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in children(n) {
        if c.kind() == "string" {
            out.push(c);
        } else {
            collect_strings(c, out);
        }
    }
}

rule!(NoDuplicateString, "js/no-duplicate-string", JS, &["program"], |n, ctx, out| {
    let src = ctx.source;
    let threshold = uint(ctx.options, "threshold", 3);
    let mut all = Vec::new();
    collect_strings(n, &mut all);
    let mut seen: HashMap<String, Vec<Node<'_>>> = HashMap::new();
    let mut order = Vec::new();
    for s in all {
        if string_excluded(s) {
            continue;
        }
        if s.parent().is_some_and(|p| p.kind() == "arguments") && s.parent().and_then(|p| p.parent()).and_then(|c| c.child_by_field_name("function")).is_some_and(|f| text(f, src) == "require") {
            continue;
        }
        let v = crate::util::string_value(s, src);
        // short strings and single words are left, as upstream
        if v.chars().count() < 10 || v.chars().all(|c| c.is_alphanumeric() || c == '_') || v == "application/json" {
            continue;
        }
        let e = seen.entry(v.clone()).or_default();
        if e.is_empty() {
            order.push(v);
        }
        e.push(s);
    }
    for v in order {
        let uses = &seen[&v];
        if uses.len() >= threshold {
            out.push(finding(uses[0], format!("This string is written {} times: define a constant for it", uses.len()), vec![]));
        }
    }
});

// ---- sonarjs: no-identical-functions -------------------------------------

fn squash(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn collect_functions<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in children(n) {
        if matches!(c.kind(), "function_declaration" | "function_expression" | "arrow_function" | "method_definition") {
            out.push(c);
        }
        collect_functions(c, out);
    }
}

rule!(NoIdenticalFunctions, "js/no-identical-functions", JS, &["program"], |n, ctx, out| {
    let src = ctx.source;
    let mut fns = Vec::new();
    collect_functions(n, &mut fns);
    let mut first: HashMap<String, Node<'_>> = HashMap::new();
    for f in fns {
        let Some(body) = f.child_by_field_name("body").filter(|b| b.kind() == "statement_block") else { continue };
        // bodies of at least three lines, as upstream
        if body.end_position().row < body.start_position().row + 2 {
            continue;
        }
        let params = f.child_by_field_name("parameters").map_or(String::new(), |p| squash(text(p, src)));
        let key = format!("{params}{}", squash(text(body, src)));
        match first.get(&key) {
            Some(orig) => {
                let (s, e) = function_name_span(f);
                out.push(span(s, e, format!("This function has the same body as the one on line {}: reuse it", orig.start_position().row + 1), vec![]));
            }
            None => {
                first.insert(key, f);
            }
        }
    }
});

// ---- unicorn: prefer-unicode-code-point-escapes, prefer-literal-ascii -----

/// The code unit of a `\xHH` or `\uHHHH` escape, or the code point of `\u{…}`.
fn escape_value(t: &str) -> Option<(u32, bool)> {
    let h = t.strip_prefix('\\')?;
    if let Some(x) = h.strip_prefix('x') {
        return (x.len() == 2).then(|| u32::from_str_radix(x, 16).ok()).flatten().map(|v| (v, false));
    }
    let u = h.strip_prefix('u')?;
    if let Some(b) = u.strip_prefix('{') {
        return u32::from_str_radix(b.strip_suffix('}')?, 16).ok().map(|v| (v, true));
    }
    (u.len() == 4).then(|| u32::from_str_radix(u, 16).ok()).flatten().map(|v| (v, false))
}

fn escapes<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    named_children(n).into_iter().filter(|c| c.kind() == "escape_sequence").collect()
}

fn printable_ascii(v: u32) -> bool {
    (0x20..=0x7e).contains(&v)
}

rule!(PreferUnicodeCodePointEscapes, "js/prefer-unicode-code-point-escapes", JS, &["string", "template_string"], |n, ctx, out| {
    let src = ctx.source;
    let es = escapes(n);
    let mut edits = Vec::new();
    let mut i = 0;
    while i < es.len() {
        let e = es[i];
        let t = text(e, src);
        let Some((v, braced)) = escape_value(t) else {
            i += 1;
            continue;
        };
        if braced || printable_ascii(v) {
            i += 1;
            continue;
        }
        // a surrogate pair written as two adjacent escapes is one code point
        if (0xd800..0xdc00).contains(&v)
            && let Some(next) = es.get(i + 1)
            && next.start_byte() == e.end_byte()
            && let Some((lo, false)) = escape_value(text(*next, src))
            && (0xdc00..0xe000).contains(&lo)
        {
            let cp = 0x10000 + ((v - 0xd800) << 10) + (lo - 0xdc00);
            edits.push(TextEdit::new(e.start_byte(), next.end_byte(), format!("\\u{{{cp:X}}}")));
            i += 2;
            continue;
        }
        if (0xd800..0xe000).contains(&v) {
            i += 1;
            continue; // a lone surrogate has no code point form
        }
        edits.push(TextEdit::new(e.start_byte(), e.end_byte(), format!("\\u{{{v:X}}}")));
        i += 1;
    }
    if !edits.is_empty() {
        out.push(finding(n, "Write these escapes as code point escapes, `\\u{…}`", edits));
    }
});

fn in_directive(n: Node<'_>) -> bool {
    n.parent().is_some_and(|p| p.kind() == "expression_statement" && p.parent().is_some_and(|b| matches!(b.kind(), "program" | "statement_block")))
}

rule!(PreferLiteralAscii, "js/prefer-literal-ascii", JS, &["string", "template_string"], |n, ctx, out| {
    let src = ctx.source;
    if in_directive(n) || n.parent().is_some_and(|p| p.kind() == "call_expression") {
        return; // a directive prologue, or a tagged template, keeps its escapes
    }
    let quote = text(n, src).chars().next().unwrap_or('"');
    let mut edits = Vec::new();
    for e in escapes(n) {
        let Some((v, _)) = escape_value(text(e, src)) else { continue };
        if !printable_ascii(v) {
            continue;
        }
        let c = char::from_u32(v).unwrap_or('\0');
        if c == quote || c == '\\' || (quote == '`' && matches!(c, '$' | '{')) {
            continue;
        }
        // a digit right after `\0` would turn it into an octal escape
        if c.is_ascii_digit() && src[..e.start_byte()].ends_with("\\0") {
            continue;
        }
        edits.push(TextEdit::new(e.start_byte(), e.end_byte(), c.to_string()));
    }
    if !edits.is_empty() {
        out.push(finding(n, "Write printable ASCII as itself, not as an escape", edits));
    }
});

// ---- sonarjs: no-unenclosed-multiline-block ------------------------------

rule!(NoUnenclosedMultilineBlock, "js/no-unenclosed-multiline-block", JS, &["if_statement", "for_statement", "for_in_statement", "while_statement"], |n, _ctx, out| {
    let field = if n.kind() == "if_statement" { "consequence" } else { "body" };
    let Some(body) = n.child_by_field_name(field) else { return };
    if body.kind() == "statement_block" || (n.kind() == "if_statement" && n.child_by_field_name("alternative").is_some()) {
        return;
    }
    if body.start_position().row == n.start_position().row {
        return; // a body on the header's line is not indented as a block
    }
    let Some(next) = n.next_named_sibling() else { return };
    if next.kind() == "comment" || next.start_position().row <= body.end_position().row {
        return;
    }
    let col = |x: Node<'_>| x.start_position().column;
    if col(next) == col(body) && col(body) > col(n) {
        out.push(finding(next, "This statement is indented as if it were in the block above, but it is not: add braces or dedent it", vec![]));
    }
});

// ---- sonarjs: use-type-alias ---------------------------------------------

fn collect_unions<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in children(n) {
        if matches!(c.kind(), "union_type" | "intersection_type") && !c.parent().is_some_and(|p| p.kind() == c.kind()) && !c.parent().is_some_and(|p| p.kind() == "type_alias_declaration") {
            out.push(c);
        }
        collect_unions(c, out);
    }
}

fn members(n: Node<'_>) -> usize {
    named_children(n).into_iter().map(|c| if c.kind() == n.kind() { members(c) } else { 1 }).sum()
}

rule!(UseTypeAlias, "js/use-type-alias", TS, &["program"], |n, ctx, out| {
    let src = ctx.source;
    let mut all = Vec::new();
    collect_unions(n, &mut all);
    let mut seen: HashMap<String, Vec<Node<'_>>> = HashMap::new();
    let mut order = Vec::new();
    for u in all {
        if members(u) < 3 {
            continue;
        }
        let key = squash(text(u, src));
        let e = seen.entry(key.clone()).or_default();
        if e.is_empty() {
            order.push(key);
        }
        e.push(u);
    }
    for k in order {
        let uses = &seen[&k];
        if uses.len() >= 3 {
            out.push(finding(uses[0], format!("This type is written {} times: name it with a type alias", uses.len()), vec![]));
        }
    }
});

pub static RULES: &[&dyn NativeRule] = &[
    &MaxNestedCalls,
    &TryComplexity,
    &TooManyBreakOrContinueInLoop,
    &ExpressionComplexity,
    &CognitiveComplexity,
    &SortKeys,
    &SortVars,
    &SortImports,
    &NoDuplicateString,
    &NoIdenticalFunctions,
    &PreferUnicodeCodePointEscapes,
    &PreferLiteralAscii,
    &NoUnenclosedMultilineBlock,
    &UseTypeAlias,
    &scope::NoThisOutsideOfClass,
    &scope::NoTopLevelAssignmentInFunction,
    &scope::NoUselessRecursion,
    &scope::NoUselessReExport,
    &scope::NoUndeclaredClassMembers,
    &scope::NoIncompleteAccessorOverride,
    &scope::NoLateCurrentTargetAccess,
    &scope::NoLateEventControl,
    &scope::NoLoopIterableMutation,
    &scope::NoMismatchedMapKey,
    &scope::NoCanvasToImage,
    &scope::ConsistentFunctionScoping,
    &scope::IsolatedFunctions,
    &scope::NoUncalledMethod,
    &scope::NoUnusedProperties,
    &scope::PreferGetOrInsertComputed,
    &scope::NoTopLevelSideEffects,
    &scope::BlockScopedVar,
    &scope::NoInvalidThis,
    &scope::NoLoopFunc,
    &scope::NoUnmodifiedLoopCondition,
    &scope::NoUseBeforeDefine,
    &scope::NoRepeatedCallWrappers,
    &scope_fix::PreferClassFields,
    &scope_fix::PreferObjectDefineProperties,
    &scope_fix::PreferSetHas,
    &scope_fix::PreferExportFrom,
    &scope_fix::NoDeclarationsBeforeEarlyExit,
    &scope_fix::NoImmediateMutation,
    &scope_fix::ConsistentDestructuring,
    &scope_fix::NoBlobToFile,
    &scope_fix::PreferPromiseWithResolvers,
    &scope_fix::PreferAbortSignalTimeout,
    &scope_fix::NoForLoop,
    &scope_fix::PreferSmallerScope,
    &scope_fix::NoUnnecessaryDefaultArguments,
    &scope_fix::NoUnsafePromiseAllSettledValues,
    &sonar_scope::AvoidMutatingNestedPropertiesOfShallowClones,
    &sonar_scope::FunctionInsideLoop,
    &sonar_scope::InconsistentFunctionCall,
    &sonar_scope::MemoizeCacheKey,
    &sonar_scope::MisplacedLoopCounter,
    &sonar_scope::NoDebounceThrottleInRender,
    &sonar_scope::NoEmptyCollection,
    &sonar_scope::NoUnusedCollection,
    &sonar_scope::NoExtraArguments,
    &sonar_scope::NoGratuitousExpressions,
    &sonar_scope::NoInvariantReturns,
    &sonar_scope::NoMutateReactiveStateInUpdatedHook,
    &sonar_scope::NoUseOfEmptyReturnValue,
    &sonar_scope::NoVariableUsageBeforeDeclaration,
    &sonar_scope::StatefulRegex,
    &files::FilenameCase,
    &files::FileNameDifferFromClass,
    &files::ImportUnambiguous,
    &files::NoExportsInScripts,
    &files::ImportStyle,
    &files::NoInternalApiUse,
    &files::ImportNoSelfImport,
    &files::ImportExtensions,
    &files::ImportNoUselessPathSegments,
    &files::ImportNoRelativePackages,
    &files::NodeFileExtensionInImport,
    &files::NodeNoUnpublishedBin,
    &files::NodePreferProcessGetBuiltinModule,
    &files::ImportExport,
    &files::NoMissingLocalResource,
    &polyfills::NoUnnecessaryPolyfills,
    &shape::ConsistentArrowReturnStyle,
    &shape::NoDuplicateLoops,
    &shape::NoRedundantComparison,
    &shape::PreferContinue,
    &shape::PreferEarlyReturn,
    &shape::PreferSingleReplace,
    &shape::PreferHoistingBranchCode,
    &shape::OneVar,
    &shape::NoSameLineConditional,
    &shape::NoConflictingConstraints,
    &shape::NoInvalidIntegrity,
    &shape::ConsistentClassMemberOrder,
    &coll_rules::NoCollectionBracketAccess,
    &coll_rules::NoObjectMethodsWithCollections,
    &coll_rules::NoUselessSetConstruction,
    &coll_rules::PreferArrayIterableMethods,
    &iter_rules::NoUnusedIteratorHelper,
    &iter_rules::NoUselessIteratorToArray,
    &iter_rules::PreferIteratorToArray,
    &iter_rules::PreferIteratorToArrayAtEnd,
    &iter_rules::PreferIteratorHelpers,
    &iter_rules::PreferIteratorConcat,
    &text_rules::TemplateIndent,
    &temporal::PreferTemporalConversion,
    &arity::NoInvalidArgumentCount,
    &builtin_props::NoNonstandardBuiltinProperties,
    &commented::NoCommentedCode,
    &guards::PreferTypeGuard,
    &tables::TableHeaderReference,
    &tests_rules::NoDuplicateParameterizedTestCase,
    &maps::PreferMapFromEntries,
    &objects::PreferObjectIterableMethods,
    &style::NoShorthandPropertyOverrides,
];

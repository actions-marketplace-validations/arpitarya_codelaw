//! W-139 strand W-139-jsfinal: eslint-plugin-unicorn v79.0.0's
//! `prefer-iterator-zip`, read from its source and tests and written again.
//! A `for` loop whose index only reads the same position of two or more
//! arrays, and stops at the shortest, becomes a loop over `Iterator.zip`.
//! The edit is always a suggestion: `Iterator.zip` reads every row before
//! the body and no engine ships it yet.

use super::kit::{comment_in, number, operator, optional, srule, strip_parens};
use super::scope::{ancestors, fn_like};
use super::{JS, span};
use crate::util::{children, named_children, text};
use crate::{NativeFinding, as_suggestion};
use codelaw_core::TextEdit;
use codelaw_semantic::SemanticModel;
use std::collections::BTreeSet;
use tree_sitter::Node;

fn is_ident<'s>(n: Node<'_>, name: &str, src: &'s str) -> bool {
    n.kind() == "identifier" && text(n, src) == name
}

fn is_increment(update: Node<'_>, name: &str, src: &str) -> bool {
    let update = strip_parens(update);
    match update.kind() {
        "update_expression" => operator(update, src) == Some("++") && update.child_by_field_name("argument").is_some_and(|a| is_ident(a, name, src)),
        "augmented_assignment_expression" => {
            update.child_by_field_name("left").is_some_and(|l| is_ident(l, name, src)) && operator(update, src) == Some("+=") && update.child_by_field_name("right").is_some_and(|r| number(r, src) == Some(1.0))
        }
        "assignment_expression" => {
            let (Some(l), Some(r)) = (update.child_by_field_name("left"), update.child_by_field_name("right")) else { return false };
            let r = strip_parens(r);
            is_ident(l, name, src)
                && r.kind() == "binary_expression"
                && operator(r, src) == Some("+")
                && r.child_by_field_name("left").is_some_and(|x| is_ident(x, name, src))
                && r.child_by_field_name("right").is_some_and(|x| number(x, src) == Some(1.0))
        }
        _ => false,
    }
}

/// `index < bound` or `bound > index`: the bound.
fn upper_bound<'t>(test: Node<'t>, index: &str, src: &str) -> Option<Node<'t>> {
    let test = strip_parens(test);
    if test.kind() != "binary_expression" {
        return None;
    }
    let (l, r) = (test.child_by_field_name("left")?, test.child_by_field_name("right")?);
    match operator(test, src)? {
        "<" if is_ident(strip_parens(l), index, src) => Some(r),
        ">" if is_ident(strip_parens(r), index, src) => Some(l),
        _ => None,
    }
}

/// `array.length`, a plain identifier's length.
fn length_input<'t>(n: Node<'t>, src: &str) -> Option<Node<'t>> {
    let n = strip_parens(n);
    if n.kind() != "member_expression" || optional(n) {
        return None;
    }
    let (o, p) = (n.child_by_field_name("object")?, n.child_by_field_name("property")?);
    (o.kind() == "identifier" && p.kind() == "property_identifier" && text(p, src) == "length").then_some(o)
}

fn distinct<'t>(inputs: Vec<Option<Node<'t>>>, src: &str) -> Option<Vec<Node<'t>>> {
    let inputs: Vec<Node<'t>> = inputs.into_iter().collect::<Option<Vec<_>>>()?;
    let names: BTreeSet<&str> = inputs.iter().map(|i| text(*i, src)).collect();
    (names.len() == inputs.len()).then_some(inputs)
}

/// `Math.min(a.length, b.length, …)` over distinct identifiers.
fn math_min_inputs<'t>(n: Node<'t>, src: &str) -> Option<Vec<Node<'t>>> {
    let n = strip_parens(n);
    if n.kind() != "call_expression" || optional(n) {
        return None;
    }
    let f = n.child_by_field_name("function")?;
    if f.kind() != "member_expression" || optional(f) || f.child_by_field_name("object").is_none_or(|o| !is_ident(o, "Math", src)) || f.child_by_field_name("property").is_none_or(|p| text(p, src) != "min") {
        return None;
    }
    let args = crate::js_ast::arguments(n);
    if args.len() < 2 {
        return None;
    }
    distinct(args.into_iter().map(|a| length_input(a, src)).collect(), src)
}

fn and_operands<'t>(n: Node<'t>, src: &str, out: &mut Vec<Node<'t>>) {
    let n = strip_parens(n);
    if n.kind() == "binary_expression" && operator(n, src) == Some("&&") {
        if let (Some(l), Some(r)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) {
            and_operands(l, src, out);
            and_operands(r, src, out);
            return;
        }
    }
    out.push(n);
}

fn test_inputs<'t>(test: Node<'t>, index: &str, src: &str) -> Option<Vec<Node<'t>>> {
    if let Some(i) = upper_bound(test, index, src).and_then(|b| math_min_inputs(b, src)) {
        return Some(i);
    }
    let mut operands = Vec::new();
    and_operands(test, src, &mut operands);
    if operands.len() < 2 {
        return None;
    }
    distinct(operands.into_iter().map(|o| upper_bound(o, index, src).and_then(|b| length_input(b, src))).collect(), src)
}

struct Loop<'t> {
    inputs: Vec<Node<'t>>,
    /// The `const`/`let` before the loop that caches the bound, removed with it.
    removal: Option<Node<'t>>,
}

fn sole_declarator(decl: Node<'_>) -> Option<Node<'_>> {
    let ds: Vec<Node<'_>> = named_children(decl).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
    (ds.len() == 1).then(|| ds[0])
}

/// A cached bound: `i < size` where `size` is `Math.min(a.length, b.length)`
/// and is read nowhere else.
fn cached<'t>(for_stmt: Node<'t>, test: Node<'t>, index: &str, declarator: Node<'t>, removal: Option<Node<'t>>, m: &SemanticModel, src: &str) -> Option<Loop<'t>> {
    let bound = strip_parens(upper_bound(test, index, src)?);
    let name = declarator.child_by_field_name("name").filter(|n| n.kind() == "identifier")?;
    if bound.kind() != "identifier" || text(bound, src) != text(name, src) {
        return None;
    }
    let sym = m.declared_at(name.start_byte())?;
    let reads: Vec<_> = m.symbol(sym).refs.iter().map(|&r| &m.references[r]).filter(|r| r.start != name.start_byte()).collect();
    if reads.len() != 1 || reads[0].start != bound.start_byte() {
        return None;
    }
    let inputs = math_min_inputs(declarator.child_by_field_name("value")?, src)?;
    let _ = for_stmt;
    Some(Loop { inputs, removal })
}

fn loop_info<'t>(n: Node<'t>, m: &SemanticModel, src: &str) -> Option<(Loop<'t>, Node<'t>, String)> {
    let body = n.child_by_field_name("body").filter(|b| b.kind() == "statement_block")?;
    let init = n.child_by_field_name("initializer").filter(|i| i.kind() == "lexical_declaration" && text(*i, src).starts_with("let"))?;
    let decls: Vec<Node<'_>> = named_children(init).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
    if decls.is_empty() || decls.len() > 2 {
        return None;
    }
    let first = decls[0];
    let name_node = first.child_by_field_name("name").filter(|x| x.kind() == "identifier")?;
    if first.child_by_field_name("value").and_then(|v| number(v, src)) != Some(0.0) {
        return None;
    }
    let index = text(name_node, src).to_owned();
    if !is_increment(n.child_by_field_name("increment")?, &index, src) {
        return None;
    }
    let mut test = n.child_by_field_name("condition")?;
    if test.kind() == "expression_statement" {
        test = test.named_child(0)?;
    }
    let found = if decls.len() == 1 {
        match test_inputs(test, &index, src) {
            Some(inputs) => Some(Loop { inputs, removal: None }),
            None => {
                let prev = std::iter::successors(n.prev_named_sibling(), |p| p.prev_named_sibling()).find(|p| p.kind() != "comment")?;
                if prev.kind() != "lexical_declaration" {
                    return None;
                }
                let d = sole_declarator(prev)?;
                cached(n, test, &index, d, Some(prev), m, src)
            }
        }
    } else {
        cached(n, test, &index, decls[1], None, m, src)
    }?;
    Some((found, body, index))
}

/// Whether a value is known to be a string or a plain object, which `zip`
/// cannot index the way an array is indexed.
fn known_non_indexed(ident: Node<'_>, m: &SemanticModel, src: &str) -> bool {
    let root = crate::js_ast::root_of(ident);
    let Some(v) = crate::js_ast::initial_value(m, root, ident) else { return false };
    let v = crate::js_ast::unparen(v);
    match v.kind() {
        "string" | "object" | "regex" | "number" | "true" | "false" | "null" => true,
        "template_string" => true,
        "new_expression" => v.child_by_field_name("constructor").is_some_and(|c| matches!(text(c, src), "Map" | "Set" | "WeakMap" | "WeakSet" | "Object" | "String")),
        _ => false,
    }
}

fn captured(ident: Node<'_>, body: Node<'_>) -> bool {
    ancestors(ident).take_while(|a| a.id() != body.id()).any(|a| fn_like(a.kind()) || matches!(a.kind(), "class" | "class_declaration" | "class_static_block"))
}

/// The outermost `x as T` / `x!` / `(x)` around a node.
fn outermost(mut n: Node<'_>) -> Node<'_> {
    while let Some(p) = n.parent() {
        if matches!(p.kind(), "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression") && p.named_child(0).is_some_and(|c| c.id() == n.id()) {
            n = p;
        } else {
            break;
        }
    }
    n
}

fn is_left_hand_side(n: Node<'_>) -> bool {
    let Some(p) = n.parent() else { return false };
    match p.kind() {
        "assignment_expression" | "augmented_assignment_expression" => p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()),
        "assignment_pattern" | "object_assignment_pattern" => p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()),
        "update_expression" | "array_pattern" | "rest_pattern" | "pair_pattern" => true,
        "unary_expression" => p.child(0).is_some_and(|c| c.kind() == "delete"),
        "for_in_statement" => p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()),
        _ => false,
    }
}

fn is_element_read(n: Node<'_>) -> bool {
    let n = outermost(n);
    let Some(p) = n.parent() else { return true };
    !is_left_hand_side(n)
        && !(p.kind() == "call_expression" && p.child_by_field_name("function").is_some_and(|f| f.id() == n.id()))
        && !(p.kind() == "call_expression" && p.child_by_field_name("arguments").is_some_and(|a| a.kind() == "template_string" && a.id() == n.id()))
}

/// Whether `name` is free in the loop body: not bound in the body's scopes or
/// any scope around them, and not an unresolved reference there.
pub(super) fn name_free(m: &SemanticModel, body: Node<'_>, name: &str) -> bool {
    let mut cur = Some(m.scope_at(body.start_byte() + 1));
    while let Some(sc) = cur {
        if m.scopes[sc].bindings.contains_key(name) {
            return false;
        }
        cur = m.scopes[sc].parent;
    }
    if m.scopes.iter().any(|s| s.start >= body.start_byte() && s.end <= body.end_byte() && s.bindings.contains_key(name)) {
        return false;
    }
    !m.references.iter().any(|r| r.symbol.is_none() && r.name == name && r.start >= body.start_byte() && r.end <= body.end_byte())
}

fn available(name: &str, m: &SemanticModel, body: Node<'_>, taken: &BTreeSet<String>) -> String {
    let mut name = name.to_owned();
    if RESERVED.contains(&name.as_str()) {
        name.push('_');
    }
    while taken.contains(&name) || name == "Iterator" || !name_free(m, body, &name) {
        name.push('_');
    }
    name
}

const RESERVED: &[&str] = &[
    "break", "case", "catch", "class", "const", "continue", "debugger", "default", "delete", "do", "else", "enum", "export", "extends", "false", "finally", "for", "function", "if", "import", "in", "instanceof", "new", "null", "return", "super", "switch", "this", "throw", "true", "try", "typeof", "var", "void", "while", "with", "yield", "let", "static", "await", "async", "of", "undefined", "NaN", "Infinity", "arguments", "eval",
];

srule!(PreferIteratorZip, "js/prefer-iterator-zip", JS, &["for_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some((found, body, _)) = loop_info(n, m, src) else { return };
    let inputs = &found.inputs;
    if inputs.iter().any(|i| known_non_indexed(*i, m, src)) {
        return;
    }
    let Some(index_sym) = m.symbol_at(n.child_by_field_name("initializer").and_then(|i| named_children(i).into_iter().find(|c| c.kind() == "variable_declarator")).and_then(|d| d.child_by_field_name("name")).map_or(0, |x| x.start_byte())) else { return };
    // Each input as (name, the symbol it resolves to at the loop's header).
    let input_syms: Vec<(String, Option<usize>)> = inputs.iter().map(|i| (text(*i, src).to_owned(), m.reference_at(i.start_byte()).and_then(|r| r.symbol))).collect();
    let mut reads: Vec<Vec<Node<'_>>> = vec![Vec::new(); inputs.len()];
    let mut all = Vec::new();
    crate::util::descendants(body, &|_| false, &mut all);
    for id in all.into_iter().filter(|x| x.kind() == "identifier") {
        let name = text(id, src);
        let Some(r) = m.reference_at(id.start_byte()) else { continue };
        let is_index = r.symbol == Some(index_sym);
        let input_at = input_syms.iter().position(|(nm, sym)| nm == name && *sym == r.symbol);
        if !is_index && input_at.is_none() {
            continue;
        }
        let Some(member) = id.parent().filter(|p| p.kind() == "subscript_expression") else { return };
        if captured(id, body) || optional(member) {
            return;
        }
        let (Some(obj), Some(idx)) = (member.child_by_field_name("object"), member.child_by_field_name("index")) else { return };
        if obj.kind() != "identifier" || idx.kind() != "identifier" {
            return;
        }
        let obj_at = input_syms.iter().position(|(nm, sym)| *nm == text(obj, src) && *sym == m.reference_at(obj.start_byte()).and_then(|r| r.symbol));
        if obj_at.is_none() || m.reference_at(idx.start_byte()).and_then(|r| r.symbol) != Some(index_sym) || !is_element_read(member) {
            return;
        }
        // The member is reached once through its object and once through its index.
        if id.id() == obj.id()
            && let Some(at) = obj_at
        {
            reads[at].push(member);
        }
    }
    if reads.iter().any(Vec::is_empty) {
        return;
    }
    let Some(close) = children(n).into_iter().rfind(|c| c.kind() == ")" && c.end_byte() <= body.start_byte()) else { return };
    let header = (n.start_byte(), close.end_byte());
    let removal = found.removal.map(|d| (d.start_byte(), n.start_byte()));
    let report = |edits: Vec<TextEdit>, out: &mut Vec<NativeFinding>| out.push(span(header.0, header.1, "Use `Iterator.zip(…)` to iterate over these arrays together.", edits));
    let root = crate::js_ast::root_of(n);
    let mut ranges = vec![header];
    ranges.extend(reads.iter().flatten().map(|r| (r.start_byte(), r.end_byte())));
    ranges.extend(removal);
    if ranges.iter().any(|(s, e)| comment_in(root, *s, *e)) {
        report(vec![], out);
        return;
    }
        let mut taken = BTreeSet::new();
    let mut names = Vec::new();
    for (nm, _) in &input_syms {
        let base = super::scope_fix::singular(nm);
        let name = available(&base, m, body, &taken);
        taken.insert(name.clone());
        names.push(name);
    }
    let list: Vec<&str> = input_syms.iter().map(|(nm, _)| nm.as_str()).collect();
    let mut edits = Vec::new();
    if let Some((s, e)) = removal {
        edits.push(TextEdit::delete(s, e));
    }
    edits.push(TextEdit::new(header.0, header.1, format!("for (const [{}] of Iterator.zip([{}]))", names.join(", "), list.join(", "))));
    for (i, group) in reads.iter().enumerate() {
        for read in group {
            let mut rep = names[i].clone();
            let (b, a) = (src[..read.start_byte()].chars().next_back(), src[read.end_byte()..].chars().next());
            if b.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
                rep.insert(0, ' ');
            }
            if a.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
                rep.push(' ');
            }
            edits.push(TextEdit::new(read.start_byte(), read.end_byte(), rep));
        }
    }
    report(as_suggestion(edits), out);
});

//! W-139 strand W-139-js: eslint-plugin-unicorn v79.0.0's
//! `prefer-object-iterable-methods`, read from the rule's source and tests
//! and written again. A `for…of` over `Object.keys(o)` whose body reads
//! `o[key]`, a `.map()` callback over `Object.keys(o)` that does, and a
//! loop or callback over `Object.entries(o)` that ignores half of each
//! pair, want `Object.values()`, `Object.entries()` or `Object.keys()`.

use super::values::{Probe, has_comment, has_side_effect, is_optional, static_call};
use super::{JS, finding};
use crate::js_ast::{self, arguments, is_async};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const CALLBACKS: &[(&str, usize)] = &[("every", 0), ("findIndex", 0), ("findLastIndex", 0), ("flatMap", 0), ("forEach", 0), ("map", 0), ("some", 0), ("reduce", 1), ("reduceRight", 1)];

fn same_ref(a: Node<'_>, b: Node<'_>, src: &str) -> bool {
    let norm = |n: Node<'_>| -> String { text(js_ast::unparen(n), src).split_whitespace().collect() };
    norm(a) == norm(b)
}

/// The nodes below `n`, source order, entering nested functions and
/// classes only as a boundary (their insides are not the target's own).
fn own<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    let mut stack = vec![n];
    while let Some(x) = stack.pop() {
        out.push(x);
        if x.id() != n.id() && (js_ast::is_function(x) || matches!(x.kind(), "class" | "class_declaration")) {
            continue;
        }
        let mut c = named_children(x);
        c.reverse();
        stack.extend(c);
    }
    out.sort_by_key(|x| x.start_byte());
    out
}

fn is_mutation_target(m: Node<'_>) -> bool {
    let Some(par) = m.parent() else { return false };
    let own = |f: &str| par.child_by_field_name(f).is_some_and(|x| x.id() == m.id());
    match par.kind() {
        "assignment_expression" | "augmented_assignment_expression" => own("left"),
        "update_expression" => own("argument"),
        "unary_expression" => par.child(0).is_some_and(|c| c.kind() == "delete"),
        "for_in_statement" => own("left"),
        "array_pattern" | "object_pattern" | "spread_element" => false,
        _ => false,
    }
}

fn refs_in<'a>(p: &Probe<'a, '_>, sym: usize, target: Node<'_>) -> Vec<&'a codelaw_semantic::Reference> {
    p.m.symbol(sym).refs.iter().map(|&r| &p.m.references[r]).filter(|r| r.start >= target.start_byte() && r.end <= target.end_byte()).collect()
}

fn free_name(p: &Probe<'_, '_>, at: usize, target: Node<'_>) -> Option<String> {
    for cand in ["value", "value2", "value3"] {
        let mut used = p.m.lookup(p.m.scope_at(at), cand, at).is_some();
        for n in own(target) {
            if n.kind() == "identifier" && text(n, p.src) == cand {
                used = true;
            }
        }
        if !used {
            return Some(cand.to_owned());
        }
    }
    None
}

fn abrupt(target: Node<'_>, throw_only: bool) -> bool {
    own(target).iter().any(|n| matches!(n.kind(), "throw_statement") || (!throw_only && matches!(n.kind(), "break_statement" | "continue_statement" | "return_statement")))
}

struct Ctx<'a, 't> {
    p: &'a Probe<'a, 't>,
    out: &'a mut Vec<NativeFinding>,
}

/// `binding` is what receives each item; `target` the body that uses it;
/// `call` is `Object.keys(o)` / `Object.entries(o)`.
fn check_binding(c: &mut Ctx<'_, '_>, call: Node<'_>, binding: Node<'_>, target: Node<'_>, can_fix: bool, parenthesize_single: bool) {
    let p = c.p;
    let src = p.src;
    let Some(method) = static_call(p, call, "Object").filter(|m| matches!(*m, "keys" | "entries")) else { return };
    let args = arguments(call);
    let [object] = args[..] else { return };
    if object.kind() == "spread_element" {
        return;
    }
    let prop = call.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")).unwrap_or(call);
    if method == "keys" {
        if binding.kind() != "identifier" {
            return;
        }
        let Some(ksym) = p.m.symbol_at(binding.start_byte()) else { return };
        let krefs = refs_in(p, ksym, target);
        if krefs.iter().any(|r| r.write) {
            return;
        }
        let obj = js_ast::unparen(object);
        // The object must not be written to inside the target.
        if obj.kind() == "identifier"
            && let Some(osym) = p.m.symbol_at(obj.start_byte())
            && refs_in(p, osym, target).iter().any(|r| r.write)
        {
            return;
        }
        let nodes = own(target);
        let mut members: Vec<Node<'_>> = Vec::new();
        for n in &nodes {
            if n.kind() == "subscript_expression" && is_mutation_target(*n) && n.child_by_field_name("object").is_some_and(|o| same_ref(o, obj, src)) {
                return;
            }
            if n.kind() == "subscript_expression"
                && !is_optional(*n)
                && n.child_by_field_name("object").is_some_and(|o| same_ref(o, obj, src))
                && n.child_by_field_name("index").is_some_and(|i| i.kind() == "identifier" && p.m.symbol_at(i.start_byte()) == Some(ksym))
            {
                if is_mutation_target(*n) {
                    return;
                }
                members.push(*n);
            }
        }
        if members.is_empty() {
            return;
        }
        let wrapped = members.iter().any(|m| m.child_by_field_name("object").is_some_and(|o| matches!(o.kind(), "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion")));
        let member_index_ids: Vec<usize> = members.iter().filter_map(|m| m.child_by_field_name("index")).map(|i| i.start_byte()).collect();
        let key_refs = krefs.iter().filter(|r| !member_index_ids.contains(&r.start)).count();
        let preferred = if key_refs == 0 { "values" } else { "entries" };
        let mut edits = Vec::new();
        let unsafe_side_effects = members.iter().any(|m| nodes.iter().any(|n| (n.end_byte() <= m.start_byte() || n.start_byte() >= m.end_byte()) && !(n.start_byte() <= m.start_byte() && n.end_byte() >= m.end_byte()) && has_side_effect(*n)));
        if can_fix && !wrapped && !unsafe_side_effects && !has_comment(binding) && !members.iter().any(|m| has_comment(*m)) && let Some(value) = free_name(p, binding.start_byte(), target) {
            edits.push(TextEdit::new(prop.start_byte(), prop.end_byte(), preferred));
            let name = text(binding, src);
            let b = if preferred == "values" { value.clone() } else { format!("[{name}, {value}]") };
            let b = if preferred == "entries" && parenthesize_single { format!("({b})") } else { b };
            edits.push(TextEdit::new(binding.start_byte(), binding.end_byte(), b));
            for m in &members {
                edits.push(TextEdit::new(m.start_byte(), m.end_byte(), value.clone()));
            }
        }
        c.out.push(finding(prop, format!("Use `Object.{preferred}()` instead of `Object.keys()`: the loop reads each value by its key"), edits));
        return;
    }
    // entries
    if binding.kind() != "array_pattern" {
        return;
    }
    let slots = super::values::pattern_slots(binding);
    if slots.len() > 2 || slots.iter().all(Option::is_none) || slots.iter().flatten().any(|s| s.kind() != "identifier") {
        return;
    }
    let key = slots.first().copied().flatten();
    let value = slots.get(1).copied().flatten();
    let mut counts = [0usize; 2];
    for (i, s) in [key, value].into_iter().enumerate() {
        if let Some(s) = s {
            let Some(sym) = p.m.symbol_at(s.start_byte()) else { return };
            let refs = refs_in(p, sym, target);
            if refs.iter().any(|r| r.write) {
                return;
            }
            counts[i] = refs.len();
        }
    }
    if counts[0] == counts[1] || (counts[0] > 0 && counts[1] > 0) {
        return;
    }
    let preferred = if counts[0] == 0 { "values" } else { "keys" };
    let keep = if preferred == "values" { value } else { key };
    let Some(keep) = keep else { return };
    let mut edits = Vec::new();
    if can_fix && !has_comment(binding) {
        edits.push(TextEdit::new(prop.start_byte(), prop.end_byte(), preferred));
        edits.push(TextEdit::new(binding.start_byte(), binding.end_byte(), text(keep, src)));
    }
    c.out.push(finding(prop, format!("Use `Object.{preferred}()` instead of `Object.entries()`: half of each pair is not used"), edits));
}

pub struct PreferObjectIterableMethods;
impl NativeRule for PreferObjectIterableMethods {
    fn id(&self) -> &'static str {
        "js/prefer-object-iterable-methods"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["for_in_statement", "call_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        let p = Probe::new(model, src, js_ast::root_of(n));
        let mut c = Ctx { p: &p, out };
        if n.kind() == "for_in_statement" {
            if children(n).iter().any(|k| k.kind() == "await") {
                return;
            }
            let (Some(left), Some(right), Some(body)) = (n.child_by_field_name("left"), n.child_by_field_name("right"), n.child_by_field_name("body")) else { return };
            if !src[left.end_byte()..right.start_byte()].contains("of") || !matches!(n.child_by_field_name("kind").map(|k| text(k, src)), Some("const" | "let")) {
                return;
            }
            check_binding(&mut c, right, left, body, !abrupt(body, false), false);
            return;
        }
        // A callback method on Object.keys(o) / Object.entries(o).
        let Some((recv, name)) = super::values::method(src, n, &CALLBACKS.iter().map(|(m, _)| *m).collect::<Vec<_>>()) else { return };
        let (_, bi) = CALLBACKS.iter().find(|(m, _)| *m == name).copied().unwrap_or(("", 0));
        let args = arguments(n);
        let is_keys = static_call(&p, recv, "Object") == Some("keys");
        let is_entries = static_call(&p, recv, "Object") == Some("entries");
        if !(is_entries || is_keys && name == "map") || args.is_empty() || args.len() > 2 || (bi == 1 && args.len() != 2) {
            return;
        }
        let cb = args[0];
        if !matches!(cb.kind(), "arrow_function" | "function_expression" | "function") || is_async(cb) {
            return;
        }
        let params = js_ast::params(cb);
        let extra = if is_entries { 2 } else { 1 };
        if params.len() <= bi || params.len() > bi + extra || (params.len() != bi + 1 && params.last().is_some_and(|l| l.kind() != "identifier" && l.child_by_field_name("pattern").is_none_or(|x| x.kind() != "identifier"))) {
            return;
        }
        let Some(body) = cb.child_by_field_name("body") else { return };
        if js_ast::is_function(body) || matches!(body.kind(), "class" | "class_declaration") {
            return;
        }
        let uses_arguments = cb.kind() != "arrow_function" && own(cb).iter().any(|x| x.kind() == "identifier" && text(*x, src) == "arguments");
        if uses_arguments {
            return;
        }
        let param = params[bi];
        let binding = if param.kind() == "required_parameter" { param.child_by_field_name("pattern").filter(|_| param.child_by_field_name("type").is_none()) } else { Some(param) };
        let Some(binding) = binding else { return };
        let single_bare = cb.kind() == "arrow_function" && cb.child_by_field_name("parameter").is_some();
        check_binding(&mut c, recv, binding, body, !abrupt(body, true), single_bare);
    }
}

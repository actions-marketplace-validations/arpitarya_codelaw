//! W-139 strand W-139-js: eslint-plugin-unicorn v79.0.0's
//! `prefer-map-from-entries`, read from the rule's source and tests and
//! written again. A `const` made by `Object.fromEntries()` of
//! `Object.entries()` or of string-keyed pairs, and used only as a map
//! (`x.key`, `x.key = v`, `delete x.key`, `Object.hasOwn(x, 'key')`,
//! `Object.keys/values/entries(x)`), becomes a `Map` — the declaration and
//! every use together, or not at all.

use super::values::{Probe, has_comment, is_global, is_optional, static_call};
use super::{JS, finding};
use crate::js_ast::{self, arguments};
use crate::util::{named_children, string_value, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const INHERITED: &[&str] = &["__defineGetter__", "__defineSetter__", "__lookupGetter__", "__lookupSetter__", "__proto__", "constructor", "hasOwnProperty", "isPrototypeOf", "propertyIsEnumerable", "toLocaleString", "toString", "valueOf"];

fn array_index(k: &str) -> bool {
    k.parse::<u64>().is_ok_and(|n| n <= 4_294_967_294 && n.to_string() == k)
}

fn static_string(n: Node<'_>, src: &str) -> Option<String> {
    let n = js_ast::unparen(n);
    match n.kind() {
        "string" => Some(string_value(n, src)),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Some(text(n, src).trim_matches('`').to_owned()),
        _ => None,
    }
}

fn quote(k: &str) -> String {
    format!("'{}'", k.replace('\\', "\\\\").replace('\'', "\\'").replace('\n', "\\n"))
}

fn known_entries(p: &Probe<'_, '_>, arg: Node<'_>) -> bool {
    let arg = js_ast::unparen(arg);
    if static_call(p, arg, "Object") == Some("entries") && arguments(arg).len() == 1 {
        return true;
    }
    if arg.kind() != "array" {
        return false;
    }
    let els: Vec<Node<'_>> = named_children(arg).into_iter().filter(|c| c.kind() != "comment").collect();
    !els.is_empty()
        && els.iter().all(|e| {
            let e = js_ast::unparen(*e);
            if e.kind() != "array" {
                return false;
            }
            let parts: Vec<Node<'_>> = named_children(e).into_iter().filter(|c| c.kind() != "comment").collect();
            parts.len() >= 2 && parts[0].kind() != "spread_element" && static_string(parts[0], p.src).is_some_and(|k| !array_index(&k))
        })
}

/// One use of the variable and what it becomes.
struct Op {
    start: usize,
    end: usize,
    replacement: String,
}

fn operation(p: &Probe<'_, '_>, id: Node<'_>, name: &str) -> Option<Op> {
    let par = id.parent()?;
    let src = p.src;
    match par.kind() {
        "arguments" => {
            let call = par.parent().filter(|c| c.kind() == "call_expression")?;
            let method = static_call(p, call, "Object")?;
            let args = arguments(call);
            if args.first().is_none_or(|a| a.id() != id.id()) {
                return None;
            }
            if method == "hasOwn" {
                if args.len() != 2 || args[1].kind() == "spread_element" {
                    return None;
                }
                let key = static_string(args[1], src)?;
                return Some(Op { start: call.start_byte(), end: call.end_byte(), replacement: format!("{name}.has({})", quote(&key)) });
            }
            if matches!(method, "keys" | "values" | "entries") && args.len() == 1 {
                return Some(Op { start: call.start_byte(), end: call.end_byte(), replacement: format!("[...{name}.{method}()]") });
            }
            None
        }
        "member_expression" | "subscript_expression" => {
            if par.child_by_field_name("object").is_none_or(|o| o.id() != id.id()) || is_optional(par) {
                return None;
            }
            let key = if par.kind() == "member_expression" {
                let pr = par.child_by_field_name("property").filter(|x| x.kind() == "property_identifier")?;
                text(pr, src).to_owned()
            } else {
                static_string(par.child_by_field_name("index")?, src)?
            };
            if array_index(&key) || INHERITED.contains(&key.as_str()) {
                return None;
            }
            let gp = par.parent()?;
            if matches!(gp.kind(), "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion") {
                return None;
            }
            let mut a = par.parent();
            while let Some(x) = a {
                if matches!(x.kind(), "array_pattern" | "object_pattern" | "optional_chain") {
                    return None;
                }
                a = x.parent();
            }
            let standalone = |n: Node<'_>| n.parent().is_some_and(|x| x.kind() == "expression_statement");
            match gp.kind() {
                "assignment_expression" if gp.child_by_field_name("left").is_some_and(|l| l.id() == par.id()) => {
                    let right = gp.child_by_field_name("right")?;
                    if !standalone(gp) || !src[par.end_byte()..right.start_byte()].trim().eq("=") {
                        return None;
                    }
                    let value = if right.kind() == "sequence_expression" { format!("({})", text(right, src)) } else { text(right, src).to_owned() };
                    Some(Op { start: gp.start_byte(), end: gp.end_byte(), replacement: format!("{name}.set({}, {value})", quote(&key)) })
                }
                "unary_expression" if gp.child(0).is_some_and(|c| c.kind() == "delete") => {
                    if !standalone(gp) {
                        return None;
                    }
                    Some(Op { start: gp.start_byte(), end: gp.end_byte(), replacement: format!("{name}.delete({})", quote(&key)) })
                }
                "update_expression" | "augmented_assignment_expression" | "for_in_statement" => None,
                "call_expression" if gp.child_by_field_name("function").is_some_and(|f| f.id() == par.id()) => None,
                "new_expression" if gp.child_by_field_name("constructor").is_some_and(|f| f.id() == par.id()) => None,
                "member_expression" | "subscript_expression" if gp.child_by_field_name("object").is_some_and(|f| f.id() == par.id()) => Some(Op { start: par.start_byte(), end: par.end_byte(), replacement: format!("{name}.get({})", quote(&key)) }),
                _ => Some(Op { start: par.start_byte(), end: par.end_byte(), replacement: format!("{name}.get({})", quote(&key)) }),
            }
        }
        _ => None,
    }
}

pub struct PreferMapFromEntries;
impl NativeRule for PreferMapFromEntries {
    fn id(&self) -> &'static str {
        "js/prefer-map-from-entries"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["variable_declarator"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let d = node.get_inner_node();
        let src = ctx.source;
        let p = Probe::new(model, src, js_ast::root_of(d));
        let (Some(id), Some(init)) = (d.child_by_field_name("name"), d.child_by_field_name("value")) else { return };
        if id.kind() != "identifier" || d.child_by_field_name("type").is_some() {
            return;
        }
        let Some(decl) = d.parent().filter(|x| x.kind() == "lexical_declaration" && x.child(0).is_some_and(|k| text(k, src) == "const")) else { return };
        if decl.parent().is_some_and(|x| x.kind() == "export_statement") {
            return;
        }
        let init = js_ast::unparen(init);
        if static_call(&p, init, "Object") != Some("fromEntries") || init.child_by_field_name("type_arguments").is_some() {
            return;
        }
        let args = arguments(init);
        let [arg] = args[..] else { return };
        if !known_entries(&p, arg) || model.lookup(model.scope_at(init.start_byte()), "Map", init.start_byte()).is_some() {
            return;
        }
        let Some(sym) = model.symbol_at(id.start_byte()) else { return };
        let s = model.symbol(sym);
        if s.decls.len() != 1 {
            return;
        }
        let name = text(id, src);
        let mut ops: Vec<Op> = Vec::new();
        for &r in &s.refs {
            let rf = &model.references[r];
            if rf.type_only {
                return;
            }
            let Some(node_ref) = p.root.descendant_for_byte_range(rf.start, rf.end) else { return };
            let Some(op) = operation(&p, node_ref, name) else { return };
            ops.push(op);
        }
        if ops.is_empty() {
            return;
        }
        ops.sort_by_key(|o| o.start);
        if ops.windows(2).any(|w| w[0].end > w[1].start) {
            return;
        }
        let callee = init.child_by_field_name("function").expect("a call has a callee");
        let mut edits = Vec::new();
        if !has_comment(init) {
            edits.push(TextEdit::new(callee.start_byte(), callee.end_byte(), "new Map"));
            for o in &ops {
                let span = p.root.descendant_for_byte_range(o.start, o.end);
                let at_start = span.is_some_and(|n| n.parent().is_some_and(|x| x.kind() == "expression_statement") && o.replacement.starts_with('['));
                if span.is_some_and(has_comment) || at_start {
                    edits.clear();
                    break;
                }
                edits.push(TextEdit::new(o.start, o.end, o.replacement.clone()));
            }
        }
        let _ = is_global;
        out.push(finding(init, format!("`{name}` is only used like a map: build it with `new Map()` and read it with `get()`, `set()` and `has()`"), edits));
    }
}

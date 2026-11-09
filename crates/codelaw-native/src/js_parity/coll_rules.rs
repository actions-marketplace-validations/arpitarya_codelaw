//! W-139 strand W-139-js, the built-in collection rows: eslint-plugin-unicorn
//! v79.0.0's `no-collection-bracket-access`, `no-object-methods-with-collections`,
//! `no-useless-set-construction` and `prefer-array-iterable-methods`, read
//! from the rule's source and tests and written again on the syntax
//! questions of [`super::values`]: a `Map`, `Set` or array is known by its
//! constructor, its annotation, or a `const` alias of one.

use super::values::{MAP, Probe, SET, WEAK_MAP, arg_count, has_comment, has_side_effect, is_global, is_optional, member_object_text, method, pattern_slots, static_call, strip};
use super::{JS, finding};
use crate::js_ast::{self, arguments};
use crate::util::{named_children, string_value, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

macro_rules! crule {
    ($name:ident, $id:literal, $kinds:expr, |$n:ident, $ctx:ident, $p:ident, $out:ident| $body:block) => {
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
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let Some(model) = $ctx.semantic else { return };
                let $n = node.get_inner_node();
                let $p = Probe::new(model, $ctx.source, js_ast::root_of($n));
                $body
            }
        }
    };
}

pub(super) fn type_name(bit: u8) -> &'static str {
    match bit {
        MAP => "Map",
        SET => "Set",
        WEAK_MAP => "WeakMap",
        _ => "WeakSet",
    }
}

// ---- unicorn: no-collection-bracket-access ---------------------------------

const OBJECT_MEMBERS: &[&str] = &["constructor", "hasOwnProperty", "isPrototypeOf", "propertyIsEnumerable", "toLocaleString", "toString", "valueOf", "__proto__", "__defineGetter__", "__defineSetter__", "__lookupGetter__", "__lookupSetter__"];

fn has_member(kind: u8, name: &str) -> bool {
    if OBJECT_MEMBERS.contains(&name) {
        return true;
    }
    let own: &[&str] = match kind {
        MAP => &["size", "get", "set", "has", "delete", "clear", "forEach", "keys", "values", "entries", "getOrInsert", "getOrInsertComputed"],
        SET => &["size", "add", "has", "delete", "clear", "forEach", "keys", "values", "entries", "union", "intersection", "difference", "symmetricDifference", "isSubsetOf", "isSupersetOf", "isDisjointFrom"],
        WEAK_MAP => &["get", "set", "has", "delete", "getOrInsert", "getOrInsertComputed"],
        _ => &["add", "has", "delete"],
    };
    own.contains(&name)
}

fn has_symbol(kind: u8, name: &str) -> bool {
    name == "toStringTag" || (name == "iterator" && matches!(kind, MAP | SET))
}

/// What a key expression is statically: `Skip` when it depends on something
/// that could change (a branch with a call in it), else the possible values
/// as `(is_symbol, text)`, or `None` when they are not known.
enum Key {
    Skip,
    Known(Vec<(bool, String)>),
    Unknown,
}

fn key_values(p: &Probe<'_, '_>, n: Node<'_>, depth: usize) -> Key {
    let n = js_ast::unparen(n);
    if depth > 8 {
        return Key::Unknown;
    }
    match n.kind() {
        "string" => Key::Known(vec![(false, string_value(n, p.src))]),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Key::Known(vec![(false, text(n, p.src).trim_matches('`').to_owned())]),
        "number" => Key::Known(vec![(false, "\u{0}number".to_owned())]),
        "member_expression" => {
            let (Some(o), Some(pr)) = (n.child_by_field_name("object"), n.child_by_field_name("property")) else { return Key::Unknown };
            if is_global(p.m, o, "Symbol", p.src) {
                Key::Known(vec![(true, text(pr, p.src).to_owned())])
            } else {
                Key::Unknown
            }
        }
        "subscript_expression" => {
            let (Some(o), Some(i)) = (n.child_by_field_name("object"), n.child_by_field_name("index")) else { return Key::Unknown };
            if is_global(p.m, o, "Symbol", p.src) && i.kind() == "string" {
                Key::Known(vec![(true, string_value(i, p.src))])
            } else {
                Key::Unknown
            }
        }
        "identifier" => {
            let Some(sym) = p.m.symbol_at(n.start_byte()) else { return Key::Unknown };
            let s = p.m.symbol(sym);
            if s.decls.len() != 1 || s.kind != codelaw_semantic::SymbolKind::Const {
                return Key::Unknown;
            }
            let (a, b) = s.decls[0];
            let Some(id) = p.root.descendant_for_byte_range(a, b) else { return Key::Unknown };
            match super::values::const_init(id, p.src) {
                Some(init) => key_values(p, init, depth + 1),
                None => Key::Unknown,
            }
        }
        "ternary_expression" | "binary_expression" => {
            let (a, b) = if n.kind() == "ternary_expression" {
                (n.child_by_field_name("consequence"), n.child_by_field_name("alternative"))
            } else {
                if !n.child_by_field_name("operator").is_some_and(|o| matches!(text(o, p.src), "&&" | "||" | "??")) {
                    return Key::Unknown;
                }
                (n.child_by_field_name("left"), n.child_by_field_name("right"))
            };
            if has_side_effect(n) {
                return Key::Skip;
            }
            let (Some(a), Some(b)) = (a, b) else { return Key::Unknown };
            match (key_values(p, a, depth + 1), key_values(p, b, depth + 1)) {
                (Key::Known(mut x), Key::Known(y)) => {
                    x.extend(y);
                    Key::Known(x)
                }
                _ => Key::Unknown,
            }
        }
        "sequence_expression" => {
            if has_side_effect(n) {
                Key::Skip
            } else {
                Key::Unknown
            }
        }
        _ => Key::Unknown,
    }
}

fn is_write_target(n: Node<'_>) -> bool {
    let Some(parent) = n.parent() else { return false };
    let own = |field: &str| parent.child_by_field_name(field).is_some_and(|x| x.id() == n.id());
    match parent.kind() {
        "assignment_expression" | "augmented_assignment_expression" => own("left"),
        "update_expression" => own("argument"),
        "for_in_statement" => own("left"),
        _ => false,
    }
}

crule!(NoCollectionBracketAccess, "js/no-collection-bracket-access", &["subscript_expression"], |n, ctx, p, out| {
    let (Some(object), Some(index)) = (n.child_by_field_name("object"), n.child_by_field_name("index")) else { return };
    let Some(kind) = p.collection(object) else { return };
    match key_values(&p, index, 0) {
        Key::Skip => return,
        Key::Known(values) if values.iter().all(|(sym, v)| if *sym { has_symbol(kind, v) } else { has_member(kind, v) }) => return,
        _ => {}
    }
    let ty = type_name(kind);
    let parent = n.parent();
    let is_delete = parent.is_some_and(|x| x.kind() == "unary_expression" && x.child(0).is_some_and(|k| k.kind() == "delete"));
    let write = !is_delete && is_write_target(n);
    let message = if is_delete {
        format!("`{ty}` entries are not properties: `delete` on brackets does nothing to the entries, use `{ty}#delete()`")
    } else if write {
        format!("`{ty}` entries are not properties: writing through brackets sets a plain property, not an entry")
    } else {
        format!("`{ty}` entries are not properties: reading through brackets does not look an entry up, use `{ty}#get()` or `#has()`")
    };
    let mut edits = Vec::new();
    let recv_ok = matches!(object.kind(), "identifier" | "this" | "member_expression") && !is_optional(n);
    if recv_ok && let Some(par) = parent {
        let obj = text(object, ctx.source);
        let key = if index.kind() == "sequence_expression" { format!("({})", text(index, ctx.source)) } else { text(index, ctx.source).to_owned() };
        if is_delete && par.parent().is_some_and(|g| g.kind() == "expression_statement") && !has_comment(par) {
            edits.push(TextEdit::new(par.start_byte(), par.end_byte(), format!("{obj}.delete({key})")));
        } else if write
            && matches!(kind, MAP | WEAK_MAP)
            && par.kind() == "assignment_expression"
            && par.child_by_field_name("left").zip(par.child_by_field_name("right")).is_some_and(|(l, r)| ctx.source[l.end_byte()..r.start_byte()].trim() == "=")
            && par.parent().is_some_and(|g| g.kind() == "expression_statement")
            && !has_comment(par)
            && let Some(right) = par.child_by_field_name("right")
        {
            let value = if right.kind() == "sequence_expression" { format!("({})", text(right, ctx.source)) } else { text(right, ctx.source).to_owned() };
            edits.push(TextEdit::new(par.start_byte(), par.end_byte(), format!("{obj}.set({key}, {value})")));
        }
    }
    out.push(finding(n, message, edits));
});

// ---- unicorn: no-object-methods-with-collections ---------------------------

crule!(NoObjectMethodsWithCollections, "js/no-object-methods-with-collections", &["call_expression"], |n, ctx, p, out| {
    let Some(name) = static_call(&p, n, "Object").filter(|x| matches!(*x, "entries" | "keys" | "values")) else { return };
    let args = arguments(n);
    let [arg] = args[..] else { return };
    if arg.kind() == "spread_element" {
        return;
    }
    let Some(kind) = p.collection(arg).filter(|k| matches!(*k, MAP | SET)) else { return };
    let ty = type_name(kind);
    let mut edits = Vec::new();
    if !has_comment(n) {
        let recv = member_object_text(arg, ctx.source);
        let mut rep = format!("Array.from({recv}.{name}())");
        let before = ctx.source[..n.start_byte()].chars().next_back();
        let after = ctx.source[n.end_byte()..].chars().next();
        if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
            rep.insert(0, ' ');
        }
        if after.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
            rep.push(' ');
        }
        edits.push(TextEdit::new(n.start_byte(), n.end_byte(), rep));
    }
    let prop = n.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")).unwrap_or(n);
    out.push(finding(prop, format!("`Object.{name}()` does not return the contents of a {ty}: its entries are not own properties"), edits));
});

// ---- unicorn: no-useless-set-construction ----------------------------------

const SET_RETURNING: &[&str] = &["union", "intersection", "difference", "symmetricDifference"];
const SET_METHODS: &[&str] = &["union", "intersection", "difference", "symmetricDifference", "isSubsetOf", "isSupersetOf", "isDisjointFrom"];

fn set_method_call<'t, 's>(src: &'s str, n: Node<'t>, names: &[&str]) -> Option<(Node<'t>, &'s str)> {
    let (recv, name) = method(src, n, names)?;
    (arg_count(n) == 1 && !super::values::has_spread(n)).then_some((recv, name))
}

/// `Set` itself, or what `.keys()`/`.values()` of a `Set` (`.keys()` of a
/// `Map`) iterates: the collection a `new Set(…)` around it only copies.
fn collection_behind<'t>(p: &Probe<'_, 't>, n: Node<'t>) -> Option<Node<'t>> {
    if p.is_set(n) {
        return Some(n);
    }
    let (recv, name) = method(p.src, strip(n), &["keys", "values"])?;
    if arg_count(strip(n)) != 0 {
        return None;
    }
    match p.collection(recv)? {
        SET => Some(recv),
        MAP if name == "keys" => Some(recv),
        _ => None,
    }
}

crule!(NoUselessSetConstruction, "js/no-useless-set-construction", &["new_expression"], |n, ctx, p, out| {
    let Some(c) = n.child_by_field_name("constructor") else { return };
    if !is_global(p.m, c, "Set", p.src) || n.child_by_field_name("type_arguments").is_some() {
        return;
    }
    let args = arguments(n);
    let [arg] = args[..] else { return };
    if arg.kind() == "spread_element" {
        return;
    }
    let arg_s = strip(arg);
    // The node with its own parentheses: what the parent holds.
    let mut outer = n;
    while let Some(par) = outer.parent().filter(|x| x.kind() == "parenthesized_expression") {
        outer = par;
    }
    let mut found: Option<(Node<'_>, &str, bool)> = None;
    if let Some((recv, name)) = set_method_call(p.src, arg_s, SET_RETURNING)
        && p.is_set(recv)
    {
        found = Some((arg_s, name, false));
    }
    if found.is_none()
        && let Some(call) = outer.parent().filter(|x| x.kind() == "arguments").and_then(|a| a.parent())
        && let Some((recv, name)) = set_method_call(p.src, call, SET_METHODS)
        && p.is_set(recv)
        && arguments(call).first().is_some_and(|a| a.id() == outer.id())
        && let Some(rep) = collection_behind(&p, arg_s)
    {
        found = Some((rep, name, false));
    }
    if found.is_none()
        && let Some(member) = outer.parent().filter(|x| x.kind() == "member_expression" && x.child_by_field_name("object").is_some_and(|o| o.id() == outer.id()))
        && let Some(call) = member.parent().filter(|c| c.kind() == "call_expression" && c.child_by_field_name("function").is_some_and(|f| f.id() == member.id()))
        && let Some((_, name)) = set_method_call(p.src, call, SET_METHODS)
        && let Some(rep) = collection_behind(&p, arg_s)
        && p.is_set(rep)
        && let Some(other) = arguments(call).first().copied()
        && p.collection(other).is_some_and(|k| matches!(k, MAP | SET))
        && !has_side_effect(other)
    {
        found = Some((rep, name, true));
    }
    let Some((rep, method_name, as_object)) = found else { return };
    let mut edits = Vec::new();
    let t = if as_object { member_object_text(rep, ctx.source) } else { text(rep, ctx.source).to_owned() };
    let starts_statement = outer.parent().is_some_and(|x| x.kind() == "expression_statement") || {
        let mut top = outer;
        while let Some(x) = top.parent().filter(|x| matches!(x.kind(), "member_expression" | "call_expression" | "subscript_expression") && x.start_byte() == top.start_byte()) {
            top = x;
        }
        top.parent().is_some_and(|x| x.kind() == "expression_statement")
    };
    let hazard = t.starts_with(['(', '[', '`', '+', '-', '/']);
    if !(starts_statement && hazard) && !has_comment(outer) {
        edits.push(TextEdit::new(outer.start_byte(), outer.end_byte(), t));
    }
    out.push(finding(n, format!("This `Set` construction does nothing for `Set#{method_name}()`: it already accepts the collection and does not change its receiver"), edits));
});

// ---- unicorn: prefer-array-iterable-methods --------------------------------

crule!(PreferArrayIterableMethods, "js/prefer-array-iterable-methods", &["for_in_statement"], |n, ctx, p, out| {
    let src = ctx.source;
    if n.children(&mut n.walk()).any(|c| c.kind() == "await") {
        return;
    }
    let (Some(left), Some(right)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return };
    let is_of = src[left.end_byte()..right.start_byte()].contains("of");
    let kind = n.child_by_field_name("kind").map(|k| text(k, src));
    if !is_of || !matches!(kind, Some("const" | "let")) || left.kind() != "array_pattern" {
        return;
    }
    let slots = pattern_slots(left);
    if !(slots.len() == 1 || slots.len() == 2) {
        return;
    }
    let Some((recv, _)) = method(src, right, &["entries"]).filter(|_| arg_count(right) == 0) else { return };
    if !p.is_array(recv) {
        return;
    }
    let unused = |slot: Option<Node<'_>>| match slot {
        None => true,
        Some(x) if x.kind() == "identifier" => match p.m.symbol_at(x.start_byte()) {
            Some(sym) => !p.m.symbol(sym).refs.iter().any(|&r| p.m.references[r].start != x.start_byte()),
            None => false,
        },
        Some(_) => false,
    };
    let index = slots[0];
    let value = slots.get(1).copied().flatten();
    let (iu, vu) = (unused(index), unused(slots.get(1).copied().flatten()));
    if iu == vu {
        return;
    }
    let prop = right.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")).unwrap_or(right);
    let comments = has_comment(left) || has_comment(right);
    // The binding keeps a space from the `const` and `of` around it.
    let spaced = |t: &str| -> String {
        let before = src[..left.start_byte()].chars().next_back();
        let after = src[left.end_byte()..].chars().next();
        let mut s = t.to_owned();
        if before.is_some_and(|c| !c.is_whitespace()) {
            s.insert(0, ' ');
        }
        if after.is_some_and(|c| !c.is_whitespace()) {
            s.push(' ');
        }
        s
    };
    if iu {
        let Some(value) = value.filter(|v| !matches!(v.kind(), "rest_pattern" | "assignment_pattern" | "object_assignment_pattern")) else { return };
        let mut edits = Vec::new();
        if !comments {
            edits.push(TextEdit::new(left.start_byte(), left.end_byte(), spaced(text(value, src))));
            // `.entries()` goes: the receiver stays as the iterable.
            edits.push(TextEdit::delete(recv.end_byte(), right.end_byte()));
        }
        out.push(finding(prop, "The index is unused: iterate the array directly instead of `Array#entries()`", edits));
        return;
    }
    let Some(index) = index.filter(|i| i.kind() == "identifier") else { return };
    let mut edits = Vec::new();
    if !comments {
        edits.push(TextEdit::new(left.start_byte(), left.end_byte(), spaced(text(index, src))));
        edits.push(TextEdit::new(prop.start_byte(), prop.end_byte(), "keys"));
    }
    out.push(finding(prop, "The value is unused: use `Array#keys()` instead of `Array#entries()`", edits));
});

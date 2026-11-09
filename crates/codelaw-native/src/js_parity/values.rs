//! What the syntax says a value is: a built-in `Map`/`Set`, an array, an
//! iterator. Upstream's eslint-plugin-unicorn (v79.0.0) rules ask these
//! questions with type information when a TypeScript program is loaded and
//! with syntax when it is not; the rules built on this module use only the
//! syntax half — a value is known by the constructor or call that made it,
//! by a type annotation naming it, or by a single-assignment `const` whose
//! initializer is one of those. A value this cannot place is "unknown", and
//! the rules built on it say nothing about it.

use crate::js_ast::{self, arguments};
use crate::util::{children, named_children, text};
use codelaw_semantic::{SemanticModel, SymbolId, SymbolKind};
use tree_sitter::Node;

pub(super) const MAP: u8 = 1;
pub(super) const SET: u8 = 2;
pub(super) const WEAK_MAP: u8 = 4;
pub(super) const WEAK_SET: u8 = 8;

/// Everything a value question needs: the model, the text and the root.
#[derive(Clone, Copy)]
pub(super) struct Probe<'a, 't> {
    pub m: &'a SemanticModel,
    pub src: &'a str,
    pub root: Node<'t>,
}

pub(super) fn strip(mut n: Node<'_>) -> Node<'_> {
    while matches!(n.kind(), "parenthesized_expression" | "non_null_expression") {
        match n.named_child(0) {
            Some(x) => n = x,
            None => break,
        }
    }
    n
}

/// A node's inside with parentheses, TypeScript assertions and `!` removed.
pub(super) fn bare(n: Node<'_>) -> Node<'_> {
    crate::js_ast::unparen(n)
}

pub(super) fn has_comment(n: Node<'_>) -> bool {
    n.kind() == "comment" || children(n).iter().any(|c| has_comment(*c))
}

pub(super) fn is_optional(n: Node<'_>) -> bool {
    n.child_by_field_name("optional_chain").is_some() || children(n).iter().any(|c| c.kind() == "optional_chain")
}

/// Whether `n` is an identifier called `name` that no declaration in the
/// file binds: the language's global.
pub(super) fn is_global(m: &SemanticModel, n: Node<'_>, name: &str, src: &str) -> bool {
    n.kind() == "identifier" && text(n, src) == name && m.reference_at(n.start_byte()).is_some_and(|r| r.symbol.is_none())
}

/// `new Name(…)`, `Name` an unshadowed global.
pub(super) fn new_global<'t>(p: &Probe<'_, 't>, n: Node<'t>, names: &[&str]) -> Option<(Node<'t>, String)> {
    let n = bare(n);
    if n.kind() != "new_expression" {
        return None;
    }
    let c = n.child_by_field_name("constructor")?;
    let name = names.iter().find(|x| is_global(p.m, c, x, p.src))?;
    Some((n, (*name).to_owned()))
}

/// `Object.method(…)`/`Array.method(…)`: the call, the method name.
pub(super) fn static_call<'t, 's>(p: &Probe<'s, 't>, n: Node<'t>, object: &str) -> Option<&'s str> {
    let n = bare(n);
    if n.kind() != "call_expression" || is_optional(n) {
        return None;
    }
    let f = n.child_by_field_name("function")?;
    if f.kind() != "member_expression" || is_optional(f) {
        return None;
    }
    let o = f.child_by_field_name("object")?;
    if !is_global(p.m, o, object, p.src) {
        return None;
    }
    f.child_by_field_name("property").filter(|x| x.kind() == "property_identifier").map(|x| text(x, p.src))
}

/// `recv.name(…)` with `name` one of `names`, no optional chaining: the
/// receiver, the method, and the argument list.
pub(super) fn method<'t, 's>(src: &'s str, n: Node<'t>, names: &[&str]) -> Option<(Node<'t>, &'s str)> {
    if n.kind() != "call_expression" || is_optional(n) {
        return None;
    }
    let f = n.child_by_field_name("function")?;
    if f.kind() != "member_expression" || is_optional(f) {
        return None;
    }
    let p = f.child_by_field_name("property").filter(|x| x.kind() == "property_identifier")?;
    let name = text(p, src);
    names.contains(&name).then_some(())?;
    Some((f.child_by_field_name("object")?, name))
}

pub(super) fn arg_count(call: Node<'_>) -> usize {
    arguments(call).len()
}

pub(super) fn has_spread(call: Node<'_>) -> bool {
    arguments(call).iter().any(|a| a.kind() == "spread_element")
}

/// The text of `n` as the object of a member access: parenthesized when it
/// is not a primary expression.
pub(super) fn member_object_text(n: Node<'_>, src: &str) -> String {
    let t = text(n, src);
    let simple = match n.kind() {
        "identifier" | "member_expression" | "subscript_expression" | "call_expression" | "this" | "parenthesized_expression" | "array" | "string" | "template_string" | "non_null_expression" | "super" => true,
        "new_expression" => n.child_by_field_name("arguments").is_some(),
        _ => false,
    };
    if simple { t.to_owned() } else { format!("({t})") }
}

/// Whether an expression, evaluated, could do something besides compute a
/// value (a getter read is not counted).
pub(super) fn has_side_effect(n: Node<'_>) -> bool {
    match n.kind() {
        "call_expression" | "new_expression" | "assignment_expression" | "augmented_assignment_expression" | "update_expression" | "await_expression" | "yield_expression" | "tagged_template_expression" => true,
        "unary_expression" if n.child(0).is_some_and(|c| c.kind() == "delete") => true,
        k if js_ast::FUNCTION_KINDS.contains(&k) || k == "class" => false,
        _ => named_children(n).iter().any(|c| has_side_effect(*c)),
    }
}

// ---- type annotations -----------------------------------------------------

/// The bits a type annotation names when every member of it is a built-in
/// collection (`Map<K, V>`, `ReadonlySet<T>`, a union of them); a name a
/// declaration in the file also binds is the user's type, not the built-in.
fn collection_annotation(p: &Probe<'_, '_>, t: Node<'_>) -> Option<u8> {
    match t.kind() {
        "type_annotation" | "parenthesized_type" => collection_annotation(p, t.named_child(0)?),
        "union_type" => {
            let mut bits = 0;
            for c in named_children(t) {
                bits |= collection_annotation(p, c)?;
            }
            (bits != 0).then_some(bits)
        }
        "generic_type" | "type_identifier" => {
            let name = if t.kind() == "generic_type" { t.child_by_field_name("name")? } else { t };
            if name.kind() != "type_identifier" || p.m.reference_at(name.start_byte()).is_some_and(|r| r.symbol.is_some()) {
                return None;
            }
            match text(name, p.src) {
                "Map" | "ReadonlyMap" => Some(MAP),
                "Set" | "ReadonlySet" => Some(SET),
                "WeakMap" => Some(WEAK_MAP),
                "WeakSet" => Some(WEAK_SET),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether every member of a type annotation is an array or a tuple type.
fn array_annotation(p: &Probe<'_, '_>, t: Node<'_>) -> bool {
    match t.kind() {
        "type_annotation" | "parenthesized_type" | "readonly_type" => t.named_child(0).is_some_and(|c| array_annotation(p, c)),
        "union_type" => {
            let cs = named_children(t);
            !cs.is_empty() && cs.into_iter().all(|c| array_annotation(p, c))
        }
        "array_type" | "tuple_type" => true,
        "generic_type" => {
            let Some(name) = t.child_by_field_name("name") else { return false };
            name.kind() == "type_identifier" && matches!(text(name, p.src), "Array" | "ReadonlyArray") && p.m.reference_at(name.start_byte()).is_none_or(|r| r.symbol.is_none())
        }
        _ => false,
    }
}

/// Whether a type annotation is made only of built-in iterator types.
fn iterator_annotation(p: &Probe<'_, '_>, t: Node<'_>) -> bool {
    match t.kind() {
        "type_annotation" | "parenthesized_type" => t.named_child(0).is_some_and(|c| iterator_annotation(p, c)),
        "union_type" => {
            let cs: Vec<Node<'_>> = named_children(t).into_iter().filter(|c| !matches!(c.kind(), "undefined" | "null" | "predefined_type")).collect();
            !cs.is_empty() && cs.into_iter().all(|c| iterator_annotation(p, c))
        }
        "generic_type" | "type_identifier" => {
            let name = if t.kind() == "generic_type" { t.child_by_field_name("name") } else { Some(t) };
            let Some(name) = name.filter(|x| x.kind() == "type_identifier") else { return false };
            const NAMES: &[&str] = &["ArrayIterator", "Generator", "IterableIterator", "Iterator", "IteratorObject", "MapIterator", "RegExpStringIterator", "SetIterator", "StringIterator"];
            NAMES.contains(&text(name, p.src)) && p.m.reference_at(name.start_byte()).is_none_or(|r| r.symbol.is_none())
        }
        _ => false,
    }
}

// ---- bindings -------------------------------------------------------------

/// The binding a symbol was declared by, when it has exactly one: the
/// declaring identifier.
fn single_binding<'t>(p: &Probe<'_, 't>, ident: Node<'_>) -> Option<(SymbolId, Node<'t>)> {
    let sym = p.m.symbol_at(ident.start_byte())?;
    let s = p.m.symbol(sym);
    if s.decls.len() != 1 {
        return None;
    }
    let (a, b) = s.decls[0];
    Some((sym, p.root.descendant_for_byte_range(a, b)?))
}

/// The annotation on the declaring identifier of a variable or a parameter.
fn annotation_of<'t>(id: Node<'t>) -> Option<Node<'t>> {
    let parent = id.parent()?;
    let own = |field: &str| parent.child_by_field_name(field).is_some_and(|x| x.id() == id.id());
    match parent.kind() {
        "variable_declarator" if own("name") => parent.child_by_field_name("type"),
        "required_parameter" | "optional_parameter" if own("pattern") => parent.child_by_field_name("type"),
        _ => None,
    }
}

/// The initializer of a `const x = …` binding (not a destructured one).
pub(super) fn const_init<'t>(id: Node<'t>, src: &str) -> Option<Node<'t>> {
    let d = id.parent().filter(|d| d.kind() == "variable_declarator" && d.child_by_field_name("name").is_some_and(|n| n.id() == id.id()))?;
    let decl = d.parent().filter(|x| x.kind() == "lexical_declaration")?;
    (decl.child(0).is_some_and(|k| text(k, src) == "const")).then(|| d.child_by_field_name("value")).flatten()
}

impl<'a, 't> Probe<'a, 't> {
    pub fn new(m: &'a SemanticModel, src: &'a str, root: Node<'t>) -> Self {
        Probe { m, src, root }
    }

    /// The built-in collection kinds a value may be (`MAP`, `SET`, …), or
    /// `None` when it is not known to be one.
    pub fn collections(&self, n: Node<'t>) -> Option<u8> {
        self.collections_in(n, &mut Vec::new())
    }

    fn collections_in(&self, n: Node<'t>, seen: &mut Vec<SymbolId>) -> Option<u8> {
        let n = strip(n);
        if let Some((_, name)) = new_global(self, n, &["Map", "Set", "WeakMap", "WeakSet"]) {
            return Some(match name.as_str() {
                "Map" => MAP,
                "Set" => SET,
                "WeakMap" => WEAK_MAP,
                _ => WEAK_SET,
            });
        }
        match n.kind() {
            "as_expression" | "satisfies_expression" => {
                let ty = n.named_child(1);
                ty.and_then(|t| collection_annotation(self, t)).or_else(|| self.collections_in(n.named_child(0)?, seen))
            }
            "type_assertion" => named_children(n).first().and_then(|t| collection_annotation(self, *t)).or_else(|| self.collections_in(named_children(n).last().copied()?, seen)),
            "ternary_expression" => {
                let a = self.collections_in(n.child_by_field_name("consequence")?, seen)?;
                let b = self.collections_in(n.child_by_field_name("alternative")?, seen)?;
                Some(a | b)
            }
            "identifier" => {
                let (sym, id) = single_binding(self, n)?;
                if seen.contains(&sym) {
                    return None;
                }
                seen.push(sym);
                let out = annotation_of(id).and_then(|t| collection_annotation(self, t)).or_else(|| {
                    if self.m.symbol(sym).kind != SymbolKind::Const {
                        return None;
                    }
                    self.collections_in(const_init(id, self.src)?, seen)
                });
                seen.pop();
                out
            }
            _ => None,
        }
    }

    /// Exactly one built-in collection kind.
    pub fn collection(&self, n: Node<'t>) -> Option<u8> {
        self.collections(n).filter(|b| b.count_ones() == 1)
    }

    pub fn is_set(&self, n: Node<'t>) -> bool {
        self.collection(n) == Some(SET)
    }

    /// Whether a value is known to be an array (or a tuple).
    pub fn is_array(&self, n: Node<'t>) -> bool {
        self.is_array_in(n, &mut Vec::new())
    }

    fn is_array_in(&self, n: Node<'t>, seen: &mut Vec<SymbolId>) -> bool {
        let n = strip(n);
        match n.kind() {
            "array" => return true,
            "as_expression" | "satisfies_expression" => {
                return n.named_child(1).is_some_and(|t| array_annotation(self, t)) || n.named_child(0).is_some_and(|x| self.is_array_in(x, seen));
            }
            "call_expression" | "new_expression" => {
                let callee = n.child_by_field_name(if n.kind() == "call_expression" { "function" } else { "constructor" });
                if callee.is_some_and(|c| is_global(self.m, c, "Array", self.src)) {
                    return true;
                }
                if n.kind() == "call_expression" {
                    if let Some(name) = static_call(self, n, "Array") {
                        return matches!(name, "from" | "of");
                    }
                    if let Some(name) = static_call(self, n, "Object") {
                        return matches!(name, "keys" | "values" | "entries" | "getOwnPropertyNames" | "getOwnPropertySymbols");
                    }
                    if let Some((recv, name)) = method(self.src, n, &["copyWithin", "fill", "reverse", "sort", "toReversed", "toSorted", "toSpliced", "with"]) {
                        let _ = name;
                        return self.is_array_in(recv, seen);
                    }
                }
                return false;
            }
            "identifier" => {}
            _ => return false,
        }
        let Some((sym, id)) = single_binding(self, n) else { return false };
        if seen.contains(&sym) {
            return false;
        }
        seen.push(sym);
        let mut out = annotation_of(id).is_some_and(|t| array_annotation(self, t));
        if !out && annotation_of(id).is_none() {
            if let Some(p) = id.parent().filter(|p| p.kind() == "rest_pattern") {
                out = p.parent().is_some_and(|x| matches!(x.kind(), "required_parameter" | "optional_parameter")) && !self.m.is_reassigned(sym);
            } else if self.m.symbol(sym).kind == SymbolKind::Const {
                out = const_init(id, self.src).is_some_and(|init| self.is_array_in(init, seen));
            }
        }
        seen.pop();
        out
    }

    // ---- iterators ----------------------------------------------------------

    /// Whether a value is known to be an iterator with the Iterator Helpers
    /// methods: what `.values()`, `.keys()`, `.entries()` or `.matchAll()`
    /// returns, an `Iterator.from/concat/zip` result, a helper chain on
    /// one, a call of a local synchronous generator, or a `const`/parameter
    /// whose annotation names an iterator type.
    pub fn is_iterator(&self, n: Node<'t>) -> bool {
        self.is_iterator_in(n, &mut Vec::new())
    }

    pub fn is_lazy_helper_call(&self, n: Node<'t>) -> bool {
        self.helper_in(n, &mut Vec::new())
    }

    fn helper_in(&self, n: Node<'t>, seen: &mut Vec<usize>) -> bool {
        let Some((recv, _)) = method(self.src, n, &["drop", "filter", "flatMap", "map", "take"]) else { return false };
        arg_count(n) >= 1 && self.is_iterator_in(recv, seen)
    }

    fn is_iterator_in(&self, n: Node<'t>, seen: &mut Vec<usize>) -> bool {
        let n = bare(n);
        if seen.contains(&n.id()) {
            return false;
        }
        seen.push(n.id());
        if n.kind() == "call_expression" {
            if let Some(f) = n.child_by_field_name("function")
                && f.kind() == "member_expression"
                && !is_optional(f)
                && let Some(o) = f.child_by_field_name("object")
                && let Some(pn) = f.child_by_field_name("property").filter(|x| x.kind() == "property_identifier")
                && matches!(text(pn, self.src), "concat" | "from" | "zip" | "zipKeyed")
                && self.is_global_iterator(o)
            {
                return true;
            }
            if method(self.src, n, &["entries", "keys", "values"]).is_some() && arg_count(n) == 0 {
                return true;
            }
            if method(self.src, n, &["matchAll"]).is_some() && arg_count(n) <= 1 {
                return true;
            }
            if self.helper_in(n, seen) {
                return true;
            }
            if !is_optional(n)
                && let Some(callee) = n.child_by_field_name("function")
                && self.is_sync_generator(callee, &mut Vec::new())
            {
                return true;
            }
        }
        if n.kind() == "identifier"
            && let Some((sym, id)) = single_binding(self, n)
            && !self.m.is_reassigned(sym)
            && self.m.symbol(sym).kind == SymbolKind::Const
            && annotation_of(id).is_none()
            && let Some(init) = const_init(id, self.src)
            && self.is_iterator_in(init, seen)
        {
            return true;
        }
        self.iterator_typed(n)
    }

    fn is_global_iterator(&self, n: Node<'t>) -> bool {
        let n = bare(n);
        if is_global(self.m, n, "Iterator", self.src) {
            return true;
        }
        n.kind() == "member_expression"
            && !is_optional(n)
            && n.child_by_field_name("property").is_some_and(|p| text(p, self.src) == "Iterator")
            && n.child_by_field_name("object").is_some_and(|o| is_global(self.m, bare(o), "globalThis", self.src))
    }

    fn is_sync_generator(&self, n: Node<'t>, seen: &mut Vec<usize>) -> bool {
        let mut cur = Some(bare(n));
        while let Some(x) = cur {
            if seen.contains(&x.id()) {
                return false;
            }
            seen.push(x.id());
            match x.kind() {
                "generator_function_declaration" | "generator_function" => return !js_ast::is_async(x),
                "function_declaration" | "function_expression" | "function" => return false,
                "identifier" => {
                    let Some((sym, id)) = single_binding(self, x) else { return false };
                    if self.m.is_reassigned(sym) {
                        return false;
                    }
                    cur = match id.parent() {
                        Some(d) if matches!(d.kind(), "function_declaration" | "generator_function_declaration") && d.child_by_field_name("name").is_some_and(|nm| nm.id() == id.id()) => Some(d),
                        _ if self.m.symbol(sym).kind == SymbolKind::Const && annotation_of(id).is_none() => const_init(id, self.src),
                        _ => None,
                    };
                }
                _ => return false,
            }
        }
        false
    }

    fn iterator_typed(&self, n: Node<'t>) -> bool {
        match n.kind() {
            "identifier" => single_binding(self, n).is_some_and(|(_, id)| annotation_of(id).is_some_and(|t| iterator_annotation(self, t))),
            _ => false,
        }
    }

    /// An iterator value, looking through TypeScript assertions that name an
    /// iterator type.
    pub fn is_iterator_loose(&self, n: Node<'t>) -> bool {
        let s = strip(n);
        if matches!(s.kind(), "as_expression" | "satisfies_expression") && s.named_child(1).is_some_and(|t| iterator_annotation(self, t)) && !self.is_array(s) {
            return true;
        }
        self.is_iterator(n)
    }
}

/// The slots of an array pattern: `None` for an elision.
pub(super) fn pattern_slots<'t>(n: Node<'t>) -> Vec<Option<Node<'t>>> {
    let mut slots = Vec::new();
    let mut pending: Option<Option<Node<'t>>> = None;
    for c in children(n) {
        match c.kind() {
            "[" | "]" | "comment" => {}
            "," => {
                slots.push(pending.take().unwrap_or(None));
            }
            _ => pending = Some(Some(c)),
        }
    }
    if let Some(Some(last)) = pending {
        slots.push(Some(last));
    }
    slots
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flags_are_distinct() {
        assert_eq!(MAP | SET | WEAK_MAP | WEAK_SET, 15);
    }
}

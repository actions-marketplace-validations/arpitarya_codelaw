//! W-139 strand W-139-js, the scope rows: rules from ESLint and
//! eslint-plugin-unicorn that ask where a binding is declared, read or
//! written. Each reads the per-file semantic model (scopes, symbols and
//! references with their read/write flags) and the syntax tree; none follows
//! a value through a call or across files. Upstream behaviour was read from
//! the rule's source and tests at the pinned version (ESLint 10.13.0,
//! unicorn v79.0.0) and reimplemented.

use super::{JS, finding};
use crate::js_ast::{self, arguments, chain_root, unparen};
use crate::util::{children, named_children, remove_list_item, remove_statement, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{Reference, ScopeKind, SemanticModel, SymbolId, SymbolKind};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tree_sitter::Node;

macro_rules! srule {
    ($name:ident, $id:literal, $kinds:expr, |$n:ident, $ctx:ident, $m:ident, $out:ident| $body:block) => {
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
                let Some($m) = $ctx.semantic else { return };
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}

// ---- shared helpers ------------------------------------------------------

pub(super) fn ancestors<'t>(n: Node<'t>) -> impl Iterator<Item = Node<'t>> {
    std::iter::successors(n.parent(), |p| p.parent())
}

pub(super) fn fn_like(kind: &str) -> bool {
    js_ast::FUNCTION_KINDS.contains(&kind)
}

fn is_generator(kind: &str) -> bool {
    matches!(kind, "generator_function" | "generator_function_declaration")
}

/// The nearest function around `n` (arrows included).
pub(super) fn enclosing_fn(n: Node<'_>) -> Option<Node<'_>> {
    ancestors(n).find(|a| fn_like(a.kind()))
}

fn within(n: Node<'_>, outer: Node<'_>) -> bool {
    n.start_byte() >= outer.start_byte() && n.end_byte() <= outer.end_byte()
}

fn in_range(pos: usize, outer: Node<'_>) -> bool {
    pos >= outer.start_byte() && pos < outer.end_byte()
}

fn same(a: Option<Node<'_>>, b: Node<'_>) -> bool {
    a.is_some_and(|a| a.id() == b.id())
}

fn node_at<'t>(root: Node<'t>, start: usize, end: usize) -> Option<Node<'t>> {
    root.descendant_for_byte_range(start, end)
}

fn refs_of(m: &SemanticModel, sym: SymbolId) -> Vec<&Reference> {
    m.symbol(sym).refs.iter().map(|&r| &m.references[r]).collect()
}

/// The function a symbol is declared in (`None` at module level).
fn decl_fn<'t>(root: Node<'t>, m: &SemanticModel, sym: SymbolId) -> Option<Node<'t>> {
    let (s, e) = *m.symbol(sym).decls.first()?;
    enclosing_fn(node_at(root, s, e)?)
}

/// The descendants of `n` in source order, not entering functions or
/// classes below it.
fn own_descendants(n: Node<'_>) -> Vec<Node<'_>> {
    let mut out = Vec::new();
    let mut stack = named_children(n);
    stack.reverse();
    while let Some(x) = stack.pop() {
        out.push(x);
        if !fn_like(x.kind()) && !matches!(x.kind(), "class" | "class_declaration") {
            let mut c = named_children(x);
            c.reverse();
            stack.extend(c);
        }
    }
    out
}

fn member_prop<'s>(m: Node<'_>, src: &'s str) -> Option<&'s str> {
    match m.kind() {
        "member_expression" => m.child_by_field_name("property").filter(|p| p.kind() == "property_identifier").map(|p| text(p, src)),
        "subscript_expression" => {
            let i = m.child_by_field_name("index")?;
            (i.kind() == "string").then(|| crate::util::unquote(text(i, src)))
        }
        _ => None,
    }
}

fn is_optional(m: Node<'_>) -> bool {
    m.child_by_field_name("optional_chain").is_some() || children(m).iter().any(|c| c.kind() == "optional_chain")
}

/// `recv.method(…)`: the receiver, when the call is a non-optional method
/// call of that name.
fn method_call<'t, 'a>(call: Node<'t>, names: &[&'a str], src: &str) -> Option<(Node<'t>, &'a str)> {
    if call.kind() != "call_expression" {
        return None;
    }
    let f = call.child_by_field_name("function")?;
    if !matches!(f.kind(), "member_expression" | "subscript_expression") || is_optional(f) || is_optional(call) {
        return None;
    }
    let p = member_prop(f, src)?;
    let name = *names.iter().find(|n| **n == p)?;
    Some((f.child_by_field_name("object")?, name))
}

fn callee_is(call: Node<'_>, obj: &str, prop: &str, src: &str) -> bool {
    call.kind() == "call_expression"
        && call.child_by_field_name("function").is_some_and(|f| f.kind() == "member_expression" && f.child_by_field_name("object").is_some_and(|o| text(o, src) == obj) && member_prop(f, src) == Some(prop))
}

/// Whether `ident` names a global the language defines (not shadowed).
fn is_builtin(m: &SemanticModel, ident: Node<'_>) -> bool {
    ident.kind() == "identifier" && m.reference_at(ident.start_byte()).is_some_and(|r| m.is_builtin_reference(r))
}

fn first_arg(call: Node<'_>) -> Option<Node<'_>> {
    arguments(call).into_iter().next()
}

fn is_callee(n: Node<'_>) -> bool {
    n.parent().is_some_and(|p| match p.kind() {
        "call_expression" => same(p.child_by_field_name("function"), n),
        "new_expression" => same(p.child_by_field_name("constructor"), n),
        _ => false,
    })
}

fn static_value(n: Node<'_>, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "string" => Some(format!("s:{}", crate::util::unquote(text(n, src)))),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Some(format!("s:{}", text(n, src).trim_matches('`'))),
        "number" => text(n, src).replace('_', "").parse::<f64>().ok().map(|v| format!("n:{v}")),
        "true" | "false" | "null" => Some(text(n, src).to_owned()),
        _ => None,
    }
}

// ---- unicorn: no-this-outside-of-class ------------------------------------

fn has_this_param(f: Node<'_>) -> bool {
    js_ast::params(f).first().is_some_and(|p| p.kind() == "required_parameter" && p.child_by_field_name("pattern").is_some_and(|x| x.kind() == "this"))
}

srule!(NoThisOutsideOfClass, "js/no-this-outside-of-class", &["this"], |n, _ctx, _m, out| {
    if n.parent().is_some_and(|p| p.kind() == "required_parameter") || ancestors(n).any(|a| a.kind() == "type_query") {
        return;
    }
    let mut child = n;
    for a in ancestors(n) {
        match a.kind() {
            "arrow_function" => {}
            "class_static_block" => return,
            "field_definition" | "public_field_definition" => {
                if same(a.child_by_field_name("value"), child) {
                    return;
                }
            }
            "method_definition" => {
                if a.parent().is_some_and(|p| p.kind() == "class_body") {
                    return;
                }
                break;
            }
            k if fn_like(k) => {
                if has_this_param(a) {
                    return;
                }
                break;
            }
            _ => {}
        }
        child = a;
    }
    out.push(finding(n, "`this` outside a class method is whatever the caller binds: pass the object as a parameter, or move the code into a class", vec![]));
});

// ---- unicorn: no-top-level-assignment-in-function ------------------------

srule!(NoTopLevelAssignmentInFunction, "js/no-top-level-assignment-in-function", &["program"], |n, _ctx, m, out| {
    for r in &m.references {
        let Some(sym) = r.symbol.filter(|_| r.write) else { continue };
        let s = m.symbol(sym);
        if m.scopes[s.scope].kind != ScopeKind::Module || s.decls.is_empty() {
            continue;
        }
        let Some(id) = node_at(n, r.start, r.end) else { continue };
        for a in ancestors(id) {
            match a.kind() {
                "class_static_block" | "field_definition" | "public_field_definition" => break,
                k if fn_like(k) => {
                    out.push(finding(id, format!("`{}` is a module-level variable assigned inside a function: return the value, or keep the state in an object you pass", r.name), vec![]));
                    break;
                }
                _ => {}
            }
        }
    }
});

// ---- unicorn: no-useless-recursion ---------------------------------------

srule!(NoUselessRecursion, "js/no-useless-recursion", &["return_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(arg) = named_children(n).into_iter().find(|c| c.kind() != "comment") else { return };
    let call = unparen(arg);
    if call.kind() != "call_expression" || is_optional(call) {
        return;
    }
    let Some(callee) = call.child_by_field_name("function").filter(|c| c.kind() == "identifier") else { return };
    let Some(f) = enclosing_fn(n) else { return };
    if !matches!(f.kind(), "function_declaration" | "function_expression" | "function") || is_generator(f.kind()) {
        return;
    }
    let Some(name) = f.child_by_field_name("name") else { return };
    if text(name, src) != text(callee, src) || ancestors(n).take_while(|a| a.id() != f.id()).any(|a| a.kind() == "try_statement") {
        return;
    }
    let (a, b) = (m.symbol_at(callee.start_byte()), m.symbol_at(name.start_byte()));
    if a.is_some() && a == b {
        out.push(finding(callee, format!("`{}` returns a call to itself: write the recursion as a loop, which can't overflow the stack", text(callee, src)), vec![]));
    }
});

// ---- unicorn: no-useless-re-export ---------------------------------------

fn export_source(e: Node<'_>, src: &str) -> Option<String> {
    let s = e.child_by_field_name("source")?;
    let attrs: String = named_children(e).iter().filter(|c| c.kind() == "import_attribute").map(|c| text(*c, src)).collect();
    Some(format!("{}{attrs}", crate::util::unquote(text(s, src))))
}

srule!(NoUselessReExport, "js/no-useless-re-export", &["program"], |n, ctx, _m, out| {
    let src = ctx.source;
    let exports: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() == "export_statement").collect();
    let mut wild = BTreeSet::new();
    for e in &exports {
        let star = children(*e).iter().any(|c| c.kind() == "*");
        let ns = named_children(*e).iter().any(|c| c.kind() == "namespace_export");
        if star && !ns && let Some(k) = export_source(*e, src) {
            wild.insert(k);
        }
    }
    let wild: Vec<String> = wild.into_iter().collect();
    let [key] = &wild[..] else { return };
    for e in exports {
        let Some(clause) = named_children(e).into_iter().find(|c| c.kind() == "export_clause") else { continue };
        if export_source(e, src).as_deref() != Some(key.as_str()) || children(e).iter().any(|c| c.kind() == "type") {
            continue;
        }
        let specs: Vec<Node<'_>> = named_children(clause).into_iter().filter(|s| s.kind() == "export_specifier").collect();
        let redundant: Vec<Node<'_>> = specs
            .iter()
            .copied()
            .filter(|s| s.child_by_field_name("alias").is_none() && s.child_by_field_name("name").is_some_and(|x| text(x, src) != "default") && !children(*s).iter().any(|c| c.kind() == "type"))
            .collect();
        if redundant.is_empty() {
            continue;
        }
        if redundant.len() == specs.len() {
            let names: Vec<&str> = redundant.iter().map(|s| text(*s, src)).collect();
            out.push(finding(e, format!("`export * from '{key}'` already exports {}: remove this re-export", names.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(", ")), vec![remove_statement(e, src)]));
        } else {
            for s in redundant {
                out.push(finding(s, format!("`export * from '{key}'` already exports `{}`: remove it here", text(s, src)), vec![remove_list_item(s, src)]));
            }
        }
    }
});

// ---- unicorn: no-undeclared-class-members --------------------------------

fn is_static(member: Node<'_>) -> bool {
    children(member).iter().any(|c| c.kind() == "static")
}

fn key_name(key: Node<'_>, src: &str) -> Option<String> {
    match key.kind() {
        "property_identifier" | "private_property_identifier" => Some(text(key, src).to_owned()),
        "string" => Some(crate::util::unquote(text(key, src)).to_owned()),
        "number" => Some(text(key, src).to_owned()),
        "computed_property_name" => key.named_child(0).filter(|k| k.kind() == "string").map(|k| crate::util::unquote(text(k, src)).to_owned()),
        _ => None,
    }
}

fn member_key(member: Node<'_>) -> Option<Node<'_>> {
    member.child_by_field_name("name").or_else(|| member.child_by_field_name("property"))
}

fn class_body(class: Node<'_>) -> Option<Node<'_>> {
    class.child_by_field_name("body")
}

fn class_members(class: Node<'_>) -> Vec<Node<'_>> {
    class_body(class).map(|b| named_children(b).into_iter().filter(|c| c.kind() != "comment").collect()).unwrap_or_default()
}

fn has_heritage(class: Node<'_>) -> bool {
    named_children(class).iter().any(|c| c.kind() == "class_heritage")
}

/// The method a `this` belongs to: the nearest non-arrow function.
fn this_owner(this: Node<'_>) -> Option<Node<'_>> {
    ancestors(this).find(|a| (fn_like(a.kind()) && a.kind() != "arrow_function") || matches!(a.kind(), "class_static_block" | "field_definition" | "public_field_definition"))
}

srule!(NoUndeclaredClassMembers, "js/no-undeclared-class-members", &["class_declaration", "class"], |n, ctx, _m, out| {
    let src = ctx.source;
    if has_heritage(n) {
        return;
    }
    let Some(body) = class_body(n) else { return };
    let members = class_members(n);
    let mut declared: BTreeSet<String> = BTreeSet::from(["constructor".to_owned()]);
    for mem in &members {
        if let Some(k) = member_key(*mem).and_then(|k| key_name(k, src))
            && !is_static(*mem)
        {
            declared.insert(k);
        }
        let is_ctor = mem.kind() == "method_definition" && member_key(*mem).is_some_and(|k| text(k, src) == "constructor");
        if !is_ctor {
            continue;
        }
        for p in js_ast::params(*mem) {
            if p.kind() == "required_parameter" && children(p).iter().any(|c| matches!(c.kind(), "accessibility_modifier" | "readonly")) && let Some(x) = js_ast::param_name(p, src) {
                declared.insert(x.to_owned());
            }
        }
        if let Some(b) = js_ast::body(*mem) {
            for d in own_descendants(b) {
                if d.kind() == "assignment_expression"
                    && let Some(l) = d.child_by_field_name("left")
                    && l.kind() == "member_expression"
                    && l.child_by_field_name("object").is_some_and(|o| o.kind() == "this")
                    && let Some(p) = member_prop(l, src)
                {
                    declared.insert(p.to_owned());
                }
            }
        }
    }
    let first = members.first().copied();
    let mut suggested = BTreeSet::new();
    let mut stack = vec![body];
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if matches!(c.kind(), "class" | "class_declaration") {
                continue;
            }
            stack.push(c);
            if c.kind() != "member_expression" || !c.child_by_field_name("object").is_some_and(|o| o.kind() == "this") {
                continue;
            }
            let Some(p) = c.child_by_field_name("property").filter(|p| p.kind() == "property_identifier") else { continue };
            let name = text(p, src);
            if declared.contains(name) {
                continue;
            }
            let this = c.child_by_field_name("object").unwrap_or(c);
            let Some(owner) = this_owner(this) else { continue };
            if owner.kind() != "method_definition" || is_static(owner) || !owner.parent().is_some_and(|b| b.id() == body.id()) {
                continue;
            }
            let mut edits = Vec::new();
            let simple = c.parent().is_some_and(|a| a.kind() == "assignment_expression" && same(a.child_by_field_name("left"), c));
            if simple
                && let Some(f) = first
                && f.start_position().row > body.start_position().row
                && suggested.insert(name.to_owned())
            {
                let line_start = src[..f.start_byte()].rfind('\n').map_or(0, |i| i + 1);
                let indent = &src[line_start..f.start_byte()];
                if indent.trim().is_empty() {
                    edits.push(TextEdit::insert(line_start, format!("{indent}{name};\n")));
                }
            }
            out.push(finding(c, format!("`this.{name}` is not declared in the class: declare `{name}` as a field"), edits));
        }
    }
});

// ---- unicorn: no-incomplete-accessor-override ----------------------------

#[derive(Default, Clone, Copy)]
struct Desc {
    get: bool,
    set: bool,
    data: bool,
}

fn accessor_kind(member: Node<'_>) -> Option<&'static str> {
    if member.kind() != "method_definition" {
        return None;
    }
    children(member).iter().find_map(|c| match c.kind() {
        "get" => Some("get"),
        "set" => Some("set"),
        _ => None,
    })
}

/// Each statically named member's descriptor, by (name, static).
fn descriptors(class: Node<'_>, src: &str) -> Option<HashMap<(String, bool), Desc>> {
    let mut out: HashMap<(String, bool), Desc> = HashMap::new();
    for mem in class_members(class) {
        if mem.kind() == "decorator" {
            return None;
        }
        let Some(key) = member_key(mem) else { continue };
        if key.kind() == "private_property_identifier" {
            continue;
        }
        let Some(name) = key_name(key, src) else {
            if key.kind() == "computed_property_name" {
                return None;
            }
            continue;
        };
        let d = out.entry((name, is_static(mem))).or_default();
        match accessor_kind(mem) {
            Some("get") => d.get = true,
            Some("set") => d.set = true,
            _ if children(mem).iter().any(|c| c.kind() == "accessor") => {
                d.get = true;
                d.set = true;
            }
            _ => d.data = true,
        }
    }
    Some(out)
}

/// The class a `class X extends <expr>` names, in this file: a class
/// declaration, or a `const` bound to one.
fn superclass<'t>(class: Node<'t>, root: Node<'t>, m: &SemanticModel) -> Option<Node<'t>> {
    let heritage = named_children(class).into_iter().find(|c| c.kind() == "class_heritage")?;
    let ext = named_children(heritage).into_iter().find(|c| c.kind() == "extends_clause").map_or(heritage.named_child(0), |e| e.child_by_field_name("value").or(e.named_child(0)))?;
    let mut ident = unparen(ext);
    for _ in 0..8 {
        if ident.kind() != "identifier" {
            return None;
        }
        let sym = m.symbol_at(ident.start_byte())?;
        let s = m.symbol(sym);
        if s.decls.len() != 1 || refs_of(m, sym).iter().any(|r| r.write) {
            return None;
        }
        let (a, b) = s.decls[0];
        let id = node_at(root, a, b)?;
        let p = id.parent()?;
        match p.kind() {
            "class_declaration" => return Some(p),
            "variable_declarator" if s.kind == SymbolKind::Const => {
                let v = unparen(p.child_by_field_name("value")?);
                if v.kind() == "class" {
                    return Some(v);
                }
                ident = v;
            }
            _ => return None,
        }
    }
    None
}

srule!(NoIncompleteAccessorOverride, "js/no-incomplete-accessor-override", &["class_declaration", "class"], |n, ctx, m, out| {
    let src = ctx.source;
    if !has_heritage(n) || children(n).iter().any(|c| c.kind() == "decorator" || c.kind() == "declare") {
        return;
    }
    let root = js_ast::root_of(n);
    let Some(own) = descriptors(n, src) else { return };
    let mut reported = BTreeSet::new();
    for mem in class_members(n) {
        let Some(kind) = accessor_kind(mem) else { continue };
        let Some(key) = member_key(mem) else { continue };
        let Some(name) = key_name(key, src) else { continue };
        let st = is_static(mem);
        if (!st && name == "constructor") || (st && (name == "name" || name == "length")) {
            continue;
        }
        let Some(d) = own.get(&(name.clone(), st)) else { continue };
        if d.data || (d.get && d.set) || !reported.insert((name.clone(), st)) {
            continue;
        }
        let mut cur = n;
        for _ in 0..16 {
            let Some(up) = superclass(cur, root, m) else { break };
            if children(up).iter().any(|c| c.kind() == "decorator") {
                break;
            }
            let Some(ds) = descriptors(up, src) else { break };
            if let Some(a) = ds.get(&(name.clone(), st)) {
                let opposite = if kind == "get" { a.set } else { a.get };
                if !a.data && opposite {
                    let other = if kind == "get" { "set" } else { "get" };
                    out.push(finding(key, format!("`{kind} {name}` overrides half of an inherited accessor pair: the inherited `{other} {name}` is hidden too; define both here"), vec![]));
                }
                break;
            }
            cur = up;
        }
    }
});

// ---- unicorn: no-late-current-target-access / no-late-event-control ------

fn event_name(name: &str) -> bool {
    matches!(name, "e" | "event" | "evt") || (name.ends_with("Event") && name.len() > 5 && name.starts_with(|c: char| c.is_ascii_lowercase()))
}

/// The function whose parameter `ident` names, when it names an event
/// parameter: by its name, or a TypeScript annotation ending in `Event`.
fn event_param<'t>(ident: Node<'t>, root: Node<'t>, m: &SemanticModel, src: &str) -> Option<Node<'t>> {
    if ident.kind() != "identifier" {
        return None;
    }
    let sym = m.symbol_at(ident.start_byte())?;
    let s = m.symbol(sym);
    if s.kind != SymbolKind::Param {
        return None;
    }
    let (a, b) = *s.decls.first()?;
    let decl = node_at(root, a, b)?;
    let annotation = decl.parent().filter(|p| p.kind() == "required_parameter" || p.kind() == "optional_parameter").and_then(|p| p.child_by_field_name("type")).map(|t| text(t, src).trim_start_matches(':').trim().to_owned());
    let ok = match annotation {
        Some(t) => t.ends_with("Event") || t.ends_with("Event>") || (event_name(&s.name) && t == "any"),
        None => event_name(&s.name),
    };
    if !ok {
        return None;
    }
    enclosing_fn(decl)
}

fn is_suspension(n: Node<'_>) -> bool {
    match n.kind() {
        "await_expression" | "yield_expression" => true,
        "for_in_statement" => children(n).iter().any(|c| c.kind() == "await"),
        _ => false,
    }
}

/// Why an access to the event inside `h` comes too late, if it does: in a
/// nested function, after a suspension, or in a loop that suspends.
fn late(access: Node<'_>, h: Node<'_>) -> Option<&'static str> {
    if !same(enclosing_fn(access), h) {
        return Some("in a nested function, which runs after the event has been dispatched");
    }
    let body = js_ast::body(h)?;
    let suspends = own_descendants(body);
    if suspends.iter().any(|s| is_suspension(*s) && s.end_byte() <= access.start_byte() && !within(access, *s)) {
        return Some("after an `await` or `yield`, when the event has already been dispatched");
    }
    let mut child = access;
    for a in ancestors(access) {
        if a.id() == h.id() {
            break;
        }
        let repeated = match a.kind() {
            "for_in_statement" => !same(a.child_by_field_name("right"), child),
            "for_statement" => !same(a.child_by_field_name("initializer"), child),
            "while_statement" | "do_statement" => true,
            _ => false,
        };
        if repeated && own_descendants(a).iter().any(|s| is_suspension(*s)) {
            return Some("in a loop that awaits, so a later pass runs after the event has been dispatched");
        }
        child = a;
    }
    None
}

srule!(NoLateCurrentTargetAccess, "js/no-late-current-target-access", &["member_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    if member_prop(n, src) != Some("currentTarget") {
        return;
    }
    let Some(obj) = n.child_by_field_name("object").map(unparen) else { return };
    let root = js_ast::root_of(n);
    let Some(h) = event_param(obj, root, m, src) else { return };
    if let Some(why) = late(n, h) {
        out.push(finding(n, format!("`{}.currentTarget` is read {why}; it is `null` by then: read it into a variable first", text(obj, src)), vec![]));
    }
});

srule!(NoLateEventControl, "js/no-late-event-control", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(f) = n.child_by_field_name("function").filter(|f| matches!(f.kind(), "member_expression" | "subscript_expression")) else { return };
    let Some(method) = member_prop(f, src).filter(|p| matches!(*p, "preventDefault" | "stopPropagation" | "stopImmediatePropagation")) else { return };
    let Some(obj) = f.child_by_field_name("object").map(unparen) else { return };
    let root = js_ast::root_of(n);
    let Some(h) = event_param(obj, root, m, src) else { return };
    let why = if is_generator(h.kind()) { Some("in a generator, whose body runs only when it is iterated") } else { late(n, h) };
    if let Some(why) = why {
        out.push(finding(n, format!("`{method}()` is called {why}; it has no effect then: call it before the first `await`"), vec![]));
    }
});

// ---- unicorn: no-loop-iterable-mutation -----------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Coll {
    Array,
    Set,
    Map,
    Unknown,
}

fn collection_kind(recv: Node<'_>, root: Node<'_>, m: &SemanticModel, src: &str) -> Coll {
    let v = if recv.kind() == "identifier" { js_ast::initial_value(m, root, recv) } else { None };
    match v.map(unparen) {
        Some(v) if v.kind() == "array" => Coll::Array,
        Some(v) if v.kind() == "new_expression" => match v.child_by_field_name("constructor").map(|c| text(c, src)) {
            Some("Set" | "WeakSet") => Coll::Set,
            Some("Map" | "WeakMap") => Coll::Map,
            Some("Array") => Coll::Array,
            _ => Coll::Unknown,
        },
        _ => Coll::Unknown,
    }
}

fn mutators(k: Coll) -> &'static [&'static str] {
    const A: &[&str] = &["copyWithin", "fill", "pop", "push", "reverse", "shift", "sort", "splice", "unshift"];
    const S: &[&str] = &["add", "clear", "delete"];
    const M: &[&str] = &["clear", "delete", "set"];
    const U: &[&str] = &["copyWithin", "fill", "pop", "push", "reverse", "shift", "sort", "splice", "unshift", "add", "clear", "delete", "set"];
    match k {
        Coll::Array => A,
        Coll::Set => S,
        Coll::Map => M,
        Coll::Unknown => U,
    }
}

fn plain_ref(n: Node<'_>) -> bool {
    match n.kind() {
        "identifier" | "this" => true,
        "member_expression" => !is_optional(n) && n.child_by_field_name("object").is_some_and(plain_ref),
        _ => false,
    }
}

srule!(NoLoopIterableMutation, "js/no-loop-iterable-mutation", &["for_in_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let ch = children(n);
    if !ch.iter().any(|c| c.kind() == "of") || ch.iter().any(|c| c.kind() == "await") {
        return;
    }
    let Some(right) = n.child_by_field_name("right").map(unparen) else { return };
    let (recv, view) = match method_call(right, &["keys", "values", "entries"], src) {
        Some((r, v)) if arguments(right).is_empty() => (r, Some(v)),
        _ => (right, None),
    };
    if !plain_ref(recv) {
        return;
    }
    let root = js_ast::root_of(n);
    let kind = collection_kind(recv, root, m, src);
    let rtext = text(recv, src);
    let rsym = chain_root(recv).and_then(|r| m.symbol_at(r.start_byte()));
    // The loop's own `const` binding: the current item, or `[key]`.
    let left = n.child_by_field_name("left");
    let is_const = ch.iter().any(|c| c.kind() == "const");
    let current: Option<String> = left.filter(|_| is_const).and_then(|l| match (l.kind(), view) {
        ("identifier", None | Some("values" | "keys")) => Some(text(l, src).to_owned()),
        ("array_pattern", Some("entries") | None) if kind != Coll::Array => named_children(l).first().filter(|x| x.kind() == "identifier").map(|x| text(*x, src).to_owned()),
        _ => None,
    });
    let Some(body) = n.child_by_field_name("body") else { return };
    let mut deleted = false;
    for d in own_descendants(body) {
        let Some((r, method)) = method_call(d, mutators(kind), src) else { continue };
        if text(r, src) != rtext || chain_root(r).and_then(|x| m.symbol_at(x.start_byte())) != rsym {
            continue;
        }
        let arg0 = first_arg(d).map(|a| text(a, src));
        let on_current = current.is_some() && arg0 == current.as_deref();
        if on_current && method == "delete" {
            deleted = true;
            continue;
        }
        if on_current && (method == "add" || method == "set") && !deleted {
            continue;
        }
        let prop = d.child_by_field_name("function").and_then(|f| f.child_by_field_name("property").or(f.child_by_field_name("index"))).unwrap_or(d);
        out.push(finding(prop, format!("`{rtext}.{method}()` changes `{rtext}` while this loop iterates it: iterate a copy (`[...{rtext}]`) or collect the changes and apply them after"), vec![]));
    }
});

// ---- unicorn: no-mismatched-map-key ----------------------------------------

fn key_ref(n: Node<'_>) -> bool {
    match n.kind() {
        "identifier" | "this" => true,
        "member_expression" => n.child_by_field_name("object").is_some_and(key_ref),
        "subscript_expression" => n.child_by_field_name("object").is_some_and(key_ref) && n.child_by_field_name("index").is_some_and(|i| key_ref(i) || matches!(i.kind(), "string" | "number")),
        _ => false,
    }
}

fn single_statement(n: Node<'_>) -> Option<Node<'_>> {
    if n.kind() == "statement_block" {
        let s: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
        return (s.len() == 1).then(|| s[0]);
    }
    Some(n)
}

srule!(NoMismatchedMapKey, "js/no-mismatched-map-key", &["if_statement", "ternary_expression", "while_statement", "binary_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let (test, branches): (Option<Node<'_>>, Vec<Node<'_>>) = match n.kind() {
        "if_statement" => {
            let mut b = Vec::new();
            if let Some(c) = n.child_by_field_name("consequence").and_then(single_statement) {
                b.push(c);
            }
            if let Some(alt) = n.child_by_field_name("alternative").and_then(|a| named_children(a).into_iter().find(|c| c.kind() != "comment"))
                && alt.kind() != "if_statement"
                && let Some(s) = single_statement(alt)
            {
                b.push(s);
            }
            (n.child_by_field_name("condition"), b)
        }
        "while_statement" => (n.child_by_field_name("condition"), n.child_by_field_name("body").into_iter().collect()),
        "ternary_expression" => (n.child_by_field_name("condition"), [n.child_by_field_name("consequence"), n.child_by_field_name("alternative")].into_iter().flatten().collect()),
        _ => {
            if !n.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "&&" | "||" | "??")) {
                return;
            }
            (n.child_by_field_name("left"), n.child_by_field_name("right").into_iter().collect())
        }
    };
    let Some(mut test) = test.map(unparen) else { return };
    while test.kind() == "binary_expression" && test.child_by_field_name("operator").is_some_and(|o| text(o, src) == "&&") {
        let Some(r) = test.child_by_field_name("right") else { return };
        test = unparen(r);
    }
    if test.kind() == "unary_expression" && test.child_by_field_name("operator").is_some_and(|o| text(o, src) == "!") {
        let Some(a) = test.child_by_field_name("argument") else { return };
        test = unparen(a);
    }
    let Some((recv, _)) = method_call(test, &["has"], src) else { return };
    let args = arguments(test);
    let [key] = args[..] else { return };
    if key.kind() == "spread_element" || !plain_ref(recv) {
        return;
    }
    let root = js_ast::root_of(n);
    if matches!(collection_kind(recv, root, m, src), Coll::Array | Coll::Set) {
        return;
    }
    let rtext = text(recv, src);
    let root_name = chain_root(recv).map(|r| text(r, src));
    for b in branches {
        let mut stack = vec![b];
        while let Some(x) = stack.pop() {
            if fn_like(x.kind()) {
                continue;
            }
            if x.kind() == "assignment_expression" && x.child_by_field_name("left").and_then(chain_root).map(|r| text(r, src)) == root_name {
                break;
            }
            if let Some((r, _)) = method_call(x, &["has"], src)
                && text(r, src) == rtext
                && x.id() != test.id()
            {
                continue;
            }
            if let Some((r, method)) = method_call(x, &["get", "set"], src)
                && text(r, src) == rtext
                && let Some(k2) = first_arg(x)
            {
                let differ = match (static_value(key, src), static_value(k2, src)) {
                    (Some(a), Some(b)) => a != b,
                    (None, None) => key_ref(unparen(key)) && key_ref(unparen(k2)) && text(unparen(key), src) != text(unparen(k2), src),
                    _ => false,
                };
                if differ {
                    out.push(finding(k2, format!("`{rtext}.{method}()` uses a different key from the `{rtext}.has({})` that guards it: use the same key", text(key, src)), vec![]));
                }
            }
            stack.extend(named_children(x));
        }
    }
});

// ---- unicorn: no-canvas-to-image -------------------------------------------

fn single_use_init<'t>(arg: Node<'t>, root: Node<'t>, m: &SemanticModel) -> Option<Node<'t>> {
    if arg.kind() != "identifier" {
        return Some(arg);
    }
    let sym = m.symbol_at(arg.start_byte())?;
    let s = m.symbol(sym);
    if s.kind != SymbolKind::Const || s.decls.len() != 1 || s.refs.len() != 1 || s.decls[0].0 > arg.start_byte() {
        return None;
    }
    js_ast::initial_value(m, root, arg)
}

fn strip_await(n: Node<'_>) -> Node<'_> {
    let n = unparen(n);
    if n.kind() == "await_expression" { n.named_child(0).map_or(n, unparen) } else { n }
}

fn canvas_like(n: Node<'_>, src: &str) -> bool {
    let name = match n.kind() {
        "identifier" => text(n, src),
        "member_expression" => n.child_by_field_name("property").map_or("", |p| text(p, src)),
        _ => return false,
    };
    name.to_ascii_lowercase().contains("canvas")
}

srule!(NoCanvasToImage, "js/no-canvas-to-image", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some((_, method)) = method_call(n, &["drawImage", "putImageData"], src) else { return };
    let Some(arg) = first_arg(n) else { return };
    let root = js_ast::root_of(n);
    let Some(v) = single_use_init(strip_await(arg), root, m).map(strip_await) else { return };
    let hit = if method == "drawImage" {
        v.kind() == "call_expression"
            && v.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "loadImage")
            && first_arg(v).and_then(|a| method_call(a, &["toDataURL"], src)).is_some_and(|(r, _)| canvas_like(r, src))
    } else {
        method_call(v, &["getImageData"], src).is_some()
    };
    if hit {
        let what = if method == "drawImage" { "an image decoded from the canvas's own data URL: draw the canvas itself (`drawImage(canvas, …)`)" } else { "the pixels just read with `getImageData()`: copy them with `drawImage(canvas, …)` instead" };
        out.push(finding(arg, format!("`{method}()` is given {what}"), vec![]));
    }
});

// ---- unicorn: consistent-function-scoping ---------------------------------

const REACT_HOOKS: &[&str] = &["useEffect", "useLayoutEffect", "useInsertionEffect", "useMemo", "useCallback", "useImperativeHandle", "useState", "useReducer", "useRef", "useSyncExternalStore", "useActionState", "useOptimistic", "useTransition"];

fn call_of_arg(f: Node<'_>) -> Option<(Node<'_>, usize)> {
    let mut x = f;
    while x.parent().is_some_and(|p| p.kind() == "parenthesized_expression") {
        x = x.parent()?;
    }
    let args = x.parent().filter(|p| p.kind() == "arguments")?;
    let call = args.parent()?;
    let i = arguments(call).iter().position(|a| a.id() == x.id())?;
    Some((call, i))
}

fn is_iife(f: Node<'_>) -> bool {
    let mut x = f;
    while x.parent().is_some_and(|p| p.kind() == "parenthesized_expression") {
        let Some(p) = x.parent() else { break };
        x = p;
    }
    x.parent().is_some_and(|p| p.kind() == "call_expression" && same(p.child_by_field_name("function"), x))
}

fn callee_text<'s>(call: Node<'_>, src: &'s str) -> &'s str {
    call.child_by_field_name("function").map_or("", |f| text(f, src))
}

fn uses_lexical_this(f: Node<'_>) -> bool {
    own_descendants(f).iter().any(|d| d.kind() == "this" || d.kind() == "super") || {
        let mut found = false;
        let mut stack = named_children(f);
        while let Some(x) = stack.pop() {
            if x.kind() == "this" {
                found = true;
                break;
            }
            if x.kind() == "arrow_function" || !fn_like(x.kind()) {
                stack.extend(named_children(x));
            }
        }
        found
    }
}

/// Whether a function declared here is a candidate to move: a declaration in
/// a block, a `const f = …` in a block, or a function returned.
fn movable(f: Node<'_>) -> bool {
    let Some(p) = f.parent() else { return false };
    match (f.kind(), p.kind()) {
        ("function_declaration" | "generator_function_declaration", "statement_block") => true,
        (_, "variable_declarator") => p.parent().and_then(|d| d.parent()).is_some_and(|b| b.kind() == "statement_block"),
        (_, "return_statement") => true,
        ("arrow_function", "arrow_function") => same(p.child_by_field_name("body"), f) && movable(p),
        _ => false,
    }
}

srule!(ConsistentFunctionScoping, "js/consistent-function-scoping", &["function_declaration", "generator_function_declaration", "function_expression", "function", "arrow_function"], |n, ctx, m, out| {
    let src = ctx.source;
    if n.kind() == "arrow_function" && ctx.options.get("checkArrowFunctions").and_then(Value::as_bool) == Some(false) {
        return;
    }
    if !movable(n) {
        return;
    }
    let Some(outer) = enclosing_fn(n) else { return };
    if is_iife(outer) || (n.kind() == "arrow_function" && uses_lexical_this(n)) {
        return;
    }
    if let Some((call, _)) = call_of_arg(outer) {
        let c = callee_text(call, src);
        let hook = c.strip_prefix("React.").unwrap_or(c);
        if REACT_HOOKS.contains(&hook) {
            return;
        }
    }
    for a in ancestors(n) {
        if let Some((call, i)) = call_of_arg(a) {
            let c = callee_text(call, src);
            if ((c == "jest.mock" || c == "vi.mock") && i == 1) || (c == "vi.hoisted" && i == 0) {
                return;
            }
        }
    }
    let own_name = n.child_by_field_name("name").or_else(|| n.parent().filter(|p| p.kind() == "variable_declarator").and_then(|p| p.child_by_field_name("name")));
    let own = own_name.and_then(|x| m.symbol_at(x.start_byte()));
    for r in &m.references {
        if !in_range(r.start, n) {
            continue;
        }
        let Some(sym) = r.symbol else { continue };
        if Some(sym) == own {
            continue;
        }
        let Some(&(ds, _)) = m.symbol(sym).decls.first() else { continue };
        if in_range(ds, outer) && !in_range(ds, n) {
            return;
        }
    }
    let name = own_name.map_or("this function".to_owned(), |x| format!("`{}`", text(x, src)));
    let head_end = js_ast::body(n).map_or(n.end_byte(), |b| b.start_byte()).max(n.start_byte());
    let head = src[n.start_byte()..head_end].trim_end();
    let _ = head;
    out.push(super::span(n.start_byte(), head_end, format!("Move {name} to the outer scope: it uses nothing from the function it is declared in"), vec![]));
});

// ---- unicorn: isolated-functions --------------------------------------------

fn strings_opt(o: &BTreeMap<String, Value>, key: &str, default: &[&str]) -> Vec<String> {
    match o.get(key) {
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).map(str::to_owned).collect(),
        _ => default.iter().map(|s| (*s).to_owned()).collect(),
    }
}

fn tagged(f: Node<'_>, tags: &[String], src: &str) -> bool {
    let mut x = f;
    loop {
        if let Some(c) = x.prev_sibling().filter(|c| c.kind() == "comment") {
            let t = text(c, src).trim_start_matches("//").trim_start_matches("/*").trim_end_matches("*/");
            let t = t.lines().map(|l| l.trim().trim_start_matches('*').trim()).collect::<Vec<_>>().join(" ");
            let t = t.trim().to_lowercase();
            if tags.iter().any(|g| {
                let g = g.to_lowercase();
                t == g || t.starts_with(&format!("{g} - ")) || t.starts_with(&format!("{g} -- "))
            }) {
                return true;
            }
        }
        match x.parent() {
            Some(p) if matches!(p.kind(), "variable_declarator" | "lexical_declaration" | "variable_declaration" | "export_statement" | "pair") => x = p,
            _ => return false,
        }
    }
}

fn isolated(f: Node<'_>, o: &BTreeMap<String, Value>, src: &str) -> bool {
    if tagged(f, &strings_opt(o, "comments", &["@isolated"]), src) {
        return true;
    }
    if let Some((call, i)) = call_of_arg(f) {
        let c = callee_text(call, src);
        let fns = strings_opt(o, "functions", &["makeSynchronous", "workerize"]);
        if call.child_by_field_name("function").is_some_and(|x| x.kind() == "identifier") && fns.iter().any(|x| x == c) {
            return true;
        }
        if i == 0 && (c == "browser.execute" || c == "page.evaluate") {
            return true;
        }
    }
    // `chrome.scripting.executeScript({ func })`
    if let Some(pair) = f.parent().filter(|p| p.kind() == "pair")
        && pair.child_by_field_name("key").is_some_and(|k| text(k, src) == "func")
        && let Some(obj) = pair.parent()
        && let Some((call, 0)) = call_of_arg(obj)
    {
        return matches!(callee_text(call, src), "chrome.scripting.executeScript" | "browser.scripting.executeScript");
    }
    false
}

srule!(IsolatedFunctions, "js/isolated-functions", &["function_declaration", "function_expression", "function", "arrow_function", "generator_function", "generator_function_declaration", "method_definition"], |n, ctx, m, out| {
    let src = ctx.source;
    if !isolated(n, ctx.options, src) {
        return;
    }
    let own_name = n.child_by_field_name("name").and_then(|x| m.symbol_at(x.start_byte()));
    for r in &m.references {
        if !in_range(r.start, n) || r.type_only {
            continue;
        }
        let outside = match r.symbol {
            Some(s) if Some(s) == own_name => false,
            Some(s) => m.symbol(s).decls.first().is_some_and(|d| !in_range(d.0, n)),
            None => r.write || !m.is_builtin_reference(r),
        };
        if outside && let Some(id) = node_at(js_ast::root_of(n), r.start, r.end) {
            out.push(finding(id, format!("`{}` comes from outside this isolated function: it won't exist where the function runs; pass it in as an argument", r.name), vec![]));
        }
    }
    let mut stack = named_children(n);
    while let Some(x) = stack.pop() {
        match x.kind() {
            "this" | "super" => out.push(finding(x, format!("`{}` in an isolated function is not the object you expect where the function runs", text(x, src)), vec![])),
            k if fn_like(k) && k != "arrow_function" => {}
            "class" | "class_declaration" => {}
            _ => stack.extend(named_children(x)),
        }
    }
});

// ---- unicorn: no-uncalled-method --------------------------------------------

const ARRAY_METHODS: &[&str] = &[
    "at", "concat", "copyWithin", "entries", "every", "fill", "filter", "find", "findIndex", "findLast", "findLastIndex", "flat", "flatMap", "forEach", "includes", "indexOf", "join", "keys", "lastIndexOf", "map", "pop", "push",
    "reduce", "reduceRight", "reverse", "shift", "slice", "some", "sort", "splice", "toLocaleString", "toReversed", "toSorted", "toSpliced", "toString", "unshift", "values", "with",
];
const STRING_METHODS: &[&str] = &[
    "at", "charAt", "charCodeAt", "codePointAt", "concat", "endsWith", "includes", "indexOf", "isWellFormed", "lastIndexOf", "localeCompare", "match", "matchAll", "normalize", "padEnd", "padStart", "repeat", "replace",
    "replaceAll", "search", "slice", "split", "startsWith", "substr", "substring", "toLocaleLowerCase", "toLocaleUpperCase", "toLowerCase", "toString", "toUpperCase", "toWellFormed", "trim", "trimEnd", "trimStart",
    "valueOf", "anchor", "big", "blink", "bold", "fixed", "fontcolor", "fontsize", "italics", "link", "small", "strike", "sub", "sup",
];

fn receiver_kind(n: Node<'_>, root: Node<'_>, m: &SemanticModel, src: &str, depth: usize) -> Option<Coll> {
    let n = unparen(n);
    match n.kind() {
        "array" => Some(Coll::Array),
        "string" | "template_string" => Some(Coll::Set), // `Set` stands for "string" here
        "identifier" => match text(n, src) {
            "array" => Some(Coll::Array),
            "string" => Some(Coll::Set),
            _ if depth < 4 => {
                let sym = m.symbol_at(n.start_byte())?;
                let s = m.symbol(sym);
                if s.kind != SymbolKind::Const || s.decls.len() != 1 {
                    return None;
                }
                receiver_kind(js_ast::initial_value(m, root, n)?, root, m, src, depth + 1)
            }
            _ => None,
        },
        "call_expression" if callee_is(n, "Array", "from", src) || callee_is(n, "Array", "of", src) => Some(Coll::Array),
        _ => None,
    }
}

srule!(NoUncalledMethod, "js/no-uncalled-method", &["member_expression", "subscript_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(name) = member_prop(n, src) else { return };
    if !ARRAY_METHODS.contains(&name) && !STRING_METHODS.contains(&name) {
        return;
    }
    let Some(obj) = n.child_by_field_name("object") else { return };
    if text(obj, src) == "Array.prototype" || text(obj, src) == "String.prototype" {
        return;
    }
    let root = js_ast::root_of(n);
    let kind = match receiver_kind(obj, root, m, src, 0) {
        Some(Coll::Array) if ARRAY_METHODS.contains(&name) => "array",
        Some(Coll::Set) if STRING_METHODS.contains(&name) => "string",
        _ => return,
    };
    let mut x = n;
    while x.parent().is_some_and(|p| p.kind() == "parenthesized_expression" || p.kind() == "non_null_expression") {
        let Some(p) = x.parent() else { break };
        x = p;
    }
    if is_callee(x) {
        return;
    }
    let Some(p) = x.parent() else { return };
    match p.kind() {
        "call_expression" if p.child_by_field_name("arguments").is_some_and(|a| a.kind() == "template_string") => return,
        "member_expression" if matches!(member_prop(p, src), Some("call" | "apply" | "bind")) && is_callee(p) => return,
        "unary_expression" if p.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "typeof" | "delete")) => return,
        "assignment_expression" | "augmented_assignment_expression" if same(p.child_by_field_name("left"), x) => return,
        "update_expression" => return,
        "arguments" if p.parent().is_some_and(|c| callee_is(c, "Reflect", "apply", src)) && first_arg(p.parent().unwrap_or(p)).is_some_and(|a| a.id() == x.id()) => return,
        _ => {}
    }
    let prop = n.child_by_field_name("property").or(n.child_by_field_name("index")).unwrap_or(n);
    out.push(finding(prop, format!("`{name}` is a {kind} method that is read, not called: add `()`"), vec![]));
});

// ---- unicorn: no-unused-properties ------------------------------------------

/// How the uses of an object reach each property: by name, with the member
/// expressions that read it, or `None` when a use may read any.
fn property_uses<'t>(uses: &[Node<'t>], src: &str) -> Option<HashMap<String, Vec<Node<'t>>>> {
    let mut used: HashMap<String, Vec<Node<'t>>> = HashMap::new();
    for u in uses {
        let mut x = *u;
        while x.parent().is_some_and(|p| matches!(p.kind(), "parenthesized_expression" | "non_null_expression" | "as_expression" | "satisfies_expression")) {
            let Some(p) = x.parent() else { break };
            x = p;
        }
        let p = x.parent()?;
        match p.kind() {
            "member_expression" | "subscript_expression" if same(p.child_by_field_name("object"), x) => {
                let name = member_prop(p, src)?;
                let gp = p.parent();
                let written = gp.is_some_and(|g| matches!(g.kind(), "assignment_expression" | "augmented_assignment_expression") && same(g.child_by_field_name("left"), p)) || gp.is_some_and(|g| g.kind() == "update_expression");
                if written || is_callee(p) {
                    return None;
                }
                used.entry(name.to_owned()).or_default().push(p);
            }
            "variable_declarator" if same(p.child_by_field_name("value"), x) => {
                let pat = p.child_by_field_name("name")?;
                if pat.kind() != "object_pattern" {
                    return None;
                }
                for c in named_children(pat) {
                    match c.kind() {
                        "shorthand_property_identifier_pattern" => {
                            used.entry(text(c, src).to_owned()).or_default();
                        }
                        "pair_pattern" | "object_assignment_pattern" => {
                            let k = c.child_by_field_name("key").or_else(|| c.child_by_field_name("left"))?;
                            let name = key_name(k, src).or_else(|| (k.kind() == "shorthand_property_identifier_pattern").then(|| text(k, src).to_owned()))?;
                            used.entry(name).or_default();
                        }
                        "comment" => {}
                        _ => return None,
                    }
                }
            }
            _ => return None,
        }
    }
    Some(used)
}

fn check_object(obj: Node<'_>, uses: &[Node<'_>], src: &str, out: &mut Vec<NativeFinding>) {
    let props = js_ast::members(obj);
    if props.iter().any(|p| p.kind() == "method_definition" || p.kind() == "spread_element" || (p.kind() == "pair" && p.child_by_field_name("value").is_some_and(|v| matches!(unparen(v).kind(), "function_expression" | "function" | "generator_function")))) {
        return;
    }
    let Some(used) = property_uses(uses, src) else { return };
    for p in props {
        let Some(name) = js_ast::member_name(p, src) else { continue };
        if name == "__proto__" {
            continue;
        }
        match used.get(name) {
            None => out.push(finding(p, format!("Property `{name}` is defined but never used"), vec![remove_list_item(p, src)])),
            Some(reads) if !reads.is_empty() => {
                if let Some(v) = js_ast::member_value(p).map(unparen).filter(|v| v.kind() == "object") {
                    check_object(v, reads, src, out);
                }
            }
            _ => {}
        }
    }
}

srule!(NoUnusedProperties, "js/no-unused-properties", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    for (i, s) in m.symbols.iter().enumerate() {
        if s.exported || s.decls.len() != 1 || !matches!(s.kind, SymbolKind::Const | SymbolKind::Let | SymbolKind::Var) {
            continue;
        }
        let refs = refs_of(m, i);
        if !refs.iter().any(|r| r.read) || refs.iter().any(|r| r.write) {
            continue;
        }
        let (a, b) = s.decls[0];
        let Some(id) = node_at(n, a, b) else { continue };
        let Some(v) = js_ast::initial_value(m, n, id).map(unparen).filter(|v| v.kind() == "object") else { continue };
        let uses: Vec<Node<'_>> = refs.iter().filter_map(|r| node_at(n, r.start, r.end)).collect();
        check_object(v, &uses, src, out);
    }
});

// ---- unicorn: prefer-get-or-insert-computed --------------------------------

fn side_effect(n: Node<'_>, src: &str) -> bool {
    match n.kind() {
        "call_expression" | "new_expression" | "assignment_expression" | "augmented_assignment_expression" | "update_expression" | "await_expression" | "yield_expression" => true,
        "unary_expression" => n.child_by_field_name("operator").is_some_and(|o| text(o, src) == "delete") || named_children(n).iter().any(|c| side_effect(*c, src)),
        k if fn_like(k) || k == "class" => false,
        _ => named_children(n).iter().any(|c| side_effect(*c, src)),
    }
}

fn contains_kind(n: Node<'_>, pred: &dyn Fn(Node<'_>) -> bool) -> bool {
    pred(n) || named_children(n).iter().any(|c| !fn_like(c.kind()) && contains_kind(*c, pred))
}

srule!(PreferGetOrInsertComputed, "js/prefer-get-or-insert-computed", &["call_expression"], |n, ctx, _m, out| {
    let src = ctx.source;
    let Some(f) = n.child_by_field_name("function").filter(|f| f.kind() == "member_expression") else { return };
    let Some(prop) = f.child_by_field_name("property").filter(|p| text(*p, src) == "getOrInsert") else { return };
    let args = arguments(n);
    let [key, value] = args[..] else { return };
    if !side_effect(value, src) {
        return;
    }
    let blocked = contains_kind(value, &|x| matches!(x.kind(), "await_expression" | "yield_expression" | "comment") || (x.kind() == "call_expression" && x.child_by_field_name("function").is_some_and(|c| text(c, src) == "eval")));
    let mut edits = Vec::new();
    if !blocked {
        let v = unparen(value);
        let short = v.kind() == "call_expression"
            && v.child_by_field_name("function").is_some_and(|c| c.kind() == "identifier")
            && matches!(&arguments(v)[..], [a] if a.kind() == "identifier" && key.kind() == "identifier" && text(*a, src) == text(key, src));
        let body = if short {
            text(v.child_by_field_name("function").unwrap_or(v), src).to_owned()
        } else if matches!(v.kind(), "object" | "sequence_expression" | "as_expression" | "satisfies_expression") {
            format!("() => ({})", text(v, src))
        } else {
            format!("() => {}", text(value, src))
        };
        edits.push(TextEdit::new(prop.start_byte(), prop.end_byte(), "getOrInsertComputed"));
        edits.push(TextEdit::new(value.start_byte(), value.end_byte(), body));
    }
    out.push(finding(value, "`getOrInsert()` evaluates this value even when the key is present: use `getOrInsertComputed()` with a callback", edits));
});

// ---- unicorn: no-top-level-side-effects -------------------------------------

fn runtime_export(e: Node<'_>) -> bool {
    if children(e).iter().any(|c| c.kind() == "type") {
        return false;
    }
    if let Some(d) = e.child_by_field_name("declaration") {
        return !matches!(d.kind(), "type_alias_declaration" | "interface_declaration" | "ambient_declaration");
    }
    if e.child_by_field_name("source").is_some() || e.child_by_field_name("value").is_some() || children(e).iter().any(|c| c.kind() == "default") {
        return true;
    }
    named_children(e).iter().any(|c| c.kind() == "export_clause" && named_children(*c).iter().any(|s| s.kind() == "export_specifier"))
}

const OBJECT_MUTATORS: &[&str] = &["assign", "defineProperty", "defineProperties", "freeze", "seal", "preventExtensions", "setPrototypeOf"];
const REFLECT_MUTATORS: &[&str] = &["defineProperty", "deleteProperty", "set", "preventExtensions", "setPrototypeOf"];

srule!(NoTopLevelSideEffects, "js/no-top-level-side-effects", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    if src.starts_with("#!") || !named_children(n).iter().any(|c| c.kind() == "export_statement" && runtime_export(*c)) {
        return;
    }
    let global_root = |t: Node<'_>| chain_root(unparen(t)).is_some_and(|r| is_builtin(m, r));
    let mut stack = named_children(n);
    while let Some(x) = stack.pop() {
        let k = x.kind();
        if fn_like(k) || (matches!(k, "field_definition" | "public_field_definition") && !is_static(x)) {
            continue;
        }
        let hit = match k {
            "assignment_expression" | "augmented_assignment_expression" => x.child_by_field_name("left").is_some_and(|l| l.kind() != "identifier" && global_root(l)),
            "update_expression" => x.child_by_field_name("argument").is_some_and(|a| a.kind() != "identifier" && global_root(a)),
            "unary_expression" => x.child_by_field_name("operator").is_some_and(|o| text(o, src) == "delete") && x.child_by_field_name("argument").is_some_and(global_root),
            "call_expression" => {
                let f = x.child_by_field_name("function");
                let obj = f.filter(|f| f.kind() == "member_expression").and_then(|f| f.child_by_field_name("object")).filter(|o| o.kind() == "identifier" && is_builtin(m, *o));
                let method = f.and_then(|f| member_prop(f, src));
                let args = arguments(x);
                match (obj.map(|o| text(o, src)), method) {
                    (Some("Object"), Some(mt)) if OBJECT_MUTATORS.contains(&mt) => args.first().is_some_and(|a| global_root(*a)),
                    (Some("Reflect"), Some("set")) => args.len() == 3 && !args.iter().any(|a| a.kind() == "spread_element") && global_root(args[0]),
                    (Some("Reflect"), Some(mt)) if REFLECT_MUTATORS.contains(&mt) => args.first().is_some_and(|a| global_root(*a)),
                    _ => false,
                }
            }
            _ => false,
        };
        if hit {
            out.push(finding(x, "This module changes a global when it is imported: export a function that does it, and call that where the change is wanted", vec![]));
            continue;
        }
        stack.extend(named_children(x));
    }
});

// ---- ESLint: block-scoped-var -----------------------------------------------

fn binding_block(n: Node<'_>) -> Option<Node<'_>> {
    ancestors(n).find(|a| matches!(a.kind(), "statement_block" | "for_statement" | "for_in_statement" | "switch_statement" | "program" | "class_static_block"))
}

srule!(BlockScopedVar, "js/block-scoped-var", &["program"], |n, _ctx, m, out| {
    for (i, s) in m.symbols.iter().enumerate() {
        if s.kind != SymbolKind::Var {
            continue;
        }
        for &(a, b) in &s.decls {
            let Some(block) = node_at(n, a, b).and_then(binding_block) else { continue };
            for r in refs_of(m, i) {
                if !in_range(r.start, block) && let Some(id) = node_at(n, r.start, r.end) {
                    out.push(finding(id, format!("`{}` is used outside the block that declares it: declare it with `let` in the scope where it is used", r.name), vec![]));
                }
            }
        }
    }
});

// ---- ESLint: no-invalid-this ------------------------------------------------

fn nullish(n: Node<'_>, src: &str) -> bool {
    matches!(unparen(n).kind(), "null" | "undefined") || text(n, src) == "undefined" || text(n, src) == "void 0"
}

const THIS_ARG_METHODS: &[&str] = &["every", "filter", "find", "findIndex", "findLast", "findLastIndex", "flatMap", "forEach", "map", "some"];

fn capitalized(name: &str) -> bool {
    name.trim_start_matches(['_', '$']).starts_with(|c: char| c.is_uppercase())
}

/// Whether a non-arrow function gets a `this` from how it is written or
/// called (ESLint's reading of "not the default binding").
fn binds_this(f: Node<'_>, src: &str) -> bool {
    if f.kind() == "method_definition" || has_this_param(f) {
        return true;
    }
    let own = f.child_by_field_name("name").map(|x| text(x, src));
    if own.is_some_and(capitalized) {
        return true;
    }
    let mut x = f;
    while x.parent().is_some_and(|p| p.kind() == "parenthesized_expression") {
        let Some(p) = x.parent() else { break };
        x = p;
    }
    // `/** @this Foo */`
    let mut c = x;
    loop {
        if c.prev_sibling().is_some_and(|s| s.kind() == "comment" && text(s, src).contains("@this")) {
            return true;
        }
        match c.parent() {
            Some(p) if matches!(p.kind(), "variable_declarator" | "lexical_declaration" | "variable_declaration" | "export_statement" | "pair" | "assignment_expression" | "expression_statement") => c = p,
            _ => break,
        }
    }
    let Some(p) = x.parent() else { return false };
    match p.kind() {
        "pair" | "field_definition" | "public_field_definition" => true,
        "assignment_expression" => p.child_by_field_name("left").is_some_and(|l| l.kind() == "member_expression" || capitalized(text(l, src))),
        "variable_declarator" => p.child_by_field_name("name").is_some_and(|l| capitalized(text(l, src))),
        "member_expression" if matches!(member_prop(p, src), Some("call" | "apply" | "bind")) => {
            let call = p.parent().filter(|c| c.kind() == "call_expression");
            call.and_then(first_arg).is_some_and(|a| !nullish(a, src))
        }
        "arguments" => {
            let Some(call) = p.parent() else { return false };
            let args = arguments(call);
            let i = args.iter().position(|a| a.id() == x.id());
            if callee_is(call, "Array", "from", src) {
                return i == Some(1) && args.get(2).is_some_and(|a| !nullish(*a, src));
            }
            if callee_is(call, "Reflect", "apply", src) {
                return i == Some(0) && args.get(1).is_some_and(|a| !nullish(*a, src));
            }
            let method = call.child_by_field_name("function").and_then(|f| member_prop(f, src));
            method.is_some_and(|mt| THIS_ARG_METHODS.contains(&mt)) && i == Some(0) && args.get(1).is_some_and(|a| !nullish(*a, src))
        }
        _ => false,
    }
}

fn strict_file(ctx: &NativeContext<'_>) -> bool {
    !ctx.file.path().ends_with(".cjs") || ctx.source.trim_start().starts_with("\"use strict\"") || ctx.source.trim_start().starts_with("'use strict'")
}

srule!(NoInvalidThis, "js/no-invalid-this", &["this"], |n, ctx, _m, out| {
    let src = ctx.source;
    if !strict_file(ctx) || n.parent().is_some_and(|p| p.kind() == "required_parameter") || ancestors(n).any(|a| a.kind() == "type_query") {
        return;
    }
    let mut child = n;
    for a in ancestors(n) {
        match a.kind() {
            "arrow_function" => {}
            "class_static_block" | "class_heritage" | "class_body" => return,
            "field_definition" | "public_field_definition" if same(a.child_by_field_name("value"), child) => return,
            k if fn_like(k) => {
                if binds_this(a, src) {
                    return;
                }
                break;
            }
            _ => {}
        }
        child = a;
    }
    out.push(finding(n, "`this` is `undefined` here in strict code: pass the object as a parameter", vec![]));
});

// ---- ESLint: no-loop-func ----------------------------------------------------

fn loop_of(f: Node<'_>) -> Option<Node<'_>> {
    let mut child = f;
    for a in ancestors(f) {
        match a.kind() {
            "for_statement" if !same(a.child_by_field_name("initializer"), child) => return Some(a),
            "for_in_statement" if !same(a.child_by_field_name("right"), child) => return Some(a),
            "while_statement" | "do_statement" => return Some(a),
            k if fn_like(k) || k == "program" => return None,
            _ => {}
        }
        child = a;
    }
    None
}

/// Where a declaration writes: a declarator with a value, or a loop head.
fn decl_writes(root: Node<'_>, start: usize, end: usize) -> bool {
    node_at(root, start, end).and_then(|id| id.parent()).is_some_and(|p| (p.kind() == "variable_declarator" && p.child_by_field_name("value").is_some()) || p.kind() == "for_in_statement" || ancestors(p).take(3).any(|a| a.kind() == "for_in_statement"))
}

srule!(NoLoopFunc, "js/no-loop-func", &["function_declaration", "function_expression", "function", "arrow_function", "generator_function", "generator_function_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(lp) = loop_of(n) else { return };
    if is_iife(n) && !js_ast::is_async(n) && !is_generator(n.kind()) {
        return;
    }
    // The outermost loop around this one, within the same function.
    let mut top = lp;
    for a in ancestors(lp) {
        if fn_like(a.kind()) {
            break;
        }
        if matches!(a.kind(), "for_statement" | "for_in_statement" | "while_statement" | "do_statement") {
            top = a;
        }
    }
    let root = js_ast::root_of(n);
    let mut unsafe_names: Vec<String> = Vec::new();
    for r in &m.references {
        if !in_range(r.start, n) || r.type_only {
            continue;
        }
        let Some(sym) = r.symbol else { continue };
        let s = m.symbol(sym);
        let Some(&(ds, de)) = s.decls.first() else { continue };
        if in_range(ds, n) || unsafe_names.contains(&s.name) {
            continue;
        }
        let safe = match s.kind {
            SymbolKind::Const | SymbolKind::Function | SymbolKind::Import | SymbolKind::Enum | SymbolKind::Namespace => true,
            SymbolKind::Let | SymbolKind::Class if in_range(ds, lp) => true,
            _ => {
                let border = if s.kind == SymbolKind::Let && in_range(ds, top) { lp.start_byte() } else { top.start_byte() };
                let home = decl_fn(root, m, sym).map(|f| f.id());
                let writes_ok = refs_of(m, sym).iter().filter(|w| w.write).all(|w| node_at(root, w.start, w.end).and_then(enclosing_fn).map(|f| f.id()) == home && w.start < border);
                let decls_ok = s.decls.iter().all(|&(a, b)| !decl_writes(root, a, b) || a < border);
                let _ = de;
                writes_ok && decls_ok
            }
        };
        if !safe {
            unsafe_names.push(s.name.clone());
        }
    }
    if !unsafe_names.is_empty() {
        let list = unsafe_names.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(", ");
        let head_end = js_ast::body(n).map_or(n.end_byte(), |b| b.start_byte());
        let _ = src;
        out.push(super::span(n.start_byte(), head_end, format!("A function made in a loop reads {list}, which the loop changes: every call sees the last value"), vec![]));
    }
});

// ---- ESLint: no-unmodified-loop-condition -----------------------------------

fn dynamic(n: Node<'_>) -> bool {
    matches!(n.kind(), "call_expression" | "new_expression" | "member_expression" | "subscript_expression" | "yield_expression" | "await_expression") || named_children(n).iter().any(|c| !fn_like(c.kind()) && dynamic(*c))
}

fn is_group(n: Node<'_>, src: &str) -> bool {
    n.kind() == "ternary_expression" || (n.kind() == "binary_expression" && !n.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "&&" | "||" | "??")))
}

srule!(NoUnmodifiedLoopCondition, "js/no-unmodified-loop-condition", &["while_statement", "do_statement", "for_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(mut test) = n.child_by_field_name("condition") else { return };
    if test.kind() == "expression_statement" {
        let Some(t) = test.named_child(0) else { return };
        test = t;
    }
    if test.kind() == "empty_statement" {
        return;
    }
    let root = js_ast::root_of(n);
    let modified = |sym: SymbolId| {
        refs_of(m, sym).iter().any(|w| {
            if !w.write {
                return false;
            }
            if in_range(w.start, n) {
                return true;
            }
            // Written in a function declaration the loop calls.
            let f = node_at(root, w.start, w.end).and_then(|id| ancestors(id).find(|a| a.kind() == "function_declaration"));
            f.and_then(|f| f.child_by_field_name("name")).is_some_and(|name| {
                let fname = text(name, src);
                own_descendants(n).iter().any(|c| c.kind() == "call_expression" && c.child_by_field_name("function").is_some_and(|x| x.kind() == "identifier" && text(x, src) == fname))
            })
        })
    };
    let mut idents = Vec::new();
    crate::util::descendants(test, &|x| fn_like(x.kind()), &mut idents);
    if test.kind() == "identifier" {
        idents.push(test);
    }
    let mut done: BTreeSet<String> = BTreeSet::new();
    for id in idents {
        if id.kind() != "identifier" {
            continue;
        }
        let Some(r) = m.reference_at(id.start_byte()) else { continue };
        let Some(sym) = r.symbol else { continue };
        // Its group: the comparisons and conditionals around it, up to the test.
        let mut group = id;
        let mut x = id;
        let mut through_member = false;
        while x.id() != test.id() {
            let Some(p) = x.parent() else { break };
            if matches!(p.kind(), "member_expression" | "subscript_expression" | "call_expression" | "new_expression") {
                through_member = true;
            }
            if is_group(p, src) {
                group = p;
            }
            x = p;
        }
        if through_member || (group.id() != id.id() && dynamic(group)) {
            continue;
        }
        let members: Vec<SymbolId> = if group.id() == id.id() {
            vec![sym]
        } else {
            let mut ids = Vec::new();
            crate::util::descendants(group, &|x| fn_like(x.kind()), &mut ids);
            ids.into_iter().filter_map(|i| m.reference_at(i.start_byte()).and_then(|r| r.symbol)).collect()
        };
        if members.iter().any(|s| modified(*s)) || !done.insert(r.name.clone()) {
            continue;
        }
        out.push(finding(id, format!("`{}` is not changed in this loop, so the condition never changes: change it in the body or rewrite the condition", r.name), vec![]));
    }
});

// ---- ESLint: no-use-before-define -------------------------------------------

fn opt_bool(o: &BTreeMap<String, Value>, key: &str, default: bool) -> bool {
    o.get(key).and_then(Value::as_bool).unwrap_or(default)
}

srule!(NoUseBeforeDefine, "js/no-use-before-define", &["program"], |n, ctx, m, out| {
    let o = ctx.options;
    let (functions, classes, variables, named_exports) = (opt_bool(o, "functions", true), opt_bool(o, "classes", true), opt_bool(o, "variables", true), opt_bool(o, "allowNamedExports", false));
    for r in &m.references {
        if r.type_only {
            continue;
        }
        let Some(sym) = r.symbol else { continue };
        let s = m.symbol(sym);
        if matches!(s.kind, SymbolKind::Interface | SymbolKind::TypeAlias | SymbolKind::TypeParam | SymbolKind::Param) {
            continue;
        }
        let Some(&(ds, de)) = s.decls.first() else { continue };
        let Some(id) = node_at(n, r.start, r.end) else { continue };
        let Some(decl) = node_at(n, ds, de) else { continue };
        if crate::util::in_ambient(decl) {
            continue;
        }
        let before = if de <= r.start {
            // `const a = a + 1`: read while its own declarator runs.
            let own_init = decl.parent().filter(|p| p.kind() == "variable_declarator").and_then(|p| p.child_by_field_name("value"));
            own_init.is_some_and(|v| within(id, v) && !ancestors(id).take_while(|a| a.id() != v.id()).any(|a| fn_like(a.kind())))
        } else {
            true
        };
        if !before {
            continue;
        }
        if named_exports && id.parent().is_some_and(|p| p.kind() == "export_specifier") {
            continue;
        }
        let outer = enclosing_fn(id).map(|f| f.id()) != enclosing_fn(decl).map(|f| f.id());
        let forbidden = match s.kind {
            SymbolKind::Function => functions,
            SymbolKind::Class if outer => classes,
            _ if outer => variables,
            _ => true,
        };
        if forbidden {
            out.push(finding(id, format!("`{}` is used before it is defined: move the definition above this line", r.name), vec![]));
        }
    }
});

// ---- unicorn: no-repeated-call-wrappers --------------------------------------

srule!(NoRepeatedCallWrappers, "js/no-repeated-call-wrappers", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    for (i, s) in m.symbols.iter().enumerate() {
        if s.exported || s.decls.len() != 1 || !matches!(s.kind, SymbolKind::Function | SymbolKind::Const | SymbolKind::Let) {
            continue;
        }
        let (a, b) = s.decls[0];
        let Some(id) = node_at(n, a, b) else { continue };
        let f = match s.kind {
            SymbolKind::Function => id.parent().filter(|p| p.kind() == "function_declaration"),
            _ => js_ast::initial_value(m, n, id).and_then(js_ast::as_function),
        };
        let Some(f) = f else { continue };
        if js_ast::is_async(f) || is_generator(f.kind()) {
            continue;
        }
        let refs = refs_of(m, i);
        if refs.len() < 2 || refs.iter().any(|r| r.write || r.type_only) {
            continue;
        }
        let mut wrapper: Option<String> = None;
        let mut ok = true;
        for r in &refs {
            let Some(rid) = node_at(n, r.start, r.end) else { ok = false; break };
            let call = rid.parent().filter(|p| p.kind() == "call_expression" && same(p.child_by_field_name("function"), rid) && !is_optional(*p));
            let Some(call) = call.filter(|c| !within(*c, f)) else { ok = false; break };
            let args = call.parent().filter(|p| p.kind() == "arguments" && named_children(*p).len() == 1);
            let w = args.and_then(|a| a.parent()).and_then(|w| match w.kind() {
                "new_expression" => w.child_by_field_name("constructor").filter(|c| c.kind() == "identifier").map(|c| format!("new {}", text(c, src))),
                "call_expression" => w.child_by_field_name("function").filter(|c| c.kind() == "identifier" && matches!(text(*c, src), "String" | "Number" | "Boolean" | "BigInt")).map(|c| text(c, src).to_owned()),
                _ => None,
            });
            match (&wrapper, w) {
                (_, None) => {
                    ok = false;
                    break;
                }
                (None, Some(w)) => wrapper = Some(w),
                (Some(a), Some(w)) if *a == w => {}
                _ => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && let Some(w) = wrapper {
            out.push(finding(id, format!("Every call of `{}` is wrapped in `{w}(…)`: do that inside `{}` and return the result", s.name, s.name), vec![]));
        }
    }
});


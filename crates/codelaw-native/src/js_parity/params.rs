//! W-139 strand W-139-jsfinal: eslint-plugin-unicorn v79.0.0's
//! `no-unnecessary-parameters`, read from its source and tests and written
//! again. A parameter that gets the same value at every known call of a
//! function defined in the file is reported; its value can be written into
//! the function and the argument dropped from every call.
//!
//! The functions it follows are the ones whose every use is a direct call:
//! `function` declarations, `const f = () => …`, explicit constructors of
//! classes called only with `new`, and `#private` methods. An exported
//! function, a function whose binding is written, one used in any other way,
//! one that reads its `arguments`, and any file with `eval(…)` or `with` are
//! left alone.

use super::kit::{comment_in, number, srule};
use super::scope::{ancestors, enclosing_fn, fn_like};
use super::{JS, span};
use crate::js_ast::{self, arguments, root_of};
use crate::util::{children, named_children, text};
use crate::as_suggestion;
use codelaw_core::TextEdit;
use codelaw_semantic::{SemanticModel, SymbolId, SymbolKind};
use std::collections::BTreeMap;
use tree_sitter::Node;

fn is_wrapper(k: &str) -> bool {
    matches!(k, "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion")
}

fn unwrap_ts(mut n: Node<'_>) -> Node<'_> {
    loop {
        match n.kind() {
            "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" => match n.named_child(0) {
                Some(x) => n = x,
                None => return n,
            },
            "type_assertion" => match named_children(n).last() {
                Some(x) => n = *x,
                None => return n,
            },
            _ => return n,
        }
    }
}

fn outermost(mut n: Node<'_>) -> Node<'_> {
    while let Some(p) = n.parent() {
        if is_wrapper(p.kind()) && unwrap_ts(p).id() == unwrap_ts(n).id() {
            n = p;
        } else {
            break;
        }
    }
    n
}

fn inside(n: Node<'_>, outer: Node<'_>) -> bool {
    n.start_byte() >= outer.start_byte() && n.end_byte() <= outer.end_byte()
}

// ---- values -----------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum Prim {
    Undefined,
    Null,
    Bool(bool),
    Num(u64),
    Str(String),
    Big(String),
}

#[derive(Clone, Debug)]
enum Value<'t> {
    Prim(Prim, Option<Node<'t>>),
    /// A binding of the file.
    Binding(SymbolId, Option<Node<'t>>),
    /// A global of the language (`Math`, `Infinity`).
    Global(String, Option<Node<'t>>),
}

impl Value<'_> {
    fn is_undefined(&self) -> bool {
        matches!(self, Value::Prim(Prim::Undefined, _))
    }
    fn same(&self, other: &Value<'_>) -> bool {
        match (self, other) {
            (Value::Prim(a, _), Value::Prim(b, _)) => a == b,
            (Value::Binding(a, _), Value::Binding(b, _)) => a == b,
            (Value::Global(a, _), Value::Global(b, _)) => a == b,
            _ => false,
        }
    }
    fn node(&self) -> Option<Node<'_>> {
        match self {
            Value::Prim(_, n) | Value::Binding(_, n) | Value::Global(_, n) => *n,
        }
    }
}

/// A bigint literal's value, so `1n` and `0x1n` are the same.
fn bigint_key(digits: &str) -> String {
    let d: String = digits.chars().filter(|c| *c != '_').collect();
    let lower = d.to_ascii_lowercase();
    for (prefix, radix) in [("0x", 16), ("0o", 8), ("0b", 2)] {
        if let Some(rest) = lower.strip_prefix(prefix) {
            return u128::from_str_radix(rest, radix).map_or(d.clone(), |v| v.to_string());
        }
    }
    lower.parse::<u128>().map_or(d, |v| v.to_string())
}

fn num_key(v: f64) -> u64 {
    if v.is_nan() { f64::NAN.to_bits() } else { v.to_bits() }
}

/// The value of a primitive literal: strings, numbers (signed too), `true`,
/// `false`, `null`, an expression-free template.
fn primitive(n: Node<'_>, src: &str) -> Option<Prim> {
    let n = unwrap_ts(n);
    match n.kind() {
        "string" => Some(Prim::Str(crate::util::unquote(text(n, src)).to_owned())),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Some(Prim::Str(crate::util::unquote(text(n, src)).to_owned())),
        "number" => {
            let t = text(n, src);
            if let Some(digits) = t.strip_suffix('n') {
                Some(Prim::Big(bigint_key(digits)))
            } else {
                number(n, src).map(|v| Prim::Num(num_key(v)))
            }
        }
        "true" => Some(Prim::Bool(true)),
        "false" => Some(Prim::Bool(false)),
        "null" => Some(Prim::Null),
        "unary_expression" => {
            let op = n.child(0).map(|c| text(c, src))?;
            let arg = unwrap_ts(n.child_by_field_name("argument")?);
            if arg.kind() != "number" {
                return None;
            }
            let v = number(arg, src);
            match (op, v) {
                ("-", Some(v)) => Some(Prim::Num(num_key(-v))),
                ("+", Some(v)) => Some(Prim::Num(num_key(v))),
                ("-", None) if text(arg, src).ends_with('n') => Some(Prim::Big(format!("-{}", bigint_key(text(arg, src).trim_end_matches('n'))))),
                _ => None,
            }
        }
        _ => None,
    }
}

fn reassigned(m: &SemanticModel, sym: SymbolId) -> bool {
    let s = m.symbol(sym);
    s.decls.len() > 1 || s.refs.iter().map(|&r| &m.references[r]).any(|r| r.write && !s.decls.iter().any(|d| d.0 == r.start))
}

struct Ctx<'a, 't> {
    m: &'a SemanticModel,
    src: &'a str,
    root: Node<'t>,
}

impl<'a, 't> Ctx<'a, 't> {
    fn value(&self, node: Option<Node<'t>>) -> Option<Value<'t>> {
        let Some(node) = node else { return Some(Value::Prim(Prim::Undefined, None)) };
        let n = unwrap_ts(node);
        if let Some(p) = primitive(n, self.src) {
            return Some(Value::Prim(p, Some(n)));
        }
        if n.kind() == "unary_expression" && n.child(0).is_some_and(|c| text(c, self.src) == "void") && n.child_by_field_name("argument").is_some_and(|a| number(unwrap_ts(a), self.src) == Some(0.0)) {
            return Some(Value::Prim(Prim::Undefined, Some(n)));
        }
        if n.kind() == "undefined" {
            return match lookup_at(self.m, n.start_byte(), "undefined") {
                Some(sym) => self.value_of_reference(n, Some(sym), "undefined"),
                None => Some(Value::Prim(Prim::Undefined, Some(n))),
            };
        }
        if n.kind() != "identifier" {
            return None;
        }
        let r = self.m.reference_at(n.start_byte())?;
        match r.symbol {
            None => {
                if r.name == "undefined" {
                    Some(Value::Prim(Prim::Undefined, Some(n)))
                } else if self.m.is_builtin_reference(r) {
                    Some(Value::Global(r.name.clone(), Some(n)))
                } else {
                    None
                }
            }
            Some(sym) => {
                let s = self.m.symbol(sym);
                if reassigned(self.m, sym) || s.decls.len() != 1 || s.kind == SymbolKind::Var || matches!(s.kind, SymbolKind::Import | SymbolKind::Interface | SymbolKind::TypeAlias | SymbolKind::Enum | SymbolKind::Namespace | SymbolKind::TypeParam | SymbolKind::Variable) {
                    return None;
                }
                Some(Value::Binding(sym, Some(n)))
            }
        }
    }
}

// ---- targets -------------------------------------------------------------------

struct Target<'t> {
    func: Node<'t>,
    calls: Vec<Node<'t>>,
}

fn call_of<'t>(ident: Node<'t>) -> Option<Node<'t>> {
    let n = outermost(ident);
    let p = n.parent()?;
    match p.kind() {
        "call_expression" if p.child_by_field_name("function").is_some_and(|f| f.id() == n.id()) => Some(p),
        "new_expression" if p.child_by_field_name("constructor").is_some_and(|f| f.id() == n.id()) => Some(p),
        _ => None,
    }
}

fn has_decorators(n: Node<'_>) -> bool {
    children(n).iter().any(|c| c.kind() == "decorator")
}

/// The function a declaring identifier stands for, with whether the
/// declaration is a class (whose uses must all be `new`).
fn function_of<'t>(id: Node<'t>, src: &str) -> Option<(Node<'t>, bool)> {
    let parent = id.parent()?;
    match parent.kind() {
        "function_declaration" | "generator_function_declaration" if parent.child_by_field_name("name").is_some_and(|x| x.id() == id.id()) && parent.child_by_field_name("body").is_some() => Some((parent, false)),
        "function_expression" | "generator_function" | "function" if parent.child_by_field_name("name").is_some_and(|x| x.id() == id.id()) => Some((parent, false)),
        "variable_declarator" if parent.child_by_field_name("name").is_some_and(|x| x.id() == id.id()) => {
            let init = unwrap_ts(parent.child_by_field_name("value")?);
            if js_ast::is_function(init) && init.kind() != "method_definition" {
                return Some((init, false));
            }
            if matches!(init.kind(), "class") && !has_decorators(init) {
                return constructor_of(init, src).map(|c| (c, true));
            }
            None
        }
        "class_declaration" | "abstract_class_declaration" if parent.child_by_field_name("name").is_some_and(|x| x.id() == id.id()) && !has_decorators(parent) => constructor_of(parent, src).map(|c| (c, true)),
        _ => None,
    }
}

fn constructor_of<'t>(class: Node<'t>, src: &str) -> Option<Node<'t>> {
    let body = class.child_by_field_name("body")?;
    named_children(body).into_iter().find(|m| m.kind() == "method_definition" && m.child_by_field_name("name").is_some_and(|n| text(n, src) == "constructor") && m.child_by_field_name("body").is_some())
}

fn dynamic_scope(root: Node<'_>, m: &SemanticModel, src: &str) -> bool {
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        let direct_eval = x.kind() == "call_expression"
            && !super::kit::optional(x)
            && x.child_by_field_name("function").map(unwrap_ts).is_some_and(|f| f.kind() == "identifier" && text(f, src) == "eval" && m.reference_at(f.start_byte()).is_some_and(|r| r.symbol.is_none()));
        if x.kind() == "with_statement" || direct_eval {
            return true;
        }
        stack.extend(children(x));
    }
    false
}

fn targets<'t>(root: Node<'t>, m: &SemanticModel, src: &str) -> Vec<Target<'t>> {
    if dynamic_scope(root, m, src) {
        return Vec::new();
    }
    let mut found: BTreeMap<usize, (Target<'t>, bool)> = BTreeMap::new();
    for (sym_id, s) in m.symbols.iter().enumerate() {
        if !matches!(s.kind, SymbolKind::Function | SymbolKind::Const | SymbolKind::Let | SymbolKind::Var | SymbolKind::Class) || s.decls.len() != 1 {
            continue;
        }
        let (ds, de) = s.decls[0];
        let Some(id) = root.descendant_for_byte_range(ds, de) else { continue };
        let Some((func, class)) = function_of(id, src) else { continue };
        let mut excluded = s.exported || reassigned(m, sym_id);
        let mut calls = Vec::new();
        for r in s.refs.iter().map(|&r| &m.references[r]) {
            if r.start == ds || r.type_only {
                continue;
            }
            let Some(ident) = root.descendant_for_byte_range(r.start, r.end) else { continue };
            if ancestors(ident).any(|a| a.kind() == "type_query") {
                continue;
            }
            match call_of(ident) {
                Some(call) if !(class && call.kind() != "new_expression") => calls.push(call),
                _ => excluded = true,
            }
        }
        let entry = found.entry(func.id()).or_insert((Target { func, calls: Vec::new() }, false));
        entry.0.calls.extend(calls);
        entry.1 |= excluded;
    }
    // `#private` methods called as `this.#m(…)`.
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        stack.extend(children(x));
        if x.kind() != "member_expression" {
            continue;
        }
        let Some(p) = x.child_by_field_name("property").filter(|p| p.kind() == "private_property_identifier") else { continue };
        let name = text(p, src);
        let Some(method) = ancestors(x).filter(|a| a.kind() == "class_body").find_map(|b| named_children(b).into_iter().find(|e| e.child_by_field_name("name").is_some_and(|k| k.kind() == "private_property_identifier" && text(k, src) == name))) else { continue };
        if method.kind() != "method_definition" || method.child_by_field_name("body").is_none() || children(method).iter().any(|c| matches!(text(*c, src), "get" | "set") && !c.is_named()) {
            continue;
        }
        let entry = found.entry(method.id()).or_insert((Target { func: method, calls: Vec::new() }, false));
        match call_of(x) {
            Some(call) if !has_decorators(method) => entry.0.calls.push(call),
            _ => entry.1 = true,
        }
    }
    found.into_values().filter(|(_, ex)| !ex).map(|(t, _)| t).collect()
}

// ---- parameters ---------------------------------------------------------------------

struct Param<'t> {
    /// The parameter node, or the property of a destructured one.
    node: Node<'t>,
    ident: Node<'t>,
    index: usize,
    default: Option<Node<'t>>,
    /// The key of a property of a destructured parameter.
    property: Option<String>,
    /// The destructured parameter's own default (an empty object).
    outer_default: bool,
    list_node: Node<'t>,
}

/// The parameter a `x is T` or `asserts x` return type names.
fn predicate_target<'t>(return_type: Node<'t>, src: &str) -> Option<String> {
    let mut stack = vec![return_type];
    while let Some(x) = stack.pop() {
        if x.kind() == "type_predicate" {
            return x.child_by_field_name("name").map(|n| text(n, src).to_owned());
        }
        if x.kind() == "asserts"
            && let Some(id) = named_children(x).into_iter().find(|c| c.kind() == "identifier")
        {
            return Some(text(id, src).to_owned());
        }
        stack.extend(named_children(x));
    }
    None
}

/// The function type an annotation stands for, looking through a type alias of the file.
fn function_type_of<'t>(annotation: Node<'t>, m: &SemanticModel, root: Node<'t>, depth: usize) -> Option<Node<'t>> {
    let t = if annotation.kind() == "type_annotation" { annotation.named_child(0)? } else { annotation };
    match t.kind() {
        "function_type" => Some(t),
        "parenthesized_type" => function_type_of(t.named_child(0)?, m, root, depth),
        "type_identifier" if depth < 4 => {
            let sym = m.symbol_at(t.start_byte())?;
            let (s, e) = *m.symbol(sym).decls.first()?;
            let id = root.descendant_for_byte_range(s, e)?;
            let decl = id.parent().filter(|p| p.kind() == "type_alias_declaration")?;
            function_type_of(decl.child_by_field_name("value")?, m, root, depth + 1)
        }
        _ => None,
    }
}

fn type_parameter_names<'t>(ft: Node<'t>, src: &str) -> Vec<String> {
    let Some(ps) = ft.child_by_field_name("parameters") else { return Vec::new() };
    named_children(ps)
        .into_iter()
        .filter(|p| matches!(p.kind(), "required_parameter" | "optional_parameter" | "rest_parameter"))
        .filter_map(|p| p.child_by_field_name("pattern").filter(|x| x.kind() != "this").map(|x| text(x, src).to_owned()))
        .collect()
}

/// The JSDoc comment before a function, a variable declaring it or an export of it.
fn jsdoc_before<'t>(func: Node<'t>) -> Option<Node<'t>> {
    let mut at = func;
    for _ in 0..4 {
        if let Some(c) = at.prev_sibling().filter(|c| c.kind() == "comment") {
            return Some(c);
        }
        match at.parent() {
            Some(p) if matches!(p.kind(), "variable_declarator" | "lexical_declaration" | "variable_declaration" | "export_statement") => at = p,
            _ => return None,
        }
    }
    None
}

fn predicate_names(func: Node<'_>, src: &str, m: &SemanticModel, root: Node<'_>) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(rt) = func.child_by_field_name("return_type")
        && let Some(n) = predicate_target(rt, src)
    {
        names.push(n);
    }
    // A contextual function type: `const f: (x: unknown) => x is T = …`.
    let own: Vec<String> = js_ast::params(func).into_iter().filter_map(|p| js_ast::param_name(p, src).map(str::to_owned)).collect();
    if let Some(ann) = func.parent().filter(|p| p.kind() == "variable_declarator").and_then(|d| d.child_by_field_name("type"))
        && let Some(ft) = function_type_of(ann, m, root, 0)
        && let Some(rt) = ft.child_by_field_name("return_type")
        && let Some(target) = predicate_target(rt, src)
        && let Some(i) = type_parameter_names(ft, src).iter().position(|n| *n == target)
        && let Some(n) = own.get(i)
    {
        names.push(n.clone());
    }
    // A JSDoc `@returns {x is T}`.
    if let Some(c) = jsdoc_before(func) {
        let t = text(c, src);
        for tag in ["@returns", "@return"] {
            if let Some(i) = t.find(tag)
                && let Some(open) = t[i..].find('{')
                && let Some(close) = t[i + open..].find('}')
            {
                let inner = t[i + open + 1..i + open + close].trim();
                let inner = inner.strip_prefix("asserts ").unwrap_or(inner);
                let name: String = inner.chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$').collect();
                if !name.is_empty() && (t[i + open + 1..i + open + close].contains(" is ") || t[i + open + 1..i + open + close].trim().starts_with("asserts ")) {
                    names.push(name);
                }
            }
        }
    }
    names
}

/// A string literal's characters with its escapes read.
fn cooked(raw: &str) -> String {
    let mut out = String::new();
    let mut it = raw.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some('x') => {
                let h: String = it.by_ref().take(2).collect();
                out.extend(u32::from_str_radix(&h, 16).ok().and_then(char::from_u32));
            }
            Some('u') => {
                let h: String = if it.peek() == Some(&'{') {
                    it.next();
                    it.by_ref().take_while(|c| *c != '}').collect()
                } else {
                    it.by_ref().take(4).collect()
                };
                out.extend(u32::from_str_radix(&h, 16).ok().and_then(char::from_u32));
            }
            Some(o) => out.push(o),
            None => {}
        }
    }
    out
}

fn static_key(k: Node<'_>, src: &str) -> Option<String> {
    match k.kind() {
        "property_identifier" => Some(text(k, src).to_owned()),
        "string" => Some(cooked(crate::util::unquote(text(k, src)))),
        "number" => number(k, src).map(|v| format!("{v}")),
        "computed_property_name" => {
            let inner = unwrap_ts(k.named_child(0)?);
            match inner.kind() {
                "string" => Some(cooked(crate::util::unquote(text(inner, src)))),
                "template_string" if !named_children(inner).iter().any(|c| c.kind() == "template_substitution") => Some(cooked(crate::util::unquote(text(inner, src)))),
                "number" => number(inner, src).map(|v| format!("{v}")),
                _ => None,
            }
        }
        _ => None,
    }
}

fn object_pattern_properties<'t>(pat: Node<'t>, src: &str) -> Option<Vec<(String, Node<'t>, Node<'t>, Option<Node<'t>>)>> {
    let mut out: Vec<(String, Node<'t>, Node<'t>, Option<Node<'t>>)> = Vec::new();
    for p in named_children(pat).into_iter().filter(|c| c.kind() != "comment") {
        let (name, ident, default, prop) = match p.kind() {
            "rest_pattern" => continue,
            "shorthand_property_identifier_pattern" => (text(p, src).to_owned(), p, None, p),
            "object_assignment_pattern" => {
                let l = p.child_by_field_name("left").filter(|l| l.kind() == "shorthand_property_identifier_pattern")?;
                (text(l, src).to_owned(), l, p.child_by_field_name("right"), p)
            }
            "pair_pattern" => {
                let name = static_key(p.child_by_field_name("key")?, src)?;
                let v = p.child_by_field_name("value")?;
                match v.kind() {
                    "identifier" => (name, v, None, p),
                    "assignment_pattern" => (name, v.child_by_field_name("left").filter(|l| l.kind() == "identifier")?, v.child_by_field_name("right"), p),
                    _ => (name, v, None, p),
                }
            }
            _ => return None,
        };
        if name == "__proto__" || out.iter().any(|o| o.0 == name) {
            return None;
        }
        out.push((name, ident, prop, default));
    }
    Some(out)
}

fn parameters<'t>(func: Node<'t>, src: &str, m: &SemanticModel, root: Node<'t>) -> Vec<Param<'t>> {
    let predicates = predicate_names(func, src, m, root);
    let mut out = Vec::new();
    let list: Vec<Node<'t>> = js_ast::params(func);
    let mut index = 0;
    for p in list {
        // The pattern and default of one parameter, in JavaScript or TypeScript.
        let (pattern, default) = match p.kind() {
            "assignment_pattern" => (p.child_by_field_name("left"), p.child_by_field_name("right")),
            "required_parameter" | "optional_parameter" => {
                if children(p).iter().any(|c| matches!(c.kind(), "accessibility_modifier" | "override_modifier") || (!c.is_named() && text(*c, src) == "readonly")) {
                    index += 1;
                    continue;
                }
                (p.child_by_field_name("pattern"), p.child_by_field_name("value"))
            }
            _ => (Some(p), None),
        };
        let Some(pattern) = pattern else { continue };
        if pattern.kind() == "this" {
            continue;
        }
        let position = index;
        index += 1;
        match pattern.kind() {
            "identifier" => {
                if predicates.iter().any(|n| n == text(pattern, src)) {
                    continue;
                }
                out.push(Param { node: p, ident: pattern, index: position, default, property: None, outer_default: false, list_node: p });
            }
            "object_pattern" => {
                if default.is_some_and(|d| d.kind() != "object" || !named_children(d).is_empty()) {
                    continue;
                }
                let Some(props) = object_pattern_properties(pattern, src) else { continue };
                for (name, ident, prop, pd) in props {
                    if ident.kind() == "identifier" || ident.kind() == "shorthand_property_identifier_pattern" {
                        out.push(Param { node: prop, ident, index: position, default: pd, property: Some(name), outer_default: default.is_some(), list_node: p });
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn object_proto_name(n: &str) -> bool {
    matches!(n, "constructor" | "hasOwnProperty" | "isPrototypeOf" | "propertyIsEnumerable" | "toLocaleString" | "toString" | "valueOf" | "__defineGetter__" | "__defineSetter__" | "__lookupGetter__" | "__lookupSetter__" | "__proto__")
}

/// The properties of an object literal by static name, `None` when any
/// member is not a plain `key: value` or shorthand.
fn object_properties<'t>(obj: Node<'t>, src: &str) -> Option<BTreeMap<String, Node<'t>>> {
    let mut out = BTreeMap::new();
    for p in named_children(obj).into_iter().filter(|c| c.kind() != "comment") {
        let name = match p.kind() {
            "pair" => static_key(p.child_by_field_name("key")?, src)?,
            "shorthand_property_identifier" => text(p, src).to_owned(),
            _ => return None,
        };
        if name == "__proto__" || out.insert(name, p).is_some() {
            return None;
        }
    }
    Some(out)
}

struct Incoming<'t> {
    /// The argument node (or the property of an object argument).
    node: Option<Node<'t>>,
    value: Option<Value<'t>>,
    forwarding: bool,
}

fn arguments_for<'t>(cx: &Ctx<'_, 't>, target: &Target<'t>, p: &Param<'t>, psym: Option<SymbolId>) -> Option<Vec<Incoming<'t>>> {
    let mut out = Vec::new();
    for call in &target.calls {
        let args = arguments(*call);
        if args.iter().take(p.index + 1).any(|a| a.kind() == "spread_element") {
            return None;
        }
        let arg = args.get(p.index).copied();
        if let Some(name) = &p.property {
            let mut object = arg.map(unwrap_ts);
            if (object.is_none() || cx.value(object).is_some_and(|v| v.is_undefined())) && p.outer_default {
                object = None;
                out.push(property_value(cx, None, name)?);
                continue;
            }
            let object = object.filter(|o| o.kind() == "object")?;
            let props = object_properties(object, cx.src)?;
            let prop = props.get(name).copied();
            if prop.is_none() && object_proto_name(name) {
                return None;
            }
            out.push(property_value(cx, prop, name)?);
            continue;
        }
        if let Some(a) = arg {
            let un = unwrap_ts(a);
            if un.kind() == "identifier" && inside(*call, target.func) && psym.is_some() && cx.m.reference_at(un.start_byte()).and_then(|r| r.symbol) == psym {
                if reassigned(cx.m, psym?) {
                    return None;
                }
                out.push(Incoming { node: Some(a), value: None, forwarding: true });
                continue;
            }
        }
        out.push(Incoming { node: arg, value: cx.value(arg), forwarding: false });
    }
    Some(out)
}

fn property_value<'t>(cx: &Ctx<'_, 't>, prop: Option<Node<'t>>, _name: &str) -> Option<Incoming<'t>> {
    let value_node = prop.and_then(|p| match p.kind() {
        "pair" => p.child_by_field_name("value"),
        _ => Some(p),
    });
    // A shorthand `{ name }` stands for the binding `name`.
    let value = if prop.is_some_and(|p| p.kind() == "shorthand_property_identifier") {
        let r = cx.m.reference_at(prop?.start_byte())?;
        cx.value_of_reference(prop?, r.symbol, &r.name)
    } else {
        cx.value(value_node)
    };
    Some(Incoming { node: prop, value, forwarding: false })
}

impl<'a, 't> Ctx<'a, 't> {
    fn value_of_reference(&self, node: Node<'t>, symbol: Option<SymbolId>, name: &str) -> Option<Value<'t>> {
        match symbol {
            None if name == "undefined" => Some(Value::Prim(Prim::Undefined, Some(node))),
            None => self.m.reference_at(node.start_byte()).filter(|r| self.m.is_builtin_reference(r)).map(|_| Value::Global(name.to_owned(), Some(node))),
            Some(sym) => {
                let s = self.m.symbol(sym);
                if reassigned(self.m, sym) || s.decls.len() != 1 || s.kind == SymbolKind::Var || matches!(s.kind, SymbolKind::Import | SymbolKind::Interface | SymbolKind::TypeAlias | SymbolKind::Enum | SymbolKind::Namespace | SymbolKind::TypeParam | SymbolKind::Variable) {
                    None
                } else {
                    Some(Value::Binding(sym, Some(node)))
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Why {
    Same,
    Default,
    Undefined,
}

struct Verdict<'t> {
    why: Why,
    value: Option<Value<'t>>,
}

/// The scope-chain lookup of `name` from outside the function.
fn outer_symbol(m: &SemanticModel, func: Node<'_>, name: &str) -> Option<SymbolId> {
    let mut sc = m.scope_at(func.start_byte());
    while m.scopes[sc].start >= func.start_byte() && m.scopes[sc].end <= func.end_byte() {
        match m.scopes[sc].parent {
            Some(p) => sc = p,
            None => break,
        }
    }
    let mut cur = Some(sc);
    while let Some(c) = cur {
        if let Some(ids) = m.scopes[c].bindings.get(name)
            && let Some(&id) = ids.iter().find(|&&i| m.symbol(i).kind.is_value())
        {
            return Some(id);
        }
        cur = m.scopes[c].parent;
    }
    None
}

fn is_outer_binding(m: &SemanticModel, value: &Value<'_>, func: Node<'_>) -> bool {
    match value {
        Value::Binding(sym, _) => outer_symbol(m, func, &m.symbol(*sym).name) == Some(*sym),
        Value::Global(name, _) => outer_symbol(m, func, name).is_none(),
        Value::Prim(..) => true,
    }
}

fn verdict<'t>(cx: &Ctx<'_, 't>, p: &Param<'t>, incoming: &[Incoming<'t>], func: Node<'t>) -> Option<Verdict<'t>> {
    let real: Vec<&Incoming<'t>> = incoming.iter().filter(|a| !a.forwarding).collect();
    if real.iter().any(|a| a.value.is_none()) {
        return None;
    }
    let default_value = match p.default {
        Some(d) => cx.value(Some(d)),
        None => Some(Value::Prim(Prim::Undefined, None)),
    };
    let forwarding = incoming.iter().any(|a| a.forwarding);
    if real.iter().all(|a| a.value.as_ref().is_some_and(Value::is_undefined)) {
        if forwarding {
            match &default_value {
                None => return None,
                Some(v) if matches!(v, Value::Binding(..)) && !is_outer_binding(cx.m, v, func) => return None,
                _ => {}
            }
        }
        return Some(Verdict { why: if p.default.is_some() { Why::Default } else { Why::Undefined }, value: default_value });
    }
    let values: Vec<Option<Value<'t>>> = real
        .iter()
        .map(|a| match a.value.as_ref() {
            Some(v) if v.is_undefined() => default_value.clone(),
            other => other.cloned(),
        })
        .collect();
    let first = values.first()?.clone()?;
    if values.iter().any(|v| v.as_ref().is_none_or(|v| !first.same(v))) {
        return None;
    }
    if matches!(first, Value::Binding(..) | Value::Global(..)) && !is_outer_binding(cx.m, &first, func) {
        return None;
    }
    let same_as_default = p.default.is_some() && default_value.as_ref().is_some_and(|d| first.same(d));
    Some(Verdict { why: if same_as_default { Why::Default } else { Why::Same }, value: Some(first) })
}

// ---- the fix --------------------------------------------------------------------------

fn param_removal(func: Node<'_>, p: &Param<'_>, src: &str) -> Option<(usize, usize, &'static str)> {
    let node = p.node;
    if func.kind() == "arrow_function" && func.child_by_field_name("parameter").is_some_and(|x| x.id() == node.id()) {
        return Some((node.start_byte(), node.end_byte(), "()"));
    }
    let list = node.parent()?;
    let kids = children(list);
    let pos = kids.iter().position(|c| c.id() == node.id())?;
    let params: Vec<usize> = kids.iter().enumerate().filter(|(_, c)| c.is_named() && c.kind() != "comment").map(|(i, _)| i).collect();
    let nth = params.iter().position(|&i| i == pos)?;
    let (start, end);
    if nth + 1 < params.len() {
        let comma = kids[pos + 1..].iter().find(|c| text(**c, src) == ",")?;
        let after = kids.iter().find(|c| c.start_byte() >= comma.end_byte() && c.start_byte() > node.end_byte() && (c.is_named() || !matches!(text(**c, src), ",")))?;
        start = node.start_byte();
        end = after.start_byte();
    } else if nth > 0 {
        let comma = kids[..pos].iter().rfind(|c| text(**c, src) == ",")?;
        start = comma.start_byte();
        end = node.end_byte();
    } else {
        start = node.start_byte();
        end = kids.get(pos + 1).filter(|c| text(**c, src) == ",").map_or(node.end_byte(), |c| c.end_byte());
    }
    Some((start, end, ""))
}

fn argument_removal(arg: Node<'_>, src: &str) -> Option<(usize, usize)> {
    let list = arg.parent()?;
    let kids = children(list);
    let pos = kids.iter().position(|c| c.id() == arg.id())?;
    let n = kids.iter().filter(|c| c.is_named() && c.kind() != "comment").count();
    let idx = kids[..pos].iter().filter(|c| c.is_named() && c.kind() != "comment").count();
    if n == 1 {
        let end = kids.get(pos + 1).filter(|c| text(**c, src) == ",").map_or(arg.end_byte(), |c| c.end_byte());
        return Some((arg.start_byte(), end));
    }
    if idx == 0 {
        let comma = kids.get(pos + 1).filter(|c| text(**c, src) == ",")?;
        let after = kids.get(pos + 2)?;
        let _ = comma;
        return Some((arg.start_byte(), after.start_byte()));
    }
    let comma = kids[..pos].iter().rfind(|c| text(**c, src) == ",")?;
    Some((comma.start_byte(), arg.end_byte()))
}

fn same_variable_scope(root: Node<'_>, at: usize, other: Option<Node<'_>>) -> bool {
    let a = root.descendant_for_byte_range(at, at + 1).and_then(enclosing_fn);
    a.map(|n| n.id()) == other.map(|n| n.id())
}

fn can_inline(cx: &Ctx<'_, '_>, psym: SymbolId, v: &Verdict<'_>, args: &[Incoming<'_>], func: Node<'_>) -> bool {
    if reassigned(cx.m, psym) {
        return false;
    }
    let Some(value) = &v.value else { return false };
    match value {
        Value::Prim(..) => true,
        Value::Global(..) => false,
        Value::Binding(sym, _) => {
            if v.why == Why::Default {
                return false;
            }
            let s = cx.m.symbol(*sym);
            if matches!(s.kind, SymbolKind::Param | SymbolKind::CatchParam) {
                return false;
            }
            let (ds, de) = s.decls[0];
            let Some(id) = cx.root.descendant_for_byte_range(ds, de) else { return false };
            if matches!(s.kind, SymbolKind::Const | SymbolKind::Let | SymbolKind::Var) {
                let Some(decl) = js_ast::declarator_of(cx.m, cx.root, id) else { return false };
                if decl.child_by_field_name("value").is_none() {
                    return false;
                }
                if ancestors(decl).any(|a| matches!(a.kind(), "switch_case" | "switch_default")) {
                    return false;
                }
            }
            let decl_fn = enclosing_fn(id);
            let decl_end = cx.root.descendant_for_byte_range(ds, de).map_or(de, |i| {
                ancestors(i).find(|a| matches!(a.kind(), "variable_declarator" | "function_declaration" | "class_declaration" | "generator_function_declaration")).map_or(de, |x| x.end_byte())
            });
            for a in args {
                let Some(node) = a.node else { continue };
                if !same_variable_scope(cx.root, node.start_byte(), decl_fn) || decl_end > node.start_byte() && !matches!(s.kind, SymbolKind::Function) {
                    return false;
                }
            }
            // No reference in the function may resolve the value's name to something else.
            let name = &s.name;
            for r in cx.m.symbol(psym).refs.iter().map(|&r| &cx.m.references[r]) {
                if r.write {
                    continue;
                }
                let found = lookup_at(cx.m, r.start, name);
                if found != Some(*sym) && found != Some(psym) {
                    return false;
                }
            }
            let _ = func;
            true
        }
    }
}

fn lookup_at(m: &SemanticModel, at: usize, name: &str) -> Option<SymbolId> {
    let mut cur = Some(m.scope_at(at));
    while let Some(c) = cur {
        if let Some(ids) = m.scopes[c].bindings.get(name)
            && let Some(&id) = ids.iter().find(|&&i| m.symbol(i).kind.is_value())
        {
            return Some(id);
        }
        cur = m.scopes[c].parent;
    }
    None
}

fn replacement_text(cx: &Ctx<'_, '_>, ident: Node<'_>, value: &Value<'_>, v: &Verdict<'_>) -> String {
    let _ = v;
    let mut t = match value {
        Value::Prim(Prim::Undefined, _) => "(void 0)".to_owned(),
        Value::Prim(_, Some(n)) => text(*n, cx.src).to_owned(),
        Value::Prim(_, None) => "undefined".to_owned(),
        Value::Binding(sym, _) => cx.m.symbol(*sym).name.clone(),
        Value::Global(name, _) => name.clone(),
    };
    let prim_node = match value {
        Value::Prim(p, Some(n)) if *p != Prim::Undefined => Some((*n, p.clone())),
        _ => None,
    };
    let outer = outermost(ident);
    if let Some((n, p)) = prim_node {
        let parent = outer.parent();
        let member_object = parent.is_some_and(|x| x.kind() == "member_expression" && x.child_by_field_name("object").is_some_and(|o| o.id() == outer.id()));
        let number_like = matches!(p, Prim::Num(_) | Prim::Big(_));
        let new_callee = parent.is_some_and(|x| x.kind() == "new_expression" && x.child_by_field_name("constructor").is_some_and(|o| o.id() == outer.id()));
        let stmt_string = parent.is_some_and(|x| x.kind() == "expression_statement") && matches!(p, Prim::Str(_));
        if n.kind() == "unary_expression" || (member_object && number_like) || new_callee || stmt_string {
            t = format!("({t})");
        }
    }
    // The first thing in a statement after one with no semicolon.
    if let Some(stmt) = ancestors(ident).find(|a| a.kind() == "expression_statement")
        && stmt.start_byte() == ident.start_byte()
        && t.starts_with(['(', '[', '`', '+', '-', '/'])
    {
        let before = cx.src[..ident.start_byte()].trim_end();
        if !before.is_empty() && !before.ends_with([';', '{', '}']) {
            t.insert(0, ';');
        }
    }
    t
}

fn has_ts_annotations(src: &str, lang: codelaw_core::Language) -> bool {
    matches!(lang, codelaw_core::Language::Typescript | codelaw_core::Language::Tsx) || src.contains("@ts-check") || ["@type {", "@param {", "@returns {", "@return {", "@typedef {"].iter().any(|k| src.contains(k))
}

struct Fix {
    edits: Vec<TextEdit>,
}

#[allow(clippy::too_many_arguments)]
fn build_fix<'t>(cx: &Ctx<'_, 't>, target: &Target<'t>, p: &Param<'t>, psym: SymbolId, v: &Verdict<'t>, incoming: &[Incoming<'t>]) -> Option<Fix> {
    let func = target.func;
    let body = func.child_by_field_name("body")?;
    if p.property.is_some() {
        return None;
    }
    let s = cx.m.symbol(psym);
    if s.decls.len() != 1 {
        return None;
    }
    let refs: Vec<_> = s.refs.iter().map(|&r| &cx.m.references[r]).filter(|r| !s.decls.iter().any(|d| d.0 == r.start)).collect();
    if refs.iter().any(|r| r.type_only) {
        return None;
    }
    // Every use is in the body or in the parameter itself.
    if refs.iter().any(|r| !(r.start >= body.start_byte() && r.end <= body.end_byte()) && !(r.start >= p.node.start_byte() && r.end <= p.node.end_byte())) {
        return None;
    }
    if !can_inline(cx, psym, v, incoming, func) {
        return None;
    }
    let value = v.value.as_ref()?;
    let removal = param_removal(func, p, cx.src)?;
    let removals: Vec<Node<'t>> = incoming.iter().filter_map(|a| a.node).filter(|n| !inside(*n, p.node)).collect();
    let mut arg_ranges = Vec::new();
    for r in &removals {
        arg_ranges.push(argument_removal(*r, cx.src)?);
    }
    let head = (func.start_byte(), body.start_byte());
    if comment_in(cx.root, head.0, head.1) || target.calls.iter().any(|c| comment_in(cx.root, c.start_byte(), c.end_byte())) || comment_in(cx.root, removal.0, removal.1) || arg_ranges.iter().any(|(a, b)| comment_in(cx.root, *a, *b)) {
        return None;
    }
    let mut edits = vec![TextEdit::new(removal.0, removal.1, removal.2)];
    for (a, b) in arg_ranges {
        edits.push(TextEdit::new(a, b, ""));
    }
    for r in refs {
        if r.start >= p.node.start_byte() && r.end <= p.node.end_byte() {
            continue;
        }
        let ident = cx.root.descendant_for_byte_range(r.start, r.end)?;
        match ident.kind() {
            "identifier" => {
                if ident.parent().is_some_and(|x| x.kind() == "unary_expression" && x.child(0).is_some_and(|c| c.kind() == "delete")) {
                    return None;
                }
                edits.push(TextEdit::new(r.start, r.end, replacement_text(cx, ident, value, v)));
            }
            "shorthand_property_identifier" => {
                if r.name == "__proto__" {
                    return None;
                }
                edits.push(TextEdit::new(r.start, r.end, format!("{}: {}", r.name, replacement_text(cx, ident, value, v))));
            }
            _ => return None,
        }
    }
    Some(Fix { edits })
}

fn message(why: Why, name: &str) -> String {
    match why {
        Why::Same => format!("Parameter `{name}` receives the same value at every call."),
        Why::Default => format!("Parameter `{name}` always receives its default value."),
        Why::Undefined => format!("Parameter `{name}` is always `undefined`."),
    }
}

srule!(NoUnnecessaryParameters, "js/no-unnecessary-parameters", JS, &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    let root = root_of(n);
    let minimum = ctx.options.get("minimum_call_count").or_else(|| ctx.options.get("minimumCallCount")).and_then(serde_json::Value::as_u64).unwrap_or(2) as usize;
    let cx = Ctx { m, src, root };
    let review = has_ts_annotations(src, ctx.language);
    for target in targets(root, m, src) {
        let func = target.func;
        if target.calls.iter().filter(|c| !inside(**c, func)).count() < minimum {
            continue;
        }
        if named_children(func).iter().any(|c| c.kind() == "decorator") || js_ast::params(func).iter().any(|p| has_decorators(*p)) {
            continue;
        }
        // The function reads its own `arguments`.
        let mut uses_arguments = false;
        let mut stack = vec![func];
        while let Some(x) = stack.pop() {
            if x.kind() == "identifier" && text(x, src) == "arguments" && m.reference_at(x.start_byte()).is_some_and(|r| r.symbol.is_none()) && enclosing_fn_non_arrow(x).is_some_and(|f| f.id() == func.id()) {
                uses_arguments = true;
                break;
            }
            stack.extend(children(x));
        }
        if uses_arguments {
            continue;
        }
        let mut has_fix = false;
        for p in parameters(func, src, m, root) {
            let Some(psym) = m.declared_at(p.ident.start_byte()) else { continue };
            let Some(incoming) = arguments_for(&cx, &target, &p, Some(psym)) else { continue };
            let Some(v) = verdict(&cx, &p, &incoming, func) else { continue };
            let fix = if has_fix && !review { None } else { build_fix(&cx, &target, &p, psym, &v, &incoming) };
            has_fix |= fix.is_some() && !review;
            let name = text(p.ident, src);
            let edits = match fix {
                Some(f) if review => as_suggestion(f.edits),
                Some(f) => f.edits,
                None => vec![],
            };
            out.push(span(p.ident.start_byte(), p.ident.end_byte(), message(v.why, name), edits));
        }
    }
    let _ = fn_like;
});

fn enclosing_fn_non_arrow(n: Node<'_>) -> Option<Node<'_>> {
    ancestors(n).find(|a| fn_like(a.kind()) && a.kind() != "arrow_function")
}

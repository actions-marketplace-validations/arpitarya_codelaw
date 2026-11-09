//! W-139 strand W-139-js: eslint-plugin-unicorn v79.0.0's
//! `no-invalid-argument-count`, read from the rule's source and tests and
//! written again. A call is checked against a table of the language's own
//! functions and constructors (both directions), and — for too many
//! arguments only — against a function declared in the file that is called
//! by name or in place. The table here is ECMAScript's globals plus a few
//! that every host has; upstream's longer list of web and Node APIs is not
//! copied, and the `patterns` option adds any API a project wants checked.

use super::values::{has_spread, is_global};
use super::{JS, finding};
use crate::js_ast::{self, arguments, is_async, params};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use tree_sitter::Node;

#[derive(Clone, Debug)]
enum Count {
    Range(usize, usize),
    Set(Vec<usize>),
}

const INF: usize = usize::MAX;

const fn exact(n: usize) -> Count {
    Count::Range(n, n)
}

static TABLE: std::sync::LazyLock<std::collections::HashMap<&'static str, Count>> = std::sync::LazyLock::new(|| table().into_iter().collect());

fn table() -> Vec<(&'static str, Count)> {
    let mut t: Vec<(&'static str, Count)> = Vec::new();
    let mut add = |path: &'static str, c: Count| t.push((path, c));
    for e in ["Error", "EvalError", "RangeError", "ReferenceError", "SyntaxError", "TypeError", "URIError"] {
        add(e, Count::Range(0, 3));
        add(Box::leak(format!("new {e}").into_boxed_str()), Count::Range(0, 3));
    }
    add("AggregateError", Count::Range(1, 3));
    add("new AggregateError", Count::Range(1, 3));
    for (p, c) in [
        ("BigInt", exact(1)),
        ("Boolean", Count::Range(0, 1)),
        ("Date", exact(0)),
        ("Number", Count::Range(0, 1)),
        ("RegExp", Count::Range(0, 2)),
        ("String", Count::Range(0, 1)),
        ("Symbol", Count::Range(0, 1)),
        ("decodeURI", exact(1)),
        ("decodeURIComponent", exact(1)),
        ("encodeURI", exact(1)),
        ("encodeURIComponent", exact(1)),
        ("escape", exact(1)),
        ("unescape", exact(1)),
        ("eval", Count::Range(0, 1)),
        ("fetch", Count::Range(1, 2)),
        ("isFinite", exact(1)),
        ("isNaN", exact(1)),
        ("parseFloat", exact(1)),
        ("parseInt", Count::Range(1, 2)),
        ("queueMicrotask", exact(1)),
        ("structuredClone", Count::Range(1, 2)),
        ("atob", exact(1)),
        ("btoa", exact(1)),
        ("setTimeout", Count::Range(1, INF)),
        ("setInterval", Count::Range(1, INF)),
        ("clearTimeout", Count::Range(0, 1)),
        ("clearInterval", Count::Range(0, 1)),
        ("Array.from", Count::Range(1, 3)),
        ("Array.fromAsync", Count::Range(1, 3)),
        ("Array.isArray", exact(1)),
        ("ArrayBuffer.isView", exact(1)),
        ("BigInt.asIntN", exact(2)),
        ("BigInt.asUintN", exact(2)),
        ("Date.now", exact(0)),
        ("Date.parse", exact(1)),
        ("Date.UTC", Count::Range(1, 7)),
        ("JSON.parse", Count::Range(1, 2)),
        ("JSON.stringify", Count::Range(1, 3)),
        ("Map.groupBy", exact(2)),
        ("Object.groupBy", exact(2)),
        ("Number.isFinite", exact(1)),
        ("Number.isInteger", exact(1)),
        ("Number.isNaN", exact(1)),
        ("Number.isSafeInteger", exact(1)),
        ("Number.parseFloat", exact(1)),
        ("Number.parseInt", Count::Range(1, 2)),
        ("Object.assign", Count::Range(1, INF)),
        ("Object.create", Count::Range(1, 2)),
        ("Object.defineProperties", exact(2)),
        ("Object.defineProperty", exact(3)),
        ("Object.getOwnPropertyDescriptor", exact(2)),
        ("Object.hasOwn", exact(2)),
        ("Object.is", exact(2)),
        ("Object.setPrototypeOf", exact(2)),
        ("Promise.reject", Count::Range(0, 1)),
        ("Promise.resolve", Count::Range(0, 1)),
        ("Promise.try", Count::Range(1, INF)),
        ("Promise.withResolvers", exact(0)),
        ("Reflect.apply", exact(3)),
        ("Reflect.construct", Count::Range(2, 3)),
        ("Reflect.defineProperty", exact(3)),
        ("Reflect.deleteProperty", exact(2)),
        ("Reflect.get", Count::Range(2, 3)),
        ("Reflect.getOwnPropertyDescriptor", exact(2)),
        ("Reflect.has", exact(2)),
        ("Reflect.set", Count::Range(3, 4)),
        ("Reflect.setPrototypeOf", exact(2)),
        ("String.raw", Count::Range(1, INF)),
        ("Math.atan2", exact(2)),
        ("Math.imul", exact(2)),
        ("Math.pow", exact(2)),
        ("Math.random", exact(0)),
        ("Symbol.for", exact(1)),
        ("Symbol.keyFor", exact(1)),
        ("Iterator.from", exact(1)),
        ("new ArrayBuffer", Count::Range(1, 2)),
        ("new Boolean", Count::Range(0, 1)),
        ("new DataView", Count::Range(1, 3)),
        ("new Date", Count::Range(0, 7)),
        ("new FinalizationRegistry", exact(1)),
        ("new Map", Count::Range(0, 1)),
        ("new Number", Count::Range(0, 1)),
        ("new Promise", exact(1)),
        ("new Proxy", exact(2)),
        ("new RegExp", Count::Range(0, 2)),
        ("new Set", Count::Range(0, 1)),
        ("new String", Count::Range(0, 1)),
        ("new WeakMap", Count::Range(0, 1)),
        ("new WeakRef", exact(1)),
        ("new WeakSet", Count::Range(0, 1)),
    ] {
        add(p, c);
    }
    for f in ["Object.entries", "Object.freeze", "Object.fromEntries", "Object.getOwnPropertyDescriptors", "Object.getOwnPropertyNames", "Object.getOwnPropertySymbols", "Object.getPrototypeOf", "Object.isExtensible", "Object.isFrozen", "Object.isSealed", "Object.keys", "Object.preventExtensions", "Object.seal", "Object.values", "Promise.all", "Promise.allSettled", "Promise.any", "Promise.race", "Reflect.getPrototypeOf", "Reflect.isExtensible", "Reflect.ownKeys", "Reflect.preventExtensions"] {
        add(f, exact(1));
    }
    for f in [
        "abs", "acos", "acosh", "asin", "asinh", "atan", "atanh", "cbrt", "ceil", "clz32", "cos", "cosh", "exp", "expm1", "floor", "f16round", "fround", "log", "log1p", "log10", "log2", "round", "sign", "sin", "sinh", "sqrt", "tan", "tanh", "trunc",
    ] {
        add(Box::leak(format!("Math.{f}").into_boxed_str()), exact(1));
    }
    for f in ["Int8Array", "Uint8Array", "Uint8ClampedArray", "Int16Array", "Uint16Array", "Int32Array", "Uint32Array", "Float16Array", "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array"] {
        add(Box::leak(format!("new {f}").into_boxed_str()), Count::Range(0, 3));
    }
    t
}

fn parse_count(v: &Value) -> Option<Count> {
    match v {
        Value::Number(n) => n.as_u64().map(|n| exact(n as usize)),
        Value::Array(a) => Some(Count::Set(a.iter().filter_map(|x| x.as_u64().map(|n| n as usize)).collect())),
        Value::Object(o) => {
            let min = o.get("min").and_then(Value::as_u64).map_or(0, |n| n as usize);
            let max = o.get("max").and_then(Value::as_u64).map_or(INF, |n| n as usize);
            (min <= max).then_some(Count::Range(min, max))
        }
        _ => None,
    }
}

fn expected_text(c: &Count) -> String {
    let plural = |n: usize| if n == 1 { "1 argument".to_owned() } else { format!("{n} arguments") };
    match c {
        Count::Set(s) => {
            let list: Vec<String> = s.iter().map(ToString::to_string).collect();
            format!("{} arguments", list.join(", "))
        }
        Count::Range(a, b) if a == b => plural(*a),
        Count::Range(0, b) => format!("at most {}", plural(*b)),
        Count::Range(a, INF) => format!("at least {}", plural(*a)),
        Count::Range(a, b) => format!("between {a} and {}", plural(*b)),
    }
}

fn invalid(c: &Count, given: usize, spread: bool, non_spread: usize) -> bool {
    if spread {
        return match c {
            Count::Range(_, b) => non_spread > *b,
            Count::Set(s) => s.iter().max().is_some_and(|m| non_spread > *m),
        };
    }
    match c {
        Count::Range(a, b) => given < *a || given > *b,
        Count::Set(s) => !s.contains(&given),
    }
}

/// `a.b.c` as `["a", "b", "c"]`, its root identifier last-checked by the caller.
fn path<'t>(n: Node<'t>, src: &str) -> Option<(Vec<String>, Node<'t>)> {
    let mut n = js_ast::unparen(n);
    let mut parts = Vec::new();
    while n.kind() == "member_expression" {
        let pr = n.child_by_field_name("property")?;
        if pr.kind() != "property_identifier" {
            return None;
        }
        parts.insert(0, text(pr, src).to_owned());
        n = js_ast::unparen(n.child_by_field_name("object")?);
    }
    if n.kind() == "identifier" {
        parts.insert(0, text(n, src).to_owned());
        return Some((parts, n));
    }
    None
}

fn local_max(call_callee: Node<'_>, ctx: &NativeContext<'_>, root: Node<'_>) -> Option<usize> {
    let model = ctx.semantic?;
    let src = ctx.source;
    let callee = js_ast::unparen(call_callee);
    let func: Node<'_> = if callee.kind() == "identifier" {
        let sym = model.symbol_at(callee.start_byte())?;
        let s = model.symbol(sym);
        if s.decls.len() != 1 || model.is_reassigned(sym) {
            return None;
        }
        let id = root.descendant_for_byte_range(s.decls[0].0, s.decls[0].1)?;
        let par = id.parent()?;
        match par.kind() {
            "function_declaration" if par.child_by_field_name("name").is_some_and(|n| n.id() == id.id()) => {
                // A TypeScript overload list is not one signature.
                let sibling_overload = par.parent().is_some_and(|g| named_children(g).iter().any(|c| c.kind() == "function_signature" && c.child_by_field_name("name").is_some_and(|n| text(n, src) == text(id, src))));
                if sibling_overload {
                    return None;
                }
                par
            }
            "variable_declarator" if par.child_by_field_name("name").is_some_and(|n| n.id() == id.id()) && s.kind == codelaw_semantic::SymbolKind::Const => {
                let v = par.child_by_field_name("value")?;
                let v = js_ast::unparen(v);
                if !matches!(v.kind(), "arrow_function" | "function_expression" | "function") {
                    return None;
                }
                v
            }
            _ => return None,
        }
    } else if matches!(callee.kind(), "arrow_function" | "function_expression" | "function") && call_callee.kind() == "parenthesized_expression" {
        callee
    } else {
        return None;
    };
    let ps = params(func);
    let mut max = 0;
    for p in &ps {
        if p.kind() == "rest_pattern" || p.child_by_field_name("pattern").is_some_and(|x| x.kind() == "rest_pattern") {
            return None;
        }
        if p.child_by_field_name("pattern").is_some_and(|x| x.kind() == "this") {
            continue;
        }
        max += 1;
    }
    if func.kind() != "arrow_function" && uses_arguments(func, src) {
        return None;
    }
    let _ = is_async(func);
    Some(max)
}

fn uses_arguments(f: Node<'_>, src: &str) -> bool {
    let mut stack = vec![f];
    while let Some(x) = stack.pop() {
        if x.kind() == "identifier" && text(x, src) == "arguments" {
            return true;
        }
        if x.id() != f.id() && matches!(x.kind(), "function_declaration" | "function_expression" | "function" | "generator_function") {
            continue;
        }
        stack.extend(named_children(x));
    }
    false
}

pub struct NoInvalidArgumentCount;
impl NativeRule for NoInvalidArgumentCount {
    fn id(&self) -> &'static str {
        "js/no-invalid-argument-count"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression", "new_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        let is_new = n.kind() == "new_expression";
        let Some(callee) = n.child_by_field_name(if is_new { "constructor" } else { "function" }) else { return };
        let args = arguments(n);
        let spread = has_spread(n);
        let non_spread = args.iter().filter(|a| a.kind() != "spread_element").count();
        let root = js_ast::root_of(n);
        let custom: Vec<(String, Count)> = ctx.options.get("patterns").and_then(Value::as_object).map(|o| o.iter().filter_map(|(k, v)| parse_count(v).map(|c| (k.clone(), c))).collect()).unwrap_or_default();
        if let Some((mut parts, rootid)) = path(callee, src) {
            // A leading global object does not change what is called.
            while parts.len() > 1 && matches!(parts[0].as_str(), "globalThis" | "global" | "self" | "window") {
                parts.remove(0);
            }
            let key = if is_new { format!("new {}", parts.join(".")) } else { parts.join(".") };
            let hit = custom.iter().find(|(k, _)| *k == key).cloned().or_else(|| TABLE.get(key.as_str()).map(|c| (key.clone(), c.clone())));
            if let Some((_, c)) = hit {
                let global_root = is_global(model, rootid, text(rootid, src), src) || matches!(text(rootid, src), "globalThis" | "global" | "self" | "window") && is_global(model, rootid, text(rootid, src), src);
                let is_custom = custom.iter().any(|(k, _)| *k == key);
                if (global_root || is_custom) && invalid(&c, args.len(), spread, non_spread) {
                    out.push(finding(n, format!("`{}` takes {}, but this passes {}", parts.join("."), expected_text(&c), args.len()), vec![]));
                }
                return;
            }
        }
        if is_new || spread {
            return;
        }
        if let Some(max) = local_max(callee, ctx, root)
            && non_spread > max
        {
            let name = text(js_ast::unparen(callee), src);
            let shown = if name.len() > 30 { "this function" } else { name };
            out.push(finding(n, format!("`{shown}` declares {max} parameter(s), but this passes {non_spread}: the extra arguments are ignored"), vec![]));
        }
    }
}

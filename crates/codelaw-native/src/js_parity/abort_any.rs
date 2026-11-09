//! W-139 strand W-139-jsfinal: eslint-plugin-unicorn v79.0.0's
//! `prefer-abort-signal-any`, read from its source and tests and written
//! again, as it behaves without type information (no program is loaded here):
//! a source signal is one named `signal` or `…Signal`, or an
//! `AbortSignal.any()`/`.timeout()` call; an iterable is an array when the
//! syntax or an annotation says so.
//!
//! `const controller = new AbortController()` followed by statements that
//! only forward `abort` events of other signals into it, and used otherwise
//! only as `controller.signal`, is `AbortSignal.any(...)`.

use super::kit::{comment_in, optional, srule};
use super::{JS, finding};
use crate::js_ast::{self, arguments, root_of};
use crate::util::{children, named_children, remove_statement, text};
use crate::as_suggestion;
use codelaw_core::TextEdit;
use codelaw_semantic::{SemanticModel, SymbolKind};
use std::collections::BTreeSet;
use tree_sitter::Node;

/// `(x)`, `x as T`, `x satisfies T`, `x!` and `<T>x` as `x`.
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

/// A string literal (or a template without substitutions) as its value.
fn static_string(n: Node<'_>, src: &str) -> Option<String> {
    let n = unwrap_ts(n);
    match n.kind() {
        "string" => Some(crate::util::unquote(text(n, src)).to_owned()),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Some(crate::util::unquote(text(n, src)).to_owned()),
        _ => None,
    }
}

fn unresolved(m: &SemanticModel, ident: Node<'_>, name: &str, src: &str) -> bool {
    ident.kind() == "identifier" && text(ident, src) == name && m.reference_at(ident.start_byte()).is_some_and(|r| r.symbol.is_none())
}

/// `Object.method(…)` on a global, not optional.
fn static_call<'t>(n: Node<'t>, object: &str, methods: &[&str], src: &str) -> Option<Node<'t>> {
    if n.kind() != "call_expression" || optional(n) {
        return None;
    }
    let f = n.child_by_field_name("function")?;
    if f.kind() != "member_expression" || optional(f) {
        return None;
    }
    let (o, p) = (f.child_by_field_name("object")?, f.child_by_field_name("property")?);
    (p.kind() == "property_identifier" && methods.contains(&text(p, src)) && o.kind() == "identifier" && text(o, src) == object).then_some(f)
}

fn is_global_call(m: &SemanticModel, f: Node<'_>, src: &str) -> bool {
    f.child_by_field_name("object").is_some_and(|o| unresolved(m, o, "Array", src))
}

/// `Array(…)` or `new Array(…)` on the global.
fn array_constructor<'t>(n: Node<'t>, m: &SemanticModel, src: &str) -> Option<Node<'t>> {
    match n.kind() {
        "call_expression" => (!optional(n) && n.child_by_field_name("function").is_some_and(|f| unresolved(m, f, "Array", src))).then_some(n),
        "new_expression" => n.child_by_field_name("constructor").is_some_and(|f| unresolved(m, f, "Array", src)).then_some(n),
        _ => None,
    }
}

/// The initializer of a `const` bound once to a plain identifier.
fn const_init<'t>(m: &SemanticModel, n: Node<'t>) -> Option<Node<'t>> {
    if n.kind() != "identifier" {
        return None;
    }
    let sym = m.symbol_at(n.start_byte())?;
    let s = m.symbol(sym);
    if s.decls.len() != 1 || s.kind != SymbolKind::Const {
        return None;
    }
    let root = root_of(n);
    let d = js_ast::declarator_of(m, root, n)?;
    d.child_by_field_name("value")
}

/// A `const` array read after it was initialised and before `node`.
fn possibly_mutated(m: &SemanticModel, node: Node<'_>) -> bool {
    let node = unwrap_ts(node);
    let Some(init) = const_init(m, node) else { return false };
    let Some(sym) = m.symbol_at(node.start_byte()) else { return false };
    let (init_end, at) = (init.end_byte(), node.start_byte());
    m.symbol(sym).refs.iter().map(|&r| &m.references[r]).any(|r| r.start > init_end && r.start < at)
}

fn is_array(m: &SemanticModel, n: Node<'_>, src: &str, depth: usize) -> bool {
    let n = unwrap_ts(n);
    if depth > 8 {
        return false;
    }
    match n.kind() {
        "array" => true,
        "call_expression" | "new_expression" => {
            if static_call(n, "Array", &["from", "of"], src).is_some_and(|f| is_global_call(m, f, src)) || array_constructor(n, m, src).is_some() {
                return true;
            }
            static_call(n, "Object", &["keys", "values", "entries", "getOwnPropertyNames", "getOwnPropertySymbols"], src).is_some_and(|f| f.child_by_field_name("object").is_some_and(|o| unresolved(m, o, "Object", src)))
        }
        "identifier" => match const_init(m, n) {
            Some(init) => is_array(m, init, src, depth + 1),
            None => false,
        },
        _ => false,
    }
}

fn is_signal_name(name: &str) -> bool {
    name == "signal" || name.ends_with("Signal")
}

/// An expression with no call, `new`, assignment, update, `delete`, `await`
/// or `yield` in it.
fn side_effect(n: Node<'_>) -> bool {
    match n.kind() {
        "call_expression" | "new_expression" | "assignment_expression" | "augmented_assignment_expression" | "update_expression" | "await_expression" | "yield_expression" | "tagged_template_expression" => true,
        "unary_expression" if n.child(0).is_some_and(|c| c.kind() == "delete") => true,
        _ => named_children(n).iter().any(|c| side_effect(*c)),
    }
}

/// An `AbortSignal.any(…)` or `.timeout(…)` call whose arguments cannot do anything.
fn signal_factory(n: Node<'_>, src: &str, methods: &[&str]) -> bool {
    static_call(n, "AbortSignal", methods, src).is_some()
}

fn direct_bridge_source(n: Node<'_>, src: &str) -> bool {
    let n = unwrap_ts(n);
    if n.kind() == "identifier" {
        return is_signal_name(text(n, src));
    }
    signal_factory(n, src, &["any", "timeout"]) && arguments(n).iter().all(|a| a.kind() != "spread_element" && !side_effect(*a))
}

fn is_controller_signal(n: Node<'_>, cname: &str, src: &str) -> bool {
    let n = unwrap_ts(n);
    n.kind() == "member_expression"
        && !optional(n)
        && n.child_by_field_name("property").is_some_and(|p| text(p, src) == "signal")
        && n.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && text(o, src) == cname)
}

/// Whether a signal source can contain the controller's own signal or an
/// already aborted signal; one that cannot be resolved counts as one.
fn unsupported_source(m: &SemanticModel, node: Node<'_>, cname: &str, src: &str, seen: &mut BTreeSet<usize>) -> bool {
    let n = unwrap_ts(node);
    if !seen.insert(n.id()) {
        return false;
    }
    if n.kind() == "spread_element" {
        return true;
    }
    if n.kind() == "array" {
        return named_children(n).into_iter().filter(|c| c.kind() != "comment").any(|e| unsupported_source(m, e, cname, src, seen));
    }
    if static_call(n, "Array", &["of"], src).is_some_and(|f| is_global_call(m, f, src)) || array_constructor(n, m, src).is_some() {
        return arguments(n).into_iter().any(|a| unsupported_source(m, a, cname, src, seen));
    }
    if static_call(n, "Array", &["from"], src).is_some_and(|f| is_global_call(m, f, src)) {
        let args = arguments(n);
        let [source] = args[..] else { return true };
        if source.kind() == "spread_element" {
            return true;
        }
        if let Some(init) = const_init(m, unwrap_ts(source)) {
            return !is_array(m, init, src, 0) || possibly_mutated(m, source) || unsupported_source(m, init, cname, src, seen);
        }
        let unwrapped = unwrap_ts(source);
        return unwrapped.kind() != "array" || unsupported_source(m, unwrapped, cname, src, seen);
    }
    if signal_factory(n, src, &["any"]) {
        return arguments(n).first().is_some_and(|a| unsupported_source(m, *a, cname, src, seen));
    }
    if let Some(init) = const_init(m, n) {
        if is_array(m, init, src, 0) && possibly_mutated(m, n) {
            return true;
        }
        return unsupported_source(m, init, cname, src, seen);
    }
    is_controller_signal(n, cname, src) || signal_factory(unwrap_ts(n), src, &["abort"])
}

/// The elements of an array known here, when there are any to know.
fn known_elements<'t>(m: &SemanticModel, node: Node<'t>, src: &str, seen: &mut BTreeSet<usize>) -> Option<Vec<Node<'t>>> {
    let n = unwrap_ts(node);
    if !seen.insert(n.id()) {
        return None;
    }
    if n.kind() == "array" {
        return Some(named_children(n).into_iter().filter(|c| c.kind() != "comment").collect());
    }
    if static_call(n, "Array", &["of"], src).is_some_and(|f| is_global_call(m, f, src)) || array_constructor(n, m, src).is_some() {
        return Some(arguments(n));
    }
    if static_call(n, "Array", &["from"], src).is_some_and(|f| is_global_call(m, f, src)) {
        let args = arguments(n);
        if let [source] = args[..] {
            let unwrapped = unwrap_ts(source);
            if source.kind() == "spread_element" {
                return None;
            }
            if unwrapped.kind() == "array" {
                return known_elements(m, unwrapped, src, seen);
            }
            if let Some(init) = const_init(m, unwrapped).filter(|i| is_array(m, *i, src, 0) && !possibly_mutated(m, source)) {
                return known_elements(m, init, src, seen);
            }
        }
        return None;
    }
    let init = const_init(m, n)?;
    (is_array(m, init, src, 0) && !possibly_mutated(m, n)).then(|| known_elements(m, init, src, seen)).flatten()
}

fn allowed_composition(m: &SemanticModel, node: Node<'_>, src: &str) -> bool {
    match known_elements(m, node, src, &mut BTreeSet::new()) {
        None => true,
        Some(els) => els.len() > 1 && els.iter().all(|e| direct_bridge_source(*e, src)),
    }
}

fn allowed_for_of_source(m: &SemanticModel, node: Node<'_>, src: &str, seen: &mut BTreeSet<usize>) -> bool {
    let n = unwrap_ts(node);
    if !seen.insert(n.id()) {
        return false;
    }
    if n.kind() == "identifier" || n.kind() == "array" || static_call(n, "Array", &["of"], src).is_some_and(|f| is_global_call(m, f, src)) || array_constructor(n, m, src).is_some() {
        return true;
    }
    if static_call(n, "Array", &["from"], src).is_some_and(|f| is_global_call(m, f, src)) {
        let args = arguments(n);
        if let [source] = args[..] {
            let unwrapped = unwrap_ts(source);
            if source.kind() == "spread_element" {
                return false;
            }
            if unwrapped.kind() == "array" {
                return allowed_for_of_source(m, unwrapped, src, seen);
            }
            if let Some(init) = const_init(m, unwrapped).filter(|i| is_array(m, *i, src, 0) && !possibly_mutated(m, source)) {
                return allowed_for_of_source(m, init, src, seen);
            }
        }
    }
    false
}

// ---- TypeScript annotations: an array of `AbortSignal` ----------------------

fn is_signal_type(t: Node<'_>, src: &str) -> bool {
    let t = if t.kind() == "type_annotation" { t.named_child(0).unwrap_or(t) } else { t };
    match t.kind() {
        "type_identifier" => text(t, src) == "AbortSignal",
        "nested_type_identifier" => text(t, src).ends_with(".AbortSignal"),
        "generic_type" => false,
        _ => false,
    }
}

/// `Some(true)` for an array or tuple of `AbortSignal`, `Some(false)` for an array or
/// tuple of something else, `None` when the annotation does not say.
fn array_state(t: Node<'_>, src: &str) -> Option<bool> {
    let t = if t.kind() == "type_annotation" { t.named_child(0)? } else { t };
    match t.kind() {
        "parenthesized_type" => array_state(t.named_child(0)?, src),
        "readonly_type" => array_state(t.named_child(0)?, src),
        "array_type" => Some(is_signal_type(t.named_child(0)?, src)),
        "tuple_type" => {
            let items = named_children(t);
            Some(!items.is_empty() && items.iter().all(|i| is_signal_type(tuple_item(*i), src)))
        }
        "generic_type" => {
            let name = t.child_by_field_name("name")?;
            let n = text(name, src);
            let base = n.rsplit('.').next().unwrap_or(n);
            if matches!(base, "Array" | "ReadonlyArray") {
                let args = t.child_by_field_name("type_arguments")?;
                let items = named_children(args);
                if let [one] = items[..] {
                    return Some(is_signal_type(one, src));
                }
            }
            None
        }
        _ => None,
    }
}

fn tuple_item(i: Node<'_>) -> Node<'_> {
    match i.kind() {
        "required_parameter" | "optional_parameter" => i.child_by_field_name("type").map_or(i, |t| if t.kind() == "type_annotation" { t.named_child(0).unwrap_or(t) } else { t }),
        _ => i,
    }
}

fn readonly_annotation(t: Node<'_>, src: &str) -> bool {
    let t = if t.kind() == "type_annotation" { t.named_child(0).unwrap_or(t) } else { t };
    match t.kind() {
        "readonly_type" => true,
        "generic_type" => t.child_by_field_name("name").is_some_and(|n| text(n, src).rsplit('.').next() == Some("ReadonlyArray")),
        "parenthesized_type" => t.named_child(0).is_some_and(|x| readonly_annotation(x, src)),
        _ => false,
    }
}

/// The annotation of the variable an identifier names, when it is a `const` or `let` declarator's.
fn variable_annotation<'t>(m: &SemanticModel, n: Node<'t>) -> Option<Node<'t>> {
    let sym = m.symbol_at(n.start_byte())?;
    let (s, e) = *m.symbol(sym).decls.first()?;
    let id = root_of(n).descendant_for_byte_range(s, e)?;
    let d = id.parent().filter(|p| matches!(p.kind(), "variable_declarator" | "required_parameter" | "optional_parameter"))?;
    d.child_by_field_name("type")
}

fn cast_type<'t>(n: Node<'t>) -> Option<Node<'t>> {
    match n.kind() {
        "as_expression" | "satisfies_expression" => named_children(n).get(1).copied(),
        "type_assertion" => named_children(n).into_iter().find(|c| c.kind() == "type_arguments").and_then(|a| a.named_child(0)),
        _ => None,
    }
}

fn cast_inner<'t>(n: Node<'t>) -> Option<Node<'t>> {
    match n.kind() {
        "as_expression" | "satisfies_expression" => n.named_child(0),
        "type_assertion" => named_children(n).last().copied(),
        _ => None,
    }
}

fn is_for_of_array(m: &SemanticModel, node: Node<'_>, src: &str, seen: &mut BTreeSet<usize>) -> bool {
    if !seen.insert(node.id()) {
        return false;
    }
    if let (Some(t), Some(inner)) = (cast_type(node), cast_inner(node)) {
        return array_state(t, src).unwrap_or_else(|| is_for_of_array(m, inner, src, seen));
    }
    let n = unwrap_ts(node);
    if n.kind() == "identifier" {
        if let Some(t) = variable_annotation(m, n)
            && let Some(state) = array_state(t, src)
        {
            return state;
        }
        if let Some(init) = const_init(m, n)
            && is_for_of_array(m, init, src, seen)
        {
            return true;
        }
    }
    is_array(m, n, src, 0)
}

fn needs_copy(m: &SemanticModel, node: Node<'_>, src: &str, seen: &mut BTreeSet<usize>) -> bool {
    if !seen.insert(node.id()) {
        return false;
    }
    if let (Some(t), Some(inner)) = (cast_type(node), cast_inner(node)) {
        return text(t, src) == "const" || readonly_annotation(t, src) || needs_copy(m, inner, src, seen);
    }
    let n = unwrap_ts(node);
    if n.kind() != "identifier" {
        return false;
    }
    if variable_annotation(m, n).is_some_and(|t| readonly_annotation(t, src)) {
        return true;
    }
    const_init(m, n).is_some_and(|init| needs_copy(m, init, src, seen))
}

fn copy_text(m: &SemanticModel, node: Node<'_>, src: &str) -> String {
    let t = text(node, src).to_owned();
    if !needs_copy(m, node, src, &mut BTreeSet::new()) {
        return t;
    }
    let un = unwrap_ts(node);
    if un.kind() == "array" {
        return text(un, src).to_owned();
    }
    if un.id() == node.id() { format!("[...{t}]") } else { format!("[...({t})]") }
}

// ---- the listeners ------------------------------------------------------------

fn allowed_options(opt: Option<Node<'_>>, src: &str) -> bool {
    let Some(opt) = opt else { return true };
    let n = unwrap_ts(opt);
    match n.kind() {
        "true" | "false" => true,
        "object" => named_children(n).into_iter().filter(|c| c.kind() != "comment").all(|p| {
            if p.kind() != "pair" {
                return false;
            }
            let (Some(k), Some(v)) = (p.child_by_field_name("key"), p.child_by_field_name("value")) else { return false };
            let key = match k.kind() {
                "property_identifier" => text(k, src).to_owned(),
                "string" => crate::util::unquote(text(k, src)).to_owned(),
                _ => return false,
            };
            matches!(key.as_str(), "capture" | "once" | "passive") && matches!(unwrap_ts(v).kind(), "true" | "false")
        }),
        _ => false,
    }
}

/// `source.addEventListener('abort', callback[, options]);`: the call and the source.
fn abort_listener<'t>(stmt: Node<'t>, src: &str) -> Option<(Node<'t>, Node<'t>)> {
    if stmt.kind() != "expression_statement" {
        return None;
    }
    let call = stmt.named_child(0).filter(|c| c.kind() == "call_expression" && !optional(*c))?;
    let f = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && !optional(*f))?;
    if f.child_by_field_name("property").is_none_or(|p| text(p, src) != "addEventListener") {
        return None;
    }
    let args = arguments(call);
    if !(2..=3).contains(&args.len()) || static_string(args[0], src).as_deref() != Some("abort") || !allowed_options(args.get(2).copied(), src) {
        return None;
    }
    Some((call, f.child_by_field_name("object")?))
}

/// The `controller` identifier of a callback that only calls `controller.abort()`;
/// `controller.abort(source.reason)` too when a source identifier is given.
fn abort_reference<'t>(m: &SemanticModel, callback: Node<'t>, cname: &str, source: Option<Node<'t>>, src: &str) -> Option<Node<'t>> {
    if !matches!(callback.kind(), "arrow_function" | "function_expression") || js_ast::is_async(callback) || !js_ast::params(callback).is_empty() {
        return None;
    }
    let body = callback.child_by_field_name("body")?;
    let expr = if body.kind() == "statement_block" {
        let stmts: Vec<Node<'_>> = named_children(body).into_iter().filter(|c| c.kind() != "comment").collect();
        match stmts[..] {
            [s] if s.kind() == "expression_statement" => s.named_child(0)?,
            _ => return None,
        }
    } else {
        body
    };
    if expr.kind() != "call_expression" || optional(expr) {
        return None;
    }
    let f = expr.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && !optional(*f))?;
    let (o, p) = (f.child_by_field_name("object")?, f.child_by_field_name("property")?);
    if o.kind() != "identifier" || text(o, src) != cname || text(p, src) != "abort" {
        return None;
    }
    let args = arguments(expr);
    let ok = args.is_empty()
        || (args.len() == 1
            && source.is_some_and(|s| {
                let a = unwrap_ts(args[0]);
                let s = unwrap_ts(s);
                a.kind() == "member_expression"
                    && !optional(a)
                    && a.child_by_field_name("property").is_some_and(|p| text(p, src) == "reason")
                    && a.child_by_field_name("object").is_some_and(|ao| {
                        let ao = unwrap_ts(ao);
                        ao.kind() == "identifier" && s.kind() == "identifier" && same_binding(m, ao, s, src)
                    })
            }));
    ok.then_some(o)
}

fn same_binding(m: &SemanticModel, a: Node<'_>, b: Node<'_>, src: &str) -> bool {
    let (sa, sb) = (m.symbol_at(a.start_byte()), m.symbol_at(b.start_byte()));
    if sa.is_some() || sb.is_some() { sa == sb } else { text(a, src) == text(b, src) }
}

fn next_statement<'t>(stmt: Node<'t>) -> Option<Node<'t>> {
    std::iter::successors(stmt.next_named_sibling(), |s| s.next_named_sibling()).find(|s| s.kind() != "comment")
}

struct Bridge<'t> {
    abort_refs: Vec<Node<'t>>,
    replacement: String,
    statements: Vec<Node<'t>>,
}

fn direct_bridge<'t>(m: &SemanticModel, decl: Node<'t>, cname: &str, src: &str) -> Option<Bridge<'t>> {
    let mut statements = Vec::new();
    let mut sources = Vec::new();
    let mut refs = Vec::new();
    let mut next = next_statement(decl);
    while let Some(stmt) = next {
        let Some((call, source)) = abort_listener(stmt, src) else { break };
        if unsupported_source(m, source, cname, src, &mut BTreeSet::new()) || !direct_bridge_source(source, src) {
            break;
        }
        let Some(r) = arguments(call).get(1).and_then(|cb| abort_reference(m, *cb, cname, Some(source), src)) else { break };
        statements.push(stmt);
        sources.push(source);
        refs.push(r);
        next = next_statement(stmt);
    }
    if statements.len() < 2 {
        return None;
    }
    let list: Vec<&str> = sources.iter().map(|s| text(*s, src)).collect();
    Some(Bridge { abort_refs: refs, replacement: format!("AbortSignal.any([{}])", list.join(", ")), statements })
}

fn for_of_bridge<'t>(m: &SemanticModel, decl: Node<'t>, cname: &str, src: &str) -> Option<Bridge<'t>> {
    let stmt = next_statement(decl).filter(|s| s.kind() == "for_in_statement")?;
    if stmt.child_by_field_name("operator").is_none_or(|o| text(o, src) != "of") || children(stmt).iter().any(|c| c.kind() == "await") {
        return None;
    }
    let right = stmt.child_by_field_name("right")?;
    if !allowed_for_of_source(m, right, src, &mut BTreeSet::new())
        || possibly_mutated(m, right)
        || unsupported_source(m, right, cname, src, &mut BTreeSet::new())
        || !is_for_of_array(m, right, src, &mut BTreeSet::new())
        || !allowed_composition(m, right, src)
    {
        return None;
    }
    let kind = stmt.child_by_field_name("kind")?;
    let signal = stmt.child_by_field_name("left").filter(|l| l.kind() == "identifier")?;
    if text(kind, src) != "const" || !is_signal_name(text(signal, src)) {
        return None;
    }
    let body = stmt.child_by_field_name("body")?;
    let only = if body.kind() == "statement_block" {
        let s: Vec<Node<'_>> = named_children(body).into_iter().filter(|c| c.kind() != "comment").collect();
        match s[..] {
            [x] => x,
            _ => return None,
        }
    } else {
        body
    };
    let (call, source) = abort_listener(only, src)?;
    if !same_binding(m, unwrap_ts(source), signal, src) || unwrap_ts(source).kind() != "identifier" {
        return None;
    }
    let r = arguments(call).get(1).and_then(|cb| abort_reference(m, *cb, cname, Some(signal), src))?;
    Some(Bridge { abort_refs: vec![r], replacement: format!("AbortSignal.any({})", copy_text(m, right, src)), statements: vec![stmt] })
}

fn alias_or_write(member: Node<'_>) -> bool {
    let mut cur = member;
    while let Some(p) = cur.parent() {
        if matches!(p.kind(), "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion") {
            cur = p;
        } else {
            return matches!(p.kind(), "variable_declarator" | "assignment_expression");
        }
    }
    false
}

fn left_hand(member: Node<'_>) -> bool {
    let mut cur = member;
    while let Some(p) = cur.parent() {
        if matches!(p.kind(), "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion") {
            cur = p;
            continue;
        }
        return match p.kind() {
            "assignment_expression" | "augmented_assignment_expression" | "assignment_pattern" | "object_assignment_pattern" => p.child_by_field_name("left").is_some_and(|l| l.id() == cur.id()),
            "update_expression" | "array_pattern" | "rest_pattern" | "pair_pattern" => true,
            "for_in_statement" => p.child_by_field_name("left").is_some_and(|l| l.id() == cur.id()),
            "unary_expression" => p.child(0).is_some_and(|c| c.kind() == "delete"),
            _ => false,
        };
    }
    false
}

fn reason_sensitive(member: Node<'_>, src: &str) -> bool {
    let mut cur = member;
    while let Some(p) = cur.parent() {
        if matches!(p.kind(), "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion") {
            cur = p;
        } else {
            return p.kind() == "member_expression" && p.child_by_field_name("object").is_some_and(|o| o.id() == cur.id()) && p.child_by_field_name("property").is_some_and(|x| matches!(text(x, src), "reason" | "throwIfAborted"));
        }
    }
    false
}

/// The `controller.signal` reads that can be replaced, when the controller is used in no other way.
fn signal_members<'t>(m: &SemanticModel, id: Node<'t>, abort_refs: &[Node<'t>], src: &str) -> Option<Vec<Node<'t>>> {
    let sym = m.declared_at(id.start_byte())?;
    let root = root_of(id);
    for r in abort_refs {
        if m.symbol_at(r.start_byte()) != Some(sym) {
            return None;
        }
    }
    let mut members = Vec::new();
    for rf in m.symbol(sym).refs.iter().map(|&r| &m.references[r]) {
        if rf.start == id.start_byte() || abort_refs.iter().any(|a| a.start_byte() == rf.start) {
            continue;
        }
        if rf.write {
            return None;
        }
        let ident = root.descendant_for_byte_range(rf.start, rf.end)?;
        let member = ident.parent().filter(|p| p.kind() == "member_expression" && !optional(*p) && p.child_by_field_name("object").is_some_and(|o| o.id() == ident.id()) && p.child_by_field_name("property").is_some_and(|x| x.kind() == "property_identifier" && text(x, src) == "signal"))?;
        if left_hand(member) || reason_sensitive(member, src) || alias_or_write(member) {
            return None;
        }
        members.push(member);
    }
    (!members.is_empty()).then_some(members)
}

fn name_taken(m: &SemanticModel, at: usize, name: &str, own: usize) -> bool {
    let mut cur = Some(m.scope_at(at));
    while let Some(sc) = cur {
        if let Some(ids) = m.scopes[sc].bindings.get(name)
            && ids.iter().any(|&i| i != own && m.symbol(i).kind.is_value())
        {
            return true;
        }
        cur = m.scopes[sc].parent;
    }
    false
}

srule!(PreferAbortSignalAny, "js/prefer-abort-signal-any", JS, &["lexical_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(kind) = n.child_by_field_name("kind").filter(|k| text(*k, src) == "const") else { return };
    let _ = kind;
    if !n.parent().is_some_and(|p| matches!(p.kind(), "program" | "statement_block" | "switch_case" | "switch_default" | "class_static_block")) {
        return;
    }
    let decls: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
    let [d] = decls[..] else { return };
    let (Some(id), Some(init)) = (d.child_by_field_name("name").filter(|x| x.kind() == "identifier"), d.child_by_field_name("value")) else { return };
    let new = unwrap_ts(init);
    let ctor_ok = init.kind() == "new_expression" && new.id() == init.id() && init.child_by_field_name("constructor").is_some_and(|c| unresolved(m, c, "AbortController", src)) && arguments(init).is_empty();
    if !ctor_ok || name_taken_any(m, id, "AbortSignal") {
        return;
    }
    let cname = text(id, src);
    let Some(bridge) = for_of_bridge(m, n, cname, src).or_else(|| direct_bridge(m, n, cname, src)) else { return };
    let Some(members) = signal_members(m, id, &bridge.abort_refs, src) else { return };
    let Some(sym) = m.declared_at(id.start_byte()) else { return };
    let new_name = match cname {
        "abortController" => "abortSignal",
        "controller" => "signal",
        other => other,
    };
    let new_name = if new_name != cname && (name_taken(m, id.start_byte(), new_name, sym) || members.iter().any(|mb| name_taken(m, mb.start_byte(), new_name, sym))) { cname } else { new_name };
    let last = *bridge.statements.last().unwrap_or(&n);
    let root = root_of(n);
    let annotation = d.child_by_field_name("type");
    let trailing = {
        let rest = &src[last.end_byte()..];
        let line_end = rest.find('\n').unwrap_or(rest.len());
        rest[..line_end].contains("//") || rest[..line_end].contains("/*")
    };
    let mut commented = comment_in(root, n.end_byte(), last.end_byte()) || trailing || comment_in(root, init.start_byte(), init.end_byte());
    if let Some(a) = annotation {
        commented |= comment_in(root, a.start_byte(), a.end_byte());
    }
    commented |= members.iter().any(|mb| comment_in(root, mb.start_byte(), mb.end_byte()));
    let mut edits = vec![TextEdit::new(init.start_byte(), init.end_byte(), bridge.replacement.clone())];
    if let Some(a) = annotation {
        edits.push(TextEdit::new(a.start_byte(), a.end_byte(), ": AbortSignal"));
    }
    if new_name != cname {
        edits.push(TextEdit::new(id.start_byte(), id.end_byte(), new_name));
    }
    for mb in &members {
        edits.push(TextEdit::new(mb.start_byte(), mb.end_byte(), new_name));
    }
    for s in &bridge.statements {
        edits.push(remove_statement(*s, src));
    }
    out.push(finding(id, "Prefer `AbortSignal.any()` over manually forwarding abort events between signals.", if commented { vec![] } else { as_suggestion(edits) }));
});

/// Whether `name` is bound to a runtime value visible at `at`.
fn name_taken_any(m: &SemanticModel, at: Node<'_>, name: &str) -> bool {
    let mut cur = Some(m.scope_at(at.start_byte()));
    while let Some(sc) = cur {
        if m.scopes[sc].bindings.get(name).is_some_and(|ids| ids.iter().any(|&i| m.symbol(i).kind.is_value())) {
            return true;
        }
        cur = m.scopes[sc].parent;
    }
    false
}

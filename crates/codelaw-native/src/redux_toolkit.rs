//! Native rules behind `redux-toolkit/recommended`: codelaw's own checks
//! of the Redux Style Guide's Priority A and B rules that have a
//! deterministic match (L5). Most of them first find the reducers in a
//! file:
//!
//! - **Immer reducers:** the case reducers of a `createSlice` `reducers`
//!   object (`added(state, action) {}` or `{ reducer, prepare }`), and the
//!   callbacks a builder's `addCase`, `addMatcher` and `addDefaultCase`
//!   take — `state` there is an Immer draft, which may be changed in place;
//! - **hand-written reducers:** a function `(state, action)` that tests
//!   `action.type`, outside `createSlice`, `createReducer` and `produce`.

use crate::js_ast::{
    arguments, as_function, body, callee_name, chain_root, descendants_same_function, initial_value, is_function, is_named_import, member, member_name, member_value, members, param_name, params, root_of, unparen, FUNCTION_KINDS,
};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::SemanticModel;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

fn finding(n: Node<'_>, message: impl Into<String>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits: Vec::new() }
}

fn finding_fix(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

/// The leading whitespace of the line `at` is on.
fn indent(src: &str, at: usize) -> &str {
    let line = src[..at].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let lead = &src[line..];
    &lead[..lead.len() - lead.trim_start_matches([' ', '\t']).len()]
}

/// The statement holding an expression, and the statement after it in
/// the same block.
fn statement_and_next(e: Node<'_>) -> Option<(Node<'_>, Node<'_>)> {
    let stmt = e.parent().filter(|p| p.kind() == "expression_statement")?;
    let mut next = stmt.next_named_sibling()?;
    while next.kind() == "comment" {
        next = next.next_named_sibling()?;
    }
    Some((stmt, next))
}

fn calls(call: Node<'_>, src: &str, names: &[&str]) -> bool {
    call.kind() == "call_expression" && callee_name(call, src).is_some_and(|n| names.contains(&n))
}

/// Whether `f` is a case reducer whose `state` is an Immer draft.
fn is_immer_reducer(f: Node<'_>, src: &str) -> bool {
    let Some(p) = f.parent() else { return false };
    // `builder.addCase(type, (state, action) => …)`, and `create.reducer(fn)`.
    if p.kind() == "arguments" {
        let Some(call) = p.parent() else { return false };
        let last = arguments(call).last().is_some_and(|a| unparen(*a).id() == f.id() || a.id() == f.id());
        let member_call = call.child_by_field_name("function").is_some_and(|c| c.kind() == "member_expression");
        return last && member_call && calls(call, src, &["addCase", "addMatcher", "addDefaultCase", "reducer", "preparedReducer"]);
    }
    // `reducers: { added(state, action) {} }`, `added: (state) => …`, `added: { reducer, prepare }`.
    let (holder, name) = match p.kind() {
        "pair" => (p.parent(), member_name(p, src)),
        "object" if f.kind() == "method_definition" => (Some(p), member_name(f, src)),
        _ => return false,
    };
    let Some(object) = holder.filter(|o| o.kind() == "object") else { return false };
    if is_reducers_object(object, src) {
        return true;
    }
    // `{ reducer, prepare }` inside the `reducers` object.
    name == Some("reducer") && object.parent().and_then(|pair| pair.parent()).is_some_and(|o| o.kind() == "object" && is_reducers_object(o, src))
}

/// The `reducers` object of a `createSlice({ … })` call.
fn is_reducers_object(object: Node<'_>, src: &str) -> bool {
    let Some(pair) = object.parent().filter(|p| p.kind() == "pair" && member_name(*p, src) == Some("reducers")) else { return false };
    let Some(options) = pair.parent().filter(|o| o.kind() == "object") else { return false };
    options.parent().filter(|a| a.kind() == "arguments").and_then(|a| a.parent()).is_some_and(|c| calls(c, src, &["createSlice"]))
}

/// A reducer's `(state, action)` parameter names.
fn reducer_params<'s>(f: Node<'_>, src: &'s str) -> Option<(&'s str, Option<&'s str>)> {
    let ps = params(f);
    let state = param_name(*ps.first()?, src)?;
    Some((state, ps.get(1).and_then(|a| param_name(*a, src))))
}

/// `<action>.type`.
fn is_action_type(n: Node<'_>, action: &str, src: &str) -> bool {
    let n = unparen(n);
    n.kind() == "member_expression"
        && n.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && text(o, src) == action)
        && n.child_by_field_name("property").is_some_and(|p| text(p, src) == "type")
}

/// The first test of `action.type` in a hand-written reducer: the switch
/// discriminant or a comparison.
fn action_type_test<'t>(f: Node<'t>, src: &str) -> Option<Node<'t>> {
    let (_, Some(action)) = reducer_params(f, src)? else { return None };
    let b = body(f)?;
    descendants_same_function(b).into_iter().find(|n| match n.kind() {
        "switch_statement" => n.child_by_field_name("value").is_some_and(|v| is_action_type(v, action, src)),
        "binary_expression" => {
            n.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "===" | "==" | "!==" | "!="))
                && [n.child_by_field_name("left"), n.child_by_field_name("right")].into_iter().flatten().any(|s| is_action_type(s, action, src))
        }
        _ => false,
    })
}

/// Whether `f` is a hand-written reducer: `(state, action)` testing
/// `action.type`, not inside a `createSlice`, `createReducer` or `produce`.
fn is_plain_reducer(f: Node<'_>, src: &str) -> bool {
    if params(f).len() < 2 || is_immer_reducer(f, src) {
        return false;
    }
    let inside_immer = std::iter::successors(f.parent(), |p| p.parent()).any(|a| calls(a, src, &["createSlice", "createReducer", "produce"]));
    !inside_immer && action_type_test(f, src).is_some()
}

/// A node rooted at `state`, below it: `state.a`, `state.a[0].b`.
fn below(n: Node<'_>, state: &str, src: &str) -> bool {
    matches!(unparen(n).kind(), "member_expression" | "subscript_expression") && chain_root(n).is_some_and(|r| text(r, src) == state)
}

const MUTATORS: &[&str] = &["push", "pop", "shift", "unshift", "splice", "sort", "reverse", "fill", "copyWithin", "set", "add", "delete", "clear"];

/// The places in a hand-written reducer that change `state` in place.
fn mutations<'t>(f: Node<'t>, state: &str, src: &str) -> Vec<Node<'t>> {
    let Some(b) = body(f) else { return Vec::new() };
    descendants_same_function(b)
        .into_iter()
        .filter(|n| match n.kind() {
            "assignment_expression" | "augmented_assignment_expression" => n.child_by_field_name("left").is_some_and(|l| below(l, state, src)),
            "update_expression" => n.child_by_field_name("argument").is_some_and(|a| below(a, state, src)),
            "unary_expression" => n.child(0).is_some_and(|o| o.kind() == "delete") && n.child_by_field_name("argument").is_some_and(|a| below(a, state, src)),
            "call_expression" => {
                let Some(callee) = n.child_by_field_name("function").filter(|c| c.kind() == "member_expression") else { return false };
                let method = callee.child_by_field_name("property").map(|p| text(p, src)).unwrap_or("");
                let object = callee.child_by_field_name("object");
                let on_state = object.is_some_and(|o| {
                    let o = unparen(o);
                    (o.kind() == "identifier" && text(o, src) == state) || below(o, state, src)
                });
                let assign = text(callee, src) == "Object.assign" && arguments(*n).first().is_some_and(|a| {
                    let a = unparen(*a);
                    (a.kind() == "identifier" && text(a, src) == state) || below(a, state, src)
                });
                (on_state && MUTATORS.contains(&method)) || assign
            }
            _ => false,
        })
        .collect()
}

// ---------------------------------------------------------------- no-mutating-reducer

/// The edits for a change followed at once by `return state;`:
/// `state.push(x)` returns `[...state, x]`, `state.key = v` returns
/// `{ ...state, key: v }`.
fn copy_then_return(m: Node<'_>, state: &str, src: &str) -> Option<Vec<TextEdit>> {
    let (stmt, next) = statement_and_next(m)?;
    if next.kind() != "return_statement" || next.named_child(0).is_none_or(|v| text(v, src) != state) {
        return None;
    }
    let copy = match m.kind() {
        "call_expression" => {
            let callee = m.child_by_field_name("function")?;
            if text(callee.child_by_field_name("object")?, src) != state || text(callee.child_by_field_name("property")?, src) != "push" {
                return None;
            }
            let args = m.child_by_field_name("arguments")?;
            let inner = src[args.start_byte() + 1..args.end_byte() - 1].trim().trim_end_matches(',');
            if inner.is_empty() || crate::util::mentions(inner, state) {
                return None;
            }
            format!("[...{state}, {inner}]")
        }
        "assignment_expression" => {
            let left = m.child_by_field_name("left").filter(|l| l.kind() == "member_expression")?;
            if text(left.child_by_field_name("object")?, src) != state {
                return None;
            }
            let key = left.child_by_field_name("property").filter(|p| p.kind() == "property_identifier")?;
            let value = text(m.child_by_field_name("right")?, src);
            if crate::util::mentions(value, state) {
                return None;
            }
            format!("{{ ...{state}, {}: {value} }}", text(key, src))
        }
        _ => return None,
    };
    Some(vec![TextEdit::new(stmt.start_byte(), next.end_byte(), format!("return {copy};"))])
}

pub struct NoMutatingReducer;

impl NativeRule for NoMutatingReducer {
    fn id(&self) -> &'static str {
        "redux-toolkit/no-mutating-reducer"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        FUNCTION_KINDS
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let f = node.get_inner_node();
        let src = ctx.source;
        if !is_plain_reducer(f, src) {
            return;
        }
        let Some((state, _)) = reducer_params(f, src) else { return };
        for m in mutations(f, state, src) {
            let edits = copy_then_return(m, state, src).unwrap_or_default();
            out.push(finding_fix(m, format!("This changes `{state}` in place; a hand-written reducer returns a new value, or is written with `createSlice`, whose Immer draft may be changed"), edits));
        }
    }
}

// ---------------------------------------------------------------- no-side-effects-in-reducer

const EFFECT_CALLS: &[&str] = &["fetch", "setTimeout", "setInterval", "setImmediate", "requestAnimationFrame", "queueMicrotask", "nanoid", "dispatch", "alert"];
const EFFECT_MEMBERS: &[&str] = &["Date.now", "Math.random", "crypto.randomUUID", "crypto.getRandomValues", "performance.now", "window.fetch"];
const EFFECT_OBJECTS: &[&str] = &["localStorage", "sessionStorage", "axios", "navigator"];
const EFFECT_CLASSES: &[&str] = &["Promise", "XMLHttpRequest", "WebSocket", "EventSource", "Worker"];

/// What a node does that a reducer may not, if anything.
fn side_effect(n: Node<'_>, src: &str) -> Option<String> {
    match n.kind() {
        "await_expression" => Some("waits on async work".into()),
        "call_expression" => {
            let f = n.child_by_field_name("function")?;
            let callee = text(f, src);
            if f.kind() == "identifier" && (EFFECT_CALLS.contains(&callee) || callee == "axios") {
                return Some(format!("calls `{callee}()`"));
            }
            if f.kind() == "member_expression" {
                if EFFECT_MEMBERS.contains(&callee) {
                    return Some(format!("calls `{callee}()`"));
                }
                let root = chain_root(f).map(|r| text(r, src)).unwrap_or("");
                if EFFECT_OBJECTS.contains(&root) {
                    return Some(format!("calls `{callee}()`"));
                }
            }
            None
        }
        "new_expression" => {
            let c = n.child_by_field_name("constructor")?;
            let class = text(c, src);
            if EFFECT_CLASSES.contains(&class) {
                return Some(format!("creates a `{class}`"));
            }
            let no_args = n.child_by_field_name("arguments").is_none_or(|a| named_children(a).is_empty());
            (class == "Date" && no_args).then(|| "reads the clock with `new Date()`".into())
        }
        _ => None,
    }
}

/// The edit that moves a side effect into a `prepare` callback: a case
/// reducer `name(state, action) { … }` whose effect sits in an object
/// literal built from `action.payload` (and nothing of `state`) becomes
/// `name: { reducer(state, action) { … action.payload … }, prepare(payload)
/// { return { payload: <the object> }; } }`, when the rest of the reducer
/// doesn't read `action`.
fn to_prepare(f: Node<'_>, effect: Node<'_>, src: &str) -> Option<Vec<TextEdit>> {
    if f.kind() != "method_definition" {
        return None;
    }
    let (state, Some(action)) = reducer_params(f, src)? else { return None };
    let object = std::iter::successors(Some(effect), |n| n.parent()).take_while(|n| n.id() != f.id()).filter(|n| n.kind() == "object").last()?;
    let payload = format!("{action}.payload");
    let obj = text(object, src);
    if crate::util::mentions(obj, state) || !obj.contains(&payload) || obj.replace(&payload, "").contains(action) {
        return None;
    }
    let rest = format!("{}{}", &src[f.start_byte()..object.start_byte()], &src[object.end_byte()..f.end_byte()]);
    let params_end = f.child_by_field_name("parameters")?.end_byte();
    if src[params_end..object.start_byte()].contains(action) || src[object.end_byte()..f.end_byte()].contains(action) {
        return None;
    }
    let _ = rest;
    let name = text(f.child_by_field_name("name")?, src);
    let pad = indent(src, f.start_byte());
    let method = format!("reducer{}{}{}", &src[f.child_by_field_name("parameters")?.start_byte()..object.start_byte()], payload, &src[object.end_byte()..f.end_byte()]);
    let method: String = method.split('\n').enumerate().map(|(i, l)| if i == 0 || l.is_empty() { l.to_owned() } else { format!("  {l}") }).collect::<Vec<_>>().join("\n");
    let built = obj.replace(&payload, "payload");
    let out = format!("{name}: {{\n{pad}  {method},\n{pad}  prepare(payload) {{\n{pad}    return {{ payload: {built} }};\n{pad}  }},\n{pad}}}");
    Some(vec![TextEdit::new(f.start_byte(), f.end_byte(), out)])
}

pub struct NoSideEffectsInReducer;

impl NativeRule for NoSideEffectsInReducer {
    fn id(&self) -> &'static str {
        "redux-toolkit/no-side-effects-in-reducer"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        FUNCTION_KINDS
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let f = node.get_inner_node();
        let src = ctx.source;
        if !(is_immer_reducer(f, src) || is_plain_reducer(f, src)) {
            return;
        }
        // The reducer and every callback inside it.
        let Some(b) = body(f) else { return };
        let mut stack = vec![b];
        let mut first = true;
        while let Some(n) = stack.pop() {
            stack.extend(named_children(n));
            if let Some(what) = side_effect(n, src) {
                let edits = if std::mem::take(&mut first) { to_prepare(f, n, src).unwrap_or_default() } else { Vec::new() };
                out.push(finding_fix(n, format!("A reducer that {what} has a side effect; reducers compute the next state from `state` and `action` alone — do this in a thunk, a listener or a `prepare` callback"), edits));
            }
        }
    }
}

// ---------------------------------------------------------------- no-non-serializable-state

/// What makes a value not serializable, if it is a value that never is.
fn non_serializable(n: Node<'_>, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "new_expression" => {
            let c = text(n.child_by_field_name("constructor")?, src);
            (!matches!(c, "Array" | "Object")).then(|| format!("a `{c}`"))
        }
        "call_expression" => n.child_by_field_name("function").filter(|f| text(*f, src) == "Symbol").map(|_| "a `Symbol`".into()),
        "arrow_function" | "function_expression" | "function" | "generator_function" => Some("a function".into()),
        "class" => Some("a class".into()),
        "regex" => Some("a regular expression".into()),
        _ => None,
    }
}

/// Walk an initial state's literal, following `const` names once.
fn walk_state<'t>(model: Option<&SemanticModel>, root: Node<'t>, n: Node<'t>, src: &str, depth: usize, out: &mut Vec<(Node<'t>, String)>) {
    if depth > 32 {
        return;
    }
    let n = unparen(n);
    match n.kind() {
        "object" => {
            for m in members(n) {
                match m.kind() {
                    "pair" => {
                        if let Some(v) = m.child_by_field_name("value") {
                            walk_state(model, root, v, src, depth + 1, out);
                        }
                    }
                    "method_definition" => out.push((m, "a function".into())),
                    _ => {}
                }
            }
        }
        "array" => {
            for e in named_children(n) {
                walk_state(model, root, e, src, depth + 1, out);
            }
        }
        "identifier" if depth == 0 => {
            if let Some(v) = model.and_then(|m| initial_value(m, root, n)) {
                walk_state(model, root, v, src, depth + 1, out);
            }
        }
        _ => {
            if let Some(what) = non_serializable(n, src) {
                out.push((n, what));
            }
        }
    }
}

pub struct NoNonSerializableState;

impl NativeRule for NoNonSerializableState {
    fn id(&self) -> &'static str {
        "redux-toolkit/no-non-serializable-state"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression", "assignment_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        if n.kind() == "assignment_expression" {
            // `state.x = new Map()` in a reducer.
            let (Some(left), Some(right)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return };
            let Some(f) = std::iter::successors(n.parent(), |p| p.parent()).find(|p| is_function(*p)) else { return };
            if !(is_immer_reducer(f, src) || is_plain_reducer(f, src)) {
                return;
            }
            let Some((state, _)) = reducer_params(f, src) else { return };
            if below(left, state, src)
                && let Some(what) = non_serializable(right, src)
            {
                out.push(finding(right, format!("{} in the state isn't serializable; keep state to plain objects, arrays and primitives", capitalise(&what))));
            }
            return;
        }
        let initial = if calls(n, src, &["createSlice"]) {
            arguments(n).first().filter(|o| o.kind() == "object").and_then(|o| member(*o, "initialState", src)).and_then(member_value)
        } else if calls(n, src, &["createReducer"]) {
            arguments(n).first().copied()
        } else {
            return;
        };
        let Some(initial) = initial else { return };
        let root = root_of(n);
        let mut initial = unparen(initial);
        if initial.kind() == "shorthand_property_identifier" || initial.kind() == "identifier" {
            match ctx.semantic.and_then(|m| initial_value(m, root, initial)) {
                Some(v) => initial = unparen(v),
                None => return,
            }
        }
        // A lazy initializer, `() => ({ … })`, is allowed: its result is the state.
        if let Some(f) = as_function(initial).filter(|f| f.kind() == "arrow_function") {
            match body(f) {
                Some(b) if b.kind() != "statement_block" => initial = b,
                _ => return,
            }
        }
        let mut found = Vec::new();
        walk_state(ctx.semantic, root, initial, src, 1, &mut found);
        for (at, what) in found {
            out.push(finding(at, format!("{} in the initial state isn't serializable; keep state to plain objects, arrays and primitives", capitalise(&what))));
        }
    }
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

// ---------------------------------------------------------------- prefer-create-slice

/// Whether a function is handed to React's `useReducer`: its state is
/// component state, not Redux's.
fn used_by_use_reducer(model: Option<&SemanticModel>, f: Node<'_>, src: &str) -> bool {
    let in_use_reducer = |n: Node| n.parent().filter(|a| a.kind() == "arguments").and_then(|a| a.parent()).is_some_and(|c| calls(c, src, &["useReducer"]));
    if in_use_reducer(f) {
        return true;
    }
    let Some(model) = model else { return false };
    let name = match f.child_by_field_name("name") {
        Some(n) => Some(n),
        None => f.parent().filter(|p| p.kind() == "variable_declarator").and_then(|p| p.child_by_field_name("name")),
    };
    let Some(sym) = name.and_then(|n| model.symbol_at(n.start_byte())) else { return false };
    let root = root_of(f);
    model.symbol(sym).refs.iter().any(|&r| {
        let r = &model.references[r];
        root.descendant_for_byte_range(r.start, r.end).is_some_and(in_use_reducer)
    })
}

pub struct PreferCreateSlice;

impl NativeRule for PreferCreateSlice {
    fn id(&self) -> &'static str {
        "redux-toolkit/prefer-create-slice"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        FUNCTION_KINDS
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let f = node.get_inner_node();
        let src = ctx.source;
        if !is_plain_reducer(f, src) || used_by_use_reducer(ctx.semantic, f, src) {
            return;
        }
        if let Some(test) = action_type_test(f, src) {
            let at = if test.kind() == "switch_statement" { test.child_by_field_name("value").unwrap_or(test) } else { test };
            out.push(finding(at, "A hand-written reducer that switches on `action.type`; write it with `createSlice`, which makes the action creators and lets the reducer change an Immer draft"));
        }
    }
}

// ---------------------------------------------------------------- no-blind-payload-return

/// `action.payload`.
fn is_payload(n: Node<'_>, action: &str, src: &str) -> bool {
    let n = unparen(n);
    n.kind() == "member_expression"
        && n.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && text(o, src) == action)
        && n.child_by_field_name("property").is_some_and(|p| text(p, src) == "payload")
}

/// A returned value that takes its shape from the action: `action.payload`,
/// or an object spreading it.
fn blind(v: Node<'_>, action: &str, src: &str) -> bool {
    let v = unparen(v);
    is_payload(v, action, src) || (v.kind() == "object" && members(v).iter().any(|m| m.kind() == "spread_element" && m.named_child(0).is_some_and(|a| is_payload(a, action, src))))
}

/// The edits that replace `return action.payload;` in a `createSlice`
/// case reducer with one `state.key = action.payload.key;` per key of the
/// slice's literal `initialState`.
fn assign_fields(f: Node<'_>, ret: Node<'_>, v: Node<'_>, state: &str, action: &str, src: &str) -> Option<Vec<TextEdit>> {
    if text(unparen(v), src) != format!("{action}.payload") {
        return None;
    }
    let slice = std::iter::successors(f.parent(), |p| p.parent()).find(|a| calls(*a, src, &["createSlice"]))?;
    let options = arguments(slice).first().copied().map(unparen).filter(|o| o.kind() == "object")?;
    let init = members(options).into_iter().find(|m| member_name(*m, src) == Some("initialState")).and_then(|m| m.child_by_field_name("value")).map(unparen).filter(|o| o.kind() == "object")?;
    let mut keys = Vec::new();
    for m in members(init) {
        match m.kind() {
            "pair" if m.child_by_field_name("key").is_some_and(|k| k.kind() == "property_identifier") => keys.push(member_name(m, src)?),
            "shorthand_property_identifier" => keys.push(text(m, src)),
            _ => return None,
        }
    }
    if keys.is_empty() {
        return None;
    }
    let pad = indent(src, ret.start_byte());
    let lines: Vec<String> = keys.iter().map(|k| format!("{state}.{k} = {action}.payload.{k};")).collect();
    Some(vec![TextEdit::new(ret.start_byte(), ret.end_byte(), lines.join(&format!("\n{pad}")))])
}

pub struct NoBlindPayloadReturn;

impl NativeRule for NoBlindPayloadReturn {
    fn id(&self) -> &'static str {
        "redux-toolkit/no-blind-payload-return"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        FUNCTION_KINDS
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let f = node.get_inner_node();
        let src = ctx.source;
        let immer = is_immer_reducer(f, src);
        if !(immer || is_plain_reducer(f, src)) {
            return;
        }
        let Some((state, Some(action))) = reducer_params(f, src) else { return };
        let Some(b) = body(f) else { return };
        let message = "The state takes whatever shape the action's payload has; the reducer should build the state it owns from the payload's fields";
        if b.kind() != "statement_block" {
            if blind(b, action, src) {
                out.push(finding(unparen(b), message));
            }
            return;
        }
        for n in descendants_same_function(b) {
            match n.kind() {
                "return_statement" => {
                    if let Some(v) = n.named_child(0).filter(|v| blind(*v, action, src)) {
                        let edits = if immer { assign_fields(f, n, v, state, action, src).unwrap_or_default() } else { Vec::new() };
                        out.push(finding_fix(unparen(v), message, edits));
                    }
                }
                // `Object.assign(state, action.payload)` on an Immer draft.
                "call_expression" if immer => {
                    let args = arguments(n);
                    let callee_ok = n.child_by_field_name("function").is_some_and(|c| text(c, src) == "Object.assign");
                    if callee_ok && args.len() == 2 && unparen(args[0]).kind() == "identifier" && text(unparen(args[0]), src) == state && is_payload(args[1], action, src) {
                        out.push(finding(n, message));
                    }
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------- slice-key-names

/// `usersReducer` as `users`.
fn without_reducer_suffix(name: &str) -> Option<&str> {
    let stem = name.strip_suffix("Reducer").or_else(|| name.strip_suffix("_reducer"))?;
    (!stem.is_empty()).then_some(stem)
}

pub struct SliceKeyNames;

impl NativeRule for SliceKeyNames {
    fn id(&self) -> &'static str {
        "redux-toolkit/slice-key-names"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let call = node.get_inner_node();
        let src = ctx.source;
        let map = if calls(call, src, &["combineReducers", "combineSlices"]) && call.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier") {
            arguments(call).first().copied()
        } else if calls(call, src, &["configureStore"]) {
            arguments(call).first().filter(|o| o.kind() == "object").and_then(|o| member(*o, "reducer", src)).and_then(|m| m.child_by_field_name("value"))
        } else {
            None
        };
        let Some(map) = map.map(unparen).filter(|m| m.kind() == "object") else { return };
        for m in members(map) {
            let Some(name) = member_name(m, src) else { continue };
            if let Some(stem) = without_reducer_suffix(name) {
                let at = m.child_by_field_name("key").unwrap_or(m);
                out.push(finding(at, format!("Name the state slice after the data it holds — `{stem}`, not `{name}`: the key is the name every selector reads")));
            }
        }
    }
}

// ---------------------------------------------------------------- no-sequential-dispatch

fn is_dispatch_statement(s: Node<'_>, src: &str) -> bool {
    s.kind() == "expression_statement"
        && s.named_child(0).map(unparen).is_some_and(|c| {
            c.kind() == "call_expression"
                && c.child_by_field_name("function").is_some_and(|f| match f.kind() {
                    "identifier" => text(f, src) == "dispatch",
                    "member_expression" => f.child_by_field_name("property").is_some_and(|p| text(p, src) == "dispatch"),
                    _ => false,
                })
        })
}

pub struct NoSequentialDispatch;

impl NativeRule for NoSequentialDispatch {
    fn id(&self) -> &'static str {
        "redux-toolkit/no-sequential-dispatch"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["statement_block", "program", "switch_case"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let block = node.get_inner_node();
        let src = ctx.source;
        let max = ctx.options.get("max").and_then(|v| v.as_u64()).unwrap_or(2) as usize;
        let mut run: Vec<Node> = Vec::new();
        let flush = |run: &mut Vec<Node>, out: &mut Vec<NativeFinding>| {
            if run.len() > max {
                let (first, last) = (run[0], run[run.len() - 1]);
                out.push(NativeFinding {
                    start: first.start_byte(),
                    end: last.end_byte(),
                    message: format!("{} actions dispatched in a row: each one re-renders and leaves the state halfway; dispatch one action that describes the event", run.len()),
                    edits: Vec::new(),
                });
            }
            run.clear();
        };
        for s in named_children(block) {
            if s.kind() == "comment" {
                continue;
            }
            if is_dispatch_statement(s, src) {
                run.push(s);
            } else {
                flush(&mut run, out);
            }
        }
        flush(&mut run, out);
    }
}

// ---------------------------------------------------------------- object-shorthand-mapdispatch

/// The object form of a `mapDispatch` whose every member is
/// `(a, b) => dispatch(creator(a, b))`: `{ key: creator }` (`{ creator }`
/// when the names agree). A `mapDispatch` named by a `const` read only
/// here goes with it.
fn shorthand(model: &SemanticModel, root: Node<'_>, arg: Node<'_>, func: Node<'_>, src: &str) -> Option<Vec<TextEdit>> {
    let dispatch = params(func).first().and_then(|p| param_name(*p, src))?;
    let obj = body(func).map(unparen).filter(|b| b.kind() == "object")?;
    let mut entries = Vec::new();
    for m in members(obj) {
        let key = (m.kind() == "pair").then(|| member_name(m, src)).flatten()?;
        let f = m.child_by_field_name("value").map(unparen).filter(|v| v.kind() == "arrow_function")?;
        let names: Vec<&str> = params(f).into_iter().map(|p| param_name(p, src)).collect::<Option<_>>()?;
        let call = body(f).map(unparen).filter(|b| b.kind() == "call_expression")?;
        if text(call.child_by_field_name("function")?, src) != dispatch {
            return None;
        }
        let inner = arguments(call);
        let [creator_call] = inner.as_slice() else { return None };
        let creator_call = unparen(*creator_call);
        let creator = creator_call.child_by_field_name("function").filter(|c| c.kind() == "identifier")?;
        let passed: Vec<&str> = arguments(creator_call).into_iter().map(|a| text(a, src)).collect();
        if passed != names {
            return None;
        }
        let c = text(creator, src);
        entries.push(if c == key { c.to_owned() } else { format!("{key}: {c}") });
    }
    if entries.is_empty() {
        return None;
    }
    let mut edits = vec![TextEdit::new(arg.start_byte(), arg.end_byte(), format!("{{ {} }}", entries.join(", ")))];
    if unparen(arg).kind() == "identifier" {
        let sym = model.symbol_at(arg.start_byte())?;
        if model.symbol(sym).refs.len() == 1 {
            let (s, e) = *model.symbol(sym).decls.first()?;
            let decl = root.descendant_for_byte_range(s, e)?.parent().filter(|d| d.kind() == "variable_declarator")?;
            let stmt = decl.parent().filter(|st| st.kind() == "lexical_declaration" && named_children(*st).len() == 1)?;
            let (a, b) = crate::util::whole_lines(src, stmt.start_byte(), stmt.end_byte());
            // The blank line after it goes too.
            let b = if src[b..].starts_with('\n') { b + 1 } else { b };
            edits.push(TextEdit::delete(a, b));
        }
    }
    Some(edits)
}

pub struct ObjectShorthandMapDispatch;

impl NativeRule for ObjectShorthandMapDispatch {
    fn id(&self) -> &'static str {
        "redux-toolkit/object-shorthand-mapdispatch"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let call = node.get_inner_node();
        let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { return };
        if !is_named_import(model, f, "connect", &|s| s == "react-redux") {
            return;
        }
        let Some(arg) = arguments(call).get(1).copied() else { return };
        let root = root_of(call);
        let func = match unparen(arg) {
            a if is_function(a) => Some(a),
            a if a.kind() == "identifier" => {
                let sym = model.symbol_at(a.start_byte());
                let decl = sym.and_then(|s| model.symbol(s).decls.first().copied()).and_then(|(s, e)| root.descendant_for_byte_range(s, e));
                match decl.and_then(|d| d.parent()) {
                    Some(p) if p.kind() == "function_declaration" => Some(p),
                    _ => initial_value(model, root, a).and_then(as_function),
                }
            }
            _ => None,
        };
        // A `mapDispatch` that reads `ownProps` needs the function form.
        if let Some(func) = func.filter(|f| params(*f).len() < 2) {
            let edits = shorthand(model, root, arg, func, ctx.source).unwrap_or_default();
            out.push(finding_fix(arg, "`mapDispatch` as a function; pass an object of action creators, `connect(mapState, { added })`, and `connect` binds them to `dispatch`", edits));
        }
    }
}

// ---------------------------------------------------------------- no-object-selector

fn is_selector_hook(name: &str) -> bool {
    name == "useSelector" || (name.starts_with("use") && name.ends_with("Selector") && name[3..].starts_with(|c: char| c.is_ascii_uppercase()))
}

/// `const { a, b } = useSelector((s) => ({ a: s.a, b: s.b }));` as one
/// `const a = useSelector((s) => s.a);` per key, when the pattern takes
/// exactly the object's keys, by name.
fn one_per_key(call: Node<'_>, selector: Node<'_>, obj: Node<'_>, src: &str) -> Option<Vec<TextEdit>> {
    let decl = call.parent().filter(|d| d.kind() == "variable_declarator")?;
    let stmt = decl.parent().filter(|s| s.kind() == "lexical_declaration" && named_children(*s).len() == 1)?;
    let pattern = decl.child_by_field_name("name").filter(|p| p.kind() == "object_pattern")?;
    let names: Vec<&str> = named_children(pattern).into_iter().map(|p| (p.kind() == "shorthand_property_identifier_pattern").then(|| text(p, src))).collect::<Option<_>>()?;
    let mut values = Vec::new();
    for m in members(obj) {
        let (key, value) = match m.kind() {
            "pair" => (member_name(m, src)?, text(m.child_by_field_name("value")?, src)),
            "shorthand_property_identifier" => return None,
            _ => return None,
        };
        values.push((key, value));
    }
    if values.len() != names.len() || values.iter().any(|(k, _)| !names.contains(k)) || selector.kind() != "arrow_function" {
        return None;
    }
    let hook = text(call.child_by_field_name("function")?, src);
    let head = &src[selector.start_byte()..selector.child_by_field_name("body")?.start_byte()];
    let keyword = stmt.child(0).map(|k| text(k, src)).unwrap_or("const");
    let pad = indent(src, stmt.start_byte());
    let lines: Vec<String> = values.iter().map(|(k, v)| format!("{keyword} {k} = {hook}({head}{v});")).collect();
    Some(vec![TextEdit::new(stmt.start_byte(), stmt.end_byte(), lines.join(&format!("\n{pad}")))])
}

/// `shallowEqual` from `react-redux` as the hook's second argument, when
/// the file imports from `react-redux` by name.
fn shallow_equal(call: Node<'_>, ctx: &NativeContext<'_>, src: &str) -> Option<Vec<TextEdit>> {
    let model = ctx.semantic?;
    let args = call.child_by_field_name("arguments")?;
    let close = args.end_byte() - 1;
    let mut edits = vec![TextEdit::insert(close, ", shallowEqual")];
    let has = model.imports.iter().any(|i| i.source == "react-redux" && matches!(&i.imported, codelaw_semantic::Imported::Named(n) if n == "shallowEqual"));
    if !has {
        let spec = model.imports.iter().rfind(|i| i.source == "react-redux" && matches!(i.imported, codelaw_semantic::Imported::Named(_)))?;
        edits.push(TextEdit::insert(spec.end, ", shallowEqual"));
    }
    let _ = src;
    Some(edits)
}

pub struct NoObjectSelector;

impl NativeRule for NoObjectSelector {
    fn id(&self) -> &'static str {
        "redux-toolkit/no-object-selector"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let call = node.get_inner_node();
        let src = ctx.source;
        let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { return };
        if !is_selector_hook(text(f, src)) {
            return;
        }
        // An equality function (`shallowEqual`) makes a new object harmless.
        let args = arguments(call);
        if args.len() != 1 {
            return;
        }
        let Some(selector) = as_function(args[0]) else { return };
        let Some(b) = body(selector) else { return };
        let returned = if b.kind() == "statement_block" {
            let returns: Vec<Node> = named_children(b).into_iter().filter(|s| s.kind() == "return_statement").collect();
            match returns.as_slice() {
                [only] => only.named_child(0),
                _ => None,
            }
        } else {
            Some(b)
        };
        if let Some(obj) = returned.map(unparen).filter(|o| o.kind() == "object") {
            let mut choices = Vec::new();
            if let Some(e) = one_per_key(call, selector, obj, src) {
                choices.push(crate::choice("one per value", true, e));
            }
            if let Some(e) = shallow_equal(call, ctx, src) {
                choices.push(crate::choice("shallowEqual", false, e));
            }
            out.push(finding_fix(obj, "The selector returns a new object every time, so the component re-renders on every dispatched action; call `useSelector` once per value", crate::choice_edits(choices)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(rule: &dyn NativeRule, src: &str) -> Vec<String> {
        crate::run(rule, Language::Tsx, src).into_iter().map(|f| src[f.start..f.end].to_owned()).collect()
    }

    #[test]
    fn mutating_reducer() {
        let src = "function todos(state = [], action) {\n  switch (action.type) {\n    case 'added': state.push(action.payload); return state;\n    case 'renamed': state[0].name = action.name; return state;\n    case 'kept': return [...state, action.payload];\n  }\n}\nconst slice = createSlice({ name: 'a', initialState: [], reducers: { added(state, action) { state.push(action.payload); } } });";
        assert_eq!(found(&NoMutatingReducer, src), ["state.push(action.payload)", "state[0].name = action.name"]);
    }

    #[test]
    fn side_effects() {
        let src = "createSlice({ name: 'a', initialState: {}, reducers: {\n  added: { reducer(state, action) { state.at = Date.now(); }, prepare(text) { return { payload: { id: nanoid(), at: Date.now() } }; } },\n  saved(state) { fetch('/save'); },\n} });\ncreateReducer(0, (builder) => builder.addCase(x, (state) => { state.n = Math.random(); }));";
        assert_eq!(found(&NoSideEffectsInReducer, src), ["Date.now()", "fetch('/save')", "Math.random()"]);
    }

    #[test]
    fn non_serializable() {
        let src = "const initialState = { byId: new Map(), at: new Date(), list: [], ok: 1 };\nconst a = createSlice({ name: 'a', initialState, reducers: { s(state) { state.seen = new Set(); } } });\nconst b = createSlice({ name: 'b', initialState: () => ({ cb: () => 1 }), reducers: {} });";
        assert_eq!(found(&NoNonSerializableState, src), ["new Map()", "new Date()", "new Set()", "() => 1"]);
    }

    #[test]
    fn create_slice_and_blind_returns() {
        let src = "export default function counter(state = 0, action) {\n  switch (action.type) {\n    case 'set': return action.payload;\n    case 'merge': return { ...state, ...action.payload };\n    default: return state;\n  }\n}\nfunction local(state, action) { if (action.type === 'x') return state; }\nfunction C() { useReducer(local, 0); }";
        assert_eq!(found(&PreferCreateSlice, src), ["(action.type)"]);
        assert_eq!(found(&NoBlindPayloadReturn, src), ["action.payload", "{ ...state, ...action.payload }"]);
    }

    #[test]
    fn store_shape_and_dispatch() {
        let src = "const root = combineReducers({ usersReducer, posts: postsReducer, todos_reducer: t });\nconfigureStore({ reducer: { cartReducer: cart } });\nfunction save() {\n  dispatch(a());\n  dispatch(b());\n  // and\n  dispatch(c());\n}\nfunction two() { dispatch(a()); dispatch(b()); }";
        assert_eq!(found(&SliceKeyNames, src), ["usersReducer", "todos_reducer", "cartReducer"]);
        assert_eq!(found(&NoSequentialDispatch, src).len(), 1);
    }

    #[test]
    fn connect_and_selectors() {
        let src = "import { connect } from 'react-redux';\nconst mapDispatch = (dispatch) => ({ add: (t) => dispatch(add(t)) });\nexport default connect(mapState, mapDispatch)(List);\nconnect(mapState, (dispatch, ownProps) => ({}))(Item);\nconnect(mapState, { add })(Other);\nfunction C() {\n  const { a, b } = useSelector((state) => ({ a: state.a, b: state.b }));\n  const c = useAppSelector((state) => state.c);\n  const d = useSelector((s) => ({ d: s.d }), shallowEqual);\n}";
        assert_eq!(found(&ObjectShorthandMapDispatch, src), ["mapDispatch"]);
        assert_eq!(found(&NoObjectSelector, src), ["{ a: state.a, b: state.b }"]);
    }
}

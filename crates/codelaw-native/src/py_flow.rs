//! W-174, the flow group of SR-PY-INFERENCE: Pylint 4.1.3 checks that ask
//! whether a name is bound on the path that reaches a read, or what a loop
//! or a generator leaves behind — written fresh from the behaviour its
//! functional tests pin (never its code). The questions of binding come from
//! `codelaw_semantic::python_flow`, run per scope and only when one of these
//! rules is on; everything else reads the tree and the per-file model.

use crate::python_util::{call_args, decorators, hit, kids, py_rule, unparen};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_semantic::{FlowConfig, FlowKind, ScopeKind, SemanticModel, python_flow};
use std::cell::RefCell;
use std::collections::BTreeSet;
use tree_sitter::Node;

// ---------------------------------------------------------------- helpers

fn root_of(n: Node<'_>) -> Node<'_> {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        cur = p;
    }
    cur
}

thread_local! {
    static NORETURN: RefCell<Option<(usize, usize, BTreeSet<String>)>> = const { RefCell::new(None) };
}

/// The names of functions this file annotates `-> NoReturn` or `-> Never`.
fn noreturn_names(root: Node<'_>, src: &str) -> BTreeSet<String> {
    let key = (src.as_ptr() as usize, src.len());
    if let Some(hit) = NORETURN.with(|c| c.borrow().as_ref().filter(|(p, l, _)| (*p, *l) == key).map(|(_, _, s)| s.clone())) {
        return hit;
    }
    let mut out = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        for c in named_children(n) {
            if c.kind() == "function_definition"
                && let (Some(name), Some(ret)) = (c.child_by_field_name("name"), c.child_by_field_name("return_type"))
            {
                let r = text(ret, src);
                if matches!(r.rsplit('.').next(), Some("NoReturn" | "Never")) {
                    out.insert(text(name, src).to_owned());
                }
            }
            stack.push(c);
        }
    }
    NORETURN.with(|c| *c.borrow_mut() = Some((key.0, key.1, out.clone())));
    out
}

fn int_value(n: Node<'_>, src: &str) -> Option<i64> {
    match n.kind() {
        "integer" => text(n, src).replace('_', "").parse().ok(),
        "unary_operator" => {
            let v = int_value(n.child_by_field_name("argument")?, src)?;
            match n.child_by_field_name("operator").map(|o| o.kind()) {
                Some("-") => Some(-v),
                Some("+") => Some(v),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether an iterable certainly yields at least once: a non-empty literal,
/// `range` of integer literals, and the builtins that wrap such an iterable.
fn nonempty(n: Node<'_>, src: &str, shadowed: &dyn Fn(&str) -> bool) -> bool {
    let n = unparen(n);
    match n.kind() {
        "list" | "tuple" | "set" => kids(n).iter().any(|c| !matches!(c.kind(), "list_splat" | "dictionary_splat")),
        "dictionary" => kids(n).iter().any(|c| c.kind() == "pair"),
        "string" | "concatenated_string" => {
            let t = text(n, src);
            let body = t.trim_start_matches(|c: char| c.is_ascii_alphabetic());
            let q = if body.starts_with("\"\"\"") || body.starts_with("'''") { 3 } else { 1 };
            body.len() > 2 * q
        }
        "conditional_expression" => {
            let parts = kids(n);
            matches!(parts.as_slice(), [a, _, b] if nonempty(*a, src, shadowed) && nonempty(*b, src, shadowed))
        }
        "call" => {
            let Some(f) = n.child_by_field_name("function") else { return false };
            if f.kind() != "identifier" || shadowed(text(f, src)) {
                return false;
            }
            let a = call_args(n, src);
            if a.starred || !a.keywords.is_empty() {
                return false;
            }
            match text(f, src) {
                "range" => {
                    let v: Option<Vec<i64>> = a.positional.iter().map(|x| int_value(*x, src)).collect();
                    match v.as_deref() {
                        Some([stop]) => *stop > 0,
                        Some([start, stop]) => start < stop,
                        Some([start, stop, step]) => (*step > 0 && start < stop) || (*step < 0 && start > stop),
                        _ => false,
                    }
                }
                "enumerate" | "reversed" | "sorted" | "list" | "tuple" | "set" | "frozenset" | "iter" => {
                    a.positional.first().is_some_and(|x| nonempty(*x, src, shadowed))
                }
                "zip" => !a.positional.is_empty() && a.positional.iter().all(|x| nonempty(*x, src, shadowed)),
                "map" => a.positional.len() > 1 && a.positional.iter().skip(1).all(|x| nonempty(*x, src, shadowed)),
                _ => false,
            }
        }
        _ => false,
    }
}

fn flow_uses(scope: Node<'_>, ctx: &NativeContext<'_>) -> Vec<codelaw_semantic::FlowUse> {
    let src = ctx.source;
    let model = ctx.semantic;
    let root = root_of(scope);
    let names = noreturn_names(root, src);
    let noreturn = move |n: &str| names.contains(n);
    let at = scope.start_byte();
    let shadowed = |n: &str| {
        model.is_some_and(|m| {
            let s = m.scope_at(at);
            m.lookup(s, n, usize::MAX).is_some()
        })
    };
    let ne = |n: Node<'_>| nonempty(n, src, &shadowed);
    let builtin = |n: &str| codelaw_semantic::PYTHON_BUILTINS.contains(&n);
    let cfg = FlowConfig { noreturn: &noreturn, builtin: &builtin, nonempty: &ne };
    python_flow(scope, src, &cfg)
}

fn flow_rule(scope: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>, kind: FlowKind) {
    for u in flow_uses(scope, ctx).into_iter().filter(|u| u.kind == kind) {
        let message = match kind {
            FlowKind::Unassigned => format!("Using variable `{}` before assignment", u.name),
            FlowKind::Maybe => format!("Possibly using variable `{}` before assignment", u.name),
            FlowKind::LoopVariable => format!("Using possibly undefined loop variable `{}`", u.name),
        };
        out.push(NativeFinding { start: u.start, end: u.end, message, edits: Vec::new() });
    }
}

fn used_before_assignment(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    flow_rule(n, ctx, out, FlowKind::Unassigned)
}
fn possibly_used_before_assignment(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    flow_rule(n, ctx, out, FlowKind::Maybe)
}
fn undefined_loop_variable(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    flow_rule(n, ctx, out, FlowKind::LoopVariable)
}

py_rule!(UsedBeforeAssignment, "py/used-before-assignment", ["module", "function_definition"], true, used_before_assignment);
py_rule!(PossiblyUsedBeforeAssignment, "py/possibly-used-before-assignment", ["module", "function_definition"], true, possibly_used_before_assignment);
py_rule!(UndefinedLoopVariable, "py/undefined-loop-variable", ["module", "function_definition"], true, undefined_loop_variable);

// ---------------------------------------------------------------- possibly-unused-variable

/// Pylint's default `dummy-variables-rgx`:
/// `_+$|(_[a-zA-Z0-9_]*[a-zA-Z0-9]+?$)|dummy|^ignored_|^unused_`.
fn is_dummy(name: &str) -> bool {
    name.chars().all(|c| c == '_')
        || (name.starts_with('_') && name.chars().last().is_some_and(|c| c.is_ascii_alphanumeric()))
        || name.contains("dummy")
        || name.starts_with("ignored_")
        || name.starts_with("unused_")
}

fn own_nodes<'t>(body: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in named_children(body) {
        out.push(c);
        if !matches!(c.kind(), "function_definition" | "class_definition" | "lambda") {
            own_nodes(c, out);
        }
    }
}

fn possibly_unused_variable(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let Some(body) = n.child_by_field_name("body") else { return };
    let Some(scope) = model.scope_starting_at(ScopeKind::Function, n.start_byte()) else { return };
    let mut nodes = Vec::new();
    own_nodes(body, &mut nodes);
    // The last `locals()` call in the function's own body.
    let locals_call = nodes
        .iter()
        .filter(|c| c.kind() == "call")
        .filter(|c| c.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "locals"))
        .filter(|c| call_args(**c, src).positional.is_empty())
        .map(|c| c.start_byte())
        .max();
    let Some(call_at) = locals_call else { return };
    if model.lookup(scope, "locals", usize::MAX).is_some() {
        return;
    }
    for c in nodes.iter().filter(|c| c.kind() == "assignment") {
        let (Some(left), Some(_)) = (c.child_by_field_name("left"), c.child_by_field_name("right")) else { continue };
        if left.kind() != "identifier" || c.start_byte() > call_at || c.parent().is_none_or(|p| p.kind() != "expression_statement") {
            continue;
        }
        let name = text(left, src);
        if is_dummy(name) {
            continue;
        }
        let Some(id) = model.declared_at(left.start_byte()) else { continue };
        let sym = model.symbol(id);
        if sym.scope != scope || sym.refs.iter().any(|&r| model.references[r].read) {
            continue;
        }
        out.push(hit(left, format!("Possibly unused variable `{name}`: the function calls `locals()`")));
    }
}

py_rule!(PossiblyUnusedVariable, "py/possibly-unused-variable", ["function_definition"], true, possibly_unused_variable);

// ---------------------------------------------------------------- looping-through-iterator

const ITER_BUILTINS: &[&str] = &["map", "filter", "zip", "iter", "reversed", "enumerate"];
const ITER_ITERTOOLS: &[&str] = &[
    "chain", "count", "cycle", "repeat", "islice", "zip_longest", "starmap", "takewhile", "dropwhile", "accumulate", "groupby", "tee",
    "permutations", "combinations", "product", "compress", "filterfalse", "pairwise", "batched",
];

fn has_yield(f: Node<'_>) -> bool {
    let mut stack = vec![f];
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if c.kind() == "yield" {
                return true;
            }
            if !matches!(c.kind(), "function_definition" | "class_definition" | "lambda") {
                stack.push(c);
            }
        }
    }
    false
}

/// Whether the value of an assignment is an iterator: a generator
/// expression, a builtin or `itertools` producer, or a call to a generator
/// function this file defines.
fn is_iterator_value(v: Node<'_>, root: Node<'_>, src: &str, model: Option<&SemanticModel>) -> bool {
    let v = unparen(v);
    match v.kind() {
        "generator_expression" => true,
        "call" => {
            let Some(f) = v.child_by_field_name("function") else { return false };
            let shadowed = |name: &str| model.is_some_and(|m| m.lookup(m.scope_at(v.start_byte()), name, usize::MAX).is_some());
            match f.kind() {
                "identifier" => {
                    let name = text(f, src);
                    if ITER_BUILTINS.contains(&name) && !shadowed(name) {
                        return true;
                    }
                    if ITER_ITERTOOLS.contains(&name) && model.is_some_and(|m| m.lookup(m.scope_at(v.start_byte()), name, usize::MAX).is_some_and(|s| m.imported_from(s, "itertools", Some(name)))) {
                        return true;
                    }
                    defines_generator(root, name, src)
                }
                "attribute" => {
                    let (Some(o), Some(a)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { return false };
                    o.kind() == "identifier" && text(o, src) == "itertools" && ITER_ITERTOOLS.contains(&text(a, src))
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn defines_generator(root: Node<'_>, name: &str, src: &str) -> bool {
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if c.kind() == "function_definition" && c.child_by_field_name("name").is_some_and(|n| text(n, src) == name) && has_yield(c) {
                return true;
            }
            stack.push(c);
        }
    }
    false
}

/// The identifiers an iterable expression reads as iterators: the name, or a
/// name passed (however deeply) through calls.
fn iterable_names<'t>(e: Node<'t>, src: &str, out: &mut Vec<Node<'t>>) {
    let e = unparen(e);
    match e.kind() {
        "identifier" => out.push(e),
        "call" => {
            for a in call_args(e, src).positional {
                iterable_names(a, src, out);
            }
        }
        _ => {}
    }
}

/// Whether `n` contains a `return`, or a `break` that leaves `n`'s own loop.
fn exits_loop(n: Node<'_>, nested_loop: bool) -> bool {
    for c in named_children(n) {
        match c.kind() {
            "return_statement" => return true,
            "break_statement" if !nested_loop => return true,
            "function_definition" | "class_definition" | "lambda" => {}
            "for_statement" | "while_statement" => {
                if exits_loop(c, true) {
                    return true;
                }
            }
            _ => {
                if exits_loop(c, nested_loop) {
                    return true;
                }
            }
        }
    }
    false
}

fn loop_else<'t>(l: Node<'t>) -> Option<Node<'t>> {
    let mut c = l.walk();
    l.children_by_field_name("alternative", &mut c).next()
}

fn bindings_of<'t>(scope: Node<'t>, name: &str, src: &str, out: &mut Vec<(usize, Option<Node<'t>>)>) {
    for c in named_children(scope) {
        match c.kind() {
            "function_definition" | "class_definition" | "lambda" => {
                if c.kind() != "lambda" && c.child_by_field_name("name").is_some_and(|n| text(n, src) == name) {
                    out.push((c.start_byte(), None));
                }
                continue;
            }
            "assignment" => {
                if let Some(l) = c.child_by_field_name("left") {
                    if l.kind() == "identifier" && text(l, src) == name {
                        out.push((c.start_byte(), c.child_by_field_name("right")));
                    } else if l.kind() != "attribute" && l.kind() != "subscript" && crate::util::mentions(text(l, src), name) {
                        out.push((c.start_byte(), None));
                    }
                }
            }
            "for_statement" => {
                if let Some(l) = c.child_by_field_name("left")
                    && crate::util::mentions(text(l, src), name)
                {
                    out.push((c.start_byte(), None));
                }
            }
            "as_pattern_target" | "named_expression" if crate::util::mentions(text(c, src), name) => {
                out.push((c.start_byte(), None));
            }
            _ => {}
        }
        bindings_of(c, name, src, out);
    }
}

fn looping_through_iterator(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    // The inner loop is a statement of the body of a `for` or `while`.
    let Some(block) = n.parent().filter(|p| p.kind() == "block") else { return };
    let Some(outer) = block.parent().filter(|p| matches!(p.kind(), "for_statement" | "while_statement")) else { return };
    if outer.child_by_field_name("body") != Some(block) {
        return;
    }
    // …and the loop is not left by the inner one or right after it.
    if let Some(body) = n.child_by_field_name("body")
        && exits_loop(body, false)
    {
        return;
    }
    if loop_else(n).is_some_and(|e| exits_loop(e, false)) {
        return;
    }
    let after = kids(block).into_iter().filter(|s| s.start_byte() >= n.end_byte());
    for s in after {
        if matches!(s.kind(), "break_statement" | "return_statement" | "raise_statement") {
            return;
        }
    }
    let Some(iter) = n.child_by_field_name("right") else { return };
    let root = root_of(n);
    // The function (or module) the loops belong to.
    let mut scope = n.parent();
    while let Some(p) = scope {
        if matches!(p.kind(), "function_definition" | "module") {
            break;
        }
        scope = p.parent();
    }
    let Some(scope) = scope else { return };
    let mut names = Vec::new();
    iterable_names(iter, src, &mut names);
    let mut seen = BTreeSet::new();
    for id in names {
        let name = text(id, src);
        if !seen.insert(name) {
            continue;
        }
        let mut binds = Vec::new();
        bindings_of(scope, name, src, &mut binds);
        // Rebound inside the outer loop: it is a fresh iterator each pass.
        if binds.iter().any(|(at, _)| *at >= outer.start_byte() && *at < outer.end_byte()) {
            continue;
        }
        let Some((_, Some(value))) = binds.iter().filter(|(at, _)| *at < outer.start_byte()).max_by_key(|(at, _)| *at).copied() else { continue };
        if is_iterator_value(value, root, src, ctx.semantic) {
            out.push(hit(id, format!("Iterator `{name}` from an outer scope is reused or consumed in a nested loop")));
        }
    }
}

py_rule!(LoopingThroughIterator, "py/looping-through-iterator", ["for_statement"], true, looping_through_iterator);

// ---------------------------------------------------------------- contextmanager-generator-missing-cleanup

fn is_contextmanager_decorator(d: Node<'_>, src: &str) -> bool {
    let t = text(crate::python_util::decorator_target(d), src);
    matches!(t.rsplit('.').next(), Some("contextmanager" | "asynccontextmanager"))
}

fn yields_in<'t>(f: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in named_children(f) {
        if c.kind() == "yield" {
            out.push(c);
        }
        if !matches!(c.kind(), "function_definition" | "class_definition" | "lambda") {
            yields_in(c, out);
        }
    }
}

fn yield_has_value(y: Node<'_>, src: &str) -> bool {
    kids(y).first().is_some_and(|v| !(v.kind() == "none" || text(*v, src) == "None"))
}

/// Whether the cleanup after a context manager's `yield` is run when the
/// generator is closed: the yield sits in a `try` with a `finally`, or one
/// whose handler catches `GeneratorExit` (or `BaseException`, `Exception` or
/// everything), or nothing follows the yield.
fn cleanup_runs(def: Node<'_>, y: Node<'_>, src: &str) -> bool {
    let mut cur = y;
    while let Some(p) = cur.parent() {
        if p.id() == def.id() {
            break;
        }
        if p.kind() == "try_statement" && p.child_by_field_name("body").is_some_and(|b| b.start_byte() <= y.start_byte() && y.end_byte() <= b.end_byte()) {
            for c in kids(p) {
                match c.kind() {
                    "finally_clause" => return true,
                    "except_clause" | "except_group_clause" => {
                        let header: Vec<Node<'_>> = kids(c).into_iter().filter(|x| x.kind() != "block").collect();
                        if header.is_empty() {
                            return true;
                        }
                        let t = header.iter().map(|h| text(*h, src)).collect::<Vec<_>>().join(" ");
                        if ["GeneratorExit", "BaseException", "Exception"].iter().any(|n| crate::util::mentions(&t, n)) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        cur = p;
    }
    // Nothing runs after the yield: no cleanup to skip.
    let mut cur = y;
    while let Some(p) = cur.parent() {
        if p.kind() == "block" && kids(p).iter().any(|s| s.start_byte() >= cur.end_byte()) {
            return false;
        }
        if p.id() == def.id() {
            break;
        }
        cur = p;
    }
    true
}

fn contextmanager_generator_missing_cleanup(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(body) = n.child_by_field_name("body") else { return };
    let Some(fname) = n.child_by_field_name("name") else { return };
    if decorators(n).iter().any(|d| is_contextmanager_decorator(*d, src)) {
        return;
    }
    let mut stack = vec![body];
    let root = root_of(n);
    while let Some(x) = stack.pop() {
        for w in named_children(x) {
            if matches!(w.kind(), "function_definition" | "class_definition" | "lambda") {
                continue;
            }
            stack.push(w);
            if w.kind() != "with_statement" {
                continue;
            }
            let Some(clause) = kids(w).into_iter().find(|c| c.kind() == "with_clause") else { continue };
            for item in kids(clause) {
                let value = if item.kind() == "with_item" { item.child_by_field_name("value").unwrap_or(item) } else { item };
                if value.kind() != "as_pattern" {
                    continue;
                }
                let parts = kids(value);
                let (Some(call), Some(alias)) = (parts.first().copied(), parts.iter().find(|p| p.kind() == "as_pattern_target").and_then(|t| kids(*t).first().copied())) else { continue };
                if call.kind() != "call" || alias.kind() != "identifier" {
                    continue;
                }
                let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { continue };
                let Some(cm) = find_def(root, text(f, src), src) else { continue };
                if !decorators(cm).iter().any(|d| is_contextmanager_decorator(*d, src)) {
                    continue;
                }
                // The body yields something built from the managed value.
                let mut ys = Vec::new();
                if let Some(wb) = w.child_by_field_name("body") {
                    yields_in(wb, &mut ys);
                }
                let alias_name = text(alias, src);
                if !ys.iter().any(|y| crate::util::mentions(text(*y, src), alias_name)) {
                    continue;
                }
                // The manager hands out a value and its cleanup can be skipped.
                let mut cm_yields = Vec::new();
                yields_in(cm.child_by_field_name("body").unwrap_or(cm), &mut cm_yields);
                if cm_yields.is_empty() || !cm_yields.iter().all(|y| yield_has_value(*y, src)) {
                    continue;
                }
                if cm_yields.iter().all(|y| cleanup_runs(cm, *y, src)) {
                    continue;
                }
                out.push(NativeFinding {
                    start: w.start_byte(),
                    end: w.end_byte(),
                    message: format!("The context used in function `{}` will not be exited", text(fname, src)),
                    edits: Vec::new(),
                });
            }
        }
    }
}

fn find_def<'t>(root: Node<'t>, name: &str, src: &str) -> Option<Node<'t>> {
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if c.kind() == "function_definition" && c.child_by_field_name("name").is_some_and(|n| text(n, src) == name) {
                return Some(c);
            }
            stack.push(c);
        }
    }
    None
}

py_rule!(
    ContextmanagerGeneratorMissingCleanup,
    "py/contextmanager-generator-missing-cleanup",
    ["function_definition"],
    false,
    contextmanager_generator_missing_cleanup
);

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &UsedBeforeAssignment,
    &PossiblyUsedBeforeAssignment,
    &UndefinedLoopVariable,
    &PossiblyUnusedVariable,
    &LoopingThroughIterator,
    &ContextmanagerGeneratorMissingCleanup,
];

#[cfg(test)]
mod tests {
    use super::*;
    use codelaw_core::Language;
    use std::collections::BTreeMap;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        let root = codelaw_lang::parse(Language::Python, src);
        let model = codelaw_semantic::build(Language::Python, &root, src);
        let options = BTreeMap::new();
        let data = BTreeMap::new();
        let file = crate::MemoryFiles::new("example.py", src, &[]);
        let ctx = NativeContext { source: src, language: Language::Python, semantic: model.as_ref(), options: &options, data: &data, file: &file };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            if rule.kinds().contains(&node.kind().as_ref()) {
                rule.check(node, &ctx, &mut out);
            }
        }
        out
    }

    #[test]
    fn binding_paths() {
        assert_eq!(run(&UsedBeforeAssignment, "def f():\n    print(x)\n    x = 1\n").len(), 1);
        assert!(run(&UsedBeforeAssignment, "def f(a):\n    if a:\n        x = 1\n    else:\n        x = 2\n    return x\n").is_empty());
        assert_eq!(run(&PossiblyUsedBeforeAssignment, "def f(a):\n    if a:\n        x = 1\n    return x\n").len(), 1);
        // A chained assignment binds every target.
        assert!(run(&UsedBeforeAssignment, "def f():\n    a = b = 0\n    return a, b\n").is_empty());
        // The same condition guards the read.
        assert!(run(&PossiblyUsedBeforeAssignment, "def f(a):\n    if a:\n        x = 1\n    if a:\n        print(x)\n").is_empty());
    }

    #[test]
    fn loops_and_handlers() {
        assert_eq!(run(&UndefinedLoopVariable, "def f(xs):\n    for v in xs:\n        pass\n    return v\n").len(), 1);
        assert!(run(&UndefinedLoopVariable, "def f():\n    for v in range(3):\n        pass\n    return v\n").is_empty());
        assert_eq!(run(&UsedBeforeAssignment, "def f(t):\n    try:\n        n = int(t)\n    except ValueError:\n        print(n)\n").len(), 1);
        assert!(run(&UsedBeforeAssignment, "def f(t):\n    try:\n        n = 0\n        n = int(t)\n    except ValueError:\n        print(n)\n").is_empty());
    }
}

//! W-139 strand W-139-js, eslint-plugin-sonarjs 4.2.2's scope rows: rules
//! that ask where a binding is declared, written and read, or what a
//! library import names. Behaviour only (SonarJS's licence, L2): read from
//! the rule code and tests at the package's `gitHead`, reimplemented on the
//! per-file semantic model. None follows a value through a call or another
//! file; where upstream reads its code-path graph, the rule reads the
//! syntax and its page says what it leaves out.

use super::scope::{ancestors, enclosing_fn, fn_like};
use super::{JS, finding, span};
use crate::js_ast::{self, arguments, unparen};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{Imported, Reference, SemanticModel, SymbolId, SymbolKind};
use std::collections::{BTreeSet, HashMap};
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

fn same(a: Option<Node<'_>>, b: Node<'_>) -> bool {
    a.is_some_and(|a| a.id() == b.id())
}

fn in_range(pos: usize, outer: Node<'_>) -> bool {
    pos >= outer.start_byte() && pos < outer.end_byte()
}

fn node_at<'t>(root: Node<'t>, start: usize, end: usize) -> Option<Node<'t>> {
    root.descendant_for_byte_range(start, end)
}

fn refs_of(m: &SemanticModel, sym: SymbolId) -> Vec<&Reference> {
    m.symbol(sym).refs.iter().map(|&r| &m.references[r]).collect()
}

fn all_nodes(n: Node<'_>) -> Vec<Node<'_>> {
    let mut v = vec![n];
    crate::util::descendants(n, &|_| false, &mut v);
    v
}

/// Nodes under `n` in its own function: not entering nested functions.
fn own_nodes(n: Node<'_>) -> Vec<Node<'_>> {
    let mut v = Vec::new();
    crate::util::descendants(n, &|x| fn_like(x.kind()), &mut v);
    v
}

fn keyword_span(n: Node<'_>) -> (usize, usize) {
    n.child(0).map_or((n.start_byte(), n.end_byte()), |k| (k.start_byte(), k.end_byte()))
}

const LIBS: &[&str] = &["lodash", "lodash-es", "underscore"];

/// Whether `callee` names `name` of lodash, lodash-es or underscore: a named
/// import, a per-method package (`lodash/debounce`, `lodash.debounce`), or a
/// member of the default or namespace import.
fn lib_fn(m: &SemanticModel, callee: Node<'_>, name: &str, src: &str) -> bool {
    let callee = unparen(callee);
    let imp = |id: Node<'_>| m.symbol_at(id.start_byte()).and_then(|s| m.import_of(s));
    match callee.kind() {
        "identifier" => imp(callee).is_some_and(|i| match &i.imported {
            Imported::Named(x) => x == name && LIBS.contains(&i.source.as_str()),
            Imported::Default => LIBS.iter().any(|l| i.source == format!("{l}/{name}")) || i.source == format!("lodash.{name}"),
            _ => false,
        }),
        "member_expression" => {
            callee.child_by_field_name("property").is_some_and(|p| text(p, src) == name)
                && callee.child_by_field_name("object").filter(|o| o.kind() == "identifier").and_then(imp).is_some_and(|i| matches!(i.imported, Imported::Default | Imported::Namespace) && LIBS.contains(&i.source.as_str()))
        }
        _ => false,
    }
}

/// Whether `ident` is `name` imported from `module`.
fn imported(m: &SemanticModel, ident: Node<'_>, module: &str, name: &str) -> bool {
    ident.kind() == "identifier" && m.symbol_at(ident.start_byte()).is_some_and(|s| m.imported_from(s, module, Some(name)))
}

/// The function a callee identifier is bound to once: a declaration, or a
/// variable whose value is a function.
fn bound_function<'t>(callee: Node<'_>, root: Node<'t>, m: &SemanticModel) -> Option<Node<'t>> {
    if callee.kind() != "identifier" {
        return None;
    }
    let sym = m.symbol_at(callee.start_byte())?;
    let s = m.symbol(sym);
    if s.decls.len() != 1 {
        return None;
    }
    let id = node_at(root, s.decls[0].0, s.decls[0].1)?;
    match s.kind {
        SymbolKind::Function => id.parent().filter(|p| matches!(p.kind(), "function_declaration" | "generator_function_declaration")),
        SymbolKind::Const | SymbolKind::Let | SymbolKind::Var => js_ast::initial_value(m, root, id).and_then(js_ast::as_function),
        _ => None,
    }
}

fn has_rest(f: Node<'_>) -> bool {
    js_ast::params(f).iter().any(|p| p.kind() == "rest_pattern" || p.child_by_field_name("pattern").is_some_and(|x| x.kind() == "rest_pattern"))
}

fn runtime_params(f: Node<'_>) -> usize {
    js_ast::params(f).iter().filter(|p| !p.child_by_field_name("pattern").is_some_and(|x| x.kind() == "this")).count()
}

fn uses_arguments(f: Node<'_>, m: &SemanticModel, src: &str) -> bool {
    let mut v = Vec::new();
    crate::util::descendants(f, &|x| fn_like(x.kind()) && x.kind() != "arrow_function", &mut v);
    v.into_iter().any(|x| x.kind() == "identifier" && text(x, src) == "arguments" && m.symbol_at(x.start_byte()).is_none())
}

fn is_loop(k: &str) -> bool {
    matches!(k, "for_statement" | "for_in_statement" | "while_statement" | "do_statement")
}

fn squash(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// ---- S9135 avoid-mutating-nested-properties-of-shallow-clones ---------------

/// The member chain's root identifier and its static keys, outermost last.
fn static_chain<'t>(n: Node<'t>, src: &str) -> Option<(Node<'t>, Vec<String>)> {
    let mut keys = Vec::new();
    let mut x = unparen(n);
    loop {
        match x.kind() {
            "identifier" => {
                keys.reverse();
                return Some((x, keys));
            }
            "member_expression" => {
                let p = x.child_by_field_name("property").filter(|p| p.kind() == "property_identifier")?;
                keys.push(text(p, src).to_owned());
                x = unparen(x.child_by_field_name("object")?);
            }
            "subscript_expression" => {
                let i = x.child_by_field_name("index").filter(|i| matches!(i.kind(), "string" | "number"))?;
                keys.push(crate::util::unquote(text(i, src)).to_owned());
                x = unparen(x.child_by_field_name("object")?);
            }
            _ => return None,
        }
    }
}

srule!(AvoidMutatingNestedPropertiesOfShallowClones, "js/avoid-mutating-nested-properties-of-shallow-clones", &["assignment_expression", "augmented_assignment_expression", "update_expression", "unary_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let target = match n.kind() {
        "unary_expression" if text(n, src).starts_with("delete") => n.child_by_field_name("argument"),
        "update_expression" => n.child_by_field_name("argument"),
        "unary_expression" => None,
        _ => n.child_by_field_name("left"),
    };
    let Some((root_id, keys)) = target.and_then(|t| static_chain(t, src)) else { return };
    if keys.len() < 2 {
        return;
    }
    let Some(sym) = m.symbol_at(root_id.start_byte()) else { return };
    let s = m.symbol(sym);
    if s.decls.len() != 1 || refs_of(m, sym).iter().any(|r| r.write) {
        return;
    }
    let root = js_ast::root_of(n);
    let Some(decl) = node_at(root, s.decls[0].0, s.decls[0].1) else { return };
    let Some(call) = js_ast::initial_value(m, root, decl).map(unparen).filter(|c| c.kind() == "call_expression") else { return };
    let args = arguments(call);
    if args.len() != 1 || !call.child_by_field_name("function").is_some_and(|f| lib_fn(m, f, "clone", src)) {
        return;
    }
    // An earlier sibling statement that gives a prefix a fresh value isolates it.
    let rname = text(root_id, src);
    let mut child = n;
    for a in ancestors(n) {
        if fn_like(a.kind()) {
            break;
        }
        if matches!(a.kind(), "statement_block" | "program" | "switch_case") {
            for st in named_children(a) {
                if st.start_byte() >= child.start_byte() {
                    break;
                }
                for x in all_nodes(st) {
                    if x.kind() != "assignment_expression" {
                        continue;
                    }
                    let (Some(l), Some(r)) = (x.child_by_field_name("left"), x.child_by_field_name("right")) else { continue };
                    let Some((lr, lk)) = static_chain(l, src) else { continue };
                    if text(lr, src) != rname || lk.is_empty() || lk.len() >= keys.len() || keys[..lk.len()] != lk[..] {
                        continue;
                    }
                    let r = unparen(r);
                    let callee = r.child_by_field_name("function").map(|f| text(f, src)).unwrap_or("");
                    let deep = r.kind() == "call_expression" && (callee == "structuredClone" || callee.ends_with("cloneDeep"));
                    let shallow = matches!(r.kind(), "object" | "array") || (r.kind() == "call_expression" && r.child_by_field_name("function").is_some_and(|f| lib_fn(m, f, "clone", src)));
                    if deep || (shallow && keys.len() == lk.len() + 1) {
                        return;
                    }
                }
            }
        }
        child = a;
    }
    let edit = TextEdit::new(call.start_byte(), call.end_byte(), format!("structuredClone({})", text(args[0], src)));
    out.push(finding(n, format!("`{rname}` is a shallow clone, so this changes the original's nested object too: clone it deeply (`structuredClone()`) or copy the nested object first"), vec![edit]));
});

// ---- S1515 function-inside-loop ------------------------------------------------

const CALLBACK_METHODS: &[&str] = &["replace", "forEach", "filter", "map", "find", "findIndex", "every", "some", "reduce", "reduceRight", "sort", "each"];

fn loop_header(l: Node<'_>) -> Option<(usize, usize)> {
    let body = l.child_by_field_name("body")?;
    match l.kind() {
        "for_statement" => l.child_by_field_name("condition").filter(|c| c.kind() != "empty_statement").map(|c| (c.start_byte(), body.start_byte())),
        "for_in_statement" => Some((l.start_byte(), body.start_byte())),
        _ => l.child_by_field_name("condition").map(|c| (c.start_byte(), c.end_byte())),
    }
}

srule!(FunctionInsideLoop, "js/function-inside-loop", &["function_declaration", "function_expression", "function", "arrow_function", "generator_function", "generator_function_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(lp) = ancestors(n).find(|a| is_loop(a.kind())) else { return };
    let mut x = n;
    while x.parent().is_some_and(|p| p.kind() == "parenthesized_expression") {
        let Some(p) = x.parent() else { break };
        x = p;
    }
    if let Some(p) = x.parent() {
        if p.kind() == "call_expression" && same(p.child_by_field_name("function"), x) {
            return;
        }
        if p.kind() == "member_expression" && same(p.child_by_field_name("object"), x) {
            return;
        }
        if p.kind() == "arguments"
            && let Some(call) = p.parent()
            && call.child_by_field_name("function").is_some_and(|f| f.kind() == "member_expression" && f.child_by_field_name("property").is_some_and(|q| CALLBACK_METHODS.contains(&text(q, src))))
        {
            return;
        }
    }
    let Some(body) = lp.child_by_field_name("body") else { return };
    let header = loop_header(lp);
    let root = js_ast::root_of(n);
    let mut names = BTreeSet::new();
    for r in &m.references {
        if !in_range(r.start, n) || r.type_only {
            continue;
        }
        let Some(sym) = r.symbol else { continue };
        let s = m.symbol(sym);
        if matches!(s.kind, SymbolKind::Let | SymbolKind::Const | SymbolKind::Class | SymbolKind::Import) || s.decls.first().is_some_and(|d| in_range(d.0, n)) {
            continue;
        }
        let unsafe_write = refs_of(m, sym).iter().any(|w| {
            w.write
                && (header.is_some_and(|(a, b)| w.start >= a && w.start < b)
                    || node_at(root, w.start, w.end).is_some_and(|id| ancestors(id).find(|a| matches!(a.kind(), "statement_block") || fn_like(a.kind())).is_some_and(|b| b.id() == body.id())))
        });
        if unsafe_write {
            names.insert(s.name.clone());
        }
    }
    if names.is_empty() {
        return;
    }
    let arrow = children(n).into_iter().find(|c| c.kind() == "=>").map(|c| (c.start_byte(), c.end_byte()));
    let head = n.child_by_field_name("name").map(|nm| (nm.start_byte(), nm.end_byte())).or(arrow).unwrap_or_else(|| keyword_span(n));
    let list = names.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(", ");
    out.push(span(head.0, head.1, format!("This function is made in a loop and reads {list}, which the loop changes: define it outside the loop"), vec![]));
});

// ---- S3686 inconsistent-function-call -------------------------------------------

srule!(InconsistentFunctionCall, "js/inconsistent-function-call", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    let mut last: HashMap<SymbolId, [Option<usize>; 2]> = HashMap::new();
    let mut reported = BTreeSet::new();
    for x in all_nodes(n) {
        let (callee, is_new) = match x.kind() {
            "call_expression" => (x.child_by_field_name("function"), 0),
            "new_expression" => (x.child_by_field_name("constructor"), 1),
            _ => continue,
        };
        let Some(c) = callee.filter(|c| c.kind() == "identifier") else { continue };
        let Some(sym) = m.symbol_at(c.start_byte()).filter(|s| !m.symbol(*s).decls.is_empty()) else { continue };
        let e = last.entry(sym).or_default();
        if let Some(other) = e[1 - is_new]
            && reported.insert(sym)
        {
            let line = crate::util::line_of(src, other);
            let how = if is_new == 1 { "without `new`" } else { "with `new`" };
            out.push(finding(c, format!("`{}` is called {how} on line {line}: call it the same way everywhere", text(c, src)), vec![]));
        }
        e[is_new] = Some(c.start_byte());
    }
});

// ---- S8932 memoize-cache-key ----------------------------------------------------

srule!(MemoizeCacheKey, "js/memoize-cache-key", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(callee) = n.child_by_field_name("function") else { return };
    if !lib_fn(m, callee, "memoize", src) {
        return;
    }
    let args = arguments(n);
    if let Some(r) = args.get(1)
        && !matches!(text(*r, src), "undefined" | "null")
        && !(r.kind() == "unary_expression" && text(*r, src).starts_with("void"))
    {
        return;
    }
    let Some(first) = args.first() else { return };
    let root = js_ast::root_of(n);
    let f = js_ast::as_function(*first).or_else(|| bound_function(*first, root, m));
    let Some(f) = f else { return };
    if runtime_params(f) < 2 && !has_rest(f) {
        return;
    }
    let at = if callee.kind() == "member_expression" { callee.child_by_field_name("property").unwrap_or(callee) } else { callee };
    out.push(finding(at, "`memoize` keys its cache by the first argument only, but this function takes more: pass a resolver as the second argument", vec![]));
});

// ---- S1994 misplaced-loop-counter -------------------------------------------------

fn leaves<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    match n.kind() {
        "array_pattern" | "object_pattern" | "pair_pattern" | "assignment_pattern" | "object_assignment_pattern" | "rest_pattern" | "parenthesized_expression" => {
            for c in named_children(n) {
                if n.kind() == "pair_pattern" && same(n.child_by_field_name("key"), c) {
                    continue;
                }
                if matches!(n.kind(), "assignment_pattern" | "object_assignment_pattern") && same(n.child_by_field_name("right"), c) {
                    continue;
                }
                leaves(c, out);
            }
        }
        _ => out.push(n),
    }
}

srule!(MisplacedLoopCounter, "js/misplaced-loop-counter", &["for_statement"], |n, ctx, _m, out| {
    let src = ctx.source;
    let Some(mut test) = n.child_by_field_name("condition") else { return };
    if test.kind() == "expression_statement" {
        let Some(t) = test.named_child(0) else { return };
        test = t;
    }
    if test.kind() == "empty_statement" {
        return;
    }
    let Some(update) = n.child_by_field_name("increment") else { return };
    let mut updated = Vec::new();
    for x in all_nodes(update) {
        match x.kind() {
            "assignment_expression" | "augmented_assignment_expression" => {
                if let Some(l) = x.child_by_field_name("left") {
                    leaves(l, &mut updated);
                }
            }
            "update_expression" => updated.extend(x.child_by_field_name("argument")),
            "call_expression" => {
                if let Some(f) = x.child_by_field_name("function").filter(|f| f.kind() == "member_expression")
                    && let Some(r) = js_ast::chain_root(f)
                {
                    updated.push(r);
                }
            }
            _ => {}
        }
    }
    if updated.is_empty() {
        return;
    }
    let mut tested = Vec::new();
    for x in all_nodes(test) {
        let p = x.parent();
        match x.kind() {
            "identifier" | "this" => tested.push(x),
            "member_expression" if !p.is_some_and(|p| matches!(p.kind(), "member_expression" | "call_expression")) => tested.push(x),
            _ => {}
        }
    }
    let ut: BTreeSet<String> = updated.iter().map(|u| squash(text(*u, src))).collect();
    if tested.iter().any(|t| ut.contains(&squash(text(*t, src)))) {
        return;
    }
    let tlist: Vec<String> = tested.iter().filter(|t| t.kind() != "identifier" || !t.parent().is_some_and(|p| p.kind() == "member_expression")).map(|t| text(*t, src).to_owned()).collect();
    let (s, e) = keyword_span(n);
    out.push(span(s, e, format!("This loop's condition tests {} but its update changes {}: update the variable the condition tests", tlist.join(", "), updated.iter().map(|u| text(*u, src)).collect::<Vec<_>>().join(", ")), vec![]));
});

// ---- S9114 no-debounce-throttle-in-render -------------------------------------------

fn component_name<'s>(f: Node<'_>, src: &'s str) -> Option<&'s str> {
    if let Some(nm) = f.child_by_field_name("name") {
        return Some(text(nm, src));
    }
    let mut x = f;
    for _ in 0..3 {
        let p = x.parent()?;
        match p.kind() {
            "variable_declarator" => return p.child_by_field_name("name").map(|nm| text(nm, src)),
            "arguments" => x = p.parent()?,
            "parenthesized_expression" => x = p,
            _ => return None,
        }
    }
    None
}

fn is_render(f: Node<'_>, src: &str) -> bool {
    let member = if f.kind() == "method_definition" { Some(f) } else { f.parent().filter(|p| matches!(p.kind(), "field_definition" | "public_field_definition")) };
    let Some(mem) = member else { return false };
    if children(mem).iter().any(|c| c.kind() == "static") || !mem.child_by_field_name("name").or(mem.child_by_field_name("property")).is_some_and(|k| text(k, src) == "render") {
        return false;
    }
    let class = ancestors(mem).find(|a| matches!(a.kind(), "class" | "class_declaration"));
    class.and_then(|c| named_children(c).into_iter().find(|x| x.kind() == "class_heritage")).is_some_and(|h| {
        let t = text(h, src).trim_start_matches("extends").trim();
        matches!(t, "Component" | "PureComponent" | "React.Component" | "React.PureComponent")
    })
}

srule!(NoDebounceThrottleInRender, "js/no-debounce-throttle-in-render", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(callee) = n.child_by_field_name("function") else { return };
    let which = if lib_fn(m, callee, "debounce", src) { "debounce" } else if lib_fn(m, callee, "throttle", src) { "throttle" } else { return };
    let Some(f) = enclosing_fn(n) else { return };
    let class = is_render(f, src);
    let comp = component_name(f, src).is_some_and(|nm| nm.starts_with(|c: char| c.is_ascii_uppercase()) || (nm.starts_with("use") && nm[3..].starts_with(|c: char| c.is_ascii_uppercase())));
    if !class && !comp {
        return;
    }
    let hook = |c: Node<'_>, names: &[&str]| c.kind() == "call_expression" && c.child_by_field_name("function").is_some_and(|x| names.contains(&text(x, src).trim_start_matches("React.")));
    if ancestors(n).take_while(|a| a.id() != f.id()).any(|a| hook(a, &["useMemo", "useCallback"])) {
        return;
    }
    if n.parent().filter(|p| p.kind() == "arguments").and_then(|a| a.parent()).is_some_and(|c| hook(c, &["useRef"]) && arguments(c).first().is_some_and(|x| x.id() == n.id())) {
        return;
    }
    // `if (!ref.current) ref.current = debounce(…)`
    if let Some(asg) = n.parent().filter(|p| p.kind() == "assignment_expression")
        && let Some(l) = asg.child_by_field_name("left")
        && text(l, src).ends_with(".current")
        && ancestors(asg).find(|a| a.kind() == "if_statement").and_then(|i| i.child_by_field_name("condition")).is_some_and(|c| {
            let t = squash(text(c, src));
            let lt = squash(text(l, src));
            t == format!("(!{lt})") || t == format!("({lt}==null)") || t == format!("({lt}===null)") || t == format!("(null=={lt})") || t == format!("(null==={lt})")
        })
    {
        return;
    }
    let at = if callee.kind() == "member_expression" { callee.child_by_field_name("property").unwrap_or(callee) } else { callee };
    let fix = if class { "create it once in the constructor" } else { "create it once, outside the component or with `useMemo`" };
    out.push(finding(at, format!("`{which}()` in render makes a new function every render, so it never fires as meant: {fix}"), vec![]));
});

// ---- S4158 no-empty-collection / S4030 no-unused-collection ----------------------------

const COLLECTIONS: &[&str] = &["Array", "Map", "Set", "WeakSet", "WeakMap"];

fn collection(v: Node<'_>, src: &str, empty: bool) -> bool {
    let v = unparen(v);
    match v.kind() {
        "array" => !empty || named_children(v).iter().all(|c| c.kind() == "comment"),
        "new_expression" | "call_expression" => {
            let c = v.child_by_field_name("constructor").or_else(|| v.child_by_field_name("function"));
            c.is_some_and(|c| COLLECTIONS.contains(&text(c, src))) && (!empty || arguments(v).is_empty())
        }
        _ => false,
    }
}

const READS: &[&str] = &[
    "copyWithin", "pop", "reverse", "shift", "sort", "clear", "delete", "concat", "flat", "flatMap", "includes", "indexOf", "join", "lastIndexOf", "slice", "toSource", "toString", "toLocaleString", "get", "has", "entries", "every",
    "filter", "find", "findIndex", "forEach", "keys", "map", "reduce", "reduceRight", "some", "values",
];
const WRITES: &[&str] = &["push", "pop", "shift", "unshift", "splice", "sort", "reverse", "fill", "copyWithin", "set", "add", "clear", "delete"];

/// `x = <value>;` as a statement, where `id` is `x`.
fn assigned_statement<'t>(id: Node<'t>) -> Option<Node<'t>> {
    let a = id.parent().filter(|a| a.kind() == "assignment_expression" && same(a.child_by_field_name("left"), id))?;
    a.parent().filter(|s| s.kind() == "expression_statement")?;
    a.child_by_field_name("right")
}

srule!(NoEmptyCollection, "js/no-empty-collection", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    for (i, s) in m.symbols.iter().enumerate() {
        if !matches!(s.kind, SymbolKind::Const | SymbolKind::Let | SymbolKind::Var) || s.decls.len() != 1 || s.name == "arguments" {
            continue;
        }
        let Some(decl) = node_at(n, s.decls[0].0, s.decls[0].1) else { continue };
        let init = js_ast::initial_value(m, n, decl);
        let refs = refs_of(m, i);
        if refs.len() + usize::from(init.is_some()) < 2 {
            continue;
        }
        let mut empty = init.is_some_and(|v| collection(v, src, true));
        if init.is_some() && !empty {
            continue;
        }
        let mut readings = Vec::new();
        let mut ok = true;
        for r in &refs {
            let Some(id) = node_at(n, r.start, r.end) else { ok = false; break };
            if r.write {
                if assigned_statement(id).is_some_and(|v| collection(v, src, true)) {
                    empty = true;
                    continue;
                }
                ok = false;
                break;
            }
            let p = id.parent();
            let reading = match p.map(|p| p.kind()) {
                Some("member_expression") => p.is_some_and(|p| same(p.child_by_field_name("object"), id) && p.child_by_field_name("property").is_some_and(|q| READS.contains(&text(q, src))) && p.parent().is_some_and(|c| c.kind() == "call_expression" && same(c.child_by_field_name("function"), p))),
                Some("subscript_expression") => p.is_some_and(|p| same(p.child_by_field_name("object"), id) && !p.parent().is_some_and(|a| matches!(a.kind(), "assignment_expression" | "augmented_assignment_expression") && same(a.child_by_field_name("left"), p))),
                Some("for_in_statement") => p.is_some_and(|p| same(p.child_by_field_name("right"), id)),
                _ => false,
            };
            if !reading {
                ok = false;
                break;
            }
            readings.push(id);
        }
        if ok && empty {
            for id in readings {
                out.push(finding(id, format!("`{}` is always empty here: nothing ever adds to it", text(id, src)), vec![]));
            }
        }
    }
});

srule!(NoUnusedCollection, "js/no-unused-collection", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    for (i, s) in m.symbols.iter().enumerate() {
        if !matches!(s.kind, SymbolKind::Const | SymbolKind::Let | SymbolKind::Var) || s.decls.len() != 1 || s.exported {
            continue;
        }
        let Some(decl) = node_at(n, s.decls[0].0, s.decls[0].1) else { continue };
        let init = js_ast::initial_value(m, n, decl);
        if init.is_some_and(|v| !collection(v, src, false)) {
            continue;
        }
        let refs = refs_of(m, i);
        if refs.len() + usize::from(init.is_some()) < 2 {
            continue;
        }
        let mut any_collection = init.is_some();
        let ok = refs.iter().all(|r| {
            let Some(id) = node_at(n, r.start, r.end) else { return false };
            if r.write {
                let c = assigned_statement(id).is_some_and(|v| collection(v, src, false));
                any_collection |= c;
                return c;
            }
            let Some(p) = id.parent().filter(|p| matches!(p.kind(), "member_expression" | "subscript_expression") && same(p.child_by_field_name("object"), id)) else { return false };
            let Some(gp) = p.parent() else { return false };
            let stmt_of = |x: Node<'_>| x.parent().is_some_and(|s| s.kind() == "expression_statement");
            (matches!(gp.kind(), "assignment_expression" | "augmented_assignment_expression") && same(gp.child_by_field_name("left"), p) && stmt_of(gp))
                || (gp.kind() == "call_expression" && same(gp.child_by_field_name("function"), p) && p.child_by_field_name("property").is_some_and(|q| WRITES.contains(&text(q, src))) && stmt_of(gp))
        });
        if ok && any_collection {
            out.push(finding(decl, format!("`{}` is filled but never read: remove it, or use what it collects", s.name), vec![]));
        }
    }
});

// ---- S930 no-extra-arguments ---------------------------------------------------------

srule!(NoExtraArguments, "js/no-extra-arguments", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(callee) = n.child_by_field_name("function") else { return };
    let root = js_ast::root_of(n);
    let f = js_ast::as_function(callee).or_else(|| bound_function(callee, root, m));
    let Some(f) = f else { return };
    if callee.kind() == "identifier" && m.symbol_at(callee.start_byte()).is_some_and(|s| m.symbol(s).kind == SymbolKind::Param) {
        return;
    }
    let args = arguments(n);
    let params = runtime_params(f);
    if args.len() <= params || has_rest(f) || args.iter().any(|a| a.kind() == "spread_element") {
        return;
    }
    if f.kind() != "arrow_function" && uses_arguments(f, m, src) {
        return;
    }
    if params == 0 && js_ast::body(f).is_some_and(|b| b.kind() == "statement_block" && named_children(b).iter().all(|c| c.kind() == "comment")) {
        return;
    }
    let expects = match params {
        0 => "no arguments".to_owned(),
        1 => "1 argument".to_owned(),
        k => format!("{k} arguments"),
    };
    out.push(finding(callee, format!("This function expects {expects}, but {} were provided: remove the extra ones", args.len()), vec![]));
});

// ---- S2589 no-gratuitous-expressions ----------------------------------------------------

fn conjuncts<'t>(t: Node<'t>, src: &str, out: &mut Vec<(Node<'t>, bool)>) {
    let t = unparen(t);
    match t.kind() {
        "binary_expression" if t.child_by_field_name("operator").is_some_and(|o| text(o, src) == "&&") => {
            if let (Some(l), Some(r)) = (t.child_by_field_name("left"), t.child_by_field_name("right")) {
                conjuncts(l, src, out);
                conjuncts(r, src, out);
            }
        }
        "identifier" => out.push((t, true)),
        "unary_expression" if t.child_by_field_name("operator").is_some_and(|o| text(o, src) == "!") => {
            let Some(a) = t.child_by_field_name("argument").map(unparen) else { return };
            if a.kind() == "identifier" {
                out.push((a, false));
            } else if a.kind() == "unary_expression" && a.child_by_field_name("operator").is_some_and(|o| text(o, src) == "!") && let Some(b) = a.child_by_field_name("argument").map(unparen).filter(|b| b.kind() == "identifier") {
                out.push((b, true));
            }
        }
        _ => {}
    }
}

srule!(NoGratuitousExpressions, "js/no-gratuitous-expressions", &["if_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(cond) = n.child_by_field_name("condition") else { return };
    let c = unparen(cond);
    if matches!(c.kind(), "true" | "false") {
        out.push(finding(c, format!("This condition is always {}: remove the `if` or the dead branch", if c.kind() == "true" { "true" } else { "false" }), vec![]));
        return;
    }
    let mut branches: Vec<(Node<'_>, Vec<(Node<'_>, bool)>)> = Vec::new();
    if let Some(b) = n.child_by_field_name("consequence").filter(|b| b.kind() == "statement_block") {
        let mut k = Vec::new();
        conjuncts(cond, src, &mut k);
        branches.push((b, k));
    }
    if let Some(alt) = n.child_by_field_name("alternative").and_then(|a| named_children(a).into_iter().find(|x| x.kind() != "comment")).filter(|b| b.kind() == "statement_block")
        && c.kind() == "identifier"
    {
        branches.push((alt, vec![(c, false)]));
    }
    let root = js_ast::root_of(n);
    for (branch, known) in branches {
        let bfn = enclosing_fn(branch).map(|f| f.id());
        let mut table: HashMap<SymbolId, (bool, Node<'_>)> = HashMap::new();
        for (id, truthy) in known {
            let Some(sym) = m.symbol_at(id.start_byte()) else { continue };
            let changed = refs_of(m, sym).iter().any(|w| w.write && (in_range(w.start, branch) || node_at(root, w.start, w.end).and_then(enclosing_fn).map(|f| f.id()) != bfn));
            if !changed {
                table.insert(sym, (truthy, id));
            }
        }
        if table.is_empty() {
            continue;
        }
        for x in all_nodes(branch) {
            if x.kind() != "identifier" {
                continue;
            }
            let Some(sym) = m.reference_at(x.start_byte()).and_then(|r| r.symbol) else { continue };
            let Some((truthy, _)) = table.get(&sym) else { continue };
            let Some(p) = x.parent() else { continue };
            let op = p.child_by_field_name("operator").map(|o| text(o, src));
            let fires = match p.kind() {
                "binary_expression" => op == Some("&&") || (op == Some("||") && same(p.child_by_field_name("left"), x)),
                "unary_expression" => op == Some("!"),
                "parenthesized_expression" => p.parent().is_some_and(|g| g.kind() == "if_statement"),
                _ => false,
            };
            let jsx = op == Some("&&") && same(p.child_by_field_name("right"), x) && p.parent().is_some_and(|g| g.kind() == "jsx_expression");
            if fires && !jsx {
                out.push(finding(x, format!("`{}` is always {} here: the enclosing condition already decided it", text(x, src), if *truthy { "truthy" } else { "falsy" }), vec![]));
            }
        }
    }
});

// ---- S3516 no-invariant-returns ------------------------------------------------------------

fn literal_value(n: Node<'_>, root: Node<'_>, m: &SemanticModel, f: Node<'_>, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "string" => Some(format!("s:{}", crate::util::unquote(text(n, src)))),
        "number" => text(n, src).replace('_', "").parse::<f64>().ok().map(|v| format!("n:{v}")),
        "true" | "false" | "null" => Some(text(n, src).to_owned()),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Some(format!("s:{}", text(n, src).trim_matches('`'))),
        "unary_expression" => {
            let op = text(n.child_by_field_name("operator")?, src);
            let inner = literal_value(n.child_by_field_name("argument")?, root, m, f, src)?;
            matches!(op, "-" | "+" | "!" | "~" | "typeof").then(|| format!("{op}({inner})"))
        }
        "identifier" => {
            let sym = m.symbol_at(n.start_byte())?;
            let s = m.symbol(sym);
            if s.decls.len() != 1 || refs_of(m, sym).iter().any(|r| r.write) || !in_range(s.decls[0].0, f) {
                return None;
            }
            let v = js_ast::initial_value(m, root, node_at(root, s.decls[0].0, s.decls[0].1)?)?;
            if unparen(v).kind() == "identifier" {
                return None;
            }
            literal_value(v, root, m, f, src)
        }
        _ => None,
    }
}

srule!(NoInvariantReturns, "js/no-invariant-returns", &["function_declaration", "function_expression", "function", "arrow_function", "method_definition"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(body) = js_ast::body(n).filter(|b| b.kind() == "statement_block") else { return };
    let nodes = own_nodes(body);
    if nodes.iter().any(|x| x.kind() == "throw_statement") || !crate::util::returns_value(&body) {
        return;
    }
    let returns: Vec<Node<'_>> = nodes.iter().copied().filter(|x| x.kind() == "return_statement").collect();
    if returns.len() < 2 {
        return;
    }
    let values: Vec<Node<'_>> = returns.iter().filter_map(|r| named_children(*r).into_iter().find(|c| c.kind() != "comment")).collect();
    if values.len() != returns.len() {
        return;
    }
    let root = js_ast::root_of(n);
    let lits: Vec<Option<String>> = values.iter().map(|v| literal_value(*v, root, m, n, src)).collect();
    let literal_case = lits.iter().all(Option::is_some) && lits.windows(2).all(|w| w[0] == w[1]);
    let ident_case = !literal_case && values.iter().all(|v| unparen(*v).kind() == "identifier") && {
        let syms: BTreeSet<Option<SymbolId>> = values.iter().map(|v| m.symbol_at(unparen(*v).start_byte())).collect();
        match syms.into_iter().collect::<Vec<_>>()[..] {
            [Some(sym)] => {
                let s = m.symbol(sym);
                s.decls.len() == 1 && in_range(s.decls[0].0, body) && !refs_of(m, sym).iter().any(|r| r.write) && !refs_of(m, sym).iter().any(|r| {
                    node_at(root, r.start, r.end).is_some_and(|id| ancestors(id).take_while(|a| a.kind() != "expression_statement" && !fn_like(a.kind())).any(|a| a.kind() == "call_expression" || (matches!(a.kind(), "assignment_expression" | "augmented_assignment_expression") && a.child_by_field_name("left").is_some_and(|l| l.kind() != "identifier" && in_range(id.start_byte(), l)))) && ancestors(id).any(|a| a.kind() == "expression_statement"))
                })
            }
            _ => false,
        }
    };
    if !literal_case && !ident_case {
        return;
    }
    // Branch frames: a block with a side effect and no return of its own is
    // the guard-and-log shape upstream leaves alone.
    let side = |b: Node<'_>| named_children(b).iter().any(|s| s.kind() == "expression_statement" && s.named_child(0).is_some_and(|e| matches!(e.kind(), "call_expression" | "assignment_expression" | "augmented_assignment_expression" | "await_expression")));
    let has_ret = |b: Node<'_>| named_children(b).iter().any(|s| s.kind() == "return_statement");
    let blocks: Vec<Node<'_>> = std::iter::once(body).chain(nodes.iter().copied().filter(|x| x.kind() == "statement_block" || x.kind() == "switch_case")).collect();
    if ident_case && blocks.iter().any(|b| side(*b) && !has_ret(*b)) {
        return;
    }
    if literal_case && blocks.iter().any(|b| side(*b) && has_ret(*b)) && !blocks.iter().any(|b| has_ret(*b) && !side(*b)) {
        return;
    }
    let head = n.child_by_field_name("name").map_or_else(|| keyword_span(n), |nm| (nm.start_byte(), nm.end_byte()));
    out.push(span(head.0, head.1, format!("Every return of this function gives `{}`: return nothing, or return what differs", text(values[0], src)), vec![]));
});

// ---- S9163 no-mutate-reactive-state-in-updated-hook ------------------------------------------

fn hook_functions<'t>(root: Node<'t>, m: &SemanticModel, src: &str) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    for x in all_nodes(root) {
        let f = match x.kind() {
            "call_expression" if x.child_by_field_name("function").is_some_and(|f| imported(m, f, "vue", "onUpdated")) => arguments(x).first().copied(),
            "pair" if x.child_by_field_name("key").is_some_and(|k| crate::util::unquote(text(k, src)) == "updated") => x.child_by_field_name("value").filter(|_| component_object(x.parent(), m)),
            "method_definition" if x.parent().is_some_and(|o| o.kind() == "object") && x.child_by_field_name("name").is_some_and(|k| text(k, src) == "updated") && component_object(x.parent(), m) => Some(x),
            _ => None,
        };
        if let Some(f) = f {
            if let Some(func) = js_ast::as_function(f).or_else(|| bound_function(f, root, m)) {
                out.push(func);
            } else if f.kind() == "method_definition" {
                out.push(f);
            }
        }
    }
    out
}

fn component_object(obj: Option<Node<'_>>, m: &SemanticModel) -> bool {
    let Some(o) = obj.filter(|o| o.kind() == "object") else { return false };
    let p = o.parent();
    p.is_some_and(|p| p.kind() == "export_statement") || p.filter(|p| p.kind() == "arguments").and_then(|a| a.parent()).is_some_and(|c| c.child_by_field_name("function").is_some_and(|f| imported(m, f, "vue", "defineComponent")))
}

srule!(NoMutateReactiveStateInUpdatedHook, "js/no-mutate-reactive-state-in-updated-hook", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    for f in hook_functions(n, m, src) {
        let Some(body) = js_ast::body(f) else { continue };
        for x in own_nodes(body) {
            let (target, compound) = match x.kind() {
                "update_expression" => (x.child_by_field_name("argument"), true),
                "augmented_assignment_expression" => (x.child_by_field_name("left"), true),
                "assignment_expression" => (x.child_by_field_name("left"), false),
                _ => continue,
            };
            let Some(t) = target.filter(|t| t.kind() == "member_expression") else { continue };
            let (Some(obj), Some(prop)) = (t.child_by_field_name("object").filter(|o| o.kind() == "identifier"), t.child_by_field_name("property")) else { continue };
            let Some(init) = js_ast::initial_value(m, n, obj).map(unparen).filter(|v| v.kind() == "call_expression") else { continue };
            let ctor = init.child_by_field_name("function");
            let reactive = ctor.is_some_and(|c| imported(m, c, "vue", "reactive"));
            let is_ref = ctor.is_some_and(|c| imported(m, c, "vue", "ref")) && text(prop, src) == "value";
            if !reactive && !is_ref {
                continue;
            }
            // Guarded by a condition, or after an early exit.
            let mut child = x;
            let mut guarded = false;
            for a in ancestors(x) {
                if a.id() == f.id() {
                    break;
                }
                match a.kind() {
                    "if_statement" | "ternary_expression" if !a.child_by_field_name("condition").is_some_and(|c| in_range(x.start_byte(), c)) => guarded = true,
                    "binary_expression" if !a.child_by_field_name("left").is_some_and(|c| in_range(x.start_byte(), c)) && a.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "&&" | "||" | "??")) => guarded = true,
                    "statement_block" if named_children(a).into_iter().take_while(|s| s.start_byte() < child.start_byte()).any(|s| s.kind() == "if_statement" && s.child_by_field_name("alternative").is_none() && s.child_by_field_name("consequence").is_some_and(|c| crate::util::exits(&c))) => guarded = true,
                    _ => {}
                }
                child = a;
            }
            if guarded {
                continue;
            }
            let red = compound
                || x.child_by_field_name("right").is_some_and(|r| {
                    own_nodes(r).into_iter().chain([r]).any(|y| {
                        matches!(y.kind(), "update_expression" | "augmented_assignment_expression")
                            || (y.kind() == "member_expression" && squash(text(y, src)) == squash(text(t, src)))
                            || (y.kind() == "call_expression" && y.child_by_field_name("function").is_some_and(|c| matches!(text(c, src), "Date.now" | "Math.random" | "performance.now")))
                    })
                });
            if red {
                out.push(finding(x, format!("Changing `{}` in an `updated` hook re-renders the component, which runs the hook again: an endless loop", text(t, src)), vec![]));
            }
        }
    }
});

// ---- S3699 no-use-of-empty-return-value ------------------------------------------------------

/// Whether a call's value is used: not a statement, an arrow's body, an
/// operand of a unary, `await`, `return` or `throw`, the right side of a
/// logical operator, a branch of `?:`, or an earlier item of a sequence.
fn value_used(call: Node<'_>, src: &str) -> bool {
    let mut x = call;
    loop {
        let Some(p) = x.parent() else { return false };
        match p.kind() {
            "expression_statement" | "unary_expression" | "await_expression" | "return_statement" | "throw_statement" | "arrow_function" => return false,
            "parenthesized_expression" => x = p,
            "binary_expression" if p.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "&&" | "||" | "??")) => return same(p.child_by_field_name("left"), x),
            "ternary_expression" => return same(p.child_by_field_name("condition"), x),
            "sequence_expression" => {
                if !named_children(p).last().is_some_and(|l| l.id() == x.id()) {
                    return false;
                }
                x = p;
            }
            _ => return true,
        }
    }
}

srule!(NoUseOfEmptyReturnValue, "js/no-use-of-empty-return-value", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(callee) = n.child_by_field_name("function").filter(|c| c.kind() == "identifier") else { return };
    let root = js_ast::root_of(n);
    let Some(f) = bound_function(callee, root, m) else { return };
    if js_ast::is_async(f) || matches!(f.kind(), "generator_function" | "generator_function_declaration") {
        return;
    }
    let Some(body) = js_ast::body(f).filter(|b| b.kind() == "statement_block") else { return };
    if named_children(body).iter().all(|c| c.kind() == "comment") || own_nodes(body).iter().any(|x| x.kind() == "return_statement" && named_children(*x).iter().any(|c| c.kind() != "comment")) {
        return;
    }
    if !value_used(n, src) {
        return;
    }
    let name = text(callee, src);
    out.push(finding(callee, format!("`{name}` returns nothing, so this uses `undefined`: don't use the result of `{name}()`"), vec![]));
});

// ---- S1526 no-variable-usage-before-declaration ------------------------------------------------

srule!(NoVariableUsageBeforeDeclaration, "js/no-variable-usage-before-declaration", &["program"], |n, _ctx, m, out| {
    for (i, s) in m.symbols.iter().enumerate() {
        if s.kind != SymbolKind::Var {
            continue;
        }
        let Some(&(d, _)) = s.decls.first() else { continue };
        if let Some(r) = refs_of(m, i).into_iter().filter(|r| r.start < d).min_by_key(|r| r.start)
            && let Some(id) = node_at(n, r.start, r.end)
        {
            out.push(finding(id, format!("`{}` is used before its `var` declaration: move the declaration above this line", s.name), vec![]));
        }
    }
});

// ---- S6351 stateful-regex -----------------------------------------------------------------------

fn regex_flags(n: Node<'_>, m: &SemanticModel, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "regex" => Some(n.child_by_field_name("flags").map_or(String::new(), |f| text(f, src).to_owned())),
        "new_expression" | "call_expression" => {
            let c = n.child_by_field_name("constructor").or_else(|| n.child_by_field_name("function"))?;
            if text(c, src) != "RegExp" || m.symbol_at(c.start_byte()).is_some() {
                return None;
            }
            let args = arguments(n);
            match args.get(1) {
                Some(f) if f.kind() == "string" => Some(crate::util::unquote(text(*f, src)).to_owned()),
                Some(_) => None,
                None => Some(String::new()),
            }
        }
        _ => None,
    }
}

srule!(StatefulRegex, "js/stateful-regex", &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    for x in all_nodes(n) {
        // (a) `g` with `y`.
        if matches!(x.kind(), "regex" | "new_expression" | "call_expression") && regex_flags(x, m, src).is_some_and(|f| f.contains('g') && f.contains('y')) {
            out.push(finding(x, "With both `g` and `y` the `g` flag is ignored: drop one of them", vec![]));
        }
        // (b) `/x/g.exec(…)` in a loop condition makes a new regex every pass.
        if x.kind() == "call_expression"
            && let Some(f) = x.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && f.child_by_field_name("property").is_some_and(|p| text(p, src) == "exec"))
            && let Some(o) = f.child_by_field_name("object")
            && regex_flags(o, m, src).is_some_and(|fl| fl.contains('g'))
        {
            let mut child = x;
            for a in ancestors(x) {
                if fn_like(a.kind()) {
                    break;
                }
                if matches!(a.kind(), "while_statement" | "do_statement") && same(a.child_by_field_name("condition"), child) {
                    out.push(finding(o, "This regular expression is created again on every pass, so `exec()` starts over each time and the loop never ends: create it once, before the loop", vec![]));
                    break;
                }
                child = a;
            }
        }
    }
    // (c) one global regex tested against different inputs.
    for (i, s) in m.symbols.iter().enumerate() {
        if s.decls.len() != 1 || !matches!(s.kind, SymbolKind::Const | SymbolKind::Let | SymbolKind::Var) {
            continue;
        }
        let Some(decl) = node_at(n, s.decls[0].0, s.decls[0].1) else { continue };
        let Some(init) = js_ast::initial_value(m, n, decl) else { continue };
        if !regex_flags(init, m, src).is_some_and(|f| f.contains('g')) {
            continue;
        }
        let refs = refs_of(m, i);
        if refs.iter().any(|r| r.write) {
            continue;
        }
        let mut inputs = BTreeSet::new();
        let mut reset = false;
        for r in &refs {
            let Some(id) = node_at(n, r.start, r.end) else { continue };
            let Some(p) = id.parent().filter(|p| p.kind() == "member_expression" && same(p.child_by_field_name("object"), id)) else { continue };
            let prop = p.child_by_field_name("property").map_or("", |q| text(q, src));
            if prop == "lastIndex" && p.parent().is_some_and(|a| a.kind() == "assignment_expression" && same(a.child_by_field_name("left"), p)) {
                reset = true;
            }
            if matches!(prop, "exec" | "test")
                && let Some(c) = p.parent().filter(|c| c.kind() == "call_expression")
                && let [arg] = arguments(c)[..]
            {
                inputs.insert(text(arg, src).to_owned());
            }
        }
        if inputs.len() > 1 && !reset && !inputs.iter().any(|t| t == "''" || t == "\"\"") {
            out.push(finding(init, format!("`{}` has the `g` flag, so each `test()`/`exec()` starts where the last one stopped: on a different input that skips matches; reset `lastIndex` or drop `g`", s.name), vec![]));
        }
    }
});


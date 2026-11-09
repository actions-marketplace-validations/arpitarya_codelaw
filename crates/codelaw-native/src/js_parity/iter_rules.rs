//! W-139 strand W-139-js, the iterator rows: eslint-plugin-unicorn v79.0.0's
//! `no-unused-iterator-helper`, `no-useless-iterator-to-array`,
//! `prefer-iterator-to-array`, `prefer-iterator-to-array-at-end`,
//! `prefer-iterator-helpers` and `prefer-iterator-concat`, read from the
//! rule's source and tests and written again on the syntax questions of
//! [`super::values`] (what a value is known to be from its constructor,
//! annotation or `const` alias — no type program).

use super::values::{Probe, arg_count, bare, has_comment, has_side_effect, has_spread, is_global, is_optional, member_object_text, method, static_call, strip};
use super::{JS, finding};
use crate::js_ast::{self, arguments};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule, as_suggestion};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

macro_rules! irule {
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

const TYPED_ARRAYS: &[&str] = &["Int8Array", "Uint8Array", "Uint8ClampedArray", "Int16Array", "Uint16Array", "Int32Array", "Uint32Array", "Float16Array", "Float32Array", "Float64Array", "BigInt64Array", "BigUint64Array"];
const COLLECTIONS: &[&str] = &["Map", "WeakMap", "Set", "WeakSet"];
const PROMISE_METHODS: &[&str] = &["all", "allSettled", "any", "race"];

/// The parenthesized expression a node is held by, or the node.
fn first_named(n: Node<'_>) -> Option<Node<'_>> {
    named_children(n).into_iter().find(|c| c.kind() != "comment")
}

fn outer(n: Node<'_>) -> Node<'_> {
    let mut o = n;
    while let Some(p) = o.parent().filter(|p| p.kind() == "parenthesized_expression") {
        o = p;
    }
    o
}

fn is_first_arg(call: Node<'_>, n: Node<'_>) -> bool {
    arguments(call).first().is_some_and(|a| a.id() == n.id())
}

/// `Name.method(…)` with `Name` one of `objects`: the object's name.
fn static_of<'s>(p: &Probe<'s, '_>, call: Node<'_>, objects: &[&str], methods: &[&str]) -> Option<String> {
    for o in objects {
        if let Some(m) = static_call(p, call, o)
            && methods.contains(&m)
        {
            return Some((*o).to_owned());
        }
    }
    None
}

fn new_of<'t>(p: &Probe<'_, 't>, n: Node<'t>, names: &[&str]) -> Option<String> {
    super::values::new_global(p, n, names).filter(|(x, _)| x.id() == n.id()).map(|(_, name)| name)
}

/// Whether the expression `n` (not a call's own parenthesized callee) sits
/// where any iterable is accepted: `for…of`, `yield*`, a spread in an array
/// or an argument list, the first argument of a collection or typed-array
/// constructor, `Promise.all…`, `Array.from`, `Object.fromEntries`.
fn iterable_accepting(p: &Probe<'_, '_>, n: Node<'_>) -> bool {
    let o = outer(n);
    let Some(par) = o.parent() else { return false };
    match par.kind() {
        "for_in_statement" => par.child_by_field_name("right").is_some_and(|r| r.id() == o.id()) && p.src[par.child_by_field_name("left").map_or(0, |l| l.end_byte())..o.start_byte()].contains("of"),
        "yield_expression" => children(par).iter().any(|c| c.kind() == "*"),
        "spread_element" => par.parent().is_some_and(|g| g.kind() == "array" || (g.kind() == "arguments" && g.parent().is_some_and(|c| matches!(c.kind(), "call_expression" | "new_expression")))),
        "arguments" => {
            let Some(call) = par.parent() else { return false };
            if !is_first_arg(call, o) {
                return false;
            }
            if call.kind() == "new_expression" {
                return (new_of(p, call, COLLECTIONS).is_some() && arg_count(call) == 1) || new_of(p, call, TYPED_ARRAYS).is_some();
            }
            let mut objs: Vec<&str> = vec!["Array"];
            objs.extend_from_slice(TYPED_ARRAYS);
            static_of(p, call, &["Promise"], PROMISE_METHODS).is_some() && arg_count(call) == 1 || static_of(p, call, &objs, &["from"]).is_some() || static_of(p, call, &["Object"], &["fromEntries"]).is_some()
        }
        _ => false,
    }
}

/// A receiver that is known not to be an `Iterator`: a literal, a function,
/// a class or an instance of a local class, or a `const` bound to one, or
/// to a call that returns an array or a string.
fn known_non_iterator(p: &Probe<'_, '_>, n: Node<'_>) -> bool {
    fn value(p: &Probe<'_, '_>, n: Node<'_>) -> bool {
        let n = bare(n);
        match n.kind() {
            "object" | "array" | "number" | "string" | "template_string" | "true" | "false" | "null" | "regex" | "function_expression" | "function" | "arrow_function" | "class" | "generator_function" => true,
            "new_expression" => n.child_by_field_name("constructor").is_some_and(|c| c.kind() == "identifier" && p.m.symbol_at(c.start_byte()).is_some_and(|s| p.m.symbol(s).kind == codelaw_semantic::SymbolKind::Class)),
            _ => false,
        }
    }
    if value(p, n) {
        return true;
    }
    let n = bare(n);
    if n.kind() != "identifier" {
        return false;
    }
    let Some(sym) = p.m.symbol_at(n.start_byte()) else { return false };
    let s = p.m.symbol(sym);
    if s.decls.len() != 1 || s.kind != codelaw_semantic::SymbolKind::Const {
        return false;
    }
    let (a, b) = s.decls[0];
    let Some(id) = p.root.descendant_for_byte_range(a, b) else { return false };
    let Some(init) = super::values::const_init(id, p.src) else { return false };
    value(p, init) || method(p.src, init, &["toArray", "slice", "concat", "split"]).is_some()
}

fn to_array_call<'t>(p: &Probe<'_, 't>, n: Node<'t>) -> Option<Node<'t>> {
    let (recv, _) = method(p.src, n, &["toArray"])?;
    (arg_count(n) == 0 && !known_non_iterator(p, recv)).then_some(recv)
}

fn remove_to_array(call: Node<'_>) -> Option<TextEdit> {
    let f = call.child_by_field_name("function")?;
    let recv = f.child_by_field_name("object")?;
    Some(TextEdit::delete(recv.end_byte(), call.end_byte()))
}

fn to_array_prop(call: Node<'_>) -> Node<'_> {
    call.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")).unwrap_or(call)
}

/// Whether a callback can read the `array` argument an `Array` method
/// passes and an `Iterator` helper does not.
fn can_observe_array(p: &Probe<'_, '_>, cb: Option<Node<'_>>, index: usize, resolve: bool) -> bool {
    let Some(cb) = cb else { return false };
    let mut f = bare(cb);
    if resolve
        && f.kind() == "identifier"
        && let Some(sym) = p.m.symbol_at(f.start_byte())
        && let s = p.m.symbol(sym)
        && s.decls.len() == 1
        && let Some(id) = p.root.descendant_for_byte_range(s.decls[0].0, s.decls[0].1)
        && let Some(par) = id.parent()
    {
        if par.kind() == "function_declaration" {
            f = par;
        } else if let Some(init) = super::values::const_init(id, p.src) {
            f = bare(init);
        }
    }
    if !matches!(f.kind(), "arrow_function" | "function_expression" | "function" | "function_declaration" | "generator_function") {
        return true;
    }
    let ps = js_ast::params(f);
    ps.len() > index || ps.last().is_some_and(|l| l.kind() == "rest_pattern" || l.child_by_field_name("pattern").is_some_and(|x| x.kind() == "rest_pattern")) || (f.kind() != "arrow_function" && mentions_arguments(f, p.src))
}

fn mentions_arguments(f: Node<'_>, src: &str) -> bool {
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

// ---- unicorn: no-unused-iterator-helper ------------------------------------

irule!(NoUnusedIteratorHelper, "js/no-unused-iterator-helper", &["call_expression"], |n, _ctx, p, out| {
    if !p.is_lazy_helper_call(n) {
        return;
    }
    let mut node = n;
    let mut wrapped = false;
    while let Some(par) = node.parent().filter(|x| matches!(x.kind(), "parenthesized_expression" | "await_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion")) {
        wrapped |= matches!(par.kind(), "as_expression" | "satisfies_expression" | "non_null_expression" | "type_assertion");
        node = par;
    }
    let Some(par) = node.parent() else { return };
    let discarded = match par.kind() {
        "expression_statement" => true,
        "unary_expression" => par.child(0).is_some_and(|c| c.kind() == "void"),
        "for_statement" => par.child_by_field_name("initializer").is_some_and(|i| i.id() == node.id()) || par.child_by_field_name("increment").is_some_and(|i| i.id() == node.id()),
        _ => false,
    };
    if !discarded {
        return;
    }
    let Some(prop) = n.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")) else { return };
    let name = text(prop, p.src);
    let mut edits = Vec::new();
    if name == "map" && arg_count(n) == 1 && !has_spread(n) && n.child_by_field_name("type_arguments").is_none() && !wrapped {
        edits.push(TextEdit::new(prop.start_byte(), prop.end_byte(), "forEach"));
    }
    out.push(finding(prop, format!("`.{name}(…)` is lazy: nothing reads this iterator, so its callback never runs"), edits));
});

// ---- unicorn: no-useless-iterator-to-array ---------------------------------

irule!(NoUselessIteratorToArray, "js/no-useless-iterator-to-array", &["new_expression", "call_expression", "for_in_statement", "yield_expression", "spread_element"], |n, _ctx, p, out| {
    let push = |out: &mut Vec<NativeFinding>, call: Node<'_>, what: String, fix: bool| {
        let edits = remove_to_array(call).map(|e| vec![e]).unwrap_or_default();
        let edits = if has_comment(call) { vec![] } else { edits };
        let edits = if fix { edits } else { as_suggestion(edits) };
        out.push(finding(to_array_prop(call), what, edits));
    };
    match n.kind() {
        "new_expression" => {
            let args = arguments(n);
            let Some(first) = args.first().copied() else { return };
            let ctor = new_of(&p, n, COLLECTIONS).filter(|_| args.len() == 1).or_else(|| new_of(&p, n, TYPED_ARRAYS));
            let Some(ctor) = ctor else { return };
            if let Some(_recv) = to_array_call(&p, first) {
                push(out, first, format!("`new {ctor}(…)` takes an iterable: `.toArray()` first builds an array for nothing"), args.len() == 1);
            }
        }
        "call_expression" => {
            let args = arguments(n);
            let Some(first) = args.first().copied() else { return };
            let mut objs: Vec<&str> = vec!["Array"];
            objs.extend_from_slice(TYPED_ARRAYS);
            if (static_of(&p, n, &objs, &["from"]).is_some() || static_of(&p, n, &["Object"], &["fromEntries"]).is_some()) && to_array_call(&p, first).is_some() {
                let who = n.child_by_field_name("function").map_or("", |f| text(f, p.src)).to_owned();
                push(out, first, format!("`{who}(…)` takes an iterable: `.toArray()` first builds an array for nothing"), args.len() == 1);
                return;
            }
            if static_of(&p, n, &["Promise"], PROMISE_METHODS).is_some() && args.len() == 1 && to_array_call(&p, first).is_some() {
                let who = n.child_by_field_name("function").map_or("", |f| text(f, p.src)).to_owned();
                push(out, first, format!("`{who}(…)` takes an iterable: `.toArray()` first builds an array for nothing"), false);
                return;
            }
            let terminal = method(p.src, n, &["every", "find", "forEach", "some"]).filter(|_| args.len() <= 1).map(|(r, m)| (r, m, 2)).or_else(|| method(p.src, n, &["reduce"]).filter(|_| (1..=2).contains(&args.len())).map(|(r, m)| (r, m, 3)));
            let Some((recv, name, idx)) = terminal else { return };
            if to_array_call(&p, recv).is_none() || can_observe_array(&p, args.first().copied(), idx, false) {
                return;
            }
            push(out, recv, format!("`Iterator` has `.{name}()`: `.toArray()` first builds an array for nothing"), false);
        }
        "for_in_statement" => {
            let (Some(left), Some(right)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return };
            if !p.src[left.end_byte()..right.start_byte()].contains("of") {
                return;
            }
            if to_array_call(&p, right).is_some() {
                push(out, right, "`for…of` iterates any iterable: `.toArray()` first reads the whole iterator".to_owned(), false);
            }
        }
        "yield_expression" => {
            if !children(n).iter().any(|c| c.kind() == "*") {
                return;
            }
            if let Some(arg) = named_children(n).into_iter().find(|c| c.kind() != "comment")
                && to_array_call(&p, arg).is_some()
            {
                push(out, arg, "`yield*` delegates to any iterable: `.toArray()` first reads the whole iterator".to_owned(), false);
            }
        }
        _ => {
            // spread_element
            let Some(par) = n.parent() else { return };
            let ok = par.kind() == "array" || (par.kind() == "arguments" && par.parent().is_some_and(|c| matches!(c.kind(), "call_expression" | "new_expression")));
            let Some(arg) = first_named(n) else { return };
            if ok && to_array_call(&p, arg).is_some() {
                push(out, arg, "Spread takes any iterable: `.toArray()` first builds an array for nothing".to_owned(), true);
            }
        }
    }
});

// ---- unicorn: prefer-iterator-to-array -------------------------------------

/// `Some(true)` when the spread argument is certainly an iterator
/// (`Iterator.from`, `Iterator.concat`, or a helper chain on one), `Some(false)`
/// when it is only named like one (`.values()`).
fn iterator_fix_type<'t>(p: &Probe<'_, 't>, n: Node<'t>) -> Option<bool> {
    let n = bare(n);
    if n.kind() != "call_expression" || is_optional(n) {
        return None;
    }
    if let Some(f) = n.child_by_field_name("function")
        && f.kind() == "member_expression"
        && !is_optional(f)
        && f.child_by_field_name("object").is_some_and(|o| is_global(p.m, o, "Iterator", p.src))
        && let Some(pr) = f.child_by_field_name("property")
    {
        match text(pr, p.src) {
            "from" if arg_count(n) == 1 => return Some(true),
            "concat" => return Some(true),
            _ => {}
        }
    }
    if method(p.src, n, &["entries", "keys", "values"]).is_some() && arg_count(n) == 0 {
        return Some(false);
    }
    if method(p.src, n, &["matchAll"]).is_some() && arg_count(n) == 1 {
        return Some(false);
    }
    let (recv, _) = method(p.src, n, &["drop", "filter", "flatMap", "map", "take"])?;
    if arg_count(n) != 1 {
        return None;
    }
    iterator_fix_type(p, recv)
}

irule!(PreferIteratorToArray, "js/prefer-iterator-to-array", &["array"], |n, ctx, p, out| {
    let els: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    let [spread] = els[..] else { return };
    if spread.kind() != "spread_element" || iterable_accepting(&p, n) {
        return;
    }
    let Some(arg) = first_named(spread) else { return };
    let Some(certain) = iterator_fix_type(&p, arg) else { return };
    let mut edits = Vec::new();
    let mut rep = format!("{}.toArray()", member_object_text(arg, ctx.source));
    let before = ctx.source[..n.start_byte()].chars().next_back();
    let after = ctx.source[n.end_byte()..].chars().next();
    if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
        rep.insert(0, ' ');
    }
    if after.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
        rep.push(' ');
    }
    let in_new_callee = n.parent().is_some_and(|x| x.kind() == "new_expression" && x.child_by_field_name("constructor").is_some_and(|c| c.id() == n.id()));
    let at_statement_start = n.parent().is_some_and(|x| x.kind() == "expression_statement");
    if !has_comment(n) && !in_new_callee && !at_statement_start {
        edits.push(TextEdit::new(n.start_byte(), n.end_byte(), rep));
    }
    let edits = if certain { edits } else { as_suggestion(edits) };
    out.push(finding(n, "A temporary array from one iterator spread: `.toArray()` says it and keeps a helper chain readable", edits));
});

// ---- unicorn: prefer-iterator-to-array-at-end -------------------------------

irule!(PreferIteratorToArrayAtEnd, "js/prefer-iterator-to-array-at-end", &["call_expression"], |n, ctx, p, out| {
    let Some((recv, name)) = method(ctx.source, n, &["filter", "flatMap", "map"]) else { return };
    let args = arguments(n);
    if args.len() != 1 {
        return;
    }
    let Some(inner) = to_array_call(&p, recv) else { return };
    // The callback must be written here and unable to read the `array` argument.
    let f = bare(args[0]);
    if !matches!(f.kind(), "arrow_function" | "function_expression" | "function") || can_observe_array(&p, Some(args[0]), 2, false) {
        return;
    }
    let mut edits = Vec::new();
    if !has_comment(n) {
        let tail = &ctx.source[recv.end_byte()..n.end_byte()];
        edits.push(TextEdit::new(n.start_byte(), n.end_byte(), format!("{}{tail}.toArray()", member_object_text(inner, ctx.source))));
    }
    out.push(finding(to_array_prop(recv), format!("Move `.toArray()` after `.{name}(…)`: the helper is lazy and the array is built once, at the end"), as_suggestion(edits)));
});

// ---- unicorn: prefer-iterator-helpers --------------------------------------

/// The iterator a temporary array holds: `[...it]` or `Array.from(it)`, with
/// iterator evidence.
fn temp_array_iterator<'t>(p: &Probe<'_, 't>, n: Node<'t>) -> Option<Node<'t>> {
    let n = bare(n);
    match n.kind() {
        "array" => {
            let els: Vec<Node<'t>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
            let [spread] = els[..] else { return None };
            let arg = (spread.kind() == "spread_element").then(|| first_named(spread)).flatten()?;
            p.is_iterator_loose(arg).then_some(arg)
        }
        "call_expression" => {
            if n.child_by_field_name("type_arguments").is_some() || static_call(p, n, "Array") != Some("from") || arg_count(n) != 1 {
                return None;
            }
            let arg = arguments(n)[0];
            p.is_iterator_loose(arg).then_some(arg)
        }
        _ => None,
    }
}

fn static_nonneg(n: Node<'_>, src: &str) -> Option<u64> {
    let n = bare(n);
    (n.kind() == "number").then(|| text(n, src).replace('_', "").parse::<u64>().ok()).flatten().filter(|v| *v <= 9_007_199_254_740_991)
}

irule!(PreferIteratorHelpers, "js/prefer-iterator-helpers", &["call_expression"], |n, ctx, p, out| {
    let src = ctx.source;
    if n.child_by_field_name("type_arguments").is_some() {
        return;
    }
    let args = arguments(n);
    // slice
    if let Some((recv, _)) = method(src, n, &["slice"])
        && (1..=2).contains(&args.len())
    {
        let iterator = method(src, bare(recv), &["toArray"]).filter(|_| arg_count(bare(recv)) == 0 && bare(recv).child_by_field_name("type_arguments").is_none() && !is_optional(bare(recv))).map(|(r, _)| r).or_else(|| temp_array_iterator(&p, recv));
        let Some(iterator) = iterator else { return };
        if iterator.kind() == "super" {
            return;
        }
        let bounds: Vec<Option<u64>> = args.iter().map(|a| static_nonneg(*a, src)).collect();
        if bounds.iter().any(Option::is_none) {
            return;
        }
        let start = bounds[0].unwrap_or(0);
        let end = bounds.get(1).copied().flatten();
        let calls = match end {
            Some(e) if e <= start => "take(0)".to_owned(),
            _ => {
                let mut c = Vec::new();
                if start > 0 {
                    c.push(format!("drop({start})"));
                }
                if let Some(e) = end {
                    c.push(format!("take({})", e - start));
                }
                if c.is_empty() {
                    return;
                }
                c.join(".")
            }
        };
        let prop = n.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")).unwrap_or(n);
        let mut edits = Vec::new();
        if !has_comment(n) {
            edits.push(TextEdit::new(n.start_byte(), n.end_byte(), format!("{}.{calls}.toArray()", member_object_text(iterator, src))));
        }
        out.push(finding(prop, format!("Slice the iterator before it becomes an array: `.{calls}` before `.toArray()`"), as_suggestion(edits)));
        return;
    }
    // terminal
    let terminal = method(src, n, &["every", "find", "forEach", "some"]).filter(|_| args.len() == 1).map(|(r, m)| (r, m, 2)).or_else(|| method(src, n, &["reduce"]).filter(|_| (1..=2).contains(&args.len())).map(|(r, m)| (r, m, 3)));
    let Some((recv, name, idx)) = terminal else { return };
    if can_observe_array(&p, args.first().copied(), idx, true) {
        return;
    }
    let Some(iterator) = temp_array_iterator(&p, recv) else { return };
    let mut edits = Vec::new();
    let r = bare(recv);
    if !has_comment(r) {
        let text_ = member_object_text(iterator, src);
        edits.push(TextEdit::new(recv.start_byte(), recv.end_byte(), text_));
    }
    out.push(finding(r, format!("Call `{name}()` on the iterator itself instead of building an array first"), as_suggestion(edits)));
});

// ---- unicorn: prefer-iterator-concat ---------------------------------------

irule!(PreferIteratorConcat, "js/prefer-iterator-concat", &["array"], |n, ctx, p, out| {
    let els: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    if els.len() < 2 || !els.iter().all(|e| e.kind() == "spread_element") || !iterable_accepting_exact(&p, n) {
        return;
    }
    let args: Vec<Node<'_>> = els.iter().filter_map(|e| first_named(*e)).collect();
    if args.len() != els.len() || args.iter().any(|a| to_array_call(&p, *a).is_some() || is_stringish(&p, *a)) {
        return;
    }
    // new Set([...a, ...b]) of known Sets is a union the engine already handles.
    let o = outer(n);
    if o.parent().is_some_and(|x| x.kind() == "arguments")
        && let Some(call) = o.parent().and_then(|a| a.parent())
        && new_of(&p, call, &["Set"]).is_some()
        && args.iter().all(|a| p.is_set(*a))
    {
        return;
    }
    let value_storing = o.parent().is_some_and(|x| x.kind() == "arguments") && o.parent().and_then(|a| a.parent()).is_some_and(|call| {
        arg_count(call) == 1 && (new_of(&p, call, COLLECTIONS).is_some() || new_of(&p, call, TYPED_ARRAYS).is_some() || {
            let mut objs: Vec<&str> = vec!["Array"];
            objs.extend_from_slice(TYPED_ARRAYS);
            static_of(&p, call, &objs, &["from"]).is_some() || static_of(&p, call, &["Object"], &["fromEntries"]).is_some()
        })
    });
    let certain = value_storing && args.iter().skip(1).all(|a| quiet(*a));
    let mut edits = Vec::new();
    if !has_comment(n) {
        let list: Vec<&str> = args.iter().map(|a| text(*a, ctx.source)).collect();
        let mut rep = format!("Iterator.concat({})", list.join(", "));
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
    let edits = if certain { edits } else { as_suggestion(edits) };
    out.push(finding(n, "A temporary array made only of spreads: `Iterator.concat(…)` joins the iterables without building it", edits));
});

/// `iterable_accepting` for concat: the same contexts, but a spread is not
/// one (the array would be spread, not consumed once).
fn iterable_accepting_exact(p: &Probe<'_, '_>, n: Node<'_>) -> bool {
    let o = outer(n);
    if o.parent().is_some_and(|x| x.kind() == "spread_element") {
        return false;
    }
    iterable_accepting(p, n)
}

/// A value that is a string (spreading one works, `Iterator.concat` of one throws).
fn is_stringish(p: &Probe<'_, '_>, n: Node<'_>) -> bool {
    let n = strip(n);
    match n.kind() {
        "string" | "template_string" => true,
        "call_expression" => n.child_by_field_name("function").is_some_and(|f| is_global(p.m, f, "String", p.src)),
        "binary_expression" => n.child_by_field_name("operator").is_some_and(|o| text(o, p.src) == "+") && n.child_by_field_name("left").zip(n.child_by_field_name("right")).is_some_and(|(l, r)| is_stringish(p, l) || is_stringish(p, r)),
        _ => false,
    }
}

/// An expression that reads nothing that could run code: names and literals.
fn quiet(n: Node<'_>) -> bool {
    match strip(n).kind() {
        "identifier" | "this" | "string" | "number" | "true" | "false" | "null" | "undefined" => true,
        "array" | "object" | "template_string" | "parenthesized_expression" => !has_side_effect(n) && named_children(n).iter().all(|c| quiet(*c) || c.kind() == "comment"),
        _ => false,
    }
}

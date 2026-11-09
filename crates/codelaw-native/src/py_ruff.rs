//! W-139 strand W-139-ruff, the natives that read a function's paths, a
//! class's slots or a module's unused private names: RET503, FURB113,
//! PYI018, PYI046, PYI047, PYI049, PYI036, PLE0237, PLW0244, I002, ICN002
//! and W505 — each Ruff 0.16.9's behaviour (MIT; read from its source over
//! the network, research only, L5), written fresh.

use crate::python_util::{call_args, decorators, hit, kids, qualified_name, remove_stmt, statements, string_parts, unparen, PY};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::SemanticModel;
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

macro_rules! rule {
    ($name:ident, $id:literal, $kinds:expr, $sem:literal, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn needs_semantic(&self) -> bool {
                $sem
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

fn span(start: usize, end: usize, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

fn root_of(n: Node<'_>) -> Node<'_> {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        cur = p;
    }
    cur
}

/// The expression under a (possibly nested) `type` node.
fn ann_expr(n: Node<'_>) -> Node<'_> {
    let mut cur = n;
    while cur.kind() == "type" {
        match cur.named_child(0) {
            Some(c) => cur = c,
            None => break,
        }
    }
    cur
}

fn is_none(n: Node<'_>, src: &str) -> bool {
    let n = ann_expr(n);
    n.kind() == "none" || text(n, src) == "None"
}

fn qname(n: Node<'_>, ctx: &NativeContext<'_>) -> Option<String> {
    qualified_name(ann_expr(n), ctx.semantic?, ctx.source)
}

fn is_typing(q: &str, name: &str) -> bool {
    q == format!("typing.{name}") || q == format!("typing_extensions.{name}")
}

/// The head of an annotation or expression that may be subscripted.
fn subscript_head(n: Node<'_>) -> Node<'_> {
    let n = ann_expr(n);
    match n.kind() {
        "generic_type" => n.named_child(0).unwrap_or(n),
        "subscript" => field(n, "value").unwrap_or(n),
        _ => n,
    }
}

/// The type arguments of `X[a, b]` in annotation or expression form.
fn subscript_args(n: Node<'_>) -> Vec<Node<'_>> {
    let n = ann_expr(n);
    match n.kind() {
        "generic_type" => n
            .named_children(&mut n.walk())
            .filter(|c| c.kind() == "type_parameter")
            .flat_map(|tp| named_children(tp).into_iter().map(ann_expr).collect::<Vec<_>>())
            .collect(),
        "subscript" => {
            let mut c = n.walk();
            n.children_by_field_name("subscript", &mut c).map(ann_expr).flat_map(|s| if s.kind() == "tuple" { kids(s) } else { vec![s] }).collect()
        }
        _ => Vec::new(),
    }
}

fn is_overload(def: Node<'_>, src: &str) -> bool {
    decorators(def).into_iter().any(|d| {
        let t = text(d, src);
        t == "overload" || t.ends_with(".overload")
    })
}

// ------------------------------------------------------------------ RET503

fn const_true(n: Node<'_>, src: &str) -> bool {
    let n = unparen(n);
    match n.kind() {
        "true" => true,
        "integer" | "float" => text(n, src).trim_start_matches('0').trim_start_matches(['.', '_', 'x', 'o', 'b']).chars().any(|c| c != '0' && c != '_' && c != '.'),
        "string" => string_parts(n, src).is_some_and(|(_, c)| !c.is_empty()),
        _ => false,
    }
}

fn const_false(n: Node<'_>, src: &str) -> bool {
    let n = unparen(n);
    match n.kind() {
        "false" | "none" => true,
        "integer" | "float" => !const_true(n, src),
        "string" => string_parts(n, src).is_some_and(|(_, c)| c.is_empty()),
        _ => false,
    }
}

fn last_stmt<'t>(block: Option<Node<'t>>) -> Option<Node<'t>> {
    statements(block?).last().copied()
}

/// Whether control can leave the end of a function through `stmt`, the
/// last statement of its body (Ruff's `has_implicit_return`).
fn falls_through(stmt: Node<'_>, ctx: &NativeContext<'_>, noreturn_fns: &BTreeSet<String>) -> bool {
    let src = ctx.source;
    let rec = |b: Option<Node<'_>>| last_stmt(b).is_some_and(|l| falls_through(l, ctx, noreturn_fns));
    match stmt.kind() {
        "if_statement" => {
            if rec(field(stmt, "consequence")) {
                return true;
            }
            let mut c = stmt.walk();
            let alts: Vec<Node<'_>> = stmt.children_by_field_name("alternative", &mut c).collect();
            for a in &alts {
                let body = if a.kind() == "elif_clause" { field(*a, "consequence") } else { field(*a, "body") };
                if rec(body) {
                    return true;
                }
            }
            alts.last().is_none_or(|a| a.kind() == "elif_clause")
        }
        "assert_statement" => !stmt.named_child(0).is_some_and(|t| const_false(t, src)),
        "while_statement" => {
            if field(stmt, "condition").is_some_and(|c| const_true(c, src)) {
                return false;
            }
            match field(stmt, "alternative") {
                Some(e) => rec(field(e, "body")),
                None => true,
            }
        }
        "for_statement" => match field(stmt, "alternative") {
            Some(e) => rec(field(e, "body")),
            None => true,
        },
        "match_statement" => field(stmt, "body").is_some_and(|b| kids(b).into_iter().filter(|c| c.kind() == "case_clause").any(|c| rec(field(c, "consequence")))),
        "with_statement" => rec(field(stmt, "body")),
        "return_statement" | "raise_statement" | "try_statement" => false,
        "expression_statement" => {
            let Some(call) = stmt.named_child(0).filter(|c| c.kind() == "call") else { return true };
            let Some(func) = field(call, "function") else { return true };
            !is_noreturn(func, ctx, noreturn_fns)
        }
        _ => true,
    }
}

fn is_noreturn(func: Node<'_>, ctx: &NativeContext<'_>, local: &BTreeSet<String>) -> bool {
    if let Some(model) = ctx.semantic
        && let Some(q) = qualified_name(func, model, ctx.source)
    {
        let hit = matches!(
            q.as_str(),
            "builtins.exit" | "sys.exit" | "_thread.exit" | "pytest.exit" | "builtins.quit" | "os._exit" | "posix._exit" | "os.abort" | "posix.abort" | "_winapi.ExitProcess" | "pytest.fail" | "pytest.skip" | "pytest.xfail"
        ) || is_typing(&q, "assert_never");
        if hit {
            return true;
        }
    }
    func.kind() == "identifier" && local.contains(text(func, ctx.source))
}

/// Functions of the file annotated `-> NoReturn` or `-> Never`.
fn noreturn_functions(root: Node<'_>, src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "function_definition"
            && let (Some(name), Some(r)) = (field(n, "name"), field(n, "return_type"))
        {
            let t = text(ann_expr(r), src);
            if matches!(t.rsplit('.').next(), Some("NoReturn" | "Never")) {
                out.insert(text(name, src).to_owned());
            }
        }
        stack.extend(named_children(n));
    }
    out
}

struct Flow {
    returns: Vec<Option<bool>>,
    generator: bool,
}

/// `Some(true)` for a return of a value other than `None`.
fn walk_flow(n: Node<'_>, src: &str, out: &mut Flow) {
    for c in named_children(n) {
        match c.kind() {
            "function_definition" | "class_definition" | "lambda" => continue,
            "return_statement" => {
                let v = c.named_child(0);
                out.returns.push(v.map(|v| !(v.kind() == "none" || text(v, src) == "None")));
            }
            "yield" => out.generator = true,
            _ => {}
        }
        walk_flow(c, src, out);
    }
}

rule!(ImplicitReturn, "py/implicit-return", &["function_definition"], true, |n, ctx, out| {
    let src = ctx.source;
    let Some(body) = field(n, "body") else { return };
    let stmts = statements(body);
    let Some(last) = stmts.last().copied() else { return };
    if stmts.len() == 1 && last.kind() == "return_statement" {
        return;
    }
    let mut flow = Flow { returns: Vec::new(), generator: false };
    walk_flow(body, src, &mut flow);
    if flow.generator || flow.returns.is_empty() || !flow.returns.contains(&Some(true)) {
        return;
    }
    let noreturn = noreturn_functions(root_of(n), src);
    if !falls_through(last, ctx, &noreturn) {
        return;
    }
    // Insert `return None` below the last statement, at its indentation.
    let end = last.end_byte();
    let line_end = src[end..].find('\n').map_or(src.len(), |i| end + i);
    let rest = src[end..line_end].trim();
    let edits = if rest.is_empty() || rest.starts_with('#') {
        let ls = src[..last.start_byte()].rfind('\n').map_or(0, |i| i + 1);
        let indent: String = src[ls..].chars().take_while(|c| *c == ' ' || *c == '\t').collect();
        vec![TextEdit::new(line_end, line_end, format!("\n{indent}return None"))]
    } else {
        Vec::new()
    };
    out.push(span(n.start_byte(), n.end_byte(), "Missing explicit `return` at the end of function able to return non-`None` value", edits));
});

// ----------------------------------------------------------------- FURB113

/// Whether the symbol the name `recv` read at `at` resolves to was bound
/// once, to a list.
fn bound_to_list(model: &SemanticModel, root: Node<'_>, src: &str, recv: Node<'_>) -> Option<usize> {
    let scope = model.scope_at(recv.start_byte());
    let id = model.lookup(scope, text(recv, src), recv.start_byte())?;
    let sym = model.symbol(id);
    if sym.decls.len() != 1 {
        return None;
    }
    let (s, e) = sym.decls[0];
    let ident = root.descendant_for_byte_range(s, e)?;
    let parent = ident.parent()?;
    if parent.kind() != "assignment" || field(parent, "left") != Some(ident) {
        return None;
    }
    let right = unparen(field(parent, "right")?);
    let is_list = match right.kind() {
        "list" | "list_comprehension" => true,
        "call" => field(right, "function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "list" && model.symbol_at(f.start_byte()).is_none()),
        _ => false,
    };
    is_list.then_some(id)
}

struct Append<'t> {
    stmt: Node<'t>,
    recv: Node<'t>,
    sym: usize,
    arg: Node<'t>,
}

fn match_append<'t>(stmt: Node<'t>, ctx: &NativeContext<'_>, root: Node<'t>) -> Option<Append<'t>> {
    let src = ctx.source;
    if stmt.kind() != "expression_statement" {
        return None;
    }
    let call = stmt.named_child(0).filter(|c| c.kind() == "call" && stmt.named_child_count() == 1)?;
    let args = call_args(call, src);
    if args.positional.len() != 1 || !args.keywords.is_empty() || args.starred {
        return None;
    }
    let arg = args.positional[0];
    if matches!(arg.kind(), "generator_expression" | "list_splat" | "dictionary_splat" | "parenthesized_expression" if arg.kind() != "parenthesized_expression") {
        return None;
    }
    let func = field(call, "function").filter(|f| f.kind() == "attribute")?;
    if text(field(func, "attribute")?, src) != "append" {
        return None;
    }
    let recv = field(func, "object").filter(|o| o.kind() == "identifier")?;
    let sym = bound_to_list(ctx.semantic?, root, src, recv)?;
    Some(Append { stmt, recv, sym, arg })
}

rule!(RepeatedAppend, "py/repeated-append", &["module", "block"], true, |n, ctx, out| {
    let src = ctx.source;
    let root = root_of(n);
    let stmts = statements(n);
    let mut i = 0;
    while i < stmts.len() {
        let Some(first) = match_append(stmts[i], ctx, root) else {
            i += 1;
            continue;
        };
        // The run of appends starting here.
        let mut run = vec![first];
        let mut j = i + 1;
        while j < stmts.len() {
            match match_append(stmts[j], ctx, root) {
                Some(a) => run.push(a),
                None => break,
            }
            j += 1;
        }
        i = j.max(i + 1);
        let mut bys: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (k, a) in run.iter().enumerate() {
            bys.entry(a.sym).or_default().push(k);
        }
        for ks in bys.values().filter(|ks| ks.len() > 1) {
            let consecutive = ks.windows(2).all(|w| w[1] == w[0] + 1);
            let (a0, an) = (&run[ks[0]], &run[*ks.last().unwrap()]);
            let name = text(a0.recv, src);
            let elems: Vec<&str> = ks.iter().map(|k| text(run[*k].arg, src)).collect();
            let replacement = format!("{name}.extend(({}))", elems.join(", "));
            let (s, e) = (a0.stmt.start_byte(), an.stmt.end_byte());
            let has_comment = named_children(n).iter().any(|c| c.kind() == "comment" && c.start_byte() >= s && c.end_byte() <= e);
            let edits = if consecutive && !has_comment { vec![TextEdit::new(s, e, replacement.clone())] } else { Vec::new() };
            out.push(span(s, e, format!("Use `{replacement}` instead of repeatedly calling `{name}.append()`"), edits));
        }
    }
});

// ----------------------------------------------- PYI018, PYI046, PYI047, PYI049

fn is_private(name: &str) -> bool {
    name.starts_with('_') && !(name.starts_with("__") && name.ends_with("__")) && name.len() > 1
}

/// Whether `name` appears as a whole word inside any string of the file
/// (a forward reference).
fn named_in_strings(root: Node<'_>, src: &str, name: &str, skip: (usize, usize)) -> bool {
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "string" {
            if n.start_byte() >= skip.0 && n.end_byte() <= skip.1 {
                continue;
            }
            let t = text(n, src);
            let mut from = 0;
            while let Some(i) = t[from..].find(name) {
                let (a, b) = (from + i, from + i + name.len());
                let before = t[..a].chars().next_back().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
                let after = t[b..].chars().next().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
                if before && after {
                    return true;
                }
                from = b;
            }
            continue;
        }
        stack.extend(named_children(n));
    }
    false
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Private {
    TypeVar,
    Protocol,
    TypeAlias,
    TypedDict,
}

fn base_is(def: Node<'_>, ctx: &NativeContext<'_>, name: &str) -> bool {
    let Some(sup) = field(def, "superclasses") else { return false };
    kids(sup).into_iter().filter(|c| c.kind() != "keyword_argument").any(|b| qname(subscript_head(b), ctx).is_some_and(|q| is_typing(&q, name)))
}

/// What a private declaration at the identifier `ident` is, with the
/// statement that holds it.
fn private_kind<'t>(ident: Node<'t>, ctx: &NativeContext<'_>) -> Option<(Private, &'static str, Node<'t>)> {
    let src = ctx.source;
    let parent = ident.parent()?;
    match parent.kind() {
        "class_definition" if field(parent, "name") == Some(ident) => {
            let stmt = parent.parent().filter(|p| p.kind() == "decorated_definition").unwrap_or(parent);
            if base_is(parent, ctx, "Protocol") {
                Some((Private::Protocol, "Protocol", stmt))
            } else if base_is(parent, ctx, "TypedDict") {
                Some((Private::TypedDict, "TypedDict", stmt))
            } else {
                None
            }
        }
        "assignment" if field(parent, "left") == Some(ident) => {
            let stmt = parent.parent().filter(|p| p.kind() == "expression_statement")?;
            if let Some(t) = field(parent, "type")
                && qname(t, ctx).is_some_and(|q| is_typing(&q, "TypeAlias"))
            {
                return Some((Private::TypeAlias, "type alias", stmt));
            }
            let right = unparen(field(parent, "right")?);
            if right.kind() != "call" {
                return None;
            }
            let q = qname(field(right, "function")?, ctx)?;
            for (k, what) in [("TypeVar", "TypeVar"), ("ParamSpec", "ParamSpec"), ("TypeVarTuple", "TypeVarTuple")] {
                if is_typing(&q, k) {
                    return Some((Private::TypeVar, what, stmt));
                }
            }
            if is_typing(&q, "TypedDict") {
                return Some((Private::TypedDict, "TypedDict", stmt));
            }
            let _ = src;
            None
        }
        "type_alias_statement" => Some((Private::TypeAlias, "type alias", parent)),
        _ => None,
    }
}

/// The edit that deletes a declaration statement alone on its lines, with
/// the blank lines it leaves behind: none at the end of the file, and one
/// separator where it stood between two blank lines.
fn delete_declaration(stmt: Node<'_>, src: &str) -> Vec<TextEdit> {
    if remove_stmt(stmt, src).is_none() {
        return Vec::new();
    }
    let mut start = src[..stmt.start_byte()].rfind('\n').map_or(0, |i| i + 1);
    let e = stmt.end_byte();
    let mut end = src[e..].find('\n').map_or(src.len(), |i| e + i + 1);
    let blank_before = start == 0 || src[..start - 1].rsplit('\n').next().is_some_and(|l| l.trim().is_empty());
    if src[end..].trim().is_empty() {
        // The end of the file: drop the blank lines above as well.
        while start > 0 {
            let prev = src[..start - 1].rfind('\n').map_or(0, |i| i + 1);
            if src[prev..start].trim().is_empty() {
                start = prev;
            } else {
                break;
            }
        }
        end = src.len();
    } else if blank_before {
        while end < src.len() {
            let next = src[end..].find('\n').map_or(src.len(), |i| end + i + 1);
            if src[end..next].trim().is_empty() {
                end = next;
            } else {
                break;
            }
        }
    }
    vec![TextEdit::new(start, end, "")]
}

fn unused_private(n: Node<'_>, ctx: &NativeContext<'_>, want: Private, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    for sym in &model.symbols {
        if !is_private(&sym.name) || !sym.refs.is_empty() || sym.exported || sym.decls.len() != 1 {
            continue;
        }
        let (s, e) = sym.decls[0];
        let Some(ident) = n.descendant_for_byte_range(s, e) else { continue };
        // A type alias statement holds its name as a `type` node.
        let ident = if ident.kind() == "identifier" { ident } else { continue };
        let probe = if ident.parent().is_some_and(|p| p.kind() == "type") { ident.parent().unwrap_or(ident) } else { ident };
        let Some((kind, what, stmt)) = private_kind(probe, ctx) else { continue };
        if kind != want || named_in_strings(n, src, &sym.name, (stmt.start_byte(), stmt.end_byte())) {
            continue;
        }
        let edits = delete_declaration(stmt, src);
        let label = match kind {
            Private::TypeVar => format!("Private {what} `{}` is never used", sym.name),
            Private::Protocol => format!("Private protocol `{}` is never used", sym.name),
            Private::TypeAlias => format!("Private type alias `{}` is never used", sym.name),
            Private::TypedDict => format!("Private TypedDict `{}` is never used", sym.name),
        };
        out.push(span(s, e, label, edits));
    }
}

rule!(UnusedPrivateTypeVar, "py/unused-private-type-var", &["module"], true, |n, ctx, out| { unused_private(n, ctx, Private::TypeVar, out) });
rule!(UnusedPrivateProtocol, "py/unused-private-protocol", &["module"], true, |n, ctx, out| { unused_private(n, ctx, Private::Protocol, out) });
rule!(UnusedPrivateTypeAlias, "py/unused-private-type-alias", &["module"], true, |n, ctx, out| { unused_private(n, ctx, Private::TypeAlias, out) });
rule!(UnusedPrivateTypedDict, "py/unused-private-typed-dict", &["module"], true, |n, ctx, out| { unused_private(n, ctx, Private::TypedDict, out) });

// ------------------------------------------------------- PLE0237, PLW0244

/// The string slots a `__slots__` assignment value lists, with their
/// spans; `None` when an element is not a plain string.
fn slot_literals<'t>(v: Node<'t>, src: &str) -> Option<Vec<(String, Node<'t>)>> {
    let v = unparen(v);
    let mut out = Vec::new();
    match v.kind() {
        "tuple" | "list" | "set" | "expression_list" => {
            for e in kids(v) {
                if e.kind() != "string" {
                    return None;
                }
                let (p, c) = string_parts(e, src)?;
                if p.contains(['f', 'b']) {
                    return None;
                }
                out.push((c, e));
            }
        }
        "dictionary" => {
            for p in kids(v) {
                let k = field(p, "key")?;
                if k.kind() != "string" {
                    return None;
                }
                out.push((string_parts(k, src)?.1, k));
            }
        }
        _ => {}
    }
    Some(out)
}

/// The `__slots__` statements of a class body: the names they list.
/// `Err(())` when one of them holds something that is not a literal.
fn class_slots<'t>(class: Node<'t>, src: &str) -> Result<Vec<(String, Node<'t>)>, ()> {
    let mut out = Vec::new();
    let Some(body) = field(class, "body") else { return Ok(out) };
    for s in statements(body) {
        if s.kind() != "expression_statement" {
            continue;
        }
        let Some(a) = s.named_child(0).filter(|a| matches!(a.kind(), "assignment" | "augmented_assignment")) else { continue };
        let Some(left) = field(a, "left").filter(|l| l.kind() == "identifier" && text(*l, src) == "__slots__") else { continue };
        let _ = left;
        let Some(right) = field(a, "right") else { continue };
        out.extend(slot_literals(right, src).ok_or(())?);
    }
    Ok(out)
}

fn superclass_names(class: Node<'_>, src: &str) -> Vec<String> {
    field(class, "superclasses")
        .map(|s| kids(s).into_iter().filter(|c| c.kind() != "keyword_argument").map(|c| text(c, src).to_owned()).collect())
        .unwrap_or_default()
}

rule!(NonSlotAssignment, "py/non-slot-assignment", &["class_definition"], false, |n, ctx, out| {
    let src = ctx.source;
    if !superclass_names(n, src).iter().all(|b| b == "object") {
        return;
    }
    let Ok(slots) = class_slots(n, src) else { return };
    let mut names: BTreeSet<String> = slots.into_iter().map(|(s, _)| s).collect();
    if names.is_empty() || names.contains("__dict__") {
        return;
    }
    let Some(body) = field(n, "body") else { return };
    for s in statements(body) {
        let def = if s.kind() == "decorated_definition" { field(s, "definition") } else { Some(s) };
        let Some(def) = def.filter(|d| d.kind() == "function_definition") else { continue };
        for d in decorators(def) {
            if d.kind() == "attribute"
                && field(d, "attribute").is_some_and(|a| text(a, src) == "setter")
                && let Some(o) = field(d, "object").filter(|o| o.kind() == "identifier")
            {
                names.insert(text(o, src).to_owned());
            }
        }
    }
    for s in statements(body) {
        let Some(def) = Some(s).filter(|d| d.kind() == "function_definition") else { continue };
        if field(def, "name").is_none_or(|nm| text(nm, src) != "__init__") {
            continue;
        }
        for st in field(def, "body").map(statements).unwrap_or_default() {
            let Some(a) = st.named_child(0).filter(|a| st.kind() == "expression_statement" && matches!(a.kind(), "assignment" | "augmented_assignment")) else { continue };
            if a.kind() == "assignment" && a.parent().is_some() && field(a, "right").is_some_and(|r| r.kind() == "assignment") {
                continue;
            }
            let Some(left) = field(a, "left").filter(|l| l.kind() == "attribute") else { continue };
            let (Some(obj), Some(attr)) = (field(left, "object"), field(left, "attribute")) else { continue };
            if obj.kind() == "identifier" && text(obj, src) == "self" && !names.contains(text(attr, src)) {
                out.push(hit(left, format!("Attribute `{}` is not defined in class's `__slots__`", text(attr, src))));
            }
        }
    }
});

/// The classes defined in the file, by name; a name defined twice is left out.
fn classes_by_name<'t>(root: Node<'t>, src: &str) -> BTreeMap<String, Option<Node<'t>>> {
    let mut map: BTreeMap<String, Option<Node<'t>>> = BTreeMap::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "class_definition"
            && let Some(nm) = field(n, "name")
        {
            let e = map.entry(text(nm, src).to_owned()).or_insert(Some(n));
            if *e != Some(n) {
                *e = None;
            }
        }
        stack.extend(named_children(n));
    }
    map
}

rule!(RedefinedSlotsInSubclass, "py/redefined-slots-in-subclass", &["class_definition"], false, |n, ctx, out| {
    let src = ctx.source;
    let bases = superclass_names(n, src);
    if bases.is_empty() {
        return;
    }
    let Ok(slots) = class_slots(n, src) else { return };
    if slots.is_empty() {
        return;
    }
    let classes = classes_by_name(root_of(n), src);
    // Every class of the file this one inherits from, nearest first.
    let mut supers: Vec<Node<'_>> = Vec::new();
    let mut seen: BTreeSet<usize> = BTreeSet::new();
    seen.insert(n.id());
    let mut queue: Vec<String> = bases;
    while !queue.is_empty() {
        let name = queue.remove(0);
        let Some(Some(def)) = classes.get(&name) else { continue };
        if !seen.insert(def.id()) {
            continue;
        }
        supers.push(*def);
        queue.extend(superclass_names(*def, src));
    }
    let mut done: BTreeSet<(usize, String)> = BTreeSet::new();
    for (name, node) in &slots {
        for sup in &supers {
            let theirs = class_slots(*sup, src).unwrap_or_default();
            if theirs.iter().any(|(t, _)| t == name) && done.insert((node.start_byte(), field(*sup, "name").map_or(String::new(), |x| text(x, src).to_owned()))) {
                let base = field(*sup, "name").map_or("", |x| text(x, src));
                out.push(hit(*node, format!("Slot `{name}` redefined from base class `{base}`")));
            }
        }
    }
});

// ------------------------------------------------------------------ PYI036

struct Prm<'t> {
    node: Node<'t>,
    ann: Option<Node<'t>>,
    default: bool,
}

struct Params<'t> {
    positional: Vec<Prm<'t>>,
    kwonly: Vec<Prm<'t>>,
    vararg: Option<Prm<'t>>,
    kwarg: bool,
    range: (usize, usize),
}

fn read_params<'t>(def: Node<'t>) -> Option<Params<'t>> {
    let ps = field(def, "parameters")?;
    let mut p = Params { positional: Vec::new(), kwonly: Vec::new(), vararg: None, kwarg: false, range: (ps.start_byte(), ps.end_byte()) };
    let mut after_star = false;
    for c in kids(ps) {
        let (ann, default) = match c.kind() {
            "identifier" => (None, false),
            "typed_parameter" => (field(c, "type"), false),
            "default_parameter" => (None, true),
            "typed_default_parameter" => (field(c, "type"), true),
            "list_splat_pattern" => {
                p.vararg = Some(Prm { node: c, ann: None, default: false });
                after_star = true;
                continue;
            }
            "keyword_separator" => {
                after_star = true;
                continue;
            }
            "dictionary_splat_pattern" => {
                p.kwarg = true;
                continue;
            }
            "positional_separator" => continue,
            _ => continue,
        };
        if c.kind() == "typed_parameter" && c.named_child(0).is_some_and(|i| i.kind() == "list_splat_pattern") {
            p.vararg = Some(Prm { node: c, ann, default: false });
            after_star = true;
            continue;
        }
        if c.kind() == "typed_parameter" && c.named_child(0).is_some_and(|i| i.kind() == "dictionary_splat_pattern") {
            p.kwarg = true;
            continue;
        }
        let prm = Prm { node: c, ann, default };
        if after_star {
            p.kwonly.push(prm);
        } else {
            p.positional.push(prm);
        }
    }
    Some(p)
}

fn is_object_or_unused(ann: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    qname(ann, ctx).is_some_and(|q| q == "builtins.object" || q == "_typeshed.Unused")
}

fn is_base_exception_type(ann: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    let a = ann_expr(ann);
    let head = subscript_head(a);
    if head == a {
        return false;
    }
    let ok_head = qname(head, ctx).is_some_and(|q| q == "builtins.type" || is_typing(&q, "Type"));
    let args = subscript_args(a);
    ok_head && args.len() == 1 && qname(args[0], ctx).is_some_and(|q| q == "builtins.BaseException")
}

fn is_traceback_type(ann: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    qname(ann, ctx).is_some_and(|q| q == "types.TracebackType")
}

fn is_builtin_base_exception(ann: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    qname(ann, ctx).is_some_and(|q| q == "builtins.BaseException")
}

/// The one non-`None` element of `X | None`, `Optional[X]` or `Union[X, None]`.
fn non_none_element<'t>(ann: Node<'t>, ctx: &NativeContext<'_>) -> Option<Node<'t>> {
    let a = ann_expr(ann);
    let src = ctx.source;
    if a.kind() == "union_type" || (a.kind() == "binary_operator" && field(a, "operator").is_some_and(|o| text(o, src) == "|")) {
        let parts: Vec<Node<'_>> = if a.kind() == "union_type" { named_children(a).into_iter().map(ann_expr).collect() } else { vec![ann_expr(field(a, "left")?), ann_expr(field(a, "right")?)] };
        if parts.len() != 2 {
            return None;
        }
        return match (is_none(parts[0], src), is_none(parts[1], src)) {
            (false, _) => Some(parts[0]),
            (true, false) => Some(parts[1]),
            _ => None,
        };
    }
    let head = subscript_head(a);
    if head == a {
        return None;
    }
    let q = qname(head, ctx)?;
    let args = subscript_args(a);
    if is_typing(&q, "Optional") {
        return args.first().copied().filter(|x| !is_none(*x, src));
    }
    if !is_typing(&q, "Union") || args.len() != 2 {
        return None;
    }
    match (is_none(args[0], src), is_none(args[1], src)) {
        (false, true) => Some(args[0]),
        (true, false) => Some(args[1]),
        _ => None,
    }
}

fn check_exit(def: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(name) = field(def, "name").map(|n| text(n, src)) else { return };
    let is_async = children(def).iter().any(|c| c.kind() == "async");
    let kind = match (name, is_async) {
        ("__exit__", false) => "__exit__",
        ("__aexit__", true) => "__aexit__",
        _ => return,
    };
    let class = {
        let mut p = def.parent();
        if p.is_some_and(|x| x.kind() == "decorated_definition") {
            p = p.and_then(|x| x.parent());
        }
        match p.and_then(|b| b.parent()).filter(|c| c.kind() == "class_definition") {
            Some(c) => c,
            None => return,
        }
    };
    let Some(params) = read_params(def) else { return };
    let non_self: Vec<&Prm<'_>> = params.positional.iter().skip(1).collect();
    let msg = |m: String| m;
    if is_overload(def, src) {
        overload_check(def, class, &non_self, kind, &params, ctx, out);
        return;
    }
    if non_self.len() < 3 {
        match &params.vararg {
            Some(v) => {
                if let Some(ann) = v.ann.filter(|a| !is_object_or_unused(*a, ctx)) {
                    let a = ann_expr(ann);
                    out.push(span(a.start_byte(), a.end_byte(), msg(format!("Star-args in `{kind}` should be annotated with `object`")), vec![TextEdit::new(a.start_byte(), a.end_byte(), "object")]));
                }
            }
            None => out.push(span(params.range.0, params.range.1, format!("If there are no star-args, `{kind}` should have at least 3 non-keyword-only args (excluding `self`)"), Vec::new())),
        }
    }
    for p in non_self.iter().skip(3).filter(|p| !p.default) {
        out.push(hit(p.node, format!("All arguments after the first four in `{kind}` must have a default value")));
    }
    for p in params.kwonly.iter().filter(|p| !p.default) {
        out.push(hit(p.node, format!("All keyword-only arguments in `{kind}` must have a default value")));
    }
    type Check = fn(Node<'_>, &NativeContext<'_>) -> bool;
    let checks: [(&str, Check); 3] = [
        ("The first argument in `{k}` should be annotated with `object` or `type[BaseException] | None`", is_base_exception_type),
        ("The second argument in `{k}` should be annotated with `object` or `BaseException | None`", is_builtin_base_exception),
        ("The third argument in `{k}` should be annotated with `object` or `types.TracebackType | None`", is_traceback_type),
    ];
    for (p, (message, pred)) in non_self.iter().take(3).zip(checks) {
        let Some(ann) = p.ann else { continue };
        if is_object_or_unused(ann, ctx) || non_none_element(ann, ctx).is_some_and(|e| pred(e, ctx)) {
            continue;
        }
        let a = ann_expr(ann);
        out.push(hit(a, message.replace("{k}", kind)));
    }
}

fn overload_check(def: Node<'_>, class: Node<'_>, non_self: &[&Prm<'_>], kind: &str, params: &Params<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(body) = field(class, "body") else { return };
    let mut overloads: Vec<Node<'_>> = Vec::new();
    for s in statements(body) {
        let d = if s.kind() == "decorated_definition" { field(s, "definition") } else { Some(s) };
        if let Some(d) = d.filter(|d| d.kind() == "function_definition")
            && field(d, "name").is_some_and(|nm| text(nm, src) == kind)
            && is_overload(d, src)
        {
            overloads.push(d);
        }
    }
    if overloads.len() != 2 {
        return;
    }
    for d in &overloads {
        let is_async = children(*d).iter().any(|c| c.kind() == "async");
        if is_async != (kind == "__aexit__") {
            return;
        }
        let Some(p) = read_params(*d) else { return };
        if p.vararg.is_some() || p.kwarg || !p.kwonly.is_empty() || p.positional.len() != 4 {
            return;
        }
    }
    let _ = def;
    let loose = |p: &Prm<'_>, f: &dyn Fn(Node<'_>) -> bool| p.ann.is_none_or(|a| f(a) || is_object_or_unused(a, ctx));
    if non_self.iter().all(|p| loose(p, &|a| is_none(a, src))) {
        return;
    }
    if non_self.len() == 3
        && loose(non_self[0], &|a| is_base_exception_type(a, ctx))
        && loose(non_self[1], &|a| is_builtin_base_exception(a, ctx))
        && loose(non_self[2], &|a| is_traceback_type(a, ctx))
    {
        return;
    }
    out.push(span(params.range.0, params.range.1, format!("Annotations for a three-argument `{kind}` overload (excluding `self`) should either be `None, None, None` or `type[BaseException], BaseException, types.TracebackType`"), Vec::new()));
}

rule!(BadExitAnnotation, "py/bad-exit-annotation", &["function_definition"], true, |n, ctx, out| { check_exit(n, ctx, out) });

// ------------------------------------------------------------------- I002

fn required_imports(ctx: &NativeContext<'_>) -> Vec<String> {
    ctx.options.get("required_imports").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|s| s.as_str().map(|s| s.trim().to_owned())).collect()).unwrap_or_default()
}

/// Whether a top-level statement is `import_text` (`import a`, `import a as b`,
/// `from m import n`, `from m import n as o`).
fn statement_has_import(stmt: Node<'_>, want: &str, src: &str) -> bool {
    let norm = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let want = norm(want);
    match stmt.kind() {
        "import_statement" => {
            let names: Vec<String> = children(stmt).iter().filter(|c| matches!(c.kind(), "dotted_name" | "aliased_import")).map(|c| norm(text(*c, src))).collect();
            names.iter().any(|n| format!("import {n}") == want)
        }
        "import_from_statement" | "future_import_statement" => {
            let m = match field(stmt, "module_name") {
                Some(m) => norm(text(m, src)),
                None if stmt.kind() == "future_import_statement" => "__future__".to_owned(),
                None => return false,
            };
            let mut c = stmt.walk();
            let names: Vec<String> = stmt.children_by_field_name("name", &mut c).map(|c| norm(text(c, src))).collect();
            names.iter().any(|n| format!("from {m} import {n}") == want)
        }
        _ => false,
    }
}

rule!(MissingRequiredImport, "py/missing-required-import", &["module"], false, |n, ctx, out| {
    let src = ctx.source;
    let wanted = required_imports(ctx);
    if wanted.is_empty() {
        return;
    }
    let stmts = statements(n);
    if stmts.iter().all(|s| s.kind() == "expression_statement" && s.named_child(0).is_some_and(|c| c.kind() == "string")) {
        return;
    }
    let is_stub = ctx.file.path().ends_with(".pyi");
    // Where the import goes: below the module docstring, else above the first
    // statement but below the leading comments.
    let doc = stmts.first().filter(|s| s.kind() == "expression_statement" && s.named_child(0).is_some_and(|c| c.kind() == "string")).copied();
    let (at, prefix) = match doc {
        Some(d) => {
            let e = d.end_byte();
            let le = src[e..].find('\n').map_or(src.len(), |i| e + i + 1);
            (le, if le == e { "\n\n" } else { "\n" })
        }
        None => {
            let mut at = 0;
            for line in src.split_inclusive('\n') {
                let t = line.trim();
                if t.starts_with('#') {
                    at += line.len();
                } else {
                    break;
                }
            }
            (at, "")
        }
    };
    for w in wanted.iter().rev() {
        if is_stub && w.starts_with("from __future__") {
            continue;
        }
        if stmts.iter().any(|s| statement_has_import(*s, w, src)) {
            continue;
        }
        let tail = if doc.is_none() && !src[at..].trim().is_empty() { "\n" } else { "" };
        let text = format!("{prefix}{w}\n{tail}");
        out.push(span(0, 0, format!("Missing required import: `{w}`"), vec![TextEdit::new(at, at, text)]));
    }
});

// ------------------------------------------------------------------- ICN002

rule!(BannedImportAlias, "py/banned-import-alias", &["import_statement", "import_from_statement"], false, |n, ctx, out| {
    let src = ctx.source;
    let Some(map) = ctx.options.get("banned_aliases").and_then(|v| v.as_object()) else { return };
    if map.is_empty() {
        return;
    }
    let mut pairs: Vec<(String, String)> = Vec::new();
    if n.kind() == "import_statement" {
        for c in children(n).into_iter().filter(|c| c.kind() == "aliased_import") {
            if let (Some(nm), Some(al)) = (field(c, "name"), field(c, "alias")) {
                pairs.push((text(nm, src).to_owned(), text(al, src).to_owned()));
            }
        }
    } else if let Some(m) = field(n, "module_name") {
        let module = text(m, src);
        let mut cur = n.walk();
        for c in n.children_by_field_name("name", &mut cur).filter(|c| c.kind() == "aliased_import") {
            if let (Some(nm), Some(al)) = (field(c, "name"), field(c, "alias")) {
                pairs.push((format!("{module}.{}", text(nm, src)), text(al, src).to_owned()));
            }
        }
    }
    for (name, alias) in pairs {
        if map.get(&name).and_then(|v| v.as_array()).is_some_and(|a| a.iter().any(|x| x.as_str() == Some(alias.as_str()))) {
            out.push(hit(n, format!("`{name}` should not be imported as `{alias}`")));
        }
    }
});

// -------------------------------------------------------------------- W505

fn width(line: &str, tab: usize) -> usize {
    let mut w = 0;
    for c in line.chars() {
        if c == '\t' {
            w += tab - w % tab;
        } else {
            w += 1;
        }
    }
    w
}

rule!(DocLineTooLong, "py/doc-line-too-long", &["module"], false, |n, ctx, out| {
    let src = ctx.source;
    let limit = ctx.options.get("max_doc_length").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    if limit == 0 {
        return;
    }
    // Lines that belong to a standalone comment or a string statement.
    let mut doc_lines: BTreeSet<usize> = BTreeSet::new();
    let line_of = |off: usize| src[..off.min(src.len())].bytes().filter(|b| *b == b'\n').count();
    let mut stack = vec![n];
    while let Some(c) = stack.pop() {
        match c.kind() {
            "comment" => {
                let ls = src[..c.start_byte()].rfind('\n').map_or(0, |i| i + 1);
                if src[ls..c.start_byte()].trim().is_empty() {
                    doc_lines.insert(line_of(c.start_byte()));
                }
            }
            "expression_statement" if c.named_child_count() == 1 && c.named_child(0).is_some_and(|s| matches!(s.kind(), "string" | "concatenated_string")) => {
                let s = c.named_child(0).unwrap_or(c);
                for l in line_of(s.start_byte())..=line_of(s.end_byte().saturating_sub(1)) {
                    doc_lines.insert(l);
                }
                continue;
            }
            _ => {}
        }
        stack.extend(named_children(c));
    }
    let mut offset = 0;
    for (i, line) in src.split_inclusive('\n').enumerate() {
        let start = offset;
        offset += line.len();
        if !doc_lines.contains(&i) {
            continue;
        }
        let l = line.trim_end_matches(['\n', '\r']);
        let w = width(l, 4);
        if w <= limit {
            continue;
        }
        let chunks: Vec<&str> = l.split_whitespace().collect();
        if chunks.len() < 2 {
            continue;
        }
        if let Some(last) = chunks.last()
            && last.contains("://")
            && w.saturating_sub(width(last, 4)) <= limit
        {
            continue;
        }
        if chunks[0] == "#" && matches!(chunks[1], "SPDX-License-Identifier:" | "SPDX-FileCopyrightText:") {
            continue;
        }
        // The part past the limit.
        let mut cut = 0;
        let mut cw = 0;
        for (bi, ch) in l.char_indices() {
            if cw >= limit {
                break;
            }
            cw += if ch == '\t' { 4 - cw % 4 } else { 1 };
            cut = bi + ch.len_utf8();
        }
        out.push(span(start + cut, start + l.len(), format!("Doc line too long ({w} > {limit})"), Vec::new()));
    }
});

pub static RULES: &[&dyn NativeRule] = &[
    &ImplicitReturn,
    &RepeatedAppend,
    &UnusedPrivateTypeVar,
    &UnusedPrivateProtocol,
    &UnusedPrivateTypeAlias,
    &UnusedPrivateTypedDict,
    &NonSlotAssignment,
    &RedefinedSlotsInSubclass,
    &BadExitAnnotation,
    &MissingRequiredImport,
    &BannedImportAlias,
    &DocLineTooLong,
];

#[cfg(test)]
mod tests {
    use super::*;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        crate::run(rule, Language::Python, src)
    }

    #[test]
    fn implicit_return() {
        let src = "def f(a):\n    if a:\n        return 1\n";
        let r = run(&ImplicitReturn, src);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].edits[0].text, "\n    return None");
        assert!(run(&ImplicitReturn, "def f(a):\n    if a:\n        return 1\n    else:\n        return 2\n").is_empty());
        assert!(run(&ImplicitReturn, "def f(a):\n    while True:\n        if a:\n            return 1\n").is_empty());
        assert!(run(&ImplicitReturn, "import sys\ndef f(a):\n    if a:\n        return 1\n    sys.exit(1)\n").is_empty());
    }

    #[test]
    fn repeated_append() {
        let src = "xs = []\nxs.append(1)\nxs.append(2)\n";
        let r = run(&RepeatedAppend, src);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].edits[0].text, "xs.extend((1, 2))");
        assert!(run(&RepeatedAppend, "xs = foo()\nxs.append(1)\nxs.append(2)\n").is_empty());
    }

    #[test]
    fn slots() {
        let src = "class A:\n    __slots__ = ('a',)\n    def __init__(self):\n        self.a = 1\n        self.b = 2\n";
        assert_eq!(run(&NonSlotAssignment, src).len(), 1);
        let src = "class A:\n    __slots__ = ('a',)\nclass B(A):\n    __slots__ = ('a', 'c')\n";
        assert_eq!(run(&RedefinedSlotsInSubclass, src).len(), 1);
    }

    #[test]
    fn private() {
        let src = "from typing import TypeVar, Protocol\n_T = TypeVar('_T')\nclass _P(Protocol):\n    pass\n";
        assert_eq!(run(&UnusedPrivateTypeVar, src).len(), 1);
        assert_eq!(run(&UnusedPrivateProtocol, src).len(), 1);
        let used = "from typing import TypeVar\n_T = TypeVar('_T')\ndef f(x: _T) -> _T: ...\n";
        assert!(run(&UnusedPrivateTypeVar, used).is_empty());
    }

    #[test]
    fn exit() {
        let src = "class A:\n    def __exit__(self, a: int, b, c): ...\n";
        assert_eq!(run(&BadExitAnnotation, src).len(), 1);
        let ok = "class A:\n    def __exit__(self, a: object, b: object, c: object): ...\n";
        assert!(run(&BadExitAnnotation, ok).is_empty());
    }
}

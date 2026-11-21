//! W-183 (and W-181's UP035): the rows whose finding depends on the project's Python version
//! (SR-RULE-FORMAT decision 2d) — Ruff's FA100, FA102 and UP036. The engine
//! gates each on its rule file's `finding_target:`; the rules read the
//! target themselves where the finding differs inside the range
//! (`NativeContext::file.python_target`). Behaviour is Ruff 0.16.9's (MIT;
//! read from its source and tests, research only, L5), each rule written
//! fresh. PERF203 and Pylint's two `using-*` rows are declarative rules in
//! the pack and need nothing here.

use crate::python_util::{PY, hit, hit_fix, indent_at, is_docstring_stmt, kids, qualified_name, remove_stmt, starts_line};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

fn root_of(n: Node<'_>) -> Node<'_> {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        cur = p;
    }
    cur
}

fn has_future_annotations(root: Node<'_>, src: &str) -> bool {
    kids(root).into_iter().any(|s| s.kind() == "future_import_statement" && text(s, src).contains("annotations"))
}

/// The edit that adds `from __future__ import annotations`: after the last
/// `__future__` import, else after the module docstring, else before the
/// first statement.
fn future_import_edit(root: Node<'_>, src: &str) -> TextEdit {
    const LINE: &str = "from __future__ import annotations\n";
    let after_line = |end: usize| src[end..].find('\n').map(|i| end + i + 1);
    let top = kids(root);
    let at_end = |end: usize, lead: &str| match after_line(end) {
        Some(at) => TextEdit::insert(at, format!("{lead}{LINE}")),
        None => TextEdit::insert(src.len(), format!("\n{lead}{LINE}")),
    };
    if let Some(last) = top.iter().rev().find(|s| s.kind() == "future_import_statement") {
        return at_end(last.end_byte(), "");
    }
    match top.first() {
        Some(first) if is_docstring_stmt(*first) => at_end(first.end_byte(), "\n"),
        Some(first) => {
            let at = src[..first.start_byte()].rfind('\n').map(|i| i + 1).unwrap_or(0);
            TextEdit::insert(at, format!("{LINE}\n"))
        }
        None => TextEdit::insert(0, LINE),
    }
}

/// The annotations the interpreter evaluates when the file lacks the
/// `__future__` import: a parameter's, a return type, a variable's outside
/// any function (outermost `type` nodes only).
fn runtime_annotation(ann: Node<'_>) -> bool {
    let Some(parent) = ann.parent() else { return false };
    match parent.kind() {
        "typed_parameter" | "typed_default_parameter" => true,
        "function_definition" => parent.child_by_field_name("return_type") == Some(ann),
        "assignment" => {
            let mut q = parent.parent();
            while let Some(x) = q {
                if x.kind() == "function_definition" {
                    return false;
                }
                q = x.parent();
            }
            parent.child_by_field_name("type") == Some(ann)
        }
        _ => false,
    }
}

/// What a file's rule needs before it looks at annotations: a Python file,
/// not a stub, without the import, and a known target.
fn setup<'a>(ctx: &NativeContext<'a>, id: &str, ann: Node<'_>) -> Option<Vec<u64>> {
    if ctx.file.path().ends_with(".pyi") || !runtime_annotation(ann) {
        return None;
    }
    let target = ctx.file.python_target(id)?;
    (!has_future_annotations(root_of(ann), ctx.source)).then_some(target)
}

fn below(target: &[u64], v: &[u64]) -> bool {
    target < v
}

/// Every node of `ann` that is not inside a string, outermost first.
fn nodes_of<'t>(ann: Node<'t>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    let mut stack = vec![ann];
    while let Some(n) = stack.pop() {
        if n.kind() == "string" {
            continue;
        }
        out.push(n);
        let mut kids = named_children(n);
        kids.reverse();
        stack.extend(kids);
    }
    out
}

// ---------------------------------------------------------------- FA102

const PEP585: &[&str] = &[
    "builtins.list",
    "builtins.dict",
    "builtins.set",
    "builtins.frozenset",
    "builtins.tuple",
    "builtins.type",
    "collections.deque",
    "collections.defaultdict",
    "collections.OrderedDict",
    "collections.Counter",
    "collections.ChainMap",
    "collections.abc.Awaitable",
    "collections.abc.Coroutine",
    "collections.abc.AsyncIterable",
    "collections.abc.AsyncIterator",
    "collections.abc.AsyncGenerator",
    "collections.abc.Iterable",
    "collections.abc.Iterator",
    "collections.abc.Generator",
    "collections.abc.Reversible",
    "collections.abc.Container",
    "collections.abc.Collection",
    "collections.abc.Callable",
    "collections.abc.Set",
    "collections.abc.MutableSet",
    "collections.abc.Mapping",
    "collections.abc.MutableMapping",
    "collections.abc.Sequence",
    "collections.abc.MutableSequence",
    "collections.abc.MappingView",
    "collections.abc.KeysView",
    "collections.abc.ItemsView",
    "collections.abc.ValuesView",
    "contextlib.AbstractContextManager",
    "contextlib.AbstractAsyncContextManager",
    "re.Pattern",
    "re.Match",
];

/// Whether `n` is a part of a larger `X | Y` (so the whole is reported once).
fn nested_union(n: Node<'_>, src: &str) -> bool {
    let is_union = |x: Node<'_>| x.kind() == "union_type" || (x.kind() == "binary_operator" && x.child_by_field_name("operator").is_some_and(|o| text(o, src) == "|"));
    let mut p = n.parent();
    if p.is_some_and(|x| x.kind() == "type") {
        p = p.and_then(|x| x.parent());
    }
    p.is_some_and(is_union)
}

fn fa102(ann: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let id = "py/future-required-type-annotation";
    if ann.parent().is_some_and(|p| p.kind() == "type") {
        return;
    }
    let Some(target) = setup(ctx, id, ann) else { return };
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let edit = future_import_edit(root_of(ann), src);
    for n in nodes_of(ann) {
        let message = match n.kind() {
            "union_type" | "binary_operator" if nested_union(n, src) => None,
            "union_type" if below(&target, &[3, 10]) => Some("Missing `from __future__ import annotations`, but uses a PEP 604 union (`X | Y`)"),
            "binary_operator" if below(&target, &[3, 10]) && n.child_by_field_name("operator").is_some_and(|o| text(o, src) == "|") => {
                Some("Missing `from __future__ import annotations`, but uses a PEP 604 union (`X | Y`)")
            }
            "generic_type" if below(&target, &[3, 9]) => kids(n)
                .first()
                .filter(|b| matches!(b.kind(), "identifier"))
                .and_then(|b| qualified_name(*b, model, src))
                .filter(|q| PEP585.contains(&q.as_str()))
                .map(|_| "Missing `from __future__ import annotations`, but uses a PEP 585 collection"),
            "subscript" if below(&target, &[3, 9]) => n
                .child_by_field_name("value")
                .and_then(|v| qualified_name(v, model, src))
                .filter(|q| PEP585.contains(&q.as_str()))
                .map(|_| "Missing `from __future__ import annotations`, but uses a PEP 585 collection"),
            _ => None,
        };
        if let Some(m) = message {
            out.push(hit_fix(n, m, vec![edit.clone()]));
        }
    }
}

// ---------------------------------------------------------------- FA100

/// The `typing` names written as a built-in or `collections` type under
/// PEP 585 (Python 3.9, or 3.7 with the import).
const TYPING_585: &[&str] = &[
    "List",
    "Dict",
    "Set",
    "FrozenSet",
    "Tuple",
    "Type",
    "Deque",
    "DefaultDict",
    "OrderedDict",
    "Counter",
    "ChainMap",
    "AbstractSet",
    "Awaitable",
    "Coroutine",
    "AsyncIterable",
    "AsyncIterator",
    "AsyncGenerator",
    "Iterable",
    "Iterator",
    "Generator",
    "Reversible",
    "Container",
    "Collection",
    "Callable",
    "MutableSet",
    "Mapping",
    "MutableMapping",
    "Sequence",
    "MutableSequence",
    "MappingView",
    "KeysView",
    "ItemsView",
    "ValuesView",
    "ContextManager",
    "AsyncContextManager",
    "Pattern",
    "Match",
];

fn fa100(ann: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let id = "py/future-rewritable-type-annotation";
    if ann.parent().is_some_and(|p| p.kind() == "type") {
        return;
    }
    let Some(target) = setup(ctx, id, ann) else { return };
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let edit = future_import_edit(root_of(ann), src);
    for n in nodes_of(ann) {
        if !matches!(n.kind(), "identifier" | "attribute") {
            continue;
        }
        if n.kind() == "identifier" && n.parent().is_some_and(|p| p.kind() == "attribute") {
            continue;
        }
        let Some(q) = qualified_name(n, model, src) else { continue };
        let Some(name) = q.strip_prefix("typing.").or_else(|| q.strip_prefix("typing_extensions.")) else { continue };
        let rewritable = (TYPING_585.contains(&name) && below(&target, &[3, 9])) || (matches!(name, "Optional" | "Union") && below(&target, &[3, 10]));
        if rewritable {
            out.push(hit_fix(n, format!("Add `from __future__ import annotations` to simplify `typing.{name}`"), vec![edit.clone()]));
        }
    }
}

macro_rules! ann_rule {
    ($name:ident, $id:literal, $f:path) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &["type"]
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                $f(node.get_inner_node(), ctx, out)
            }
        }
    };
}

ann_rule!(FutureRequiredTypeAnnotation, "py/future-required-type-annotation", fa102);
ann_rule!(FutureRewritableTypeAnnotation, "py/future-rewritable-type-annotation", fa100);

// ---------------------------------------------------------------- UP036

/// Whether `cond` is a `sys.version_info` comparison whose result is the
/// same on every version from `target` up: `Some(true)` or `Some(false)`.
/// Python compares a tuple of the version with the written one, longer
/// being greater, so `> (3, 9)` and `>= (3, 9)` both mean "3.9 or later" and
/// `< (3, 9)` and `<= (3, 9)` both "earlier than 3.9".
fn static_version_test(cond: Node<'_>, target: &[u64], ctx: &NativeContext<'_>) -> Option<bool> {
    let model = ctx.semantic?;
    let src = ctx.source;
    if cond.kind() != "comparison_operator" {
        return None;
    }
    let operands = kids(cond);
    let [left, right] = operands.as_slice() else { return None };
    let mut op = None;
    let mut c = cond.walk();
    for ch in cond.children(&mut c) {
        if !ch.is_named() {
            op = Some(text(ch, src).to_owned());
        }
    }
    let op = op?;
    let int = |n: Node<'_>| (n.kind() == "integer").then(|| text(n, src).parse::<u64>().ok()).flatten();
    let is_version = |n: Node<'_>| qualified_name(n, model, src).is_some_and(|q| q == "sys.version_info");
    let target: Vec<u64> = target.iter().copied().take(2).collect();
    if is_version(*left) && right.kind() == "tuple" {
        let v: Option<Vec<u64>> = kids(*right).into_iter().map(int).collect();
        let v = v.filter(|v| (1..=2).contains(&v.len()))?;
        let reached = target >= v;
        return match op.as_str() {
            ">=" | ">" if reached => Some(true),
            "<" | "<=" if reached => Some(false),
            _ => None,
        };
    }
    if left.kind() == "subscript" && is_version(left.child_by_field_name("value")?) && int(left.child_by_field_name("subscript")?) == Some(0) {
        let n = int(*right)?;
        let major = *target.first()?;
        return match (op.as_str(), major.cmp(&n)) {
            (">=", std::cmp::Ordering::Equal | std::cmp::Ordering::Greater) | (">", std::cmp::Ordering::Greater) => Some(true),
            ("<", std::cmp::Ordering::Equal | std::cmp::Ordering::Greater) | ("<=", std::cmp::Ordering::Greater) => Some(false),
            ("==", std::cmp::Ordering::Equal) | ("!=", std::cmp::Ordering::Greater) => Some(true),
            ("==", std::cmp::Ordering::Greater) | ("!=", std::cmp::Ordering::Equal) => Some(false),
            _ => None,
        };
    }
    None
}

/// `block`'s statements as lines at `indent` (the first line carries none:
/// the edit starts after the `if`'s own indentation). `None` when the
/// block shares its line with the header or a line is indented less than
/// the block.
fn lifted(block: Node<'_>, src: &str, indent: &str) -> Option<String> {
    let first = block.start_byte();
    if !starts_line(src, first) {
        return None;
    }
    let body_indent = indent_at(src, first);
    let mut lines = Vec::new();
    for (i, l) in src[first..block.end_byte()].split('\n').enumerate() {
        if i == 0 {
            lines.push(l.to_owned());
        } else if l.trim().is_empty() {
            lines.push(String::new());
        } else {
            lines.push(format!("{indent}{}", l.strip_prefix(body_indent.as_str())?));
        }
    }
    Some(lines.join("\n"))
}

fn up036(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let id = "py/outdated-version-block";
    if ctx.file.path().ends_with(".pyi") {
        return;
    }
    let Some(target) = ctx.file.python_target(id) else { return };
    let Some(cond) = n.child_by_field_name("condition") else { return };
    let Some(always) = static_version_test(cond, &target, ctx) else { return };
    let src = ctx.source;
    let t = format!("{}.{}", target.first().copied().unwrap_or(3), target.get(1).copied().unwrap_or(0));
    let message = if always {
        format!("This version check is always true on Python {t} and later; keep the body and drop the check")
    } else {
        format!("This version check is always false on Python {t} and later; drop the block")
    };
    if n.kind() != "if_statement" || !starts_line(src, n.start_byte()) {
        out.push(hit(n, message));
        return;
    }
    let alternatives: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| matches!(c.kind(), "elif_clause" | "else_clause")).collect();
    let indent = indent_at(src, n.start_byte());
    let edits = if always {
        n.child_by_field_name("consequence").and_then(|b| lifted(b, src, &indent)).map(|t| vec![TextEdit::new(n.start_byte(), n.end_byte(), t)])
    } else {
        match alternatives.as_slice() {
            [] => remove_stmt(n, src),
            [e] if e.kind() == "else_clause" => e.child_by_field_name("body").and_then(|b| lifted(b, src, &indent)).map(|t| vec![TextEdit::new(n.start_byte(), n.end_byte(), t)]),
            _ => None,
        }
    };
    match edits {
        Some(e) => out.push(hit_fix(n, message, e)),
        None => out.push(hit(n, message)),
    }
}

macro_rules! if_rule {
    ($name:ident, $id:literal, $f:path) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &["if_statement", "elif_clause"]
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                $f(node.get_inner_node(), ctx, out)
            }
        }
    };
}

if_rule!(OutdatedVersionBlock, "py/outdated-version-block", up036);

// ---------------------------------------------------------------- UP035

/// The order Ruff reports one statement's groups in (by the module the
/// names move to).
const TARGET_ORDER: &[&str] = &["collections.abc", "warnings", "types", "typing", "collections", "re", "shlex", "enum"];

fn version_of(s: &str) -> Option<Vec<u64>> {
    s.split('.').map(|p| p.parse().ok()).collect()
}

/// UP035: a name imported from where it no longer lives, judged against
/// the table of moved names (`data/python/moved-names.json`, generated by
/// `scripts/ruff-moved-names.py` from Ruff's own table) and the file's
/// target. A name moved and renamed (`typing.List` is `list`) is reported
/// without an edit; the others are one finding per module they move to.
fn up035(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let id = "py/deprecated-import";
    let Some(target) = ctx.file.python_target(id) else { return };
    let Some(table) = ctx.data.get("python/moved-names").and_then(|t| t.get("entries")).and_then(|e| e.as_array()) else { return };
    let src = ctx.source;
    let Some(module) = n.child_by_field_name("module_name").filter(|m| m.kind() == "dotted_name").map(|m| text(m, src)) else { return };
    let mut cursor = n.walk();
    let names: Vec<Node<'_>> = n.children_by_field_name("name", &mut cursor).collect();
    if names.is_empty() || n.children(&mut n.walk()).any(|c| c.kind() == "wildcard_import") {
        return;
    }
    let target: Vec<u64> = target.iter().copied().take(2).collect();
    let live = |e: &serde_json::Value| -> bool {
        let at = |k: &str| e.get(k).and_then(|v| v.as_str()).and_then(version_of);
        e.get("module").and_then(|m| m.as_str()) == Some(module) && at("since").is_some_and(|s| target >= s) && at("until").is_none_or(|u| target < u)
    };
    let entries: Vec<&serde_json::Value> = table.iter().filter(|e| live(e)).collect();
    if entries.is_empty() {
        return;
    }
    let base = |name: Node<'_>| if name.kind() == "aliased_import" { name.child_by_field_name("name").map(|x| text(x, src)).unwrap_or_default() } else { text(name, src) };
    let has_comment = n.children(&mut n.walk()).any(|c| c.kind() == "comment");
    let moved_to = |target_module: &str, name: &str| entries.iter().any(|e| e.get("rename").is_none() && e.get("target").and_then(|t| t.as_str()) == Some(target_module) && e.get("name").and_then(|x| x.as_str()) == Some(name));
    for tm in TARGET_ORDER {
        let matched: Vec<Node<'_>> = names.iter().copied().filter(|x| moved_to(tm, base(*x))).collect();
        if matched.is_empty() {
            continue;
        }
        let list = |v: &[Node<'_>]| v.iter().map(|x| text(*x, src)).collect::<Vec<_>>().join(", ");
        let message = format!("Import from `{tm}` instead: {}", matched.iter().map(|x| format!("`{}`", base(*x))).collect::<Vec<_>>().join(", "));
        let unmatched: Vec<Node<'_>> = names.iter().copied().filter(|x| !matched.contains(x)).collect();
        let moved = format!("from {tm} import {}", list(&matched));
        let edit = if has_comment {
            None
        } else if unmatched.is_empty() {
            Some(moved)
        } else if starts_line(src, n.start_byte()) {
            Some(format!("from {module} import {}\n{}{moved}", list(&unmatched), indent_at(src, n.start_byte())))
        } else {
            None
        };
        match edit {
            Some(t) => out.push(hit_fix(n, message, vec![TextEdit::new(n.start_byte(), n.end_byte(), t)])),
            None => out.push(hit(n, message)),
        }
    }
    for name in &names {
        if let Some(e) = entries.iter().find(|e| e.get("rename").is_some() && e.get("name").and_then(|x| x.as_str()) == Some(base(*name))) {
            let to = e.get("target").and_then(|t| t.as_str()).unwrap_or_default();
            out.push(hit(n, format!("`{module}.{}` is deprecated, use `{to}` instead", base(*name))));
        }
    }
}

pub struct DeprecatedImport;
impl NativeRule for DeprecatedImport {
    fn id(&self) -> &'static str {
        "py/deprecated-import"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["import_from_statement"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        up035(node.get_inner_node(), ctx, out)
    }
}

pub static RULES: &[&dyn NativeRule] = &[&FutureRequiredTypeAnnotation, &FutureRewritableTypeAnnotation, &OutdatedVersionBlock, &DeprecatedImport];

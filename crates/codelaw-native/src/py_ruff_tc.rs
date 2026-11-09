//! W-139 strand W-139-ruff: flake8-type-checking's imports — TC001, TC002
//! and TC003 (an import used only in annotations belongs under
//! `if TYPE_CHECKING:`, by where the module comes from) and TC004 (a name
//! imported under `TYPE_CHECKING` that the code uses at run time). Behaviour
//! is Ruff 0.16.9's (MIT; read from its source over the network, research
//! only, L5), written fresh: one reading of every import of the file, shared
//! by the four rules, and one edit — carried by the first finding of the
//! file — that moves every import of the kind.
//!
//! A use counts as typing-only when it is in an annotation and the file has
//! `from __future__ import annotations`, in an annotation inside a function
//! body, in a quoted annotation, or under `if TYPE_CHECKING:`; every other use
//! is a run-time use. Strict mode (a module's other imports are checked
//! even when one of them is used at run time) is not read: as Ruff's default,
//! a module with a run-time import keeps its other imports with it.

use crate::python_util::{kids, remove_stmt, statements, PY};
use crate::util::{children, named_children, text};
use crate::{as_suggestion, NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Origin {
    FirstParty,
    ThirdParty,
    Stdlib,
}

/// One name an import statement binds.
struct Binding<'t> {
    stmt: Node<'t>,
    item: Node<'t>,
    module: String,
    local: String,
    /// What the item says: `n`, `n as a`, `m`, `m as a`.
    spec: String,
    from: bool,
    runtime: usize,
    typing: usize,
    in_block: bool,
}

fn is_type_checking_if(n: Node<'_>, src: &str) -> bool {
    n.kind() == "if_statement"
        && field(n, "condition").is_some_and(|c| matches!(text(c, src), "TYPE_CHECKING" | "typing.TYPE_CHECKING"))
        && field(n, "alternative").is_none()
}

fn in_type_checking_block(n: Node<'_>, src: &str) -> bool {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        if is_type_checking_if(p, src) && field(p, "consequence").is_some_and(|c| c.start_byte() <= n.start_byte() && n.end_byte() <= c.end_byte()) {
            return true;
        }
        cur = p;
    }
    false
}

fn has_future_annotations(root: Node<'_>, src: &str) -> bool {
    kids(root).into_iter().any(|s| s.kind() == "future_import_statement" && text(s, src).contains("annotations"))
}

/// Whether the use at `r` (an identifier) is typing-only.
fn typing_only_use(r: Node<'_>, src: &str, future: bool) -> bool {
    if in_type_checking_block(r, src) {
        return true;
    }
    let mut cur = r;
    let mut annotation: Option<Node<'_>> = None;
    while let Some(p) = cur.parent() {
        if p.kind() == "type" {
            annotation = Some(p);
        }
        if matches!(p.kind(), "block" | "module") && annotation.is_some() {
            break;
        }
        cur = p;
    }
    let Some(ann) = annotation else { return false };
    if future {
        return true;
    }
    // Without the future import an annotation runs, unless it is a variable's inside a function, a type alias or a type parameter's.
    let parent = ann.parent();
    if parent.is_some_and(|p| matches!(p.kind(), "type_alias_statement" | "type_parameter" | "constrained_type")) {
        return true;
    }
    if parent.is_some_and(|p| p.kind() == "assignment") {
        let mut q = ann.parent();
        while let Some(x) = q {
            if x.kind() == "function_definition" {
                return true;
            }
            q = x.parent();
        }
    }
    false
}

fn word_in(t: &str, name: &str) -> bool {
    t.match_indices(name).any(|(i, _)| {
        let before = t[..i].chars().next_back().is_none_or(|c| !(c.is_alphanumeric() || c == '_' || c == '.'));
        let after = t[i + name.len()..].chars().next().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
        before && after
    })
}

/// How many quoted annotations of the file name `name`.
fn quoted_uses(root: Node<'_>, src: &str, name: &str) -> usize {
    let mut count = 0;
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "type" && n.parent().is_some_and(|p| p.kind() != "type") {
            let mut inner = vec![n];
            while let Some(x) = inner.pop() {
                if x.kind() == "string" && word_in(text(x, src), name) {
                    count += 1;
                }
                inner.extend(named_children(x));
            }
            continue;
        }
        stack.extend(named_children(n));
    }
    count
}

fn module_of_from<'t>(stmt: Node<'t>, src: &str) -> Option<String> {
    let m = field(stmt, "module_name")?;
    Some(text(m, src).split_whitespace().collect())
}

/// Every import binding of the module: the top-level statements and the
/// ones inside a top-level `if TYPE_CHECKING:`.
fn bindings<'t>(root: Node<'t>, ctx: &NativeContext<'_>) -> Vec<Binding<'t>> {
    let src = ctx.source;
    let Some(model) = ctx.semantic else { return Vec::new() };
    let future = has_future_annotations(root, src);
    let mut out = Vec::new();
    let mut stmts: Vec<(Node<'t>, bool)> = Vec::new();
    for s in kids(root) {
        if matches!(s.kind(), "import_statement" | "import_from_statement") {
            stmts.push((s, false));
        } else if is_type_checking_if(s, src)
            && let Some(b) = field(s, "consequence")
        {
            for i in statements(b) {
                if matches!(i.kind(), "import_statement" | "import_from_statement") {
                    stmts.push((i, true));
                }
            }
        }
    }
    for (stmt, in_block) in stmts {
        let from = stmt.kind() == "import_from_statement";
        let module_from = if from { module_of_from(stmt, src) } else { None };
        if from && module_from.is_none() {
            continue;
        }
        let mut c = stmt.walk();
        let items: Vec<Node<'t>> = if from {
            stmt.children_by_field_name("name", &mut c).collect()
        } else {
            children(stmt).into_iter().filter(|x| matches!(x.kind(), "dotted_name" | "aliased_import")).collect()
        };
        for item in items {
            let (name_node, alias) = if item.kind() == "aliased_import" { (field(item, "name"), field(item, "alias")) } else { (Some(item), None) };
            let Some(name_node) = name_node else { continue };
            let name = text(name_node, src).split_whitespace().collect::<String>();
            if !from && alias.is_none() && name.contains('.') {
                continue;
            }
            let local = alias.map_or_else(|| name.clone(), |a| text(a, src).to_owned());
            let module = if from { module_from.clone().unwrap_or_default() } else { name.clone() };
            let spec = match alias {
                Some(a) => format!("{name} as {}", text(a, src)),
                None => name.clone(),
            };
            // The symbol this item declares.
            let decl_at = alias.map_or_else(|| name_node.named_child(0).map_or(name_node.start_byte(), |i| i.start_byte()), |a| a.start_byte());
            let Some(id) = model.declared_at(decl_at).or_else(|| model.declared_at(name_node.start_byte())) else { continue };
            let sym = model.symbol(id);
            if sym.decls.len() != 1 || sym.exported {
                continue;
            }
            let (mut runtime, mut typing) = (0, 0);
            for r in &sym.refs {
                let r = &model.references[*r];
                let Some(node) = root.descendant_for_byte_range(r.start, r.end) else { continue };
                if typing_only_use(node, src, future) {
                    typing += 1;
                } else {
                    runtime += 1;
                }
            }
            typing += quoted_uses(root, src, &local);
            out.push(Binding { stmt, item, module, local, spec, from, runtime, typing, in_block });
        }
    }
    out
}

fn origin_of(module: &str, relative: bool, ctx: &NativeContext<'_>) -> Origin {
    if relative {
        return Origin::FirstParty;
    }
    let top = module.split('.').next().unwrap_or("");
    let stdlib = ctx.data.get("python/stdlib").and_then(|v| v.as_array().cloned()).is_some_and(|a| a.iter().filter_map(|v| v.as_str()).any(|m| m == top));
    if stdlib {
        return Origin::Stdlib;
    }
    let first = [top.to_owned(), format!("{top}.py"), format!("src/{top}"), format!("src/{top}.py")].iter().any(|p| ctx.file.exists(p));
    if first { Origin::FirstParty } else { Origin::ThirdParty }
}

fn spec_line(from: bool, module: &str, specs: &[String]) -> String {
    if from { format!("from {module} import {}", specs.join(", ")) } else { format!("import {}", specs.join(", ")) }
}

fn line_range(stmt: Node<'_>, src: &str) -> (usize, usize) {
    let start = src[..stmt.start_byte()].rfind('\n').map_or(0, |i| i + 1);
    let e = stmt.end_byte();
    let end = src[e..].find('\n').map_or(src.len(), |i| e + i + 1);
    (start, end)
}

fn alone_on_lines(stmt: Node<'_>, src: &str) -> bool {
    remove_stmt(stmt, src).is_some()
}

/// The text moving `moving` between runtime and a `TYPE_CHECKING` block
/// asks for: removals from each statement, and the lines to write.
type Removal = (Vec<TextEdit>, Vec<(usize, usize)>, Vec<String>);

fn removal_edits(moving: &[&Binding<'_>], src: &str) -> Option<Removal> {
    // Group by statement.
    let mut groups: BTreeMap<usize, Vec<&Binding<'_>>> = BTreeMap::new();
    for b in moving {
        groups.entry(b.stmt.start_byte()).or_default().push(*b);
    }
    let mut edits = Vec::new();
    let mut deleted: Vec<(usize, usize)> = Vec::new();
    let mut lines = Vec::new();
    for bs in groups.values() {
        let stmt = bs[0].stmt;
        if !alone_on_lines(stmt, src) {
            return None;
        }
        let mut c = stmt.walk();
        let all_items: Vec<Node<'_>> = if stmt.kind() == "import_from_statement" {
            stmt.children_by_field_name("name", &mut c).collect()
        } else {
            children(stmt).into_iter().filter(|x| matches!(x.kind(), "dotted_name" | "aliased_import")).collect()
        };
        let specs: Vec<String> = bs.iter().map(|b| b.spec.clone()).collect();
        lines.push(spec_line(bs[0].from, &bs[0].module, &specs));
        if all_items.len() == bs.len() {
            let (mut s, e) = line_range(stmt, src);
            let blank_after = src[e..].is_empty() || src[e..].lines().next().is_some_and(|l| l.trim().is_empty());
            if blank_after {
                while s > 0 {
                    let prev = src[..s - 1].rfind('\n').map_or(0, |i| i + 1);
                    if src[prev..s].trim().is_empty() {
                        s = prev;
                    } else {
                        break;
                    }
                }
            }
            deleted.push((s, e));
            edits.push(TextEdit::new(s, e, ""));
        } else {
            for b in bs {
                let pos = all_items.iter().position(|i| *i == b.item)?;
                if let Some(next) = all_items.get(pos + 1) {
                    edits.push(TextEdit::new(b.item.start_byte(), next.start_byte(), ""));
                } else if pos > 0 {
                    edits.push(TextEdit::new(all_items[pos - 1].end_byte(), b.item.end_byte(), ""));
                }
            }
        }
    }
    Some((edits, deleted, lines))
}

fn last_stmt_end_line<'t>(stmt: Node<'t>, src: &str) -> usize {
    let e = stmt.end_byte();
    src[e..].find('\n').map_or(src.len(), |i| e + i)
}

/// All typing-only bindings of the file, in source order.
fn typing_only<'t>(all: &'t [Binding<'t>]) -> Vec<&'t Binding<'t>> {
    let runtime_modules: BTreeSet<&str> = all.iter().filter(|b| !b.in_block && b.runtime > 0).map(|b| b.module.as_str()).collect();
    all.iter()
        .filter(|b| {
            !b.in_block
                && b.runtime == 0
                && b.typing > 0
                && !matches!(b.module.as_str(), "typing" | "typing_extensions" | "__future__")
                && !runtime_modules.contains(b.module.as_str())
        })
        .collect()
}

/// The edit that moves every typing-only import under `TYPE_CHECKING`.
fn move_under_type_checking(root: Node<'_>, all: &[Binding<'_>], ctx: &NativeContext<'_>) -> Vec<TextEdit> {
    let src = ctx.source;
    let moving = typing_only(all);
    let Some((mut edits, deleted, lines)) = removal_edits(&moving, src) else { return Vec::new() };
    // An existing block takes the lines at its end.
    let block = kids(root).into_iter().find(|s| is_type_checking_if(*s, src));
    if let Some(b) = block
        && let Some(body) = field(b, "consequence")
        && let Some(last) = statements(body).last().copied()
    {
        let ls = src[..last.start_byte()].rfind('\n').map_or(0, |i| i + 1);
        let indent: String = src[ls..].chars().take_while(|c| *c == ' ' || *c == '\t').collect();
        let at = last_stmt_end_line(last, src);
        let text_: String = lines.iter().map(|l| format!("\n{indent}{l}")).collect();
        edits.push(TextEdit::new(at, at, text_));
        return edits;
    }
    // A new block goes below the last import that stays.
    let stays = |s: &Node<'_>| matches!(s.kind(), "import_statement" | "import_from_statement" | "future_import_statement") && !deleted.iter().any(|(a, b)| line_range(*s, src).0 >= *a && line_range(*s, src).1 <= *b);
    let anchor = kids(root).into_iter().rfind(stays);
    // `TYPE_CHECKING` is imported from `typing`, joining an import that is already there.
    let mut tc_import: Option<TextEdit> = None;
    let mut already = false;
    for s in kids(root) {
        if s.kind() == "import_from_statement" && module_of_from(s, src).as_deref() == Some("typing") && stays(&s) {
            let mut c = s.walk();
            let names: Vec<Node<'_>> = s.children_by_field_name("name", &mut c).collect();
            if names.iter().any(|n| text(*n, src) == "TYPE_CHECKING") {
                already = true;
            } else if tc_import.is_none()
                && let Some(l) = names.last()
                && !children(s).iter().any(|c| c.kind() == "wildcard_import")
            {
                tc_import = Some(TextEdit::new(l.end_byte(), l.end_byte(), ", TYPE_CHECKING"));
            }
        }
    }
    let mut block_text = String::new();
    if let Some(e) = tc_import.filter(|_| !already) {
        edits.push(e);
    } else if !already {
        block_text.push_str("from typing import TYPE_CHECKING\n");
    }
    let body: String = lines.iter().map(|l| format!("    {l}\n")).collect();
    match anchor {
        Some(a) => {
            let at = last_stmt_end_line(a, src);
            edits.push(TextEdit::new(at, at, format!("\n\n{block_text}{}if TYPE_CHECKING:\n{}", if block_text.is_empty() { "" } else { "\n" }, body.trim_end_matches('\n'))));
        }
        None => {
            // Every import moves: the block takes the place of the first one.
            let Some(first) = deleted.first().copied() else { return Vec::new() };
            edits.retain(|e| !(e.start == first.0 && e.end == first.1 && e.text.is_empty()));
            edits.push(TextEdit::new(first.0, first.1, format!("{block_text}{}if TYPE_CHECKING:\n{body}", if block_text.is_empty() { "" } else { "\n" })));
        }
    }
    edits
}

fn tc001_3(root: Node<'_>, ctx: &NativeContext<'_>, want: Origin, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let all = bindings(root, ctx);
    let moving = typing_only(&all);
    let Some(first) = moving.first() else { return };
    let carrier = first.item.start_byte();
    let mut plan: Option<Vec<TextEdit>> = None;
    for b in &moving {
        let relative = b.from && field(b.stmt, "module_name").is_some_and(|m| m.kind() == "relative_import");
        if origin_of(&b.module, relative, ctx) != want {
            continue;
        }
        let edits = if b.item.start_byte() == carrier {
            plan.get_or_insert_with(|| move_under_type_checking(root, &all, ctx)).clone()
        } else {
            // The carrier of another origin holds the edit when this one is not first.
            Vec::new()
        };
        out.push(NativeFinding {
            start: b.item.start_byte(),
            end: b.item.end_byte(),
            message: format!("Move the {} import `{}` into a type-checking block", match want {
                Origin::FirstParty => "application",
                Origin::ThirdParty => "third-party",
                Origin::Stdlib => "standard-library",
            }, b.local),
            edits: as_suggestion(edits),
        });
    }
    // When the first binding is of another origin, its rule's finding holds the edit; this rule's
    // findings only report.
    let _ = src;
}

fn tc004(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let all = bindings(root, ctx);
    let moving: Vec<&Binding<'_>> = all.iter().filter(|b| b.in_block && b.runtime > 0).collect();
    let Some(first) = moving.first() else { return };
    let carrier = first.item.start_byte();
    for b in &moving {
        let mut edits = Vec::new();
        if b.item.start_byte() == carrier
            && let Some((mut e, deleted, lines)) = removal_edits(&moving, src)
        {
            // The block loses the statement: `pass` keeps it valid when it was the only one.
            for (s, en) in &deleted {
                let stmt_in_block = moving.iter().find(|m| line_range(m.stmt, src) == (*s, *en)).map(|m| m.stmt);
                if let Some(st) = stmt_in_block
                    && let Some(block) = st.parent()
                    && statements(block).len() == 1
                {
                    let indent: String = src[*s..].chars().take_while(|c| *c == ' ' || *c == '\t').collect();
                    e.retain(|x| !(x.start == *s && x.end == *en));
                    e.push(TextEdit::new(*s, *en, format!("{indent}pass\n")));
                }
            }
            // The import goes below the last top-level import that is not in a block.
            let anchor = kids(root).into_iter().rfind(|s| matches!(s.kind(), "import_statement" | "import_from_statement" | "future_import_statement"));
            let at = anchor.map_or(0, |a| last_stmt_end_line(a, src));
            let text_: String = lines.iter().map(|l| format!("\n{l}")).collect();
            if anchor.is_some() {
                e.push(TextEdit::new(at, at, text_));
                edits = e;
            }
        }
        out.push(NativeFinding { start: b.item.start_byte(), end: b.item.end_byte(), message: format!("Move the import `{}` out of the type-checking block: it is used at run time", b.local), edits: as_suggestion(edits) });
    }
}


// ------------------------------------------------------------------ TC007

/// The names of a type alias's value that need quotes: the imports under
/// `TYPE_CHECKING` it names outside a `Literal[...]`.
fn alias_names<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>, src: &str) {
    match n.kind() {
        "identifier" => out.push(n),
        "binary_operator" => {
            for c in [field(n, "left"), field(n, "right")].into_iter().flatten() {
                alias_names(c, out, src);
            }
        }
        "attribute" => {
            if let Some(o) = field(n, "object") {
                alias_names(o, out, src);
            }
        }
        "subscript" => {
            let Some(v) = field(n, "value") else { return };
            alias_names(v, out, src);
            if text(v, src).rsplit('.').next() == Some("Literal") {
                return;
            }
            let mut c = n.walk();
            let args: Vec<Node<'t>> = n.children_by_field_name("subscript", &mut c).collect();
            let annotated = text(v, src).rsplit('.').next() == Some("Annotated");
            for (i, a) in args.into_iter().enumerate() {
                if annotated && i > 0 {
                    break;
                }
                alias_names(a, out, src);
            }
        }
        "list" | "tuple" | "expression_list" => {
            for c in kids(n) {
                alias_names(c, out, src);
            }
        }
        _ => {}
    }
}

fn tc007(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(model) = ctx.semantic else { return };
    let root = {
        let mut c = n;
        while let Some(p) = c.parent() {
            c = p;
        }
        c
    };
    let (Some(ann), Some(value)) = (field(n, "type"), field(n, "right")) else { return };
    if !text(ann, src).trim().ends_with("TypeAlias") || value.kind() == "string" {
        return;
    }
    let future = has_future_annotations(root, src);
    let mut names = Vec::new();
    alias_names(value, &mut names, src);
    let mut needing: Vec<Node<'_>> = Vec::new();
    for id in names {
        let Some(sym_id) = model.symbol_at(id.start_byte()) else { continue };
        let sym = model.symbol(sym_id);
        if sym.kind != codelaw_semantic::SymbolKind::Import || sym.decls.len() != 1 {
            continue;
        }
        let (s, e) = sym.decls[0];
        let Some(decl) = root.descendant_for_byte_range(s, e) else { continue };
        if !in_type_checking_block(decl, src) {
            continue;
        }
        // Another use that runs is the other rule's: the two would chase each other.
        let runtime_elsewhere = sym.refs.iter().any(|r| {
            let r = &model.references[*r];
            let inside = r.start >= n.start_byte() && r.end <= n.end_byte();
            !inside && root.descendant_for_byte_range(r.start, r.end).is_some_and(|x| !typing_only_use(x, src, future))
        });
        if !runtime_elsewhere {
            needing.push(id);
        }
    }
    if needing.is_empty() {
        return;
    }
    let vt = text(value, src);
    let edit = if vt.contains(['\'', '"', '\n', '\\']) || value.kind() == "parenthesized_expression" { Vec::new() } else { vec![TextEdit::new(value.start_byte(), value.end_byte(), format!("\"{vt}\""))] };
    // Every name needing quotes is reported; the edit quotes the whole value, and one finding carries it.
    for (i, id) in needing.iter().enumerate() {
        let e = if i == 0 { edit.clone() } else { Vec::new() };
        out.push(NativeFinding { start: id.start_byte(), end: id.end_byte(), message: format!("Add quotes to the type alias: `{}` is only imported under `TYPE_CHECKING`", text(*id, src)), edits: as_suggestion(e) });
    }
}

// ------------------------------------------------------------------ TC010

fn tc010(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let future = has_future_annotations(root, src);
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "union_type" || (n.kind() == "binary_operator" && field(n, "operator").is_some_and(|o| text(o, src) == "|") && n.parent().is_some_and(|p| p.kind() != "binary_operator")) {
            // The strings directly joined by `|`.
            let mut strings: Vec<Node<'_>> = Vec::new();
            let mut walk = vec![n];
            while let Some(x) = walk.pop() {
                match x.kind() {
                    "string" | "concatenated_string" => strings.push(x),
                    "type" | "union_type" | "binary_operator" => walk.extend(named_children(x)),
                    _ => {}
                }
            }
            let in_annotation = {
                let mut c = n.parent();
                let mut found = n.kind() == "union_type";
                while let Some(p) = c {
                    if p.kind() == "type" {
                        found = true;
                    }
                    c = p.parent();
                }
                found
            };
            if in_annotation && !strings.is_empty() && !typing_only_use(n, src, future) {
                for s in strings {
                    out.push(NativeFinding { start: s.start_byte(), end: s.end_byte(), message: "Invalid string member in `X | Y`-style union type".into(), edits: Vec::new() });
                }
            }
        }
        stack.extend(named_children(n));
    }
}

// ------------------------------------------------------------------ TC008

/// Every string a type alias's value reads as a forward reference: the
/// strings joined by `|`, in a subscript's arguments, in a list or tuple of
/// them — not inside `Literal[...]`, not after the first argument of
/// `Annotated[...]`.
fn alias_strings<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>, src: &str) {
    match n.kind() {
        "string" | "concatenated_string" => out.push(n),
        "type" | "union_type" | "parenthesized_expression" | "list" | "tuple" | "expression_list" | "type_parameter" => {
            for c in named_children(n) {
                alias_strings(c, out, src);
            }
        }
        "binary_operator" => {
            if field(n, "operator").is_some_and(|o| text(o, src) == "|") {
                for c in [field(n, "left"), field(n, "right")].into_iter().flatten() {
                    alias_strings(c, out, src);
                }
            }
        }
        "generic_type" | "subscript" => {
            let kids_: Vec<Node<'t>> = named_children(n);
            let Some(head) = kids_.first().copied() else { return };
            let head_text = text(head, src);
            let last = head_text.rsplit('.').next().unwrap_or("");
            if last == "Literal" {
                return;
            }
            let args: Vec<Node<'t>> = kids_.into_iter().skip(1).collect();
            for (i, a) in args.into_iter().enumerate() {
                if last == "Annotated" && i > 0 && n.kind() == "subscript" {
                    break;
                }
                alias_strings(a, out, src);
            }
            if last == "Annotated" && n.kind() == "generic_type" {
                // `Annotated[X, "meta"]` in a `type` statement: the first type argument only.
                let mut first_only: Vec<Node<'t>> = Vec::new();
                if let Some(tp) = named_children(n).into_iter().find(|c| c.kind() == "type_parameter")
                    && let Some(f) = named_children(tp).first().copied()
                {
                    alias_strings(f, &mut first_only, src);
                }
                out.retain(|o| !(o.start_byte() >= n.start_byte() && o.end_byte() <= n.end_byte()));
                out.extend(first_only);
            }
        }
        _ => {}
    }
}

/// The names a statement binds at run time in its own scope.
fn binds<'t>(stmt: Node<'t>, name: &str, src: &str, out: &mut Vec<Node<'t>>) {
    let mut names: Vec<String> = Vec::new();
    fn targets(n: Node<'_>, src: &str, names: &mut Vec<String>) {
        match n.kind() {
            "identifier" => names.push(text(n, src).to_owned()),
            "pattern_list" | "tuple_pattern" | "list_pattern" | "tuple" | "list" | "expression_list" | "list_splat_pattern" | "parenthesized_expression" => {
                for c in named_children(n) {
                    targets(c, src, names);
                }
            }
            _ => {}
        }
    }
    match stmt.kind() {
        "import_statement" | "import_from_statement" => {
            let mut c = stmt.walk();
            let items: Vec<Node<'t>> = if stmt.kind() == "import_from_statement" { stmt.children_by_field_name("name", &mut c).collect() } else { children(stmt).into_iter().filter(|x| matches!(x.kind(), "dotted_name" | "aliased_import")).collect() };
            for it in items {
                if it.kind() == "aliased_import" {
                    if let Some(a) = field(it, "alias") {
                        names.push(text(a, src).to_owned());
                    }
                } else {
                    let t: String = text(it, src).split_whitespace().collect();
                    names.push(if stmt.kind() == "import_statement" { t.split('.').next().unwrap_or("").to_owned() } else { t });
                }
            }
        }
        "expression_statement" => {
            for c in named_children(stmt) {
                if c.kind() == "assignment" && field(c, "right").is_some() {
                    if let Some(l) = field(c, "left") {
                        targets(l, src, &mut names);
                    }
                } else if c.kind() == "assignment" {
                    // `x: int` binds nothing at run time.
                }
            }
        }
        "function_definition" | "class_definition" => {
            if let Some(n) = field(stmt, "name") {
                names.push(text(n, src).to_owned());
            }
        }
        "for_statement" => {
            if let Some(l) = field(stmt, "left") {
                targets(l, src, &mut names);
            }
        }
        _ => {}
    }
    if names.iter().any(|n| n == name) {
        out.push(stmt);
    }
}

/// Every statement of the block, and of the compound statements in it, that
/// binds `name` in the block's own scope (not in a nested function or class).
fn scope_bindings<'t>(block: Node<'t>, name: &str, src: &str, out: &mut Vec<Node<'t>>) {
    for s in named_children(block) {
        match s.kind() {
            "decorated_definition" => {
                if let Some(d) = field(s, "definition") {
                    binds(d, name, src, out);
                }
            }
            "if_statement" | "try_statement" | "with_statement" | "for_statement" | "while_statement" | "elif_clause" | "else_clause" | "except_clause" | "finally_clause" | "block" | "match_statement" | "case_clause" => {
                binds(s, name, src, out);
                scope_bindings(s, name, src, out);
            }
            _ => binds(s, name, src, out),
        }
    }
}

/// Whether `stmt` sits in the `if TYPE_CHECKING:` branch of an `if`.
fn under_type_checking(stmt: Node<'_>, src: &str) -> bool {
    let mut cur = stmt;
    while let Some(p) = cur.parent() {
        if p.kind() == "if_statement"
            && field(p, "condition").is_some_and(|c| matches!(text(c, src), "TYPE_CHECKING" | "typing.TYPE_CHECKING"))
            && field(p, "consequence").is_some_and(|c| c.start_byte() <= stmt.start_byte() && stmt.end_byte() <= c.end_byte())
        {
            return true;
        }
        cur = p;
    }
    false
}

/// What an alias statement needs to know to judge a name in its value.
struct AliasSite<'t> {
    stmt: Node<'t>,
    src: &'t str,
    in_tc: bool,
}

impl AliasSite<'_> {
    /// A name the file binds, but not yet — or never — at run time, where
    /// the alias's value runs.
    fn name_is_unavailable(&self, name: &str) -> bool {
        let mut scopes: Vec<Node<'_>> = Vec::new();
        let mut in_function = false;
        let mut cur = self.stmt.parent();
        while let Some(p) = cur {
            match p.kind() {
                "function_definition" => in_function = true,
                "class_definition" | "module" => {
                    if let Some(b) = field(p, "body").filter(|_| p.kind() == "class_definition") {
                        scopes.push(b);
                    } else if p.kind() == "module" {
                        scopes.push(p);
                    }
                }
                _ => {}
            }
            cur = p.parent();
        }
        let mut any = false;
        for sc in scopes {
            let mut found = Vec::new();
            scope_bindings(sc, name, self.src, &mut found);
            for b in found {
                any = true;
                let before = in_function || b.end_byte() <= self.stmt.start_byte();
                if before && (self.in_tc || !under_type_checking(b, self.src)) {
                    return false;
                }
            }
        }
        any
    }

    /// Ruff's reading of whether quoting a value is required: a subscript or
    /// attribute outside a `TYPE_CHECKING` block may not exist at run time,
    /// and a name may not yet be bound.
    fn unremovable(&self, e: Node<'_>, src2: &str) -> bool {
        match e.kind() {
            "binary_operator" => {
                if field(e, "operator").is_some_and(|o| text(o, src2) == "|") {
                    [field(e, "left"), field(e, "right")].into_iter().flatten().any(|c| self.unremovable(c, src2))
                } else {
                    true
                }
            }
            "list_splat" | "parenthesized_expression" => named_children(e).into_iter().any(|c| self.unremovable(c, src2)),
            "subscript" => {
                if !self.in_tc {
                    return true;
                }
                let Some(value) = field(e, "value") else { return true };
                if self.unremovable(value, src2) {
                    return true;
                }
                let mut c = e.walk();
                let args: Vec<Node<'_>> = e.children_by_field_name("subscript", &mut c).collect();
                if text(value, src2).rsplit('.').next() == Some("Annotated") {
                    return args.first().is_some_and(|a| args.len() > 1 && self.unremovable(*a, src2));
                }
                args.into_iter().any(|a| self.unremovable(a, src2))
            }
            "attribute" => {
                if !self.in_tc {
                    return true;
                }
                field(e, "object").is_some_and(|o| self.unremovable(o, src2))
            }
            "list" | "tuple" | "expression_list" => named_children(e).into_iter().any(|c| self.unremovable(c, src2)),
            "identifier" => self.name_is_unavailable(text(e, src2)),
            _ => false,
        }
    }
}

fn tc008(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let explicit = n.kind() == "assignment";
    let value = if explicit {
        let (Some(ann), Some(v)) = (field(n, "type"), field(n, "right")) else { return };
        if !text(ann, src).trim().ends_with("TypeAlias") {
            return;
        }
        v
    } else {
        let Some(v) = field(n, "right") else { return };
        v
    };
    let stmt = if explicit { n.parent().filter(|p| p.kind() == "expression_statement").unwrap_or(n) } else { n };
    let site = AliasSite { stmt, src, in_tc: under_type_checking(stmt, src) };
    let mut strings = Vec::new();
    alias_strings(value, &mut strings, src);
    for s in strings {
        let Some(content) = crate::python_util::string_value(s, src) else { continue };
        if content.trim().is_empty() {
            continue;
        }
        if explicit {
            let wrapped = format!("({content}\n)");
            let tree = codelaw_lang::parse(Language::Python, &wrapped);
            let root = tree.root();
            let r = root.get_inner_node();
            if r.has_error() {
                continue;
            }
            let Some(expr) = r.named_child(0).and_then(|st| st.named_child(0)).filter(|p| p.kind() == "parenthesized_expression").and_then(|p| p.named_child(0)) else { continue };
            if site.unremovable(expr, &wrapped) {
                continue;
            }
        }
        let multiline = content.contains(['\n', '\r']);
        let parenthesized_outside = s.parent().is_some_and(|p| p.kind() == "parenthesized_expression");
        let parenthesized_inside = {
            let t = content.trim();
            t.starts_with('(') && t.ends_with(')') && {
                let wrapped = format!("({content}\n)");
                let tree = codelaw_lang::parse(Language::Python, &wrapped);
                let r = tree.root().get_inner_node();
                r.named_child(0).and_then(|st| st.named_child(0)).and_then(|p| p.named_child(0)).is_some_and(|e| e.kind() == "parenthesized_expression")
            }
        };
        let fixed = if multiline && !parenthesized_outside && !parenthesized_inside { format!("({content})") } else { content.clone() };
        let edit = vec![TextEdit::new(s.start_byte(), s.end_byte(), fixed)];
        let has_comment = children(s).iter().any(|c| c.kind() == "comment");
        out.push(NativeFinding { start: s.start_byte(), end: s.end_byte(), message: "Remove quotes from type alias".into(), edits: if has_comment { as_suggestion(edit) } else { edit } });
    }
}

pub struct QuotedTypeAlias;
impl NativeRule for QuotedTypeAlias {
    fn id(&self) -> &'static str {
        "py/quoted-type-alias"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["assignment", "type_alias_statement"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        tc008(node.get_inner_node(), ctx, out)
    }
}

macro_rules! tc_rule {
    ($name:ident, $id:literal, $f:expr) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &["module"]
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                let n = node.get_inner_node();
                let f: fn(Node<'_>, &NativeContext<'_>, &mut Vec<NativeFinding>) = $f;
                f(n, ctx, out)
            }
        }
    };
}

tc_rule!(TypingOnlyFirstPartyImport, "py/typing-only-first-party-import", |n, ctx, out| tc001_3(n, ctx, Origin::FirstParty, out));
tc_rule!(TypingOnlyThirdPartyImport, "py/typing-only-third-party-import", |n, ctx, out| tc001_3(n, ctx, Origin::ThirdParty, out));
tc_rule!(TypingOnlyStandardLibraryImport, "py/typing-only-standard-library-import", |n, ctx, out| tc001_3(n, ctx, Origin::Stdlib, out));
tc_rule!(RuntimeImportInTypeCheckingBlock, "py/runtime-import-in-type-checking-block", tc004);
tc_rule!(RuntimeStringUnion, "py/runtime-string-union", tc010);

pub struct UnquotedTypeAlias;
impl NativeRule for UnquotedTypeAlias {
    fn id(&self) -> &'static str {
        "py/unquoted-type-alias"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["assignment"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        tc007(node.get_inner_node(), ctx, out)
    }
}

pub static RULES: &[&dyn NativeRule] = &[&TypingOnlyFirstPartyImport, &TypingOnlyThirdPartyImport, &TypingOnlyStandardLibraryImport, &RuntimeImportInTypeCheckingBlock, &RuntimeStringUnion, &UnquotedTypeAlias, &QuotedTypeAlias];

#[cfg(test)]
mod tests {
    use super::*;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        let root = codelaw_lang::parse(Language::Python, src);
        let model = codelaw_semantic::build(Language::Python, &root, src);
        let options = BTreeMap::new();
        let mut data = BTreeMap::new();
        data.insert("python/stdlib".to_owned(), std::sync::Arc::new(serde_json::json!(["os", "pathlib", "json"])));
        let file = crate::DetachedFile::new("example.py", src);
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
    fn typing_only() {
        let src = "from __future__ import annotations\n\nfrom pathlib import Path\n\n\ndef f(p: Path) -> None: ...\n";
        let r = run(&TypingOnlyStandardLibraryImport, src);
        assert_eq!(r.len(), 1);
        assert!(!r[0].edits.is_empty());
        // A run-time use keeps the import where it is.
        let src = "from __future__ import annotations\n\nfrom pathlib import Path\n\n\ndef f(p: Path) -> None:\n    return Path()\n";
        assert!(run(&TypingOnlyStandardLibraryImport, src).is_empty());
        // Without the future import the annotation runs.
        let src = "from pathlib import Path\n\n\ndef f(p: Path) -> None: ...\n";
        assert!(run(&TypingOnlyStandardLibraryImport, src).is_empty());
    }

    #[test]
    fn runtime_in_block() {
        let src = "from typing import TYPE_CHECKING\n\nif TYPE_CHECKING:\n    from pathlib import Path\n\nx = Path()\n";
        assert_eq!(run(&RuntimeImportInTypeCheckingBlock, src).len(), 1);
    }

    /// The lines of a file that `py/quoted-type-alias` reports, one-based.
    fn lines_of_findings(src: &str) -> Vec<usize> {
        run(&QuotedTypeAlias, src).iter().map(|f| src[..f.start].matches('\n').count() + 1).collect()
    }

    #[test]
    fn quoted_type_alias_rows() {
        let src = "from __future__ import annotations\n\nfrom typing import TypeAlias, TYPE_CHECKING\n\nfrom foo import Foo\n\nif TYPE_CHECKING:\n    from typing import Dict\n\n    OptStr: TypeAlias = str | None\n    Bar: TypeAlias = Foo[int]\nelse:\n    Bar = Foo\n\na: TypeAlias = 'int'\nb: TypeAlias = 'Dict'\nc: TypeAlias = 'Foo'\nd: TypeAlias = 'Foo[str]'\ne: TypeAlias = 'Foo.bar'\nf: TypeAlias = 'Foo | None'\ng: TypeAlias = 'OptStr'\nh: TypeAlias = 'Bar'\ni: TypeAlias = Foo['str']\nj: TypeAlias = 'Baz'\nk: TypeAlias = 'k | None'\n\n\nclass Baz:\n    a: TypeAlias = 'Baz'\n    type A = 'Baz'\n";
        assert_eq!(lines_of_findings(src), vec![15, 17, 20, 22, 23, 30]);
    }

    #[test]
    fn quoted_type_alias_statement_and_multiline() {
        let src = "type B = 'Dict'\ntype I = Foo['str']\ntype K = 'K | None'\ntype L = Literal['x']\n";
        assert_eq!(lines_of_findings(src), vec![1, 2, 3]);
        let o = "from typing import TypeAlias\no: TypeAlias = \"\"\"int\n| None\"\"\"\np: TypeAlias = (\"\"\"int\n| None\"\"\")\n";
        let r = run(&QuotedTypeAlias, o);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].edits[0].text, "(int\n| None)");
        assert_eq!(r[1].edits[0].text, "int\n| None");
    }
}

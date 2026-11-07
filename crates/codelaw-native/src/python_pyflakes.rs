//! Native rules behind `py/recommended`'s Pyflakes ports (Ruff's `F`
//! rules): imports, names, scopes and statements in the wrong place.

use crate::NativeContext;
use crate::NativeFinding;
use crate::python_util::{
    call_args, decorator_target, decorators, hit, hit_fix, is_docstring_stmt, is_fstring, is_pure, kids,
    names_one_of, py_rule, remove_stmt, starts_line, statement_of, statements, string_parts, string_value, unparen,

};
use crate::util::{children, text};
use codelaw_core::TextEdit;
use codelaw_semantic::{Imported, ScopeKind, SemanticModel, SymbolId, SymbolKind};
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

/// How an identifier came to be bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Bind {
    Import,
    Def,
    Class,
    Assign,
    AnnOnly,
    Aug,
    Loop,
    With,
    Except,
    Walrus,
    Other,
}

/// The binding context of a declaring identifier, and the node that
/// binds it (the assignment, `for`, `as` pattern, …).
pub(crate) fn bind_ctx(id: Node<'_>) -> (Bind, Option<Node<'_>>) {
    let mut cur = id;
    while let Some(p) = cur.parent() {
        let kind = match p.kind() {
            "function_definition" => {
                if p.child_by_field_name("name") == Some(cur) {
                    Bind::Def
                } else {
                    Bind::Other
                }
            }
            "class_definition" => {
                if p.child_by_field_name("name") == Some(cur) {
                    Bind::Class
                } else {
                    Bind::Other
                }
            }
            "dotted_name" | "aliased_import" | "pattern_list" | "tuple_pattern" | "list_pattern" | "tuple" | "list"
            | "parenthesized_expression" | "list_splat_pattern" | "list_splat" | "as_pattern_target" | "expression_list" => {
                cur = p;
                continue;
            }
            "import_statement" | "import_from_statement" | "future_import_statement" => Bind::Import,
            "assignment" => {
                if p.child_by_field_name("left") == Some(cur) {
                    if p.child_by_field_name("right").is_none() { Bind::AnnOnly } else { Bind::Assign }
                } else {
                    Bind::Other
                }
            }
            "augmented_assignment" => Bind::Aug,
            "for_statement" | "for_in_clause" => Bind::Loop,
            "as_pattern" => match p.parent().map(|g| g.kind()) {
                Some("with_item") => Bind::With,
                Some("except_clause") => Bind::Except,
                _ => Bind::Other,
            },
            "named_expression" => Bind::Walrus,
            _ => Bind::Other,
        };
        return (kind, Some(p));
    }
    (Bind::Other, None)
}

/// The innermost branch a node is in: a block of an `if`/`elif`/`else`,
/// a `try` clause or a `case`, within the same function.
fn branch(n: Node<'_>) -> Option<Node<'_>> {
    let mut cur = statement_of(n);
    while let Some(p) = cur.parent() {
        match p.kind() {
            "if_statement" | "elif_clause" | "else_clause" | "except_clause" | "finally_clause" | "case_clause" | "try_statement"
                if cur.kind() == "block" =>
            {
                return Some(cur);
            }
            "function_definition" | "class_definition" | "lambda" => return None,
            _ => {}
        }
        cur = p;
    }
    None
}

fn ident_at(root: Node<'_>, start: usize, end: usize) -> Option<Node<'_>> {
    root.descendant_for_byte_range(start, end).filter(|n| n.kind() == "identifier")
}

fn line_of(src: &str, offset: usize) -> usize {
    crate::util::line_of(src, offset)
}

/// Whether two nodes sit in branches of one `if`, `try` or `match` that
/// can't both run.
pub(crate) fn different_forks(a: Node<'_>, b: Node<'_>) -> bool {
    let mut path_a = Vec::new();
    let mut cur = Some(a);
    while let Some(x) = cur {
        path_a.push(x);
        cur = x.parent();
    }
    let mut child_b = b;
    let mut cur = b.parent();
    while let Some(x) = cur {
        if let Some(i) = path_a.iter().position(|n| n.id() == x.id()) {
            if i == 0 {
                return false;
            }
            let child_a = path_a[i - 1];
            if child_a.id() == child_b.id() {
                return false;
            }
            return match x.kind() {
                "if_statement" => true,
                "try_statement" => {
                    let group = |c: Node<'_>| if c.kind() == "except_clause" { c.start_byte() + 1 } else { 0 };
                    group(child_a) != group(child_b)
                }
                "block" => x.parent().is_some_and(|p| p.kind() == "match_statement"),
                _ => false,
            };
        }
        child_b = x;
        cur = x.parent();
    }
    false
}

/// Names that appear inside string annotations (`x: "Foo"`), which use the
/// imports they name.
fn names_in_string_annotations(root: Node<'_>, src: &str, model: &SemanticModel) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        // `# type:` comments are not read: Ruff 0.17 doesn't count them as uses either.
        if n.kind() == "string" && n.parent().is_some() && (has_type_ancestor(n) || in_typing_context(n, model, src)) {
            if let Some((_, content)) = string_parts(n, src) {
                collect_words(&content, &mut out);
            }
            continue;
        }
        stack.extend(kids(n));
    }
    out
}

/// Whether a string sits where typing reads it as a type: inside a
/// subscript of a typing generic (`Dict["Name", int]`), as `cast()`'s first
/// argument, or as a `TypeVar`'s `bound=`.
fn in_typing_context(n: Node<'_>, model: &SemanticModel, src: &str) -> bool {
    const GENERIC_BUILTINS: &[&str] = &["builtins.list", "builtins.dict", "builtins.set", "builtins.frozenset", "builtins.tuple", "builtins.type"];
    let typing = |q: &str| q.starts_with("typing.") || q.starts_with("typing_extensions.") || q.starts_with("collections.abc.") || GENERIC_BUILTINS.contains(&q);
    // `X: TypeAlias = "Decimal | int"`: the value of an explicit alias is a type.
    if let Some(a) = n.parent().filter(|p| p.kind() == "assignment" && p.child_by_field_name("right").is_some_and(|r| r.id() == n.id()))
        && let Some(t) = a.child_by_field_name("type")
        && let Some(e) = t.named_child(0).or(Some(t))
        && crate::python_util::qualified_name(e, model, src).is_some_and(|q| q == "typing.TypeAlias" || q == "typing_extensions.TypeAlias")
    {
        return true;
    }
    let mut cur = n;
    while let Some(p) = cur.parent() {
        match p.kind() {
            "subscript" => {
                if p.child_by_field_name("value").and_then(|v| crate::python_util::qualified_name(v, model, src)).is_some_and(|q| typing(&q)) {
                    return true;
                }
            }
            "argument_list" => {
                let Some(call) = p.parent() else { return false };
                let q = call.child_by_field_name("function").and_then(|f| crate::python_util::qualified_name(f, model, src)).unwrap_or_default();
                let first = kids(p).first().is_some_and(|f| f.id() == cur.id());
                if (q.ends_with(".cast") && typing(&q) && first) || (cur.kind() == "keyword_argument" && q.ends_with(".TypeVar")) {
                    return true;
                }
                return false;
            }
            "keyword_argument" => {
                if p.child_by_field_name("name").is_none_or(|k| text(k, src) != "bound") {
                    return false;
                }
            }
            "block" | "module" | "lambda" | "function_definition" | "class_definition" => return false,
            _ => {}
        }
        cur = p;
    }
    false
}

fn has_type_ancestor(n: Node<'_>) -> bool {
    let mut p = n.parent();
    while let Some(x) = p {
        if x.kind() == "type" {
            return true;
        }
        if matches!(x.kind(), "block" | "module") {
            return false;
        }
        p = x.parent();
    }
    false
}

fn collect_words(s: &str, out: &mut BTreeSet<String>) {
    let mut word = String::new();
    for c in s.chars().chain(std::iter::once(' ')) {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
        } else if !word.is_empty() {
            if !word.chars().next().is_some_and(|f| f.is_ascii_digit()) {
                out.insert(std::mem::take(&mut word));
            }
            word.clear();
        }
    }
}

/// The import entry node (`dotted_name` / `aliased_import`) of an import.
fn import_entry<'t>(root: Node<'t>, start: usize, end: usize) -> Option<Node<'t>> {
    let mut n = root.descendant_for_byte_range(start, end)?;
    while n.start_byte() == start && n.end_byte() == end {
        if matches!(n.parent().map(|p| p.kind()), Some("import_statement" | "import_from_statement" | "future_import_statement")) {
            return Some(n);
        }
        n = n.parent()?;
    }
    None
}

/// Edits that remove one name from an import statement, with its comma.
fn remove_entry(entry: Node<'_>, src: &str) -> Option<Vec<TextEdit>> {
    let next = entry.next_sibling();
    if let Some(comma) = next.filter(|c| c.kind() == ",") {
        let after = comma.next_sibling()?;
        let line_end = src[comma.end_byte()..].find('\n').map(|i| comma.end_byte() + i);
        if starts_line(src, entry.start_byte())
            && let Some(le) = line_end
            && src[comma.end_byte()..le].trim().is_empty()
        {
            let ls = src[..entry.start_byte()].rfind('\n').map(|i| i + 1).unwrap_or(0);
            return Some(vec![TextEdit::new(ls, le + 1, "")]);
        }
        let mut stop = comma.end_byte();
        while src[stop..].starts_with(' ') {
            stop += 1;
        }
        let stop = stop.min(after.start_byte().max(comma.end_byte()));
        return Some(vec![TextEdit::new(entry.start_byte(), stop, "")]);
    }
    let prev = entry.prev_sibling().filter(|c| c.kind() == ",")?;
    Some(vec![TextEdit::new(prev.start_byte(), entry.end_byte(), "")])
}

fn qualified_import(model: &SemanticModel, i: usize) -> String {
    let imp = &model.imports[i];
    match &imp.imported {
        Imported::Named(x) => {
            if imp.source.ends_with('.') { format!("{}{}", imp.source, x) } else { format!("{}.{}", imp.source, x) }
        }
        _ => imp.source.clone(),
    }
}

/// The last declaration of a symbol, by position.
fn last_decl(model: &SemanticModel, id: SymbolId) -> (usize, usize) {
    model.symbol(id).decls.iter().copied().max().unwrap_or((0, 0))
}

fn inside_try_body(n: Node<'_>) -> bool {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        if p.kind() == "try_statement" && p.child_by_field_name("body") == Some(cur) {
            return true;
        }
        if matches!(p.kind(), "function_definition" | "class_definition") {
            return false;
        }
        cur = p;
    }
    false
}

fn unused_import(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let in_annotations = names_in_string_annotations(root, src, model);
    // Statement start → (entries, unused entries).
    let mut by_stmt: BTreeMap<usize, (usize, Vec<usize>)> = BTreeMap::new();
    let mut unused = Vec::new();
    for (i, imp) in model.imports.iter().enumerate() {
        let Some(entry) = import_entry(root, imp.start, imp.end) else { continue };
        let stmt = entry.parent().expect("an entry has a statement");
        let slot = by_stmt.entry(stmt.start_byte()).or_insert((0, Vec::new()));
        slot.0 += 1;
        let Some(sym) = imp.local else { continue };
        let s = model.symbol(sym);
        if s.implicitly_used || !model.is_unused(sym) || in_annotations.contains(&s.name) || s.name == "__all__" {
            continue;
        }
        // An import in a class body defines a class attribute.
        if model.scopes[s.scope].kind == ScopeKind::Class {
            continue;
        }
        // `import a as x` plus `import a.b`: the second loads a submodule `x.b` uses.
        if imp.imported == Imported::Module && imp.source.contains('.') && !src[imp.start..imp.end].contains(" as ") {
            let root = imp.source.split('.').next().unwrap_or("");
            let aliased = model.imports.iter().any(|o| o.imported == Imported::Module && o.source.split('.').next() == Some(root) && src[o.start..o.end].contains(" as "));
            if aliased {
                continue;
            }
        }
        let decl = s.decls.iter().copied().find(|d| d.0 >= imp.start && d.1 <= imp.end);
        if decl.is_none_or(|d| d != last_decl(model, sym)) {
            continue;
        }
        slot.1.push(i);
        unused.push((i, entry, stmt));
    }
    for (i, entry, stmt) in unused {
        let (total, ref gone) = by_stmt[&stmt.start_byte()];
        let edits = if inside_try_body(stmt) {
            None
        } else if gone.len() == total {
            // When every statement of a block goes, the block keeps a `pass`.
            let whole_block = stmt.parent().filter(|b| b.kind() == "block").and_then(|b| {
                let stmts = statements(b);
                let all_gone = stmts.iter().all(|x| by_stmt.get(&x.start_byte()).is_some_and(|(t, g)| *t == g.len() && *t > 0));
                all_gone.then(|| vec![TextEdit::new(stmts[0].start_byte(), stmts[stmts.len() - 1].end_byte(), "pass")])
            });
            whole_block.or_else(|| remove_stmt(stmt, src))
        } else {
            remove_entry(entry, src)
        };
        out.push(hit_fix(entry, format!("`{}` is imported but never used", qualified_import(model, i)), edits.unwrap_or_default()));
    }
}
py_rule!(UnusedImport, "py/unused-import", ["module"], true, unused_import);

/// The identifiers a target binds.
fn target_names<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    match n.kind() {
        "identifier" => out.push(n),
        "pattern_list" | "tuple_pattern" | "list_pattern" | "tuple" | "list" | "parenthesized_expression" | "list_splat_pattern"
        | "list_splat" => {
            for c in kids(n) {
                target_names(c, out);
            }
        }
        _ => {}
    }
}

fn import_shadowed_by_loop_var(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let Some(left) = n.child_by_field_name("left") else { return };
    let mut names = Vec::new();
    target_names(left, &mut names);
    for id in names {
        let Some(sym) = model.declared_at(id.start_byte()) else { continue };
        let s = model.symbol(sym);
        let shadowed = if s.kind == SymbolKind::Import {
            s.decls.iter().find(|d| d.0 < id.start_byte()).copied()
        } else {
            None
        };
        if let Some(d) = shadowed {
            let edits = loop_rename(n, model, sym, &format!("{}_", s.name), ctx.source);
            out.push(hit_fix(id, format!("The loop variable `{}` replaces the import on line {}", s.name, line_of(ctx.source, d.0)), edits));
        }
    }
}
/// The edits renaming a `for` loop's target `sym` to `new` in the loop —
/// its target, body and `else`, not the iterable, which is evaluated
/// before the name is bound. `auto` when the loop is in a function that
/// reads the name nowhere after it, from no nested scope outside it, and
/// calls no `locals()`, `vars()`, `eval()` or `exec()`, and declares no
/// `global` or `nonlocal`: the name then holds what it held before the loop
/// everywhere it is read. Otherwise a suggestion (a module-level loop
/// rebinds a global others may read). None when `new` is bound anywhere in
/// the module.
pub(crate) fn loop_rename(for_stmt: Node<'_>, model: &SemanticModel, sym: SymbolId, new: &str, src: &str) -> Vec<TextEdit> {
    if model.symbols.iter().any(|x| x.name == new) {
        return Vec::new();
    }
    let s = model.symbol(sym);
    let (lo, hi) = (for_stmt.start_byte(), for_stmt.end_byte());
    let iter = for_stmt.child_by_field_name("right").map_or((0, 0), |r| (r.start_byte(), r.end_byte()));
    let inside = |at: usize| at >= lo && at < hi && !(at >= iter.0 && at < iter.1);
    let mut spans: BTreeSet<(usize, usize)> = s.decls.iter().copied().filter(|d| inside(d.0)).collect();
    let mut safe = model.scopes[s.scope].kind == ScopeKind::Function;
    for &r in &s.refs {
        let r = &model.references[r];
        let in_iter = r.start >= iter.0 && r.start < iter.1;
        if inside(r.start) {
            spans.insert((r.start, r.end));
        } else if (r.start >= lo && !in_iter) || r.scope != s.scope {
            safe = false;
        }
    }
    if spans.iter().any(|&(a, b)| src.get(a..b) != Some(s.name.as_str())) {
        return Vec::new();
    }
    if let Some(def) = crate::python_util::enclosing_def(for_stmt) {
        let body = text(def, src);
        if ["locals(", "vars(", "eval(", "exec(", "global ", "nonlocal "].iter().any(|w| body.contains(w)) {
            safe = false;
        }
    }
    let edits: Vec<TextEdit> = spans.into_iter().map(|(a, b)| TextEdit::new(a, b, new)).collect();
    if safe { edits } else { crate::as_suggestion(edits) }
}

py_rule!(ImportShadowedByLoopVar, "py/import-shadowed-by-loop-var", ["for_statement"], true, import_shadowed_by_loop_var);

fn star_import(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(stmt) = n.parent() else { return };
    let module = stmt.child_by_field_name("module_name").map(|m| text(m, ctx.source)).unwrap_or("");
    // `from .sub import *` is how a package re-exports its submodules' API.
    if module.starts_with('.') {
        return;
    }
    out.push(hit(stmt, format!("`from {module} import *` hides which names the module uses")));
}
py_rule!(UndefinedLocalWithImportStar, "py/no-star-import", ["wildcard_import"], false, star_import);

fn star_import_usage(_root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    if !model.has_star_import {
        return;
    }
    let mut extra = extra_builtins(ctx);
    // In a package's `__init__.py`, `from .sub import *` also binds `sub`.
    for i in &model.imports {
        if i.imported == Imported::Star
            && i.source.starts_with('.')
            && let Some(last) = i.source.trim_start_matches('.').rsplit('.').next().filter(|x| !x.is_empty())
        {
            extra.insert(last.to_owned());
        }
    }
    for st in dunder_all_strings(_root, ctx.source) {
        if let Some(name) = string_value(st, ctx.source)
            && !model.scopes[0].bindings.contains_key(&name)
            && !extra.contains(&name)
        {
            out.push(hit(st, format!("`{name}` in `__all__` may be undefined, or come from a star import")));
        }
    }
    for r in &model.references {
        if r.symbol.is_none() && !codelaw_semantic::PYTHON_BUILTINS.contains(&r.name.as_str()) && !extra.contains(&r.name) && !implicit_name(model, r.scope, &r.name) {
            out.push(NativeFinding {
                start: r.start,
                end: r.end,
                message: format!("`{}` may be undefined, or come from a star import", r.name),
                edits: Vec::new(),
            });
        }
    }
}
py_rule!(StarImportUsage, "py/star-import-usage", ["module"], true, star_import_usage);

fn nested_star_import(n: Node<'_>, _ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if crate::python_util::enclosing_def(n).is_some()
        && let Some(stmt) = n.parent()
    {
        out.push(hit(stmt, "`import *` is only allowed at module level"));
    }
}
py_rule!(NestedStarImport, "py/nested-star-import", ["wildcard_import"], false, nested_star_import);

fn late_future_import(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(parent) = n.parent() else { return };
    if parent.kind() != "module" {
        out.push(hit(n, "`from __future__` imports must come first in the module"));
        return;
    }
    for (i, s) in statements(parent).into_iter().enumerate() {
        if s.id() == n.id() {
            return;
        }
        if s.kind() == "future_import_statement" || (i == 0 && is_docstring_stmt(s)) {
            continue;
        }
        // Moved above the first statement that isn't one: the module never
        // ran (a late `__future__` import is a SyntaxError), so the move is
        // its only meaning.
        let edits = match remove_stmt(n, ctx.source) {
            Some(mut del) => {
                let at = ctx.source[..s.start_byte()].rfind('\n').map_or(0, |i| i + 1);
                del.push(TextEdit::insert(at, format!("{}\n", text(n, ctx.source))));
                del
            }
            None => Vec::new(),
        };
        out.push(hit_fix(n, "`from __future__` imports must come first in the module", edits));
        return;
    }
}
py_rule!(LateFutureImport, "py/late-future-import", ["future_import_statement"], false, late_future_import);

const FUTURE_FEATURES: &[&str] = &[
    "nested_scopes", "generators", "division", "absolute_import", "with_statement", "print_function", "unicode_literals",
    "barry_as_FLUFL", "generator_stop", "annotations",
];

fn future_feature_not_defined(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let names: Vec<Node> = kids(n).into_iter().filter(|c| matches!(c.kind(), "dotted_name" | "aliased_import")).collect();
    for c in kids(n) {
        let name = match c.kind() {
            "dotted_name" => c,
            "aliased_import" => match c.child_by_field_name("name") {
                Some(x) => x,
                None => continue,
            },
            _ => continue,
        };
        let t = text(name, ctx.source);
        if !FUTURE_FEATURES.contains(&t) {
            // The features within two edits, nearest first (the default), or
            // the name removed; a SyntaxError today.
            let mut choices: Vec<codelaw_core::FixChoice> = crate::util::nearest(t, FUTURE_FEATURES.iter().copied(), 2)
                .into_iter()
                .enumerate()
                .map(|(i, f)| crate::choice(f, i == 0, vec![TextEdit::new(name.start_byte(), name.end_byte(), f)]))
                .collect();
            let remove = if names.len() == 1 { remove_stmt(n, ctx.source).unwrap_or_default() } else { vec![crate::util::remove_list_item(c, ctx.source)] };
            choices.push(crate::choice("remove", false, remove));
            out.push(hit_fix(name, format!("`{t}` is not a `__future__` feature"), crate::choice_edits(choices)));
        }
    }
}
py_rule!(FutureFeatureNotDefined, "py/undefined-future-feature", ["future_import_statement"], false, future_feature_not_defined);

fn starred_targets(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>, many: bool) {
    let Some(p) = n.parent() else { return };
    let is_target = matches!(p.kind(), "assignment" | "for_statement" | "for_in_clause") && p.child_by_field_name("left") == Some(n);
    if !is_target {
        return;
    }
    let items = kids(n);
    let stars: Vec<usize> = items.iter().enumerate().filter(|(_, c)| matches!(c.kind(), "list_splat_pattern" | "list_splat")).map(|(i, _)| i).collect();
    if many {
        if stars.len() > 1 {
            // One choice per starred target: it keeps its star, the others lose theirs.
            let choices = stars
                .iter()
                .map(|&k| {
                    let edits = stars.iter().filter(|&&o| o != k).map(|&o| TextEdit::delete(items[o].start_byte(), items[o].start_byte() + 1)).collect();
                    crate::choice(format!("keep {}", text(items[k], ctx.source)), false, edits)
                })
                .collect();
            out.push(hit_fix(n, "Only one starred expression is allowed in an assignment", crate::choice_edits(choices)));
        }
    } else if let [i] = stars[..]
        && (i >= 1 << 8 || items.len() - i > 1 << 24)
    {
        out.push(hit(n, "Too many expressions around the starred target"));
    }
}
fn multiple_starred(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    starred_targets(n, ctx, out, true)
}
fn star_too_many(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    starred_targets(n, ctx, out, false)
}
py_rule!(MultipleStarredExpressions, "py/multiple-starred-expressions", ["pattern_list", "tuple_pattern", "list_pattern", "tuple", "list"], false, multiple_starred);
py_rule!(ExpressionsInStarAssignment, "py/too-many-expressions-in-star-assignment", ["pattern_list", "tuple_pattern", "list_pattern", "tuple", "list"], false, star_too_many);

/// A tuple's items as plain expressions: none when it holds a comment or
/// a starred item.
fn tuple_items<'t>(t: Node<'t>) -> Option<Vec<Node<'t>>> {
    let items = kids(t);
    (!items.is_empty() && items.iter().all(|c| !matches!(c.kind(), "comment" | "list_splat" | "parenthesized_list_splat"))).then_some(items)
}

fn assert_tuple(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if let Some(first) = kids(n).first()
        && first.kind() == "tuple"
        && !kids(*first).is_empty()
    {
        // Without the parentheses: a test and its message, as meant.
        let edits = match tuple_items(*first).as_deref() {
            Some([t]) => vec![TextEdit::new(first.start_byte(), first.end_byte(), text(*t, ctx.source))],
            Some([t, m]) => vec![TextEdit::new(first.start_byte(), first.end_byte(), format!("{}, {}", text(*t, ctx.source), text(*m, ctx.source)))],
            _ => Vec::new(),
        };
        out.push(hit_fix(*first, "A non-empty tuple is always true, so this assert never fails", edits));
    }
}
py_rule!(AssertTuple, "py/assert-tuple", ["assert_statement"], false, assert_tuple);

fn if_tuple(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if let Some(c) = n.child_by_field_name("condition") {
        let c = unparen(c);
        if c.kind() == "tuple" && !kids(c).is_empty() {
            // Test the first item, or all of them joined with `and`.
            let edits = match tuple_items(c) {
                Some(items) => {
                    let src = ctx.source;
                    let all = items
                        .iter()
                        .map(|i| if matches!(i.kind(), "boolean_operator" | "conditional_expression" | "lambda" | "named_expression") { format!("({})", text(*i, src)) } else { text(*i, src).to_owned() })
                        .collect::<Vec<_>>()
                        .join(" and ");
                    let mut choices = vec![crate::choice("first item", false, vec![TextEdit::new(c.start_byte(), c.end_byte(), text(items[0], src))])];
                    if items.len() > 1 {
                        choices.push(crate::choice("all items", false, vec![TextEdit::new(c.start_byte(), c.end_byte(), all)]));
                    }
                    crate::choice_edits(choices)
                }
                None => Vec::new(),
            };
            out.push(hit_fix(c, "A non-empty tuple is always true, so this condition never changes", edits));
        }
    }
}
py_rule!(IfTuple, "py/if-tuple", ["if_statement", "elif_clause"], false, if_tuple);

/// A literal `is` must not compare against: strings, bytes, numbers and
/// tuples of them.
fn is_constant_non_singleton(n: Node<'_>) -> bool {
    let n = unparen(n);
    match n.kind() {
        "string" | "concatenated_string" | "integer" | "float" => true,
        "tuple" => kids(n).iter().all(|c| is_constant_non_singleton(*c) || matches!(c.kind(), "true" | "false" | "none")),
        "unary_operator" => n.child_by_field_name("argument").is_some_and(|a| matches!(a.kind(), "integer" | "float")),
        _ => false,
    }
}

fn is_literal(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let mut operands: Vec<Node> = Vec::new();
    let mut tokens: Vec<Vec<Node>> = vec![Vec::new()];
    for c in children(n) {
        if c.kind() == "comment" {
            continue;
        }
        if c.is_named() {
            operands.push(c);
            tokens.push(Vec::new());
        } else if let Some(t) = tokens.last_mut() {
            t.push(c);
        }
    }
    // tokens[i] sits between operands[i-1] and operands[i].
    for i in 1..operands.len() {
        let toks = &tokens[i];
        let op: Vec<&str> = toks.iter().map(|t| text(*t, src)).collect();
        let op = op.join(" ").split_whitespace().collect::<Vec<_>>().join(" ");
        if op != "is" && op != "is not" {
            continue;
        }
        if !(is_constant_non_singleton(operands[i - 1]) || is_constant_non_singleton(operands[i])) {
            continue;
        }
        let (Some(first), Some(last)) = (toks.first(), toks.last()) else { continue };
        let repl = if op == "is" { "==" } else { "!=" };
        out.push(NativeFinding {
            start: operands[i - 1].start_byte(),
            end: operands[i].end_byte(),
            message: format!("Use `{repl}` to compare with a literal; `{op}` compares identity"),
            edits: vec![TextEdit::new(first.start_byte(), last.end_byte(), repl)],
        });
    }
}
py_rule!(IsLiteral, "py/is-literal", ["comparison_operator"], false, is_literal);

fn invalid_print_syntax(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    match n.kind() {
        "print_statement" => {
            if let Some(chevron) = kids(n).into_iter().find(|c| c.kind() == "chevron") {
                // As 2to3 writes it: the arguments, then `file=`; a trailing
                // comma is `end=" "`.
                let src = ctx.source;
                let edits = match chevron.named_child(0) {
                    Some(f) if !kids(n).iter().any(|c| c.kind() == "comment") => {
                        let mut args: Vec<String> = kids(n).into_iter().filter(|c| c.id() != chevron.id()).map(|c| text(c, src).to_owned()).collect();
                        if children(n).last().is_some_and(|t| t.kind() == ",") {
                            args.push("end=\" \"".into());
                        }
                        args.push(format!("file={}", text(f, src)));
                        vec![TextEdit::new(n.start_byte(), n.end_byte(), format!("print({})", args.join(", ")))]
                    }
                    _ => Vec::new(),
                };
                out.push(hit_fix(n, "`print >>` is Python 2 syntax; use `print(..., file=...)`", edits));
            }
        }
        _ => {
            if let (Some(l), Some(op)) = (n.child_by_field_name("left"), n.child_by_field_name("operator"))
                && text(op, ctx.source) == ">>"
                && l.kind() == "identifier"
                && text(l, ctx.source) == "print"
                && ctx.semantic.is_none_or(|m| m.reference_at(l.start_byte()).is_none_or(|r| r.symbol.is_none()))
            {
                out.push(hit(n, "`print >>` is Python 2 syntax; use `print(..., file=...)`"));
            }
        }
    }
}
py_rule!(InvalidPrintSyntax, "py/invalid-print-syntax", ["print_statement", "binary_operator"], true, invalid_print_syntax);

/// Whether `n` sits in the body of a loop, within the same function.
fn in_loop(n: Node<'_>) -> bool {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        match p.kind() {
            "for_statement" | "while_statement" if p.child_by_field_name("body") == Some(cur) => return true,
            "function_definition" | "class_definition" | "lambda" => return false,
            _ => {}
        }
        cur = p;
    }
    false
}

/// The ways out of a `break` or `continue` outside a loop: `return`, in a
/// function, or the statement deleted (`pass` when its block empties); a
/// SyntaxError today, so neither is the default.
fn jump_choices(n: Node<'_>, src: &str) -> Vec<TextEdit> {
    let mut choices = Vec::new();
    if frame(n) == "function" {
        choices.push(crate::choice("return", false, vec![TextEdit::new(n.start_byte(), n.end_byte(), "return")]));
    }
    choices.push(crate::choice("delete", false, remove_stmt(n, src).unwrap_or_default()));
    crate::choice_edits(choices)
}

fn break_outside_loop(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if !in_loop(n) {
        out.push(hit_fix(n, "`break` outside a loop", jump_choices(n, ctx.source)));
    }
}
fn continue_outside_loop(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if !in_loop(n) {
        out.push(hit_fix(n, "`continue` outside a loop", jump_choices(n, ctx.source)));
    }
}
py_rule!(BreakOutsideLoop, "py/break-outside-loop", ["break_statement"], false, break_outside_loop);
py_rule!(ContinueOutsideLoop, "py/continue-outside-loop", ["continue_statement"], false, continue_outside_loop);

/// The innermost function, class or module around a node.
fn frame(n: Node<'_>) -> &'static str {
    let mut p = n.parent();
    while let Some(x) = p {
        match x.kind() {
            "function_definition" | "lambda" => return "function",
            "class_definition" => return "class",
            _ => {}
        }
        p = x.parent();
    }
    "module"
}

fn yield_outside_function(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let f = frame(n);
    if f == "function" {
        return;
    }
    let what = if n.kind() == "await" {
        "await"
    } else if children(n).iter().any(|c| c.kind() == "from") {
        "yield from"
    } else {
        "yield"
    };
    let _ = ctx;
    out.push(hit(n, format!("`{what}` outside a function (at {f} level)")));
}
py_rule!(YieldOutsideFunction, "py/yield-outside-function", ["yield", "await"], false, yield_outside_function);

fn return_outside_function(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if frame(n) != "function" {
        // Deleted (a SyntaxError today, so nothing ran); a value that does
        // something stays as a statement.
        let edits = match n.named_child(0).filter(|v| v.kind() != "comment") {
            Some(v) if !is_pure(v) => vec![TextEdit::new(n.start_byte(), n.end_byte(), text(v, ctx.source))],
            _ => remove_stmt(n, ctx.source).unwrap_or_default(),
        };
        out.push(hit_fix(n, "`return` outside a function", edits));
    }
}
py_rule!(ReturnOutsideFunction, "py/return-outside-function", ["return_statement"], false, return_outside_function);

fn default_except_not_last(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let handlers: Vec<Node> = kids(n).into_iter().filter(|c| c.kind() == "except_clause").collect();
    let bare_count = handlers.iter().filter(|h| kids(**h).first().is_none_or(|c| c.kind() == "block")).count();
    for (i, h) in handlers.iter().enumerate() {
        let bare = kids(*h).first().is_none_or(|c| c.kind() == "block");
        if bare && i + 1 < handlers.len() {
            let end = h.child(0).map(|c| c.end_byte()).unwrap_or(h.end_byte());
            // Moved after the last handler, its lines whole: a SyntaxError
            // today, so the order every handler can run in is its meaning.
            let edits = match (bare_count, handlers.last()) {
                (1, Some(last)) if starts_line(src, h.start_byte()) => {
                    let lines = |a: usize, b: usize| {
                        let s = src[..a].rfind('\n').map_or(0, |x| x + 1);
                        let e = src[b..].find('\n').map_or(src.len(), |x| b + x + 1);
                        (s, e)
                    };
                    let (hs, he) = lines(h.start_byte(), h.end_byte());
                    let (_, le) = lines(last.start_byte(), last.end_byte());
                    let moved = &src[hs..he];
                    if src[h.end_byte()..he].trim().is_empty() && src[last.end_byte()..le].trim().is_empty() && moved.ends_with('\n') {
                        vec![TextEdit::delete(hs, he), TextEdit::insert(le, moved)]
                    } else {
                        Vec::new()
                    }
                }
                _ => Vec::new(),
            };
            out.push(NativeFinding { start: h.start_byte(), end, message: "A bare `except:` must be the last handler".into(), edits });
        }
    }
}
py_rule!(DefaultExceptNotLast, "py/default-except-not-last", ["try_statement"], false, default_except_not_last);

fn in_literal_or_annotated(n: Node<'_>, src: &str) -> bool {
    let mut p = n.parent();
    while let Some(x) = p {
        if x.kind() == "generic_type" || x.kind() == "subscript" {
            let head = x.child(0).map(|h| text(h, src)).unwrap_or("");
            if head.ends_with("Literal") || head.ends_with("Annotated") {
                return true;
            }
        }
        if x.kind() == "type" {
            return false;
        }
        p = x.parent();
    }
    false
}

fn forward_annotation_syntax_error(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    if !has_type_ancestor(n) || n.parent().is_some_and(|p| p.kind() == "concatenated_string") || in_literal_or_annotated(n, ctx.source) {
        return;
    }
    let Some((prefix, content)) = string_parts(n, ctx.source) else { return };
    if prefix.contains('b') || prefix.contains('f') {
        return;
    }
    let body = content.trim();
    if body.is_empty() {
        out.push(hit(n, "This string annotation is empty"));
        return;
    }
    let parses = |body: &str| {
        let wrapped = format!("__codelaw_annotation__: ({body})\n");
        codelaw_lang::syntax_error_count(&codelaw_lang::parse(codelaw_core::Language::Python, &wrapped)) == 0
    };
    if !parses(body) {
        // The brackets left open, closed: when that makes it parse.
        let mut open = Vec::new();
        let mut balanced = true;
        for ch in body.chars() {
            match ch {
                '(' | '[' | '{' => open.push(ch),
                ')' | ']' | '}' => {
                    let want = match ch {
                        ')' => '(',
                        ']' => '[',
                        _ => '{',
                    };
                    if open.pop() != Some(want) {
                        balanced = false;
                    }
                }
                '"' | '\'' => balanced = false,
                _ => {}
            }
        }
        let closers: String = open.iter().rev().map(|c| match c {
            '(' => ')',
            '[' => ']',
            _ => '}',
        }).collect();
        let t = text(n, ctx.source);
        let q = if t.ends_with("\"\"\"") || t.ends_with("'''") { 3 } else { 1 };
        let edits = if balanced && !closers.is_empty() && parses(&format!("{body}{closers}")) && content.ends_with(body) {
            vec![TextEdit::insert(n.end_byte() - q, closers)]
        } else {
            Vec::new()
        };
        out.push(hit_fix(n, format!("The string annotation `{body}` is not a valid expression"), edits));
    }
}
py_rule!(ForwardAnnotationSyntaxError, "py/forward-annotation-syntax-error", ["string"], false, forward_annotation_syntax_error);

fn is_overload(def: Node<'_>, model: &SemanticModel, src: &str) -> bool {
    decorators(def).into_iter().any(|d| {
        let t = decorator_target(d);
        text(t, src).ends_with("overload") || names_one_of(t, model, src, &["typing.overload", "typing_extensions.overload"])
    })
}

fn redefined_while_unused(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let in_annotations = names_in_string_annotations(root, src, model);
    for (id, s) in model.symbols.iter().enumerate() {
        if s.decls.len() < 2 || in_annotations.contains(&s.name) {
            continue;
        }
        let mut decls = s.decls.clone();
        decls.sort();
        decls.dedup();
        for w in decls.windows(2) {
            let (a, b) = (w[0], w[1]);
            let (Some(na), Some(nb)) = (ident_at(root, a.0, a.1), ident_at(root, b.0, b.1)) else { continue };
            let (ka, ctx_a) = bind_ctx(na);
            if !matches!(ka, Bind::Import | Bind::Def | Bind::Class) {
                continue;
            }
            let (kb, ctx_b) = bind_ctx(nb);
            if matches!(kb, Bind::AnnOnly | Bind::Aug | Bind::Other) {
                continue;
            }
            // A definition's own body runs later; a reference there doesn't use it now.
            let body = if matches!(ka, Bind::Def | Bind::Class) { ctx_a } else { None };
            let used = s.refs.iter().any(|&r| {
                let r = &model.references[r];
                let between = r.start >= a.1 && r.start <= b.0;
                // `x = f(x)`: the right-hand side runs before the new binding.
                let in_rhs = kb == Bind::Assign
                    && ctx_b.and_then(|c| c.child_by_field_name("right")).is_some_and(|rh| rh.start_byte() <= r.start && r.start < rh.end_byte());
                r.read && (between || in_rhs) && body.is_none_or(|d| !(d.start_byte() <= r.start && r.start < d.end_byte()))
            });
            // As in Ruff, only a redefinition in the same branch counts.
            if used || different_forks(na, nb) || branch(na).map(|b| b.id()) != branch(nb).map(|b| b.id()) {
                continue;
            }
            // An optional accelerator: `try: from _speedups import f`.
            if kb == Bind::Import && in_try(nb) {
                continue;
            }
            if ka == Bind::Def && ctx_a.is_some_and(|d| is_overload(d, model, src)) {
                continue;
            }
            if ka == Bind::Import && kb == Bind::Import {
                // `import a.b` and `import a.c` both bind `a`, for different submodules.
                let entry = |id: Node<'_>| {
                    let p = id.parent().filter(|p| p.kind() == "dotted_name");
                    p.filter(|p| p.parent().is_some_and(|g| g.kind() == "import_statement")).map(|p| text(p, src).to_owned())
                };
                let (ea, eb) = (entry(na), entry(nb));
                if (ea.as_deref().is_some_and(|e| e.contains('.')) || eb.as_deref().is_some_and(|e| e.contains('.'))) && ea != eb {
                    continue;
                }
            }
            // A `global` declaration rebinds from inside a function.
            let outer = |n: Node<'_>| crate::python_util::enclosing_def(statement_of(n)).map(|d| d.id());
            if outer(na) != outer(nb) {
                continue;
            }
            if ka == Bind::Import && in_try(na) {
                continue;
            }
            let _ = id;
            let edits = match (import_key(na, src), import_key(nb, src)) {
                // The same import twice, the later alone on its statement:
                // it goes, and the name stays bound to the same object.
                (Some((_, ka)), Some((sb, kb))) if ka == kb && import_names(sb) == 1 => remove_stmt(sb, src).unwrap_or_default(),
                (Some(_), Some(_)) => Vec::new(),
                // An unused definition: it goes, but its decorators and
                // defaults no longer run.
                _ if matches!(ka, Bind::Def | Bind::Class) => crate::as_suggestion(remove_stmt(statement_of(na), src).unwrap_or_default()),
                _ => Vec::new(),
            };
            out.push(hit_fix(nb, format!("`{}` is redefined here, but the definition on line {} was never used", s.name, line_of(src, a.0)), edits));
        }
    }
}
/// The import statement binding `id`, and what it imports there as text:
/// its kind, module and entry (`import a.b`, `from m import a as b`).
fn import_key<'t>(id: Node<'t>, src: &str) -> Option<(Node<'t>, String)> {
    let mut cur = id;
    while let Some(p) = cur.parent() {
        if matches!(p.kind(), "import_statement" | "import_from_statement") {
            let module = p.child_by_field_name("module_name").map_or("", |m| text(m, src));
            return Some((p, format!("{}|{module}|{}", p.kind(), text(cur, src))));
        }
        if !matches!(p.kind(), "dotted_name" | "aliased_import") {
            return None;
        }
        cur = p;
    }
    None
}

/// How many names an import statement imports.
fn import_names(stmt: Node<'_>) -> usize {
    let mut cursor = stmt.walk();
    stmt.children_by_field_name("name", &mut cursor).count()
}

py_rule!(RedefinedWhileUnused, "py/redefined-while-unused", ["module"], true, redefined_while_unused);

fn extra_builtins(ctx: &NativeContext<'_>) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = ["WindowsError", "__builtins__"].iter().map(|s| s.to_string()).collect();
    for b in ctx.options.get("builtins").and_then(|v| v.as_array()).into_iter().flatten().filter_map(|v| v.as_str()) {
        out.insert(b.to_owned());
    }
    out
}

/// Names Python defines implicitly in some scopes: `__class__` in methods,
/// `__module__` and `__qualname__` in class bodies.
fn implicit_name(model: &SemanticModel, scope: usize, name: &str) -> bool {
    let mut sc = Some(scope);
    while let Some(s) = sc {
        let kind = model.scopes[s].kind;
        if kind == ScopeKind::Class && matches!(name, "__module__" | "__qualname__" | "__class__") {
            return name != "__class__" || s != scope;
        }
        sc = model.scopes[s].parent;
    }
    false
}

/// Whether the reference sits in a `try` body whose handlers catch
/// `NameError`.
fn guarded_by_name_error(root: Node<'_>, start: usize, src: &str) -> bool {
    let Some(mut cur) = root.descendant_for_byte_range(start, start) else { return false };
    while let Some(p) = cur.parent() {
        if p.kind() == "try_statement" && p.child_by_field_name("body") == Some(cur) {
            let catches = kids(p).into_iter().filter(|c| c.kind() == "except_clause").any(|h| {
                kids(h).first().is_some_and(|t| {
                    let t = if t.kind() == "as_pattern" { t.named_child(0).unwrap_or(*t) } else { *t };
                    text(t, src).contains("NameError")
                })
            });
            if catches {
                return true;
            }
        }
        cur = p;
    }
    false
}

fn undefined_name(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    if model.has_star_import {
        return;
    }
    let extra = extra_builtins(ctx);
    for r in &model.references {
        if r.symbol.is_some() || !r.read || codelaw_semantic::PYTHON_BUILTINS.contains(&r.name.as_str()) || extra.contains(&r.name) {
            continue;
        }
        if implicit_name(model, r.scope, &r.name) || guarded_by_name_error(root, r.start, ctx.source) {
            continue;
        }
        // The names in scope within two edits, nearest first (the default).
        // The innermost scope's first, so a tie goes to the nearer name.
        let mut known: Vec<&str> = Vec::new();
        let mut sc = Some(r.scope);
        while let Some(i) = sc {
            if model.scopes[i].kind != ScopeKind::Class || i == r.scope {
                known.extend(model.scopes[i].bindings.keys().map(String::as_str));
            }
            sc = model.scopes[i].parent;
        }
        known.extend(codelaw_semantic::PYTHON_BUILTINS);
        let edits = crate::util::name_choices(&crate::util::nearest(&r.name, known, 2), r.start, r.end, &|c| c.to_owned());
        out.push(NativeFinding { start: r.start, end: r.end, message: format!("`{}` is not defined", r.name), edits });
    }
}
py_rule!(UndefinedName, "py/undefined-name", ["module"], true, undefined_name);

/// The string elements of module-level `__all__` definitions.
pub(crate) fn dunder_all_strings<'t>(root: Node<'t>, src: &str) -> Vec<Node<'t>> {
    // Module-level statements, including those inside `if` / `try` blocks.
    let mut stmts = Vec::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        for c in kids(n) {
            match c.kind() {
                "function_definition" | "class_definition" | "decorated_definition" => {}
                "if_statement" | "elif_clause" | "else_clause" | "try_statement" | "except_clause" | "finally_clause" | "block" | "with_statement" => {
                    stack.push(c)
                }
                _ if n.kind() == "module" || n.kind() == "block" => stmts.push(c),
                _ => {}
            }
        }
    }
    stmts.sort_by_key(|s| s.start_byte());
    let mut out = Vec::new();
    for stmt in stmts {
        let Some(e) = stmt.named_child(0).filter(|_| stmt.kind() == "expression_statement") else { continue };
        let value = match e.kind() {
            "assignment" | "augmented_assignment" => {
                if e.child_by_field_name("left").map(|l| text(l, src)) != Some("__all__") {
                    continue;
                }
                e.child_by_field_name("right")
            }
            "call" => {
                let f = e.child_by_field_name("function").map(|f| text(f, src)).unwrap_or("");
                if f != "__all__.extend" && f != "__all__.append" {
                    continue;
                }
                let args = call_args(e, src);
                if f == "__all__.append" {
                    out.extend(args.positional.into_iter().filter(|a| a.kind() == "string"));
                    continue;
                }
                args.positional.first().copied()
            }
            _ => continue,
        };
        if let Some(v) = value
            && matches!(v.kind(), "list" | "tuple" | "expression_list")
        {
            out.extend(kids(v).into_iter().filter(|s| s.kind() == "string"));
        }
    }
    out
}

fn undefined_export(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    if model.has_star_import {
        return;
    }
    for s in dunder_all_strings(root, ctx.source) {
        let Some(name) = string_value(s, ctx.source) else { continue };
        if !model.scopes[0].bindings.contains_key(&name) && !codelaw_semantic::PYTHON_BUILTINS.contains(&name.as_str()) {
            // A module `__getattr__` may define it; otherwise the module's
            // names within two edits (the nearest the default), or the
            // entry removed.
            let edits = if model.scopes[0].bindings.contains_key("__getattr__") || !s.parent().is_some_and(|p| matches!(p.kind(), "list" | "tuple")) {
                Vec::new()
            } else {
                let t = text(s, ctx.source);
                let q = &t[t.len() - 1..];
                let near = crate::util::nearest(&name, model.scopes[0].bindings.keys().map(String::as_str).filter(|k| !k.starts_with("__")), 2);
                let mut choices: Vec<codelaw_core::FixChoice> = near.iter().enumerate().map(|(i, n)| crate::choice(*n, i == 0, vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{n}{q}"))])).collect();
                choices.push(crate::choice("remove", false, vec![crate::util::remove_list_item(s, ctx.source)]));
                crate::choice_edits(choices)
            };
            out.push(hit_fix(s, format!("`{name}` is listed in `__all__` but not defined in the module"), edits));
        }
    }
}
py_rule!(UndefinedExport, "py/undefined-export", ["module"], true, undefined_export);

fn undefined_local(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    for s in &model.symbols {
        if model.scopes[s.scope].kind != ScopeKind::Function || s.kind == SymbolKind::Param {
            continue;
        }
        let Some(&first) = s.decls.iter().min() else { continue };
        let Some(decl) = ident_at(root, first.0, first.1) else { continue };
        let rhs = match bind_ctx(decl) {
            (Bind::Assign, Some(a)) => a.child_by_field_name("right"),
            _ => None,
        };
        // Defined in an enclosing function or the module.
        let mut outer = model.scopes[s.scope].parent;
        let mut defined_outside = None;
        while let Some(sc) = outer {
            if model.scopes[sc].kind != ScopeKind::Class && model.scopes[sc].bindings.contains_key(&s.name) {
                defined_outside = Some(model.scopes[sc].kind);
                break;
            }
            outer = model.scopes[sc].parent;
        }
        let Some(where_) = defined_outside else { continue };
        for &r in &s.refs {
            let r = &model.references[r];
            if !r.read || r.scope != s.scope {
                continue;
            }
            let before = r.start <= first.0 || rhs.is_some_and(|x| x.start_byte() <= r.start && r.start < x.end_byte());
            if before {
                // `global` (a module binding) or `nonlocal` (an enclosing
                // function's) at the top of the function: the function then
                // reads and writes the outer name.
                let word = if where_ == ScopeKind::Function { "nonlocal" } else { "global" };
                let edits = crate::python_util::enclosing_def(decl)
                    .filter(|d| d.kind() == "function_definition")
                    .and_then(|d| d.child_by_field_name("body"))
                    .and_then(|body| {
                        let stmts = statements(body);
                        let first = stmts.iter().find(|x| !is_docstring_stmt(**x) && x.kind() != "comment").copied()?;
                        starts_line(ctx.source, first.start_byte()).then(|| {
                            let indent = crate::python_util::indent_at(ctx.source, first.start_byte());
                            vec![TextEdit::insert(first.start_byte(), format!("{word} {}\n{indent}", s.name))]
                        })
                    })
                    .unwrap_or_default();
                out.push(NativeFinding {
                    start: r.start,
                    end: r.end,
                    message: format!("`{}` is read here before this function assigns it, so it is never the outer `{}`", s.name, s.name),
                    edits,
                });
                break;
            }
        }
    }
}
py_rule!(UndefinedLocal, "py/undefined-local", ["module"], true, undefined_local);

/// Whether a function calls `locals()` in its own body.
fn calls_locals(func: Node<'_>, src: &str) -> bool {
    let mut stack = kids(func);
    while let Some(n) = stack.pop() {
        if matches!(n.kind(), "function_definition" | "lambda" | "class_definition") {
            continue;
        }
        if n.kind() == "call" && n.child_by_field_name("function").is_some_and(|f| text(f, src) == "locals") {
            return true;
        }
        stack.extend(kids(n));
    }
    false
}

fn unused_locals(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>, annotations: bool) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let mut locals_cache: BTreeMap<usize, bool> = BTreeMap::new();
    for s in &model.symbols {
        let scope = &model.scopes[s.scope];
        if s.kind != SymbolKind::Variable || s.name.starts_with('_') {
            continue;
        }
        if scope.kind != ScopeKind::Function {
            // Outside functions only an unused `except … as name` is reported.
            let last = s.decls.iter().copied().max().unwrap_or((0, 0));
            let is_except = !annotations && ident_at(root, last.0, last.1).is_some_and(|i| bind_ctx(i).0 == Bind::Except);
            if !is_except || scope.kind == ScopeKind::Class {
                continue;
            }
        }
        // Read anywhere, or written from a nested function (`nonlocal`).
        if s.refs.iter().any(|&r| {
            let r = &model.references[r];
            r.read || r.scope != s.scope || model.scope_at(r.start) != s.scope
        }) {
            continue;
        }
        let uses_locals = *locals_cache.entry(s.scope).or_insert_with(|| {
            root.descendant_for_byte_range(scope.start, scope.end)
                .map(|mut f| {
                    while !matches!(f.kind(), "function_definition" | "lambda") {
                        match f.parent() {
                            Some(p) => f = p,
                            None => return false,
                        }
                    }
                    calls_locals(f, src)
                })
                .unwrap_or(false)
        });
        if uses_locals {
            continue;
        }
        let last = s.decls.iter().copied().max().unwrap_or((0, 0));
        let Some(id) = ident_at(root, last.0, last.1) else { continue };
        let (kind, at) = bind_ctx(id);
        match (kind, annotations) {
            (Bind::AnnOnly, true) => {
                let edits = at.map(statement_of).and_then(|st| remove_stmt(st, src)).unwrap_or_default();
                out.push(hit_fix(id, format!("`{}` is annotated but never used", s.name), edits));
            }
            (Bind::Assign | Bind::With | Bind::Except | Bind::Walrus, false) => {
                if unpacking_partly_used(id, at, model) {
                    continue;
                }
                let edits = unused_variable_edits(id, kind, at, src).unwrap_or_default();
                out.push(hit_fix(id, format!("`{}` is assigned but never used", s.name), edits));
            }
            _ => {}
        }
    }
}

/// For a name bound by unpacking (`a, b = …`): Ruff reports unpacking only
/// from a tuple or list literal, where each name is a separate assignment.
fn unpacking_partly_used(_id: Node<'_>, at: Option<Node<'_>>, _model: &SemanticModel) -> bool {
    let Some(a) = at else { return false };
    match a.kind() {
        "assignment" => {
            let Some(left) = a.child_by_field_name("left") else { return false };
            left.kind() != "identifier" && !a.child_by_field_name("right").is_some_and(|r| matches!(r.kind(), "expression_list" | "tuple" | "list"))
        }
        "as_pattern" => a.child_by_field_name("alias").and_then(|t| kids(t).first().copied()).is_none_or(|t| t.kind() != "identifier"),
        _ => false,
    }
}

fn in_try(n: Node<'_>) -> bool {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        match p.kind() {
            "try_statement" => return true,
            "function_definition" | "class_definition" => return false,
            _ => {}
        }
        cur = p;
    }
    false
}

fn unused_variable_edits(id: Node<'_>, kind: Bind, at: Option<Node<'_>>, src: &str) -> Option<Vec<TextEdit>> {
    let at = at?;
    match kind {
        Bind::Except => {
            let value = at.named_child(0)?;
            Some(vec![TextEdit::new(value.end_byte(), at.end_byte(), "")])
        }
        Bind::Walrus => {
            let value = at.child_by_field_name("value")?;
            Some(vec![TextEdit::new(at.start_byte(), at.end_byte(), text(value, src))])
        }
        Bind::Assign => {
            if at.child_by_field_name("left") != Some(id) {
                return None;
            }
            let stmt = at.parent().filter(|p| p.kind() == "expression_statement")?;
            let right = at.child_by_field_name("right")?;
            if right.kind() == "assignment" {
                return None;
            }
            if is_pure(right) {
                remove_stmt(stmt, src)
            } else if matches!(right.kind(), "yield" | "expression_list") {
                None
            } else {
                Some(vec![TextEdit::new(at.start_byte(), right.start_byte(), "")])
            }
        }
        _ => None,
    }
}

fn unused_variable(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    unused_locals(root, ctx, out, false)
}
fn unused_annotation(root: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    unused_locals(root, ctx, out, true)
}
py_rule!(UnusedVariable, "py/unused-variable", ["module"], true, unused_variable);
py_rule!(UnusedAnnotation, "py/unused-annotation", ["module"], true, unused_annotation);

fn raise_not_implemented(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(e) = kids(n).first().copied() else { return };
    let target = if e.kind() == "call" { e.child_by_field_name("function") } else { Some(e) };
    let Some(t) = target.filter(|t| t.kind() == "identifier" && text(*t, ctx.source) == "NotImplemented") else { return };
    if ctx.semantic.is_some_and(|m| m.reference_at(t.start_byte()).is_some_and(|r| r.symbol.is_some())) {
        return;
    }
    out.push(hit_fix(t, "`NotImplemented` is a value, not an exception; raise `NotImplementedError`", vec![TextEdit::new(t.start_byte(), t.end_byte(), "NotImplementedError")]));
}
py_rule!(RaiseNotImplemented, "py/raise-not-implemented", ["raise_statement"], true, raise_not_implemented);

/// A dictionary key as a comparable string, for literal keys.
fn literal_key(n: Node<'_>, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "string" | "concatenated_string" => {
            if is_fstring(n, src) {
                return None;
            }
            let bytes = text(n, src).trim_start_matches(|c: char| c.is_ascii_alphabetic()).len() != text(n, src).len()
                && text(n, src).chars().take_while(|c| c.is_ascii_alphabetic()).any(|c| c == 'b' || c == 'B');
            let v = match n.kind() {
                "string" => {
                    let (_, c) = string_parts(n, src)?;
                    c
                }
                _ => kids(n).iter().filter_map(|p| string_parts(*p, src).map(|x| x.1)).collect(),
            };
            Some(format!("{}:{v}", if bytes { "b" } else { "s" }))
        }
        "integer" | "float" => Some(format!("n:{}", text(n, src).replace('_', "").to_ascii_lowercase())),
        "true" | "false" | "none" => Some(format!("k:{}", text(n, src))),
        "unary_operator" => {
            let a = n.child_by_field_name("argument")?;
            if matches!(a.kind(), "integer" | "float") { Some(format!("n:{}", text(n, src).replace([' ', '_'], ""))) } else { None }
        }
        "tuple" => {
            let parts: Option<Vec<String>> = kids(n).iter().map(|c| literal_key(*c, src)).collect();
            Some(format!("t:({})", parts?.join(",")))
        }
        _ => None,
    }
}

fn repeated_keys(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>, variables: bool) {
    let src = ctx.source;
    let mut seen: BTreeMap<String, Vec<(String, Node)>> = BTreeMap::new();
    for p in kids(n) {
        if p.kind() != "pair" {
            continue;
        }
        let (Some(k), Some(v)) = (p.child_by_field_name("key"), p.child_by_field_name("value")) else { continue };
        let key = if variables {
            let k = unparen(k);
            if k.kind() != "identifier" {
                continue;
            }
            text(k, src).to_owned()
        } else {
            match literal_key(k, src) {
                Some(x) => x,
                None => continue,
            }
        };
        let value = text(v, src).split_whitespace().collect::<String>();
        let prev = seen.entry(key).or_default();
        if let Some((_, first)) = prev.first() {
            let shown = text(k, src);
            let same = prev.iter().any(|(pv, _)| *pv == value);
            let msg = if same {
                format!("The key `{shown}` repeats with the same value")
            } else {
                format!("The key `{shown}` repeats; only its last value is kept")
            };
            let remove_later = crate::util::remove_list_item(p, src);
            let edits = if same && is_pure(v) && prev.len() == 1 {
                // The same value again, nothing run: it goes.
                vec![remove_later]
            } else if let Some(fv) = first.child_by_field_name("value").filter(|_| prev.len() == 1) {
                // Python keeps the first key's place and the last value:
                // `keep last` writes that (the default), `keep first` keeps
                // the first value.
                crate::choice_edits(vec![
                    crate::choice("keep last", true, vec![TextEdit::new(fv.start_byte(), fv.end_byte(), text(v, src)), remove_later.clone()]),
                    crate::choice("keep first", false, vec![remove_later]),
                ])
            } else {
                Vec::new()
            };
            out.push(hit_fix(k, msg, edits));
        }
        prev.push((value, p));
    }
}
fn repeated_key_literal(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    repeated_keys(n, ctx, out, false)
}
fn repeated_key_variable(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    repeated_keys(n, ctx, out, true)
}
py_rule!(MultiValueRepeatedKeyLiteral, "py/duplicate-dict-key", ["dictionary"], false, repeated_key_literal);
py_rule!(MultiValueRepeatedKeyVariable, "py/duplicate-dict-key-variable", ["dictionary"], false, repeated_key_variable);

fn fstring_missing_placeholders(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    if n.kind() == "string" && n.parent().is_some_and(|p| p.kind() == "concatenated_string") {
        return;
    }
    let parts: Vec<Node> = if n.kind() == "string" { vec![n] } else { kids(n).into_iter().filter(|c| c.kind() == "string").collect() };
    if !parts.iter().any(|p| is_fstring(*p, src)) {
        return;
    }
    let has_placeholder = |p: &Node<'_>| children(*p).iter().any(|c| c.kind() == "interpolation");
    if parts.iter().any(has_placeholder) {
        return;
    }
    let mut edits = Vec::new();
    for p in parts.iter().filter(|p| is_fstring(**p, src)) {
        let Some(start) = children(*p).into_iter().find(|c| c.kind() == "string_start") else { continue };
        let st = text(start, src);
        if st.chars().any(|c| c == 't' || c == 'T') {
            return;
        }
        let new_start: String = st.chars().filter(|c| *c != 'f' && *c != 'F').collect();
        edits.push(TextEdit::new(start.start_byte(), start.end_byte(), new_start));
        let mut stack = vec![*p];
        while let Some(x) = stack.pop() {
            if x.kind() == "escape_interpolation" {
                let t = text(x, src);
                edits.push(TextEdit::new(x.start_byte(), x.end_byte(), &t[..1]));
            }
            stack.extend(children(x));
        }
    }
    edits.sort_by_key(|e| e.start);
    out.push(hit_fix(n, "This f-string has no placeholders; drop the `f` prefix", edits));
}
py_rule!(FStringMissingPlaceholders, "py/f-string-missing-placeholders", ["string", "concatenated_string"], false, fstring_missing_placeholders);

#[cfg(test)]
mod tests {
    use crate::run;
    use codelaw_core::Language;

    fn spans(rule: &dyn crate::NativeRule, src: &str) -> Vec<String> {
        run(rule, Language::Python, src).iter().map(|f| src[f.start..f.end].to_owned()).collect()
    }

    #[test]
    fn unused_imports_and_locals() {
        let src = "import os, sys\nfrom x import (a, b)\nprint(sys, a)\ndef f():\n    y = 1\n    _z = 2\n    w: int\n";
        assert_eq!(spans(&super::UnusedImport, src), ["os", "b"]);
        assert_eq!(spans(&super::UnusedVariable, src), ["y"]);
        assert_eq!(spans(&super::UnusedAnnotation, src), ["w"]);
    }

    #[test]
    fn redefinition_skips_forks_and_setters() {
        let src = "try:\n    import json\nexcept ImportError:\n    json = None\nclass C:\n    @property\n    def p(self): pass\n    @p.setter\n    def p(self, v): pass\ndef g(): pass\ndef g(): pass\n";
        assert_eq!(spans(&super::RedefinedWhileUnused, src), ["g"]);
    }

    #[test]
    fn undefined_local_reads_before_assignment() {
        let src = "x = 1\ndef f():\n    x += 1\ndef g():\n    print(x)\n    x = 2\ndef h():\n    x = 3\n    print(x)\n";
        assert_eq!(spans(&super::UndefinedLocal, src), ["x", "x"]);
    }
}

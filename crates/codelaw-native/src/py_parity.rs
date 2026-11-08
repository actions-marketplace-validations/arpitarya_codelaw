//! W-139 strand W-139-py, native rows: Ruff and Pylint checks that count
//! statements against a threshold, compare two nodes' lengths, read the
//! file's path or its bytes before the first token, or edit one character
//! inside a string — each its upstream behaviour (Ruff 0.16.9; Pylint 4.1.3,
//! behaviour only), read from the upstream source and tests and written
//! fresh. Where upstream infers a type or a value, these read literals only,
//! and each rule's page says so.

use crate::python_util::{decorators, hit, hit_fix, kids, statements, string_parts, unparen, PY};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tree_sitter::Node;

macro_rules! rule {
    ($name:ident, $id:literal, $kinds:expr, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
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
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}

fn uint(o: &BTreeMap<String, Value>, key: &str, default: usize) -> usize {
    o.get(key).and_then(Value::as_u64).map_or(default, |v| v as usize)
}

fn span(start: usize, end: usize, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

/// The statements of a node's block: the `body`/`consequence` field, or
/// the first `block` child (an `except` clause has no field).
fn body_of(n: Node<'_>) -> Vec<Node<'_>> {
    let b = field(n, "body").or_else(|| field(n, "consequence")).or_else(|| named_children(n).into_iter().find(|c| c.kind() == "block"));
    b.map(statements).unwrap_or_default()
}

/// A decorated definition's definition; any other node itself.
fn undecorated(n: Node<'_>) -> Node<'_> {
    if n.kind() == "decorated_definition" {
        field(n, "definition").unwrap_or(n)
    } else {
        n
    }
}

fn is_scope(kind: &str) -> bool {
    matches!(kind, "function_definition" | "class_definition" | "lambda")
}

fn is_stub(ctx: &NativeContext<'_>) -> bool {
    ctx.file.path().ends_with(".pyi")
}

// ---------------------------------------------------------------- characters

/// The control and invisible characters Pylint and Ruff report inside a
/// string (PLE2510, PLE2512–PLE2515): the character and its escape.
const INVALID: &[(char, &str, &str)] = &[
    ('\u{8}', "\\b", "backspace"),
    ('\u{1a}', "\\x1a", "SUB"),
    ('\u{1b}', "\\x1b", "ESC"),
    ('\u{200b}', "\\u200b", "zero-width space"),
];

fn invalid_characters(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>, wanted: char) {
    let src = ctx.source;
    let Some((_, esc, name)) = INVALID.iter().find(|(c, _, _)| *c == wanted) else { return };
    let mut raw = false;
    for c in children(n) {
        match c.kind() {
            "string_start" => raw = text(c, src).to_ascii_lowercase().contains('r'),
            "string_content" => {
                let t = text(c, src);
                for (i, ch) in t.char_indices() {
                    if ch != wanted {
                        continue;
                    }
                    let start = c.start_byte() + i;
                    let escaped = t[..i].chars().rev().take_while(|c| *c == '\\').count() % 2 == 1;
                    let edits = if raw || escaped { Vec::new() } else { vec![TextEdit::new(start, start + ch.len_utf8(), (*esc).to_owned())] };
                    out.push(span(start, start + ch.len_utf8(), format!("Write the {name} character as `{esc}`"), edits));
                }
            }
            _ => {}
        }
    }
}

rule!(InvalidCharacterBackspace, "py/invalid-character-backspace", &["string"], |n, ctx, out| { invalid_characters(n, ctx, out, '\u{8}') });
rule!(InvalidCharacterSub, "py/invalid-character-sub", &["string"], |n, ctx, out| { invalid_characters(n, ctx, out, '\u{1a}') });
rule!(InvalidCharacterEsc, "py/invalid-character-esc", &["string"], |n, ctx, out| { invalid_characters(n, ctx, out, '\u{1b}') });
rule!(InvalidCharacterZeroWidthSpace, "py/invalid-character-zero-width-space", &["string"], |n, ctx, out| {
    invalid_characters(n, ctx, out, '\u{200b}')
});

// ---------------------------------------------------------------- __slots__

rule!(UnsortedDunderSlots, "py/unsorted-dunder-slots", &["assignment", "augmented_assignment"], |n, ctx, out| {
    let src = ctx.source;
    let (Some(left), Some(right)) = (field(n, "left"), field(n, "right")) else { return };
    if left.kind() != "identifier" || text(left, src) != "__slots__" {
        return;
    }
    // Only in a class body.
    let in_class = n.parent().and_then(|s| s.parent()).and_then(|b| b.parent()).is_some_and(|c| c.kind() == "class_definition");
    if !in_class {
        return;
    }
    let list = unparen(right);
    if !matches!(list.kind(), "list" | "tuple" | "expression_list" | "set") {
        return;
    }
    let items = kids(list);
    let values: Option<Vec<String>> = items.iter().map(|i| crate::python_util::string_value(*i, src).filter(|_| i.kind() == "string")).collect();
    let Some(values) = values else { return };
    if values.len() < 2 {
        return;
    }
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|&a, &b| crate::python_isort::natural(&values[a], &values[b]));
    if order.iter().enumerate().all(|(i, &o)| i == o) {
        return;
    }
    let has_comments = named_children(list).iter().any(|c| c.kind() == "comment");
    let mut edits = Vec::new();
    if !has_comments {
        let first = items[0];
        let last = *items.last().expect("two or more");
        let multiline = text(list, src).contains('\n');
        let column = crate::python_util::indent_at(src, first.start_byte());
        let sep = if multiline && crate::python_util::starts_line(src, first.start_byte()) { format!(",\n{column}") } else { ", ".to_owned() };
        let joined = order.iter().map(|&o| text(items[o], src)).collect::<Vec<_>>().join(&sep);
        edits.push(TextEdit::new(first.start_byte(), last.end_byte(), joined));
    }
    out.push(hit_fix(list, "`__slots__` is not sorted", edits));
});

// ---------------------------------------------------------------- the file

/// The folder of a project-relative path (`""` at the root).
fn parent_of(path: &str) -> &str {
    path.rfind('/').map_or("", |i| &path[..i])
}

fn file_name(path: &str) -> &str {
    path.rfind('/').map_or(path, |i| &path[i + 1..])
}

/// The source folders Ruff's `src` setting names by default: the project
/// root and `src/`.
fn is_source_root(dir: &str) -> bool {
    dir.is_empty() || dir == "src"
}

/// Whether the file is in a package: its folder holds an `__init__.py`
/// (or `__init__.pyi`).
fn in_package(ctx: &NativeContext<'_>, dir: &str) -> bool {
    let join = |f: &str| if dir.is_empty() { f.to_owned() } else { format!("{dir}/{f}") };
    let (py, pyi) = (join("__init__.py"), join("__init__.pyi"));
    ctx.file.path() == py || ctx.file.path() == pyi || ctx.file.exists(&py) || ctx.file.exists(&pyi)
}

/// PEP 723's script block (`# /// script`).
fn is_pep723_script(src: &str) -> bool {
    src.lines().any(|l| l.trim_end() == "# /// script")
}

rule!(ImplicitNamespacePackage, "py/implicit-namespace-package", &["module"], |n, ctx, out| {
    let path = ctx.file.path();
    if !path.ends_with(".py") {
        return;
    }
    let dir = parent_of(path);
    if is_source_root(dir) || in_package(ctx, dir) || ctx.source.starts_with("#!") || is_pep723_script(ctx.source) {
        return;
    }
    let _ = n;
    out.push(span(0, 0, format!("`{path}` is in a folder with no `__init__.py`: add one, or it is an implicit namespace package"), Vec::new()));
});

/// The module's own name: the file's stem, or its folder's name for an
/// `__init__.py`.
fn module_name(path: &str) -> &str {
    let name = file_name(path);
    if name == "__init__.py" || name == "__init__.pyi" {
        return file_name(parent_of(path));
    }
    name.strip_suffix(".pyi").or_else(|| name.strip_suffix(".py")).unwrap_or(name)
}

fn is_module_identifier(name: &str, digits_first: bool) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else { return false };
    (first == '_' || first.is_ascii_lowercase() || (digits_first && first.is_ascii_digit())) && chars.all(|c| c == '_' || c.is_ascii_lowercase() || c.is_ascii_digit())
}

rule!(InvalidModuleName, "py/invalid-module-name", &["module"], |n, ctx, out| {
    let _ = n;
    let path = ctx.file.path();
    if !(path.ends_with(".py") || path.ends_with(".pyi")) {
        return;
    }
    let is_init = matches!(file_name(path), "__init__.py" | "__init__.pyi");
    // Ruff reads a module name only inside a package.
    let dir = parent_of(path);
    if !in_package(ctx, dir) && !is_init {
        return;
    }
    let name = module_name(path);
    if name.is_empty() {
        return;
    }
    let migrations = matches!(file_name(parent_of(path)), "versions" | "migrations") && !is_init;
    if is_module_identifier(name, migrations) {
        return;
    }
    let ignore = ctx.options.get("ignore_names").and_then(Value::as_array);
    if ignore.is_some_and(|l| l.iter().filter_map(Value::as_str).any(|p| glob_match(p, name))) {
        return;
    }
    out.push(span(0, 0, format!("Module name `{name}` is not lowercase with underscores"), Vec::new()));
});

/// A glob with `*` and `?` only, as Ruff's `ignore-names`.
fn glob_match(pattern: &str, name: &str) -> bool {
    fn go(p: &[char], s: &[char]) -> bool {
        match (p.first(), s.first()) {
            (None, None) => true,
            (Some('*'), _) => go(&p[1..], s) || (!s.is_empty() && go(p, &s[1..])),
            (Some('?'), Some(_)) => go(&p[1..], &s[1..]),
            (Some(a), Some(b)) if a == b => go(&p[1..], &s[1..]),
            _ => false,
        }
    }
    go(&pattern.chars().collect::<Vec<_>>(), &name.chars().collect::<Vec<_>>())
}

rule!(NonAsciiFileName, "py/non-ascii-file-name", &["module"], |n, ctx, out| {
    let _ = n;
    let path = ctx.file.path();
    let name = module_name(path);
    if name.is_ascii() {
        return;
    }
    out.push(span(0, 0, format!("Module name `{name}` has non-ASCII characters"), Vec::new()));
});

rule!(StdlibModuleShadowing, "py/stdlib-module-shadowing", &["module"], |n, ctx, out| {
    let _ = n;
    let path = ctx.file.path();
    if !path.ends_with(".py") {
        return;
    }
    // Relative to the longest source root (the project root or `src/`).
    let rel = path.strip_prefix("src/").unwrap_or(path);
    let is_init = file_name(rel) == "__init__.py";
    let module_path = if is_init { parent_of(rel).to_owned() } else { rel.trim_end_matches(".py").to_owned() };
    let parts: Vec<&str> = module_path.split('/').filter(|p| !p.is_empty()).collect();
    let Some(name) = parts.last() else { return };
    let strict = ctx.options.get("strict_checking").and_then(Value::as_bool).unwrap_or(false);
    if !strict && parts.len() > 1 {
        return;
    }
    if name.starts_with('_') && !name.starts_with("__") {
        return;
    }
    let allowed = ctx.options.get("allowed_modules").and_then(Value::as_array);
    if allowed.is_some_and(|l| l.iter().filter_map(Value::as_str).any(|a| a == *name)) {
        return;
    }
    let Some(stdlib) = ctx.data.get("python/stdlib").and_then(|v| v.as_array().cloned()) else { return };
    if !stdlib.iter().filter_map(Value::as_str).any(|m| m == *name) {
        return;
    }
    out.push(span(0, 0, format!("Module `{name}` shadows a Python standard-library module"), Vec::new()));
});

/// The file's dotted module name: its folders up to the first without an
/// `__init__.py`, as Ruff's package root.
fn dotted_module(ctx: &NativeContext<'_>) -> Option<Vec<String>> {
    let path = ctx.file.path();
    if !path.ends_with(".py") && !path.ends_with(".pyi") {
        return None;
    }
    let mut parts = vec![module_name(path).to_owned()];
    let is_init = matches!(file_name(path), "__init__.py" | "__init__.pyi");
    let mut dir = parent_of(path).to_owned();
    if is_init {
        dir = parent_of(&dir).to_owned();
    }
    if !in_package(ctx, parent_of(path)) {
        return None;
    }
    while !dir.is_empty() && in_package(ctx, &dir) {
        parts.insert(0, file_name(&dir).to_owned());
        dir = parent_of(&dir).to_owned();
    }
    Some(parts)
}

rule!(ImportSelf, "py/import-self", &["import_statement", "import_from_statement"], |n, ctx, out| {
    let src = ctx.source;
    let Some(me) = dotted_module(ctx) else { return };
    let dotted = me.join(".");
    if n.kind() == "import_statement" {
        for name in named_children(n) {
            let target = if name.kind() == "aliased_import" { field(name, "name") } else { Some(name) };
            if let Some(t) = target
                && text(t, src) == dotted
            {
                out.push(hit(name, format!("Module `{dotted}` imports itself")));
            }
        }
        return;
    }
    // `from pkg import mod` in pkg/mod.py, `from . import mod` too.
    let Some(module) = field(n, "module_name") else { return };
    let written = text(module, src);
    let level = written.chars().take_while(|c| *c == '.').count();
    let rest = &written[level..];
    let imported: Option<String> = if level == 0 {
        Some(rest.to_owned())
    } else {
        // A relative import resolves against the module's package.
        let is_init = matches!(file_name(ctx.file.path()), "__init__.py" | "__init__.pyi");
        let package_len = if is_init { me.len() } else { me.len() - 1 };
        (level - 1 <= package_len).then(|| {
            let mut base: Vec<&str> = me[..package_len - (level - 1)].iter().map(String::as_str).collect();
            if !rest.is_empty() {
                base.push(rest);
            }
            base.join(".")
        })
    };
    let Some(imported) = imported else { return };
    let (Some(last), parent) = (me.last(), me[..me.len() - 1].join(".")) else { return };
    if imported != parent {
        return;
    }
    for c in named_children(n) {
        if c.id() == module.id() {
            continue;
        }
        let name = if c.kind() == "aliased_import" { field(c, "name") } else { Some(c) };
        if let Some(nm) = name
            && text(nm, src) == last
        {
            out.push(hit(c, format!("Module `{dotted}` imports itself")));
        }
    }
});

// ---------------------------------------------------------------- calls and literals

/// Whether a name's every binding in the enclosing function (or module) is
/// a `str` (or `bytes`) literal or carries that annotation, as Ruff's
/// `is_string` reads a binding.
fn name_is(n: Node<'_>, src: &str, bytes: bool) -> bool {
    let name = text(n, src);
    let want = if bytes { "bytes" } else { "str" };
    let mut scope = n.parent();
    while let Some(s) = scope {
        if matches!(s.kind(), "function_definition" | "module") {
            break;
        }
        scope = s.parent();
    }
    let Some(scope) = scope else { return false };
    let mut found = false;
    let mut ok = true;
    if scope.kind() == "function_definition"
        && let Some(params) = field(scope, "parameters")
    {
        for p in kids(params) {
            let pname = if p.kind() == "identifier" { Some(p) } else { named_children(p).into_iter().find(|k| k.kind() == "identifier") };
            if pname.is_some_and(|x| text(x, src) == name) {
                found = true;
                ok &= field(p, "type").is_some_and(|t| text(t, src) == want);
            }
        }
    }
    let body = if scope.kind() == "module" { Some(scope) } else { field(scope, "body") };
    let mut stack: Vec<Node> = body.into_iter().collect();
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if is_scope(c.kind()) {
                continue;
            }
            if matches!(c.kind(), "assignment" | "augmented_assignment")
                && let Some(l) = field(c, "left")
                && l.kind() == "identifier"
                && text(l, src) == name
            {
                found = true;
                let lit = c.kind() == "assignment" && field(c, "right").is_some_and(|r| r.kind() == "string" && string_parts(r, src).is_some_and(|(p, _)| p.contains('b') == bytes && !p.contains('f')));
                let ann = field(c, "type").is_some_and(|t| text(t, src) == want);
                ok &= lit || ann;
            }
            stack.push(c);
        }
    }
    found && ok
}

rule!(BadStrStripCall, "py/bad-str-strip-call", &["call"], |n, ctx, out| {
    let src = ctx.source;
    let Some(func) = field(n, "function") else { return };
    if func.kind() != "attribute" {
        return;
    }
    let (Some(obj), Some(attr)) = (field(func, "object"), field(func, "attribute")) else { return };
    if !matches!(text(attr, src), "strip" | "lstrip" | "rstrip") {
        return;
    }
    let Some(args) = field(n, "arguments") else { return };
    let a = kids(args);
    let [arg] = a.as_slice() else { return };
    if arg.kind() != "string" {
        return;
    }
    let Some((prefix, content)) = string_parts(*arg, src) else { return };
    if prefix.contains('f') {
        return;
    }
    let bytes = prefix.contains('b');
    let receiver_ok = match obj.kind() {
        "string" => string_parts(obj, src).is_some_and(|(p, _)| p.contains('b') == bytes && !p.contains('f')),
        "identifier" => name_is(obj, src, bytes),
        _ => false,
    };
    if !receiver_ok {
        return;
    }
    let value = if prefix.contains('r') { content.clone() } else { crate::python_util::unescape(&content) };
    let mut seen = BTreeSet::new();
    if value.chars().all(|c| seen.insert(c)) {
        return;
    }
    // The duplicate-free argument strips the same characters; offered only
    // when the literal has no escapes to re-spell.
    let mut edits = Vec::new();
    if !content.contains('\\') {
        let mut seen = BTreeSet::new();
        let dedup: String = content.chars().filter(|c| seen.insert(*c)).collect();
        let start = children(*arg).into_iter().find(|c| c.kind() == "string_content");
        if let Some(sc) = start {
            edits = crate::as_suggestion(vec![TextEdit::new(sc.start_byte(), sc.end_byte(), dedup)]);
        }
    }
    let m = text(attr, src);
    let hint = match m {
        "lstrip" => "; to remove a prefix, use `removeprefix`",
        "rstrip" => "; to remove a suffix, use `removesuffix`",
        _ => "",
    };
    out.push(hit_fix(*arg, format!("`{m}` takes a set of characters: this one repeats some{hint}"), edits));
});

fn int_value(n: Node<'_>, src: &str) -> Option<i128> {
    let n = unparen(n);
    match n.kind() {
        "integer" => {
            let t = text(n, src).replace('_', "").to_ascii_lowercase();
            if let Some(h) = t.strip_prefix("0x") {
                i128::from_str_radix(h, 16).ok()
            } else if let Some(o) = t.strip_prefix("0o") {
                i128::from_str_radix(o, 8).ok()
            } else if let Some(b) = t.strip_prefix("0b") {
                i128::from_str_radix(b, 2).ok()
            } else {
                t.parse().ok()
            }
        }
        "unary_operator" => {
            let op = field(n, "operator").map(|o| text(o, src).to_owned());
            let arg = field(n, "argument")?;
            if arg.kind() != "integer" {
                return None;
            }
            let v = int_value(arg, src)?;
            match op.as_deref() {
                Some("-") => Some(-v),
                _ => None,
            }
        }
        _ => None,
    }
}

rule!(PotentialIndexError, "py/potential-index-error", &["subscript"], |n, ctx, out| {
    let src = ctx.source;
    let Some(value) = field(n, "value") else { return };
    let value = unparen_keep_tuple(value);
    if !matches!(value.kind(), "tuple" | "list") {
        return;
    }
    let subs: Vec<Node> = named_children(n).into_iter().filter(|c| c.id() != value.id() && Some(c.id()) != field(n, "value").map(|v| v.id()) && c.kind() != "comment").collect();
    let [index] = subs.as_slice() else { return };
    let len = kids(value).len() as i128;
    if kids(value).iter().any(|k| matches!(k.kind(), "list_splat" | "parenthesized_list_splat")) {
        return;
    }
    let Some(i) = int_value(*index, src) else { return };
    if i >= len || i < -len {
        out.push(hit(*index, format!("Index {i} is out of range for a sequence of {len}")));
    }
});

/// Parentheses around a tuple literal are the tuple's own.
fn unparen_keep_tuple(n: Node<'_>) -> Node<'_> {
    if n.kind() == "parenthesized_expression" { unparen(n) } else { n }
}

/// `%` conversions in a logging message: the positional count, or `None`
/// when a mapping key, a `*` width or a malformed spec makes it unknowable.
fn percent_placeholders(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut count = 0;
    while i < chars.len() {
        if chars[i] != '%' {
            i += 1;
            continue;
        }
        i += 1;
        if i >= chars.len() {
            return None;
        }
        if chars[i] == '%' {
            i += 1;
            continue;
        }
        if chars[i] == '(' {
            return None;
        }
        while i < chars.len() && "#0- +".contains(chars[i]) {
            i += 1;
        }
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '*' || chars[i] == '.') {
            if chars[i] == '*' {
                return None;
            }
            i += 1;
        }
        while i < chars.len() && "hlL".contains(chars[i]) {
            i += 1;
        }
        if i >= chars.len() || !"diouxXeEfFgGcrsa".contains(chars[i]) {
            return None;
        }
        count += 1;
        i += 1;
    }
    Some(count)
}

const LEVELS: &[&str] = &["debug", "info", "warn", "warning", "error", "critical", "exception"];

/// A logging call's message and its positional and keyword argument
/// counts, when the callee is a logger method or a `logging` function.
fn logging_call<'t>(n: Node<'t>, src: &str) -> Option<(Node<'t>, usize, usize)> {
    let func = field(n, "function")?;
    let level = match func.kind() {
        "attribute" => {
            let obj = field(func, "object")?;
            let attr = text(field(func, "attribute")?, src);
            let receiver = text(obj, src);
            let last = receiver.rsplit('.').next().unwrap_or(receiver).to_ascii_lowercase();
            if receiver != "logging" && !last.contains("log") {
                return None;
            }
            attr
        }
        "identifier" => {
            let name = text(func, src);
            // `from logging import info`.
            let imported = src.lines().any(|l| {
                let l = l.trim();
                l.starts_with("from logging import") && l.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == name)
            });
            if !imported {
                return None;
            }
            name
        }
        _ => return None,
    };
    if !LEVELS.contains(&level) {
        return None;
    }
    let args = field(n, "arguments")?;
    let all = kids(args);
    if all.iter().any(|a| matches!(a.kind(), "list_splat" | "dictionary_splat")) {
        return None;
    }
    let positional: Vec<Node> = all.iter().copied().filter(|a| a.kind() != "keyword_argument").collect();
    let keywords = all.len() - positional.len();
    let first = *positional.first()?;
    Some((first, positional.len() - 1, keywords))
}

fn logging_counts<'t>(n: Node<'t>, src: &str) -> Option<(Node<'t>, usize, usize, usize)> {
    let (msg, args, keywords) = logging_call(n, src)?;
    if msg.kind() != "string" {
        return None;
    }
    let value = crate::python_util::string_value(msg, src)?;
    let placeholders = percent_placeholders(&value)?;
    Some((field(n, "function")?, placeholders, args, keywords))
}

rule!(LoggingTooManyArgs, "py/logging-too-many-args", &["call"], |n, ctx, out| {
    let Some((func, placeholders, args, _)) = logging_counts(n, ctx.source) else { return };
    if placeholders < args {
        out.push(hit(func, format!("The message has {placeholders} placeholder(s) for {args} argument(s)")));
    }
});

rule!(LoggingTooFewArgs, "py/logging-too-few-args", &["call"], |n, ctx, out| {
    let Some((func, placeholders, args, keywords)) = logging_counts(n, ctx.source) else { return };
    if args > 0 && keywords == 0 && placeholders > args {
        out.push(hit(func, format!("The message has {placeholders} placeholders for {args} argument(s)")));
    }
});

// ---------------------------------------------------------------- counting

/// Pylint's statement count over a body, as Ruff ports it: a branch or a
/// handler counts as one, a `return` as none, a `for` adds only its bodies.
fn num_statements(stmts: &[Node<'_>]) -> usize {
    let mut count = 0;
    for s in stmts {
        let s = undecorated(*s);
        match s.kind() {
            "if_statement" => {
                count += 1 + num_statements(&body_of(s));
                for c in named_children(s) {
                    match c.kind() {
                        "elif_clause" | "else_clause" => count += 1 + num_statements(&body_of(c)),
                        _ => {}
                    }
                }
            }
            "for_statement" => {
                count += num_statements(&body_of(s));
                if let Some(e) = field(s, "alternative") {
                    count += num_statements(&body_of(e));
                }
            }
            "while_statement" => {
                count += 1 + num_statements(&body_of(s));
                if let Some(e) = field(s, "alternative") {
                    count += num_statements(&body_of(e));
                }
            }
            "match_statement" => {
                count += 1;
                if let Some(b) = field(s, "body") {
                    for case in kids(b) {
                        if case.kind() == "case_clause" {
                            count += 1 + num_statements(&body_of(case));
                        }
                    }
                }
            }
            "try_statement" => {
                count += 1 + num_statements(&body_of(s));
                let handlers: Vec<Node> = named_children(s).into_iter().filter(|c| matches!(c.kind(), "except_clause" | "except_group_clause")).collect();
                for c in named_children(s) {
                    match c.kind() {
                        "else_clause" => count += 1 + num_statements(&body_of(c)),
                        "finally_clause" => count += 2 + num_statements(&body_of(c)),
                        _ => {}
                    }
                }
                if handlers.len() > 1 {
                    count += 1;
                }
                for h in handlers {
                    count += 1 + num_statements(&body_of(h));
                }
            }
            "function_definition" | "with_statement" => count += 1 + num_statements(&body_of(s)),
            "return_statement" => {}
            _ => count += 1,
        }
    }
    count
}

/// The nodes of a kind in a body, not inside a nested function, class or
/// lambda.
fn collect_in<'t>(root: Node<'t>, kinds: &[&str], out: &mut Vec<Node<'t>>) {
    for c in named_children(root) {
        if is_scope(c.kind()) {
            continue;
        }
        if kinds.contains(&c.kind()) {
            out.push(c);
        }
        collect_in(c, kinds, out);
    }
}

fn function_name(n: Node<'_>) -> Option<Node<'_>> {
    field(n, "name")
}

rule!(TooManyReturnStatements, "py/too-many-return-statements", &["function_definition"], |n, ctx, out| {
    let max = uint(ctx.options, "max", 6);
    let Some(body) = field(n, "body") else { return };
    let mut returns = Vec::new();
    collect_in(body, &["return_statement"], &mut returns);
    if returns.len() > max {
        let name = function_name(n).unwrap_or(n);
        out.push(hit(name, format!("Too many return statements ({} > {max})", returns.len())));
    }
});

rule!(TooManyStatements, "py/too-many-statements", &["function_definition"], |n, ctx, out| {
    let max = uint(ctx.options, "max", 50);
    let count = num_statements(&body_of(n));
    if count > max {
        let name = function_name(n).unwrap_or(n);
        out.push(hit(name, format!("Too many statements ({count} > {max})")));
    }
});

rule!(TooManyStatementsInTryClause, "py/too-many-statements-in-try-clause", &["try_statement"], |n, ctx, out| {
    let max = uint(ctx.options, "max", 5);
    if !named_children(n).iter().any(|c| matches!(c.kind(), "except_clause" | "except_group_clause")) {
        return;
    }
    let count = num_statements(&body_of(n));
    if count > max {
        out.push(span(n.start_byte(), n.start_byte() + 3, format!("The `try` clause has too many statements ({count} > {max})"), Vec::new()));
    }
});

/// The names a target binds: an identifier, or each identifier of a
/// tuple or list pattern.
fn target_names<'t>(t: Node<'t>, out: &mut Vec<Node<'t>>) {
    match t.kind() {
        "identifier" => out.push(t),
        "pattern_list" | "tuple_pattern" | "list_pattern" | "list_splat_pattern" | "parenthesized_expression" | "tuple" | "list" | "expression_list" => {
            for c in kids(t) {
                target_names(c, out);
            }
        }
        _ => {}
    }
}

rule!(TooManyLocals, "py/too-many-locals", &["function_definition"], |n, ctx, out| {
    let src = ctx.source;
    let max = uint(ctx.options, "max", 15);
    let Some(body) = field(n, "body") else { return };
    // The last binding of each name, in source order: an assignment or not.
    let mut last: HashMap<String, bool> = HashMap::new();
    let mut declared = BTreeSet::new();
    let mut stack = vec![body];
    let mut events: Vec<(usize, String, bool)> = Vec::new();
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            match c.kind() {
                "function_definition" | "class_definition" => {
                    if let Some(nm) = field(c, "name") {
                        events.push((c.start_byte(), text(nm, src).to_owned(), false));
                    }
                    continue;
                }
                "lambda" | "list_comprehension" | "set_comprehension" | "dictionary_comprehension" | "generator_expression" => continue,
                "global_statement" | "nonlocal_statement" => {
                    for i in kids(c) {
                        declared.insert(text(i, src).to_owned());
                    }
                }
                "assignment" => {
                    // `x: int` without a value is an annotation, not a binding.
                    if field(c, "right").is_some() {
                        let mut names = Vec::new();
                        if let Some(l) = field(c, "left") {
                            target_names(l, &mut names);
                        }
                        // A chained `a = b = 1` nests its other targets in `right`.
                        for nm in names {
                            events.push((nm.start_byte(), text(nm, src).to_owned(), true));
                        }
                    }
                }
                "augmented_assignment" => {
                    if let Some(l) = field(c, "left")
                        && l.kind() == "identifier"
                    {
                        events.push((l.start_byte(), text(l, src).to_owned(), true));
                    }
                }
                "for_statement" | "for_in_clause" => {
                    let mut names = Vec::new();
                    if let Some(l) = field(c, "left") {
                        target_names(l, &mut names);
                    }
                    for nm in names {
                        events.push((nm.start_byte(), text(nm, src).to_owned(), false));
                    }
                }
                "as_pattern" | "named_expression" => {
                    let target = if c.kind() == "named_expression" { field(c, "name") } else { field(c, "alias") };
                    if let Some(t) = target {
                        let mut names = Vec::new();
                        target_names(t, &mut names);
                        if t.kind() == "as_pattern_target" {
                            names.extend(kids(t).into_iter().filter(|k| k.kind() == "identifier"));
                        }
                        for nm in names {
                            events.push((nm.start_byte(), text(nm, src).to_owned(), false));
                        }
                    }
                }
                "import_statement" | "import_from_statement" => {
                    for i in named_children(c) {
                        let bound = if i.kind() == "aliased_import" { field(i, "alias") } else if i.kind() == "dotted_name" && Some(i.id()) != field(c, "module_name").map(|m| m.id()) { i.named_child(0) } else { None };
                        if let Some(b) = bound {
                            events.push((b.start_byte(), text(b, src).to_owned(), false));
                        }
                    }
                    continue;
                }
                _ => {}
            }
            stack.push(c);
        }
    }
    events.sort_by_key(|e| e.0);
    for (_, name, is_assign) in events {
        last.insert(name, is_assign);
    }
    let count = last.iter().filter(|(k, v)| **v && !declared.contains(*k)).count();
    if count > max {
        let name = function_name(n).unwrap_or(n);
        out.push(hit(name, format!("Too many local variables ({count} > {max})")));
    }
});

fn count_bools(n: Node<'_>) -> usize {
    let n = unparen(n);
    if n.kind() != "boolean_operator" {
        return 1;
    }
    let (Some(l), Some(r)) = (field(n, "left"), field(n, "right")) else { return 1 };
    count_bools(l) + count_bools(r)
}

rule!(TooManyBooleanExpressions, "py/too-many-boolean-expressions", &["if_statement", "elif_clause"], |n, ctx, out| {
    let max = uint(ctx.options, "max", 5);
    let Some(cond) = field(n, "condition") else { return };
    let cond = unparen(cond);
    if cond.kind() != "boolean_operator" {
        return;
    }
    let count = count_bools(cond);
    if count > max {
        out.push(hit(cond, format!("Too many Boolean expressions ({count} > {max})")));
    }
});

/// The methods of a class body: each `def`, decorated or not, with its
/// decorators.
fn methods<'t>(class: Node<'t>) -> Vec<(Node<'t>, Node<'t>)> {
    let mut out = Vec::new();
    for s in body_of(class) {
        let def = undecorated(s);
        if def.kind() == "function_definition" {
            out.push((s, def));
        }
    }
    out
}

fn decorator_names(stmt: Node<'_>, src: &str) -> Vec<String> {
    if stmt.kind() != "decorated_definition" {
        return Vec::new();
    }
    decorators(undecorated(stmt)).into_iter().map(|d| text(d, src).trim_start_matches('@').trim().to_owned()).collect()
}

/// Ruff's method visibility: a property's setter or deleter is private; a
/// name without a leading underscore, or a dunder, is public.
fn is_public_method(stmt: Node<'_>, def: Node<'_>, src: &str) -> bool {
    let name = field(def, "name").map(|n| text(n, src)).unwrap_or("");
    if decorator_names(stmt, src).iter().any(|d| d == &format!("{name}.setter") || d == &format!("{name}.deleter")) {
        return false;
    }
    !name.starts_with('_') || (name.starts_with("__") && name.ends_with("__") && name.len() > 4)
}

rule!(TooManyPublicMethods, "py/too-many-public-methods", &["class_definition"], |n, ctx, out| {
    if is_stub(ctx) {
        return;
    }
    let src = ctx.source;
    let max = uint(ctx.options, "max", 20);
    let count = methods(n)
        .into_iter()
        .filter(|(s, d)| is_public_method(*s, *d, src) && !decorator_names(*s, src).iter().any(|d| d == "overload" || d == "typing.overload" || d == "typing_extensions.overload"))
        .count();
    if count > max {
        out.push(hit(n, format!("Too many public methods ({count} > {max})")));
    }
});

rule!(TooManyInstanceAttributes, "py/too-many-instance-attributes", &["class_definition"], |n, ctx, out| {
    let src = ctx.source;
    let max = uint(ctx.options, "max", 7);
    let mut attrs = BTreeSet::new();
    for (stmt, def) in methods(n) {
        let decos = decorator_names(stmt, src);
        if decos.iter().any(|d| d == "staticmethod" || d == "classmethod") {
            continue;
        }
        let Some(params) = field(def, "parameters") else { continue };
        let Some(first) = kids(params).into_iter().next() else { continue };
        let first = if first.kind() == "identifier" { first } else { match named_children(first).into_iter().find(|c| c.kind() == "identifier") { Some(f) => f, None => continue } };
        let me = text(first, src);
        let Some(body) = field(def, "body") else { continue };
        let mut stack = vec![body];
        while let Some(x) = stack.pop() {
            for c in named_children(x) {
                if c.kind() == "class_definition" {
                    continue;
                }
                if matches!(c.kind(), "assignment" | "augmented_assignment") {
                    let mut targets = Vec::new();
                    if let Some(l) = field(c, "left") {
                        attr_targets(l, &mut targets);
                    }
                    for t in targets {
                        if let (Some(o), Some(a)) = (field(t, "object"), field(t, "attribute"))
                            && text(o, src) == me
                        {
                            attrs.insert(text(a, src).to_owned());
                        }
                    }
                }
                stack.push(c);
            }
        }
    }
    if attrs.len() > max {
        let name = field(n, "name").unwrap_or(n);
        out.push(hit(name, format!("Too many instance attributes ({} > {max})", attrs.len())));
    }
});

fn attr_targets<'t>(t: Node<'t>, out: &mut Vec<Node<'t>>) {
    match t.kind() {
        "attribute" => out.push(t),
        "pattern_list" | "tuple_pattern" | "list_pattern" | "list_splat_pattern" | "parenthesized_expression" => {
            for c in kids(t) {
                attr_targets(c, out);
            }
        }
        _ => {}
    }
}

rule!(TooFewPublicMethods, "py/too-few-public-methods", &["class_definition"], |n, ctx, out| {
    let src = ctx.source;
    let min = uint(ctx.options, "min", 2);
    // Only a class with no base but `object`: Pylint counts the methods it
    // inherits, which codelaw cannot see across files.
    if let Some(bases) = field(n, "superclasses") {
        let b = kids(bases);
        if !(b.is_empty() || (b.len() == 1 && text(b[0], src) == "object")) {
            return;
        }
    }
    // A dataclass or an attrs class is data, not behaviour.
    if let Some(parent) = n.parent()
        && parent.kind() == "decorated_definition"
    {
        let names = decorator_names(parent, src);
        let data = names.iter().any(|d| {
            let callee = d.split('(').next().unwrap_or(d);
            let last = callee.rsplit('.').next().unwrap_or(callee);
            matches!(last, "dataclass" | "attrs" | "define" | "frozen")
        });
        if data {
            return;
        }
    }
    let count = methods(n).into_iter().filter(|(_, d)| field(*d, "name").is_some_and(|nm| !text(nm, src).starts_with('_'))).count();
    if count < min {
        let name = field(n, "name").unwrap_or(n);
        out.push(hit(name, format!("Too few public methods ({count} < {min})")));
    }
});

// ---------------------------------------------------------------- shapes

/// Whether an expression is a literal Ruff's `as_literal_expr` accepts.
fn is_literal(n: Node<'_>) -> bool {
    matches!(n.kind(), "string" | "concatenated_string" | "integer" | "float" | "true" | "false" | "none" | "ellipsis")
        || (n.kind() == "unary_operator" && field(n, "argument").is_some_and(|a| matches!(a.kind(), "integer" | "float")))
}

/// Whether evaluating an expression can have an effect: a call, an
/// `await`, a `yield` or an assignment expression anywhere in it.
fn has_effect(n: Node<'_>) -> bool {
    if matches!(n.kind(), "call" | "await" | "yield" | "named_expression") {
        return true;
    }
    named_children(n).into_iter().any(has_effect)
}

/// `name == <literal>`: the name and the literal's text.
fn eq_test<'s>(cond: Node<'_>, src: &'s str) -> Option<(&'s str, &'s str)> {
    let cond = unparen(cond);
    if cond.kind() != "comparison_operator" {
        return None;
    }
    let operands = kids(cond);
    let ops: Vec<&str> = children(cond).into_iter().filter(|c| !c.is_named()).map(|c| text(c, src)).collect();
    if operands.len() != 2 || ops != ["=="] || operands[0].kind() != "identifier" || !is_literal(operands[1]) {
        return None;
    }
    Some((text(operands[0], src), text(operands[1], src)))
}

/// A body that is one `return`, whose value has no effect.
fn single_return(stmts: &[Node<'_>]) -> bool {
    match stmts {
        [s] if s.kind() == "return_statement" => named_children(*s).into_iter().all(|v| !has_effect(v)),
        _ => false,
    }
}

rule!(IfElseBlockInsteadOfDictLookup, "py/if-else-block-instead-of-dict-lookup", &["if_statement"], |n, ctx, out| {
    let src = ctx.source;
    let Some((target, first)) = field(n, "condition").and_then(|c| eq_test(c, src)) else { return };
    if !single_return(&body_of(n)) {
        return;
    }
    let mut literals = BTreeSet::new();
    literals.insert(first.to_owned());
    for c in named_children(n) {
        match c.kind() {
            "elif_clause" => {
                let Some((name, lit)) = field(c, "condition").and_then(|x| eq_test(x, src)) else { return };
                if name != target || !single_return(&body_of(c)) {
                    return;
                }
                literals.insert(lit.to_owned());
            }
            "else_clause" if !single_return(&body_of(c)) => return,
            _ => {}
        }
    }
    if literals.len() < 3 {
        return;
    }
    out.push(hit(n, format!("Use a dictionary lookup on `{target}` instead of an `if` chain")));
});

/// The length a value unpacks to, as Pylint reads a literal: a tuple,
/// list, set or dict's items, a string's characters; `None` when unknown.
fn unpack_len(n: Node<'_>, src: &str) -> Option<usize> {
    let n = unparen_keep_tuple(n);
    match n.kind() {
        "tuple" | "list" | "set" | "expression_list" => {
            let items = kids(n);
            if items.iter().any(|i| matches!(i.kind(), "list_splat" | "parenthesized_list_splat")) {
                return None;
            }
            Some(items.len())
        }
        "dictionary" => {
            let items = kids(n);
            if items.iter().any(|i| i.kind() == "dictionary_splat") {
                return None;
            }
            Some(items.len())
        }
        "string" => crate::python_util::string_value(n, src).map(|s| s.chars().count()),
        _ => None,
    }
}

fn has_star(targets: &[Node<'_>]) -> bool {
    targets.iter().any(|t| t.kind() == "list_splat_pattern")
}

fn unpacking_targets<'t>(left: Node<'t>) -> Option<Vec<Node<'t>>> {
    matches!(left.kind(), "pattern_list" | "tuple_pattern" | "list_pattern").then(|| kids(left))
}

fn in_comprehension(n: Node<'_>) -> bool {
    let mut p = n.parent();
    while let Some(x) = p {
        if x.kind().ends_with("comprehension") || x.kind() == "generator_expression" {
            return true;
        }
        if is_scope(x.kind()) {
            return false;
        }
        p = x.parent();
    }
    false
}

rule!(UnbalancedTupleUnpacking, "py/unbalanced-tuple-unpacking", &["assignment"], |n, ctx, out| {
    let src = ctx.source;
    let (Some(left), Some(right)) = (field(n, "left"), field(n, "right")) else { return };
    let Some(targets) = unpacking_targets(left) else { return };
    if has_star(&targets) || in_comprehension(n) {
        return;
    }
    let r = unparen_keep_tuple(right);
    if !matches!(r.kind(), "tuple" | "list" | "set" | "expression_list") {
        return;
    }
    let Some(values) = unpack_len(r, src) else { return };
    if values != targets.len() {
        out.push(hit(n, format!("Unbalanced unpacking: {} target(s), {values} value(s)", targets.len())));
    }
});

rule!(UnbalancedDictUnpacking, "py/unbalanced-dict-unpacking", &["assignment", "for_statement"], |n, ctx, out| {
    let src = ctx.source;
    if n.kind() == "assignment" {
        let (Some(left), Some(right)) = (field(n, "left"), field(n, "right")) else { return };
        let Some(targets) = unpacking_targets(left) else { return };
        if has_star(&targets) || in_comprehension(n) {
            return;
        }
        let r = unparen(right);
        if r.kind() != "dictionary" {
            return;
        }
        let Some(values) = unpack_len(r, src) else { return };
        if values != targets.len() {
            out.push(hit(n, format!("Unbalanced dict unpacking: {} target(s), {values} key(s)", targets.len())));
        }
        return;
    }
    // `for a, b in {...}` and `.keys()`, `.values()`, `.items()` on a literal.
    let (Some(left), Some(right)) = (field(n, "left"), field(n, "right")) else { return };
    let Some(targets) = unpacking_targets(left) else { return };
    let starred = has_star(&targets);
    let right = unparen(right);
    let (dict, method) = match right.kind() {
        "dictionary" => (right, "keys"),
        "call" => {
            let Some(f) = field(right, "function") else { return };
            if f.kind() != "attribute" || field(right, "arguments").is_some_and(|a| !kids(a).is_empty()) {
                return;
            }
            let (Some(o), Some(a)) = (field(f, "object"), field(f, "attribute")) else { return };
            let o = unparen(o);
            if o.kind() != "dictionary" {
                return;
            }
            match text(a, src) {
                m @ ("keys" | "values" | "items") => (o, m),
                _ => return,
            }
        }
        _ => return,
    };
    let pairs: Vec<Node> = kids(dict).into_iter().filter(|p| p.kind() == "pair").collect();
    if pairs.is_empty() || pairs.len() != kids(dict).len() {
        return;
    }
    if method == "items" {
        if targets.len() == 2 || starred {
            return;
        }
        out.push(hit(n, format!("Unbalanced dict unpacking: {} target(s), each item has 2 values", targets.len())));
        return;
    }
    for p in pairs {
        let v = if method == "keys" { field(p, "key") } else { field(p, "value") };
        let Some(v) = v else { return };
        let len = unpack_len(v, src).unwrap_or(1);
        if len != targets.len() && !(starred && len >= targets.len()) {
            out.push(hit(n, format!("Unbalanced dict unpacking: {} target(s), a {} with {len} value(s)", targets.len(), if method == "keys" { "key" } else { "value" })));
            return;
        }
    }
});

pub static RULES: &[&dyn NativeRule] = &[
    &InvalidCharacterBackspace,
    &InvalidCharacterSub,
    &InvalidCharacterEsc,
    &InvalidCharacterZeroWidthSpace,
    &UnsortedDunderSlots,
    &ImplicitNamespacePackage,
    &InvalidModuleName,
    &NonAsciiFileName,
    &StdlibModuleShadowing,
    &ImportSelf,
    &BadStrStripCall,
    &PotentialIndexError,
    &LoggingTooManyArgs,
    &LoggingTooFewArgs,
    &TooManyReturnStatements,
    &TooManyStatements,
    &TooManyStatementsInTryClause,
    &TooManyLocals,
    &TooManyBooleanExpressions,
    &TooManyPublicMethods,
    &TooManyInstanceAttributes,
    &TooFewPublicMethods,
    &IfElseBlockInsteadOfDictLookup,
    &UnbalancedTupleUnpacking,
    &UnbalancedDictUnpacking,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryFiles;
    use std::sync::Arc;

    fn run_at(rule: &dyn NativeRule, path: &str, src: &str, files: &[(&str, &str)]) -> Vec<NativeFinding> {
        let root = codelaw_lang::parse(Language::Python, src);
        let options = BTreeMap::new();
        let mut data = BTreeMap::new();
        data.insert("python/stdlib".to_owned(), Arc::new(serde_json::json!(["os", "logging", "json"])));
        let file = MemoryFiles::new(path, src, files);
        let ctx = NativeContext { source: src, language: Language::Python, semantic: None, options: &options, data: &data, file: &file };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            if rule.kinds().contains(&node.kind().as_ref()) {
                rule.check(node, &ctx, &mut out);
            }
        }
        out
    }

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        run_at(rule, "example.py", src, &[])
    }

    #[test]
    fn statements_count_like_pylint() {
        let src = "def f():\n    if a:\n        b()\n    elif c:\n        d()\n    else:\n        e()\n    for x in y:\n        g()\n    try:\n        h()\n    except A:\n        i()\n    except B:\n        j()\n    finally:\n        k()\n    return 1\n";
        let root = codelaw_lang::parse(Language::Python, src);
        let def = root.root().get_inner_node().named_child(0).unwrap();
        // if 1+1, elif 1+1, else 1+1, for 1, try 1+1, handlers 1 + 2*(1+1), finally 2+1.
        assert_eq!(num_statements(&body_of(def)), 6 + 1 + 2 + 1 + 4 + 3);
    }

    #[test]
    fn files() {
        assert_eq!(run_at(&ImplicitNamespacePackage, "pkg/mod.py", "x = 1\n", &[]).len(), 1);
        assert!(run_at(&ImplicitNamespacePackage, "pkg/mod.py", "x = 1\n", &[("pkg/__init__.py", "")]).is_empty());
        assert!(run_at(&ImplicitNamespacePackage, "setup.py", "x = 1\n", &[]).is_empty());
        assert_eq!(run_at(&InvalidModuleName, "pkg/BadName.py", "", &[("pkg/__init__.py", "")]).len(), 1);
        assert!(run_at(&InvalidModuleName, "pkg/migrations/0001_initial.py", "", &[("pkg/migrations/__init__.py", "")]).is_empty());
        assert_eq!(run_at(&StdlibModuleShadowing, "logging.py", "", &[]).len(), 1);
        assert!(run_at(&StdlibModuleShadowing, "app/logging.py", "", &[]).is_empty());
        assert_eq!(run_at(&ImportSelf, "pkg/mod.py", "import pkg.mod\nfrom pkg import mod\nfrom . import mod\n", &[("pkg/__init__.py", "")]).len(), 3);
    }

    #[test]
    fn literals() {
        assert_eq!(run(&PotentialIndexError, "(1, 2)[2]\n[1][-2]\n(1, 2)[1]\n").len(), 2);
        assert_eq!(run(&BadStrStripCall, "'x'.strip('aab')\n").len(), 1);
        assert_eq!(run(&LoggingTooManyArgs, "logging.info('%s', a, b)\n").len(), 1);
        assert_eq!(run(&LoggingTooFewArgs, "logger.info('%s %s', a)\n").len(), 1);
        assert!(run(&LoggingTooFewArgs, "logger.info('%(a)s %s', a)\n").is_empty());
        assert_eq!(run(&UnbalancedTupleUnpacking, "a, b = 1, 2, 3\n").len(), 1);
        assert_eq!(run(&UnbalancedDictUnpacking, "for a, b in {'abc': 1}:\n    pass\n").len(), 1);
        assert!(run(&UnbalancedDictUnpacking, "for a, b in {'ab': 1}.items():\n    pass\n").is_empty());
    }
}

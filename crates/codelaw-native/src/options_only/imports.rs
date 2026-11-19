//! The import rules a project configures: `js/import-prefer-namespace-import`
//! (eslint-plugin-import-x 4.17.1), `js/import-no-restricted-paths`
//! (eslint-plugin-import 2.32.0), `node/no-restricted-import` and
//! `node/no-restricted-require` (eslint-plugin-n 18.5.0) — each read and
//! reimplemented, and each silent until its option is set.

use super::{TextMatcher, at, finding, option_problem, opt_strings, orule};
use crate::util::{children, string_value, text};
use crate::{NativeContext, NativeFinding, resolve_path};
use codelaw_core::TextEdit;
use globset::{Glob, GlobBuilder};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use tree_sitter::Node;

// ---------------------------------------------------------------------------
// js/import-prefer-namespace-import

orule!(ImportPreferNamespaceImport, "js/import-prefer-namespace-import", &["patterns"], &["import_statement"], false, |n, ctx, out| {
    let src = ctx.source;
    let Some(source) = n.child_by_field_name("source") else { return };
    let Some(clause) = children(n).into_iter().find(|c| c.kind() == "import_clause") else { return };
    let Some(default) = children(clause).into_iter().find(|c| c.kind() == "identifier") else { return };
    let mut matchers = Vec::new();
    for p in opt_strings(ctx, "patterns") {
        match TextMatcher::parse(&p) {
            Ok(m) => matchers.push(m),
            Err(e) => return option_problem(n, &["import_statement"], out, format!("`patterns` entry `{p}` is not a valid regular expression: {e}")),
        }
    }
    let module = string_value(source, src);
    if !matchers.iter().any(|m| m.is_match(&module)) {
        return;
    }
    let local = text(default, src);
    let others: Vec<Node<'_>> = children(clause).into_iter().filter(|c| matches!(c.kind(), "named_imports" | "namespace_import")).collect();
    let message = format!("Prefer importing {local} as 'import * as {local} from \"{module}\"';");
    if others.is_empty() {
        out.push(finding(n.start_byte(), n.end_byte(), message, vec![TextEdit::new(default.start_byte(), default.end_byte(), format!("* as {local}"))]));
        return;
    }
    // `import a, { b } from "m"` becomes two statements; with a namespace import already there is no fix.
    let named = others.iter().find(|c| c.kind() == "named_imports");
    let edits = match named {
        Some(named) if others.len() == 1 => {
            let prefix = if children(n).iter().any(|c| c.kind() == "type") { "import type" } else { "import" };
            let rest = src[named.end_byte()..n.end_byte()].trim_start();
            vec![TextEdit::new(n.start_byte(), n.end_byte(), format!("{prefix} * as {local} {rest}\n{prefix} {}{}", text(*named, src), &src[named.end_byte()..n.end_byte()]))]
        }
        _ => Vec::new(),
    };
    out.push(finding(default.start_byte(), default.end_byte(), message, edits));
});

// ---------------------------------------------------------------------------
// the module a statement or call names

/// `(the string node, its value)` of every module the node names.
fn modules<'t>(n: Node<'t>, src: &str, with_require: bool, with_import: bool) -> Vec<(Node<'t>, String)> {
    let mut v = Vec::new();
    match n.kind() {
        "import_statement" | "export_statement" if with_import => {
            if let Some(s) = n.child_by_field_name("source").filter(|s| s.kind() == "string") {
                v.push((s, string_value(s, src)));
            }
        }
        "call_expression" => {
            let Some(f) = n.child_by_field_name("function") else { return v };
            let callee = text(f, src);
            let wanted = (with_require && callee == "require") || (with_import && callee == "import");
            if !wanted {
                return v;
            }
            let Some(args) = n.child_by_field_name("arguments") else { return v };
            let named: Vec<Node<'_>> = children(args).into_iter().filter(|c| c.is_named() && c.kind() != "comment").collect();
            if let [only] = named.as_slice()
                && (only.kind() == "string" || (only.kind() == "template_string" && !text(*only, src).contains("${")))
            {
                v.push((*only, string_value(*only, src)));
            }
        }
        _ => {}
    }
    v
}

fn glob(pattern: &str) -> Option<globset::GlobMatcher> {
    static CACHE: LazyLock<Mutex<HashMap<String, Option<globset::GlobMatcher>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    cache.entry(pattern.to_owned()).or_insert_with(|| GlobBuilder::new(pattern).literal_separator(true).build().ok().map(|g: Glob| g.compile_matcher())).clone()
}

// ---------------------------------------------------------------------------
// node/no-restricted-import, node/no-restricted-require

/// The globs of `modules`, in order: a `!`-prefixed one takes a name back out.
struct Restriction {
    globs: Vec<(bool, String)>,
}

impl Restriction {
    /// The glob that restricts `name`, when one does.
    fn matching(&self, name: &str) -> Option<&str> {
        let mut by: Option<&str> = None;
        for (negate, g) in &self.globs {
            let hit = glob(g).is_some_and(|m| m.is_match(name));
            if *negate {
                if hit {
                    by = None;
                }
            } else if hit {
                by = Some(g);
            }
        }
        by
    }
}

fn restrictions(ctx: &NativeContext<'_>) -> Result<Restriction, String> {
    let mut globs = Vec::new();
    for raw in opt_strings(ctx, "modules") {
        let negate = raw.starts_with('!') && !raw.starts_with("!(");
        let pattern = if negate { raw[1..].to_owned() } else { raw };
        if glob(&pattern).is_none() {
            return Err(format!("`modules` has `{pattern}`, which is not a valid glob"));
        }
        globs.push((negate, pattern));
    }
    Ok(Restriction { globs })
}

fn restricted(n: Node<'_>, ctx: &NativeContext<'_>, kinds: &[&str], require: bool, out: &mut Vec<NativeFinding>) {
    let r = match restrictions(ctx) {
        Ok(r) => r,
        Err(e) => return option_problem(n, kinds, out, e),
    };
    let messages = ctx.options.get("messages").and_then(Value::as_object);
    for (node, name) in modules(n, ctx.source, require, !require) {
        if let Some(by) = r.matching(&name) {
            let extra = messages.and_then(|m| m.get(by)).and_then(Value::as_str).map(|m| format!(" {m}")).unwrap_or_default();
            out.push(at(node, format!("'{name}' module is restricted from being used.{extra}")));
        }
    }
}

orule!(NoRestrictedImport, "node/no-restricted-import", &["modules"], &["import_statement", "export_statement", "call_expression"], false, |n, ctx, out| {
    restricted(n, ctx, &["import_statement", "export_statement", "call_expression"], false, out);
});

orule!(NoRestrictedRequire, "node/no-restricted-require", &["modules"], &["call_expression"], false, |n, ctx, out| {
    restricted(n, ctx, &["call_expression"], true, out);
});

// ---------------------------------------------------------------------------
// js/import-no-restricted-paths

const EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx", "mjs", "cjs", "mts", "cts", "json"];

/// The project file an import names, through the extensions and `index` files
/// a resolver tries; the path itself when none exists.
fn resolve_file(ctx: &NativeContext<'_>, from: &str, spec: &str) -> Option<String> {
    let base = resolve_path(from, spec)?;
    if ctx.file.exists(&base) && !ctx.file.is_dir(&base) {
        return Some(base);
    }
    for e in EXTENSIONS {
        let c = format!("{base}.{e}");
        if ctx.file.exists(&c) {
            return Some(c);
        }
    }
    if ctx.file.is_dir(&base) {
        for e in EXTENSIONS {
            let c = format!("{base}/index.{e}");
            if ctx.file.exists(&c) {
                return Some(c);
            }
        }
    }
    Some(base)
}

fn is_glob(p: &str) -> bool {
    p.chars().any(|c| matches!(c, '*' | '?' | '[' | '{' | '!'))
}

/// A path pattern under `base`: a glob, or a directory that holds its files.
fn path_matches(pattern: &str, base: &str, path: &str) -> bool {
    let pat = join(base, pattern);
    if is_glob(pattern) {
        glob(&pat).is_some_and(|m| m.is_match(path))
    } else {
        path == pat || path.strip_prefix(&pat).is_some_and(|r| r.starts_with('/')) || pat.is_empty()
    }
}

/// `base/rel` as a project-relative path, `./` and `..` folded.
fn join(base: &str, rel: &str) -> String {
    let mut parts: Vec<&str> = base.split('/').filter(|s| !s.is_empty() && *s != ".").collect();
    for seg in rel.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    parts.join("/")
}

fn list(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect(),
        _ => Vec::new(),
    }
}

orule!(ImportNoRestrictedPaths, "js/import-no-restricted-paths", &["zones"], &["import_statement", "export_statement", "call_expression"], false, |n, ctx, out| {
    let Some(Value::Object(zones)) = ctx.options.get("zones") else { return };
    let base = ctx.options.get("basePath").and_then(Value::as_str).unwrap_or("");
    let file = ctx.file.path().to_owned();
    for (node, spec) in modules(n, ctx.source, true, true) {
        // Only a relative or root-relative specifier names a file of the project.
        if !(spec.starts_with("./") || spec.starts_with("../") || spec.starts_with('/')) {
            continue;
        }
        let Some(target_path) = resolve_file(ctx, &file, &spec) else { continue };
        for z in zones.values() {
            let Value::Object(z) = z else {
                return option_problem(n, &["import_statement", "export_statement", "call_expression"], out, "each zone of `zones` is an object with `target` and `from`");
            };
            let (targets, froms) = (list(z.get("target")), list(z.get("from")));
            if targets.is_empty() || froms.is_empty() {
                return option_problem(n, &["import_statement", "export_statement", "call_expression"], out, "each zone of `zones` needs a `target` and a `from`");
            }
            if !targets.iter().any(|t| path_matches(t, base, &file)) {
                continue;
            }
            let Some(from) = froms.iter().find(|f| path_matches(f, base, &target_path)) else { continue };
            // `except` is relative to a directory `from`, and a glob on its own for a glob `from`.
            let excepted = list(z.get("except")).iter().any(|e| if is_glob(from) { path_matches(e, base, &target_path) } else { path_matches(e, &join(base, from), &target_path) });
            if excepted {
                continue;
            }
            let extra = z.get("message").and_then(Value::as_str).map(|m| format!(" {m}")).unwrap_or_default();
            out.push(at(node, format!("Unexpected path \"{spec}\" imported in restricted zone.{extra}")));
        }
    }
});

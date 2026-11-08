//! W-139 strand W-139-js, the rows that read the file's own path, whether
//! another file exists, or the nearest `package.json`: eslint-plugin-unicorn
//! v79.0.0, eslint-plugin-import 2.32.0, eslint-plugin-n 18.5.0 and
//! eslint-plugin-sonarjs 4.2.2 (behaviour only). They read the project
//! through `NativeContext::file` (SR-ARCHITECTURE decision 15b), which
//! records every question, so a cached result is reused only while each
//! answer holds; none needs another module's exports, which is the project
//! graph's.

use super::{JS, finding, span};
use crate::js_ast::arguments;
use crate::util::{children, named_children, text, unquote};
use crate::{NativeContext, NativeFinding, NativeRule, resolve_path};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::BTreeMap;
use tree_sitter::Node;

macro_rules! frule {
    ($name:ident, $id:literal, $kinds:expr, $sem:expr, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
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
                $sem
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}

const NODE_BUILTINS: &[&str] = &[
    "assert", "async_hooks", "buffer", "child_process", "cluster", "console", "constants", "crypto", "dgram", "diagnostics_channel", "dns", "domain", "events", "fs", "http", "http2", "https", "inspector", "module", "net", "os",
    "path", "perf_hooks", "process", "punycode", "querystring", "readline", "repl", "stream", "string_decoder", "sys", "timers", "tls", "trace_events", "tty", "url", "util", "v8", "vm", "wasi", "worker_threads", "zlib",
];

fn is_builtin(spec: &str) -> bool {
    let s = spec.strip_prefix("node:").unwrap_or(spec);
    NODE_BUILTINS.contains(&s.split('/').next().unwrap_or(s)) || spec.starts_with("node:")
}

fn opt_str<'a>(o: &'a BTreeMap<String, Value>, k: &str) -> Option<&'a str> {
    o.get(k).and_then(Value::as_str)
}

fn opt_bool(o: &BTreeMap<String, Value>, k: &str, default: bool) -> bool {
    o.get(k).and_then(Value::as_bool).unwrap_or(default)
}

fn strings(o: &BTreeMap<String, Value>, k: &str) -> Vec<String> {
    o.get(k).and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_str).map(str::to_owned).collect()).unwrap_or_default()
}

fn file_start(out: &mut Vec<NativeFinding>, message: String) {
    out.push(span(0, 0, message, vec![]));
}

/// Each module specifier of the file: the string node, its text, and the
/// form (`import`, `export`, `dynamic`, `require`).
fn specifiers<'t>(root: Node<'t>, src: &str) -> Vec<(Node<'t>, String, &'static str)> {
    let mut out = Vec::new();
    let mut all = Vec::new();
    crate::util::descendants(root, &|_| false, &mut all);
    for x in all {
        let (s, form) = match x.kind() {
            "import_statement" => (x.child_by_field_name("source"), "import"),
            "export_statement" => (x.child_by_field_name("source"), "export"),
            "call_expression" => {
                let f = x.child_by_field_name("function");
                let args = arguments(x);
                match (f.map(|f| (f.kind(), text(f, src))), &args[..]) {
                    (Some(("import", _)), [a, ..]) => (Some(*a), "dynamic"),
                    (Some(("identifier", "require")), [a]) => (Some(*a), "require"),
                    _ => (None, ""),
                }
            }
            _ => (None, ""),
        };
        if let Some(s) = s.filter(|s| s.kind() == "string") {
            out.push((s, unquote(text(s, src)).to_owned(), form));
        }
    }
    out
}

// ---- a small resolver over the project's files ------------------------------

const PROBE: &[&str] = &[".mjs", ".js", ".json", ".node", ".cjs", ".jsx", ".ts", ".tsx", ".mts", ".cts"];

/// The file a relative specifier names: as written, with an extension
/// added, or a folder's `index`.
fn resolve(ctx: &NativeContext<'_>, spec: &str, exts: &[&str]) -> Option<String> {
    let path = resolve_path(ctx.file.path(), spec)?;
    if !spec.ends_with('/') && ctx.file.exists(&path) && !ctx.file.is_dir(&path) {
        return Some(path);
    }
    if !spec.ends_with('/') {
        for e in exts {
            let p = format!("{path}{e}");
            if ctx.file.exists(&p) && !ctx.file.is_dir(&p) {
                return Some(p);
            }
        }
    }
    if ctx.file.is_dir(&path) {
        for e in exts {
            let p = if path.is_empty() { format!("index{e}") } else { format!("{path}/index{e}") };
            if ctx.file.exists(&p) {
                return Some(p);
            }
        }
    }
    None
}

fn relative(spec: &str) -> bool {
    spec == "." || spec == ".." || spec.starts_with("./") || spec.starts_with("../")
}

fn dir_of(p: &str) -> &str {
    p.rfind('/').map_or("", |i| &p[..i])
}

/// The nearest `package.json` above `path` that has a `name` (or any, when
/// `named` is false): its folder and its parsed value.
fn package_json(ctx: &NativeContext<'_>, path: &str, named: bool) -> Option<(String, Value)> {
    let mut dir = dir_of(path).to_owned();
    loop {
        let p = if dir.is_empty() { "package.json".to_owned() } else { format!("{dir}/package.json") };
        if let Some(t) = ctx.file.read(&p)
            && let Ok(v) = serde_json::from_str::<Value>(&t)
            && (!named || v.get("name").and_then(Value::as_str).is_some())
        {
            return Some((dir, v));
        }
        if dir.is_empty() {
            return None;
        }
        dir = dir_of(&dir).to_owned();
    }
}

// ---- unicorn: filename-case ---------------------------------------------------

/// change-case's word split: at non-alphanumerics, between a lower case
/// letter or digit and an upper case letter, and before the last capital of
/// a run followed by a lower case letter.
fn words(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let cs: Vec<char> = s.chars().collect();
    let mut cur = String::new();
    for (i, &c) in cs.iter().enumerate() {
        if !c.is_ascii_alphanumeric() {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        if let Some(&p) = i.checked_sub(1).map(|j| &cs[j]) {
            let next_lower = cs.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            if c.is_ascii_uppercase() && (p.is_ascii_lowercase() || p.is_ascii_digit() || (p.is_ascii_uppercase() && next_lower)) && !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        }
        cur.push(c);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn cap(w: &str) -> String {
    let mut c = w.chars();
    c.next().map_or(String::new(), |f| f.to_ascii_uppercase().to_string() + &c.as_str().to_ascii_lowercase())
}

fn convert(case: &str, s: &str) -> String {
    let ws = words(s);
    match case {
        "kebabCase" => ws.iter().map(|w| w.to_ascii_lowercase()).collect::<Vec<_>>().join("-"),
        "snakeCase" => ws.iter().map(|w| w.to_ascii_lowercase()).collect::<Vec<_>>().join("_"),
        "pascalCase" => ws.iter().map(|w| cap(w)).collect(),
        _ => ws.iter().enumerate().map(|(i, w)| if i == 0 { w.to_ascii_lowercase() } else { cap(w) }).collect(),
    }
}

/// The name's parts: its leading underscores, then runs of word characters
/// (checked) and of other characters (kept as they are).
fn valid_name(name: &str, cases: &[String]) -> Option<Vec<String>> {
    let rest = name.trim_start_matches('_');
    let lead = &name[..name.len() - rest.len()];
    let mut runs: Vec<(String, bool)> = Vec::new();
    for c in rest.chars() {
        let word = c.is_ascii_alphanumeric() || c == '_' || c == '-';
        match runs.last_mut() {
            Some((r, w)) if *w == word => r.push(c),
            _ => runs.push((c.to_string(), word)),
        }
    }
    let ok = cases.iter().any(|case| runs.iter().all(|(r, w)| !w || convert(case, r) == *r));
    if ok {
        return None;
    }
    let mut names: Vec<String> = cases.iter().map(|case| format!("{lead}{}", runs.iter().map(|(r, w)| if *w { convert(case, r) } else { r.clone() }).collect::<String>())).collect();
    names.dedup();
    Some(names)
}

fn case_label(c: &str) -> &'static str {
    match c {
        "camelCase" => "camel case",
        "snakeCase" => "snake case",
        "pascalCase" => "pascal case",
        _ => "kebab case",
    }
}

frule!(FilenameCase, "js/filename-case", &["program"], false, |_n, ctx, out| {
    let o = ctx.options;
    let mut cases: Vec<String> = match o.get("cases").and_then(Value::as_object) {
        Some(m) if !m.is_empty() => m.iter().filter(|(_, v)| v.as_bool() == Some(true)).map(|(k, _)| k.clone()).collect(),
        _ => vec![opt_str(o, "case").unwrap_or("kebabCase").to_owned()],
    };
    if cases.is_empty() {
        cases.push("kebabCase".to_owned());
    }
    let path = ctx.file.path();
    let segments: Vec<&str> = path.split('/').collect();
    let ignore: Vec<regex::Regex> = strings(o, "ignore").iter().filter_map(|p| regex::Regex::new(p).ok()).collect();
    if segments.iter().any(|s| ignore.iter().any(|r| r.is_match(s))) {
        return;
    }
    let labels = cases.iter().map(|c| case_label(c)).collect::<Vec<_>>().join(" or ");
    let quoted = |v: Vec<String>| v.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(" or ");
    if opt_bool(o, "checkDirectories", true) {
        let roots = strings(o, "directoryRoots");
        let dirs = &segments[..segments.len() - 1];
        let mut start = 0;
        let mut acc = String::new();
        for (i, d) in dirs.iter().enumerate() {
            acc = if acc.is_empty() { (*d).to_owned() } else { format!("{acc}/{d}") };
            if roots.iter().any(|r| *r == acc || regex::Regex::new(r).is_ok_and(|re| re.is_match(&acc))) {
                start = i + 1;
            }
        }
        for d in &dirs[start..] {
            if !d.starts_with('$')
                && let Some(names) = valid_name(d, &cases)
            {
                file_start(out, format!("Folder name `{d}` is not in {labels}: rename it to {}", quoted(names)));
                return;
            }
        }
    }
    let base = segments[segments.len() - 1];
    if matches!(base, "index.js" | "index.mjs" | "index.cjs" | "index.ts" | "index.tsx" | "index.vue") {
        return;
    }
    let (stem, ext) = base.rfind('.').map_or((base, ""), |i| (&base[..i], &base[i..]));
    let (name, extra) = if opt_bool(o, "multipleFileExtensions", true) { stem.find('.').map_or((stem, ""), |i| (&stem[..i], &stem[i..])) } else { (stem, "") };
    if name.starts_with('$') {
        return;
    }
    match valid_name(name, &cases) {
        None if ext != ext.to_ascii_lowercase() => file_start(out, format!("The file extension `{ext}` is not lower case: rename the file to `{name}{extra}{}`", ext.to_ascii_lowercase())),
        None => {}
        Some(names) => file_start(out, format!("The file name is not in {labels}: rename it to {}", quoted(names.into_iter().map(|x| format!("{x}{extra}{}", ext.to_ascii_lowercase())).collect()))),
    }
});

// ---- sonarjs: file-name-differ-from-class (S3317) ---------------------------------

frule!(FileNameDifferFromClass, "js/file-name-differ-from-class", &["program"], true, |n, ctx, out| {
    let src = ctx.source;
    let Some(m) = ctx.semantic else { return };
    let exports: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() == "export_statement").collect();
    let is_default = |e: &Node<'_>| children(*e).iter().any(|c| c.kind() == "default");
    if exports.iter().any(|e| !is_default(e)) {
        return;
    }
    let Some(e) = exports.iter().find(|e| is_default(e)) else { return };
    let name = if let Some(d) = e.child_by_field_name("declaration") {
        d.child_by_field_name("name").map(|x| text(x, src).to_owned())
    } else if let Some(v) = e.child_by_field_name("value").filter(|v| v.kind() == "identifier") {
        m.symbol_at(v.start_byte()).filter(|s| {
            let s = m.symbol(*s);
            s.decls.len() == 1 && matches!(s.kind, codelaw_semantic::SymbolKind::Class | codelaw_semantic::SymbolKind::Function | codelaw_semantic::SymbolKind::Const)
        }).map(|_| text(v, src).to_owned())
    } else {
        None
    };
    let Some(name) = name else { return };
    let base = ctx.file.path().rsplit('/').next().unwrap_or("");
    let stem = base.rfind('.').map_or(base, |i| &base[..i]);
    if stem == "index" {
        return;
    }
    let norm = |s: &str| s.chars().filter(|c| !matches!(c, '_' | '-' | '.')).collect::<String>().to_lowercase();
    let short = stem.rfind('.').map_or(stem, |i| &stem[..i]);
    if norm(stem) != norm(&name) && norm(short) != norm(&name) {
        file_start(out, format!("This file exports `{name}` by default: rename the file to match it"));
    }
});

// ---- eslint-plugin-import: unambiguous; unicorn: no-exports-in-scripts --------------

frule!(ImportUnambiguous, "js/import-unambiguous", &["program"], false, |n, ctx, out| {
    if ctx.file.path().ends_with(".cjs") || ctx.file.path().ends_with(".cts") {
        return;
    }
    if named_children(n).iter().any(|c| matches!(c.kind(), "import_statement" | "export_statement")) {
        return;
    }
    if ctx.source.trim().is_empty() {
        return;
    }
    file_start(out, "This file has no `import` or `export`, so it parses as a script as well as a module: add an `export` or name it `.cjs`".to_owned());
});

frule!(NoExportsInScripts, "js/no-exports-in-scripts", &["export_statement"], false, |n, ctx, out| {
    if !ctx.source.starts_with("#!") {
        return;
    }
    out.push(finding(n, "A file run as a script (it starts with `#!`) has nothing to export: move the exports to a module the script imports", vec![]));
});

// ---- unicorn: import-style --------------------------------------------------------------

/// The styles a module allows: `(default, named, namespace, unassigned)`.
fn allowed(module: &str, o: &BTreeMap<String, Value>) -> Option<[bool; 4]> {
    let m = module.strip_prefix("node:").unwrap_or(module);
    let builtin = match m {
        "path" | "chalk" => Some([true, false, false, false]),
        "util" => Some([false, true, false, false]),
        _ => None,
    };
    let extend = opt_bool(o, "extendDefaultStyles", true);
    let user = o.get("styles").and_then(Value::as_object).and_then(|s| s.get(m).or_else(|| s.get(module)));
    match user {
        Some(Value::Bool(false)) => None,
        Some(Value::Object(st)) => {
            let get = |k: &str| st.get(k).and_then(Value::as_bool);
            let base = if extend { builtin.unwrap_or_default() } else { [false; 4] };
            Some([get("default").unwrap_or(base[0]), get("named").unwrap_or(base[1]), get("namespace").unwrap_or(base[2]), get("unassigned").unwrap_or(base[3])])
        }
        _ if extend => builtin,
        _ => None,
    }
}

frule!(ImportStyle, "js/import-style", &["import_statement", "call_expression", "export_statement"], false, |n, ctx, out| {
    let src = ctx.source;
    let o = ctx.options;
    let (module, used): (String, Vec<usize>) = match n.kind() {
        "import_statement" => {
            if !opt_bool(o, "checkImport", true) || children(n).iter().any(|c| c.kind() == "type") {
                return;
            }
            let Some(s) = n.child_by_field_name("source") else { return };
            let mut used = Vec::new();
            match named_children(n).into_iter().find(|c| c.kind() == "import_clause") {
                None => used.push(3),
                Some(clause) => {
                    for c in named_children(clause) {
                        match c.kind() {
                            "identifier" => used.push(0),
                            "namespace_import" => used.push(2),
                            "named_imports" => {
                                for sp in named_children(c).into_iter().filter(|x| x.kind() == "import_specifier") {
                                    if children(sp).iter().any(|x| x.kind() == "type") {
                                        continue;
                                    }
                                    used.push(if sp.child_by_field_name("name").is_some_and(|x| text(x, src) == "default") { 0 } else { 1 });
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            (unquote(text(s, src)).to_owned(), used)
        }
        "export_statement" => {
            if !opt_bool(o, "checkExportFrom", false) {
                return;
            }
            let Some(s) = n.child_by_field_name("source") else { return };
            let ns = children(n).iter().any(|c| c.kind() == "*" || c.kind() == "namespace_export");
            (unquote(text(s, src)).to_owned(), vec![if ns { 2 } else { 1 }])
        }
        _ => {
            let f = n.child_by_field_name("function");
            let dynamic = f.is_some_and(|f| f.kind() == "import");
            let req = f.is_some_and(|f| f.kind() == "identifier" && text(f, src) == "require");
            if (dynamic && !opt_bool(o, "checkDynamicImport", true)) || (req && !opt_bool(o, "checkRequire", true)) || (!dynamic && !req) {
                return;
            }
            let args = arguments(n);
            let Some(s) = args.first().filter(|a| a.kind() == "string" && (dynamic || args.len() == 1)) else { return };
            let mut x = n;
            if x.parent().is_some_and(|p| p.kind() == "await_expression") {
                x = x.parent().unwrap_or(x);
            }
            let style = match x.parent() {
                Some(p) if p.kind() == "expression_statement" => 3,
                Some(p) if p.kind() == "variable_declarator" => match p.child_by_field_name("name").map(|nm| nm.kind()) {
                    Some("identifier" | "array_pattern") => 2,
                    Some("object_pattern") => {
                        let pat = p.child_by_field_name("name").unwrap_or(p);
                        if named_children(pat).is_empty() {
                            3
                        } else if named_children(pat).iter().any(|c| c.child_by_field_name("key").is_some_and(|k| text(k, src) == "default")) {
                            0
                        } else {
                            1
                        }
                    }
                    _ => return,
                },
                _ if dynamic => return,
                _ => return,
            };
            (unquote(text(*s, src)).to_owned(), vec![style])
        }
    };
    let Some(allow) = allowed(&module, o) else { return };
    let names = ["default", "named", "namespace", "unassigned"];
    let req = n.kind() == "call_expression" && n.child_by_field_name("function").is_some_and(|f| text(f, src) == "require");
    for u in used {
        let ok = allow[u] || (req && u == 2 && allow[0]);
        if !ok {
            let want: Vec<&str> = names.iter().zip(allow).filter(|(_, a)| *a).map(|(k, _)| *k).collect();
            let msg = if want.is_empty() { format!("`{module}` is not to be imported") } else { format!("Import `{module}` with a {} import, not a {} one", want.join(" or "), names[u]) };
            out.push(finding(n, msg, vec![]));
            return;
        }
    }
});

// ---- sonarjs: no-internal-api-use (S6627) -------------------------------------------

frule!(NoInternalApiUse, "js/no-internal-api-use", &["program"], false, |n, ctx, out| {
    for (s, spec, form) in specifiers(n, ctx.source) {
        if matches!(form, "import" | "require") && spec.contains("node_modules") {
            out.push(finding(s, "Don't import from a dependency's own files under `node_modules`: import the package by its name", vec![]));
        }
    }
});

// ---- eslint-plugin-import: no-self-import ----------------------------------------------

frule!(ImportNoSelfImport, "js/import-no-self-import", &["program"], false, |n, ctx, out| {
    let own = ctx.file.path();
    for (s, spec, _) in specifiers(n, ctx.source) {
        if relative(&spec) && resolve(ctx, &spec, PROBE).as_deref() == Some(own) {
            out.push(finding(s, "This module imports itself: remove the import", vec![]));
        }
    }
});

// ---- eslint-plugin-import: extensions ----------------------------------------------------

const RESOLVER: &[&str] = &[".mjs", ".js", ".json", ".node"];

frule!(ImportExtensions, "js/import-extensions", &["program"], false, |n, ctx, out| {
    let mode = opt_str(ctx.options, "mode").unwrap_or("never");
    for (s, spec, _) in specifiers(n, ctx.source) {
        if !relative(&spec) && !spec.starts_with('/') {
            continue;
        }
        let spec = spec.split('?').next().unwrap_or(&spec).to_owned();
        let Some(file) = resolve(ctx, &spec, RESOLVER) else { continue };
        let last = spec.rsplit('/').next().unwrap_or("");
        let ext = file.rfind('.').map_or("", |i| &file[i..]);
        let written = !ext.is_empty() && last.ends_with(ext);
        let q = &ctx.source[s.start_byte()..s.start_byte() + 1];
        if mode == "never" && written && ext != ".json" {
            let bare = &spec[..spec.len() - ext.len()];
            if resolve(ctx, bare, RESOLVER).as_deref() == Some(file.as_str()) {
                out.push(finding(s, format!("Unexpected file extension `{ext}`: the path resolves without it"), crate::as_suggestion(vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{bare}{q}"))])));
            }
        } else if mode == "always" && !written && !ext.is_empty() && !spec.ends_with('/') && !file.ends_with(&format!("/index{ext}")) {
            out.push(finding(s, format!("Missing file extension `{ext}` in the import path"), crate::as_suggestion(vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{spec}{ext}{q}"))])));
        }
    }
});

// ---- eslint-plugin-import: no-useless-path-segments ------------------------------------------

fn normalize(spec: &str) -> String {
    let lead_dots = spec.starts_with('.');
    let mut parts: Vec<&str> = Vec::new();
    for seg in spec.split('/') {
        match seg {
            "" | "." => {}
            ".." if parts.last().is_some_and(|p| *p != "..") => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    let joined = parts.join("/");
    if joined.is_empty() {
        return ".".to_owned();
    }
    if lead_dots && !joined.starts_with("..") { format!("./{joined}") } else { joined }
}

/// The relative path from folder `from` to file `to`, both project-relative.
fn relative_path(from: &str, to: &str) -> String {
    let a: Vec<&str> = from.split('/').filter(|s| !s.is_empty()).collect();
    let b: Vec<&str> = to.split('/').filter(|s| !s.is_empty()).collect();
    let common = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let ups = a.len() - common;
    let rest = b[common..].join("/");
    if ups == 0 { format!("./{rest}") } else { format!("{}{rest}", "../".repeat(ups)) }
}

frule!(ImportNoUselessPathSegments, "js/import-no-useless-path-segments", &["program"], false, |n, ctx, out| {
    let commonjs = opt_bool(ctx.options, "commonjs", false);
    let no_index = opt_bool(ctx.options, "noUselessIndex", false);
    let here = dir_of(ctx.file.path()).to_owned();
    for (s, spec, form) in specifiers(n, ctx.source) {
        if !spec.starts_with('.') || (form == "require" && !commonjs) {
            continue;
        }
        let q = &ctx.source[s.start_byte()..s.start_byte() + 1];
        let file = resolve(ctx, &spec, PROBE);
        let mut proposal = None;
        let norm = normalize(&spec);
        if norm != spec && resolve(ctx, &norm, PROBE) == file {
            proposal = Some(norm.clone());
        }
        let cur = proposal.clone().unwrap_or(spec.clone());
        if proposal.is_none() && no_index {
            let last = cur.rsplit('/').next().unwrap_or("");
            if (last == "index" || last.starts_with("index.")) && cur.contains('/') {
                let parent = &cur[..cur.rfind('/').unwrap_or(0)];
                if parent != "." && parent != ".." {
                    proposal = Some(parent.to_owned());
                }
            }
        }
        if proposal.is_none()
            && cur.starts_with("../")
            && let Some(f) = &file
        {
            let rel = relative_path(&here, f);
            let ups = |p: &str| p.split('/').take_while(|x| *x == "..").count();
            if ups(&rel) < ups(&cur) {
                let tail_ext = |p: &str| p.rsplit('/').next().is_some_and(|l| l.contains('.'));
                let mut r = rel.clone();
                if !tail_ext(&cur) && let Some(i) = r.rfind('.') && r[i..].find('/').is_none() && i > r.rfind('/').map_or(0, |j| j) {
                    r.truncate(i);
                }
                if resolve(ctx, &r, PROBE) == file {
                    proposal = Some(r);
                }
            }
        }
        if let Some(p) = proposal {
            out.push(finding(s, format!("Useless path segments in `{spec}`: write `{p}`"), vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{p}{q}"))]));
        }
    }
});

// ---- eslint-plugin-import: no-relative-packages ------------------------------------------------

frule!(ImportNoRelativePackages, "js/import-no-relative-packages", &["program"], false, |n, ctx, out| {
    let commonjs = opt_bool(ctx.options, "commonjs", false);
    let Some((own_dir, own)) = package_json(ctx, ctx.file.path(), true) else { return };
    let own_name = own.get("name").and_then(Value::as_str).unwrap_or("").to_owned();
    for (s, spec, form) in specifiers(n, ctx.source) {
        if !relative(&spec) || (form == "require" && !commonjs) {
            continue;
        }
        let Some(file) = resolve(ctx, &spec, PROBE) else { continue };
        let Some((dir, pkg)) = package_json(ctx, &file, true) else { continue };
        let Some(name) = pkg.get("name").and_then(Value::as_str) else { continue };
        if name == own_name || dir == own_dir {
            continue;
        }
        let q = &ctx.source[s.start_byte()..s.start_byte() + 1];
        let target = resolve_path(ctx.file.path(), &spec).unwrap_or_default();
        let inner = target.strip_prefix(&dir).unwrap_or(&target).trim_start_matches('/');
        let new = if inner.is_empty() { name.to_owned() } else { format!("{name}/{inner}") };
        out.push(finding(s, format!("`{spec}` reaches into the package `{name}` by a relative path: import it as `{new}`"), vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{new}{q}"))]));
    }
});

// ---- eslint-plugin-n: file-extension-in-import -----------------------------------------------

const TRY_EXTS: &[&str] = &[".js", ".json", ".node", ".mjs", ".cjs"];

frule!(NodeFileExtensionInImport, "node/file-extension-in-import", &["program"], false, |n, ctx, out| {
    let mode = opt_str(ctx.options, "mode").unwrap_or("always");
    let ts = matches!(ctx.language, Language::Typescript | Language::Tsx);
    for (s, spec, form) in specifiers(n, ctx.source) {
        if form == "require" || is_builtin(&spec) || !(relative(&spec) || spec.starts_with('/')) {
            continue;
        }
        let Some(path) = resolve_path(ctx.file.path(), &spec) else { continue };
        let q = &ctx.source[s.start_byte()..s.start_byte() + 1];
        let js_ext = |e: &str| -> String {
            match e {
                ".ts" | ".tsx" if ts => ".js".to_owned(),
                ".mts" if ts => ".mjs".to_owned(),
                ".cts" if ts => ".cjs".to_owned(),
                other => other.to_owned(),
            }
        };
        // The file the specifier means, and the extension it should be written with.
        let (expected, suffix) = if ctx.file.is_dir(&path) {
            let found = TRY_EXTS.iter().chain([".ts", ".tsx", ".mts", ".cts"].iter()).find(|e| ctx.file.exists(&format!("{path}/index{e}")));
            match found {
                Some(e) => (js_ext(e), format!("/index{}", js_ext(e))),
                None => continue,
            }
        } else if ctx.file.exists(&path) {
            let last = path.rsplit('/').next().unwrap_or("");
            let e = last.rfind('.').map_or(String::new(), |i| last[i..].to_owned());
            (e.clone(), e)
        } else {
            let base = path.rsplit('/').next().unwrap_or("").to_owned();
            let siblings = ctx.file.list(dir_of(&path)).unwrap_or_default();
            let mine: Vec<&String> = siblings.iter().filter(|f| f.strip_prefix(&base).is_some_and(|r| r.starts_with('.') && !r[1..].contains('.'))).collect();
            let pick = TRY_EXTS.iter().chain([".ts", ".tsx", ".mts", ".cts"].iter()).find_map(|e| mine.iter().find(|f| f.ends_with(e)).map(|_| *e)).or_else(|| mine.first().map(|f| &f[base.len()..]));
            match pick {
                Some(e) => (js_ext(e), js_ext(e)),
                None => continue,
            }
        };
        let last = spec.rsplit('/').next().unwrap_or("");
        let written = last.rfind('.').map_or("", |i| &last[i..]);
        if mode == "always" && written != expected && !(ctx.file.exists(&path) && !ctx.file.is_dir(&path)) {
            out.push(finding(s, format!("Write the file extension: `{spec}{suffix}`"), vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{spec}{suffix}{q}"))]));
        } else if mode == "never" && !written.is_empty() && written == expected {
            let bare = &spec[..spec.len() - written.len()];
            let base = bare.rsplit('/').next().unwrap_or("");
            let count = ctx.file.list(dir_of(&path)).unwrap_or_default().iter().filter(|f| f.strip_prefix(base).is_some_and(|r| r.starts_with('.'))).count();
            let edits = if count == 1 { vec![TextEdit::new(s.start_byte(), s.end_byte(), format!("{q}{bare}{q}"))] } else { vec![] };
            out.push(finding(s, format!("Leave out the file extension `{written}`"), edits));
        }
    }
});

// ---- eslint-plugin-n: no-unpublished-bin ----------------------------------------------------

fn glob_covers(pattern: &str, path: &str) -> bool {
    let p = pattern.trim_start_matches("./").trim_end_matches('/');
    if p.is_empty() {
        return false;
    }
    let lower = path.to_lowercase();
    let p = p.to_lowercase();
    if lower == p || lower.starts_with(&format!("{p}/")) {
        return true;
    }
    globset::Glob::new(&p).is_ok_and(|g| g.compile_matcher().is_match(&lower)) || globset::Glob::new(&format!("{p}/**")).is_ok_and(|g| g.compile_matcher().is_match(&lower))
}

frule!(NodeNoUnpublishedBin, "node/no-unpublished-bin", &["program"], false, |_n, ctx, out| {
    let path = ctx.file.path();
    let Some((dir, pkg)) = package_json(ctx, path, false) else { return };
    let rel = path.strip_prefix(&dir).unwrap_or(path).trim_start_matches('/').to_owned();
    let bins: Vec<String> = match pkg.get("bin") {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Object(m)) => m.values().filter_map(Value::as_str).map(str::to_owned).collect(),
        _ => return,
    };
    let clean = |b: &str| normalize(b).trim_start_matches("./").to_owned();
    let is_bin = bins.iter().map(|b| clean(b)).any(|b| b == rel || format!("{b}.js") == rel || format!("{b}/index.js") == rel);
    if !is_bin {
        return;
    }
    let Some(files) = pkg.get("files").and_then(Value::as_array) else { return };
    let pats: Vec<&str> = files.iter().filter_map(Value::as_str).collect();
    let mut included = false;
    for p in &pats {
        if let Some(neg) = p.strip_prefix('!') {
            if glob_covers(neg, &rel) {
                included = false;
            }
        } else if glob_covers(p, &rel) {
            included = true;
        }
    }
    if !included {
        file_start(out, format!("npm leaves `{rel}` out of the published package (`files` in package.json), but it is a `bin`: add it to `files`"));
    }
});

// ---- eslint-plugin-n: prefer-process-get-builtin-module ------------------------------------------

/// Whether every version the `engines.node` range allows is at least
/// 20.16 (below 21) or 22.3: where `process.getBuiltinModule()` exists.
fn has_get_builtin_module(range: &str) -> bool {
    let ok = |v: &semver::Version| (v.major == 20 && v.minor >= 16) || v.major > 22 || (v.major == 22 && v.minor >= 3);
    range.split("||").all(|part| {
        let part = part.trim();
        let Ok(req) = semver::VersionReq::parse(&part.replace(".x", "")) else { return false };
        let mut lower: Option<semver::Version> = None;
        let mut upper_21 = false;
        for c in &req.comparators {
            let v = semver::Version::new(c.major, c.minor.unwrap_or(0), c.patch.unwrap_or(0));
            match c.op {
                semver::Op::Greater | semver::Op::GreaterEq | semver::Op::Exact | semver::Op::Tilde | semver::Op::Caret | semver::Op::Wildcard => {
                    if matches!(c.op, semver::Op::Caret | semver::Op::Tilde) && c.major == 20 {
                        upper_21 = true;
                    }
                    lower = Some(lower.map_or(v.clone(), |l| l.max(v)));
                }
                semver::Op::Less | semver::Op::LessEq => {
                    if c.major <= 21 {
                        upper_21 = true;
                    }
                }
                _ => {}
            }
        }
        lower.is_some_and(|l| ok(&l) && (l.major != 20 || upper_21))
    })
}

frule!(NodePreferProcessGetBuiltinModule, "node/prefer-process-get-builtin-module", &["call_expression"], true, |n, ctx, out| {
    let src = ctx.source;
    let Some(m) = ctx.semantic else { return };
    let Some(f) = n.child_by_field_name("function") else { return };
    let args = arguments(n);
    let (is_req, ok_shape) = match f.kind() {
        "identifier" if text(f, src) == "require" => (true, args.len() == 1 && m.symbol_at(f.start_byte()).is_none()),
        "import" => (false, args.len() == 1 && n.parent().is_some_and(|p| p.kind() == "await_expression")),
        _ => return,
    };
    let Some(spec) = args.first().filter(|a| a.kind() == "string").map(|a| unquote(text(*a, src)).to_owned()) else { return };
    if !ok_shape || !is_builtin(&spec) || m.symbols.iter().any(|s| s.name == "process") {
        return;
    }
    let range = ctx.options.get("version").and_then(Value::as_str).filter(|v| !v.trim().is_empty()).map(str::to_owned).or_else(|| package_json(ctx, ctx.file.path(), false).and_then(|(_, p)| p.get("engines").and_then(|e| e.get("node")).and_then(Value::as_str).map(str::to_owned)));
    if !range.is_some_and(|r| has_get_builtin_module(&r)) {
        return;
    }
    let how = if is_req { "`require()`" } else { "`await import()`" };
    let at = if is_req { n } else { n.parent().unwrap_or(n) };
    // `require(x)` and `process.getBuiltinModule(x)` give the same object;
    // `await import(x)` gives a namespace, so it is not rewritten.
    let edits = if is_req { vec![TextEdit::new(f.start_byte(), f.end_byte(), "process.getBuiltinModule")] } else { vec![] };
    out.push(finding(at, format!("Load the built-in `{spec}` with `process.getBuiltinModule(\"{spec}\")` instead of {how}"), edits));
});

// ---- eslint-plugin-import: export (this file's duplicates) ---------------------------------------

frule!(ImportExport, "js/import-export", &["program"], false, |n, ctx, out| {
    let src = ctx.source;
    let mut names: BTreeMap<(String, bool), Vec<Node<'_>>> = BTreeMap::new();
    for e in named_children(n).into_iter().filter(|c| c.kind() == "export_statement") {
        if children(e).iter().any(|c| c.kind() == "default") {
            names.entry(("default".into(), false)).or_default().push(e);
            continue;
        }
        if let Some(clause) = named_children(e).into_iter().find(|c| c.kind() == "export_clause") {
            let ty = children(e).iter().any(|c| c.kind() == "type");
            for sp in named_children(clause).into_iter().filter(|x| x.kind() == "export_specifier") {
                let nm = sp.child_by_field_name("alias").or(sp.child_by_field_name("name"));
                let as_default = text(sp, src).split_whitespace().rev().take(2).collect::<Vec<_>>() == ["default", "as"];
                if let Some(nm) = nm {
                    let key = if as_default { "default".to_owned() } else if nm.kind() == "string" { unquote(text(nm, src)).to_owned() } else { text(nm, src).to_owned() };
                    names.entry((key, ty)).or_default().push(if as_default { sp } else { nm });
                }
            }
        }
        if let Some(d) = e.child_by_field_name("declaration") {
            let ty = matches!(d.kind(), "type_alias_declaration" | "interface_declaration");
            if matches!(d.kind(), "function_signature" | "module" | "internal_module" | "ambient_declaration") {
                continue;
            }
            if let Some(nm) = d.child_by_field_name("name") {
                names.entry((text(nm, src).to_owned(), ty)).or_default().push(nm);
            }
            for dec in named_children(d).into_iter().filter(|c| c.kind() == "variable_declarator") {
                if let Some(p) = dec.child_by_field_name("name") {
                    let mut ids = Vec::new();
                    crate::util::descendants(p, &|_| false, &mut ids);
                    ids.push(p);
                    for id in ids.into_iter().filter(|x| matches!(x.kind(), "identifier" | "shorthand_property_identifier_pattern")) {
                        if id.parent().is_some_and(|q| q.kind() == "pair_pattern" && q.child_by_field_name("key").is_some_and(|k| k.id() == id.id())) {
                            continue;
                        }
                        names.entry((text(id, src).to_owned(), false)).or_default().push(id);
                    }
                }
            }
        }
    }
    // TypeScript declaration merging (a namespace with a function, class or enum) is allowed.
    for ((name, _), nodes) in names {
        if nodes.len() < 2 {
            continue;
        }
        let merging = nodes.iter().any(|x| x.parent().is_some_and(|p| matches!(p.kind(), "module" | "internal_module")));
        if merging {
            continue;
        }
        for x in nodes {
            let msg = if name == "default" { "This file has more than one default export: keep one".to_owned() } else { format!("`{name}` is exported more than once: keep one export of it") };
            out.push(finding(x, msg, vec![]));
        }
    }
});

// ---- unicorn: no-missing-local-resource (JavaScript) -----------------------------------------------

frule!(NoMissingLocalResource, "js/no-missing-local-resource", &["new_expression"], false, |n, ctx, out| {
    let src = ctx.source;
    if !n.child_by_field_name("constructor").is_some_and(|c| text(c, src) == "URL") {
        return;
    }
    let args = arguments(n);
    let [first, base] = args[..] else { return };
    if first.kind() != "string" || text(base, src).replace(' ', "") != "import.meta.url" {
        return;
    }
    let raw = unquote(text(first, src));
    if raw.is_empty() || raw.starts_with('/') || raw.contains(['\t', '\r', '\n', '\\']) || raw.contains("://") || raw.starts_with("data:") || raw.starts_with('#') {
        return;
    }
    let value = raw.split(['?', '#']).next().unwrap_or(raw);
    let Some(path) = resolve_path(ctx.file.path(), value) else { return };
    if ctx.file.exists(&path) && (!value.ends_with('/') || ctx.file.is_dir(&path)) {
        return;
    }
    // The same path in another case.
    let mut fixed = Vec::new();
    let mut dir = dir_of(ctx.file.path()).to_owned();
    let mut ok = true;
    for seg in value.split('/') {
        match seg {
            "" | "." => fixed.push(seg.to_owned()),
            ".." => {
                fixed.push(seg.to_owned());
                dir = dir_of(&dir).to_owned();
            }
            s => {
                let found = ctx.file.list(&dir).unwrap_or_default().into_iter().find(|e| e.eq_ignore_ascii_case(s) && e.len() == s.len());
                match found {
                    Some(e) => {
                        dir = if dir.is_empty() { e.clone() } else { format!("{dir}/{e}") };
                        fixed.push(e);
                    }
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
        }
    }
    if ok {
        let new = fixed.join("/");
        let rest = &raw[value.len()..];
        let q = &src[first.start_byte()..first.start_byte() + 1];
        out.push(finding(first, format!("`{value}` exists only as `{new}`: match the file's case"), vec![TextEdit::new(first.start_byte(), first.end_byte(), format!("{q}{new}{rest}{q}"))]));
    } else {
        out.push(finding(first, format!("`{value}` names no file next to this module"), vec![]));
    }
});


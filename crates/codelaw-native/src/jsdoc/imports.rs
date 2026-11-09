//! `imports-as-dependencies` and `no-undefined-types`: the names a type uses.

use super::block::Block;
use super::tags::{Role, role};
use super::types::{self, Kind, Parent};
use super::{find, jsdoc_rule, opt_str_list};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::HashSet;

const BUILTIN_MODULES: &[&str] = &[
    "assert", "async_hooks", "buffer", "child_process", "cluster", "console", "constants", "crypto", "dgram", "diagnostics_channel", "dns", "domain", "events", "fs", "http", "http2", "https", "inspector", "module", "net", "os", "path", "perf_hooks",
    "process", "punycode", "querystring", "readline", "repl", "stream", "string_decoder", "sys", "timers", "tls", "trace_events", "tty", "url", "util", "v8", "vm", "wasi", "worker_threads", "zlib",
];

/// The nearest `package.json` at or above the file: its folder and its text.
fn nearest_package(ctx: &NativeContext<'_>) -> Option<(String, Value)> {
    let path = ctx.file.path();
    let mut dir: Vec<&str> = path.split('/').collect();
    dir.pop();
    loop {
        let folder = dir.join("/");
        let p = if folder.is_empty() { "package.json".to_owned() } else { format!("{folder}/package.json") };
        if let Some(t) = ctx.file.read(&p) {
            return serde_json::from_str(&t).ok().map(|v| (folder, v));
        }
        dir.pop()?;
    }
}

/// `@scope/name` or `name` from a module specifier.
fn package_name(spec: &str) -> &str {
    let mut parts = spec.splitn(if spec.starts_with('@') { 3 } else { 2 }, '/');
    let first = parts.next().unwrap_or(spec);
    if spec.starts_with('@') {
        let end = first.len() + 1 + parts.next().map_or(0, str::len);
        return &spec[..end.min(spec.len())];
    }
    first
}

jsdoc_rule!(ImportsAsDependencies, "js/jsdoc-imports-as-dependencies", |n, b, ctx, out| {
    let _ = n;
    if !b.tags.iter().any(|t| t.type_text(ctx.source).is_some_and(|x| x.contains("import("))) {
        return;
    }
    let Some((folder, pkg)) = nearest_package(ctx) else { return };
    let mut deps: HashSet<String> = HashSet::new();
    for k in ["dependencies", "devDependencies"] {
        if let Some(o) = pkg.get(k).and_then(Value::as_object) {
            deps.extend(o.keys().cloned());
        }
    }
    for t in &b.tags {
        let Some(text) = t.type_text(ctx.source) else { continue };
        let Ok(ty) = types::parse(text) else { continue };
        types::walk(&ty, &mut |nd, _| {
            let Kind::Import(spec) = &nd.kind else { return };
            let m = package_name(spec);
            if m.starts_with('.') || m.starts_with('/') || m.is_empty() || spec.starts_with("node:") || BUILTIN_MODULES.contains(&m) {
                return;
            }
            let installed = ctx.file.read(&if folder.is_empty() { format!("node_modules/{m}/package.json") } else { format!("{folder}/node_modules/{m}/package.json") });
            let typed = installed.as_deref().and_then(|t| serde_json::from_str::<Value>(t).ok()).map(|v| v.get("types").is_some() || v.get("typings").is_some() || v.to_string().contains("\"types\""));
            let missing = match typed {
                Some(true) => !deps.contains(m),
                Some(false) => !deps.contains(&format!("@types/{m}")),
                // Nothing installed to look in: either spelling is a dependency.
                None => !deps.contains(m) && !deps.contains(&format!("@types/{m}")),
            };
            if missing {
                out.push(find(t.at, t.tag_end, "import points to package which is not found in dependencies", vec![]));
            }
        });
    }
});

// ---- no-undefined-types ---------------------------------------------------------------

/// Written in a type with no declaration: the keywords and the globals a type may name.
const EXTRA_TYPES: &[&str] = &[
    "null", "undefined", "void", "string", "boolean", "object", "function", "symbol", "number", "bigint", "NaN", "Infinity", "any", "never", "unknown", "const", "this", "true", "false", "Array", "Object", "RegExp", "Date", "Function", "Intl", "globalThis",
    "global", "window", "self",
];

pub struct NoUndefinedTypes;

/// Names bound by a mapped type's `[key in …]` within a type's text.
fn mapped_names(text: &str, out: &mut HashSet<String>) {
    let mut rest = text;
    while let Some(k) = rest.find('[') {
        rest = &rest[k + 1..];
        let t = rest.trim_start();
        let word: String = t.chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$').collect();
        if !word.is_empty() && t[word.len()..].trim_start().starts_with("in ") {
            out.insert(word);
        }
    }
}

impl NativeRule for NoUndefinedTypes {
    fn id(&self) -> &'static str {
        "js/jsdoc-no-undefined-types"
    }
    fn languages(&self) -> &'static [Language] {
        super::JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let root = node.get_inner_node();
        let mut nodes = Vec::new();
        crate::util::descendants(root, &|_| false, &mut nodes);
        let blocks: Vec<Block<'_>> = nodes.iter().filter(|c| c.kind() == "comment").filter_map(|c| Block::parse(ctx.source, c.start_byte(), c.end_byte())).collect();
        let mut defined: HashSet<String> = model.symbols.iter().map(|s| s.name.clone()).collect();
        defined.extend(EXTRA_TYPES.iter().map(|s| (*s).to_owned()));
        defined.extend(super::libtypes::TS_LIB_TYPES.iter().map(|s| (*s).to_owned()));
        defined.extend(opt_str_list(ctx.options, "defined_types"));
        defined.extend(opt_str_list(ctx.options, "globals"));
        for env in opt_str_list(ctx.options, "env") {
            match ctx.data.get(&format!("globals/{env}")).map(|v| v.as_ref()) {
                Some(Value::Object(o)) => defined.extend(o.keys().cloned()),
                Some(Value::Array(a)) => defined.extend(a.iter().filter_map(|x| x.as_str().map(str::to_owned))),
                _ => {}
            }
        }
        // `/* globals a, b */` comments.
        for c in nodes.iter().filter(|c| c.kind() == "comment") {
            let t = &ctx.source[c.byte_range()];
            let inner = t.trim_start_matches("/*").trim_end_matches("*/").trim_start();
            if let Some(rest) = inner.strip_prefix("globals").or_else(|| inner.strip_prefix("global")) {
                defined.extend(rest.split([',', ' ', '\n', '\t']).map(|w| w.split(':').next().unwrap_or("").trim()).filter(|w| !w.is_empty()).map(str::to_owned));
            }
        }
        // Names the file's own blocks define: typedefs, callbacks, classes, templates, imports.
        for b in &blocks {
            for t in &b.tags {
                if matches!(role(&t.tag), Role::NamepathDefining | Role::NameDefining) && !t.name.is_empty() {
                    defined.insert(t.name.clone());
                    defined.insert(t.name.split(['.', '#', '~']).next().unwrap_or("").to_owned());
                }
                if t.tag == "template" {
                    for item in t.name.split(',') {
                        let item = item.trim().trim_start_matches('[');
                        defined.insert(item.split(['=', ']', ' ']).next().unwrap_or("").to_owned());
                    }
                }
                if t.tag == "import" {
                    let mut text = t.type_text(ctx.source).map(|x| format!("{x} ")).unwrap_or_default();
                    text.push_str(&t.name);
                    text.push(' ');
                    text.push_str(&b.tag_description(t));
                    let head = text.split(" from ").next().unwrap_or("");
                    defined.extend(head.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '$')).filter(|w| !w.is_empty() && !matches!(*w, "as" | "type")).map(str::to_owned));
                }
            }
        }
        for b in &blocks {
            for t in &b.tags {
                let src_text = if super::tags::might_have_type(&t.tag) && t.tag != "import" { t.type_text(ctx.source) } else { None };
                let mut texts: Vec<&str> = src_text.into_iter().collect();
                if matches!(t.tag.as_str(), "augments" | "extends") && !t.name.is_empty() {
                    texts.push(t.name.as_str());
                }
                for text in texts {
                    let Ok(ty) = types::parse(text) else { continue };
                    // Type parameters of function types, and `infer X`, are in scope within the type.
                    let mut local: HashSet<String> = HashSet::new();
                    mapped_names(text, &mut local);
                    types::walk(&ty, &mut |nd, _: Parent| match &nd.kind {
                        Kind::Func(f) => local.extend(f.tparams.iter().map(|p| p.name.clone())),
                        Kind::Object(ms) => {
                            for m in ms {
                                if let types::MKind::Method { f, .. } | types::MKind::Call { f } = &m.kind {
                                    local.extend(f.tparams.iter().map(|p| p.name.clone()));
                                }
                            }
                        }
                        Kind::Prefix(op, x) if op == "infer" => {
                            if let Kind::Name(n) = &x.kind {
                                local.insert(n.clone());
                            }
                        }
                        _ => {}
                    });
                    types::walk(&ty, &mut |nd, _| {
                        let Kind::Name(name) = &nd.kind else { return };
                        if name.is_empty() || !name.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$') || name.contains(':') {
                            return;
                        }
                        let root_name = name.split(['.', '#', '~']).next().unwrap_or(name);
                        if defined.contains(name) || defined.contains(root_name) || local.contains(root_name) {
                            return;
                        }
                        let probe = codelaw_semantic::Reference { name: root_name.to_owned(), start: 0, end: 0, scope: 0, symbol: None, read: true, write: false, type_only: true, any_namespace: false };
                        if !model.is_undeclared(&probe, &|_| false) {
                            return;
                        }
                        out.push(find(t.at, t.tag_end, format!("The type '{name}' is undefined."), vec![]));
                    });
                }
            }
        }
    }
}

//! codelaw's own config, as it is typed (W-157, SR-EDITOR decision 9).
//!
//! The server opens `codelaw.toml`, `[tool.codelaw]` in `pyproject.toml`
//! and `"codelaw"` in `package.json` (SR-CONFIG decision 1) and offers what
//! goes there: keys and values from the shipped schema, rule ids, presets,
//! scopes and options from the packs the run's own pack search finds
//! ([`codelaw_engine::pack_search`]), and every config load error at its
//! line. Nothing here is a second list: the schema is
//! [`codelaw_config::SCHEMA`], the rules and presets are the packs', and
//! the errors are the loader's.
//!
//! A half-typed file does not parse, so the cursor's place — the table it
//! is in, a key or a value — comes from a tolerant scan, never the loader.

use codelaw_config::{Config, Home};
use codelaw_core::FixClass;
use codelaw_rules::{ENGINE_RULES, ExtendsSpec, FIRST_PARTY_NAMESPACES, FIRST_PARTY_PACKS, OptionSpec, PackSearch, RulePack, RuleSource, load_pack_from, load_rules_dir};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::OnceLock;

/// Where the rule pages are (`scripts/rule-docs.py`'s `BLOB`).
const PAGES: &str = "https://github.com/arpitarya/codelaw/blob/main/docs/rules/";

/// Which home a file is, by its name.
pub fn home(path: &Path) -> Option<Home> {
    match path.file_name()?.to_str()? {
        "codelaw.toml" => Some(Home::CodelawToml),
        "pyproject.toml" => Some(Home::Pyproject),
        "package.json" => Some(Home::PackageJson),
        _ => None,
    }
}

// ---- the cursor's place ------------------------------------------------

/// What the cursor is on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum At {
    /// A key of the table at `path`.
    Key,
    /// A table header's next name, under `path`.
    Header,
    /// The value of the key `path` (an item of it, when `in_array`).
    Value,
}

/// The cursor's place in a config, its path relative to codelaw's own
/// table (`[tool.codelaw]`, `"codelaw"`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    pub path: Vec<String>,
    pub at: At,
    pub in_array: bool,
    /// The partly typed word before the cursor, in bytes: inside the quotes
    /// when `quoted`.
    pub token: (usize, usize),
    pub quoted: bool,
}

/// The cursor's place, or `None` outside codelaw's own table, in a comment,
/// or in a file that is not a home.
pub fn place(text: &str, offset: usize, home: Home) -> Option<Place> {
    let offset = floor(text, offset.min(text.len()));
    let mut p = match home {
        Home::PackageJson => json_place(text, offset)?,
        _ => toml_place(text, offset)?,
    };
    let own: &[&str] = match home {
        Home::CodelawToml => &[],
        Home::Pyproject => &["tool", "codelaw"],
        Home::PackageJson => &["codelaw"],
    };
    if p.path.len() < own.len() || p.path.iter().zip(own).any(|(a, b)| a != b) {
        return None;
    }
    p.path.drain(..own.len());
    Some(p)
}

fn floor(text: &str, mut i: usize) -> usize {
    while !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn is_bare(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-'
}

/// A key being typed: bare, or a rule id not yet quoted.
fn is_keyish(c: u8) -> bool {
    is_bare(c) || matches!(c, b'/' | b':' | b'@')
}

/// A bare value or a word being typed: anything up to a delimiter.
fn is_word(c: u8) -> bool {
    is_bare(c) || matches!(c, b'/' | b':' | b'.' | b'@' | b'+')
}

/// The end of the string opening at `i` (one past its closing quote), and
/// where its content starts; `None` for the end when it is unterminated.
fn string_at(b: &[u8], i: usize, multiline: bool) -> (usize, Option<usize>) {
    let q = b[i];
    let triple = multiline && b.len() >= i + 3 && b[i + 1] == q && b[i + 2] == q;
    let start = if triple { i + 3 } else { i + 1 };
    let mut j = start;
    while j < b.len() {
        if b[j] == b'\\' && q == b'"' {
            j += 2;
            continue;
        }
        if !triple && b[j] == b'\n' {
            return (start, None);
        }
        if b[j] == q && (!triple || (b.len() >= j + 3 && b[j + 1] == q && b[j + 2] == q)) {
            return (start, Some(j + if triple { 3 } else { 1 }));
        }
        j += 1;
    }
    (start, None)
}

fn unquote(s: &str) -> String {
    s.trim().trim_matches('"').trim_matches('\'').to_owned()
}

/// A header's names: `scope."my-scope".rules` → `scope`, `my-scope`, `rules`.
fn dotted(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    for c in s.chars() {
        match (quote, c) {
            (None, '"' | '\'') => quote = Some(c),
            (Some(q), c) if c == q => quote = None,
            (None, '.') => out.push(std::mem::take(&mut cur).trim().to_owned()),
            _ => cur.push(c),
        }
    }
    out.push(cur.trim().to_owned());
    out.into_iter().map(|s| unquote(&s)).collect()
}

enum Frame {
    Inline(Vec<String>),
    Array(Vec<String>),
}

fn toml_place(text: &str, offset: usize) -> Option<Place> {
    let b = text.as_bytes();
    let mut base: Vec<String> = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    let mut key: Vec<String> = Vec::new();
    let mut after_eq = false;
    let mut line_start = true;
    let mut i = 0;
    let container = |base: &Vec<String>, stack: &Vec<Frame>| match stack.last() {
        Some(Frame::Inline(p) | Frame::Array(p)) => p.clone(),
        None => base.clone(),
    };
    while i < offset {
        let c = b[i];
        match c {
            b'\n' => {
                if stack.is_empty() {
                    key.clear();
                    after_eq = false;
                }
                line_start = true;
                i += 1;
                continue;
            }
            b' ' | b'\t' | b'\r' => {
                i += 1;
                continue;
            }
            b'#' => {
                while i < b.len() && b[i] != b'\n' {
                    if i >= offset {
                        return None;
                    }
                    i += 1;
                }
                if i >= offset {
                    return None;
                }
                continue;
            }
            b'[' if line_start && stack.is_empty() => {
                let end = text[i..].find('\n').map_or(text.len(), |n| i + n);
                let open = if b.get(i + 1) == Some(&b'[') { 2 } else { 1 };
                if offset <= end {
                    // The cursor is in this header.
                    let inner = &text[i + open..offset];
                    if inner.contains(']') {
                        return None;
                    }
                    let mut names = dotted(inner);
                    names.pop();
                    let quoted = inner.matches('"').count() % 2 == 1;
                    let start = offset - inner.rsplit(['.', '"', '[', ' ']).next().map_or(0, str::len);
                    return Some(Place { path: names, at: At::Header, in_array: false, token: (start, offset), quoted });
                }
                let header = text[i + open..end].split(']').next().unwrap_or_default();
                base = dotted(header);
                key.clear();
                after_eq = false;
                i = end;
                line_start = false;
                continue;
            }
            b'"' | b'\'' => {
                let (start, end) = string_at(b, i, after_eq);
                if end.is_none_or(|e| offset < e) {
                    // The cursor is in this string.
                    let in_array = matches!(stack.last(), Some(Frame::Array(_)));
                    let mut path = container(&base, &stack);
                    let at = if in_array || after_eq { At::Value } else { At::Key };
                    if !in_array {
                        path.extend(key.iter().cloned());
                    }
                    return Some(Place { path, at, in_array, token: (start.min(offset), offset), quoted: true });
                }
                let end = end.unwrap_or(b.len());
                if !after_eq && !matches!(stack.last(), Some(Frame::Array(_))) {
                    key.push(text[start..end - (start - i)].to_owned());
                }
                i = end;
            }
            b'=' if !after_eq => {
                after_eq = true;
                i += 1;
            }
            b'.' if !after_eq => i += 1,
            b'{' => {
                let mut p = container(&base, &stack);
                if !matches!(stack.last(), Some(Frame::Array(_))) {
                    p.append(&mut key);
                }
                stack.push(Frame::Inline(p));
                after_eq = false;
                i += 1;
            }
            b'[' => {
                let mut p = container(&base, &stack);
                if !matches!(stack.last(), Some(Frame::Array(_))) {
                    p.append(&mut key);
                }
                stack.push(Frame::Array(p));
                i += 1;
            }
            b'}' | b']' => {
                stack.pop();
                key.clear();
                after_eq = true;
                i += 1;
            }
            b',' => {
                if matches!(stack.last(), Some(Frame::Inline(_))) {
                    key.clear();
                    after_eq = false;
                }
                i += 1;
            }
            c if is_word(c) => {
                let s = i;
                let word = |c: u8| if after_eq || matches!(stack.last(), Some(Frame::Array(_))) { is_word(c) } else { is_keyish(c) };
                while i < b.len() && word(b[i]) {
                    i += 1;
                }
                if i == s {
                    i += 1;
                }
                if i >= offset {
                    break;
                }
                if !after_eq && !matches!(stack.last(), Some(Frame::Array(_))) {
                    key.push(text[s..i].to_owned());
                }
            }
            _ => i += 1,
        }
        line_start = false;
    }
    // At the cursor, outside any string: the word before it.
    let in_array = matches!(stack.last(), Some(Frame::Array(_)));
    let value = in_array || after_eq;
    let mut start = offset;
    while start > 0 && (if value { is_word(b[start - 1]) } else { is_keyish(b[start - 1]) }) {
        start -= 1;
    }
    let mut path = container(&base, &stack);
    if !in_array {
        path.extend(key);
    }
    let at = if value { At::Value } else { At::Key };
    Some(Place { path, at, in_array, token: (start, offset), quoted: false })
}

enum JFrame {
    Object { path: Vec<String>, key: Option<String>, colon: bool },
    Array(Vec<String>),
}

fn json_place(text: &str, offset: usize) -> Option<Place> {
    let b = text.as_bytes();
    let mut stack: Vec<JFrame> = Vec::new();
    let mut i = 0;
    let value_path = |stack: &Vec<JFrame>| -> Vec<String> {
        match stack.last() {
            Some(JFrame::Object { path, key, .. }) => path.iter().cloned().chain(key.clone()).collect(),
            Some(JFrame::Array(p)) => p.clone(),
            None => Vec::new(),
        }
    };
    while i < offset {
        match b[i] {
            b'{' => {
                let path = value_path(&stack);
                stack.push(JFrame::Object { path, key: None, colon: false });
                i += 1;
            }
            b'[' => {
                let path = value_path(&stack);
                stack.push(JFrame::Array(path));
                i += 1;
            }
            b'}' | b']' => {
                stack.pop();
                i += 1;
            }
            b':' => {
                if let Some(JFrame::Object { colon, .. }) = stack.last_mut() {
                    *colon = true;
                }
                i += 1;
            }
            b',' => {
                if let Some(JFrame::Object { key, colon, .. }) = stack.last_mut() {
                    *key = None;
                    *colon = false;
                }
                i += 1;
            }
            b'"' => {
                let (start, end) = string_at(b, i, false);
                let is_key = matches!(stack.last(), Some(JFrame::Object { colon: false, .. }));
                if end.is_none_or(|e| offset < e) {
                    return Some(match stack.last() {
                        Some(JFrame::Object { path, colon: false, .. }) => Place { path: path.clone(), at: At::Key, in_array: false, token: (start, offset), quoted: true },
                        Some(JFrame::Array(p)) => Place { path: p.clone(), at: At::Value, in_array: true, token: (start, offset), quoted: true },
                        _ => Place { path: value_path(&stack), at: At::Value, in_array: false, token: (start, offset), quoted: true },
                    });
                }
                let end = end.unwrap_or(b.len());
                if is_key && let Some(JFrame::Object { key, .. }) = stack.last_mut() {
                    *key = Some(text[start..end - 1].to_owned());
                }
                i = end;
            }
            _ => i += 1,
        }
    }
    let mut start = offset;
    while start > 0 && is_word(b[start - 1]) {
        start -= 1;
    }
    Some(match stack.last()? {
        JFrame::Object { path, colon: false, .. } => Place { path: path.clone(), at: At::Key, in_array: false, token: (start, offset), quoted: false },
        JFrame::Array(p) => Place { path: p.clone(), at: At::Value, in_array: true, token: (start, offset), quoted: false },
        JFrame::Object { .. } => Place { path: value_path(&stack), at: At::Value, in_array: false, token: (start, offset), quoted: false },
    })
}

// ---- the schema ----------------------------------------------------------

fn schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| serde_json::from_str(codelaw_config::SCHEMA).expect("codelaw.schema.json is JSON"))
}

fn resolve(mut v: &'static Value) -> &'static Value {
    for _ in 0..16 {
        let Some(r) = v.get("$ref").and_then(Value::as_str) else { break };
        match r.strip_prefix('#').and_then(|p| schema().pointer(p)) {
            Some(t) => v = t,
            None => break,
        }
    }
    v
}

/// The branches of a node: itself, or each of its `oneOf`/`anyOf`.
fn branches(v: &'static Value) -> Vec<&'static Value> {
    let v = resolve(v);
    let mut out = vec![v];
    for k in ["oneOf", "anyOf"] {
        if let Some(list) = v.get(k).and_then(Value::as_array) {
            out.extend(list.iter().flat_map(branches));
        }
    }
    out
}

fn child(v: &'static Value, name: &str) -> Option<&'static Value> {
    for b in branches(v) {
        if let Some(p) = b.get("properties").and_then(|p| p.get(name)) {
            return Some(resolve(p));
        }
        if let Some(a) = b.get("additionalProperties").filter(|a| a.is_object()) {
            return Some(resolve(a));
        }
        if let Some(items) = b.get("items") {
            return child(items, name);
        }
    }
    None
}

/// The schema node of a path, `$ref`s followed.
pub fn node(path: &[String]) -> Option<&'static Value> {
    path.iter().try_fold(resolve(schema()), |v, seg| child(v, seg))
}

fn same(a: Option<&'static Value>, pointer: &str) -> bool {
    match (a, schema().pointer(pointer)) {
        (Some(a), Some(b)) => std::ptr::eq(a, resolve(b)),
        _ => false,
    }
}

/// Whether a path is a table of rule settings: `[rules]` (the schema's
/// `rootRules`, which adds `require_reason`), a scope's or a profile's `rules`.
fn is_rules(path: &[String]) -> bool {
    same(node(path), "/definitions/rules") || same(node(path), "/definitions/rootRules")
}

/// The keys of a rules table that are settings, not rule ids:
/// `require_reason` in the top-level `[rules]` (SR-CONFIG decision 11c).
fn rules_keys(path: &[String]) -> impl Iterator<Item = (&'static String, &'static Value)> {
    node(path).filter(|_| is_rules(path)).and_then(|n| n.get("properties")).and_then(Value::as_object).into_iter().flatten()
}

/// The rule a path is inside, and what follows its id: `[rules]` →
/// `"js/max-lines"` → `options` → `max`. A rules table's own key
/// (`require_reason`) is not a rule.
fn rule_of(path: &[String]) -> Option<(&str, &[String])> {
    (0..path.len())
        .rev()
        .find(|&i| is_rules(&path[..i]) && !rules_keys(&path[..i]).any(|(k, _)| *k == path[i]))
        .map(|i| (path[i].as_str(), &path[i + 1..]))
}

/// The record a key is defined by: its own `x-codelaw-record`, else the
/// nearest table's.
fn record_of(path: &[String]) -> Option<&'static str> {
    (0..=path.len()).rev().find_map(|i| node(&path[..i]).and_then(|n| n.get("x-codelaw-record")).and_then(Value::as_str))
}

fn description(v: &'static Value) -> Option<&'static str> {
    branches(v).into_iter().find_map(|b| b.get("description").and_then(Value::as_str))
}

// ---- what is installed ---------------------------------------------------

/// One rule a config may name.
#[derive(Clone, Debug)]
pub struct RuleInfo {
    pub title: String,
    pub fix: FixClass,
    pub severity: String,
    /// `name@version`, or where a local rule lives.
    pub pack: String,
    pub page: Option<String>,
    pub summary: String,
    pub options: BTreeMap<String, OptionSpec>,
}

/// One preset `extends` may name.
#[derive(Clone, Debug)]
pub struct PresetInfo {
    pub spec: String,
    pub pack: String,
    pub description: Option<String>,
    pub scopes: Vec<String>,
}

/// What the project's pack search finds: every rule, preset and preset
/// scope a config may name, and the first-party packs that are missing.
#[derive(Clone, Debug, Default)]
pub struct Catalogue {
    pub rules: BTreeMap<String, RuleInfo>,
    pub presets: Vec<PresetInfo>,
    /// `(namespace, how to install it)` of each first-party pack not found.
    pub missing: Vec<(String, String)>,
    /// The scopes the config's presets ship: `(name, preset)`.
    pub scopes: Vec<(String, String)>,
    /// The `extends` this was built for.
    pub extends: Vec<String>,
}

fn page(rule: &codelaw_rules::RuleFile, first_party: bool) -> Option<String> {
    if first_party {
        let (ns, name) = rule.id.split_once('/')?;
        return Some(format!("{PAGES}{ns}/{name}.md"));
    }
    Some(crate::uri::from_path(&rule.path))
}

fn add_pack(cat: &mut Catalogue, pack: &RulePack) {
    let label = pack.label();
    for r in &pack.rules {
        cat.rules.entry(r.id.clone()).or_insert_with(|| RuleInfo {
            title: r.title.clone(),
            fix: r.fix,
            severity: r.severity.to_string(),
            pack: label.clone(),
            page: page(r, pack.first_party),
            summary: r.summary.clone(),
            options: r.options.clone(),
        });
    }
    for (name, p) in &pack.presets {
        let spec = if pack.first_party { format!("codelaw:{name}") } else { name.clone() };
        if !pack.first_party || cat.presets.iter().any(|x| x.spec == spec) {
            continue;
        }
        cat.presets.push(PresetInfo { spec, pack: label.clone(), description: p.description.clone(), scopes: p.scope.iter().map(|(n, _)| n.clone()).collect() });
    }
}

/// Every first-party pack the search finds, the team packs `extends` names,
/// the local rules, and — when the project loads — its own rule set, which
/// wins (its precedence is the run's).
pub fn catalogue(config: &Config, search: &PackSearch, project: Option<&codelaw_engine::Project>) -> Catalogue {
    let mut cat = Catalogue { extends: config.extends().to_vec(), ..Default::default() };
    if let Some(p) = project {
        for r in &p.rules.rules {
            let (pack, first_party) = match &r.source {
                RuleSource::Pack { name, version, first_party, .. } => (format!("{name}@{version}"), *first_party),
                other => (other.label(), false),
            };
            cat.rules.insert(
                r.rule.id.clone(),
                RuleInfo {
                    title: r.rule.title.clone(),
                    fix: r.rule.fix,
                    severity: r.rule.severity.to_string(),
                    pack,
                    page: page(&r.rule, first_party),
                    summary: r.rule.summary.clone(),
                    options: r.rule.options.clone(),
                },
            );
        }
    }
    for e in ENGINE_RULES {
        cat.rules.entry(e.id.to_owned()).or_insert_with(|| RuleInfo {
            title: e.title.to_owned(),
            fix: e.fix,
            severity: e.severity.to_string(),
            pack: format!("codelaw {} (the engine's own)", codelaw_core::VERSION),
            page: Some(format!("{PAGES}engine.md")),
            summary: String::new(),
            options: BTreeMap::new(),
        });
    }
    if let Ok(local) = load_rules_dir(&config.rules_dir()) {
        let shown = codelaw_core::normalize_path(config.rules_dir().strip_prefix(&config.root_dir).unwrap_or(&config.rules_dir()));
        for r in local {
            cat.rules.entry(r.id.clone()).or_insert_with(|| RuleInfo {
                title: r.title.clone(),
                fix: r.fix,
                severity: r.severity.to_string(),
                pack: format!("local ({shown})"),
                page: page(&r, false),
                summary: r.summary.clone(),
                options: r.options.clone(),
            });
        }
    }
    for p in FIRST_PARTY_PACKS {
        let ns = FIRST_PARTY_NAMESPACES.iter().find(|n| codelaw_rules::namespace_pack(n) == Some(p.short)).copied().unwrap_or(p.short);
        match search.find_first_party(p.short).and_then(|d| load_pack_from(&d, true).ok()) {
            Some(pack) => add_pack(&mut cat, &pack),
            None => cat.missing.push((ns.to_owned(), PackSearch::missing_first_party(&format!("codelaw:{ns}/…"), p.short))),
        }
    }
    for spec in config.extends() {
        let dir = match ExtendsSpec::parse(spec) {
            Ok(ExtendsSpec::Npm(n)) => search.find_npm(&n),
            Ok(ExtendsSpec::Pypi(n)) => search.find_pypi(&n),
            Ok(ExtendsSpec::Path(p)) => Some(config.config_dir().join(p)).filter(|d| d.join("codelaw-pack.toml").is_file()),
            _ => None,
        };
        if let Some(pack) = dir.and_then(|d| load_pack_from(&d, false).ok()) {
            add_pack(&mut cat, &pack);
        }
    }
    for spec in config.extends() {
        if let Some(p) = cat.presets.iter().find(|p| &p.spec == spec) {
            let scopes: Vec<(String, String)> = p.scopes.iter().map(|s| (s.clone(), spec.clone())).collect();
            cat.scopes.extend(scopes);
        }
    }
    cat
}

impl Catalogue {
    /// The missing first-party packs the config's `extends` names.
    fn missing_extended(&self) -> impl Iterator<Item = &(String, String)> {
        self.missing.iter().filter(|(ns, _)| {
            self.extends.iter().any(|e| {
                e.strip_prefix("codelaw:").and_then(|r| r.split_once('/')).is_some_and(|(n, _)| codelaw_rules::namespace_pack(n) == codelaw_rules::namespace_pack(ns))
            })
        })
    }
}

// ---- completion ------------------------------------------------------------

/// What a completion item is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Key,
    Table,
    Rule,
    Preset,
    Value,
    Note,
}

/// One completion, before it becomes an LSP item.
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub label: String,
    pub insert: String,
    pub kind: Kind,
    pub detail: Option<String>,
    pub doc: Option<String>,
}

fn needs_quotes(s: &str) -> bool {
    s.is_empty() || !s.bytes().all(is_bare)
}

/// The completions at the cursor.
pub fn complete(text: &str, offset: usize, home: Home, cat: &Catalogue) -> Option<(Place, Vec<Item>)> {
    let place = place(text, offset, home)?;
    let json = home == Home::PackageJson;
    let key_insert = |name: &str| -> String {
        if place.quoted {
            name.to_owned()
        } else if json || needs_quotes(name) {
            format!("\"{name}\"")
        } else {
            name.to_owned()
        }
    };
    let string_insert = |v: &str| if place.quoted { v.to_owned() } else { format!("\"{v}\"") };
    let mut items = Vec::new();
    let path = &place.path;
    match place.at {
        At::Key | At::Header => {
            if is_rules(path) {
                for (id, r) in &cat.rules {
                    items.push(Item {
                        label: id.clone(),
                        insert: key_insert(id),
                        kind: Kind::Rule,
                        detail: Some(format!("{} · fix {} · {}", r.title, r.fix, r.pack)),
                        doc: Some(rule_doc(id, r)),
                    });
                }
                for (ns, how) in cat.missing_extended() {
                    items.push(Item { label: format!("{ns}/…"), insert: key_insert(&format!("{ns}/")), kind: Kind::Note, detail: Some(format!("not installed: {how}")), doc: None });
                }
                if place.at == At::Key {
                    for (name, v) in rules_keys(path) {
                        let v = resolve(v);
                        items.push(Item { label: name.clone(), insert: key_insert(name), kind: Kind::Key, detail: Some(type_of(v)), doc: description(v).map(str::to_owned) });
                    }
                }
            } else if same(node(path), "/properties/scope") {
                for (name, preset) in &cat.scopes {
                    items.push(Item { label: name.clone(), insert: key_insert(name), kind: Kind::Table, detail: Some(format!("the scope {preset} ships; yours extends it")), doc: None });
                }
            } else if same(node(path), "/properties/profile") {
                for (name, when) in [("ci", "when CI is set"), ("local", "otherwise: a laptop, the editor")] {
                    items.push(Item { label: name.into(), insert: key_insert(name), kind: Kind::Table, detail: Some(format!("profile {name}: {when}")), doc: None });
                }
            } else if let Some((id, rest)) = rule_of(path).filter(|(_, rest)| rest.len() == 1 && rest[0] == "options") {
                let _ = rest;
                if let Some(r) = cat.rules.get(id) {
                    for (name, spec) in &r.options {
                        let default = spec.default.as_ref().map(|d| format!(" (default {d})")).unwrap_or_default();
                        items.push(Item { label: name.clone(), insert: key_insert(name), kind: Kind::Key, detail: Some(format!("{}{default}", spec.kind)), doc: spec.description.clone() });
                    }
                }
            } else if let Some(n) = node(path) {
                let mut seen = Vec::new();
                for b in branches(n) {
                    for (name, v) in b.get("properties").and_then(Value::as_object).into_iter().flatten() {
                        if seen.contains(name) {
                            continue;
                        }
                        seen.push(name.clone());
                        let v = resolve(v);
                        let table = branches(v).iter().any(|b| b.get("type").and_then(Value::as_str) == Some("object"));
                        if place.at == At::Header && !table {
                            continue;
                        }
                        let mut p = path.clone();
                        p.push(name.clone());
                        let record = record_of(&p).map(|r| format!(" ({r})")).unwrap_or_default();
                        items.push(Item {
                            label: name.clone(),
                            insert: key_insert(name),
                            kind: if table { Kind::Table } else { Kind::Key },
                            detail: Some(format!("{}{record}", type_of(v))),
                            doc: description(v).map(str::to_owned),
                        });
                    }
                }
            }
        }
        At::Value => {
            if path.as_slice() == ["extends"] && place.in_array {
                for p in &cat.presets {
                    items.push(Item { label: p.spec.clone(), insert: string_insert(&p.spec), kind: Kind::Preset, detail: Some(p.pack.clone()), doc: p.description.clone() });
                }
                for (ns, how) in &cat.missing {
                    let spec = format!("codelaw:{ns}/");
                    items.push(Item { label: format!("codelaw:{ns}/…"), insert: string_insert(&spec), kind: Kind::Note, detail: Some(format!("not installed: {how}")), doc: None });
                }
            } else if let Some((id, rest)) = rule_of(path) {
                let rule = cat.rules.get(id);
                match rest {
                    [] | [_] if rest.first().is_none_or(|k| k == "severity") => {
                        for s in ["error", "warn", "info", "off"] {
                            items.push(Item { label: s.into(), insert: string_insert(s), kind: Kind::Value, detail: rule.map(|r| format!("{id}: {} by default", r.severity)), doc: None });
                        }
                    }
                    // A project may lower a rule's fix, never raise it
                    // (SR-LAW-4 decision 16).
                    [k] if k == "fix" => {
                        if let Some(r) = rule {
                            for f in [FixClass::Auto, FixClass::Suggest, FixClass::Manual].into_iter().filter(|f| rung(*f) >= rung(r.fix)) {
                                let what = if f == r.fix { "the rule's own" } else { "lowered" };
                                items.push(Item { label: f.name().into(), insert: string_insert(f.name()), kind: Kind::Value, detail: Some(format!("{what}: {id} is {}", r.fix)), doc: None });
                            }
                        }
                    }
                    [o, name] if o == "options" => {
                        if let Some(spec) = rule.and_then(|r| r.options.get(name)) {
                            let values: Vec<String> = match spec.kind.as_str() {
                                "boolean" => vec!["true".into(), "false".into()],
                                "extend|replace" => vec!["\"extend\"".into(), "\"replace\"".into()],
                                _ => spec.default.iter().map(|d| d.to_string()).collect(),
                            };
                            for v in values {
                                let insert = if place.quoted { v.trim_matches('"').to_owned() } else { v.clone() };
                                let default = spec.default.as_ref().is_some_and(|d| *d == v);
                                items.push(Item { label: v, insert, kind: Kind::Value, detail: Some(format!("{}{}", spec.kind, if default { " · default" } else { "" })), doc: spec.description.clone() });
                            }
                        }
                    }
                    _ => {}
                }
            } else if let Some(n) = node(path) {
                let n = if place.in_array { branches(n).into_iter().find_map(|b| b.get("items")).map(resolve).unwrap_or(n) } else { n };
                for b in branches(n) {
                    for v in b.get("enum").and_then(Value::as_array).into_iter().flatten() {
                        let label = v.as_str().map_or_else(|| v.to_string(), str::to_owned);
                        let insert = if v.is_string() { string_insert(&label) } else { label.clone() };
                        if !items.iter().any(|i: &Item| i.label == label) {
                            items.push(Item { label, insert, kind: Kind::Value, detail: None, doc: description(n).map(str::to_owned) });
                        }
                    }
                    if b.get("type").and_then(Value::as_str) == Some("boolean") && !place.quoted {
                        for v in ["true", "false"] {
                            let default = b.get("default").and_then(Value::as_bool).map(|d| d.to_string()) == Some(v.to_owned());
                            items.push(Item { label: v.into(), insert: v.into(), kind: Kind::Value, detail: default.then(|| "default".to_owned()), doc: description(n).map(str::to_owned) });
                        }
                    }
                }
            }
        }
    }
    Some((place, items))
}

/// auto 0, suggest 1, manual 2: lowering is moving right.
fn rung(f: FixClass) -> u8 {
    match f {
        FixClass::Auto => 0,
        FixClass::Suggest => 1,
        FixClass::Manual => 2,
    }
}

fn type_of(v: &'static Value) -> String {
    let mut kinds: Vec<&str> = branches(v).iter().filter_map(|b| b.get("type").and_then(Value::as_str)).collect();
    kinds.dedup();
    if kinds.is_empty() { "value".into() } else { kinds.join(" or ") }
}

fn rule_doc(id: &str, r: &RuleInfo) -> String {
    let mut s = format!("**{id}** — {}\n\nfix `{}` · `{}` by default · {}", r.title, r.fix, r.severity, r.pack);
    if !r.summary.is_empty() {
        s.push_str(&format!("\n\n{}", r.summary.trim()));
    }
    if let Some(p) = &r.page {
        s.push_str(&format!("\n\n[Rule page]({p})"));
    }
    s
}

// ---- hover -----------------------------------------------------------------

/// The word under the cursor, in bytes.
fn word_at(text: &str, offset: usize) -> (usize, usize) {
    let b = text.as_bytes();
    let offset = floor(text, offset.min(text.len()));
    let (mut s, mut e) = (offset, offset);
    while s > 0 && is_word(b[s - 1]) {
        s -= 1;
    }
    while e < b.len() && is_word(b[e]) {
        e += 1;
    }
    (s, e)
}

/// A rule id's card, a preset's pack, or a key's description.
pub fn hover(text: &str, offset: usize, home: Home, cat: &Catalogue) -> Option<(String, (usize, usize))> {
    let (s, e) = word_at(text, offset);
    let word = &text[s..e];
    place(text, s, home)?;
    if let Some(r) = cat.rules.get(word) {
        return Some((rule_doc(word, r), (s, e)));
    }
    if let Some(p) = cat.presets.iter().find(|p| p.spec == word) {
        let d = p.description.as_deref().map(|d| format!("\n\n{d}")).unwrap_or_default();
        return Some((format!("**{}** — a preset of {}{d}", p.spec, p.pack), (s, e)));
    }
    // A key: the cursor's place is of the key's start; the dotted names
    // before it are part of the path.
    let (ks, ke) = {
        let b = text.as_bytes();
        let (mut ks, mut ke) = (offset.min(text.len()), offset.min(text.len()));
        while ks > 0 && is_bare(b[ks - 1]) {
            ks -= 1;
        }
        while ke < b.len() && is_bare(b[ke]) {
            ke += 1;
        }
        (ks, ke)
    };
    let p = place(text, ks, home)?;
    if !matches!(p.at, At::Key | At::Header) || ks == ke {
        return None;
    }
    let mut path = p.path.clone();
    path.push(text[ks..ke].to_owned());
    let n = node(&path)?;
    let record = record_of(&path).map(|r| format!("\n\nDefined by {r}.")).unwrap_or_default();
    let shown = path.join(".");
    Some((format!("**`{shown}`** — {}{}{record}", type_of(n), description(n).map(|d| format!("\n\n{d}")).unwrap_or_default()), (ks, ke)))
}

// ---- live errors -----------------------------------------------------------

/// A config load error or warning, where it is in the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    pub start: usize,
    pub end: usize,
    pub message: String,
    pub error: bool,
    /// The loader's "did you mean …?", the quick fix's text.
    pub suggestion: Option<String>,
}

/// The loader's "did you mean `x`?".
pub fn suggestion(message: &str) -> Option<String> {
    let rest = message.split("did you mean `").nth(1)?;
    Some(rest.split('`').next()?.to_owned())
}

/// Whether the text holds any name a message gives in backticks: a
/// warning that names nothing here is not shown on this file.
pub fn names_here(text: &str, message: &str) -> bool {
    backticked(message).iter().any(|n| text.contains(n))
}

fn backticked(message: &str) -> Vec<&str> {
    message.split('`').skip(1).step_by(2).filter(|s| !s.is_empty()).collect()
}

fn line_start(text: &str, line: usize) -> Option<usize> {
    if line <= 1 {
        return Some(0);
    }
    text.match_indices('\n').nth(line - 2).map(|(i, _)| i + 1)
}

/// The text a message is about: a `line:column:` it starts with, or the
/// line of a "line N column M", or the last name in backticks that the
/// text holds (not the suggestion), quoted or bare.
pub fn locate(text: &str, message: &str, line: Option<usize>) -> (usize, usize) {
    let near = suggestion(message);
    let names: Vec<&str> = backticked(message).into_iter().filter(|n| Some(*n) != near.as_deref()).collect();
    let mut from = 0;
    let mut limit = text.len();
    let lc = message
        .split_once(": ")
        .and_then(|(head, _)| head.split_once(':'))
        .and_then(|(l, c)| Some((l.trim().parse::<usize>().ok()?, c.trim().parse::<usize>().ok()?)))
        .or_else(|| {
            let rest = message.split("line ").nth(1)?;
            let (l, rest) = rest.split_once(" column ")?;
            Some((l.trim().parse().ok()?, rest.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()?))
        });
    if let Some(l) = lc.map(|(l, _)| l).or(line)
        && let Some(s) = line_start(text, l)
    {
        let end = text[s..].find('\n').map_or(text.len(), |n| s + n);
        let start = match lc {
            Some((_, c)) => text[s..end].char_indices().nth(c.saturating_sub(1)).map_or(end, |(i, _)| s + i),
            None => s,
        };
        from = s;
        limit = end;
        let found = names.iter().rev().find_map(|n| text[from..limit].find(n).map(|i| (from + i, from + i + n.len())));
        if let Some(f) = found {
            return f;
        }
        let (ws, we) = word_at(text, start);
        return if we > ws { (ws, we) } else if lc.is_some() { (start, (start + 1).min(end).max(start)) } else { (s, end) };
    }
    for n in names.iter().rev() {
        for q in [format!("\"{n}\""), format!("'{n}'")] {
            if let Some(i) = text[from..limit].find(&q) {
                return (from + i + 1, from + i + 1 + n.len());
            }
        }
        if let Some(i) = text[from..limit].find(n) {
            return (from + i, from + i + n.len());
        }
        // `[deps] ignore` and the like: the last name of it.
        if let Some(last) = n.rsplit([' ', '.', '[', ']']).find(|s| !s.is_empty())
            && last != *n
            && let Some(i) = text[from..limit].find(last)
        {
            return (from + i, from + i + last.len());
        }
    }
    (0, text.find('\n').unwrap_or(text.len()))
}

/// One problem from the loader's message, placed in `text`.
pub fn problem(text: &str, message: &str, line: Option<usize>, error: bool) -> Problem {
    let (start, end) = locate(text, message, line);
    Problem { start, end, message: message.to_owned(), error, suggestion: suggestion(message) }
}

/// The quick fix's edit for a problem: the unknown name replaced by the
/// suggestion, `codelaw:` kept once.
pub fn fix(text: &str, p: &Problem) -> Option<(usize, usize, String)> {
    let near = p.suggestion.clone()?;
    let mut start = p.start;
    if near.starts_with("codelaw:") && text[..start].ends_with("codelaw:") {
        start -= "codelaw:".len();
    }
    Some((start, p.end, near))
}

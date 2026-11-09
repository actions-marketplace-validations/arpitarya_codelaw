//! W-139 strand W-139-jsfinal: eslint-plugin-unicorn v79.0.0's
//! `name-replacements` (the successor of `prevent-abbreviations`), read from
//! its source and tests and written again, for JavaScript and TypeScript:
//! variable names (every binding of the file, a class or type too), the
//! file's own name, and with `checkProperties` property names. Its CSS,
//! HTML and data halves are the other packs' files, not this rule's.
//!
//! A name is split into words; a word with a replacement (`err` to `error`,
//! `cb` to `callback`, the table is upstream's default) is reported with
//! the names it could have. One replacement is a rename of the binding and
//! every use of it; several are offered as choices.

use super::kit::srule;
use super::{JS, span};
use crate::util::{children, named_children, text};
use crate::{choice, choice_edits};
use codelaw_core::TextEdit;
use codelaw_semantic::{ScopeId, SemanticModel, SymbolId};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tree_sitter::Node;

/// Upstream's default replacements: a discouraged word and the words it
/// stands for.
const DEFAULT_REPLACEMENTS: &[(&str, &[&str])] = &[
    ("acc", &["accumulator"]),
    ("application", &["app"]),
    ("applications", &["apps"]),
    ("arg", &["argument"]),
    ("args", &["arguments"]),
    ("arr", &["array"]),
    ("attr", &["attribute"]),
    ("attrs", &["attributes"]),
    ("btn", &["button"]),
    ("buf", &["buffer"]),
    ("cb", &["callback"]),
    ("cfg", &["config"]),
    ("cmd", &["command"]),
    ("conf", &["config"]),
    ("configuration", &["config"]),
    ("ctx", &["context"]),
    ("cur", &["current"]),
    ("curr", &["current"]),
    ("db", &["database"]),
    ("decl", &["declaration"]),
    ("decls", &["declarations"]),
    ("def", &["defer", "deferred", "define", "definition"]),
    ("dep", &["dependency"]),
    ("deps", &["dependencies"]),
    ("dest", &["destination"]),
    ("dev", &["development"]),
    ("dir", &["direction", "directory"]),
    ("dirs", &["directories"]),
    ("dist", &["distribution"]),
    ("doc", &["document"]),
    ("docs", &["documentation", "documents"]),
    ("dst", &["daylightSavingTime", "destination", "distribution"]),
    ("e", &["error", "event"]),
    ("el", &["element"]),
    ("elem", &["element"]),
    ("elems", &["elements"]),
    ("env", &["environment"]),
    ("envs", &["environments"]),
    ("err", &["error"]),
    ("ev", &["event"]),
    ("evt", &["event"]),
    ("expr", &["expression"]),
    ("exprs", &["expressions"]),
    ("ext", &["extension"]),
    ("exts", &["extensions"]),
    ("fn", &["function"]),
    ("func", &["function"]),
    ("i", &["index"]),
    ("ident", &["identifier"]),
    ("idents", &["identifiers"]),
    ("idx", &["index"]),
    ("j", &["index"]),
    ("len", &["length"]),
    ("lib", &["library"]),
    ("mod", &["module"]),
    ("msg", &["message"]),
    ("num", &["number"]),
    ("obj", &["object"]),
    ("opts", &["options"]),
    ("param", &["parameter"]),
    ("params", &["parameters"]),
    ("perf", &["performance"]),
    ("pkg", &["package"]),
    ("prev", &["previous"]),
    ("prod", &["production"]),
    ("prop", &["property"]),
    ("props", &["properties"]),
    ("proto", &["prototype"]),
    ("ref", &["reference"]),
    ("refs", &["references"]),
    ("rel", &["related", "relationship", "relative"]),
    ("repo", &["repository"]),
    ("req", &["request"]),
    ("res", &["resource", "response", "result"]),
    ("ret", &["returnValue"]),
    ("retval", &["returnValue"]),
    ("sep", &["separator"]),
    ("src", &["source"]),
    ("stdDev", &["standardDeviation"]),
    ("stmt", &["statement"]),
    ("stmts", &["statements"]),
    ("str", &["string"]),
    ("tbl", &["table"]),
    ("temp", &["temporary"]),
    ("tit", &["title"]),
    ("tmp", &["temporary"]),
    ("util", &["utility"]),
    ("utils", &["utilities"]),
    ("val", &["value"]),
    ("var", &["variable"]),
    ("vars", &["variables"]),
    ("ver", &["version"]),
];

const DEFAULT_ALLOW: &[&str] = &["defaultProps", "devDependencies", "EmberENV", "getDerivedStateFromProps", "getInitialProps", "getServerSideProps", "getStaticProps", "ImportMetaEnv", "iOS", "ProcessEnv", "propTypes", "setupFilesAfterEnv"];

const DEFAULT_IGNORE: &[&str] = &["i18n", "l10n", "a11y", "e2e", "jQuery", "vite-env"];

static DEFAULT_IGNORE_RE: std::sync::LazyLock<Vec<Regex>> = std::sync::LazyLock::new(|| DEFAULT_IGNORE.iter().filter_map(|p| Regex::new(p).ok()).collect());

#[derive(Clone, Copy, PartialEq)]
enum Imports {
    Internal,
    All,
    None,
}

struct Opts {
    check_properties: bool,
    check_variables: bool,
    default_and_namespace: Imports,
    shorthand_imports: Imports,
    shorthand_properties: bool,
    check_filenames: bool,
    replacements: HashMap<String, BTreeMap<String, bool>>,
    allow: BTreeSet<String>,
    ignore: Vec<Regex>,
}

fn flag(o: &BTreeMap<String, serde_json::Value>, key: &str, default: bool) -> bool {
    o.get(key).and_then(serde_json::Value::as_bool).unwrap_or(default)
}

fn imports_mode(o: &BTreeMap<String, serde_json::Value>, key: &str) -> Imports {
    match o.get(key) {
        Some(serde_json::Value::String(s)) => match s.as_str() {
            "all" | "true" => Imports::All,
            "none" | "false" => Imports::None,
            _ => Imports::Internal,
        },
        Some(serde_json::Value::Bool(true)) => Imports::All,
        Some(serde_json::Value::Bool(false)) => Imports::None,
        _ => Imports::Internal,
    }
}

fn opts(o: &BTreeMap<String, serde_json::Value>) -> Opts {
    let custom = o.get("replacements").and_then(serde_json::Value::as_object);
    let extend = flag(o, "extendDefaultReplacements", true);
    let mut replacements: HashMap<String, BTreeMap<String, bool>> = HashMap::new();
    let entry_of = |v: &serde_json::Value| -> Option<BTreeMap<String, bool>> { v.as_object().map(|m| m.iter().map(|(k, v)| (k.clone(), v.as_bool().unwrap_or(false))).collect()) };
    if extend {
        for (name, words) in DEFAULT_REPLACEMENTS {
            replacements.insert((*name).to_owned(), words.iter().map(|w| ((*w).to_owned(), true)).collect());
        }
        if let Some(c) = custom {
            for (name, v) in c {
                if v.as_bool() == Some(false) {
                    replacements.insert(name.clone(), BTreeMap::new());
                } else if let Some(e) = entry_of(v) {
                    replacements.entry(name.clone()).or_default().extend(e);
                }
            }
        }
    } else if let Some(c) = custom {
        for (name, v) in c {
            replacements.insert(name.clone(), entry_of(v).unwrap_or_default());
        }
    }
    let mut allow: BTreeSet<String> = if flag(o, "extendDefaultAllowList", true) { DEFAULT_ALLOW.iter().map(|s| (*s).to_owned()).collect() } else { BTreeSet::new() };
    if let Some(a) = o.get("allowList").and_then(serde_json::Value::as_object) {
        for (k, v) in a {
            if v.as_bool().unwrap_or(false) {
                allow.insert(k.clone());
            } else {
                allow.remove(k);
            }
        }
    }
    let mut ignore: Vec<Regex> = DEFAULT_IGNORE_RE.clone();
    if let Some(list) = o.get("ignore").and_then(serde_json::Value::as_array) {
        ignore.extend(list.iter().filter_map(|v| v.as_str()).filter_map(|p| Regex::new(p).ok()));
    }
    Opts {
        check_properties: flag(o, "checkProperties", false),
        check_variables: flag(o, "checkVariables", true),
        default_and_namespace: imports_mode(o, "checkDefaultAndNamespaceImports"),
        shorthand_imports: imports_mode(o, "checkShorthandImports"),
        shorthand_properties: flag(o, "checkShorthandProperties", false),
        check_filenames: flag(o, "checkFilenames", true),
        replacements,
        allow,
        ignore,
    }
}

fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_lowercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

fn upper_first(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

fn is_upper_case(s: &str) -> bool {
    s == s.to_uppercase()
}

/// `localeCompare` for the names here: by letters first, ignoring case, then lowercase before uppercase.
fn locale_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase().cmp(&b.to_lowercase()).then_with(|| b.cmp(a))
}

fn word_replacements(word: &str, o: &Opts) -> Vec<String> {
    let found = o.replacements.get(&lower_first(word)).or_else(|| o.replacements.get(word)).or_else(|| o.replacements.get(&upper_first(word)));
    let Some(found) = found else { return Vec::new() };
    if is_upper_case(word) || o.allow.contains(word) {
        return Vec::new();
    }
    let upper = word.chars().next().is_some_and(|c| c.to_string() == c.to_uppercase().to_string());
    let mut out: Vec<String> = found.iter().filter(|(_, on)| **on).map(|(name, _)| if upper { upper_first(name) } else { lower_first(name) }).collect();
    out.sort_by(|a, b| locale_cmp(a, b));
    out
}

/// A name split before every character that is not a lowercase letter and
/// after every one that is not a letter.
fn split_words(name: &str) -> Vec<String> {
    let chars: Vec<char> = name.chars().collect();
    let mut words = Vec::new();
    let mut cur = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (!c.is_lowercase() || !chars[i - 1].is_alphabetic()) && !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
        cur.push(*c);
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words
}

struct Found {
    total: usize,
    samples: Vec<String>,
}

fn name_replacements(name: &str, o: &Opts, limit: usize) -> Option<Found> {
    if is_upper_case(name) || o.allow.contains(name) || o.ignore.iter().any(|r| r.is_match(name)) {
        return None;
    }
    let exact = word_replacements(name, o);
    if !exact.is_empty() {
        return Some(Found { total: exact.len(), samples: exact.into_iter().take(limit).collect() });
    }
    if name.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let mut any = false;
    let combos: Vec<Vec<String>> = split_words(name)
        .into_iter()
        .map(|w| {
            let r = word_replacements(&w, o);
            if r.is_empty() {
                vec![w]
            } else {
                any = true;
                r
            }
        })
        .collect();
    if !any {
        return None;
    }
    let total: usize = combos.iter().map(Vec::len).product();
    let mut samples = Vec::new();
    for k in 0..total.min(limit) {
        let mut rest = k;
        let mut parts: Vec<String> = vec![String::new(); combos.len()];
        for i in (0..combos.len()).rev() {
            let len = combos[i].len();
            parts[i] = combos[i][rest % len].clone();
            rest /= len;
        }
        // `retVal` -> `returnValue` + `Value` -> `returnValue`
        let mut i = parts.len();
        while i > 1 {
            i -= 1;
            if parts[i].chars().all(|c| c.is_ascii_alphabetic()) && parts[i - 1].ends_with(parts[i].as_str()) {
                parts.remove(i);
            }
        }
        samples.push(parts.concat());
    }
    Some(Found { total, samples })
}

fn message(old: &str, f: &Found, kind: &str) -> String {
    if f.total == 1 {
        return format!("The {kind} `{old}` should be named `{}`. A more descriptive name will do too.", f.samples.first().map(String::as_str).unwrap_or_default());
    }
    let mut list = f.samples.iter().map(|s| format!("`{s}`")).collect::<Vec<_>>().join(", ");
    let omitted = f.total.saturating_sub(f.samples.len());
    if omitted > 0 {
        list.push_str(&format!(", ... ({} more omitted)", if omitted > 99 { "99+".to_owned() } else { omitted.to_string() }));
    }
    format!("Please rename the {kind} `{old}`. Suggested names are: {list}. A more descriptive name will do too.")
}

const RESERVED: &[&str] = &[
    "break", "case", "catch", "class", "const", "continue", "debugger", "default", "delete", "do", "else", "enum", "export", "extends", "false", "finally", "for", "function", "if", "import", "in", "instanceof", "new", "null", "return", "super", "switch", "this", "throw", "true", "try", "typeof", "var", "void", "while", "with", "as", "implements", "interface", "let", "package", "private", "protected", "public", "static", "yield", "any", "boolean", "constructor", "declare", "get", "module", "require", "number", "set", "string", "symbol", "type", "from", "of",
];

fn valid_identifier(name: &str) -> bool {
    let mut c = name.chars();
    c.next().is_some_and(|f| f.is_alphabetic() || f == '$' || f == '_') && c.all(|x| x.is_alphanumeric() || x == '$' || x == '_') && !RESERVED.contains(&name)
}

// ---- scopes -------------------------------------------------------------------

fn visible(m: &SemanticModel, scope: ScopeId, name: &str) -> bool {
    let mut cur = Some(scope);
    while let Some(c) = cur {
        if m.scopes[c].bindings.contains_key(name) {
            return true;
        }
        cur = m.scopes[c].parent;
    }
    false
}

/// The offsets of the references to nothing in the file, by name.
type Unresolved<'m> = HashMap<&'m str, Vec<usize>>;

fn unresolved_of(m: &SemanticModel) -> Unresolved<'_> {
    let mut out: Unresolved<'_> = HashMap::new();
    for r in m.references.iter().filter(|r| r.symbol.is_none()) {
        out.entry(r.name.as_str()).or_default().push(r.start);
    }
    out
}

fn unresolved_in(m: &SemanticModel, un: &Unresolved<'_>, scope: ScopeId, name: &str) -> bool {
    let s = &m.scopes[scope];
    un.get(name).is_some_and(|v| v.iter().any(|&at| at >= s.start && at < s.end))
}

fn available(name: &str, scopes: &[ScopeId], m: &SemanticModel, un: &Unresolved<'_>, tracker: &BTreeMap<ScopeId, BTreeSet<String>>) -> Option<String> {
    let mut name = name.to_owned();
    if !valid_identifier(&name) {
        name.push('_');
        if !valid_identifier(&name) {
            return None;
        }
    }
    while scopes.iter().any(|s| visible(m, *s, &name) || unresolved_in(m, un, *s, &name) || tracker.get(s).is_some_and(|t| t.contains(&name))) {
        name.push('_');
    }
    Some(name)
}

// ---- what may be renamed ----------------------------------------------------------

fn exported_declaration(id: Node<'_>) -> bool {
    let mut stmt = id.parent();
    // `export const e = 1`, `export function e() {}`, `export class E {}`, `export type …`.
    while let Some(s) = stmt {
        if matches!(s.kind(), "variable_declarator" | "lexical_declaration" | "variable_declaration") {
            stmt = s.parent();
            continue;
        }
        break;
    }
    let Some(s) = stmt else { return false };
    if !matches!(s.kind(), "function_declaration" | "generator_function_declaration" | "class_declaration" | "abstract_class_declaration" | "lexical_declaration" | "variable_declaration" | "type_alias_declaration" | "interface_declaration" | "enum_declaration" | "internal_module" | "module") {
        return false;
    }
    s.parent().is_some_and(|p| p.kind() == "export_statement" && !children(p).iter().any(|c| c.kind() == "default"))
}

fn is_internal(source: &str) -> bool {
    !source.contains("node_modules") && (source.starts_with('.') || source.starts_with('/'))
}

/// The module a default or namespace import, a shorthand named import or a
/// `require()` binding comes from.
enum Binding {
    Plain,
    /// `import x from`, `import * as x from`, `import {default as x} from`, `const x = require('…')`.
    DefaultOrNamespace(String),
    /// `import {x} from`.
    ShorthandImport(String),
}

fn binding_of(id: Node<'_>, src: &str) -> Binding {
    let Some(p) = id.parent() else { return Binding::Plain };
    let source_of = |n: Node<'_>| -> String {
        let mut cur = Some(n);
        while let Some(c) = cur {
            if c.kind() == "import_statement" {
                return c.child_by_field_name("source").map(|s| crate::util::unquote(text(s, src)).to_owned()).unwrap_or_default();
            }
            cur = c.parent();
        }
        String::new()
    };
    match p.kind() {
        "import_clause" => Binding::DefaultOrNamespace(source_of(p)),
        "namespace_import" => Binding::DefaultOrNamespace(source_of(p)),
        "import_specifier" => {
            let name = p.child_by_field_name("name");
            let alias = p.child_by_field_name("alias");
            let is_local = alias.map_or(name.is_some_and(|n| n.id() == id.id()), |a| a.id() == id.id());
            if !is_local {
                return Binding::Plain;
            }
            if name.is_some_and(|n| text(n, src) == "default") && alias.is_some() {
                return Binding::DefaultOrNamespace(source_of(p));
            }
            if alias.is_none() { Binding::ShorthandImport(source_of(p)) } else { Binding::Plain }
        }
        "variable_declarator" if p.child_by_field_name("name").is_some_and(|n| n.id() == id.id()) => {
            let Some(v) = p.child_by_field_name("value").filter(|v| v.kind() == "call_expression") else { return Binding::Plain };
            let is_require = v.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "require");
            let args = crate::js_ast::arguments(v);
            if is_require && args.len() == 1 && matches!(args[0].kind(), "string" | "template_string") { Binding::DefaultOrNamespace(crate::util::unquote(text(args[0], src)).to_owned()) } else { Binding::Plain }
        }
        _ => Binding::Plain,
    }
}

fn shorthand_value(id: Node<'_>) -> bool {
    matches!(id.kind(), "shorthand_property_identifier" | "shorthand_property_identifier_pattern") && id.parent().is_none_or(|p| p.kind() != "object_assignment_pattern")
}

fn has_param_jsdoc(param: Node<'_>, src: &str) -> bool {
    let Some(func) = std::iter::successors(param.parent(), |p| p.parent()).find(|a| crate::js_ast::is_function(*a) || a.kind() == "function_type" || a.kind() == "method_signature") else { return false };
    let mut at = func;
    for _ in 0..6 {
        if let Some(c) = at.prev_sibling().filter(|c| c.kind() == "comment") {
            let t = text(c, src);
            return t.starts_with("/*") && t[2..].trim_start().starts_with('*') && t.contains("@param") && c.end_position().row + 1 >= at.start_position().row;
        }
        match at.parent() {
            Some(p) if matches!(p.kind(), "variable_declarator" | "lexical_declaration" | "variable_declaration" | "export_statement" | "expression_statement" | "assignment_expression" | "pair" | "method_definition" | "field_definition" | "public_field_definition" | "type_alias_declaration") => at = p,
            _ => return false,
        }
    }
    false
}

fn parameter_property(id: Node<'_>) -> bool {
    id.parent().is_some_and(|p| matches!(p.kind(), "required_parameter" | "optional_parameter") && children(p).iter().any(|c| matches!(c.kind(), "accessibility_modifier" | "override_modifier")))
}

fn is_parameter(id: Node<'_>) -> bool {
    std::iter::successors(id.parent(), |p| p.parent()).take(4).any(|p| matches!(p.kind(), "formal_parameters" | "required_parameter" | "optional_parameter" | "rest_pattern" | "assignment_pattern")) && std::iter::successors(id.parent(), |p| p.parent()).any(|p| p.kind() == "formal_parameters")
}

/// The edits renaming a symbol everywhere it is written, `None` when one of its uses cannot be renamed.
fn rename_edits(m: &SemanticModel, root: Node<'_>, src: &str, sym: SymbolId, new: &str) -> Option<Vec<TextEdit>> {
    let s = m.symbol(sym);
    let mut spans: Vec<(usize, usize)> = s.decls.clone();
    spans.extend(s.refs.iter().map(|&r| (m.references[r].start, m.references[r].end)));
    spans.sort_unstable();
    spans.dedup();
    let mut edits = Vec::new();
    for (a, b) in spans {
        let n = root.descendant_for_byte_range(a, b)?;
        let old = &src[a..b];
        let replacement = match n.kind() {
            "jsx_identifier" => return None,
            "shorthand_property_identifier" | "shorthand_property_identifier_pattern" => format!("{old}: {new}"),
            "identifier" | "type_identifier" | "property_identifier" => {
                let p = n.parent();
                match p.map(|p| (p.kind(), p)) {
                    Some(("object_assignment_pattern", p)) if p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()) => format!("{old}: {new}"),
                    Some(("import_specifier", p)) if p.child_by_field_name("alias").is_none() => format!("{old} as {new}"),
                    Some(("export_specifier", p)) if p.child_by_field_name("alias").is_none() => format!("{new} as {old}"),
                    _ => new.to_owned(),
                }
            }
            _ => return None,
        };
        edits.push(TextEdit::new(a, b, replacement));
    }
    Some(edits)
}

fn file_ext_is_js(path: &str) -> bool {
    matches!(path.rsplit('.').next(), Some("js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "mts" | "cts"))
}

srule!(NameReplacements, "js/name-replacements", JS, &["program"], |n, ctx, m, out| {
    let src = ctx.source;
    let o = opts(ctx.options);
    let root = n;
    // The file's own name.
    let path = ctx.file.path();
    if o.check_filenames && !path.is_empty() && file_ext_is_js(path) {
        let file = path.rsplit('/').next().unwrap_or(path);
        let (stem, ext) = match file.rfind('.') {
            Some(i) if i > 0 => (&file[..i], &file[i..]),
            _ => (file, ""),
        };
        if let Some(mut f) = name_replacements(stem, &o, 3)
            && let Some(first) = named_children(n).into_iter().find(|c| c.kind() != "comment")
        {
            f.samples = f.samples.iter().map(|s| format!("{s}{ext}")).collect();
            out.push(span(first.start_byte(), first.end_byte(), message(file, &f, "filename"), vec![]));
        }
    }
    if o.check_variables {
        let mut tracker: BTreeMap<ScopeId, BTreeSet<String>> = BTreeMap::new();
        let unresolved = unresolved_of(m);
        for (sym_id, s) in m.symbols.iter().enumerate() {
            let Some(&(ds, de)) = s.decls.first() else { continue };
            let Some(id) = root.descendant_for_byte_range(ds, de) else { continue };
            if id.parent().is_some_and(|p| p.kind() == "enum_body" || p.kind() == "enum_assignment") {
                continue;
            }
            match binding_of(id, src) {
                Binding::DefaultOrNamespace(source) if o.default_and_namespace == Imports::None || (o.default_and_namespace == Imports::Internal && !is_internal(&source)) => continue,
                Binding::ShorthandImport(source) if o.shorthand_imports == Imports::None || (o.shorthand_imports == Imports::Internal && !is_internal(&source)) => continue,
                _ => {}
            }
            if !o.shorthand_properties && shorthand_value(id) {
                continue;
            }
            let Some(mut found) = name_replacements(&s.name, &o, 3) else { continue };
            let mut scopes: Vec<ScopeId> = s.refs.iter().map(|&r| m.references[r].scope).collect();
            scopes.push(s.scope);
            scopes.sort_unstable();
            scopes.dedup();
            found.samples = found.samples.iter().filter_map(|nm| available(nm, &scopes, m, &unresolved, &tracker)).collect();
            if found.samples.is_empty() {
                continue;
            }
            // May the variable be renamed at all?
            let renamable = !s.decls.iter().filter_map(|(a, b)| root.descendant_for_byte_range(*a, *b)).any(exported_declaration) && !(is_parameter(id) && (parameter_property(id) || has_param_jsdoc(id, src)));
            let mut edits = Vec::new();
            if renamable {
                if found.total == 1 {
                    if let Some(e) = rename_edits(m, root, src, sym_id, &found.samples[0]) {
                        for sc in &scopes {
                            tracker.entry(*sc).or_default().insert(found.samples[0].clone());
                        }
                        edits = e;
                    }
                } else {
                    let choices = found.samples.iter().filter(|nm| valid_identifier(nm)).filter_map(|nm| rename_edits(m, root, src, sym_id, nm).map(|e| choice(nm.clone(), false, e))).collect::<Vec<_>>();
                    edits = choice_edits(choices);
                }
            }
            out.push(span(ds, de, message(&s.name, &found, "variable"), edits));
        }
    }
    if o.check_properties {
        let mut stack = vec![n];
        while let Some(x) = stack.pop() {
            stack.extend(children(x));
            if !matches!(x.kind(), "property_identifier" | "identifier") {
                continue;
            }
            if !is_property_name(x, src) || text(x, src) == "__proto__" {
                continue;
            }
            let Some(found) = name_replacements(text(x, src), &o, 3) else { continue };
            let edits = if found.total > 1 && x.parent().is_none_or(|p| p.kind() != "export_specifier") {
                choice_edits(found.samples.iter().filter(|nm| valid_identifier(nm)).map(|nm| choice(nm.clone(), false, vec![TextEdit::new(x.start_byte(), x.end_byte(), nm.clone())])).collect())
            } else {
                vec![]
            };
            out.push(span(x.start_byte(), x.end_byte(), message(text(x, src), &found, "property"), edits));
        }
    }
});

/// A property name that is defined or written here: `a.b = …`, `a.b++`, a
/// non-shorthand key of an object literal, `export { a as b }`, a class or
/// interface member.
fn is_property_name(x: Node<'_>, _src: &str) -> bool {
    let Some(p) = x.parent() else { return false };
    let is = |f: &str| p.child_by_field_name(f).is_some_and(|n| n.id() == x.id());
    match p.kind() {
        "member_expression" if is("property") && x.kind() == "property_identifier" => p.parent().is_some_and(|g| matches!(g.kind(), "assignment_expression" | "augmented_assignment_expression") && g.child_by_field_name("left").is_some_and(|l| l.id() == p.id()) || g.kind() == "update_expression"),
        "pair" => is("key") && x.kind() == "property_identifier" && p.parent().is_some_and(|g| g.kind() == "object"),
        "export_specifier" => is("alias") && x.kind() == "identifier",
        "method_definition" | "field_definition" | "public_field_definition" | "property_signature" | "method_signature" | "abstract_method_signature" => (is("name") || is("property")) && x.kind() == "property_identifier",
        _ => false,
    }
}

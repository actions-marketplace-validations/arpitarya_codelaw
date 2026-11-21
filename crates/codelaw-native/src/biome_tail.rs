//! W-196: the rows of Biome 2.6.4's rule list that a pattern cannot state —
//! `js/no-process-global`, `js/no-shouty-constants`, `js/no-exported-imports`,
//! `js/no-redundant-default-export`, `js/no-parameters-only-used-in-recursion`
//! and the `package.json` and JSON rules `json/no-duplicate-dependencies`,
//! `json/use-required-scripts` and `json/no-json-unsafe-values`. Behaviour
//! read from Biome's description of each rule and written again.

use crate::data_json::pairs;
use crate::js_ast::{self, arguments, root_of};
use crate::util::{children, named_children, remove_statement, text};
use crate::{NativeContext, NativeFinding, NativeRule, as_suggestion};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::SymbolKind;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const JSON: &[Language] = &[Language::Json];

fn finding(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

macro_rules! native {
    ($name:ident, $id:literal, $langs:expr, $kinds:expr, semantic, |$n:ident, $ctx:ident, $m:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                $langs
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let Some($m) = $ctx.semantic else { return };
                let $n = node.get_inner_node();
                $body
            }
        }
    };
    ($name:ident, $id:literal, $langs:expr, $kinds:expr, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                $langs
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

// ---------------------------------------------------------------- JavaScript

/// Where a new top-level import goes: after the last import, else after a
/// hashbang and the directives, else the start of the file.
fn import_point(root: Node<'_>) -> (usize, bool) {
    let kids = named_children(root);
    if let Some(last) = kids.iter().filter(|k| k.kind() == "import_statement").last() {
        return (last.end_byte(), true);
    }
    let mut at = 0;
    let mut after = false;
    for k in kids {
        let directive = k.kind() == "expression_statement" && k.named_child(0).is_some_and(|c| c.kind() == "string");
        if k.kind() == "hash_bang_line" || directive {
            at = k.end_byte();
            after = true;
        } else if k.kind() != "comment" {
            break;
        }
    }
    (at, after)
}

native!(NoProcessGlobal, "js/no-process-global", JS, &["identifier"], semantic, |n, ctx, m, out| {
    let src = ctx.source;
    if text(n, src) != "process" || !m.reference_at(n.start_byte()).is_some_and(|r| r.symbol.is_none()) {
        return;
    }
    let (at, after) = import_point(root_of(n));
    let line = "import process from \"node:process\";";
    let edit = if after { TextEdit::insert(at, format!("\n{line}")) } else { TextEdit::insert(at, format!("{line}\n")) };
    out.push(finding(n, "Import `process` from `node:process` instead of using the global", as_suggestion(vec![edit])));
});

native!(NoShoutyConstants, "js/no-shouty-constants", JS, &["variable_declarator"], semantic, |n, ctx, m, out| {
    let src = ctx.source;
    let (Some(name), Some(value)) = (n.child_by_field_name("name"), n.child_by_field_name("value")) else { return };
    if name.kind() != "identifier" || value.kind() != "string" {
        return;
    }
    let ident = text(name, src);
    let lit = crate::util::string_value(value, src);
    if ident != lit || ident.chars().any(|c| c.is_ascii_lowercase()) || !ident.chars().any(|c| c.is_ascii_uppercase()) {
        return;
    }
    let Some(decl) = n.parent().filter(|p| p.kind() == "lexical_declaration" && text(*p, src).starts_with("const")) else { return };
    if named_children(decl).iter().filter(|c| c.kind() == "variable_declarator").count() != 1 || decl.parent().is_some_and(|p| p.kind() != "program" && p.kind() != "statement_block") {
        return;
    }
    let Some(sym) = m.symbol_at(name.start_byte()) else { return };
    let s = m.symbol(sym);
    if s.exported || s.refs.len() != 1 {
        return;
    }
    let r = &m.references[s.refs[0]];
    if r.write || r.type_only {
        return;
    }
    let shorthand = root_of(n).descendant_for_byte_range(r.start, r.end).is_some_and(|u| matches!(u.kind(), "shorthand_property_identifier") || u.parent().is_some_and(|p| p.kind() == "export_specifier"));
    let edits = if shorthand { Vec::new() } else { as_suggestion(vec![remove_statement(decl, src), TextEdit::new(r.start, r.end, text(value, src))]) };
    out.push(finding(name, format!("`{ident}` is a constant whose value is its own name; use the string"), edits));
});

native!(NoExportedImports, "js/no-exported-imports", JS, &["export_specifier"], semantic, |n, ctx, m, out| {
    let src = ctx.source;
    let Some(clause) = n.parent().filter(|p| p.kind() == "export_clause") else { return };
    if clause.parent().is_some_and(|e| e.child_by_field_name("source").is_some()) {
        return;
    }
    let Some(local) = n.child_by_field_name("name") else { return };
    let Some(sym) = m.symbol_at(local.start_byte()) else { return };
    if m.symbol(sym).kind != SymbolKind::Import {
        return;
    }
    out.push(finding(n, format!("`{}` is imported and then exported; re-export it with `export {{ … }} from`", text(local, src)), Vec::new()));
});

/// Names an `export` statement without `default` puts out under their own name.
fn named_exports(e: Node<'_>, src: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(d) = e.child_by_field_name("declaration") {
        match d.kind() {
            "lexical_declaration" | "variable_declaration" => {
                for v in named_children(d).into_iter().filter(|c| c.kind() == "variable_declarator") {
                    if let Some(nm) = v.child_by_field_name("name").filter(|x| x.kind() == "identifier") {
                        out.push(text(nm, src).to_owned());
                    }
                }
            }
            _ => {
                if let Some(nm) = d.child_by_field_name("name") {
                    out.push(text(nm, src).to_owned());
                }
            }
        }
    } else if e.child_by_field_name("source").is_none() {
        for c in named_children(e).into_iter().filter(|c| c.kind() == "export_clause") {
            for s in named_children(c).into_iter().filter(|s| s.kind() == "export_specifier") {
                if let Some(local) = s.child_by_field_name("name") {
                    out.push(text(local, src).to_owned());
                }
            }
        }
    }
    out
}

native!(NoRedundantDefaultExport, "js/no-redundant-default-export", JS, &["export_statement"], |n, ctx, out| {
    let src = ctx.source;
    if !children(n).iter().any(|c| c.kind() == "default") {
        return;
    }
    let Some(value) = n.child_by_field_name("value").filter(|v| v.kind() == "identifier") else { return };
    let name = text(value, src);
    let Some(program) = n.parent() else { return };
    let dup = named_children(program).into_iter().filter(|e| e.kind() == "export_statement" && e.id() != n.id() && !children(*e).iter().any(|c| c.kind() == "default")).any(|e| named_exports(e, src).iter().any(|x| x == name));
    if dup {
        out.push(finding(n, format!("`{name}` is exported by name as well; this default export is redundant"), Vec::new()));
    }
});

fn param_ident(p: Node<'_>) -> Option<Node<'_>> {
    match p.kind() {
        "identifier" => Some(p),
        "required_parameter" | "optional_parameter" => p.child_by_field_name("pattern").filter(|x| x.kind() == "identifier"),
        _ => None,
    }
}

native!(NoParametersOnlyUsedInRecursion, "js/no-parameters-only-used-in-recursion", JS, &["function_declaration", "method_definition"], semantic, |n, ctx, m, out| {
    let src = ctx.source;
    let Some(name_node) = n.child_by_field_name("name") else { return };
    let method = n.kind() == "method_definition";
    let fname = text(name_node, src);
    let fsym = if method { None } else { m.symbol_at(name_node.start_byte()) };
    if !method && fsym.is_none() {
        return;
    }
    let root = root_of(n);
    for (i, p) in js_ast::params(n).iter().enumerate() {
        let Some(id) = param_ident(*p) else { continue };
        if text(id, src).starts_with('_') {
            continue;
        }
        let Some(sym) = m.symbol_at(id.start_byte()) else { continue };
        let s = m.symbol(sym);
        if s.refs.is_empty() {
            continue;
        }
        let mut uses = Vec::new();
        let mut only = true;
        for &r in &s.refs {
            let rf = &m.references[r];
            let Some(u) = root.descendant_for_byte_range(rf.start, rf.end) else { only = false; break };
            // the argument is `x` or `x || e`
            let arg = match u.parent() {
                Some(b) if b.kind() == "binary_expression" && b.child_by_field_name("left").is_some_and(|l| l.id() == u.id()) => b,
                _ => u,
            };
            let call = arg.parent().filter(|a| a.kind() == "arguments").and_then(|a| a.parent()).filter(|c| c.kind() == "call_expression");
            let Some(call) = call else { only = false; break };
            if arguments(call).get(i).is_none_or(|a| a.id() != arg.id()) {
                only = false;
                break;
            }
            let Some(f) = call.child_by_field_name("function") else { only = false; break };
            let recursive = if method {
                f.kind() == "member_expression" && f.child_by_field_name("object").is_some_and(|o| o.kind() == "this") && f.child_by_field_name("property").is_some_and(|pn| text(pn, src) == fname)
            } else {
                f.kind() == "identifier" && text(f, src) == fname && m.symbol_at(f.start_byte()) == fsym
            };
            if !recursive || call.start_byte() < n.start_byte() || call.end_byte() > n.end_byte() {
                only = false;
                break;
            }
            uses.push(u);
        }
        if !only {
            continue;
        }
        let pname = text(id, src);
        let mut edits = vec![TextEdit::new(id.start_byte(), id.start_byte(), "_")];
        edits.extend(uses.iter().map(|u| TextEdit::new(u.start_byte(), u.start_byte(), "_")));
        out.push(finding(id, format!("`{pname}` is only passed on in recursive calls and does nothing"), as_suggestion(edits)));
    }
});

// --------------------------------------------------------------------- JSON

fn top_object(doc: Node<'_>) -> Option<Node<'_>> {
    named_children(doc).into_iter().find(|c| c.kind() == "object")
}

const DEP_GROUPS: &[&str] = &["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"];

pub struct NoDuplicateDependencies;
impl NativeRule for NoDuplicateDependencies {
    fn id(&self) -> &'static str {
        "json/no-duplicate-dependencies"
    }
    fn languages(&self) -> &'static [Language] {
        JSON
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["document"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let Some(top) = top_object(node.get_inner_node()) else { return };
        let top_pairs = pairs(top, src);
        let mut seen: Vec<(String, &str)> = Vec::new();
        for group in DEP_GROUPS {
            let Some((_, _, Some(v))) = top_pairs.iter().find(|(k, _, _)| k == group) else { continue };
            if v.kind() != "object" {
                continue;
            }
            for (name, key, _) in pairs(*v, src) {
                // a peer dependency that is also a dev dependency is the usual way to test against it
                let earlier = seen.iter().find(|(n, g)| *n == name && !((*g == "devDependencies" && *group == "peerDependencies") || (*g == "peerDependencies" && *group == "devDependencies")));
                if let Some((_, g)) = earlier {
                    out.push(finding(key, format!("`{name}` is already in `{g}`; list it once"), Vec::new()));
                }
                seen.push((name, group));
            }
        }
    }
}

pub struct UseRequiredScripts;
impl NativeRule for UseRequiredScripts {
    fn id(&self) -> &'static str {
        "json/use-required-scripts"
    }
    fn languages(&self) -> &'static [Language] {
        JSON
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["document"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let wanted: Vec<String> = ctx.options.get("scripts").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect()).unwrap_or_default();
        if wanted.is_empty() {
            return;
        }
        let Some(top) = top_object(node.get_inner_node()) else { return };
        let top_pairs = pairs(top, src);
        let (anchor, have): (Node<'_>, Vec<String>) = match top_pairs.iter().find(|(k, _, _)| k == "scripts") {
            Some((_, key, Some(v))) if v.kind() == "object" => (*key, pairs(*v, src).into_iter().map(|(k, _, _)| k).collect()),
            Some((_, key, _)) => (*key, Vec::new()),
            None => (top, Vec::new()),
        };
        let missing: Vec<String> = wanted.iter().filter(|w| !have.contains(w)).map(|m| format!("`{m}`")).collect();
        if missing.is_empty() {
            return;
        }
        let end = if anchor.kind() == "object" { anchor.start_byte() + 1 } else { anchor.end_byte() };
        out.push(NativeFinding { start: anchor.start_byte(), end, message: format!("`scripts` has no {}", missing.join(", ")), edits: Vec::new() });
    }
}

pub struct NoJsonUnsafeValues;
impl NativeRule for NoJsonUnsafeValues {
    fn id(&self) -> &'static str {
        "json/no-json-unsafe-values"
    }
    fn languages(&self) -> &'static [Language] {
        JSON
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["number", "string"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let n = node.get_inner_node();
        let t = text(n, src);
        if n.kind() == "string" {
            if has_lone_surrogate(t) {
                out.push(finding(n, "This string holds a lone surrogate, which is not valid Unicode and is not kept by every parser", Vec::new()));
            }
            return;
        }
        let digits = t.trim_start_matches('-');
        let integer = digits.chars().all(|c| c.is_ascii_digit());
        let value: f64 = t.parse().unwrap_or(0.0);
        let mantissa_zero = digits.split(['e', 'E']).next().is_some_and(|m| m.chars().all(|c| c == '0' || c == '.'));
        let msg = if value.is_infinite() {
            Some("This number is too large and reads as Infinity")
        } else if value == 0.0 && t.starts_with('-') {
            Some("`-0` is read as `0` by some parsers and as negative zero by others")
        } else if value == 0.0 && !mantissa_zero {
            Some("This number is too small and reads as 0")
        } else if integer && digits.parse::<u128>().map_or(digits.len() > 16, |v| v > 9_007_199_254_740_991) {
            Some("This integer is outside the range a double holds exactly (±9007199254740991) and loses precision")
        } else {
            None
        };
        if let Some(m) = msg {
            out.push(finding(n, m, Vec::new()));
        }
    }
}

/// A `\uD800`–`\uDFFF` escape that is not half of a pair.
fn has_lone_surrogate(t: &str) -> bool {
    let b = t.as_bytes();
    let hex = |i: usize| -> Option<u32> {
        if i + 6 <= b.len() && b[i] == b'\\' && b[i + 1] == b'u' {
            u32::from_str_radix(&t[i + 2..i + 6], 16).ok()
        } else {
            None
        }
    };
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            if let Some(v) = hex(i) {
                if (0xD800..0xDC00).contains(&v) {
                    if hex(i + 6).is_some_and(|l| (0xDC00..0xE000).contains(&l)) {
                        i += 12;
                        continue;
                    }
                    return true;
                }
                if (0xDC00..0xE000).contains(&v) {
                    return true;
                }
                i += 6;
                continue;
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    false
}


// ---------------------------------------------------------------------- CSS

const CSS: &[Language] = &[Language::Css];

fn number_like(s: &str) -> bool {
    let s = s.strip_prefix(['+', '-']).unwrap_or(s);
    let (mant, exp) = match s.split_once(['e', 'E']) {
        Some((m, e)) => (m, Some(e)),
        None => (s, None),
    };
    let ok = |x: &str, allow_dot: bool| {
        let mut dot = false;
        let mut digit = false;
        for c in x.chars() {
            match c {
                '0'..='9' => digit = true,
                '.' if allow_dot && !dot => dot = true,
                _ => return false,
            }
        }
        digit
    };
    ok(mant, true) && exp.is_none_or(|e| ok(e.strip_prefix(['+', '-']).unwrap_or(e), false))
}

fn with_unit(v: &str, units: &[&str]) -> bool {
    let v = v.to_ascii_lowercase();
    units.iter().any(|u| v.strip_suffix(u).is_some_and(number_like))
}

const LENGTH: &[&str] = &["px", "em", "rem", "ex", "ch", "cap", "ic", "lh", "rlh", "vh", "vw", "vmin", "vmax", "vi", "vb", "svh", "svw", "lvh", "lvw", "dvh", "dvw", "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax", "cm", "mm", "q", "in", "pt", "pc"];

fn is_function(v: &str, names: &[&str]) -> bool {
    let l = v.to_ascii_lowercase();
    v.ends_with(')') && names.iter().any(|n| l.starts_with(&format!("{n}(")))
}

/// Whether `value` is a value of the single `<type>` or keyword `component`
/// of a registered property's `syntax`. A type this does not know accepts.
fn matches_component(component: &str, value: &str) -> bool {
    let value = value.trim();
    let ident = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') && !s.starts_with(|c: char| c.is_ascii_digit());
    match component {
        "<number>" => number_like(value) || is_function(value, &["calc", "min", "max", "clamp"]),
        "<integer>" => value.strip_prefix(['+', '-']).unwrap_or(value).chars().all(|c| c.is_ascii_digit()) && !value.is_empty() && value != "+" && value != "-",
        "<length>" => value == "0" || with_unit(value, LENGTH) || is_function(value, &["calc", "min", "max", "clamp"]),
        "<percentage>" => value.strip_suffix('%').is_some_and(number_like) || is_function(value, &["calc"]),
        "<length-percentage>" => value == "0" || with_unit(value, LENGTH) || value.strip_suffix('%').is_some_and(number_like) || is_function(value, &["calc", "min", "max", "clamp"]),
        "<angle>" => value == "0" || with_unit(value, &["deg", "grad", "rad", "turn"]) || is_function(value, &["calc"]),
        "<time>" => with_unit(value, &["s", "ms"]) || is_function(value, &["calc"]),
        "<resolution>" => with_unit(value, &["dpi", "dpcm", "dppx", "x"]),
        "<color>" => {
            let hex = value.strip_prefix('#').is_some_and(|h| matches!(h.len(), 3 | 4 | 6 | 8) && h.chars().all(|c| c.is_ascii_hexdigit()));
            hex || is_function(value, &["rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "oklab", "oklch", "color", "color-mix", "light-dark"]) || (ident(value) && !value.starts_with("--"))
        }
        "<string>" => (value.starts_with('"') && value.ends_with('"') && value.len() >= 2) || (value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2),
        "<url>" => is_function(value, &["url"]),
        "<custom-ident>" => ident(value),
        c if c.starts_with('<') && c.ends_with('>') => true,
        keyword => value == keyword,
    }
}

fn syntax_accepts(syntax: &str, value: &str) -> bool {
    let syntax = syntax.trim();
    if syntax == "*" || value.starts_with("var(") {
        return true;
    }
    syntax.split('|').any(|alt| {
        let alt = alt.trim();
        if let Some(base) = alt.strip_suffix('+') {
            return value.split_whitespace().all(|v| matches_component(base, v)) && !value.is_empty();
        }
        if let Some(base) = alt.strip_suffix('#') {
            return value.split(',').all(|v| matches_component(base, v)) && !value.is_empty();
        }
        matches_component(alt, value)
    })
}

native!(NoInvalidPropertyInitValue, "css/no-invalid-property-init-value", CSS, &["at_rule"], |n, ctx, out| {
    let src = ctx.source;
    if !children(n).iter().any(|c| c.kind() == "at_keyword" && text(*c, src).eq_ignore_ascii_case("@property")) {
        return;
    }
    let Some(block) = children(n).into_iter().find(|c| c.kind() == "block") else { return };
    let (mut syntax, mut init) = (None, None);
    for d in named_children(block).into_iter().filter(|d| d.kind() == "declaration") {
        let Some(name) = children(d).into_iter().find(|c| c.kind() == "property_name") else { continue };
        let value = text(d, src).split_once(':').map(|(_, v)| v.trim().trim_end_matches(';').trim().to_owned()).unwrap_or_default();
        match text(name, src) {
            "syntax" => syntax = Some(value.trim_matches(['"', '\'']).to_owned()),
            "initial-value" => init = Some((d, value)),
            _ => {}
        }
    }
    if let (Some(syntax), Some((decl, value))) = (syntax, init) {
        if !syntax_accepts(&syntax, &value) {
            out.push(finding(decl, format!("`initial-value: {value}` does not match the registered `syntax: \"{syntax}\"`"), Vec::new()));
        }
    }
});

// ----------------------------------------------------------------- secrets

fn secret_patterns() -> &'static [(&'static str, regex::Regex)] {
    static P: std::sync::LazyLock<Vec<(&'static str, regex::Regex)>> = std::sync::LazyLock::new(|| {
        let table: &[(&str, &str)] = &[
            ("a JSON Web Token", r"eyJ[A-Za-z0-9_-]{8,}\.eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]+"),
            ("a Slack token", r"xox[baprs]-[A-Za-z0-9-]{10,}"),
            ("a Slack webhook URL", r"https://hooks\.slack\.com/services/T[A-Z0-9]+/B[A-Z0-9]+/[A-Za-z0-9]+"),
            ("a GitHub token", r"gh[pousr]_[A-Za-z0-9]{36,}"),
            ("an AWS access key", r"\bAKIA[0-9A-Z]{16}\b"),
            ("a Twilio key", r"\bSK[0-9a-fA-F]{32}\b"),
            ("a password in a URL", r"[a-z][a-z0-9+.-]*://[^\s/:@]+:[^\s/@]+@"),
            ("a private key", r"-----BEGIN (RSA |OPENSSH |DSA |EC |PGP )?PRIVATE KEY"),
            ("a Google service account", r#""type"\s*:\s*"service_account""#),
        ];
        table.iter().filter_map(|(n, r)| regex::Regex::new(r).ok().map(|re| (*n, re))).collect()
    });
    &P
}

/// Shannon entropy of the characters of `s`, in bits per character.
fn entropy(s: &str) -> f64 {
    let mut counts = std::collections::BTreeMap::new();
    let mut n = 0usize;
    for c in s.chars() {
        *counts.entry(c).or_insert(0usize) += 1;
        n += 1;
    }
    counts.values().map(|&c| { let p = c as f64 / n as f64; -p * p.log2() }).sum()
}

native!(NoSecrets, "js/no-secrets", JS, &["string", "template_string"], |n, ctx, out| {
    let src = ctx.source;
    if n.kind() == "template_string" && named_children(n).iter().any(|c| c.kind() == "template_substitution") {
        return;
    }
    if n.parent().is_some_and(|p| matches!(p.kind(), "import_statement" | "export_statement")) {
        return;
    }
    let raw = text(n, src);
    let inner = if raw.len() >= 2 { &raw[1..raw.len() - 1] } else { raw };
    for (name, re) in secret_patterns() {
        if re.is_match(inner) {
            out.push(finding(n, format!("This looks like {name}; keep secrets out of the source"), Vec::new()));
            return;
        }
    }
    let threshold = ctx.options.get("entropyThreshold").and_then(|v| v.as_f64()).unwrap_or(4.1);
    let min_length = ctx.options.get("minLength").and_then(|v| v.as_u64()).unwrap_or(24) as usize;
    let token = !inner.is_empty() && !inner.contains(char::is_whitespace) && !inner.contains("://") && !inner.starts_with(['.', '/']);
    if token && inner.chars().count() >= min_length && entropy(inner) > threshold {
        out.push(finding(n, "This string is random enough to be a secret; keep secrets out of the source", Vec::new()));
    }
});

pub static RULES: &[&dyn NativeRule] = &[&NoSecrets, &NoInvalidPropertyInitValue, &NoProcessGlobal, &NoShoutyConstants, &NoExportedImports, &NoRedundantDefaultExport, &NoParametersOnlyUsedInRecursion, &NoDuplicateDependencies, &UseRequiredScripts, &NoJsonUnsafeValues];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DetachedFile, NativeContext};
    use std::collections::BTreeMap;

    fn run(rule: &dyn NativeRule, language: Language, src: &str) -> Vec<NativeFinding> {
        let root = codelaw_lang::parse(language, src);
        let model = codelaw_semantic::build(language, &root, src);
        let options = BTreeMap::new();
        let data = BTreeMap::new();
        let file = DetachedFile::new("example", src);
        let ctx = NativeContext { source: src, language, semantic: model.as_ref(), options: &options, data: &data, file: &file };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            if rule.kinds().contains(&node.kind().as_ref()) {
                rule.check(node, &ctx, &mut out);
            }
        }
        out
    }

    #[test]
    fn process_global_is_reported_and_import_is_not() {
        assert_eq!(run(&NoProcessGlobal, Language::Javascript, "const a = process.env.A;\n").len(), 1);
        assert!(run(&NoProcessGlobal, Language::Javascript, "import process from 'node:process';\nconst a = process.env.A;\n").is_empty());
    }

    #[test]
    fn shouty_constant_with_one_use() {
        assert_eq!(run(&NoShoutyConstants, Language::Javascript, "const FOO = 'FOO';\nuse(FOO);\n").len(), 1);
        assert!(run(&NoShoutyConstants, Language::Javascript, "const FOO = 'FOO';\nuse(FOO);\nuse(FOO);\n").is_empty());
    }

    #[test]
    fn recursion_only_parameter() {
        let src = "function f(n, acc) {\n  if (n === 0) return 0;\n  return f(n - 1, acc);\n}\n";
        let found = run(&NoParametersOnlyUsedInRecursion, Language::Javascript, src);
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("`acc`"));
    }

    #[test]
    fn property_syntax() {
        assert!(syntax_accepts("<length>", "10px"));
        assert!(syntax_accepts("<length>", "0"));
        assert!(!syntax_accepts("<length>", "red"));
        assert!(syntax_accepts("<color> | auto", "auto"));
        assert!(!syntax_accepts("<number>", "10px"));
        assert!(syntax_accepts("<length>+", "1px 2px"));
        assert!(syntax_accepts("*", "anything at all"));
    }

    #[test]
    fn secrets_by_format_and_entropy() {
        assert_eq!(run(&NoSecrets, Language::Javascript, "const k = 'AKIAIOSFODNN7EXAMPLE';\n").len(), 1);
        assert_eq!(run(&NoSecrets, Language::Javascript, "const k = 'wJalrXUtnFEMI7K7MDENGbPxRfiCYzq9';\n").len(), 1);
        assert!(run(&NoSecrets, Language::Javascript, "const k = 'a perfectly ordinary sentence of words';\nimport x from 'some-package-with-a-long-name';\n").is_empty());
    }

    #[test]
    fn json_numbers() {
        assert_eq!(run(&NoJsonUnsafeValues, Language::Json, "[1e999, 12345678901234567890, 1, -0]").len(), 3);
    }
}

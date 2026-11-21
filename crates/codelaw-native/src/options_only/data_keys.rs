//! `data/key-name-casing` (W-178): the keys of a JSON, YAML or TOML file are
//! written in one casing style. Silent until a project sets `case`.

use super::{TextMatcher, finding, opt_strings, option_problem};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const DATA: &[Language] = &[Language::Json, Language::Yaml, Language::Toml];

/// The casing styles `case` accepts.
const CASES: &[&str] = &["camelCase", "PascalCase", "snake_case", "kebab-case", "SCREAMING_SNAKE_CASE"];

/// A key's words: split at `_`, `-` and at a lower-to-upper step or an
/// acronym's end (`userID` → `user`, `id`), lowercased. `None` for a key that
/// is not made of words (it holds a character outside ASCII letters, digits,
/// `_` and `-`, or it has no letter): `$schema`, `@type`, `.github`, `a b`.
fn words(key: &str) -> Option<Vec<String>> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') || !key.chars().any(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut out: Vec<String> = Vec::new();
    for chunk in key.split(['_', '-']).filter(|c| !c.is_empty()) {
        let chars: Vec<char> = chunk.chars().collect();
        let mut cur = String::new();
        for (i, &c) in chars.iter().enumerate() {
            let prev_lower = i > 0 && (chars[i - 1].is_ascii_lowercase() || chars[i - 1].is_ascii_digit());
            let acronym_end = i > 0 && c.is_ascii_uppercase() && chars[i - 1].is_ascii_uppercase() && chars.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            if c.is_ascii_uppercase() && (prev_lower || acronym_end) && !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.push(c.to_ascii_lowercase());
        }
        if !cur.is_empty() {
            out.push(cur);
        }
    }
    Some(out)
}

fn capital(w: &str) -> String {
    let mut c = w.chars();
    c.next().map(|f| f.to_ascii_uppercase().to_string() + c.as_str()).unwrap_or_default()
}

/// `key` written in `case`.
fn render(words: &[String], case: &str) -> String {
    match case {
        "camelCase" => words.iter().enumerate().map(|(i, w)| if i == 0 { w.clone() } else { capital(w) }).collect(),
        "PascalCase" => words.iter().map(|w| capital(w)).collect(),
        "snake_case" => words.join("_"),
        "kebab-case" => words.join("-"),
        _ => words.join("_").to_ascii_uppercase(),
    }
}

/// One key: `name` as written, `start..end` the bytes an edit replaces.
fn check(name: &str, start: usize, end: usize, case: &str, ignore: &[TextMatcher], out: &mut Vec<NativeFinding>) {
    if ignore.iter().any(|m| m.is_match(name)) {
        return;
    }
    let Some(w) = words(name) else { return };
    let want = render(&w, case);
    if want != name {
        out.push(finding(start, end, format!("Key `{name}` is not {case}: `{want}`"), vec![TextEdit::new(start, end, want)]));
    }
}

/// The inside of a quoted token (`"a"`, `'a'`), or the token itself.
fn inner(n: Node<'_>, src: &str) -> (usize, usize) {
    let t = text(n, src);
    if t.len() >= 2 && (t.starts_with('"') && t.ends_with('"') || t.starts_with('\'') && t.ends_with('\'')) {
        (n.start_byte() + 1, n.end_byte() - 1)
    } else {
        (n.start_byte(), n.end_byte())
    }
}

fn toml_parts<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    match n.kind() {
        "dotted_key" => named_children(n).into_iter().for_each(|c| toml_parts(c, out)),
        "bare_key" | "quoted_key" => out.push(n),
        _ => {}
    }
}

pub struct KeyNameCasing;

impl NativeRule for KeyNameCasing {
    fn id(&self) -> &'static str {
        "data/key-name-casing"
    }
    fn languages(&self) -> &'static [Language] {
        DATA
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["pair", "block_mapping_pair", "flow_pair", "table", "table_array_element"]
    }
    fn needs_options(&self) -> &'static [&'static str] {
        &["case"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        let Some(case) = ctx.options.get("case").and_then(|v| v.as_str()) else { return };
        if !CASES.contains(&case) {
            option_problem(n, self.kinds(), out, format!("`case` is `{case}`, which is not one of {}", CASES.join(", ")));
            return;
        }
        let mut ignore = Vec::new();
        for s in opt_strings(ctx, "ignore") {
            match TextMatcher::parse(&s) {
                Ok(m) => ignore.push(m),
                Err(e) => {
                    option_problem(n, self.kinds(), out, format!("`ignore` has a pattern that does not compile: {e}"));
                    return;
                }
            }
        }
        match (ctx.language, n.kind()) {
            (Language::Json, "pair") => {
                if let Some(k) = n.child_by_field_name("key").filter(|k| k.kind() == "string") {
                    let (s, e) = inner(k, src);
                    check(&src[s..e], s, e, case, &ignore, out);
                }
            }
            (Language::Yaml, "block_mapping_pair" | "flow_pair") => {
                let Some(k) = n.child_by_field_name("key") else { return };
                let scalar = if k.kind() == "flow_node" { named_children(k).into_iter().find(|c| c.kind() != "comment") } else { Some(k) };
                let Some(scalar) = scalar else { return };
                if named_children(k).iter().any(|c| matches!(c.kind(), "anchor" | "tag")) || !matches!(scalar.kind(), "plain_scalar" | "single_quote_scalar" | "double_quote_scalar") {
                    return;
                }
                let (s, e) = inner(scalar, src);
                let raw = &src[s..e];
                // Only a key with no escape to decode is renamed in place.
                if raw.contains('\\') || (scalar.kind() == "single_quote_scalar" && raw.contains("''")) {
                    return;
                }
                check(raw.trim(), s, s + raw.trim_end().len(), case, &ignore, out);
            }
            (Language::Toml, "pair" | "table" | "table_array_element") => {
                let Some(key) = crate::data_toml::key_node(n) else { return };
                let mut parts = Vec::new();
                toml_parts(key, &mut parts);
                for p in parts {
                    let (s, e) = inner(p, src);
                    let raw = &src[s..e];
                    if raw.contains('\\') {
                        continue;
                    }
                    check(raw, s, e, case, &ignore, out);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(k: &str, case: &str) -> Option<String> {
        words(k).map(|w| render(&w, case))
    }

    #[test]
    fn words_split_and_render() {
        assert_eq!(w("user_name", "camelCase").as_deref(), Some("userName"));
        assert_eq!(w("userName", "snake_case").as_deref(), Some("user_name"));
        assert_eq!(w("userID", "snake_case").as_deref(), Some("user_id"));
        assert_eq!(w("HTTPServer", "kebab-case").as_deref(), Some("http-server"));
        assert_eq!(w("base64Data", "snake_case").as_deref(), Some("base64_data"));
        assert_eq!(w("name", "PascalCase").as_deref(), Some("Name"));
        assert_eq!(w("max-retries", "SCREAMING_SNAKE_CASE").as_deref(), Some("MAX_RETRIES"));
        assert_eq!(w("$schema", "camelCase"), None);
        assert_eq!(w("a b", "camelCase"), None);
        assert_eq!(w("2024", "camelCase"), None);
    }
}

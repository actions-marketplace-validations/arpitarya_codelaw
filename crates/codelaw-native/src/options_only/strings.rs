//! `js/string-content`: the text a project does not want in a string.
//! eslint-plugin-unicorn 79.0.0's rule for JavaScript, read and
//! reimplemented: `patterns` maps a regular expression to the text that
//! should replace what it matches.
//!
//! A string is read as written: a match that overlaps an escape sequence
//! (`\n`, `\u2019`) is not reported, as upstream does for template text; its
//! `selectors` option, which names ESTree nodes, has no equivalent here.

use super::{finding, option_problem, regex_of};
use crate::util::{children, text};
use crate::{NativeFinding, as_suggestion};
use codelaw_core::TextEdit;
use regex::Regex;
use serde_json::Value;
use std::sync::Arc;
use tree_sitter::Node;

struct Replacement {
    matched: String,
    re: Arc<Regex>,
    suggest: String,
    fix: bool,
    message: Option<String>,
}

fn replacements(patterns: &serde_json::Map<String, Value>) -> Result<Vec<Replacement>, String> {
    let mut out = Vec::new();
    for (m, v) in patterns {
        let (suggest, fix, case_sensitive, message) = match v {
            Value::String(s) => (s.clone(), true, true, None),
            Value::Object(o) => (
                o.get("suggest").and_then(Value::as_str).ok_or_else(|| format!("`patterns` entry `{m}` needs a `suggest`"))?.to_owned(),
                o.get("fix").and_then(Value::as_bool).unwrap_or(true),
                o.get("caseSensitive").and_then(Value::as_bool).unwrap_or(true),
                o.get("message").and_then(Value::as_str).map(str::to_owned),
            ),
            _ => return Err(format!("`patterns` entry `{m}` is a replacement string or an object with `suggest`")),
        };
        let source = if case_sensitive { m.clone() } else { format!("(?i){m}") };
        let re = regex_of(&source).map_err(|e| format!("`patterns` entry `{m}` is not a valid regular expression: {e}"))?;
        out.push(Replacement { matched: m.clone(), re, suggest, fix, message });
    }
    Ok(out)
}

/// The byte ranges of escape sequences in raw string text.
fn escape_ranges(raw: &str) -> Vec<(usize, usize)> {
    let b = raw.as_bytes();
    let mut v = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            let rest = &raw[i + 1..];
            let len = if let Some(r) = rest.strip_prefix("u{") {
                r.find('}').map_or(2, |k| 2 + k + 1)
            } else if rest.starts_with('u') && rest.len() >= 5 && rest[1..5].chars().all(|c| c.is_ascii_hexdigit()) {
                5
            } else if (rest.starts_with('x') && rest.len() >= 3 && rest[1..3].chars().all(|c| c.is_ascii_hexdigit())) || rest.starts_with("\r\n") {
                3
            } else {
                1 + rest.chars().next().map_or(0, char::len_utf8)
            };
            v.push((i, (i + len).min(b.len())));
            i += len.max(1);
        } else {
            i += 1;
        }
    }
    v
}

/// What kind of literal the text sits in, which decides how a replacement is written into it.
#[derive(Clone, Copy, PartialEq)]
enum Host {
    Quoted(char),
    JsxAttribute(char),
    Template,
}

fn escape_into(s: &str, host: Host) -> String {
    match host {
        Host::Quoted(q) => {
            let mut o = String::new();
            for c in s.chars() {
                match c {
                    '\\' => o.push_str("\\\\"),
                    '\n' => o.push_str("\\n"),
                    '\r' => o.push_str("\\r"),
                    c if c == q => {
                        o.push('\\');
                        o.push(c);
                    }
                    c => o.push(c),
                }
            }
            o
        }
        Host::JsxAttribute(q) => s.replace(q, if q == '"' { "&quot;" } else { "&#39;" }),
        Host::Template => s.replace('\\', "\\\\").replace('`', "\\`").replace("${", "\\${").replace('\r', "\\r"),
    }
}

/// Check the raw text `raw` that starts at byte `base`.
fn check_text(raw: &str, base: usize, host: Host, rs: &[Replacement], out: &mut Vec<NativeFinding>, whole: (usize, usize)) {
    if raw.is_empty() {
        return;
    }
    let escapes = escape_ranges(raw);
    for r in rs {
        let hits: Vec<(usize, usize)> = r.re.find_iter(raw).map(|m| (m.start(), m.end())).filter(|(s, e)| escapes.iter().all(|(es, ee)| *s >= *ee || *e <= *es)).collect();
        if hits.is_empty() {
            continue;
        }
        let message = r.message.as_deref().unwrap_or("Prefer `{{suggest}}` over `{{match}}`.").replace("{{match}}", &r.matched).replace("{{suggest}}", &r.suggest);
        let escaped = escape_into(&r.suggest, host);
        let mut fixed = String::new();
        let mut at_byte = 0;
        for (s, e) in hits {
            fixed.push_str(&raw[at_byte..s]);
            fixed.push_str(&escaped);
            at_byte = e;
        }
        fixed.push_str(&raw[at_byte..]);
        let edits = vec![TextEdit::new(base, base + raw.len(), fixed)];
        out.push(finding(whole.0, whole.1, message, if r.fix { edits } else { as_suggestion(edits) }));
        // The first pattern that matches decides.
        return;
    }
}

/// `gql`, `html`, `sql`, `styled` and `svg` templates hold another language.
fn ignored_tag(template: Node<'_>, src: &str) -> bool {
    let Some(p) = template.parent().filter(|p| p.kind() == "call_expression") else { return false };
    if p.child_by_field_name("arguments").is_none_or(|a| a.id() != template.id()) {
        return false;
    }
    let mut root = p.child_by_field_name("function");
    while let Some(r) = root {
        match r.kind() {
            "call_expression" => root = r.child_by_field_name("function"),
            "member_expression" => root = r.child_by_field_name("object"),
            "identifier" => return matches!(text(r, src), "gql" | "html" | "sql" | "styled" | "svg"),
            _ => return false,
        }
    }
    false
}

super::orule!(StringContent, "js/string-content", &["patterns"], &["string", "template_string"], false, |n, ctx, out| {
    let Some(Value::Object(patterns)) = ctx.options.get("patterns") else { return };
    let rs = match replacements(patterns) {
        Ok(r) => r,
        Err(e) => return option_problem(n, &["string", "template_string"], out, e),
    };
    let src = ctx.source;
    let whole = (n.start_byte(), n.end_byte());
    if n.kind() == "string" {
        let t = text(n, src);
        let Some(q) = t.chars().next().filter(|c| *c == '"' || *c == '\'') else { return };
        if t.len() < 2 || !t.ends_with(q) {
            return;
        }
        let host = if n.parent().is_some_and(|p| p.kind() == "jsx_attribute") { Host::JsxAttribute(q) } else { Host::Quoted(q) };
        check_text(&t[1..t.len() - 1], n.start_byte() + 1, host, &rs, out, whole);
        return;
    }
    if ignored_tag(n, src) {
        return;
    }
    // Each stretch of text between the backticks and the `${…}` is one element.
    let mut from = n.start_byte() + 1;
    let end = n.end_byte().saturating_sub(1);
    for c in children(n) {
        if c.kind() == "template_substitution" {
            if c.start_byte() > from {
                check_text(&src[from..c.start_byte()], from, Host::Template, &rs, out, (from, c.start_byte()));
            }
            from = c.end_byte();
        }
    }
    if end > from {
        check_text(&src[from..end], from, Host::Template, &rs, out, (from, end));
    }
});

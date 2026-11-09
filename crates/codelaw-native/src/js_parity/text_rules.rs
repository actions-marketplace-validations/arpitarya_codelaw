//! W-139 strand W-139-js, rows that read the text of a template or a
//! comment: eslint-plugin-unicorn v79.0.0's `template-indent`.

use super::{JS, finding};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::BTreeMap;
use tree_sitter::Node;

fn strings(o: &BTreeMap<String, Value>, key: &str, default: &[&str]) -> Vec<String> {
    match o.get(key).and_then(Value::as_array) {
        Some(a) => a.iter().filter_map(Value::as_str).map(str::to_owned).collect(),
        None => default.iter().map(|s| (*s).to_owned()).collect(),
    }
}

/// The indentation unit a piece of text uses: the most common step between
/// one indented line and the next, in tabs or in spaces; empty when the text
/// shows none.
fn detect_indent(lines: &[&str]) -> String {
    let mut steps: Vec<(String, usize)> = Vec::new();
    let mut prev = "";
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let lead = &line[..line.len() - line.trim_start_matches([' ', '\t']).len()];
        let same = lead.chars().all(|c| c == ' ') && prev.chars().all(|c| c == ' ') || lead.chars().all(|c| c == '\t') && prev.chars().all(|c| c == '\t');
        if same && lead.len() > prev.len() {
            let unit = lead[prev.len()..].to_owned();
            match steps.iter_mut().find(|(u, _)| *u == unit) {
                Some(s) => s.1 += 1,
                None => steps.push((unit, 1)),
            }
        } else if lead.len() > prev.len() && prev.is_empty() {
            let unit = lead.to_owned();
            match steps.iter_mut().find(|(u, _)| *u == unit) {
                Some(s) => s.1 += 1,
                None => steps.push((unit, 1)),
            }
        }
        prev = lead;
    }
    steps.into_iter().max_by_key(|(_, n)| *n).map(|(u, _)| u).unwrap_or_default()
}

/// The text with its common leading whitespace removed (blank lines ignored
/// when finding it).
fn strip_indent(s: &str) -> String {
    let min = s.split('\n').filter(|l| !l.trim().is_empty()).map(|l| l.len() - l.trim_start_matches([' ', '\t']).len()).min().unwrap_or(0);
    if min == 0 {
        return s.to_owned();
    }
    s.split('\n').map(|l| if l.len() >= min && l[..min].chars().all(|c| c == ' ' || c == '\t') { &l[min..] } else { l.trim_start_matches([' ', '\t']) }).collect::<Vec<_>>().join("\n")
}

fn is_tag_match(f: Node<'_>, names: &[String], src: &str) -> bool {
    let t = text(f, src);
    names.iter().any(|n| n == t)
}

fn preceded_by_comment(src: &str, start: usize, comments: &[String]) -> bool {
    let before = src[..start].trim_end();
    let Some(rest) = before.strip_suffix("*/") else { return false };
    let Some(open) = rest.rfind("/*") else { return false };
    let body = rest[open + 2..].trim().to_lowercase();
    !rest[open + 2..].contains("*/") && comments.iter().any(|c| c.to_lowercase() == body)
}

pub struct TemplateIndent;
impl NativeRule for TemplateIndent {
    fn id(&self) -> &'static str {
        "js/template-indent"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["template_string"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        let tags = strings(ctx.options, "tags", &["outdent", "dedent", "gql", "sql", "html", "styled"]);
        let functions = strings(ctx.options, "functions", &["dedent", "stripIndent"]);
        let comments = strings(ctx.options, "comments", &["HTML", "indent"]);
        let Some(par) = n.parent() else { return };
        let selected = preceded_by_comment(src, n.start_byte(), &comments)
            || match par.kind() {
                // A tagged template: the tag is the call's function.
                "call_expression" if par.child_by_field_name("arguments").is_some_and(|a| a.id() == n.id()) => par.child_by_field_name("function").is_some_and(|f| is_tag_match(f, &tags, src)),
                "arguments" => par.parent().is_some_and(|c| {
                    c.kind() == "call_expression"
                        && c.child_by_field_name("function").is_some_and(|f| {
                            is_tag_match(f, &functions, src) || f.kind() == "member_expression" && f.child_by_field_name("property").is_some_and(|p| matches!(text(p, src), "toMatchInlineSnapshot" | "toThrowErrorMatchingInlineSnapshot"))
                        })
                }),
                _ => false,
            };
        if !selected {
            return;
        }
        // The raw text of each part between substitutions.
        let subs: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() == "template_substitution").collect();
        let mut segs: Vec<(usize, usize)> = Vec::new();
        let mut at = n.start_byte() + 1;
        for s in &subs {
            segs.push((at, s.start_byte()));
            at = s.end_byte();
        }
        segs.push((at, n.end_byte() - 1));
        let delim = "\u{1}\u{2}PLACEHOLDER\u{2}\u{1}";
        let joined = segs.iter().map(|(a, b)| &src[*a..*b]).collect::<Vec<_>>().join(delim);
        let Some(nl) = joined.find('\n') else { return };
        let (first, eol) = if nl > 0 && joined.as_bytes()[nl - 1] == b'\r' { (&joined[..nl - 1], "\r\n") } else { (&joined[..nl], "\n") };
        if !first.trim().is_empty() {
            return;
        }
        let line_start = src[..n.start_byte()].rfind('\n').map_or(0, |i| i + 1);
        let line = &src[line_start..];
        let margin = &line[..line.len() - line.trim_start_matches([' ', '\t']).len()];
        let indent = match ctx.options.get("indent") {
            Some(Value::String(s)) if !s.is_empty() => s.clone(),
            Some(Value::Number(k)) if k.as_u64().is_some_and(|k| k > 0) => " ".repeat(k.as_u64().unwrap_or(2) as usize),
            _ => {
                // The file's own indentation, outside every template.
                let mut ignored = vec![false; src.lines().count() + 2];
                mark_template_lines(n, src, &mut ignored);
                let lines: Vec<&str> = src.lines().enumerate().map(|(i, l)| if ignored.get(i + 1).copied().unwrap_or(false) { "" } else { l }).collect();
                let file = detect_indent(&lines);
                if !file.is_empty() {
                    file
                } else if margin.is_empty() {
                    let blank: String = joined.split('\n').map(|l| if l.trim().is_empty() { "" } else { l }).collect::<Vec<_>>().join("\n");
                    let stripped = strip_indent(&blank);
                    let own = detect_indent(&stripped.lines().collect::<Vec<_>>());
                    if own.is_empty() {
                        let own2 = detect_indent(&blank.lines().collect::<Vec<_>>());
                        if own2.is_empty() { "  ".to_owned() } else { own2 }
                    } else {
                        own
                    }
                } else if margin.starts_with('\t') {
                    "\t".to_owned()
                } else {
                    "  ".to_owned()
                }
            }
        };
        let dedented = strip_indent(&joined);
        let mut trimmed = dedented.as_str();
        trimmed = trimmed.strip_prefix(eol).unwrap_or(trimmed);
        if let Some(i) = trimmed.rfind(eol)
            && trimmed[i + eol.len()..].chars().all(|c| c == ' ' || c == '\t')
        {
            trimmed = &trimmed[..i];
        }
        let pad = format!("{margin}{indent}");
        let body = trimmed.split(eol).map(|l| if l.is_empty() { l.to_owned() } else { format!("{pad}{l}") }).collect::<Vec<_>>().join(eol);
        let fixed = format!("{eol}{body}{eol}{margin}");
        if fixed == joined {
            return;
        }
        let parts: Vec<&str> = fixed.split(delim).collect();
        let mut edits = Vec::new();
        if parts.len() == segs.len() {
            for ((a, b), new) in segs.iter().zip(parts) {
                if &src[*a..*b] != new {
                    edits.push(TextEdit::new(*a, *b, new));
                }
            }
        }
        out.push(finding(n, "This template's indentation does not follow the code around it: indent it to match", edits));
    }
}

/// Every line after the first of every template literal in the file, so the
/// file's indentation is judged by code alone.
fn mark_template_lines(n: Node<'_>, src: &str, ignored: &mut [bool]) {
    let mut root = n;
    while let Some(p) = root.parent() {
        root = p;
    }
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        if x.kind() == "template_string" {
            for line in x.start_position().row + 2..=x.end_position().row + 1 {
                if let Some(slot) = ignored.get_mut(line) {
                    *slot = true;
                }
            }
        }
        stack.extend(named_children(x));
    }
    let _ = src;
}

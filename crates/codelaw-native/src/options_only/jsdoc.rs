//! The five eslint-plugin-jsdoc 65.4.2 rules that do nothing until a project
//! says what to check: `match-name`, `no-missing-syntax`,
//! `no-restricted-syntax`, `require-tags` and `text-escaping`, on the JSDoc
//! block parser of `crate::jsdoc` — read from the plugin's source and
//! reimplemented. Unlike upstream, a missing option is not reported.
//!
//! Upstream names the code a block documents, and what a block holds, with
//! esquery selectors over ESTree and the JSDoc AST. Here a `context` is a
//! tree-sitter node kind of the documented code (`function_declaration`,
//! `class_declaration`, `lexical_declaration` …) or `any`; what a block must
//! hold is a `tag` it carries and a `matches` regular expression its text
//! matches.

use super::{finding, opt_bool, opt_strings, option_problem, regex_of};
use crate::jsdoc::block::Block;
use crate::util::children;
use crate::{NativeContext, NativeFinding};
use codelaw_core::TextEdit;
use regex::Regex;
use serde_json::Value;
use std::sync::Arc;
use tree_sitter::Node;

const COMMENT: &[&str] = &["comment"];

/// The node a block documents: the next node, up to one blank line away.
fn documented<'t>(comment: Node<'t>) -> Option<Node<'t>> {
    crate::jsdoc::target::target(comment)
}

/// The kinds of code a block documents, the declaration under an `export` and
/// the function or class a variable holds included.
fn context_kinds(comment: Node<'_>) -> Vec<String> {
    let mut v = Vec::new();
    let Some(t) = documented(comment) else { return v };
    let mut cur = Some(t);
    while let Some(n) = cur {
        v.push(n.kind().to_owned());
        cur = match n.kind() {
            "export_statement" => n.child_by_field_name("declaration").or_else(|| n.child_by_field_name("value")),
            "lexical_declaration" | "variable_declaration" => children(n).into_iter().find(|c| c.kind() == "variable_declarator").and_then(|d| {
                v.push("variable_declarator".to_owned());
                d.child_by_field_name("value")
            }),
            "expression_statement" => n.named_child(0).and_then(|e| if e.kind() == "assignment_expression" { e.child_by_field_name("right") } else { Some(e) }),
            _ => None,
        };
    }
    v
}

fn context_holds(entry_context: Option<&str>, comment: Node<'_>) -> bool {
    match entry_context {
        None | Some("any") => true,
        Some(c) => context_kinds(comment).iter().any(|k| k == c),
    }
}

/// A `contexts` entry: where, and what the block holds.
struct Entry {
    context: Option<String>,
    tag: Option<String>,
    matches: Option<Arc<Regex>>,
    matches_text: Option<String>,
    message: Option<String>,
    minimum: usize,
}

impl Entry {
    fn comment_text(&self) -> String {
        match (&self.tag, &self.matches_text) {
            (Some(t), Some(m)) => format!("@{t} and /{m}/"),
            (Some(t), None) => format!("@{t}"),
            (None, Some(m)) => format!("/{m}/"),
            (None, None) => String::new(),
        }
    }

    fn holds(&self, b: &Block<'_>, comment: Node<'_>) -> bool {
        context_holds(self.context.as_deref(), comment)
            && self.tag.as_deref().is_none_or(|t| b.tags.iter().any(|x| x.tag == t))
            && self.matches.as_ref().is_none_or(|r| r.is_match(&b.src[b.start..b.end]))
    }

    fn render(&self, default_prefix: &str) -> String {
        let context = self.context.as_deref().unwrap_or("any");
        let comment = self.comment_text();
        match &self.message {
            Some(m) => m.replace("{{context}}", context).replace("{{comment}}", &comment),
            None if comment.is_empty() => format!("{default_prefix}: {context}"),
            None => format!("{default_prefix}: {context} with {comment}"),
        }
    }
}

fn entry_of(o: &serde_json::Map<String, Value>, label: &str) -> Result<Entry, String> {
    let matches_text = o.get("matches").and_then(Value::as_str).map(str::to_owned);
    let matches = match &matches_text {
        Some(m) => Some(regex_of(m).map_err(|e| format!("`entries.{label}.matches` is not a valid regular expression: {e}"))?),
        None => None,
    };
    Ok(Entry {
        context: o.get("context").and_then(Value::as_str).map(str::to_owned),
        tag: o.get("tag").and_then(Value::as_str).map(str::to_owned),
        matches,
        matches_text,
        message: o.get("message").and_then(Value::as_str).map(str::to_owned),
        minimum: o.get("minimum").and_then(Value::as_u64).map_or(1, |m| m as usize),
    })
}

/// `contexts` (node kinds) and `entries` (named, with a tag, a text or a message).
fn entries(ctx: &NativeContext<'_>) -> Result<Vec<Entry>, String> {
    let mut out: Vec<Entry> = opt_strings(ctx, "contexts").into_iter().map(|c| Entry { context: Some(c), tag: None, matches: None, matches_text: None, message: None, minimum: 1 }).collect();
    if let Some(Value::Object(named)) = ctx.options.get("entries") {
        for (label, v) in named {
            match v {
                Value::Object(o) => out.push(entry_of(o, label)?),
                _ => return Err(format!("`entries.{label}` is an object with `context`, `tag`, `matches`, `message` or `minimum`")),
            }
        }
    }
    Ok(out)
}

/// A block's first line, for a finding about the whole block.
fn block_span(b: &Block<'_>) -> (usize, usize) {
    (b.start, b.lines.first().map_or(b.end, |l| l.end))
}

// ---------------------------------------------------------------------------
// js/jsdoc-no-restricted-syntax

super::orule!(JsdocNoRestrictedSyntax, "js/jsdoc-no-restricted-syntax", &["contexts", "entries"], COMMENT, false, |n, ctx, out| {
    let Some(b) = Block::parse(ctx.source, n.start_byte(), n.end_byte()) else { return };
    let es = match entries(ctx) {
        Ok(e) => e,
        Err(e) => return option_problem(n, COMMENT, out, e),
    };
    for e in es {
        if e.holds(&b, n) {
            let (s, t) = block_span(&b);
            out.push(finding(s, t, e.render("Syntax is restricted"), Vec::new()));
        }
    }
});

// ---------------------------------------------------------------------------
// js/jsdoc-no-missing-syntax: a requirement of the whole file

pub struct JsdocNoMissingSyntax;

impl crate::NativeRule for JsdocNoMissingSyntax {
    fn id(&self) -> &'static str {
        "js/jsdoc-no-missing-syntax"
    }
    fn languages(&self) -> &'static [codelaw_core::Language] {
        super::JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn needs_options(&self) -> &'static [&'static str] {
        &["contexts", "entries"]
    }
    fn check(&self, node: codelaw_lang::SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let root = node.get_inner_node();
        let es = match entries(ctx) {
            Ok(e) => e,
            Err(e) => return option_problem(root, &["program"], out, e),
        };
        let mut blocks: Vec<(Block<'_>, Node<'_>)> = Vec::new();
        let mut stack = vec![root];
        while let Some(n) = stack.pop() {
            if n.kind() == "comment" {
                if let Some(b) = Block::parse(ctx.source, n.start_byte(), n.end_byte()) {
                    blocks.push((b, n));
                }
            } else {
                stack.extend(children(n));
            }
        }
        // The first requirement that is not met is the one reported.
        for e in es {
            let found = blocks.iter().filter(|(b, n)| e.holds(b, *n)).count();
            if found < e.minimum {
                out.push(finding(0, 0, e.render("Syntax is required"), Vec::new()));
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// js/jsdoc-require-tags

/// `tags` (any block) and `tagContexts` (a tag and the kind of code whose block needs it).
fn required_tags(ctx: &NativeContext<'_>) -> Vec<(String, Option<String>)> {
    let mut out: Vec<(String, Option<String>)> = opt_strings(ctx, "tags").into_iter().map(|t| (t, None)).collect();
    if let Some(Value::Object(m)) = ctx.options.get("tagContexts") {
        out.extend(m.iter().filter_map(|(t, c)| c.as_str().map(|c| (t.clone(), Some(c.to_owned())))));
    }
    out
}

super::orule!(JsdocRequireTags, "js/jsdoc-require-tags", &["tags", "tagContexts"], COMMENT, false, |n, ctx, out| {
    let Some(b) = Block::parse(ctx.source, n.start_byte(), n.end_byte()) else { return };
    for (tag, context) in required_tags(ctx) {
        if context_holds(context.as_deref(), n) && !b.tags.iter().any(|t| t.tag == tag) {
            let (s, e) = block_span(&b);
            out.push(finding(s, e, format!("Missing required tag \"{tag}\""), Vec::new()));
        }
    }
});

// ---------------------------------------------------------------------------
// js/jsdoc-match-name

/// A name pattern as the plugin reads one: `/body/flags`, or the regular expression itself.
fn name_regex(s: &str) -> Result<Arc<Regex>, String> {
    if let Some(rest) = s.strip_prefix('/')
        && let Some(close) = rest.rfind('/')
        && rest[close + 1..].chars().all(|c| c.is_ascii_alphabetic())
    {
        let (body, flags) = (&rest[..close], &rest[close + 1..]);
        return regex_of(&if flags.contains('i') { format!("(?i){body}") } else { body.to_owned() });
    }
    regex_of(s)
}

super::orule!(JsdocMatchName, "js/jsdoc-match-name", &["match"], COMMENT, false, |n, ctx, out| {
    let Some(b) = Block::parse(ctx.source, n.start_byte(), n.end_byte()) else { return };
    let Some(Value::Object(items)) = ctx.options.get("match") else { return };
    for (i, item) in items.iter() {
        let Value::Object(o) = item else { return option_problem(n, COMMENT, out, format!("`match.{i}` is an object")) };
        let get = |k: &str| o.get(k).and_then(Value::as_str);
        if !context_holds(get("context"), n) {
            continue;
        }
        let compile = |k: &str| -> Result<Option<Arc<Regex>>, String> {
            match get(k) {
                Some(s) => name_regex(s).map(Some).map_err(|e| format!("`match.{i}.{k}` is not a valid regular expression: {e}")),
                None => Ok(None),
            }
        };
        let (allow, disallow) = match (compile("allowName"), compile("disallowName")) {
            (Ok(a), Ok(d)) => (a, d),
            (Err(e), _) | (_, Err(e)) => return option_problem(n, COMMENT, out, e),
        };
        let wanted: Vec<String> = match o.get("tags") {
            Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect(),
            _ => vec!["*".to_owned()],
        };
        let has_regex = allow.is_some() || disallow.is_some();
        let mut reported = false;
        for t in b.tags.iter().filter(|t| wanted.iter().any(|w| w == "*" || *w == t.tag)) {
            let name = t.name.as_str();
            let allowed = allow.as_ref().is_none_or(|r| r.is_match(name));
            let disallowed = disallow.as_ref().is_some_and(|r| r.is_match(name));
            if has_regex && allowed && !disallowed {
                continue;
            }
            if !has_regex && reported {
                continue;
            }
            let message = match get("message") {
                Some(m) => m.to_owned(),
                None if !has_regex => format!("Prohibited context for \"{name}\"."),
                None if disallowed => format!("Only allowing names not matching `{}` but found \"{name}\".", get("disallowName").unwrap_or_default()),
                None => format!("Only allowing names matching `{}` but found \"{name}\".", get("allowName").unwrap_or_default()),
            };
            // The replacement renames the tag's name where the disallowed pattern matches it.
            let edits = match (&disallow, get("replacement")) {
                (Some(r), Some(rep)) if !name.is_empty() => {
                    let start = t.name_start + usize::from(t.optional);
                    let renamed = r.replace(name, rep).into_owned();
                    vec![TextEdit::new(start, start + name.len(), renamed)]
                }
                _ => Vec::new(),
            };
            if has_regex {
                out.push(finding(t.name_start, t.name_end.max(t.name_start), message, edits));
            } else {
                let (s, e) = block_span(&b);
                out.push(finding(s, e, message, edits));
                reported = true;
            }
        }
    }
});

// ---------------------------------------------------------------------------
// js/jsdoc-text-escaping

/// `<` and an `&` that does not begin an entity, when text follows: `&lt;`, `&amp;`.
fn escape_html(line: &str) -> String {
    let b = line.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        let c = line[i..].chars().next().unwrap_or(' ');
        let next_is_text = line[i + c.len_utf8()..].chars().next().is_some_and(|n| !n.is_whitespace());
        match c {
            '<' if next_is_text => out.push_str("&lt;"),
            '&' if next_is_text && !["amp;", "lt;", "gt;", "quot;", "apos;"].iter().any(|e| line[i + 1..].starts_with(e)) => out.push_str("&amp;"),
            c => out.push(c),
        }
        i += c.len_utf8();
    }
    out
}

/// The first backtick of each paired code span is escaped: `` `a` `` becomes `` \`a` ``.
fn escape_markdown(line: &str) -> String {
    let b = line.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'`' {
            let c = line[i..].chars().next().unwrap_or(' ');
            out.push(c);
            i += c.len_utf8();
            continue;
        }
        let mut n = 0;
        while i + n < b.len() && b[i + n] == b'`' {
            n += 1;
        }
        let escaped_already = i > 0 && b[i - 1] == b'\\';
        let after = i + n;
        let content = line[after..].find('`').unwrap_or(line.len() - after);
        let close = after + content;
        let closes = content > 0 && close + n <= b.len() && b[close..close + n].iter().all(|x| *x == b'`') && b.get(close + n) != Some(&b'`');
        if !escaped_already && closes {
            out.push('\\');
            out.push_str(&line[i..close + n]);
            i = close + n;
        } else {
            out.push_str(&line[i..after]);
            i = after;
        }
    }
    out
}

fn escape_lines(b: &Block<'_>, lines: impl Iterator<Item = (usize, usize)>, html: bool) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    for (start, end) in lines {
        let original = &b.src[start..end];
        let fixed = if html { escape_html(original) } else { escape_markdown(original) };
        if fixed != original {
            edits.push(TextEdit::new(start, end, fixed));
        }
    }
    edits
}

super::orule!(JsdocTextEscaping, "js/jsdoc-text-escaping", &["escapeHTML", "escapeMarkdown"], COMMENT, false, |n, ctx, out| {
    let Some(b) = Block::parse(ctx.source, n.start_byte(), n.end_byte()) else { return };
    let html = opt_bool(ctx, "escapeHTML", false);
    // The description first; the tags only when it is clean, as upstream.
    let desc: Vec<(usize, usize)> = b.description_lines().filter(|i| !b.lines[*i].fenced).map(|i| (b.lines[i].body_start, b.lines[i].body_end)).collect();
    let edits = escape_lines(&b, desc.into_iter(), html);
    if let Some(first) = edits.first() {
        let (s, e) = (first.start, first.end);
        let what = if html { "You have unescaped HTML characters < or &" } else { "You have unescaped Markdown backtick sequences" };
        out.push(finding(s, e, what, edits));
        return;
    }
    for t in b.tags.iter().filter(|t| t.tag != "example") {
        let mut spans = vec![(t.desc_start.min(b.lines[t.desc_line].body_end), b.lines[t.desc_line].body_end)];
        for i in t.desc_line + 1..=t.last_line {
            if !b.lines[i].fenced {
                spans.push((b.lines[i].body_start, b.lines[i].body_end));
            }
        }
        let edits = escape_lines(&b, spans.into_iter(), html);
        if let Some(first) = edits.first() {
            let (s, e) = (first.start, first.end);
            let what = if html { "You have unescaped HTML characters < or & in a tag" } else { "You have unescaped Markdown backtick sequences in a tag" };
            out.push(finding(s, e, what, edits));
        }
    }
});

//! `informative-docs` and `normalize-see-links`: what a block says and how it
//! writes its links.

use super::block::inline_tags;
use super::{find, jsdoc_rule, opt_bool, opt_str_list};
use crate::util::{named_children, text};
use codelaw_core::TextEdit;
use tree_sitter::Node;

// ---- informative-docs -----------------------------------------------------------

/// The identifiers a documented node is named by.
fn names_of(n: Node<'_>, src: &str) -> Vec<String> {
    let name_of = |x: Option<Node<'_>>| x.map(|i| text(i, src).to_owned()).into_iter().collect::<Vec<_>>();
    match n.kind() {
        "export_statement" => n.child_by_field_name("declaration").or_else(|| n.child_by_field_name("value")).map(|d| names_of(d, src)).unwrap_or_default(),
        "function_declaration" | "generator_function_declaration" | "class_declaration" | "abstract_class_declaration" | "interface_declaration" | "type_alias_declaration" | "enum_declaration" | "function_signature" | "internal_module" | "module" | "function_expression" | "class" => name_of(n.child_by_field_name("name")),
        "method_definition" | "public_field_definition" | "field_definition" | "method_signature" | "abstract_method_signature" | "property_signature" => {
            let mut v = Vec::new();
            if let Some(class) = n.parent().and_then(|b| b.parent()) {
                v.extend(names_of(class, src));
            }
            v.extend(name_of(n.child_by_field_name("name")));
            v
        }
        "lexical_declaration" | "variable_declaration" => {
            let Some(d) = named_children(n).into_iter().find(|c| c.kind() == "variable_declarator") else { return Vec::new() };
            let mut v = name_of(d.child_by_field_name("name"));
            if let Some(init) = d.child_by_field_name("value") {
                v.extend(names_of(init, src));
            }
            v
        }
        "pair" => name_of(n.child_by_field_name("key")),
        _ => Vec::new(),
    }
}

/// Lower-cased words, split at case changes and anything not a letter or digit.
fn words(s: &str) -> Vec<String> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in cs.iter().enumerate() {
        if !c.is_alphanumeric() {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        if c.is_uppercase() && !cur.is_empty() {
            let prev = cs[i - 1];
            let next = cs.get(i + 1).copied();
            if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next.is_some_and(char::is_lowercase)) {
                out.push(std::mem::take(&mut cur));
            }
        }
        cur.extend(c.to_lowercase());
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Whether `docs` says something the `name` does not: any word that is not
/// useless and not one of the name's (an alias counts as its key).
fn informative(docs: &str, name: &str, uselesss: &[String], aliases: &[(String, Vec<String>)]) -> bool {
    let canon = |w: String| -> String {
        for (key, list) in aliases {
            if list.contains(&w) {
                return key.clone();
            }
        }
        w
    };
    let name_words: Vec<String> = words(name).into_iter().map(canon).collect();
    words(docs).into_iter().filter(|w| !uselesss.iter().any(|u| u == w)).map(canon).any(|w| !name_words.contains(&w))
}

jsdoc_rule!(InformativeDocs, "js/jsdoc-informative-docs", |n, b, ctx, out| {
    let node_names = super::target::target(n).map(|t| names_of(t, ctx.source)).unwrap_or_default().join(" ");
    let mut useless = opt_str_list(ctx.options, "useless_words");
    if !ctx.options.contains_key("useless_words") {
        useless = ["a", "an", "i", "in", "of", "s", "the"].iter().map(|s| (*s).to_owned()).collect();
    }
    let excluded = opt_str_list(ctx.options, "excluded_tags");
    let aliases = vec![("a".to_owned(), vec!["an".to_owned(), "our".to_owned()])];
    let redundant = |docs: &str, extra: &str| -> bool {
        let d = docs.trim();
        !d.is_empty() && !informative(d, &format!("{extra} {node_names}"), &useless, &aliases)
    };
    let description = b.description();
    for t in &b.tags {
        if excluded.contains(&t.tag) {
            continue;
        }
        if redundant(&b.tag_description(t), &t.name) {
            out.push(find(t.at, t.tag_end, "This tag description only repeats the name it describes.", vec![]));
        }
    }
    if redundant(&description, "") {
        out.push(find(b.start, b.start + 3, "This description only repeats the name it describes.", vec![]));
    }
});

// ---- normalize-see-links ----------------------------------------------------------

struct Link {
    start: usize,
    end: usize,
    label: String,
    target: String,
    /// 0 markdown, 1 prefix, 2 pipe.
    form: u8,
}

fn is_target_char(c: char) -> bool {
    !c.is_whitespace() && c != '}' && c != '|'
}

/// `[label]` at `i` (a `[` not escaped): the label and the index after `]`.
fn bracket_label(cs: &[(usize, char)], i: usize, allow_tick: bool) -> Option<(String, usize)> {
    let mut j = i + 1;
    let mut label = String::new();
    loop {
        let (_, c) = *cs.get(j)?;
        match c {
            '\\' => {
                let (_, d) = *cs.get(j + 1)?;
                label.push('\\');
                label.push(d);
                j += 2;
            }
            ']' => break,
            '`' if !allow_tick => return None,
            _ => {
                label.push(c);
                j += 1;
            }
        }
    }
    if label.is_empty() {
        return None;
    }
    Some((label, j + 1))
}

fn link_target(cs: &[(usize, char)], mut j: usize) -> (String, usize) {
    let mut t = String::new();
    while let Some(&(_, c)) = cs.get(j) {
        if !is_target_char(c) {
            break;
        }
        t.push(c);
        j += 1;
    }
    (t, j)
}

fn starts_with(cs: &[(usize, char)], j: usize, lit: &str) -> bool {
    lit.chars().enumerate().all(|(k, c)| cs.get(j + k).is_some_and(|&(_, d)| d == c))
}

fn scan_links(s: &str) -> Vec<Link> {
    let cs: Vec<(usize, char)> = s.char_indices().collect();
    let end_of = |j: usize| cs.get(j).map_or(s.len(), |&(p, _)| p);
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let (pos, c) = cs[i];
        if c == '[' && !(i > 0 && cs[i - 1].1 == '\\') {
            // `[label](target)`
            if let Some((label, j)) = bracket_label(&cs, i, false)
                && starts_with(&cs, j, "(")
            {
                let (t, k) = link_target(&cs, j + 1);
                let t: String = t.chars().take_while(|c| !matches!(c, '`' | '(' | ')')).collect();
                let after = j + 1 + t.chars().count();
                if !t.is_empty() && starts_with(&cs, after, ")") && (k >= after) {
                    out.push(Link { start: pos, end: end_of(after + 1), label, target: t, form: 0 });
                    i = after + 1;
                    continue;
                }
            }
            // `[label]{@link target}`
            if let Some((label, j)) = bracket_label(&cs, i, true)
                && starts_with(&cs, j, "{@link")
                && cs.get(j + 6).is_some_and(|&(_, c)| c.is_whitespace())
            {
                let mut k = j + 6;
                while cs.get(k).is_some_and(|&(_, c)| c.is_whitespace()) {
                    k += 1;
                }
                let (t, e) = link_target(&cs, k);
                if !t.is_empty() && starts_with(&cs, e, "}") {
                    out.push(Link { start: pos, end: end_of(e + 1), label, target: t, form: 1 });
                    i = e + 1;
                    continue;
                }
            }
        }
        if c == '{' && starts_with(&cs, i, "{@link") && cs.get(i + 6).is_some_and(|&(_, c)| c.is_whitespace()) {
            // `{@link target | label}`
            let mut k = i + 6;
            while cs.get(k).is_some_and(|&(_, c)| c.is_whitespace()) {
                k += 1;
            }
            let (t, e) = link_target(&cs, k);
            let mut m = e;
            while cs.get(m).is_some_and(|&(_, c)| c == ' ' || c == '\t') {
                m += 1;
            }
            if !t.is_empty() && starts_with(&cs, m, "|") {
                m += 1;
                while cs.get(m).is_some_and(|&(_, c)| c == ' ' || c == '\t') {
                    m += 1;
                }
                let mut label = String::new();
                let mut q = m;
                loop {
                    match cs.get(q) {
                        Some(&(_, '\\')) if cs.get(q + 1).is_some() => {
                            label.push('\\');
                            label.push(cs[q + 1].1);
                            q += 2;
                        }
                        Some(&(_, '}')) => break,
                        Some(&(_, c)) => {
                            label.push(c);
                            q += 1;
                        }
                        None => break,
                    }
                }
                if cs.get(q).is_some_and(|&(_, c)| c == '}') && !label.is_empty() {
                    out.push(Link { start: pos, end: end_of(q + 1), label, target: t, form: 2 });
                    i = q + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

fn in_code_span(s: &str, at: usize) -> bool {
    s[..at].matches('`').count() % 2 == 1 && s[at..].contains('`')
}

fn encode_target(t: &str) -> String {
    let mut out = String::new();
    let cs: Vec<char> = t.chars().collect();
    for (i, &c) in cs.iter().enumerate() {
        match c {
            '%' if !(cs.get(i + 1).is_some_and(char::is_ascii_hexdigit) && cs.get(i + 2).is_some_and(char::is_ascii_hexdigit)) => out.push_str("%25"),
            '|' => out.push_str("%7C"),
            '{' => out.push_str("%7B"),
            '}' => out.push_str("%7D"),
            _ => out.push(c),
        }
    }
    out
}

/// The label as the pipe form writes it: the source's escapes decoded, `}` and a
/// trailing or doubled `\` escaped.
fn pipe_label(label: &str, form: u8) -> String {
    let cs: Vec<char> = label.trim().chars().collect();
    let mut decoded = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let esc = cs[i] == '\\' && cs.get(i + 1).is_some_and(|&d| d == '\\' || (form == 2 && d == '}') || (form == 1 && d == ']') || (form == 0 && (d == '}' || d == ']')));
        if esc {
            decoded.push(cs[i + 1]);
            i += 2;
        } else {
            decoded.push(cs[i]);
            i += 1;
        }
    }
    let mut out = String::new();
    for (k, &c) in decoded.iter().enumerate() {
        match c {
            '}' => out.push_str("\\}"),
            '\\' if k + 1 == decoded.len() || matches!(decoded[k + 1], '\\' | '}') => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out
}

fn is_scoped_package(t: &str) -> bool {
    let Some(rest) = t.strip_prefix('@') else { return false };
    let Some((scope, name)) = rest.split_once('/') else { return false };
    let ok = |s: &str| !s.is_empty() && s.chars().next().is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '-'));
    ok(scope) && ok(name) && scope.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '-'))
}

jsdoc_rule!(NormalizeSeeLinks, "js/jsdoc-normalize-see-links", |n, b, ctx, out| {
    let _ = n;
    let fix = opt_bool(ctx.options, "enable_fixer", true);
    for t in b.tags.iter().filter(|t| t.tag == "see") {
        let mut edits = Vec::new();
        let mut ambiguous = false;
        let mut noncanonical = false;
        for li in t.first_line..=t.last_line {
            let l = &b.lines[li];
            let from = if li == t.first_line { (if t.name.is_empty() { t.desc_start } else { t.name_start }).max(l.body_start) } else { l.body_start };
            if from >= l.body_end {
                continue;
            }
            let s = &b.src[from..l.body_end];
            let links = scan_links(s);
            let mut remainder = s.to_owned();
            for lk in &links {
                remainder.replace_range(lk.start..lk.end, &" ".repeat(lk.end - lk.start));
                let blank_label = lk.label.trim().is_empty();
                if lk.form == 2 {
                    if blank_label {
                        ambiguous = true;
                    }
                    continue;
                }
                if (lk.form == 0 && (in_code_span(s, lk.start) || is_scoped_package(&lk.target))) || blank_label {
                    ambiguous = true;
                    continue;
                }
                noncanonical = true;
                edits.push(TextEdit::new(from + lk.start, from + lk.end, format!("{{@link {}|{}}}", encode_target(&lk.target), pipe_label(&lk.label, lk.form))));
            }
            // A link written `{@link target label}`, or one the scan could not read.
            for it in inline_tags(&remainder, 0) {
                if it.tag == "link" && it.text.contains(char::is_whitespace) && !it.text.contains('|') {
                    ambiguous = true;
                }
            }
            if remainder.contains("](") || remainder.contains("]{@link") || (remainder.contains("{@link") && remainder.contains('|') && !inline_tags(&remainder, 0).iter().any(|i| i.tag == "link" && !i.text.is_empty() && !i.text.contains(char::is_whitespace))) {
                // Only an attempt: text that looks like a link and is not one the scan reads.
                if remainder.contains("](") || remainder.contains("]{@link") {
                    ambiguous = true;
                }
            }
        }
        if ambiguous {
            out.push(find(t.at, t.tag_end, "@see link cannot be safely normalized.", vec![]));
        } else if noncanonical {
            out.push(find(t.at, t.tag_end, "Expected @see link to use the pipe form.", if fix { edits } else { vec![] }));
        }
    }
});

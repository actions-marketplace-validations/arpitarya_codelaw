//! Helpers the native rules share.

use codelaw_core::TextEdit;
use tree_sitter::Node;

pub fn named_children<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut c = n.walk();
    n.named_children(&mut c).collect()
}

pub fn children<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut c = n.walk();
    n.children(&mut c).collect()
}

pub fn text<'s>(n: Node<'_>, src: &'s str) -> &'s str {
    &src[n.byte_range()]
}

pub fn has_token(n: Node<'_>, token: &str) -> bool {
    children(n).iter().any(|c| !c.is_named() && c.kind() == token)
}

/// Whether a statement always leaves the block it is in.
pub fn exits(node: &Node<'_>) -> bool {
    match node.kind() {
        "return_statement" | "throw_statement" | "break_statement" | "continue_statement" => true,
        "statement_block" => named_children(*node).iter().any(exits),
        "if_statement" => {
            let consequence = node.child_by_field_name("consequence");
            let alternative = node.child_by_field_name("alternative").and_then(|a| a.named_child(0));
            matches!((consequence, alternative), (Some(c), Some(a)) if exits(&c) && exits(&a))
        }
        "try_statement" => {
            let finally = node.child_by_field_name("finalizer").and_then(|f| f.child_by_field_name("body").or(Some(f)));
            if finally.is_some_and(|f| exits(&f)) {
                return true;
            }
            let body = node.child_by_field_name("body");
            let handler = node.child_by_field_name("handler").and_then(|h| h.child_by_field_name("body"));
            body.is_some_and(|b| exits(&b)) && handler.is_none_or(|h| exits(&h))
        }
        _ => false,
    }
}

/// Function-like nodes: a boundary for `this`, `return` and `super()`.
pub fn is_function(kind: &str) -> bool {
    matches!(
        kind,
        "function_declaration"
            | "function_expression"
            | "function"
            | "generator_function"
            | "generator_function_declaration"
            | "method_definition"
            | "class_declaration"
            | "class"
            | "abstract_class_declaration"
    )
}

/// Every descendant of `n`, not entering nodes `stop` says to skip.
pub fn descendants<'t>(n: Node<'t>, stop: &dyn Fn(&Node<'t>) -> bool, out: &mut Vec<Node<'t>>) {
    for c in children(n) {
        out.push(c);
        if !stop(&c) {
            descendants(c, stop, out);
        }
    }
}

/// A key as a property name: identifiers and strings by value, numbers
/// normalised, computed keys only when they are a plain literal.
pub fn property_name(key: Node<'_>, src: &str) -> Option<String> {
    match key.kind() {
        "property_identifier" | "private_property_identifier" | "identifier" | "shorthand_property_identifier" => {
            Some(text(key, src).to_owned())
        }
        "string" => Some(string_value(key, src)),
        "number" => Some(number_key(text(key, src))),
        "computed_property_name" => {
            let inner = key.named_child(0)?;
            match inner.kind() {
                "string" => Some(string_value(inner, src)),
                "number" => Some(number_key(text(inner, src))),
                _ => None,
            }
        }
        _ => None,
    }
}

/// A string literal's value, without its quotes (escapes are kept).
pub fn string_value(n: Node<'_>, src: &str) -> String {
    let t = text(n, src);
    if t.len() >= 2 { unquote(t).to_owned() } else { t.to_owned() }
}

/// `t` without its first and last character, cut on character boundaries:
/// an unterminated string ending in `é` must not split it (byte slicing
/// `[1..len - 1]` panics there). Fewer than two characters give "".
pub fn unquote(t: &str) -> &str {
    let mut c = t.chars();
    c.next();
    c.next_back();
    c.as_str()
}

/// A numeric key as JavaScript would print it: `0x1`, `1.0` and `1` are one key.
fn number_key(t: &str) -> String {
    let t = t.replace('_', "");
    let lower = t.to_ascii_lowercase();
    let parsed = if let Some(h) = lower.strip_prefix("0x") {
        u128::from_str_radix(h, 16).ok().map(|v| v as f64)
    } else if let Some(o) = lower.strip_prefix("0o") {
        u128::from_str_radix(o, 8).ok().map(|v| v as f64)
    } else if let Some(b) = lower.strip_prefix("0b") {
        u128::from_str_radix(b, 2).ok().map(|v| v as f64)
    } else {
        lower.parse::<f64>().ok()
    };
    match parsed {
        Some(v) if v.fract() == 0.0 && v.abs() < 1e21 => format!("{}", v as i128),
        Some(v) => format!("{v}"),
        None => t,
    }
}

/// Whether a node is inside a TypeScript `declare` (an ambient context:
/// types only, no code that runs).
pub fn in_ambient(n: Node<'_>) -> bool {
    let mut p = n.parent();
    while let Some(x) = p {
        if x.kind() == "ambient_declaration" {
            return true;
        }
        p = x.parent();
    }
    false
}

/// 1-based line of a byte offset.
pub fn line_of(src: &str, offset: usize) -> usize {
    src[..offset.min(src.len())].matches('\n').count() + 1
}

/// Widen `start..end` to whole lines when nothing else shares them, so a
/// removal leaves no blank line behind.
pub fn whole_lines(src: &str, start: usize, end: usize) -> (usize, usize) {
    let line_start = src[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    if !src[line_start..start].trim().is_empty() {
        return (start, end);
    }
    let rest = &src[end..];
    let line_end = rest.find('\n').map(|i| end + i + 1).unwrap_or(src.len());
    if src[end..line_end].trim().is_empty() { (line_start, line_end) } else { (start, end) }
}

/// Whether a statement returns a value (or throws) on every path.
pub fn returns_value(n: &Node<'_>) -> bool {
    match n.kind() {
        "return_statement" => n.named_child_count() > 0,
        "throw_statement" => true,
        "statement_block" => named_children(*n).iter().any(returns_value),
        "if_statement" => {
            let c = n.child_by_field_name("consequence");
            let a = n.child_by_field_name("alternative").and_then(|a| a.named_child(0));
            matches!((c, a), (Some(c), Some(a)) if returns_value(&c) && returns_value(&a))
        }
        "try_statement" => {
            let fin = n.child_by_field_name("finalizer").and_then(|f| f.child_by_field_name("body"));
            if fin.is_some_and(|f| returns_value(&f)) {
                return true;
            }
            let body = n.child_by_field_name("body");
            let handler = n.child_by_field_name("handler").and_then(|h| h.child_by_field_name("body"));
            body.is_some_and(|b| returns_value(&b)) && handler.is_none_or(|h| returns_value(&h))
        }
        "switch_statement" => {
            let Some(body) = n.child_by_field_name("body") else { return false };
            let cases = named_children(body);
            cases.iter().any(|c| c.kind() == "switch_default")
                && cases.iter().all(|c| {
                    let stmts: Vec<Node> = named_children(*c).into_iter().filter(|s| Some(*s) != c.child_by_field_name("value")).collect();
                    stmts.is_empty() || stmts.iter().any(returns_value)
                })
        }
        _ => false,
    }
}

/// The edit removing one item of a comma-separated list (arguments,
/// import specifiers, object members, array elements) with its comma: the
/// comma after it, or before it when it is the last.
pub fn remove_list_item(item: Node<'_>, src: &str) -> TextEdit {
    let (start, end) = (item.start_byte(), item.end_byte());
    let after = &src[end..];
    let ws = after.len() - after.trim_start().len();
    if after[ws..].starts_with(',') {
        let rest = &after[ws + 1..];
        let ws2 = rest.len() - rest.trim_start_matches([' ', '\t']).len();
        let stop = end + ws + 1 + ws2;
        // A trailing comma closing the list goes with its item only when
        // another item stays before it.
        let next = src[stop..].trim_start();
        if next.starts_with([')', ']', '}']) {
            let before = src[..start].trim_end();
            if let Some(b) = before.strip_suffix(',') {
                return TextEdit::delete(b.len(), stop - ws2);
            }
        }
        return TextEdit::delete(start, stop);
    }
    let before = src[..start].trim_end();
    if let Some(b) = before.strip_suffix(',') {
        return TextEdit::delete(b.len(), end);
    }
    TextEdit::delete(start, end)
}

/// The edit removing a whole statement, with its line when nothing else
/// shares it.
pub fn remove_statement(stmt: Node<'_>, src: &str) -> TextEdit {
    let (s, e) = whole_lines(src, stmt.start_byte(), stmt.end_byte());
    TextEdit::delete(s, e)
}

/// The nearest ancestor of `n` (or `n`) whose kind is one of `kinds`.
pub fn ancestor<'t>(n: Node<'t>, kinds: &[&str]) -> Option<Node<'t>> {
    let mut cur = Some(n);
    while let Some(c) = cur {
        if kinds.contains(&c.kind()) {
            return Some(c);
        }
        cur = c.parent();
    }
    None
}

/// Levenshtein distance, for nearest-name choices.
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut cur = vec![i; b.len() + 1];
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
        }
        prev = cur;
    }
    prev[b.len()]
}

/// The names of `candidates` nearest to `name` — within `max` edits, at
/// most three, nearest first, ties in the candidates' order.
pub fn nearest<'c>(name: &str, candidates: impl IntoIterator<Item = &'c str>, max: usize) -> Vec<&'c str> {
    let mut scored: Vec<(usize, usize, &str)> = candidates
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c != name)
        .map(|(i, c)| (edit_distance(&name.to_lowercase(), &c.to_lowercase()), i, c))
        .filter(|(d, _, _)| *d <= max)
        .collect();
    scored.sort();
    scored.dedup_by(|a, b| a.2 == b.2);
    scored.into_iter().take(3).map(|(_, _, c)| c).collect()
}

/// One labelled choice per name in `names`, each replacing `start..end`
/// with it (the first the default), as [`crate::choice_edits`] carries
/// them.
pub fn name_choices(names: &[&str], start: usize, end: usize, fmt: &dyn Fn(&str) -> String) -> Vec<TextEdit> {
    crate::choice_edits(names.iter().enumerate().map(|(i, n)| crate::choice(*n, i == 0, vec![TextEdit::new(start, end, fmt(n))])).collect())
}

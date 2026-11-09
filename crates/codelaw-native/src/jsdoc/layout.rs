//! `tag-lines` and `sort-tags`: the blank lines in a block and the order of its tags.

use super::block::Block;
use super::{delete_lines, find, jsdoc_rule};
use codelaw_core::TextEdit;

/// A blank line inside the comment: no content, not the opening or closing line.
fn blank(b: &Block<'_>, i: usize) -> bool {
    i > 0 && !b.lines[i].closes && b.is_blank(i)
}

// ---- tag-lines (the plugin's defaults: "never", no start or end lines) ----

jsdoc_rule!(TagLines, "js/jsdoc-tag-lines", |n, b, ctx, out| {
    let _ = (n, ctx);
    // Blank lines between a tag's last content and the next tag (or the end).
    for t in &b.tags {
        let mut last = t.last_line;
        while last > t.first_line && (blank(&b, last) || (b.lines[last].closes && b.is_blank(last))) {
            last -= 1;
        }
        let from = last + 1;
        let mut to = t.last_line;
        while to >= from && !blank(&b, to) {
            if to == 0 {
                break;
            }
            to -= 1;
        }
        if from <= to && (from..=to).all(|i| blank(&b, i)) {
            out.push(find(b.lines[from].start, b.lines[from].end, "Expected no lines between tags", vec![delete_lines(&b, from, to)]));
        }
    }
    // Blank lines between the description and the first tag.
    if let Some(first) = b.tags.first() {
        let desc_has_text = (0..first.first_line).any(|i| !b.is_blank(i));
        if desc_has_text {
            let mut from = first.first_line;
            while from > 1 && blank(&b, from - 1) {
                from -= 1;
            }
            if from < first.first_line {
                let count = first.first_line - from;
                let msg = format!("Expected only 0 lines after block description (found {count})");
                out.push(find(b.lines[from].start, b.lines[from].end, msg, vec![delete_lines(&b, from, first.first_line - 1)]));
            }
        }
    }
});

// ---- sort-tags (the plugin's default sequence, one group) -----------------

/// The default order; `-other` stands for the tags not listed.
const ORDER: &[&str] = &[
    "summary", "typeSummary", "module", "exports", "file", "fileoverview", "overview", "import", "template", "typedef", "interface", "record", "name", "kind", "type", "alias", "external", "host", "callback", "func", "function", "method", "class",
    "constructor", "modifies", "mixes", "mixin", "mixinClass", "mixinFunction", "namespace", "borrows", "constructs", "lends", "implements", "requires", "desc", "description", "classdesc", "tutorial", "copyright", "license", "internal", "overload",
    "const", "constant", "final", "global", "readonly", "abstract", "virtual", "var", "member", "memberof", "memberof!", "inner", "instance", "inheritdoc", "inheritDoc", "override", "hideconstructor", "param", "arg", "argument", "prop", "property",
    "return", "returns", "async", "generator", "default", "defaultvalue", "enum", "augments", "extends", "throws", "exception", "yield", "yields", "event", "fires", "emits", "listens", "this", "satisfies", "static", "private", "protected", "public",
    "access", "package", "-other", "see", "example", "closurePrimitive", "customElement", "expose", "hidden", "idGenerator", "meaning", "ngInject", "owner", "wizaction", "define", "dict", "export", "externs", "implicitCast", "noalias", "nocollapse",
    "nocompile", "noinline", "nosideeffects", "polymer", "polymerBehavior", "preserve", "struct", "suppress", "unrestricted", "category", "ignore", "author", "version", "variation", "since", "deprecated", "todo",
];

fn position(tag: &str) -> usize {
    let other = ORDER.iter().position(|t| *t == "-other").unwrap_or(ORDER.len());
    ORDER.iter().position(|t| *t == tag).unwrap_or(other)
}

jsdoc_rule!(SortTags, "js/jsdoc-sort-tags", |n, b, ctx, out| {
    let _ = (n, ctx);
    if b.tags.len() < 2 {
        return;
    }
    let mut sorted: Vec<usize> = (0..b.tags.len()).collect();
    sorted.sort_by_key(|&i| position(&b.tags[i].tag));
    let first_changed = sorted.iter().enumerate().position(|(k, &i)| k != i);
    if let Some(k) = first_changed {
        let msg = "Tags are not in the prescribed order (summary, module, … param, returns, throws … see, example … deprecated, todo)";
        let tag = &b.tags[k];
        // The fix moves whole tags; the last tag's closing `*/` must be on a line of its own.
        let last = &b.tags[b.tags.len() - 1];
        let mut edits = Vec::new();
        // Several `@typedef`, `@callback` or `@overload` tags each own the tags after them: moving tags would change which belongs to which.
        let owners = b.tags.iter().filter(|t| matches!(t.tag.as_str(), "typedef" | "callback" | "overload")).count();
        if owners < 2 && (!b.lines[last.last_line].closes || b.is_blank(last.last_line)) {
            let nl = if b.src[b.start..b.end].contains("\r\n") { "\r\n" } else { "\n" };
            let end_line = if b.lines[last.last_line].closes { last.last_line - 1 } else { last.last_line };
            let from = b.lines[tag.first_line].start;
            let to = b.lines[end_line].end;
            let mut text = String::new();
            for (n_i, &i) in sorted[k..].iter().enumerate() {
                let t = &b.tags[i];
                let mut t_end = if t.last_line == last.last_line { end_line } else { t.last_line };
                // Blank lines that trailed a tag follow it.
                while t_end > t.first_line && blank(&b, t_end) && n_i + 1 == sorted[k..].len() {
                    t_end -= 1;
                }
                if n_i > 0 {
                    text.push_str(nl);
                }
                text.push_str(&b.src[b.lines[t.first_line].start..b.lines[t_end].end]);
            }
            edits.push(TextEdit::new(from, to, text));
        }
        out.push(find(tag.at, tag.tag_end, msg, edits));
        return;
    }
    // In order: a blank line between two tags is not wanted (one group, no spacing).
    for (k, t) in b.tags.iter().enumerate() {
        if k + 1 == b.tags.len() {
            break;
        }
        let mut last = t.last_line;
        while last > t.first_line && blank(&b, last) {
            last -= 1;
        }
        if last < t.last_line {
            out.push(find(b.lines[last + 1].start, b.lines[last + 1].end, "Intra-group tags have unexpected whitespace", vec![delete_lines(&b, last + 1, t.last_line)]));
        }
    }
});

//! Tag names, parameter names and the like.

use super::block::{Block, inline_tags};
use super::tags::{is_known_tag, preferred};
use super::{find, jsdoc_rule, opt_bool, opt_str_list};
use crate::NativeFinding;
use codelaw_core::TextEdit;

const INLINE_TAGS: &[&str] = &["link", "linkcode", "linkplain", "tutorial", "inheritDoc", "label", "include", "includeCode"];
const JSX_TAGS: &[&str] = &["jsx", "jsxFrag", "jsxImportSource", "jsxRuntime"];

/// Every non-fenced line's inline tags.
pub(super) fn all_inline(b: &Block<'_>) -> Vec<super::block::Inline> {
    let mut out = Vec::new();
    for (i, l) in b.lines.iter().enumerate() {
        if l.fenced {
            continue;
        }
        let _ = i;
        let body = &b.src[l.body_start..l.body_end];
        // An inline tag inside backticks is code, not a tag.
        out.extend(inline_tags(body, l.body_start).into_iter().filter(|it| {
            let at = it.start - l.body_start;
            !(body[..at].matches('`').count() % 2 == 1 && body[at..].contains('`'))
        }));
    }
    out
}

jsdoc_rule!(CheckTagNames, "js/jsdoc-check-tag-names", |n, b, ctx, out| {
    let _ = n;
    let defined = opt_str_list(ctx.options, "defined_tags");
    let jsx = opt_bool(ctx.options, "jsx_tags", false);
    for t in &b.tags {
        if jsx && JSX_TAGS.contains(&t.tag.as_str()) {
            continue;
        }
        if defined.contains(&t.tag) {
            continue;
        }
        if is_known_tag(&t.tag) {
            let p = preferred(&t.tag);
            if p != t.tag {
                let msg = format!("Invalid JSDoc tag (preference). Replace \"{}\" JSDoc tag with \"{p}\".", t.tag);
                out.push(find(t.at, t.tag_end, msg, vec![TextEdit::new(t.at + 1, t.tag_end, p)]));
            }
        } else {
            out.push(find(t.at, t.tag_end, format!("Invalid JSDoc tag name \"{}\".", t.tag), vec![]));
        }
    }
    let mut allowed: Vec<String> = INLINE_TAGS.iter().map(|s| (*s).to_owned()).collect();
    allowed.extend(opt_str_list(ctx.options, "inline_tags"));
    for it in all_inline(&b) {
        if !allowed.contains(&it.tag) {
            out.push(find(it.start, it.end, format!("Invalid JSDoc inline tag name \"{}\"", it.tag), vec![]));
        }
    }
});

// ---- shared pieces for the name checks ---------------------------------------

/// Removing a whole tag: its lines, unless it shares a line with the closing `*/`.
pub(super) fn tag_removal(b: &Block<'_>, t: &super::block::Tag) -> Option<TextEdit> {
    let last = t.last_line;
    let end = if b.lines[last].closes {
        if b.is_blank(last) {
            last.checked_sub(1)?
        } else {
            return None;
        }
    } else {
        last
    };
    if end < t.first_line {
        return None;
    }
    Some(super::delete_lines(b, t.first_line, end))
}

/// The range of a tag's name token within the comment, for renaming it.
fn name_range(b: &Block<'_>, t: &super::block::Tag) -> (usize, usize) {
    let raw = &b.src[t.name_start..t.name_end];
    match raw.find(t.name.as_str()) {
        Some(k) if t.optional => (t.name_start + k, t.name_start + k + t.name.len()),
        _ => (t.name_start, t.name_end),
    }
}

/// `a.b` / `a[].b` path declarations come after their root: the checks shared by
/// `check-param-names` and `check-property-names`.
fn check_paths(b: &Block<'_>, tag: &str, what: &str, out: &mut Vec<NativeFinding>) {
    let mut last_real: Option<String> = None;
    for t in b.tags.iter().filter(|t| t.tag == tag) {
        let name = t.name.trim();
        if name.contains('.') {
            let Some(root_real) = &last_real else {
                out.push(find(t.at, t.tag_end, format!("@{tag} path declaration (\"{name}\") appears before any real {what}."), vec![]));
                return;
            };
            let mut root = &name[..name.find('.').unwrap_or(name.len())];
            if let Some(r) = root.strip_suffix("[]") {
                root = r;
            }
            if root != root_real {
                out.push(find(t.at, t.tag_end, format!("@{tag} path declaration (\"{name}\") root node name (\"{root}\") does not match previous real {what} name (\"{root_real}\")."), vec![]));
                return;
            }
        } else {
            last_real = Some(name.to_owned());
        }
    }
}

// ---- check-param-names --------------------------------------------------------

jsdoc_rule!(CheckParamNames, "js/jsdoc-check-param-names", |n, b, ctx, out| {
    let Some(f) = super::target::target(n).and_then(super::target::function_of) else { return };
    let allow_extra = opt_bool(ctx.options, "allow_extra_trailing_param_docs", false);
    let tags: Vec<&super::block::Tag> = b.tags.iter().filter(|t| t.tag == "param").collect();
    if tags.is_empty() {
        return;
    }
    let actual: Vec<String> = tags.iter().filter(|t| !t.name.contains('.')).map(|t| t.name.trim().to_owned()).collect();
    let documents_this = actual.iter().any(|a| a == "this");
    let expected: Vec<super::target::PName> = super::target::params(f, ctx.source).into_iter().filter(|p| documents_this || *p != super::target::PName::This).collect();
    let mut dotted = 0;
    let mut done = false;
    for (index, t) in tags.iter().enumerate() {
        let name = t.name.trim();
        if tags[..index].iter().any(|e| e.name == t.name) {
            let edits = tag_removal(&b, t).map(|e| crate::as_suggestion(vec![e])).unwrap_or_default();
            out.push(find(t.at, t.tag_end, format!("Duplicate @param \"{name}\""), edits));
            done = true;
            break;
        }
        if name.contains('.') {
            dotted += 1;
            continue;
        }
        let Some(fp) = expected.get(index - dotted) else {
            if allow_extra {
                continue;
            }
            let edits = tag_removal(&b, t).map(|e| crate::as_suggestion(vec![e])).unwrap_or_default();
            out.push(find(t.at, t.tag_end, format!("@param \"{name}\" does not match an existing function parameter."), edits));
            done = true;
            break;
        };
        match fp {
            super::target::PName::Destructured => {}
            super::target::PName::This | super::target::PName::Name(_) => {
                let want = match fp {
                    super::target::PName::Name(w) => w.as_str(),
                    _ => "this",
                };
                if want != name {
                    let expected_names: Vec<String> = expected
                        .iter()
                        .enumerate()
                        .map(|(i, p)| match p {
                            super::target::PName::Name(w) => w.clone(),
                            super::target::PName::This => "this".to_owned(),
                            super::target::PName::Destructured => actual.get(i).cloned().unwrap_or_default(),
                        })
                        .collect();
                    let msg = format!("Expected @param names to be \"{}\". Got \"{}\".", expected_names.join(", "), actual.join(", "));
                    let mut sorted_a = actual.clone();
                    let mut sorted_e = expected_names.clone();
                    sorted_a.sort();
                    sorted_e.sort();
                    let reorder = sorted_a == sorted_e;
                    let edits = if reorder {
                        vec![]
                    } else {
                        let (s, e) = name_range(&b, t);
                        crate::as_suggestion(vec![TextEdit::new(s, e, want)])
                    };
                    out.push(find(t.at, t.tag_end, msg, edits));
                    done = true;
                    break;
                }
            }
        }
    }
    if !done {
        check_paths(&b, "param", "parameter", out);
    }
});

// ---- check-property-names -----------------------------------------------------

jsdoc_rule!(CheckPropertyNames, "js/jsdoc-check-property-names", |n, b, ctx, out| {
    let _ = (n, ctx);
    let props: Vec<(usize, &super::block::Tag)> = b.tags.iter().enumerate().filter(|(_, t)| t.tag == "property").collect();
    if props.is_empty() {
        return;
    }
    let typedefs: Vec<usize> = b.tags.iter().enumerate().filter(|(_, t)| t.tag == "typedef").map(|(i, _)| i).collect();
    let group_of = |i: usize| if typedefs.len() > 1 { typedefs.iter().rposition(|&d| d <= i) } else { Some(0) };
    for (k, (i, t)) in props.iter().enumerate() {
        let dupe = props[..k].iter().any(|(j, e)| e.name == t.name && group_of(*j) == group_of(*i));
        if dupe {
            let edits = tag_removal(&b, t).map(|e| crate::as_suggestion(vec![e])).unwrap_or_default();
            out.push(find(t.at, t.tag_end, format!("Duplicate @property \"{}\"", t.name), edits));
            return;
        }
    }
    check_paths(&b, "property", "property", out);
});

// ---- check-template-names -----------------------------------------------------

/// `T, [U=T[]]` as (name, default) pairs, split at the top-level commas.
fn template_names(raw: &str) -> Vec<(String, Option<String>)> {
    let mut items: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    for c in raw.chars() {
        match c {
            '[' | '<' | '(' | '{' => depth += 1,
            ']' | '>' | ')' | '}' => depth -= 1,
            ',' if depth == 0 => {
                items.push(std::mem::take(&mut cur));
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    items.push(cur);
    items
        .into_iter()
        .map(|i| i.trim().to_owned())
        .filter(|i| !i.is_empty())
        .map(|i| {
            let inner = i.strip_prefix('[').map(|x| x.strip_suffix(']').unwrap_or(x)).unwrap_or(&i);
            match inner.find('=') {
                Some(k) => (inner[..k].trim().to_owned(), Some(inner[k + 1..].trim().to_owned()).filter(|d| !d.is_empty())),
                None => (inner.trim().to_owned(), None),
            }
        })
        .collect()
}

fn used_names(src: &str, out: &mut std::collections::HashSet<String>) {
    let Ok(ty) = super::types::parse(src) else { return };
    super::types::walk(&ty, &mut |nd, _| {
        if let super::types::Kind::Name(name) = &nd.kind {
            out.insert(name.split(['.', '#', '~']).next().unwrap_or(name).to_owned());
        }
    });
}

fn type_param_names(decl: tree_sitter::Node<'_>, src: &str) -> Vec<String> {
    let Some(tp) = decl.child_by_field_name("type_parameters") else { return Vec::new() };
    crate::util::named_children(tp).into_iter().filter(|c| c.kind() == "type_parameter").filter_map(|c| c.child_by_field_name("name")).map(|n| crate::util::text(n, src).to_owned()).collect()
}

fn param_return_types(blk: &Block<'_>, used: &mut std::collections::HashSet<String>) {
    for t in blk.tags.iter().filter(|t| matches!(t.tag.as_str(), "param" | "returns")) {
        if let Some(ty) = t.type_text(blk.src) {
            used_names(ty, used);
        }
    }
}

jsdoc_rule!(CheckTemplateNames, "js/jsdoc-check-template-names", |n, b, ctx, out| {
    let templates: Vec<&super::block::Tag> = b.tags.iter().filter(|t| t.tag == "template").collect();
    if templates.is_empty() {
        return;
    }
    let src = ctx.source;
    let mut used = std::collections::HashSet::new();
    let check_templates = |used: &mut std::collections::HashSet<String>, out: &mut Vec<NativeFinding>| {
        for t in &templates {
            if let Some(ty) = t.type_text(src) {
                used_names(ty, used);
            }
            for (_, d) in template_names(&t.name) {
                if let Some(d) = d {
                    used_names(&d, used);
                }
            }
        }
        for t in &templates {
            for (name, _) in template_names(&t.name) {
                if !used.contains(&name) {
                    out.push(find(t.at, t.tag_end, format!("@template {name} not in use"), vec![]));
                }
            }
        }
    };
    let has = |name: &str| b.tags.iter().any(|t| t.tag == name);
    if has("callback") || has("function") {
        param_return_types(&b, &mut used);
        check_templates(&mut used, out);
        return;
    }
    let typedefs: Vec<&super::block::Tag> = b.tags.iter().filter(|t| t.tag == "typedef").collect();
    if typedefs.len() == 1 {
        if let Some(ty) = typedefs[0].type_text(src) {
            used_names(ty, &mut used);
        }
        for t in b.tags.iter().filter(|t| t.tag == "property") {
            if let Some(ty) = t.type_text(src) {
                used_names(ty, &mut used);
            }
        }
        check_templates(&mut used, out);
        return;
    }
    // A class, interface, type alias or function the block documents.
    let Some(mut decl) = super::target::target(n) else { return };
    if matches!(decl.kind(), "export_statement") {
        let Some(d) = decl.child_by_field_name("declaration") else { return };
        decl = d;
    }
    match decl.kind() {
        "class_declaration" | "abstract_class_declaration" | "interface_declaration" | "type_alias_declaration" | "function_declaration" | "generator_function_declaration" => {}
        _ => return,
    }
    used.extend(type_param_names(decl, src));
    if matches!(decl.kind(), "function_declaration" | "generator_function_declaration") {
        param_return_types(&b, &mut used);
    } else if matches!(decl.kind(), "class_declaration" | "abstract_class_declaration")
        && let Some(body) = decl.child_by_field_name("body")
    {
        for m in crate::util::named_children(body) {
            let Some(c) = m.prev_named_sibling() else { continue };
            if c.kind() != "comment" {
                continue;
            }
            let Some(inner) = Block::parse(src, c.start_byte(), c.end_byte()) else { continue };
            param_return_types(&inner, &mut used);
            for t in inner.tags.iter().filter(|t| t.tag == "type") {
                if let Some(ty) = t.type_text(src) {
                    used_names(ty, &mut used);
                }
            }
        }
    }
    for t in b.tags.iter().filter(|t| matches!(t.tag.as_str(), "augments" | "extends" | "implements")) {
        if let Some(ty) = t.type_text(src) {
            used_names(ty, &mut used);
        }
    }
    check_templates(&mut used, out);
});

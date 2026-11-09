//! The rules that read a tag's `{type}`: `valid-types`, `type-formatting`
//! and the three `ts-*` rules.

use super::block::{Block, Tag};
use super::names::all_inline;
use super::tags::{might_have_name, might_have_name_or_namepath, might_have_namepath, might_have_plain_name, might_have_type, must_have_name, must_have_type, type_or_name_required};
use super::types::{self, Kind, MKind, Parent, Printer, Style, Ty};
use super::{find, jsdoc_rule, opt_bool};
use codelaw_core::TextEdit;

fn tag_type<'b>(b: &'b Block<'_>, t: &Tag) -> Option<&'b str> {
    t.type_open.map(|_| &b.src[t.type_start..t.type_end])
}

/// The type of a tag that may carry one, parsed; `None` when it has none or it does not parse.
fn parsed<'b>(b: &'b Block<'_>, t: &Tag) -> Option<(&'b str, Ty)> {
    if !might_have_type(&t.tag) || t.tag == "import" {
        return None;
    }
    let text = tag_type(b, t)?;
    if text.trim().is_empty() {
        return None;
    }
    types::parse(text).ok().map(|ty| (text, ty))
}

// ---- valid-types -----------------------------------------------------------

fn segment_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '$' | '-' | '@' | ':')
}

fn namepath_ok(path: &str) -> bool {
    let mut p = path.strip_prefix("...").unwrap_or(path);
    for pre in ["module:", "external:", "event:"] {
        if let Some(r) = p.strip_prefix(pre) {
            p = r;
            break;
        }
    }
    if p.is_empty() {
        return false;
    }
    let cs: Vec<char> = p.chars().collect();
    let mut i = 0;
    let mut seg_len = 0;
    while i < cs.len() {
        let c = cs[i];
        match c {
            '"' | '\'' | '`' => {
                let Some(close) = cs[i + 1..].iter().position(|x| *x == c) else { return false };
                i += close + 2;
                seg_len += 1;
                continue;
            }
            '.' | '#' | '~' | '/' => {
                if seg_len == 0 {
                    return false;
                }
                seg_len = 0;
            }
            '[' => {
                if cs.get(i + 1) != Some(&']') || seg_len == 0 {
                    return false;
                }
                i += 2;
                continue;
            }
            c if segment_char(c) => seg_len += 1,
            _ => return false,
        }
        i += 1;
    }
    seg_len > 0
}

fn namepath_valid(path: &str, tag: &str) -> bool {
    if namepath_ok(path) || path.chars().all(|c| c == '-') {
        return true;
    }
    let last = path.chars().last();
    match tag {
        "memberof" | "memberof!" if matches!(last, Some('#' | '.' | '~')) => namepath_ok(&path[..path.len() - 1]),
        "module" | "requires" if !path.starts_with("module:") => namepath_ok(&format!("module:{path}")),
        "borrows" if matches!(path.chars().next(), Some('#' | '.' | '~')) => namepath_ok(&path[1..]),
        _ => false,
    }
}

const NULLABLE_SUFFIX_CHECKED: &str = "Syntax error in type: JsdocTypeNullable";

jsdoc_rule!(ValidTypes, "js/jsdoc-valid-types", |n, b, ctx, out| {
    let _ = n;
    let allow_empty = opt_bool(ctx.options, "allow_empty_namepaths", false);
    for t in &b.tags {
        let at = (t.at, t.tag_end);
        let tag = t.tag.as_str();
        if let Some(problem) = &t.name_problem {
            out.push(find(at.0, at.1, format!("Invalid name: {problem}"), vec![]));
            continue;
        }
        if tag == "import" {
            continue;
        }
        let desc = b.tag_description(t);
        if tag == "borrows" {
            let has_as = desc.split_whitespace().next() == Some("as") || desc.contains("as ");
            let this_path = desc.replacen("as ", "", 1).trim().to_owned();
            if !has_as || this_path.is_empty() {
                out.push(find(at.0, at.1, format!("@borrows must have an \"as\" expression. Found \"{desc}\""), vec![]));
                continue;
            }
            if namepath_valid(&this_path, "borrows") {
                if !t.name.is_empty() && !namepath_valid(&t.name, "") {
                    out.push(find(at.0, at.1, format!("Syntax error in namepath: {}", t.name), vec![]));
                }
            } else {
                out.push(find(at.0, at.1, format!("Syntax error in namepath: {this_path}"), vec![]));
            }
            continue;
        }
        let name = t.name.as_str();
        if !name.is_empty() && !might_have_name(tag) {
            out.push(find(at.0, at.1, format!("@{tag} should not have a name."), vec![]));
            continue;
        }
        // `@returns {@link Foo}` is a description, not a type.
        let mut type_text = tag_type(&b, t).map(str::trim);
        if type_text.is_some_and(|x| x.starts_with('@')) {
            type_text = None;
        }
        let has_type = type_text.is_some_and(|x| !x.is_empty());
        if has_type && !might_have_type(tag) {
            out.push(find(at.0, at.1, format!("@{tag} should not have a bracketed type."), vec![]));
            continue;
        }
        if must_have_name(tag) && name.is_empty() && !allow_empty && !matches!(tag, "arg" | "argument" | "param" | "prop" | "property") && !(tag == "see" && desc.contains("{@link")) {
            out.push(find(at.0, at.1, format!("Tag @{tag} must have a name/namepath."), vec![]));
            continue;
        }
        if must_have_type(tag) && !has_type && tag != "next" {
            out.push(find(at.0, at.1, format!("Tag @{tag} must have a type."), vec![]));
            continue;
        }
        if type_or_name_required(tag) && !must_have_type(tag) && !allow_empty {
            let has_name = (must_have_name(tag) || might_have_name_or_namepath(tag)) && !name.is_empty();
            if !(has_type && might_have_type(tag)) && !has_name {
                out.push(find(at.0, at.1, format!("Tag @{tag} must have either a type or namepath."), vec![]));
                continue;
            }
        }
        if has_type && might_have_type(tag) && !(type_text == Some("const") && tag == "type") {
            let text = tag_type(&b, t).unwrap_or("");
            match types::parse(text) {
                Err(_) => out.push(find(at.0, at.1, format!("Syntax error in type: {}", text.trim()), vec![])),
                Ok(ty) => {
                    let mut bad = false;
                    types::walk(&ty, &mut |nd, _| {
                        let nullable = |x: &Option<Ty>| matches!(x, Some(Ty { kind: Kind::Suffix('?', _), .. }));
                        if let Kind::Object(ms) = &nd.kind {
                            bad |= ms.iter().any(|m| matches!(&m.kind, MKind::Field { ty, .. } if nullable(ty)));
                        }
                    });
                    if bad {
                        out.push(find(at.0, at.1, NULLABLE_SUFFIX_CHECKED, vec![]));
                    }
                }
            }
        }
        let has_namepath_position = (must_have_name(tag) || might_have_namepath(tag)) && !name.is_empty();
        if has_namepath_position {
            if tag != "template" && !namepath_valid(name, tag) {
                out.push(find(at.0, at.1, format!("Syntax error in namepath: {name}"), vec![]));
            }
        } else if might_have_plain_name(tag) && !name.is_empty() && !namepath_valid(name, tag) {
            out.push(find(at.0, at.1, format!("Syntax error in name: {name}"), vec![]));
        }
    }
    for it in all_inline(&b) {
        if matches!(it.tag.as_str(), "include" | "includeCode" | "inheritDoc" | "inheritdoc" | "label" | "link" | "linkcode" | "linkplain" | "tutorial") && it.text.is_empty() {
            out.push(find(it.start, it.end, format!("Inline tag \"{}\" missing content", it.tag), vec![]));
        }
    }
});

// ---- type-formatting (the plugin's defaults) --------------------------------

jsdoc_rule!(TypeFormatting, "js/jsdoc-type-formatting", |n, b, ctx, out| {
    let _ = n;
    let fix = opt_bool(ctx.options, "enable_fixer", true);
    for t in &b.tags {
        if !matches!(t.tag.as_str(), "param" | "property" | "returns" | "this" | "throws" | "type" | "typedef" | "yields") {
            continue;
        }
        let Some(text) = tag_type(&b, t) else { continue };
        if text.trim().is_empty() || text.contains('\n') {
            continue;
        }
        let Ok(ty) = types::parse(text) else { continue };
        let pr = Printer { src: text, shallow: false };
        let canonical = pr.print(&ty);
        if canonical == text {
            continue;
        }
        let mut msg = types::first_format_issue(&ty, text).unwrap_or_else(|| "There was an error with type formatting".to_owned());
        if text.starts_with(char::is_whitespace) || text.ends_with(char::is_whitespace) {
            msg = "Must have no initial spacing".to_owned();
        }
        let edits = if fix { vec![TextEdit::new(t.type_start, t.type_end, canonical)] } else { vec![] };
        out.push(find(t.type_start, t.type_end, msg, edits));
    }
});

// ---- the ts-* rules ----------------------------------------------------------

/// The tags whose type a `ts-*` rule reads, parsed.
fn typed_tags<'b>(b: &'b Block<'_>) -> Vec<(&'b Tag, &'b str, Ty)> {
    b.tags.iter().filter_map(|t| parsed(b, t).map(|(text, ty)| (t, text, ty))).collect()
}

fn needs_parens_outside(p: Parent) -> bool {
    matches!(p, Parent::Other | Parent::ArrayElem)
}

jsdoc_rule!(TsPreferFunctionType, "js/jsdoc-ts-prefer-function-type", |n, b, ctx, out| {
    let _ = n;
    let fix = opt_bool(ctx.options, "enable_fixer", true);
    for (t, text, ty) in typed_tags(&b) {
        let pr = Printer { src: text, shallow: false };
        types::walk(&ty, &mut |nd, parent| {
            let Kind::Object(ms) = &nd.kind else { return };
            let [m] = ms.as_slice() else { return };
            let MKind::Call { f } = &m.kind else { return };
            if f.style != Style::Function {
                return;
            }
            let mut edits = Vec::new();
            if fix && f.ret.is_some() {
                let body = pr.arrow_text(f);
                let body = if needs_parens_outside(parent) { format!("({body})") } else { body };
                edits.push(TextEdit::new(t.type_start + nd.start, t.type_start + nd.end, body));
            }
            out.push(find(t.type_start + nd.start, t.type_start + nd.end, "Call signature found; function type preferred.", edits));
        });
    }
});

jsdoc_rule!(TsMethodSignatureStyle, "js/jsdoc-ts-method-signature-style", |n, b, ctx, out| {
    let _ = n;
    let method_style = ctx.options.get("style").and_then(|v| v.as_str()) == Some("method");
    let fix = opt_bool(ctx.options, "enable_fixer", true);
    for (t, text, ty) in typed_tags(&b) {
        let pr = Printer { src: text, shallow: false };
        types::walk(&ty, &mut |nd, _| {
            let Kind::Object(ms) = &nd.kind else { return };
            for m in ms {
                let abs = |x: usize| t.type_start + x;
                match &m.kind {
                    MKind::Method { key, optional, f } if !method_style => {
                        let same_name = ms.iter().filter(|o| matches!(&o.kind, MKind::Method { key: k2, .. } if k2.raw == key.raw)).count();
                        let mut edits = Vec::new();
                        if fix && f.ret.is_some() && same_name == 1 {
                            let text = format!("{}{}: {}", key.raw, if *optional { "?" } else { "" }, pr.arrow_text(f));
                            edits.push(TextEdit::new(abs(m.start), abs(m.end), text));
                        }
                        out.push(find(abs(m.start), abs(m.end), "Found method signature; prefer function property.", edits));
                    }
                    MKind::Field { readonly: false, key, optional, ty: Some(Ty { kind: Kind::Func(f), .. }) } if method_style && f.style != Style::New && f.style != Style::AbstractNew => {
                        let mut edits = Vec::new();
                        if fix && f.ret.is_some() {
                            edits.push(TextEdit::new(abs(m.start), abs(m.end), format!("{}{}{}", key.raw, if *optional { "?" } else { "" }, pr.sig(f))));
                        }
                        out.push(find(abs(m.start), abs(m.end), "Found function property; prefer method signature.", edits));
                    }
                    _ => {}
                }
            }
        });
    }
});

jsdoc_rule!(TsNoUnnecessaryTemplateExpression, "js/jsdoc-ts-no-unnecessary-template-expression", |n, b, ctx, out| {
    let _ = n;
    let fix = opt_bool(ctx.options, "enable_fixer", true);
    for (t, text, ty) in typed_tags(&b) {
        types::walk(&ty, &mut |nd, parent| {
            let Kind::Template { parts, interps } = &nd.kind else { return };
            let (s, e) = (t.type_start + nd.start, t.type_start + nd.end);
            if let Some(k) = interps.iter().position(|i| matches!(i.kind, Kind::Str(_))) {
                let Kind::Str(raw) = &interps[k].kind else { return };
                let inner = &raw[1..raw.len() - 1];
                let mut edits = Vec::new();
                if fix && !inner.contains(['`', '\\']) && !inner.contains("${") {
                    let mut body = String::from("`");
                    for (i, p) in parts.iter().enumerate() {
                        if i == k {
                            body.push_str(p);
                            body.push_str(inner);
                            continue;
                        }
                        body.push_str(p);
                        if i < interps.len() && i != k {
                            body.push_str("${");
                            body.push_str(&text[interps[i].start..interps[i].end]);
                            body.push('}');
                        }
                    }
                    body.push('`');
                    edits.push(TextEdit::new(s, e, body));
                }
                out.push(find(s, e, "Found an unnecessary string literal within a template.", edits));
            } else if parts.len() == 2 && parts[0].is_empty() && parts[1].is_empty() && interps.len() == 1 {
                let inner = &interps[0];
                let mut edits = Vec::new();
                if fix {
                    let src = &text[inner.start..inner.end];
                    let wrap = matches!(inner.kind, Kind::Union(_) | Kind::Inter(_) | Kind::Func(_)) && needs_parens_outside(parent);
                    edits.push(TextEdit::new(s, e, if wrap { format!("({src})") } else { src.to_owned() }));
                }
                out.push(find(s, e, "Found a lone template expression within a template.", edits));
            }
        });
    }
});

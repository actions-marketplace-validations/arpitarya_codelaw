//! More html-validate rules (W-139): naming patterns, links, deprecated
//! classes, unknown elements and attributes, CSP nonces, `<style>`, SVG focus.

use crate::html_dom::{finding, finding_with, tokens_with_offsets};
use crate::html_rule;
use codelaw_core::TextEdit;

fn opt_pattern(ctx: &crate::NativeContext<'_>, default: &str) -> Option<regex::Regex> {
    regex::Regex::new(ctx.options.get("pattern").and_then(|v| v.as_str()).unwrap_or(default)).ok()
}

fn opt_bool(ctx: &crate::NativeContext<'_>, key: &str, default: bool) -> bool {
    ctx.options.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

const KEBAB: &str = "^[a-z][a-z0-9]*(-[a-z0-9]+)*$";

html_rule!(AllowedLinks, "html/allowed-links", |dom, meta, ctx, out| {
    let (ext, rel, abs) = (opt_bool(ctx, "allow_external", true), opt_bool(ctx, "allow_relative", true), opt_bool(ctx, "allow_absolute", true));
    if ext && rel && abs {
        return;
    }
    for e in &dom.elems {
        for a in e.attrs.iter().filter(|a| matches!(a.name.as_str(), "href" | "src" | "action" | "poster" | "data")) {
            let v = a.value_or_empty();
            if v.is_empty() || a.is_dynamic() || v.starts_with('#') || v.starts_with("mailto:") || v.starts_with("tel:") || v.starts_with("data:") || v.starts_with("javascript:") {
                continue;
            }
            let kind = if v.contains("://") || v.starts_with("//") {
                (!ext).then_some("external")
            } else if v.starts_with('/') {
                (!abs).then_some("absolute")
            } else {
                (!rel).then_some("relative")
            };
            if let Some(k) = kind {
                out.push(finding(a.value_start, a.value_end, format!("This project allows no {k} links: `{v}`")));
            }
        }
    }
});

html_rule!(AttrPattern, "html/attr-pattern", |dom, meta, ctx, out| {
    let Some(re) = opt_pattern(ctx, "^[a-z0-9-:]+$") else { return };
    for e in &dom.elems {
        for a in &e.attrs {
            if !re.is_match(&a.name) {
                out.push(finding(a.name_start, a.name_end, format!("The attribute name `{}` doesn't match the pattern `{}`", a.name, re.as_str())));
            }
        }
    }
});

fn token_pattern(dom: &crate::html_dom::Dom, ctx: &crate::NativeContext<'_>, attr: &str, default: &str, what: &str, out: &mut Vec<crate::NativeFinding>) {
    let Some(re) = opt_pattern(ctx, default) else { return };
    for e in &dom.elems {
        for a in e.attrs.iter().filter(|a| a.name == attr && !a.is_dynamic()) {
            for (s, t, tok) in tokens_with_offsets(a.value_or_empty(), a.value_start) {
                if !re.is_match(tok) {
                    out.push(finding(s, t, format!("The {what} `{tok}` doesn't match the pattern `{}`", re.as_str())));
                }
            }
        }
    }
}

html_rule!(ClassPattern, "html/class-pattern", |dom, meta, ctx, out| {
    token_pattern(dom, ctx, "class", KEBAB, "class", out);
});

html_rule!(IdPattern, "html/id-pattern", |dom, meta, ctx, out| {
    token_pattern(dom, ctx, "id", KEBAB, "id", out);
});

html_rule!(NamePattern, "html/name-pattern", |dom, meta, ctx, out| {
    let Some(re) = opt_pattern(ctx, "^[a-z][a-z0-9]*([-_][a-z0-9]+)*(\\[\\])?$") else { return };
    for e in &dom.elems {
        if !matches!(e.tag.as_str(), "input" | "select" | "textarea" | "button" | "output" | "fieldset" | "object") {
            continue;
        }
        for a in e.attrs.iter().filter(|a| a.name == "name" && !a.is_dynamic()) {
            let v = a.value_or_empty();
            if !v.is_empty() && !re.is_match(v) {
                out.push(finding(a.value_start, a.value_end, format!("The form field name `{v}` doesn't match the pattern `{}`", re.as_str())));
            }
        }
    }
});

html_rule!(DeprecatedClass, "html/deprecated-class", |dom, meta, ctx, out| {
    let Some(serde_json::Value::Object(map)) = ctx.options.get("classes") else { return };
    let entries: Vec<(String, Option<String>, Option<String>)> = map.iter().map(|(k, v)| (k.clone(), v.as_str().filter(|r| !r.is_empty()).map(str::to_owned), None)).collect();
    for e in &dom.elems {
        for a in e.attrs.iter().filter(|a| a.name == "class" && !a.is_dynamic()) {
            for (s, t, tok) in tokens_with_offsets(a.value_or_empty(), a.value_start) {
                let Some((_, rep, msg)) = entries.iter().find(|x| x.0 == tok) else { continue };
                let tail = msg.as_ref().map(|m| format!(": {m}")).unwrap_or_default();
                let edits = rep.as_ref().map(|r| vec![TextEdit::new(s, t, r.clone())]).unwrap_or_default();
                out.push(finding_with(s, t, format!("The class `{tok}` is deprecated{tail}"), edits));
            }
        }
    }
});

html_rule!(NoStyleTag, "html/no-style-tag", |dom, meta, ctx, out| {
    for i in dom.by_tag("style") {
        out.push(dom.at_elem(i, "Move these styles into a stylesheet and link it: no `<style>` element"));
    }
});

fn known_attr(meta: &crate::html_dom::Meta<'_>, tag: &str, name: &str) -> bool {
    if meta.attr(tag, name).is_some() {
        return true;
    }
    ["data-", "aria-", "on", "xml:", "xmlns:"].iter().any(|p| name.starts_with(p))
}

html_rule!(NoUnknownAttributes, "html/no-unknown-attributes", |dom, meta, ctx, out| {
    if !meta.known() {
        return;
    }
    for e in &dom.elems {
        if e.in_foreign || e.implied || e.tag.contains('-') || meta.elem(&e.tag).is_none() {
            continue;
        }
        for a in &e.attrs {
            if a.name.contains(['{', '}', '[', '(', '@', ':', '*', '#']) && !a.name.starts_with("xml") {
                continue; // template syntax
            }
            if !known_attr(&meta, &e.tag, &a.name) {
                out.push(finding(a.name_start, a.name_end, format!("`{}` is not an attribute of `<{}>`", a.name, e.tag)));
            }
        }
    }
});

html_rule!(NoUnknownElements, "html/no-unknown-elements", |dom, meta, ctx, out| {
    if !meta.known() {
        return;
    }
    for e in &dom.elems {
        if e.in_foreign || e.implied || e.tag.contains('-') || e.tag.contains(['{', '%', ':']) {
            continue;
        }
        if meta.elem(&e.tag).is_none() {
            out.push(finding(e.start, e.start + 1 + e.tag.len(), format!("`<{}>` is not an HTML element", e.tag)));
        }
    }
});

html_rule!(RequireCspNonce, "html/require-csp-nonce", |dom, meta, ctx, out| {
    for e in &dom.elems {
        let inline_script = e.tag == "script" && !e.has("src");
        if (inline_script || e.tag == "style") && !e.has("nonce") {
            out.push(finding(e.start, e.open_end, format!("This inline `<{}>` has no `nonce`, so a Content-Security-Policy that requires one blocks it", e.tag)));
        }
    }
});

html_rule!(SvgFocusable, "html/svg-focusable", |dom, meta, ctx, out| {
    for i in dom.by_tag("svg") {
        let e = &dom.elems[i];
        if e.has("focusable") {
            continue;
        }
        let at = e.start + 4;
        out.push(finding_with(e.start, e.open_end, "Give the `<svg>` `focusable=\"false\"`: older browsers otherwise put it in the tab order", vec![TextEdit::insert(at, " focusable=\"false\"")]));
    }
});

pub static RULES: &[&dyn crate::NativeRule] = &[
    &AllowedLinks, &AttrPattern, &ClassPattern, &IdPattern, &NamePattern, &DeprecatedClass, &NoStyleTag, &NoUnknownAttributes, &NoUnknownElements, &RequireCspNonce, &SvgFocusable,
];

//! More stylelint-scss rules (W-139), on the SCSS reader the `scss/*` rules
//! share (`scss_rules`).

use crate::css_syntax::{self, Kind, Sheet, VKind, VNode, finding, fixed, opt_list, skip_string, text};
use crate::scss_rules::{pattern_option, sass_name, sass_statements, scss_rule, split_top, use_edit};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use std::collections::{BTreeMap, BTreeSet};

fn params(sheet: &Sheet, i: usize, src: &str) -> String {
    text(src, sheet.nodes[i].params).to_owned()
}

/// The quoted URLs of a load statement, with their ranges (inside the quotes).
fn load_urls(sheet: &Sheet, i: usize, src: &str) -> Vec<(String, usize, usize)> {
    let n = &sheet.nodes[i];
    let p = text(src, n.params);
    let b = p.as_bytes();
    let mut out = Vec::new();
    let mut j = 0;
    while j < b.len() {
        if b[j] == b'"' || b[j] == b'\'' {
            let e = skip_string(b, j);
            let (s, t) = (j + 1, e.saturating_sub(1).max(j + 1));
            out.push((p[s..t].to_owned(), n.params.0 + s, n.params.0 + t));
            j = e;
        } else {
            j += 1;
        }
    }
    out
}

fn ext_of(url: &str) -> Option<&str> {
    let base = url.rsplit('/').next()?;
    base.rfind('.').map(|d| &base[d + 1..])
}

fn import_ext_list(sheet: &Sheet, src: &str, ctx: &NativeContext<'_>, allow: bool, out: &mut Vec<NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for (i, _) in sass_statements(sheet, src, &["import"]) {
        for (u, s, e) in load_urls(sheet, i, src) {
            if u.contains("://") || u.starts_with("url(") {
                continue;
            }
            if let Some(x) = ext_of(&u)
                && list.iter().any(|l| l.eq_ignore_ascii_case(x)) != allow
            {
                let v = if allow { "is not in the allowed list" } else { "is disallowed" };
                out.push(finding(s, e, format!("The extension `.{x}` in an `@import` {v}")));
            }
        }
    }
}

scss_rule!(AtImportPartialExtensionAllowedList, "scss/at-import-partial-extension-allowed-list", |sheet, src, ctx, out| {
    import_ext_list(sheet, src, ctx, true, out);
});
scss_rule!(AtImportPartialExtensionDisallowedList, "scss/at-import-partial-extension-disallowed-list", |sheet, src, ctx, out| {
    import_ext_list(sheet, src, ctx, false, out);
});

/// Parameter names of the `@mixin`s or `@function`s defined in the file.
fn definitions(sheet: &Sheet, src: &str, kind: &str) -> BTreeMap<String, Vec<String>> {
    let mut m = BTreeMap::new();
    for (i, _) in sass_statements(sheet, src, &[kind]) {
        let p = params(sheet, i, src);
        let (name, args) = match p.find('(') {
            Some(o) => (p[..o].trim().to_owned(), p[o + 1..p.rfind(')').unwrap_or(p.len())].to_owned()),
            None => (p.trim().to_owned(), String::new()),
        };
        let names = split_top(&args).iter().filter_map(|a| a.trim().split([':', ' ']).next().filter(|x| x.starts_with('$')).map(str::to_owned)).collect();
        m.insert(name, names);
    }
    m
}

/// A call's name, its argument list's range, and its arguments.
fn call_args(src: &str, start: usize, end: usize) -> Option<(String, usize, usize)> {
    let t = &src[start..end];
    let o = t.find('(')?;
    let c = t.rfind(')')?;
    Some((t[..o].trim().to_owned(), start + o + 1, start + c))
}

#[allow(clippy::too_many_arguments)]
fn named_args(src: &str, defs: &BTreeMap<String, Vec<String>>, name: &str, a: usize, b: usize, what: &str, at: (usize, usize), out: &mut Vec<NativeFinding>) {
    let inner = &src[a..b];
    if inner.trim().is_empty() {
        return;
    }
    let parts = split_top(inner);
    if parts.iter().all(|p| p.trim_start().starts_with('$') && p.contains(':')) || parts.iter().any(|p| p.contains("...")) {
        return;
    }
    let mut edits = Vec::new();
    if let Some(params) = defs.get(name) {
        let mut off = a;
        for (k, p) in parts.iter().enumerate() {
            let lead = p.len() - p.trim_start().len();
            if !(p.trim_start().starts_with('$') && p.contains(':'))
                && let Some(pn) = params.get(k)
            {
                edits.push(TextEdit::insert(off + lead, format!("{pn}: ")));
            }
            off += p.len() + 1;
        }
        if params.len() < parts.len() {
            edits.clear();
        }
    }
    out.push(fixed(at.0, at.1, format!("Pass the arguments of the {what} `{name}` by name"), edits));
}

scss_rule!(AtFunctionNamedArguments, "scss/at-function-named-arguments", |sheet, src, ctx, out| {
    let defs = definitions(sheet, src, "function");
    if defs.is_empty() {
        return;
    }
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if !matches!(n.kind, Kind::Decl | Kind::Other) || sass_name(sheet, i, src).is_some_and(|x| x == "function" || x == "mixin") {
            continue;
        }
        fn go(list: &[VNode], src: &str, defs: &BTreeMap<String, Vec<String>>, out: &mut Vec<NativeFinding>) {
            for v in list {
                if v.kind == VKind::Func {
                    let name = v.name(src);
                    if defs.contains_key(name) {
                        named_args(src, defs, name, v.inner.0, v.inner.1, "function", (v.start, v.end), out);
                    }
                    go(&v.nodes, src, defs, out);
                }
            }
        }
        go(&sheet.values(i, src), src, &defs, out);
    }
});

scss_rule!(AtMixinNamedArguments, "scss/at-mixin-named-arguments", |sheet, src, ctx, out| {
    let defs = definitions(sheet, src, "mixin");
    for (i, _) in sass_statements(sheet, src, &["include"]) {
        let n = &sheet.nodes[i];
        let Some((name, a, b)) = call_args(src, n.params.0, n.params.1) else { continue };
        if defs.contains_key(&name) {
            named_args(src, &defs, &name, a, b, "mixin", (n.start, n.params.1), out);
        }
    }
});

scss_rule!(AtFunctionPattern, "scss/at-function-pattern", |sheet, src, ctx, out| {
    let re = pattern_option(ctx);
    for (i, _) in sass_statements(sheet, src, &["function"]) {
        let n = &sheet.nodes[i];
        let p = params(sheet, i, src);
        let name = p.split('(').next().unwrap_or("").trim();
        if !name.is_empty() && !re.is_match(name) {
            out.push(finding(n.start, n.params.1.min(n.params.0 + p.find('(').unwrap_or(p.len())), format!("The function name `{name}` doesn't match the naming pattern (kebab-case by default)")));
        }
    }
});

scss_rule!(AtMixinArgumentlessCallParentheses, "scss/at-mixin-argumentless-call-parentheses", |sheet, src, ctx, out| {
    for (i, _) in sass_statements(sheet, src, &["include"]) {
        let n = &sheet.nodes[i];
        let p = text(src, n.params);
        if let Some(o) = p.find("()") {
            let before = &p[..o];
            if !before.contains(['(', ' ']) || before.split_whitespace().count() == 1 {
                let at = n.params.0 + o;
                out.push(fixed(at, at + 2, format!("`@include {}` takes no arguments: drop the `()`", before.trim()), vec![TextEdit::delete(at, at + 2)]));
            }
        }
    }
});

scss_rule!(AtMixinNoRiskyNestingSelector, "scss/at-mixin-no-risky-nesting-selector", |sheet, src, ctx, out| {
    for (i, _) in sass_statements(sheet, src, &["mixin"]) {
        let mut stack = sheet.nodes[i].children.clone();
        while let Some(c) = stack.pop() {
            let n = &sheet.nodes[c];
            if n.kind == Kind::Rule {
                let sel = text(src, n.name);
                if sel.split(',').any(|s| {
                    let s = s.trim();
                    s.contains('&') && !s.starts_with('&')
                }) {
                    out.push(finding(n.name.0, n.name.1, format!("`{sel}` puts `&` after another selector inside a mixin: where the mixin is included decides what it matches")));
                }
            }
            stack.extend(n.children.iter().copied());
        }
    }
});

scss_rule!(AtRootNoRedundant, "scss/at-root-no-redundant", |sheet, src, ctx, out| {
    for (i, _) in sass_statements(sheet, src, &["at-root"]) {
        let n = &sheet.nodes[i];
        let p = text(src, n.params);
        if p.starts_with('(') {
            continue;
        }
        let top = n.parent.is_none();
        if top || p.contains('&') {
            let end = n.params.0;
            let why = if top { "it is already at the root" } else { "its selector holds `&`" };
            out.push(fixed(n.start, n.name.1, format!("`@at-root` is redundant: {why}"), vec![TextEdit::delete(n.start, end)]));
        }
    }
});

scss_rule!(AtRuleConditionalNoParentheses, "scss/at-rule-conditional-no-parentheses", |sheet, src, ctx, out| {
    for (i, name) in sass_statements(sheet, src, &["if", "else", "while"]) {
        let n = &sheet.nodes[i];
        let p = text(src, n.params);
        let (off, cond) = if name == "else" {
            match p.strip_prefix("if") {
                Some(r) => (p.len() - r.len(), r),
                None => continue,
            }
        } else {
            (0, p)
        };
        let lead = cond.len() - cond.trim_start().len();
        let c = cond.trim();
        if c.starts_with('(') && c.ends_with(')') && matching_close(c) == Some(c.len() - 1) {
            let s = n.params.0 + off + lead;
            let e = s + c.len();
            let space = if name == "else" && lead == 0 { " " } else { "" };
            out.push(fixed(s, e, format!("Drop the parentheses around the `@{name}` condition"), vec![TextEdit::new(s, e, format!("{space}{}", c[1..c.len() - 1].trim()))]));
        }
    }
});

fn matching_close(s: &str) -> Option<usize> {
    let mut d = 0;
    for (k, c) in s.char_indices() {
        match c {
            '(' => d += 1,
            ')' => {
                d -= 1;
                if d == 0 {
                    return Some(k);
                }
            }
            _ => {}
        }
    }
    None
}

scss_rule!(AtUseNoRedundantAlias, "scss/at-use-no-redundant-alias", |sheet, src, ctx, out| {
    for (i, _) in sass_statements(sheet, src, &["use"]) {
        let n = &sheet.nodes[i];
        let p = text(src, n.params);
        let Some(k) = p.find(" as ") else { continue };
        let alias = p[k + 4..].split_whitespace().next().unwrap_or("");
        let Some((u, _, _)) = load_urls(sheet, i, src).into_iter().next() else { continue };
        let base = u.rsplit([':', '/']).next().unwrap_or(&u);
        let base = base.trim_start_matches('_');
        let base = base.split('.').next().unwrap_or(base);
        if alias == base {
            let s = n.params.0 + k;
            out.push(fixed(s, s + 4 + alias.len(), format!("`as {alias}` repeats the module's own name"), vec![TextEdit::delete(s, s + 4 + alias.len())]));
        }
    }
});

scss_rule!(BlockNoRedundantNesting, "scss/block-no-redundant-nesting", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Rule || n.children.len() != 1 {
            continue;
        }
        let c = n.children[0];
        let cn = &sheet.nodes[c];
        if cn.kind != Kind::Rule {
            continue;
        }
        let (outer, inner) = (text(src, n.name), text(src, cn.name));
        if outer.contains([',', '&', '#', '@']) || inner.contains([',', '&', '#', '@']) || inner.starts_with(['>', '+', '~']) {
            continue;
        }
        let Some(ob) = n.block else { continue };
        let Some(ib) = cn.block else { continue };
        if !src[ob.0 + 1..cn.start].trim().is_empty() || !src[ib.1 + 1..ob.1].trim().is_empty() {
            continue;
        }
        let edits = vec![TextEdit::new(n.name.1, ib.0, format!(" {inner} ")), TextEdit::delete(ib.1, ob.1 + 1)];
        let _ = edits;
        let body = &src[ib.0..=ib.1];
        let new = format!("{outer} {inner} {body}");
        out.push(fixed(n.start, ob.1 + 1, format!("`{outer}` holds only `{inner}`: write `{outer} {inner}`"), vec![TextEdit::new(n.start, ob.1 + 1, new)]));
    }
});

/// The `//` comments of the source: their ranges.
fn line_comments(src: &str) -> Vec<(usize, usize)> {
    crate::css_syntax::scss_line_comments(src).into_iter().collect()
}

scss_rule!(CommentNoLoud, "scss/comment-no-loud", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Comment || !src[n.start..].starts_with("/*") {
            continue;
        }
        let body = &src[n.start + 2..n.end.saturating_sub(2).max(n.start + 2)];
        if body.starts_with('!') {
            continue;
        }
        let edits = if body.contains('\n') { vec![] } else { vec![TextEdit::new(n.start, n.end, format!("// {}", body.trim()))] };
        out.push(fixed(n.start, n.end, "Use a silent `//` comment: a `/* */` comment is copied into the CSS", edits));
    }
});

scss_rule!(DeclarationNestedProperties, "scss/declaration-nested-properties", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind == Kind::Other && n.block.is_some() && text(src, n.name).ends_with(':') {
            out.push(finding(n.start, n.name.1, format!("Write the nested properties of `{}` as plain declarations", text(src, n.name).trim_end_matches(':'))));
        }
    }
});

scss_rule!(DeclarationNestedPropertiesNoDividedGroups, "scss/declaration-nested-properties-no-divided-groups", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        let mut seen = BTreeSet::new();
        for &c in &n.children {
            let cn = &sheet.nodes[c];
            if cn.kind == Kind::Other && cn.block.is_some() && text(src, cn.name).ends_with(':') {
                let k = text(src, cn.name).trim_end_matches(':').trim().to_owned();
                if !seen.insert(k.clone()) {
                    out.push(finding(cn.start, cn.name.1, format!("The nested properties of `{k}` are split into two groups: merge them")));
                }
            }
        }
    }
});

scss_rule!(DimensionNoNonNumericValues, "scss/dimension-no-non-numeric-values", |sheet, src, ctx, out| {
    let re = regex::Regex::new(r"#\{\s*(\$[\w-]+)\s*\}([a-zA-Z%]+)\b").expect("valid");
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if !matches!(n.kind, Kind::Decl | Kind::Other) {
            continue;
        }
        let p = text(src, n.params);
        for m in re.captures_iter(p) {
            let w = m.get(0).unwrap();
            let (s, e) = (n.params.0 + w.start(), n.params.0 + w.end());
            let new = format!("{} * 1{}", &m[1], &m[2]);
            out.push(fixed(s, e, format!("`{}` builds a dimension from text: multiply instead, `{new}`", w.as_str()), vec![TextEdit::new(s, e, new)]));
        }
    }
});

fn sass_var_decls(sheet: &Sheet, src: &str) -> Vec<usize> {
    sheet.walk().into_iter().filter(|&i| sheet.nodes[i].kind == Kind::Decl && text(src, sheet.nodes[i].name).starts_with('$')).collect()
}

scss_rule!(DollarVariableDefault, "scss/dollar-variable-default", |sheet, src, ctx, out| {
    for i in sass_var_decls(sheet, src) {
        let n = &sheet.nodes[i];
        if n.parent.is_some() {
            continue;
        }
        let whole = &src[n.start..n.end];
        if whole.contains("!default") {
            continue;
        }
        out.push(fixed(n.name.0, n.name.1, format!("`{}` has no `!default`, so a module using this one can't configure it", text(src, n.name)), vec![TextEdit::insert(n.end, " !default")]));
    }
});

scss_rule!(DollarVariableFirstInBlock, "scss/dollar-variable-first-in-block", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let kids = &sheet.nodes[i].children;
        let mut other = false;
        for &c in kids {
            let cn = &sheet.nodes[c];
            let is_var = cn.kind == Kind::Decl && text(src, cn.name).starts_with('$');
            if cn.kind == Kind::Comment {
                continue;
            }
            if is_var && other {
                out.push(finding(cn.name.0, cn.name.1, format!("Declare `{}` at the start of its block", text(src, cn.name))));
            }
            if !is_var {
                other = true;
            }
        }
    }
});

scss_rule!(DollarVariableNoMissingInterpolation, "scss/dollar-variable-no-missing-interpolation", |sheet, src, ctx, out| {
    let re = regex::Regex::new(r"(^|[^{\w-])(\$[\w-]+)").expect("valid");
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::AtRule && n.kind != Kind::Other {
            continue;
        }
        let name = text(src, n.name).to_ascii_lowercase();
        if !matches!(name.as_str(), "keyframes" | "-webkit-keyframes" | "counter-style" | "font-feature-values" | "supports") {
            continue;
        }
        let p = text(src, n.params);
        for m in re.captures_iter(p) {
            let v = m.get(2).unwrap();
            if p[..v.start()].ends_with("#{") {
                continue;
            }
            let (s, e) = (n.params.0 + v.start(), n.params.0 + v.end());
            out.push(fixed(s, e, format!("`{}` in `@{name}` is written as its name, not its value: interpolate it", v.as_str()), vec![TextEdit::new(s, e, format!("#{{{}}}", v.as_str()))]));
        }
    }
});

scss_rule!(DoubleSlashCommentInline, "scss/double-slash-comment-inline", |sheet, src, ctx, out| {
    for (s, e) in line_comments(ctx.source) {
        let ls = ctx.source[..s].rfind('\n').map(|x| x + 1).unwrap_or(0);
        let before = &ctx.source[ls..s];
        if before.trim().is_empty() {
            continue;
        }
        let indent: String = before.chars().take_while(|c| c.is_whitespace()).collect();
        let comment = ctx.source[s..e].trim_end();
        let code_end = ls + before.trim_end().len();
        let edits = vec![TextEdit::insert(ls, format!("{indent}{comment}\n")), TextEdit::delete(code_end, s + comment.len())];
        out.push(fixed(s, s + comment.len(), "Put the `//` comment on its own line", edits));
    }
});

scss_rule!(FunctionCalculationNoInterpolation, "scss/function-calculation-no-interpolation", |sheet, src, ctx, out| {
    let re = regex::Regex::new(r"#\{\s*(\$[\w-]+)\s*\}").expect("valid");
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Decl {
            continue;
        }
        fn go(list: &[VNode], src: &str, re: &regex::Regex, out: &mut Vec<NativeFinding>) {
            for v in list {
                if v.kind != VKind::Func {
                    continue;
                }
                let name = v.name(src).to_ascii_lowercase();
                if matches!(name.as_str(), "calc" | "clamp" | "min" | "max") {
                    let inner = &src[v.inner.0..v.inner.1];
                    for m in re.captures_iter(inner) {
                        let w = m.get(0).unwrap();
                        let (s, e) = (v.inner.0 + w.start(), v.inner.0 + w.end());
                        out.push(fixed(s, e, format!("`{name}()` takes the variable directly: write `{}`", &m[1]), vec![TextEdit::new(s, e, m[1].to_owned())]));
                    }
                } else {
                    go(&v.nodes, src, re, out);
                }
            }
        }
        go(&sheet.values(i, src), src, &re, out);
    }
});

/// Sass function calls in values, by name: `(name, call range, args range)`.
fn sass_calls(sheet: &Sheet, src: &str) -> Vec<(String, usize, usize, usize, usize)> {
    let mut out = Vec::new();
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if !matches!(n.kind, Kind::Decl | Kind::Other) {
            continue;
        }
        let p = text(src, n.params);
        let b = p.as_bytes();
        let mut k = 0;
        while k < b.len() {
            if b[k] == b'"' || b[k] == b'\'' {
                k = skip_string(b, k);
                continue;
            }
            if (b[k].is_ascii_alphabetic() || b[k] == b'-') && (k == 0 || !(b[k - 1].is_ascii_alphanumeric() || b[k - 1] == b'-' || b[k - 1] == b'_' || b[k - 1] == b'$')) {
                let s = k;
                while k < b.len() && (b[k].is_ascii_alphanumeric() || b[k] == b'-' || b[k] == b'_' || b[k] == b'.') {
                    k += 1;
                }
                if k < b.len() && b[k] == b'('
                    && let Some(c) = matching_close(&p[k..]) {
                        out.push((p[s..k].to_owned(), n.params.0 + s, n.params.0 + k + c + 1, n.params.0 + k + 1, n.params.0 + k + c));
                    }
                continue;
            }
            k += 1;
        }
    }
    out
}

scss_rule!(FunctionColorChannel, "scss/function-color-channel", |sheet, src, ctx, out| {
    let channels = [("red", "rgb"), ("green", "rgb"), ("blue", "rgb"), ("hue", "hsl"), ("saturation", "hsl"), ("lightness", "hsl"), ("whiteness", "hwb"), ("blackness", "hwb")];
    for (name, s, e, a, b) in sass_calls(sheet, src) {
        let bare = name.strip_prefix("color.").unwrap_or(&name);
        let Some((ch, space)) = channels.iter().find(|x| x.0 == bare) else { continue };
        if !name.starts_with("color.") && !matches!(bare, "red" | "green" | "blue" | "hue" | "saturation" | "lightness") {
            continue;
        }
        let arg = &src[a..b];
        let mut edits = vec![TextEdit::new(s, e, format!("color.channel({arg}, \"{ch}\", $space: {space})"))];
        if let Some(u) = use_edit(sheet, ctx.source, "color") {
            edits.push(u);
        }
        out.push(fixed(s, e, format!("`{name}()` is deprecated: read the channel with `color.channel()`"), edits));
    }
});

scss_rule!(FunctionColorRelative, "scss/function-color-relative", |sheet, src, ctx, out| {
    let map = [("lighten", "lightness", ""), ("darken", "lightness", "-"), ("saturate", "saturation", ""), ("desaturate", "saturation", "-"), ("opacify", "alpha", ""), ("fade-in", "alpha", ""), ("transparentize", "alpha", "-"), ("fade-out", "alpha", "-")];
    for (name, s, e, a, b) in sass_calls(sheet, src) {
        let Some((_, ch, sign)) = map.iter().find(|x| x.0 == name) else { continue };
        let args = split_top(&src[a..b]);
        let [c, amount] = args.as_slice() else { continue };
        let amount = amount.trim();
        let amount = if sign.is_empty() { amount.to_owned() } else if let Some(x) = amount.strip_prefix('-') { x.to_owned() } else { format!("-{amount}") };
        let mut edits = vec![TextEdit::new(s, e, format!("color.adjust({}, ${ch}: {amount})", c.trim()))];
        if let Some(u) = use_edit(sheet, ctx.source, "color") {
            edits.push(u);
        }
        out.push(fixed(s, e, format!("`{name}()` is a legacy colour function: use `color.adjust()`"), edits));
    }
});

scss_rule!(FunctionDisallowedList, "scss/function-disallowed-list", |sheet, src, ctx, out| {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for (name, s, _, _, _) in sass_calls(sheet, src) {
        if crate::css_syntax::matches_any(&list, &name) {
            out.push(finding(s, s + name.len(), format!("The function `{name}()` is disallowed")));
        }
    }
});

scss_rule!(FunctionQuoteNoQuotedStringsInside, "scss/function-quote-no-quoted-strings-inside", |sheet, src, ctx, out| {
    for (name, s, e, a, b) in sass_calls(sheet, src) {
        if name != "quote" && name != "string.quote" {
            continue;
        }
        let arg = src[a..b].trim();
        if (arg.starts_with('"') && arg.ends_with('"') || arg.starts_with('\'') && arg.ends_with('\'')) && arg.len() >= 2 {
            out.push(fixed(s, e, format!("`{name}()` of a quoted string does nothing: write `{arg}`"), vec![TextEdit::new(s, e, arg.to_owned())]));
        }
    }
});

scss_rule!(FunctionUnquoteNoUnquotedStringsInside, "scss/function-unquote-no-unquoted-strings-inside", |sheet, src, ctx, out| {
    for (name, s, e, a, b) in sass_calls(sheet, src) {
        if name != "unquote" && name != "string.unquote" {
            continue;
        }
        let arg = src[a..b].trim();
        if !arg.is_empty() && arg.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') && !arg.starts_with(|c: char| c.is_ascii_digit()) {
            out.push(fixed(s, e, format!("`{name}()` of an unquoted string does nothing: write `{arg}`"), vec![TextEdit::new(s, e, arg.to_owned())]));
        }
    }
});

scss_rule!(LoadPartialExtension, "scss/load-partial-extension", |sheet, src, ctx, out| {
    for (i, _) in sass_statements(sheet, src, &["use", "forward", "import"]) {
        for (u, s, e) in load_urls(sheet, i, src) {
            if u.contains("://") || u.starts_with("sass:") {
                continue;
            }
            if let Some(x) = ext_of(&u)
                && (x == "scss" || x == "sass")
            {
                out.push(fixed(s, e, format!("Load `{u}` without its `.{x}` extension"), vec![TextEdit::delete(e - x.len() - 1, e)]));
            }
        }
    }
});

scss_rule!(MapKeysQuotes, "scss/map-keys-quotes", |sheet, src, ctx, out| {
    for i in sass_var_decls(sheet, src) {
        let n = &sheet.nodes[i];
        let v = text(src, n.params);
        let t = v.trim();
        if !t.starts_with('(') || !t.contains(':') {
            continue;
        }
        let base = n.params.0 + (v.len() - v.trim_start().len()) + 1;
        let inner = &t[1..t.rfind(')').unwrap_or(t.len())];
        let mut off = 0;
        for part in split_top(inner) {
            let lead = part.len() - part.trim_start().len();
            let key = part.trim().split(':').next().unwrap_or("").trim();
            if !key.is_empty() && part.contains(':') && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') && !key.starts_with(|c: char| c.is_ascii_digit()) {
                let s = base + off + lead;
                out.push(fixed(s, s + key.len(), format!("Quote the map key `{key}`"), vec![TextEdit::new(s, s + key.len(), format!("\"{key}\""))]));
            }
            off += part.len() + 1;
        }
    }
});

scss_rule!(MediaFeatureValueDollarVariable, "scss/media-feature-value-dollar-variable", |sheet, src, ctx, out| {
    for (_, _, value) in crate::css_lists::media_features(sheet, src) {
        let Some(v) = value else { continue };
        let t = &src[v.0..v.1];
        if !t.starts_with('$') && !t.contains("#{") {
            out.push(finding(v.0, v.1, format!("Write the media feature value `{t}` as a `$` variable")));
        }
    }
});

scss_rule!(NoDollarVariables, "scss/no-dollar-variables", |sheet, src, ctx, out| {
    for i in sass_var_decls(sheet, src) {
        let n = &sheet.nodes[i];
        out.push(finding(n.name.0, n.name.1, format!("`{}`: this project uses no Sass variables", text(src, n.name))));
    }
});

scss_rule!(NoDuplicateLoadRules, "scss/no-duplicate-load-rules", |sheet, src, ctx, out| {
    let mut seen = BTreeSet::new();
    for (i, name) in sass_statements(sheet, src, &["use", "forward", "import"]) {
        let n = &sheet.nodes[i];
        for (u, s, e) in load_urls(sheet, i, src) {
            if !seen.insert((name.clone(), u.clone())) {
                let end = n.semicolon.map(|x| x + 1).unwrap_or(n.end);
                let tail = if src[end..].starts_with('\n') { end + 1 } else { end };
                let edits = if load_urls(sheet, i, src).len() == 1 { vec![TextEdit::delete(n.start, tail)] } else { vec![] };
                out.push(fixed(s, e, format!("`{u}` is loaded twice by `@{name}`"), edits));
            }
        }
    }
});

scss_rule!(NoUnusedPrivateMembers, "scss/no-unused-private-members", |sheet, src, ctx, out| {
    let mut defs: Vec<(String, usize, usize, &str)> = Vec::new();
    for i in sass_var_decls(sheet, src) {
        let n = &sheet.nodes[i];
        let name = text(src, n.name);
        if name.starts_with("$-") || name.starts_with("$_") {
            defs.push((name.to_owned(), n.name.0, n.name.1, "variable"));
        }
    }
    for (i, kind) in sass_statements(sheet, src, &["mixin", "function"]) {
        let n = &sheet.nodes[i];
        let p = params(sheet, i, src);
        let name = p.split('(').next().unwrap_or("").trim().to_owned();
        if name.starts_with('-') || name.starts_with('_') {
            defs.push((name, n.start, n.params.0 + p.find('(').unwrap_or(p.len()), if kind == "mixin" { "mixin" } else { "function" }));
        }
    }
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind == Kind::Rule {
            for s in text(src, n.name).split(|c: char| c.is_whitespace() || c == ',') {
                if s.starts_with("%-") || s.starts_with("%_") {
                    defs.push((s.to_owned(), n.name.0, n.name.1, "placeholder"));
                }
            }
        }
    }
    for (name, s, e, kind) in defs {
        let needle = match kind {
            "placeholder" => format!("@extend {name}"),
            _ => name.clone(),
        };
        let uses = src.matches(needle.as_str()).count();
        let own = if kind == "placeholder" { 0 } else { 1 };
        if uses <= own {
            out.push(finding(s, e, format!("The private {kind} `{name}` is never used in this file")));
        }
    }
});

scss_rule!(PartialNoImport, "scss/partial-no-import", |sheet, src, ctx, out| {
    let file = ctx.file.path().rsplit('/').next().unwrap_or("");
    if !file.starts_with('_') {
        return;
    }
    for (i, _) in sass_statements(sheet, src, &["import"]) {
        let n = &sheet.nodes[i];
        out.push(finding(n.start, n.name.1, "A partial doesn't `@import`: the file that loads it does"));
    }
});

scss_rule!(SelectorNestCombinators, "scss/selector-nest-combinators", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Rule {
            continue;
        }
        let list = sheet.selectors(i, src);
        let [c] = list.as_ref() else { continue };
        if c.compounds.len() != 2 || text(src, n.name).contains(['#', '@', '&']) {
            continue;
        }
        let Some(b) = n.block else { continue };
        let first = text(src, (c.compounds[0].start, c.compounds[0].end));
        let second = text(src, (c.compounds[1].start, c.compounds[1].end));
        let comb = c.combinators.first().map(|x| x.trim()).unwrap_or("");
        let inner = if comb.is_empty() { second.to_owned() } else { format!("{comb} {second}") };
        let body = &src[b.0..(b.1 + 1).min(src.len())];
        let ls = src[..n.start].rfind('\n').map(|x| x + 1).unwrap_or(0);
        let indent: String = src[ls..n.start].chars().take_while(|c| c.is_whitespace()).collect();
        let body = body.replace('\n', "\n  ");
        let new = format!("{first} {{\n{indent}  {inner} {body}\n{indent}}}");
        out.push(fixed(n.name.0, b.1 + 1, format!("Nest `{inner}` inside `{first}`"), vec![TextEdit::new(n.start, b.1 + 1, new)]));
    }
});

scss_rule!(SelectorNoRedundantNestingSelector, "scss/selector-no-redundant-nesting-selector", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Rule || n.parent.is_none_or(|p| sheet.nodes[p].kind != Kind::Rule) {
            continue;
        }
        let sel = text(src, n.name);
        if sel.contains(',') {
            continue;
        }
        if let Some(rest) = sel.strip_prefix('&')
            && rest.starts_with(char::is_whitespace)
            && !rest.contains('&')
        {
            let r = rest.trim_start();
            if r.is_empty() {
                continue;
            }
            let s = n.name.0;
            let e = s + (sel.len() - r.len());
            out.push(fixed(s, e, format!("The `&` in `{sel}` is redundant"), vec![TextEdit::delete(s, e)]));
        }
    }
});

scss_rule!(SelectorNoUnionClassName, "scss/selector-no-union-class-name", |sheet, src, ctx, out| {
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Rule {
            continue;
        }
        let sel = text(src, n.name);
        for (k, _) in sel.match_indices('&') {
            if sel[k + 1..].starts_with(|c: char| c == '-' || c == '_' || c.is_ascii_alphanumeric()) {
                let s = n.name.0 + k;
                let e = s + 1 + sel[k + 1..].find(|c: char| !(c == '-' || c == '_' || c.is_ascii_alphanumeric())).unwrap_or(sel.len() - k - 1);
                out.push(finding(s, e, format!("`{}` builds a class name from its parent's: write the class out", &src[s..e])));
            }
        }
    }
});

pub static RULES: &[&dyn NativeRule] = &[
    &AtImportPartialExtensionAllowedList, &AtImportPartialExtensionDisallowedList, &AtFunctionNamedArguments, &AtMixinNamedArguments, &AtFunctionPattern,
    &AtMixinArgumentlessCallParentheses, &AtMixinNoRiskyNestingSelector, &AtRootNoRedundant, &AtRuleConditionalNoParentheses, &AtUseNoRedundantAlias,
    &BlockNoRedundantNesting, &CommentNoLoud, &DeclarationNestedProperties, &DeclarationNestedPropertiesNoDividedGroups, &DimensionNoNonNumericValues,
    &DollarVariableDefault, &DollarVariableFirstInBlock, &DollarVariableNoMissingInterpolation, &DoubleSlashCommentInline, &FunctionCalculationNoInterpolation,
    &FunctionColorChannel, &FunctionColorRelative, &FunctionDisallowedList, &FunctionQuoteNoQuotedStringsInside, &FunctionUnquoteNoUnquotedStringsInside,
    &LoadPartialExtension, &MapKeysQuotes, &MediaFeatureValueDollarVariable, &NoDollarVariables, &NoDuplicateLoadRules, &NoUnusedPrivateMembers,
    &PartialNoImport, &SelectorNestCombinators, &SelectorNoRedundantNestingSelector, &SelectorNoUnionClassName,
];

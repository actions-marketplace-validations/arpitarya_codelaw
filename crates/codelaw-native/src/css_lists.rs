//! stylelint's allow- and deny-list rules, and its limits (W-139): each does
//! nothing until its option is set, as upstream, except the `selector-max-*`
//! limits, which carry a default.

use crate::css_syntax::{Kind, SKind, Sheet, Simple, VKind, VNode, all_simples, css_rule, dimension, finding, matches_any, non_standard, opt_list, specificity, text, unprefixed};
use std::collections::BTreeMap;

pub(crate) fn style_rules(sheet: &Sheet, src: &str) -> Vec<usize> {
    sheet.walk().into_iter().filter(|&i| sheet.nodes[i].kind == Kind::Rule && !sheet.in_keyframes(i, src) && !non_standard(text(src, sheet.nodes[i].name))).collect()
}

pub(crate) fn prop(sheet: &Sheet, i: usize, src: &str) -> String {
    text(src, sheet.nodes[i].name).to_ascii_lowercase()
}

/// Every value node of a declaration or prelude, depth first.
pub(crate) fn each_value<'a>(list: &'a [VNode], f: &mut dyn FnMut(&'a VNode)) {
    VNode::walk(list, &mut |n| {
        f(n);
        true
    });
}

/// An option holding an object of lists: `{ "key": ["a", "/re/"] }`.
fn opt_map(ctx: &crate::NativeContext<'_>, key: &str) -> BTreeMap<String, Vec<String>> {
    let mut m = BTreeMap::new();
    if let Some(serde_json::Value::Object(o)) = ctx.options.get(key) {
        for (k, v) in o {
            let list = match v {
                serde_json::Value::Array(a) => a.iter().filter_map(|x| x.as_str().map(str::to_owned).or_else(|| x.as_f64().map(|n| n.to_string()))).collect(),
                serde_json::Value::String(s) => vec![s.clone()],
                serde_json::Value::Number(n) => vec![n.to_string()],
                _ => continue,
            };
            m.insert(k.clone(), list);
        }
    }
    m
}

/// The list in `map` whose key matches `name` (a key may be `/re/`).
fn lookup<'m>(map: &'m BTreeMap<String, Vec<String>>, name: &str) -> Option<&'m Vec<String>> {
    map.iter().find(|(k, _)| matches_any(std::slice::from_ref(k), name)).map(|(_, v)| v)
}

fn opt_int(ctx: &crate::NativeContext<'_>, key: &str, default: i64) -> i64 {
    ctx.options.get(key).and_then(|v| v.as_i64()).unwrap_or(default)
}

/// Allowed (`allow`) or disallowed: is `name` reported?
fn listed(list: &[String], allow: bool, name: &str) -> bool {
    if list.is_empty() {
        return false;
    }
    matches_any(list, name) != allow
}

fn verb(allow: bool) -> &'static str {
    if allow { "is not in the allowed list" } else { "is disallowed" }
}

fn at_rule_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::AtRule {
            continue;
        }
        let name = sheet.at_name(i, src);
        if listed(&list, allow, &name) {
            out.push(finding(n.start, n.name.1, format!("The at-rule `@{name}` {}", verb(allow))));
        }
    }
}

css_rule!(AtRuleAllowedList, "css/at-rule-allowed-list", |sheet, src, ctx, out| {
    at_rule_list(sheet, src, ctx, true, out);
});
css_rule!(AtRuleDisallowedList, "css/at-rule-disallowed-list", |sheet, src, ctx, out| {
    at_rule_list(sheet, src, ctx, false, out);
});

css_rule!(AtRulePropertyRequiredList, "css/at-rule-property-required-list", |sheet, src, ctx, out| {
    let map = opt_map(ctx, "list");
    if map.is_empty() {
        return;
    }
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::AtRule {
            continue;
        }
        let name = sheet.at_name(i, src);
        let Some(need) = map.get(&name) else { continue };
        let have: Vec<String> = n.children.iter().filter(|&&c| sheet.nodes[c].kind == Kind::Decl).map(|&c| prop(sheet, c, src)).collect();
        for p in need {
            if !have.iter().any(|h| h.eq_ignore_ascii_case(p)) {
                out.push(finding(n.start, n.name.1, format!("`@{name}` needs the descriptor `{p}`")));
            }
        }
    }
});

css_rule!(CommentPattern, "css/comment-pattern", |sheet, src, ctx, out| {
    let Some(p) = ctx.options.get("pattern").and_then(|v| v.as_str()) else { return };
    let Ok(re) = regex::Regex::new(p) else { return };
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Comment {
            continue;
        }
        let body = src[n.start..n.end].trim_start_matches("/*").trim_end_matches("*/").trim();
        if !re.is_match(body) {
            out.push(finding(n.start, n.end, format!("This comment doesn't match the pattern `{p}`")));
        }
    }
});

css_rule!(CommentWordDisallowedList, "css/comment-word-disallowed-list", |sheet, src, ctx, out| {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::Comment {
            continue;
        }
        let body = src[n.start..n.end].trim_start_matches("/*").trim_end_matches("*/").trim();
        for w in &list {
            let hit = match w.strip_prefix('/').and_then(|r| r.strip_suffix('/')) {
                Some(re) => regex::Regex::new(re).is_ok_and(|r| r.is_match(body)),
                None => body.contains(w.as_str()),
            };
            if hit {
                out.push(finding(n.start, n.end, format!("This comment holds the disallowed word `{w}`")));
            }
        }
    }
});

css_rule!(DeclarationPropertyMaxValues, "css/declaration-property-max-values", |sheet, src, ctx, out| {
    let Some(serde_json::Value::Object(map)) = ctx.options.get("max") else { return };
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        let Some(max) = map.iter().find(|(k, _)| matches_any(std::slice::from_ref(*k), &p)).and_then(|(_, v)| v.as_u64()) else { continue };
        let vals = sheet.values(i, src);
        let count = vals.iter().filter(|v| !matches!(v.kind, VKind::Space | VKind::Comment | VKind::Div)).count() as u64;
        if count > max {
            let n = &sheet.nodes[i];
            out.push(finding(n.params.0, n.params.1, format!("`{p}` has {count} values; at most {max} are allowed")));
        }
    }
});

fn unit_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let map = opt_map(ctx, "list");
    if map.is_empty() {
        return;
    }
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        let Some(units) = lookup(&map, &p) else { continue };
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Word {
                return;
            }
            if let Some((_, u)) = dimension(v.text(src))
                && !u.is_empty()
                && matches_any(units, u) != allow
            {
                out.push(finding(v.start, v.end, format!("The unit `{u}` {} for `{p}`", verb(allow))));
            }
        });
    }
}

css_rule!(DeclarationPropertyUnitAllowedList, "css/declaration-property-unit-allowed-list", |sheet, src, ctx, out| {
    unit_list(sheet, src, ctx, true, out);
});
css_rule!(DeclarationPropertyUnitDisallowedList, "css/declaration-property-unit-disallowed-list", |sheet, src, ctx, out| {
    unit_list(sheet, src, ctx, false, out);
});

fn value_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let map = opt_map(ctx, "list");
    if map.is_empty() {
        return;
    }
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        let Some(vals) = lookup(&map, &p) else { continue };
        let n = &sheet.nodes[i];
        let v = text(src, n.params);
        if listed(vals, allow, v) {
            out.push(finding(n.params.0, n.params.1, format!("The value `{v}` {} for `{p}`", verb(allow))));
        }
    }
}

css_rule!(DeclarationPropertyValueAllowedList, "css/declaration-property-value-allowed-list", |sheet, src, ctx, out| {
    value_list(sheet, src, ctx, true, out);
});
css_rule!(DeclarationPropertyValueDisallowedList, "css/declaration-property-value-disallowed-list", |sheet, src, ctx, out| {
    value_list(sheet, src, ctx, false, out);
});

fn function_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in sheet.walk() {
        if !matches!(sheet.nodes[i].kind, Kind::Decl | Kind::AtRule) {
            continue;
        }
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind == VKind::Func && v.name.0 < v.name.1 {
                let name = v.name(src);
                if listed(&list, allow, unprefixed(&name.to_ascii_lowercase())) {
                    out.push(finding(v.name.0, v.name.1, format!("The function `{name}()` {}", verb(allow))));
                }
            }
        });
    }
}

css_rule!(FunctionAllowedList, "css/function-allowed-list", |sheet, src, ctx, out| {
    function_list(sheet, src, ctx, true, out);
});
css_rule!(FunctionDisallowedList, "css/function-disallowed-list", |sheet, src, ctx, out| {
    function_list(sheet, src, ctx, false, out);
});

/// Each `url(…)`'s address, with its range.
pub(crate) fn urls(sheet: &Sheet, src: &str) -> Vec<(String, usize, usize, Option<char>)> {
    let mut out = Vec::new();
    for i in sheet.walk() {
        if !matches!(sheet.nodes[i].kind, Kind::Decl | Kind::AtRule) {
            continue;
        }
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind == VKind::Func && v.name(src).eq_ignore_ascii_case("url") {
                let inner = src[v.inner.0..v.inner.1].trim();
                let quote = inner.chars().next().filter(|c| *c == '"' || *c == '\'');
                let addr = if quote.is_some() && inner.len() >= 2 { &inner[1..inner.len() - 1] } else { inner };
                out.push((addr.to_owned(), v.inner.0, v.inner.1, quote));
            }
        });
    }
    out
}

fn scheme(url: &str) -> Option<String> {
    let c = url.find(':')?;
    let s = &url[..c];
    (!s.is_empty() && s.chars().all(|x| x.is_ascii_alphanumeric() || "+-.".contains(x))).then(|| s.to_ascii_lowercase())
}

fn url_scheme_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for (u, s, e, _) in urls(sheet, src) {
        if let Some(sc) = scheme(&u)
            && listed(&list, allow, &sc)
        {
            out.push(finding(s, e, format!("The URL scheme `{sc}:` {}", verb(allow))));
        }
    }
}

css_rule!(FunctionUrlSchemeAllowedList, "css/function-url-scheme-allowed-list", |sheet, src, ctx, out| {
    url_scheme_list(sheet, src, ctx, true, out);
});
css_rule!(FunctionUrlSchemeDisallowedList, "css/function-url-scheme-disallowed-list", |sheet, src, ctx, out| {
    url_scheme_list(sheet, src, ctx, false, out);
});

/// Each media feature of every `@media` prelude: its name, its value, and
/// their ranges.
pub(crate) type Feature = (String, (usize, usize), Option<(usize, usize)>);

pub(crate) fn media_features(sheet: &Sheet, src: &str) -> Vec<Feature> {
    let mut out = Vec::new();
    for i in sheet.walk() {
        let n = &sheet.nodes[i];
        if n.kind != Kind::AtRule || !matches!(sheet.at_name(i, src).as_str(), "media" | "custom-media") {
            continue;
        }
        let pre = &src[n.params.0..n.params.1];
        let mut k = 0;
        while let Some(o) = pre[k..].find('(') {
            let a = n.params.0 + k + o + 1;
            let Some(c) = src[a..n.params.1].find(')') else { break };
            let b = a + c;
            let body = &src[a..b];
            if !body.contains('(') {
                let (name_end, value) = match body.find(':') {
                    Some(x) => (a + x, Some(crate::css_syntax::trim(src, a + x + 1, b))),
                    None => (b, None),
                };
                let r = crate::css_syntax::trim(src, a, name_end);
                let name = src[r.0..r.1].to_ascii_lowercase();
                if !name.is_empty() && name.chars().all(|x| x.is_ascii_alphanumeric() || x == '-') {
                    out.push((name, r, value));
                }
            }
            k = b - n.params.0;
        }
    }
    out
}

fn media_name_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for (name, r, _) in media_features(sheet, src) {
        if listed(&list, allow, &name) {
            out.push(finding(r.0, r.1, format!("The media feature `{name}` {}", verb(allow))));
        }
    }
}

css_rule!(MediaFeatureNameAllowedList, "css/media-feature-name-allowed-list", |sheet, src, ctx, out| {
    media_name_list(sheet, src, ctx, true, out);
});
css_rule!(MediaFeatureNameDisallowedList, "css/media-feature-name-disallowed-list", |sheet, src, ctx, out| {
    media_name_list(sheet, src, ctx, false, out);
});

css_rule!(MediaFeatureNameUnitAllowedList, "css/media-feature-name-unit-allowed-list", |sheet, src, ctx, out| {
    let map = opt_map(ctx, "list");
    if map.is_empty() {
        return;
    }
    for (name, _, value) in media_features(sheet, src) {
        let (Some(units), Some(v)) = (lookup(&map, &name), value) else { continue };
        if let Some((_, u)) = dimension(&src[v.0..v.1])
            && !u.is_empty()
            && !matches_any(units, u)
        {
            out.push(finding(v.0, v.1, format!("The unit `{u}` is not in the allowed list for `{name}`")));
        }
    }
});

css_rule!(MediaFeatureNameValueAllowedList, "css/media-feature-name-value-allowed-list", |sheet, src, ctx, out| {
    let map = opt_map(ctx, "list");
    if map.is_empty() {
        return;
    }
    for (name, _, value) in media_features(sheet, src) {
        let (Some(vals), Some(v)) = (lookup(&map, &name), value) else { continue };
        let t = &src[v.0..v.1];
        if !matches_any(vals, t) {
            out.push(finding(v.0, v.1, format!("The value `{t}` is not in the allowed list for `{name}`")));
        }
    }
});

fn property_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in sheet.decls() {
        let p = prop(sheet, i, src);
        if p.starts_with("--") {
            continue;
        }
        if listed(&list, allow, unprefixed(&p)) && listed(&list, allow, &p) {
            let n = &sheet.nodes[i];
            out.push(finding(n.name.0, n.name.1, format!("The property `{p}` {}", verb(allow))));
        }
    }
}

css_rule!(PropertyAllowedList, "css/property-allowed-list", |sheet, src, ctx, out| {
    property_list(sheet, src, ctx, true, out);
});
css_rule!(PropertyDisallowedList, "css/property-disallowed-list", |sheet, src, ctx, out| {
    property_list(sheet, src, ctx, false, out);
});

css_rule!(RuleNestingAtRuleRequiredList, "css/rule-nesting-at-rule-required-list", |sheet, src, ctx, out| {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in style_rules(sheet, src) {
        let nested = sheet.ancestors(i).any(|a| sheet.nodes[a].kind == Kind::Rule);
        if !nested {
            continue;
        }
        let has = sheet.ancestors(i).take_while(|&a| sheet.nodes[a].kind != Kind::Rule).any(|a| sheet.nodes[a].kind == Kind::AtRule && matches_any(&list, &sheet.at_name(a, src)));
        if !has {
            let n = &sheet.nodes[i];
            out.push(finding(n.name.0, n.name.1, format!("A nested rule must sit inside one of these at-rules: {}", list.join(", "))));
        }
    }
});

css_rule!(RuleSelectorPropertyDisallowedList, "css/rule-selector-property-disallowed-list", |sheet, src, ctx, out| {
    let map = opt_map(ctx, "list");
    if map.is_empty() {
        return;
    }
    for i in style_rules(sheet, src) {
        let n = &sheet.nodes[i];
        let sel = text(src, n.name);
        let Some(props) = lookup(&map, sel) else { continue };
        for &c in &n.children {
            if sheet.nodes[c].kind == Kind::Decl {
                let p = prop(sheet, c, src);
                if matches_any(props, &p) {
                    let d = &sheet.nodes[c];
                    out.push(finding(d.name.0, d.name.1, format!("The property `{p}` is disallowed in `{sel}`")));
                }
            }
        }
    }
});

fn simples(sheet: &Sheet, src: &str, i: usize) -> Vec<Simple> {
    let mut v = Vec::new();
    all_simples(src, &sheet.selectors(i, src), &mut v);
    v
}

/// An attribute selector's name and operator, with the operator's range.
fn attr_parts(src: &str, s: &Simple) -> (String, Option<(String, usize, usize)>) {
    let inner = &src[s.start + 1..s.end.saturating_sub(1).max(s.start + 1)];
    let base = s.start + 1;
    match inner.find(['=', '~', '|', '^', '$', '*']) {
        Some(k) if inner[k..].contains('=') => {
            let eq = inner[k..].find('=').unwrap() + k;
            let op = inner[k..=eq].trim().to_owned();
            (inner[..k].trim().to_owned(), Some((op, base + k, base + eq + 1)))
        }
        _ => (inner.trim().to_owned(), None),
    }
}

css_rule!(SelectorAttributeNameDisallowedList, "css/selector-attribute-name-disallowed-list", |sheet, src, ctx, out| {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in style_rules(sheet, src) {
        for s in simples(sheet, src, i).iter().filter(|s| s.kind == SKind::Attribute) {
            let (name, _) = attr_parts(src, s);
            if matches_any(&list, &name) {
                out.push(finding(s.start, s.end, format!("The attribute `{name}` is disallowed in selectors")));
            }
        }
    }
});

fn attr_op_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in style_rules(sheet, src) {
        for s in simples(sheet, src, i).iter().filter(|s| s.kind == SKind::Attribute) {
            if let (_, Some((op, a, b))) = attr_parts(src, s)
                && list.contains(&op) != allow
            {
                out.push(finding(a, b, format!("The attribute operator `{op}` {}", verb(allow))));
            }
        }
    }
}

css_rule!(SelectorAttributeOperatorAllowedList, "css/selector-attribute-operator-allowed-list", |sheet, src, ctx, out| {
    attr_op_list(sheet, src, ctx, true, out);
});
css_rule!(SelectorAttributeOperatorDisallowedList, "css/selector-attribute-operator-disallowed-list", |sheet, src, ctx, out| {
    attr_op_list(sheet, src, ctx, false, out);
});

fn combinator_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in style_rules(sheet, src) {
        for c in sheet.selectors(i, src).iter() {
            for k in c.combinators.iter().chain(c.leading.iter()) {
                let shown = if k.trim().is_empty() { " ".to_owned() } else { k.trim().to_owned() };
                if list.contains(&shown) != allow {
                    out.push(finding(c.start, c.end, format!("The combinator `{shown}` {}", verb(allow))));
                }
            }
        }
    }
}

css_rule!(SelectorCombinatorAllowedList, "css/selector-combinator-allowed-list", |sheet, src, ctx, out| {
    combinator_list(sheet, src, ctx, true, out);
});
css_rule!(SelectorCombinatorDisallowedList, "css/selector-combinator-disallowed-list", |sheet, src, ctx, out| {
    combinator_list(sheet, src, ctx, false, out);
});

css_rule!(SelectorDisallowedList, "css/selector-disallowed-list", |sheet, src, ctx, out| {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in style_rules(sheet, src) {
        for c in sheet.selectors(i, src).iter() {
            let t = text(src, (c.start, c.end));
            if matches_any(&list, t) {
                out.push(finding(c.start, c.end, format!("The selector `{t}` is disallowed")));
            }
        }
    }
});

fn pseudo_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, kind: SKind, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    let what = if kind == SKind::PseudoClass { "pseudo-class" } else { "pseudo-element" };
    for i in style_rules(sheet, src) {
        for s in simples(sheet, src, i).iter().filter(|s| s.kind == kind) {
            let name = text(src, s.name).to_ascii_lowercase();
            if listed(&list, allow, unprefixed(&name)) && listed(&list, allow, &name) {
                out.push(finding(s.start, s.name.1, format!("The {what} `{name}` {}", verb(allow))));
            }
        }
    }
}

css_rule!(SelectorPseudoClassAllowedList, "css/selector-pseudo-class-allowed-list", |sheet, src, ctx, out| {
    pseudo_list(sheet, src, ctx, SKind::PseudoClass, true, out);
});
css_rule!(SelectorPseudoClassDisallowedList, "css/selector-pseudo-class-disallowed-list", |sheet, src, ctx, out| {
    pseudo_list(sheet, src, ctx, SKind::PseudoClass, false, out);
});
css_rule!(SelectorPseudoElementAllowedList, "css/selector-pseudo-element-allowed-list", |sheet, src, ctx, out| {
    pseudo_list(sheet, src, ctx, SKind::PseudoElement, true, out);
});
css_rule!(SelectorPseudoElementDisallowedList, "css/selector-pseudo-element-disallowed-list", |sheet, src, ctx, out| {
    pseudo_list(sheet, src, ctx, SKind::PseudoElement, false, out);
});

/// A limit on one kind of simple selector per complex selector.
fn max_kind(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, kinds: &[SKind], default: i64, what: &str, out: &mut Vec<crate::NativeFinding>) {
    let max = opt_int(ctx, "max", default);
    for i in style_rules(sheet, src) {
        for c in sheet.selectors(i, src).iter() {
            let mut v = Vec::new();
            all_simples(src, std::slice::from_ref(c), &mut v);
            let n = v.iter().filter(|s| kinds.contains(&s.kind)).count() as i64;
            if n > max {
                out.push(finding(c.start, c.end, format!("`{}` has {n} {what}; at most {max} are allowed", text(src, (c.start, c.end)))));
            }
        }
    }
}

css_rule!(SelectorMaxAttribute, "css/selector-max-attribute", |sheet, src, ctx, out| {
    max_kind(sheet, src, ctx, &[SKind::Attribute], 2, "attribute selectors", out);
});
css_rule!(SelectorMaxClass, "css/selector-max-class", |sheet, src, ctx, out| {
    max_kind(sheet, src, ctx, &[SKind::Class], 3, "classes", out);
});
css_rule!(SelectorMaxId, "css/selector-max-id", |sheet, src, ctx, out| {
    max_kind(sheet, src, ctx, &[SKind::Id], 1, "id selectors", out);
});
css_rule!(SelectorMaxPseudoClass, "css/selector-max-pseudo-class", |sheet, src, ctx, out| {
    max_kind(sheet, src, ctx, &[SKind::PseudoClass], 3, "pseudo-classes", out);
});
css_rule!(SelectorMaxType, "css/selector-max-type", |sheet, src, ctx, out| {
    max_kind(sheet, src, ctx, &[SKind::Type], 2, "type selectors", out);
});
css_rule!(SelectorMaxUniversal, "css/selector-max-universal", |sheet, src, ctx, out| {
    max_kind(sheet, src, ctx, &[SKind::Universal], 1, "universal selectors", out);
});

css_rule!(SelectorMaxCombinators, "css/selector-max-combinators", |sheet, src, ctx, out| {
    let max = opt_int(ctx, "max", 3);
    for i in style_rules(sheet, src) {
        for c in sheet.selectors(i, src).iter() {
            let n = c.combinators.len() as i64;
            if n > max {
                out.push(finding(c.start, c.end, format!("`{}` has {n} combinators; at most {max} are allowed", text(src, (c.start, c.end)))));
            }
        }
    }
});

css_rule!(SelectorMaxCompoundSelectors, "css/selector-max-compound-selectors", |sheet, src, ctx, out| {
    let max = opt_int(ctx, "max", 3);
    for i in style_rules(sheet, src) {
        let depth = sheet.ancestors(i).filter(|&a| sheet.nodes[a].kind == Kind::Rule).count() as i64;
        for c in sheet.selectors(i, src).iter() {
            let nesting = c.simples().any(|s| s.kind == SKind::Nesting);
            let n = c.compounds.len() as i64 + if nesting { depth } else { 0 };
            if n > max {
                out.push(finding(c.start, c.end, format!("`{}` has {n} compound selectors; at most {max} are allowed", text(src, (c.start, c.end)))));
            }
        }
    }
});

css_rule!(SelectorMaxSpecificity, "css/selector-max-specificity", |sheet, src, ctx, out| {
    let max = ctx.options.get("max").and_then(|v| v.as_str()).unwrap_or("0,4,0").to_owned();
    let lim: Vec<u32> = max.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let [a, b, c] = lim[..] else { return };
    for i in style_rules(sheet, src) {
        for cx in sheet.selectors(i, src).iter() {
            let s = specificity(src, cx, (0, 0, 0));
            if s > (a, b, c) {
                out.push(finding(cx.start, cx.end, format!("`{}` has specificity {},{},{}; at most {max} is allowed", text(src, (cx.start, cx.end)), s.0, s.1, s.2)));
            }
        }
    }
});

css_rule!(SelectorNestedPattern, "css/selector-nested-pattern", |sheet, src, ctx, out| {
    let Some(p) = ctx.options.get("pattern").and_then(|v| v.as_str()) else { return };
    let Ok(re) = regex::Regex::new(p) else { return };
    for i in style_rules(sheet, src) {
        if !sheet.ancestors(i).any(|a| sheet.nodes[a].kind == Kind::Rule) {
            continue;
        }
        let n = &sheet.nodes[i];
        let t = text(src, n.name);
        if !re.is_match(t) {
            out.push(finding(n.name.0, n.name.1, format!("The nested selector `{t}` doesn't match the pattern `{p}`")));
        }
    }
});

fn unit_any_list(sheet: &Sheet, src: &str, ctx: &crate::NativeContext<'_>, allow: bool, out: &mut Vec<crate::NativeFinding>) {
    let list = opt_list(ctx, "list");
    if list.is_empty() {
        return;
    }
    for i in sheet.walk() {
        if !matches!(sheet.nodes[i].kind, Kind::Decl | Kind::AtRule) {
            continue;
        }
        each_value(&sheet.values(i, src), &mut |v| {
            if v.kind != VKind::Word {
                return;
            }
            if let Some((_, u)) = dimension(v.text(src))
                && !u.is_empty()
                && list.iter().any(|x| x.eq_ignore_ascii_case(u)) != allow
            {
                out.push(finding(v.start, v.end, format!("The unit `{u}` {}", verb(allow))));
            }
        });
    }
}

css_rule!(UnitAllowedList, "css/unit-allowed-list", |sheet, src, ctx, out| {
    unit_any_list(sheet, src, ctx, true, out);
});
css_rule!(UnitDisallowedList, "css/unit-disallowed-list", |sheet, src, ctx, out| {
    unit_any_list(sheet, src, ctx, false, out);
});

pub static RULES: &[&dyn crate::NativeRule] = &[
    &AtRuleAllowedList,
    &AtRuleDisallowedList,
    &AtRulePropertyRequiredList,
    &CommentPattern,
    &CommentWordDisallowedList,
    &DeclarationPropertyMaxValues,
    &DeclarationPropertyUnitAllowedList,
    &DeclarationPropertyUnitDisallowedList,
    &DeclarationPropertyValueAllowedList,
    &DeclarationPropertyValueDisallowedList,
    &FunctionAllowedList,
    &FunctionDisallowedList,
    &FunctionUrlSchemeAllowedList,
    &FunctionUrlSchemeDisallowedList,
    &MediaFeatureNameAllowedList,
    &MediaFeatureNameDisallowedList,
    &MediaFeatureNameUnitAllowedList,
    &MediaFeatureNameValueAllowedList,
    &PropertyAllowedList,
    &PropertyDisallowedList,
    &RuleNestingAtRuleRequiredList,
    &RuleSelectorPropertyDisallowedList,
    &SelectorAttributeNameDisallowedList,
    &SelectorAttributeOperatorAllowedList,
    &SelectorAttributeOperatorDisallowedList,
    &SelectorCombinatorAllowedList,
    &SelectorCombinatorDisallowedList,
    &SelectorDisallowedList,
    &SelectorPseudoClassAllowedList,
    &SelectorPseudoClassDisallowedList,
    &SelectorPseudoElementAllowedList,
    &SelectorPseudoElementDisallowedList,
    &SelectorMaxAttribute,
    &SelectorMaxClass,
    &SelectorMaxId,
    &SelectorMaxPseudoClass,
    &SelectorMaxType,
    &SelectorMaxUniversal,
    &SelectorMaxCombinators,
    &SelectorMaxCompoundSelectors,
    &SelectorMaxSpecificity,
    &SelectorNestedPattern,
    &UnitAllowedList,
    &UnitDisallowedList,
];

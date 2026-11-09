//! W-139 strand W-139-js: eslint-plugin-unicorn v79.0.0's
//! `no-shorthand-property-overrides`, JavaScript half, read from the rule's
//! source and tests and written again: a CSS shorthand written after a
//! longhand it resets in one `style={{…}}` object of an intrinsic JSX
//! element. (Stylesheets are the `css` pack's
//! `css/declaration-block-no-shorthand-property-overrides`.)

use super::finding;
use crate::util::{named_children, string_value, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use std::collections::BTreeMap;
use tree_sitter::Node;

/// shorthand, its longhands, and what else it resets.
const SHORTHANDS: &[(&str, &str, &str)] = &[
    ("animation", "animation-duration animation-timing-function animation-delay animation-iteration-count animation-direction animation-fill-mode animation-play-state animation-name animation-timeline animation-trigger", "animation-range animation-range-start animation-range-end"),
    ("animation-range", "animation-range-start animation-range-end", ""),
    ("background", "background-image background-position background-size background-repeat background-attachment background-origin background-clip background-color background-position-x background-position-y background-repeat-x background-repeat-y", ""),
    ("background-position", "background-position-x background-position-y", ""),
    ("background-repeat", "background-repeat-x background-repeat-y", ""),
    ("border", "border-width border-style border-color border-top-width border-right-width border-bottom-width border-left-width border-top-style border-right-style border-bottom-style border-left-style border-top-color border-right-color border-bottom-color border-left-color", "border-image border-image-source border-image-slice border-image-width border-image-outset border-image-repeat"),
    ("border-block", "border-block-width border-block-style border-block-color border-block-start-width border-block-end-width border-block-start-style border-block-end-style border-block-start-color border-block-end-color", ""),
    ("border-block-color", "border-block-start-color border-block-end-color", ""),
    ("border-block-style", "border-block-start-style border-block-end-style", ""),
    ("border-block-width", "border-block-start-width border-block-end-width", ""),
    ("border-bottom", "border-bottom-width border-bottom-style border-bottom-color", ""),
    ("border-color", "border-top-color border-right-color border-bottom-color border-left-color", ""),
    ("border-image", "border-image-source border-image-slice border-image-width border-image-outset border-image-repeat", ""),
    ("border-inline", "border-inline-width border-inline-style border-inline-color border-inline-start-width border-inline-end-width border-inline-start-style border-inline-end-style border-inline-start-color border-inline-end-color", ""),
    ("border-inline-color", "border-inline-start-color border-inline-end-color", ""),
    ("border-inline-style", "border-inline-start-style border-inline-end-style", ""),
    ("border-inline-width", "border-inline-start-width border-inline-end-width", ""),
    ("border-left", "border-left-width border-left-style border-left-color", ""),
    ("border-radius", "border-top-left-radius border-top-right-radius border-bottom-right-radius border-bottom-left-radius", ""),
    ("border-right", "border-right-width border-right-style border-right-color", ""),
    ("border-style", "border-top-style border-right-style border-bottom-style border-left-style", ""),
    ("border-top", "border-top-width border-top-style border-top-color", ""),
    ("border-width", "border-top-width border-right-width border-bottom-width border-left-width", ""),
    ("column-rule", "column-rule-width column-rule-style column-rule-color", ""),
    ("columns", "column-width column-count column-height", ""),
    ("flex", "flex-grow flex-shrink flex-basis", ""),
    ("flex-flow", "flex-direction flex-wrap", ""),
    ("font", "font-style font-variant font-weight font-stretch font-size line-height font-family font-variant-ligatures font-variant-position font-variant-caps font-variant-numeric font-variant-alternates font-variant-east-asian font-variant-emoji", "font-feature-settings font-kerning font-language-override font-optical-sizing font-size-adjust font-variation-settings"),
    ("font-variant", "font-variant-ligatures font-variant-position font-variant-caps font-variant-numeric font-variant-alternates font-variant-east-asian font-variant-emoji", ""),
    ("gap", "row-gap column-gap", ""),
    ("grid", "grid-template-rows grid-template-columns grid-template-areas grid-auto-rows grid-auto-columns grid-auto-flow", ""),
    ("grid-area", "grid-row-start grid-column-start grid-row-end grid-column-end", ""),
    ("grid-column", "grid-column-start grid-column-end", ""),
    ("grid-row", "grid-row-start grid-row-end", ""),
    ("grid-template", "grid-template-rows grid-template-columns grid-template-areas", ""),
    ("inset", "top right bottom left", ""),
    ("inset-block", "inset-block-start inset-block-end", ""),
    ("inset-inline", "inset-inline-start inset-inline-end", ""),
    ("list-style", "list-style-type list-style-position list-style-image", ""),
    ("margin", "margin-top margin-right margin-bottom margin-left", ""),
    ("margin-block", "margin-block-start margin-block-end", ""),
    ("margin-inline", "margin-inline-start margin-inline-end", ""),
    ("mask", "mask-image mask-position mask-size mask-repeat mask-origin mask-clip mask-composite mask-mode", "mask-border mask-border-source mask-border-slice mask-border-width mask-border-outset mask-border-repeat mask-border-mode"),
    ("outline", "outline-width outline-style outline-color", ""),
    ("overflow", "overflow-x overflow-y", ""),
    ("overscroll-behavior", "overscroll-behavior-x overscroll-behavior-y", ""),
    ("padding", "padding-top padding-right padding-bottom padding-left", ""),
    ("padding-block", "padding-block-start padding-block-end", ""),
    ("padding-inline", "padding-inline-start padding-inline-end", ""),
    ("place-content", "align-content justify-content", ""),
    ("place-items", "align-items justify-items", ""),
    ("place-self", "align-self justify-self", ""),
    ("scroll-margin", "scroll-margin-top scroll-margin-right scroll-margin-bottom scroll-margin-left", ""),
    ("scroll-padding", "scroll-padding-top scroll-padding-right scroll-padding-bottom scroll-padding-left", ""),
    ("text-decoration", "text-decoration-line text-decoration-style text-decoration-color text-decoration-thickness", ""),
    ("text-emphasis", "text-emphasis-style text-emphasis-color", ""),
    ("text-wrap", "text-wrap-mode text-wrap-style", ""),
    ("transition", "transition-property transition-duration transition-timing-function transition-delay transition-behavior", ""),
    ("white-space", "white-space-collapse text-wrap-mode white-space-trim", ""),
];

fn affected(shorthand: &str) -> Vec<&'static str> {
    SHORTHANDS.iter().find(|(s, _, _)| *s == shorthand).map(|(_, a, b)| a.split(' ').chain(b.split(' ')).filter(|x| !x.is_empty()).collect()).unwrap_or_default()
}

fn vendor_prefix(p: &str) -> &str {
    if p.starts_with('-')
        && let Some(i) = p[1..].find('-')
    {
        return &p[..i + 2];
    }
    ""
}

fn static_value(n: Node<'_>, src: &str) -> bool {
    let n = crate::js_ast::unparen(n);
    let num = if n.kind() == "unary_expression" && n.child(0).is_some_and(|c| matches!(text(c, src), "+" | "-")) { n.child_by_field_name("argument").map(crate::js_ast::unparen).unwrap_or(n) } else { n };
    if num.kind() == "number" {
        return true;
    }
    let s = match n.kind() {
        "string" => string_value(n, src),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => text(n, src).trim_matches('`').to_owned(),
        _ => return false,
    };
    !s.trim().is_empty() && !s.to_lowercase().replace(char::is_whitespace, "").contains("!important")
}

pub struct NoShorthandPropertyOverrides;
impl NativeRule for NoShorthandPropertyOverrides {
    fn id(&self) -> &'static str {
        "js/no-shorthand-property-overrides"
    }
    fn languages(&self) -> &'static [Language] {
        &[Language::Jsx, Language::Tsx]
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_attribute"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let a = node.get_inner_node();
        let src = ctx.source;
        let (Some(k), Some(v)) = (a.named_child(0), a.named_child(1)) else { return };
        if text(k, src) != "style" || v.kind() != "jsx_expression" {
            return;
        }
        let Some(open) = a.parent() else { return };
        if !open.child_by_field_name("name").is_some_and(|n| n.kind() == "identifier" && text(n, src).chars().next().is_some_and(|c| c.is_ascii_lowercase())) {
            return;
        }
        let Some(obj) = v.named_child(0).map(crate::js_ast::unparen).filter(|o| o.kind() == "object") else { return };
        let mut props: Vec<(String, Node<'_>, Node<'_>)> = Vec::new();
        for m in named_children(obj).into_iter().filter(|c| c.kind() != "comment") {
            match m.kind() {
                "pair" => {
                    let Some(key) = m.child_by_field_name("key") else { return };
                    let name = match key.kind() {
                        "property_identifier" => text(key, src).to_owned(),
                        "string" => string_value(key, src),
                        "computed_property_name" => return,
                        _ => return,
                    };
                    let Some(val) = m.child_by_field_name("value") else { return };
                    // A later duplicate replaces the value but keeps the first position.
                    match props.iter_mut().find(|(n, _, _)| *n == name) {
                        Some(slot) => *slot = (name, key, val),
                        None => props.push((name, key, val)),
                    }
                }
                "shorthand_property_identifier" => {
                    let name = text(m, src).to_owned();
                    if let Some(slot) = props.iter_mut().find(|(n, _, _)| *n == name) {
                        *slot = (name, m, m);
                    } else {
                        props.push((name, m, m));
                    }
                }
                _ => return,
            }
        }
        let mut declared: BTreeMap<String, String> = BTreeMap::new();
        for (name, key, val) in &props {
            if val.kind() == "shorthand_property_identifier" || !static_value(*val, src) {
                continue;
            }
            let mut prop: String = name.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect();
            if prop.starts_with("ms-") {
                prop.insert(0, '-');
            }
            let vp = vendor_prefix(&prop).to_owned();
            declared.insert(prop.clone(), name.clone());
            for l in affected(&prop[vp.len()..]) {
                if let Some(orig) = declared.get(&format!("{vp}{l}")) {
                    out.push(finding(*key, format!("The shorthand `{name}` resets the longhand `{orig}` written before it: the longhand has no effect"), vec![]));
                }
            }
        }
    }
}

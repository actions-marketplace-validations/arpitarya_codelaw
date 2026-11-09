//! W-139 strand W-139-jsdoc: the rules of eslint-plugin-jsdoc (65.4.2) that
//! need a JSDoc comment parser or a JSDoc type-expression parser. A pattern
//! cannot read a comment as lines, tags and types, so these are native rules
//! (SR-ARCHITECTURE decision 15b) on this module's own small parsers:
//! [`block`] reads a `/** … */` comment, [`types`] a `{type}`, [`tags`] knows
//! the tag names. Nothing here depends on a library (L11).

pub(crate) mod block;
mod imports;
mod layout;
mod libtypes;
mod names;
mod prose;
mod tags;
mod target;
mod typerules;
#[cfg(test)]
mod tests;
pub(crate) mod types;

use crate::{NativeContext, NativeFinding, NativeRule};
use block::Block;
use codelaw_core::{Language, TextEdit};
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

pub(super) fn find(start: usize, end: usize, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

pub(super) fn opt_str_list(o: &BTreeMap<String, Value>, key: &str) -> Vec<String> {
    o.get(key).and_then(Value::as_array).map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect()).unwrap_or_default()
}

pub(super) fn opt_bool(o: &BTreeMap<String, Value>, key: &str, default: bool) -> bool {
    o.get(key).and_then(Value::as_bool).unwrap_or(default)
}

/// Delete whole lines `from..=to` of the block, their line breaks with them.
pub(super) fn delete_lines(b: &Block<'_>, from: usize, to: usize) -> TextEdit {
    let start = b.lines[from].start;
    let end = if to + 1 < b.lines.len() { b.lines[to + 1].start } else { b.lines[to].end };
    TextEdit::new(start, end, "")
}

/// A native rule on every JSDoc block: `$b` is the parsed block, `$n` the comment node.
macro_rules! jsdoc_rule {
    ($name:ident, $id:literal, |$n:ident, $b:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl crate::NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [codelaw_core::Language] {
                crate::jsdoc::JS
            }
            fn kinds(&self) -> &'static [&'static str] {
                &["comment"]
            }
            fn check(&self, node: codelaw_lang::SyntaxNode<'_>, $ctx: &crate::NativeContext<'_>, $out: &mut Vec<crate::NativeFinding>) {
                let $n = node.get_inner_node();
                let Some($b) = crate::jsdoc::block::Block::parse($ctx.source, $n.start_byte(), $n.end_byte()) else { return };
                $body
            }
        }
    };
}
pub(super) use jsdoc_rule;

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &names::CheckTagNames,
    &names::CheckParamNames,
    &names::CheckPropertyNames,
    &names::CheckTemplateNames,
    &prose::InformativeDocs,
    &prose::NormalizeSeeLinks,
    &imports::ImportsAsDependencies,
    &imports::NoUndefinedTypes,
    &layout::TagLines,
    &layout::SortTags,
    &typerules::ValidTypes,
    &typerules::TypeFormatting,
    &typerules::TsMethodSignatureStyle,
    &typerules::TsPreferFunctionType,
    &typerules::TsNoUnnecessaryTemplateExpression,
];

#[allow(dead_code)]
fn _uses(_: &NativeContext<'_>) {}

//! The `tailwind` pack's rules (SR-RULE-PACKS decision 19, W-145). Each
//! looks at the class lists the formatter sorts (`codelaw_lang::class_lists`)
//! word by word. Five read only the words; `no-unknown-classes`,
//! `no-conflicting-classes` and `enforces-canonical-classname` ask the
//! provider's class table, built from the project's own Tailwind, and find
//! nothing when there is none (the run notes why, `[providers] on_missing`).
//! No class name and no Tailwind order is written here.

use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::class_table::{ClassInfo, ClassTable};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_lang::class_lists::{ClassList, ClassListOptions, class_lists_in};
use std::collections::BTreeMap;

/// A class list with each of its words and the table's answer for it.
type Answered<'s> = Vec<(ClassList, Vec<(Word<'s>, Option<ClassInfo>)>)>;
/// Classes keyed by (variants, important, properties), each with its Tailwind order.
type Groups<'a, 's> = BTreeMap<(String, bool, Vec<String>), Vec<(&'a Word<'s>, u128)>>;

const LANGUAGES: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx, Language::Html];
const ROOTS: &[&str] = &["program", "document"];

/// One word of a class list and where it is.
struct Word<'s> {
    start: usize,
    end: usize,
    text: &'s str,
}

fn list_options(ctx: &NativeContext<'_>) -> ClassListOptions {
    let mut o = ClassListOptions::default();
    let names = |k: &str| ctx.options.get(k).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|n| n.as_str().map(str::to_owned)).collect::<Vec<_>>());
    if let Some(a) = names("attributes") {
        o.attributes = a;
    }
    if let Some(f) = names("functions") {
        o.functions = f;
    }
    o
}

/// The words of a list a rule may judge whole: a word glued to a template's
/// `${…}` is part of a class only the running code spells.
fn words<'s>(src: &'s str, l: &ClassList) -> Vec<Word<'s>> {
    let text = &src[l.start..l.end];
    if text.contains("{{") {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut at = 0;
    for w in text.split(|c: char| c.is_ascii_whitespace()) {
        if !w.is_empty() {
            out.push(Word { start: l.start + at, end: l.start + at + w.len(), text: w });
        }
        at += w.len() + 1;
    }
    if l.sort.ignore_first && !out.is_empty() && !text.starts_with(|c: char| c.is_ascii_whitespace()) {
        out.remove(0);
    }
    if l.sort.ignore_last && !out.is_empty() && !text.ends_with(|c: char| c.is_ascii_whitespace()) {
        out.pop();
    }
    out
}

fn lists(node: &SyntaxNode<'_>, ctx: &NativeContext<'_>) -> Vec<ClassList> {
    if node.parent().is_some() {
        return Vec::new();
    }
    class_lists_in(ctx.language, node, &list_options(ctx))
}

/// The word split at its last `:` outside brackets: `(variants:, base)`.
fn split_variants(w: &str) -> (&str, &str) {
    let mut depth = 0i32;
    let mut cut = 0;
    for (i, c) in w.char_indices() {
        match c {
            '[' | '(' => depth += 1,
            ']' | ')' => depth -= 1,
            ':' if depth == 0 => cut = i + 1,
            _ => {}
        }
    }
    w.split_at(cut)
}

/// The edit that removes a word and one space beside it.
fn remove(src: &str, w: &Word<'_>, l: &ClassList) -> TextEdit {
    let (mut s, mut e) = (w.start, w.end);
    if e < l.end && src.as_bytes()[e].is_ascii_whitespace() {
        e += 1;
    } else if s > l.start && src.as_bytes()[s - 1].is_ascii_whitespace() {
        s -= 1;
    }
    TextEdit { start: s, end: e, text: String::new() }
}

fn replace(w: &Word<'_>, with: String) -> TextEdit {
    TextEdit { start: w.start, end: w.end, text: with }
}

fn finding(w: &Word<'_>, message: String, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: w.start, end: w.end, message, edits }
}

fn table(ctx: &NativeContext<'_>) -> Option<std::sync::Arc<dyn ClassTable>> {
    ctx.file.class_table("tailwind")
}

/// Each list's words and the table's answer for each, all from one lookup.
fn answered<'s>(src: &'s str, ls: &[ClassList], t: &dyn ClassTable) -> Answered<'s> {
    let per_list: Vec<Vec<Word<'s>>> = ls.iter().map(|l| words(src, l)).collect();
    let all: Vec<&str> = per_list.iter().flatten().map(|w| w.text).collect();
    let Ok(mut answers) = t.lookup(&all) else { return Vec::new() };
    let mut out = Vec::new();
    let mut rest = answers.drain(..);
    for (l, ws) in ls.iter().zip(per_list) {
        let pairs = ws.into_iter().map(|w| (w, rest.next().flatten())).collect();
        out.push((*l, pairs));
    }
    out
}

macro_rules! rule {
    ($name:ident, $id:literal) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                LANGUAGES
            }
            fn kinds(&self) -> &'static [&'static str] {
                ROOTS
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                let ls = lists(&node, ctx);
                self.lists(&ls, ctx, out);
            }
            fn provider(&self) -> Option<&'static str> {
                self.needs_table().then_some("tailwind")
            }
        }
    };
}

rule!(ImportantModifierSuffix, "tailwind/important-modifier-suffix");
rule!(NegativeArbitraryValues, "tailwind/enforces-negative-arbitrary-values");
rule!(NoArbitraryValue, "tailwind/no-arbitrary-value");
rule!(NoRestrictedClasses, "tailwind/no-restricted-classes");
rule!(NoUnknownClasses, "tailwind/no-unknown-classes");
rule!(NoConflictingClasses, "tailwind/no-conflicting-classes");
rule!(CanonicalClassname, "tailwind/enforces-canonical-classname");

impl ImportantModifierSuffix {
    fn needs_table(&self) -> bool {
        false
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        for l in ls {
            for w in words(ctx.source, l) {
                let (variants, base) = split_variants(w.text);
                if let Some(rest) = base.strip_prefix('!').filter(|r| !r.is_empty() && !r.ends_with('!')) {
                    let fixed = format!("{variants}{rest}!");
                    out.push(finding(&w, format!("Write the important modifier last: `{fixed}`"), vec![replace(&w, fixed.clone())]));
                }
            }
        }
    }
}

/// `-name-[value]` → `name-[-value]`, keeping variants and `!`.
fn negative_arbitrary(word: &str) -> Option<String> {
    let (variants, base) = split_variants(word);
    let (lead, base) = match base.strip_prefix('!') {
        Some(b) => ("!", b),
        None => ("", base),
    };
    let (base, trail) = match base.strip_suffix('!') {
        Some(b) => (b, "!"),
        None => (base, ""),
    };
    let body = base.strip_prefix('-')?;
    let open = body.find("-[")?;
    let (name, value) = (&body[..open], &body[open + 2..]);
    let value = value.strip_suffix(']')?;
    if name.is_empty() || value.is_empty() || value.starts_with('-') {
        return None;
    }
    Some(format!("{variants}{lead}{name}-[-{value}]{trail}"))
}

impl NegativeArbitraryValues {
    fn needs_table(&self) -> bool {
        false
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        for l in ls {
            for w in words(ctx.source, l) {
                if let Some(fixed) = negative_arbitrary(w.text) {
                    out.push(finding(&w, format!("Put the minus inside the arbitrary value: `{fixed}`"), vec![replace(&w, fixed.clone())]));
                }
            }
        }
    }
}

impl NoArbitraryValue {
    fn needs_table(&self) -> bool {
        false
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        for l in ls {
            for w in words(ctx.source, l) {
                let (_, base) = split_variants(w.text);
                if base.contains("-[") && base.trim_end_matches('!').ends_with(']') {
                    out.push(finding(&w, format!("`{}` uses an arbitrary value; use a value from the theme", w.text), Vec::new()));
                }
            }
        }
    }
}

impl NoRestrictedClasses {
    fn needs_table(&self) -> bool {
        false
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let patterns: Vec<regex::Regex> = ctx
            .options
            .get("restrict")
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|p| p.as_str())
            .filter_map(|p| regex::Regex::new(&format!("^(?:{p})$")).ok())
            .collect();
        if patterns.is_empty() {
            return;
        }
        for l in ls {
            for w in words(ctx.source, l) {
                if patterns.iter().any(|p| p.is_match(w.text)) {
                    out.push(finding(&w, format!("`{}` is restricted in this project", w.text), vec![remove(ctx.source, &w, l)]));
                }
            }
        }
    }
}

impl NoUnknownClasses {
    fn needs_table(&self) -> bool {
        true
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(t) = table(ctx) else { return };
        let ignore: Vec<regex::Regex> =
            ctx.options.get("ignore").and_then(|v| v.as_array()).into_iter().flatten().filter_map(|p| p.as_str()).filter_map(|p| regex::Regex::new(&format!("^(?:{p})$")).ok()).collect();
        for (l, ws) in answered(ctx.source, ls, t.as_ref()) {
            for (w, info) in ws {
                if info.is_none() && w.text != "..." && w.text != "…" && !ignore.iter().any(|p| p.is_match(w.text)) {
                    let edit = remove(ctx.source, &w, &l);
                    out.push(finding(&w, format!("`{}` is not a class of this project's Tailwind", w.text), vec![edit]));
                }
            }
        }
    }
}

impl NoConflictingClasses {
    fn needs_table(&self) -> bool {
        true
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(t) = table(ctx) else { return };
        for (l, ws) in answered(ctx.source, ls, t.as_ref()) {
            // Classes that set exactly the same properties under the same
            // variants: the one later in Tailwind's order wins.
            let mut groups: Groups<'_, '_> = BTreeMap::new();
            for (w, info) in &ws {
                if let Some(i) = info.as_ref().filter(|i| !i.properties.is_empty()) {
                    groups.entry((i.variants.clone(), i.important, i.properties.clone())).or_default().push((w, i.order));
                }
            }
            for members in groups.values().filter(|m| m.len() > 1) {
                let winner = members.iter().max_by_key(|(_, o)| *o).map(|(w, _)| w.text).unwrap_or_default();
                for (w, _) in members.iter().filter(|(w, _)| w.text != winner) {
                    out.push(finding(w, format!("`{}` and `{winner}` set the same properties; `{winner}` wins, so `{}` does nothing", w.text, w.text), vec![remove(ctx.source, w, &l)]));
                }
            }
        }
    }
}

impl CanonicalClassname {
    fn needs_table(&self) -> bool {
        true
    }
    fn lists(&self, ls: &[ClassList], ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(t) = table(ctx) else { return };
        for (_, ws) in answered(ctx.source, ls, t.as_ref()) {
            for (w, info) in ws {
                if let Some(c) = info.and_then(|i| i.canonical) {
                    out.push(finding(&w, format!("Tailwind spells `{}` as `{c}`", w.text), vec![replace(&w, c.clone())]));
                }
            }
        }
    }
}

/// `tailwind/no-concatenated-classes`: a class string joined with `+`. The
/// suggested edit writes the chain as one template literal.
pub struct NoConcatenatedClasses;

fn escape_template(s: &str) -> String {
    s.replace('`', "\\`").replace("${", "\\${")
}

impl NativeRule for NoConcatenatedClasses {
    fn id(&self) -> &'static str {
        "tailwind/no-concatenated-classes"
    }
    fn languages(&self) -> &'static [Language] {
        &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx]
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let ls = lists(&node, ctx);
        let is_plus = |n: &SyntaxNode<'_>| n.kind() == "binary_expression" && n.field("operator").is_some_and(|o| o.text() == "+");
        for b in node.dfs().filter(|n| is_plus(n) && !n.parent().is_some_and(|p| is_plus(&p))) {
            let r = b.range();
            // A class list is one of the operands.
            if !ls.iter().any(|l| l.start > r.start && l.end < r.end) {
                continue;
            }
            let mut operands = Vec::new();
            let mut stack = vec![b.clone()];
            while let Some(n) = stack.pop() {
                if is_plus(&n) {
                    if let (Some(left), Some(right)) = (n.field("left"), n.field("right")) {
                        stack.push(right);
                        stack.push(left);
                    }
                } else {
                    operands.push(n);
                }
            }
            if !operands.iter().any(|o| o.kind() == "string") {
                continue;
            }
            let mut tpl = String::from("`");
            for o in &operands {
                let t = o.text();
                match o.kind().as_ref() {
                    "string" if t.len() >= 2 => tpl.push_str(&escape_template(&t[1..t.len() - 1])),
                    "template_string" if t.len() >= 2 => tpl.push_str(&t[1..t.len() - 1]),
                    _ => {
                        tpl.push_str("${");
                        tpl.push_str(&t);
                        tpl.push('}');
                    }
                }
            }
            tpl.push('`');
            out.push(NativeFinding {
                start: r.start,
                end: r.end,
                message: "Classes are joined with `+`; write one template literal or pass the parts to a class function".into(),
                edits: vec![TextEdit { start: r.start, end: r.end, text: tpl }],
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_split_at_the_last_colon_outside_brackets() {
        assert_eq!(split_variants("md:hover:p-4"), ("md:hover:", "p-4"));
        assert_eq!(split_variants("[&:hover]:p-4"), ("[&:hover]:", "p-4"));
        assert_eq!(split_variants("bg-[url(a:b)]"), ("", "bg-[url(a:b)]"));
    }

    #[test]
    fn negative_arbitrary_values_move_the_minus_inside() {
        assert_eq!(negative_arbitrary("-m-[4px]").as_deref(), Some("m-[-4px]"));
        assert_eq!(negative_arbitrary("md:-top-[3rem]!").as_deref(), Some("md:top-[-3rem]!"));
        assert_eq!(negative_arbitrary("-m-4"), None);
        assert_eq!(negative_arbitrary("m-[-4px]"), None);
    }

    #[test]
    fn syntactic_rules_find_and_fix() {
        let f = crate::run(&ImportantModifierSuffix, Language::Jsx, "const a = <p className=\"md:!p-4 flex\" />;\n");
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].edits[0].text, "md:p-4!");
        let f = crate::run(&NoConcatenatedClasses, Language::Jsx, "const a = <p className={\"p-4 \" + x} />;\n");
        assert_eq!(f[0].edits[0].text, "`p-4 ${x}`");
        assert!(crate::run(&NoConcatenatedClasses, Language::Jsx, "const a = \"p-4 \" + x;\n").is_empty());
        assert_eq!(crate::run(&NoArbitraryValue, Language::Html, "<p class=\"w-[13px] p-4\"></p>\n").len(), 1);
    }
}

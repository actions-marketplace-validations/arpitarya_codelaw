//! Rules whose fix is a rewrite no declarative step expresses (W-162;
//! SR-RULE-FORMAT decision 3c's native fixes): `js/prefer-template`, a `+`
//! chain of strings and values written as one template literal.

use crate::util::text;
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

fn is_plus(n: Node<'_>, src: &str) -> bool {
    n.kind() == "binary_expression" && n.child_by_field_name("operator").is_some_and(|o| text(o, src) == "+")
}

/// The operands of a left-leaning `+` chain, in order.
fn operands<'t>(n: Node<'t>, src: &str, out: &mut Vec<Node<'t>>) {
    match (n.child_by_field_name("left"), n.child_by_field_name("right")) {
        (Some(l), Some(r)) if is_plus(n, src) => {
            operands(l, src, out);
            out.push(r);
        }
        _ => out.push(n),
    }
}

/// A string literal's text inside its quotes, ready for a template: its
/// escapes kept (each is valid in a template too), `` ` `` and `${` escaped.
fn string_part(s: &str) -> Option<String> {
    let inner = s.get(1..s.len().checked_sub(1)?)?;
    let mut out = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let next = chars.next()?;
                // A line continuation would become a real line break.
                if next == '\n' || next == '\r' {
                    return None;
                }
                out.push('\\');
                out.push(next);
            }
            '`' => out.push_str("\\`"),
            '$' if chars.peek() == Some(&'{') => out.push_str("\\$"),
            _ => out.push(c),
        }
    }
    Some(out)
}

/// The template literal for a `+` chain whose first string comes first or
/// second, so every `+` in it joins strings; `None` when the chain holds a
/// comment, a template with no text to join to, or a string it can't copy.
fn template(n: Node<'_>, src: &str) -> Option<String> {
    let mut parts = Vec::new();
    operands(n, src, &mut parts);
    let first_string = parts.iter().position(|p| p.kind() == "string")?;
    // `a + b + "c"` adds `a` and `b` first: they stay one expression.
    let (head, rest) = if first_string <= 1 { (Vec::new(), &parts[..]) } else { (parts[..first_string].to_vec(), &parts[first_string..]) };
    let mut c = n.walk();
    let mut stack = vec![n];
    while let Some(x) = stack.pop() {
        if x.kind() == "comment" {
            return None;
        }
        if !matches!(x.kind(), "string" | "template_string") {
            stack.extend(x.children(&mut c));
        }
    }
    let mut out = String::from("`");
    if let (Some(a), Some(b)) = (head.first(), head.last()) {
        out.push_str(&format!("${{{}}}", &src[a.start_byte()..b.end_byte()]));
    }
    for p in rest {
        match p.kind() {
            "string" => out.push_str(&string_part(text(*p, src))?),
            "template_string" => {
                let t = text(*p, src);
                out.push_str(t.get(1..t.len() - 1)?);
            }
            _ => out.push_str(&format!("${{{}}}", text(*p, src))),
        }
    }
    out.push('`');
    Some(out)
}

pub struct PreferTemplate;

impl NativeRule for PreferTemplate {
    fn id(&self) -> &'static str {
        "js/prefer-template"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["binary_expression"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        if !is_plus(n, src) || n.parent().is_some_and(|p| is_plus(p, src)) {
            return;
        }
        // As the rule matched before its fix (W-162): a string beside a
        // value that is neither a literal nor another `+`.
        let mut c = n.walk();
        let kids: Vec<Node<'_>> = n.named_children(&mut c).collect();
        let has_string = kids.iter().any(|k| k.kind() == "string");
        let has_value = kids.iter().any(|k| !matches!(k.kind(), "string" | "template_string" | "binary_expression"));
        if !(has_string && has_value) {
            return;
        }
        let edits = template(n, src).map(|t| vec![TextEdit::new(n.start_byte(), n.end_byte(), t)]).unwrap_or_default();
        out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: "Build the string with a template literal".into(), edits });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(src: &str) -> Vec<String> {
        crate::run(&PreferTemplate, Language::Javascript, src)
            .into_iter()
            .map(|f| f.edits.first().map(|e| e.text.clone()).unwrap_or_default())
            .collect()
    }

    #[test]
    fn a_string_and_a_value_become_one_template() {
        assert_eq!(fixed("const s = \"Hello, \" + name;\n"), vec!["`Hello, ${name}`"]);
        assert_eq!(fixed("const s = n + \" items\";\n"), vec!["`${n} items`"]);
        assert_eq!(fixed("const s = x + \"`${y}\\n\";\n"), vec!["`${x}\\`\\${y}\\n`"]);
        assert_eq!(fixed("const s = 'it\\'s ' + n;\n"), vec!["`it\\'s ${n}`"]);
        assert_eq!(fixed("const s = \"a\" + /* why */ n;\n"), vec![""]);
        assert!(fixed("const s = \"a\" + \"b\";\n").is_empty());
        // Only the outer `+` is reported; its whole chain is rewritten.
        assert_eq!(template_of("(a + b) + \" total\""), Some("`${(a + b)} total`".into()));
        assert_eq!(template_of("a + b + \" total \" + n"), Some("`${a + b} total ${n}`".into()));
    }

    fn template_of(expr: &str) -> Option<String> {
        let src = format!("{expr};\n");
        let root = codelaw_lang::parse(Language::Javascript, &src);
        let stmt = root.root().get_inner_node().named_child(0)?;
        template(stmt.named_child(0)?, &src)
    }
}

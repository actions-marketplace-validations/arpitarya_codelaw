//! Getters must return a value on every path (`js/getter-return`); setters
//! must not return one (`js/no-setter-return`). Both need the paths through
//! the body, which a pattern can't follow.

use crate::util::{descendants, has_token, is_function, returns_value};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

fn accessor_body<'t>(n: Node<'t>, token: &str) -> Option<(Node<'t>, Node<'t>)> {
    if n.kind() != "method_definition" || !has_token(n, token) {
        return None;
    }
    Some((n.child_by_field_name("name")?, n.child_by_field_name("body")?))
}

pub struct GetterReturn;

impl NativeRule for GetterReturn {
    fn id(&self) -> &'static str {
        "js/getter-return"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["method_definition"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some((name, body)) = accessor_body(node.get_inner_node(), "get") else { return };
        if !returns_value(&body) {
            // The path that falls off the end returns `undefined` already;
            // say so, before the closing brace.
            let src = ctx.source;
            let close = body.end_byte() - 1;
            let line = src[..close].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let edits = if src[line..close].trim().is_empty() && line > body.start_byte() {
                let pad = &src[line..close];
                vec![TextEdit::insert(line, format!("{pad}  return undefined;\n"))]
            } else {
                Vec::new()
            };
            out.push(NativeFinding { start: name.start_byte(), end: name.end_byte(), message: "This getter doesn't return a value on every path".into(), edits });
        }
    }
}

pub struct NoSetterReturn;

impl NativeRule for NoSetterReturn {
    fn id(&self) -> &'static str {
        "js/no-setter-return"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["method_definition"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let Some((_, body)) = accessor_body(node.get_inner_node(), "set") else { return };
        let mut inside = Vec::new();
        descendants(body, &|c| is_function(c.kind()) || c.kind() == "arrow_function", &mut inside);
        for r in inside.into_iter().filter(|r| r.kind() == "return_statement" && r.named_child_count() > 0) {
            // `return e;` is `e; return;`: the value goes, its evaluation
            // stays; a last `return` goes too, and a plain value with it.
            let Some(mut e) = r.named_child(0) else { continue };
            while e.kind() == "parenthesized_expression" && e.named_child_count() == 1 {
                e = e.named_child(0).expect("one child");
            }
            let pure = matches!(e.kind(), "identifier" | "number" | "string" | "true" | "false" | "null" | "undefined" | "this");
            let last = r.parent().is_some_and(|p| p.id() == body.id()) && r.next_named_sibling().is_none();
            let expr = crate::util::text(e, src);
            let replacement = match (pure, last) {
                (true, true) => String::new(),
                (true, false) => "return;".to_owned(),
                (false, true) => format!("{expr};"),
                (false, false) => format!("{expr};\n{}return;", crate::python_util::indent_at(src, r.start_byte())),
            };
            let edit = if replacement.is_empty() { crate::util::remove_statement(r, src) } else { TextEdit::new(r.start_byte(), r.end_byte(), replacement) };
            out.push(NativeFinding { start: r.start_byte(), end: r.end_byte(), message: "A setter's return value is ignored; don't return one".into(), edits: vec![edit] });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(rule: &dyn NativeRule, src: &str) -> usize {
        crate::run(rule, Language::Javascript, src).len()
    }

    #[test]
    fn getters_and_setters() {
        assert_eq!(count(&GetterReturn, "o = { get a() {}, get b() { if (x) return 1; }, get c() { return 1; }, get d() { if (x) return 1; else throw e; } };"), 2);
        assert_eq!(count(&GetterReturn, "class A { get a() { switch (x) { case 1: return 1; default: return 2; } } get b() { return; } }"), 1);
        assert_eq!(count(&NoSetterReturn, "class A { set a(v) { return v; } set b(v) { if (v) return; this.x = () => v; } }"), 1);
    }
}

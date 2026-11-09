//! W-139 strand W-139-js: eslint-plugin-sonarjs 4.2.2's `prefer-type-guard`
//! (behaviour only: SonarJS is source-available, nothing here is copied),
//! written again. A TypeScript function or method whose only statement
//! returns a check on a cast of its one parameter can say so in its return
//! type.

use super::{TS, finding};
use crate::js_ast::params;
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

fn strip_parens(mut n: Node<'_>) -> Node<'_> {
    while n.kind() == "parenthesized_expression" {
        match n.named_child(0) {
            Some(x) => n = x,
            None => break,
        }
    }
    n
}

/// `(x as T).prop` / `(<T>x).prop`: the cast expression and the type.
fn cast_member<'t>(n: Node<'t>) -> Option<(Node<'t>, Node<'t>)> {
    let n = strip_parens(n);
    if n.kind() != "member_expression" {
        return None;
    }
    let obj = strip_parens(n.child_by_field_name("object")?);
    match obj.kind() {
        "as_expression" => Some((obj.named_child(0)?, obj.named_child(1)?)),
        "type_assertion" => {
            let kids = named_children(obj);
            let ty = kids.iter().find(|c| c.kind() == "type_arguments").and_then(|a| a.named_child(0))?;
            Some((*kids.last()?, ty))
        }
        _ => None,
    }
}

pub struct PreferTypeGuard;
impl NativeRule for PreferTypeGuard {
    fn id(&self) -> &'static str {
        "js/prefer-type-guard"
    }
    fn languages(&self) -> &'static [Language] {
        TS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["function_declaration", "method_definition"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let f = node.get_inner_node();
        let src = ctx.source;
        if f.kind() == "method_definition" && f.child_by_field_name("name").is_some_and(|n| matches!(text(n, src), "get" | "set")) && f.children(&mut f.walk()).any(|c| matches!(c.kind(), "get" | "set")) {
            return;
        }
        if f.child_by_field_name("return_type").is_some_and(|r| text(r, src).contains(" is ") || text(r, src).starts_with(": asserts")) {
            return;
        }
        let Some(body) = f.child_by_field_name("body").filter(|b| b.kind() == "statement_block") else { return };
        let stmts: Vec<Node<'_>> = named_children(body).into_iter().filter(|c| c.kind() != "comment").collect();
        let [ret] = stmts[..] else { return };
        if ret.kind() != "return_statement" {
            return;
        }
        let Some(expr) = named_children(ret).into_iter().find(|c| c.kind() != "comment") else { return };
        let expr = strip_parens(expr);
        let cast = match expr.kind() {
            "binary_expression" if expr.child_by_field_name("operator").is_some_and(|o| matches!(text(o, src), "!==" | "!=")) => {
                let (l, r) = (expr.child_by_field_name("left"), expr.child_by_field_name("right"));
                match (l, r) {
                    (Some(l), Some(r)) if r.kind() == "undefined" => cast_member(l),
                    (Some(l), Some(r)) if l.kind() == "undefined" => cast_member(r),
                    _ => None,
                }
            }
            "call_expression" => {
                let args = crate::js_ast::arguments(expr);
                if expr.child_by_field_name("function").is_some_and(|c| c.kind() == "identifier" && text(c, src) == "Boolean") && args.len() == 1 { cast_member(args[0]) } else { None }
            }
            "unary_expression" if expr.child(0).is_some_and(|c| c.kind() == "!") => {
                let inner = strip_parens(expr.child_by_field_name("argument").unwrap_or(expr));
                if inner.kind() == "unary_expression" && inner.child(0).is_some_and(|c| c.kind() == "!") { cast_member(inner.child_by_field_name("argument").unwrap_or(inner)) } else { None }
            }
            _ => None,
        };
        let Some((subject, ty)) = cast else { return };
        if text(ty, src) == "any" {
            return;
        }
        let nparams = params(f).len();
        if !(nparams == 1 || (nparams == 0 && subject.kind() == "this")) {
            return;
        }
        let predicate = format!(": {} is {}", text(subject, src), text(ty, src));
        let edit = match f.child_by_field_name("return_type") {
            Some(r) => TextEdit::new(r.start_byte(), r.end_byte(), predicate),
            None => {
                let Some(ps) = f.child_by_field_name("parameters") else { return };
                TextEdit::new(ps.end_byte(), ps.end_byte(), predicate)
            }
        };
        let at = f.child_by_field_name("name").unwrap_or(f);
        out.push(finding(at, format!("This function only checks `{}`: declare it as a type guard, `{} is {}`", text(subject, src), text(subject, src), text(ty, src)), vec![edit]));
    }
}

//! `js/consistent-function-style`: the syntax a function takes, by its role.
//! eslint-plugin-unicorn 79.0.0's rule, read and reimplemented. Every role's
//! style is `ignore` until the project sets one, so the rule does nothing
//! until then.

use super::{at, finding, opt_str};
use crate::util::{children, text};
use codelaw_core::TextEdit;
use tree_sitter::Node;

/// The roles, with the option that styles each and the name the message uses.
const ROLES: &[(&str, &str)] = &[
    ("default", "function"),
    ("defaultExport", "default export"),
    ("namedFunctions", "named function"),
    ("namedExports", "named export"),
    ("callbacks", "callback"),
    ("objectProperties", "object property"),
    ("reassignedVariables", "reassigned variable"),
    ("typedVariables", "typed variable"),
];

fn role_name(role: &str) -> &'static str {
    ROLES.iter().find(|(r, _)| *r == role).map_or("function", |(_, n)| n)
}

fn style_name(style: &str) -> &'static str {
    match style {
        "declaration" => "a function declaration",
        "function-expression" => "a function expression",
        "arrow-function" => "an arrow function",
        _ => "a method",
    }
}

const NEEDS: &[&str] = &["default", "defaultExport", "namedFunctions", "namedExports", "callbacks", "objectProperties", "reassignedVariables", "typedVariables"];

/// The node's parent through parentheses.
fn parent_of<'t>(n: Node<'t>) -> Option<Node<'t>> {
    let mut p = n.parent()?;
    while p.kind() == "parenthesized_expression" {
        p = p.parent()?;
    }
    Some(p)
}

/// The node, with the parentheses around it that its parent would skip.
fn outer<'t>(n: Node<'t>) -> Node<'t> {
    let mut x = n;
    while let Some(p) = x.parent() {
        if p.kind() == "parenthesized_expression" { x = p } else { break }
    }
    x
}

fn is_variable_function(n: Node<'_>) -> Option<Node<'_>> {
    let p = parent_of(n)?;
    (p.kind() == "variable_declarator" && p.child_by_field_name("value").is_some_and(|v| outer(n).id() == v.id() || v.id() == n.id()) && p.child_by_field_name("name").is_some_and(|x| x.kind() == "identifier")).then_some(p)
}

fn is_default_export(p: Node<'_>) -> bool {
    p.kind() == "export_statement" && children(p).iter().any(|c| c.kind() == "default")
}

fn is_named_export(n: Node<'_>) -> bool {
    let Some(p) = parent_of(n) else { return false };
    if p.kind() == "export_statement" && !is_default_export(p) {
        return true;
    }
    is_variable_function(n).is_some_and(|d| d.parent().and_then(|l| l.parent()).is_some_and(|e| e.kind() == "export_statement"))
}

fn is_callback(n: Node<'_>) -> bool {
    let o = outer(n);
    o.parent().is_some_and(|p| p.kind() == "arguments" && p.parent().is_some_and(|g| matches!(g.kind(), "call_expression" | "new_expression")))
}

fn actual_style(n: Node<'_>) -> &'static str {
    match n.kind() {
        "function_declaration" | "generator_function_declaration" => "declaration",
        "arrow_function" => "arrow-function",
        "method_definition" => "method",
        _ => "function-expression",
    }
}

/// Source text of the parameters without their parentheses.
fn params_text<'s>(f: Node<'_>, src: &'s str) -> &'s str {
    if let Some(p) = f.child_by_field_name("parameter") {
        return text(p, src);
    }
    f.child_by_field_name("parameters").map_or("", |p| {
        let t = text(p, src);
        t.strip_prefix('(').and_then(|x| x.strip_suffix(')')).unwrap_or(t)
    })
}

/// Whether the conversion would change what `this`, `arguments`, `super` or
/// `new.target` mean, or drop a comment.
fn unsafe_to_convert(f: Node<'_>) -> bool {
    let mut stack = children(f);
    while let Some(n) = stack.pop() {
        if matches!(n.kind(), "this" | "super" | "meta_property" | "comment") {
            return true;
        }
        stack.extend(children(n));
    }
    false
}

fn mentions_arguments(f: Node<'_>, src: &str) -> bool {
    let mut stack = children(f);
    while let Some(n) = stack.pop() {
        if n.kind() == "identifier" && text(n, src) == "arguments" {
            return true;
        }
        stack.extend(children(n));
    }
    false
}

super::orule!(ConsistentFunctionStyle, "js/consistent-function-style", NEEDS, &["function_declaration", "generator_function_declaration", "function_expression", "function", "generator_function", "arrow_function", "method_definition"], true, |n, ctx, out| {
    let src = ctx.source;
    let Some(p) = parent_of(n) else { return };
    let has_name = n.child_by_field_name("name").is_some();
    if n.kind() == "method_definition" {
        // Only a method of an object literal has a role; a class's methods have none.
        if p.kind() != "object" || children(n).iter().any(|c| matches!(c.kind(), "get" | "set")) {
            return;
        }
    } else if n.child_by_field_name("body").is_none() {
        return;
    }
    let default_export_set = ctx.options.contains_key("defaultExport");
    let is_default = is_default_export(p);
    if (is_default && !has_name && !default_export_set)
        || matches!(p.kind(), "field_definition" | "public_field_definition" | "assignment_pattern")
        || (p.kind() == "pair" && p.child_by_field_name("value").is_none())
        || (p.kind() == "call_expression" && p.child_by_field_name("function").is_some_and(|f| outer(n).id() == f.id()))
        || (p.kind() == "new_expression" && p.child_by_field_name("constructor").is_some_and(|f| outer(n).id() == f.id()))
    {
        return;
    }
    let var = is_variable_function(n);
    let model = ctx.semantic;
    let role = if default_export_set && is_default {
        "defaultExport"
    } else if var.is_some_and(|d| d.child_by_field_name("type").is_some()) {
        "typedVariables"
    } else if var.is_some_and(|d| d.child_by_field_name("name").is_some_and(|id| model.and_then(|m| m.symbol_at(id.start_byte())).is_some_and(|s| model.is_some_and(|m| m.is_reassigned(s))))) {
        "reassignedVariables"
    } else if is_named_export(n) {
        "namedExports"
    } else if is_callback(n) {
        "callbacks"
    } else if n.kind() == "method_definition" || (p.kind() == "pair" && p.child_by_field_name("value").is_some_and(|v| outer(n).id() == v.id())) {
        "objectProperties"
    } else if matches!(n.kind(), "function_declaration" | "generator_function_declaration") || var.is_some() {
        "namedFunctions"
    } else {
        "default"
    };
    let Some(expected) = opt_str(ctx, role) else { return };
    let actual = actual_style(n);
    let generator = matches!(n.kind(), "generator_function_declaration" | "generator_function") || children(n).iter().any(|c| c.kind() == "*");
    if expected == "ignore" || expected == actual || (generator && expected == "arrow-function") || !["declaration", "function-expression", "arrow-function", "method"].contains(&expected) {
        return;
    }
    let message = format!("Expected {} for this {}, but found {}.", style_name(expected), role_name(role), style_name(actual));
    // A suggestion only for a callback written the other way round, where the conversion is safe.
    let swap = (actual == "function-expression" && expected == "arrow-function") || (actual == "arrow-function" && expected == "function-expression");
    let can_suggest = role == "callbacks"
        && p.kind() == "arguments"
        && p.parent().is_some_and(|g| g.kind() == "call_expression")
        && swap
        && !has_name
        && !generator
        && n.child_by_field_name("type_parameters").is_none()
        && !children(n).iter().any(|c| c.kind() == "formal_parameters" && text(*c, src).contains("this"))
        && !unsafe_to_convert(n)
        && !mentions_arguments(n, src);
    if !can_suggest {
        out.push(at(n, message));
        return;
    }
    let Some(body) = n.child_by_field_name("body") else { return };
    let is_async = children(n).iter().any(|c| c.kind() == "async");
    let ret = n.child_by_field_name("return_type").map(|r| text(r, src)).unwrap_or("");
    let params = params_text(n, src);
    let pre = if is_async { "async " } else { "" };
    let replacement = if expected == "arrow-function" {
        format!("{pre}({params}){ret} => {}", text(body, src))
    } else if body.kind() == "statement_block" {
        format!("{pre}function ({params}){ret} {}", text(body, src))
    } else {
        format!("{pre}function ({params}){ret} {{ return {}; }}", text(body, src))
    };
    out.push(finding(n.start_byte(), n.end_byte(), message, vec![TextEdit::new(n.start_byte(), n.end_byte(), replacement)]));
});

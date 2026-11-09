//! The code a JSDoc block documents: the node right after the comment, and
//! the function (and its parameters) inside it.

use crate::util::{children, named_children, text};
use tree_sitter::Node;

/// The node a block documents: the one right after the comment, up to one
/// blank line away, with no other comment between.
pub fn target<'t>(comment: Node<'t>) -> Option<Node<'t>> {
    let n = comment.next_named_sibling()?;
    if n.kind() == "comment" || n.start_position().row > comment.end_position().row + 2 {
        return None;
    }
    Some(n)
}

/// A name a function takes, or an opaque destructured parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PName {
    Name(String),
    This,
    Destructured,
}

/// The function a documented node stands for, through `export`, a variable's
/// initializer, an assignment, an object property or a class field.
pub fn function_of<'t>(n: Node<'t>) -> Option<Node<'t>> {
    match n.kind() {
        "function_declaration" | "generator_function_declaration" | "function_signature" | "method_definition" | "method_signature" | "abstract_method_signature" | "function_expression" | "function" | "generator_function" | "arrow_function" => Some(n),
        "export_statement" => n.child_by_field_name("declaration").or_else(|| n.child_by_field_name("value")).and_then(function_of),
        "lexical_declaration" | "variable_declaration" => named_children(n).into_iter().find(|c| c.kind() == "variable_declarator").and_then(|d| d.child_by_field_name("value")).and_then(function_of),
        "expression_statement" => {
            let e = n.named_child(0)?;
            if e.kind() == "assignment_expression" { e.child_by_field_name("right").and_then(function_of) } else { function_of(e) }
        }
        "pair" | "public_field_definition" | "field_definition" => n.child_by_field_name("value").and_then(function_of),
        "property_signature" => {
            let t = n.child_by_field_name("type")?;
            named_children(t).into_iter().find(|c| c.kind() == "function_type")
        }
        "function_type" => Some(n),
        _ => None,
    }
}

/// The parameters of a function node.
pub fn params(f: Node<'_>, src: &str) -> Vec<PName> {
    if let Some(p) = f.child_by_field_name("parameter") {
        return vec![pname(p, src)];
    }
    let Some(list) = f.child_by_field_name("parameters") else { return Vec::new() };
    named_children(list).into_iter().filter(|c| c.kind() != "comment").map(|c| pname(c, src)).collect()
}

fn pname(p: Node<'_>, src: &str) -> PName {
    match p.kind() {
        "identifier" => PName::Name(text(p, src).to_owned()),
        "this" => PName::This,
        "required_parameter" | "optional_parameter" => match p.child_by_field_name("pattern") {
            Some(x) => pname(x, src),
            None => PName::Destructured,
        },
        "assignment_pattern" => p.child_by_field_name("left").map_or(PName::Destructured, |l| pname(l, src)),
        "rest_pattern" | "rest_parameter" => children(p).into_iter().find(|c| c.kind() == "identifier").map_or(PName::Destructured, |i| PName::Name(text(i, src).to_owned())),
        _ => PName::Destructured,
    }
}

//! `js/no-implicit-globals`: declarations and assignments that create globals
//! in a classic script. ESLint 10.13.0 reads a `.js` file as a module, where
//! a top-level declaration is module-scoped and nothing leaks, so the rule
//! does nothing there until the project names the files that are scripts
//! (`sourceType: "script"` in ESLint's config). Here the option `script` does
//! it, in a `[scope.<name>]` over those files.

use super::{JS, at, opt_bool};
use crate::util::{children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

pub struct NoImplicitGlobals;

/// The names a `/* exported a, b */` comment exempts.
fn exported_names(root: Node<'_>, src: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "comment" {
            let t = text(n, src);
            if let Some(body) = t.strip_prefix("/*").and_then(|b| b.strip_suffix("*/")) {
                let body = body.trim_start();
                if let Some(list) = body.strip_prefix("exported").filter(|r| r.starts_with(char::is_whitespace)) {
                    names.extend(list.split(|c: char| c == ',' || c.is_whitespace()).filter(|s| !s.is_empty()).map(str::to_owned));
                }
            }
        } else {
            stack.extend(children(n));
        }
    }
    names
}

/// The identifiers a declarator's name binds (a pattern binds several).
fn bound<'t>(name: Node<'t>, out: &mut Vec<Node<'t>>) {
    match name.kind() {
        "identifier" | "shorthand_property_identifier_pattern" => out.push(name),
        "object_pattern" | "array_pattern" | "pair_pattern" | "assignment_pattern" | "object_assignment_pattern" | "rest_pattern" => {
            for c in children(name) {
                if c.kind() == "pair_pattern" {
                    if let Some(v) = c.child_by_field_name("value") {
                        bound(v, out);
                    }
                } else if c.is_named() {
                    bound(c, out);
                }
            }
        }
        _ => {}
    }
}

fn declarators<'t>(decl: Node<'t>) -> Vec<Node<'t>> {
    children(decl).into_iter().filter(|c| c.kind() == "variable_declarator").collect()
}

/// `var` declarations anywhere outside a function (they hoist to the global scope).
fn collect_vars<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in children(n) {
        match c.kind() {
            "variable_declaration" => out.push(c),
            "function_declaration" | "generator_function_declaration" | "function_expression" | "function" | "generator_function" | "arrow_function" | "class_declaration" | "class" | "method_definition" => {}
            _ => collect_vars(c, out),
        }
    }
}

impl NativeRule for NoImplicitGlobals {
    fn id(&self) -> &'static str {
        "js/no-implicit-globals"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn needs_options(&self) -> &'static [&'static str] {
        &["script"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        if !opt_bool(ctx, "script", false) {
            return;
        }
        let program = node.get_inner_node();
        let src = ctx.source;
        let top = children(program);
        // A file that imports or exports is a module, whatever the project says.
        if top.iter().any(|c| matches!(c.kind(), "import_statement" | "export_statement")) {
            return;
        }
        let lexical = opt_bool(ctx, "lexicalBindings", false);
        let exported = exported_names(program, src);
        let exempt = |n: Node<'_>| exported.iter().any(|e| e == text(n, src));
        let non_lexical = "wrap in an IIFE for a local variable, assign as global property for a global variable";
        // `var` anywhere outside a function, and a function declared at the top.
        let mut vars = Vec::new();
        collect_vars(program, &mut vars);
        for d in vars {
            for decl in declarators(d) {
                let Some(name) = decl.child_by_field_name("name") else { continue };
                if name.kind() == "identifier" {
                    if !exempt(name) {
                        out.push(at(decl, format!("Unexpected 'var' declaration in the global scope, {non_lexical}.")));
                    }
                } else {
                    let mut ids = Vec::new();
                    bound(name, &mut ids);
                    for id in ids.into_iter().filter(|i| !exempt(*i)) {
                        out.push(at(id, format!("Unexpected 'var' declaration in the global scope, {non_lexical}.")));
                    }
                }
            }
        }
        for c in &top {
            match c.kind() {
                "function_declaration" | "generator_function_declaration" if c.child_by_field_name("name").is_some_and(|n| !exempt(n)) => {
                    out.push(at(*c, format!("Unexpected function declaration in the global scope, {non_lexical}.")));
                }
                "lexical_declaration" if lexical => {
                    let kind = if text(*c, src).starts_with("const") { "const" } else { "let" };
                    for decl in declarators(*c) {
                        let Some(name) = decl.child_by_field_name("name") else { continue };
                        let mut ids = Vec::new();
                        bound(name, &mut ids);
                        for id in ids.into_iter().filter(|i| !exempt(*i)) {
                            out.push(at(if name.kind() == "identifier" { decl } else { id }, format!("Unexpected '{kind}' declaration in the global scope, wrap in a block or in an IIFE.")));
                        }
                    }
                }
                "class_declaration" if lexical && c.child_by_field_name("name").is_some_and(|n| !exempt(n)) => {
                    out.push(at(*c, "Unexpected class declaration in the global scope, wrap in a block or in an IIFE."));
                }
                _ => {}
            }
        }
        // A write to a name nothing declares creates a global when the code is not strict.
        if let Some(model) = ctx.semantic {
            for r in &model.references {
                if r.write && !r.read && r.symbol.is_none() && model.is_undeclared(r, &|_| false) && !exported.contains(&r.name) {
                    out.push(NativeFinding { start: r.start, end: r.end, message: "Global variable leak, declare the variable if it is intended to be local.".into(), edits: Vec::new() });
                }
            }
        }
    }
}

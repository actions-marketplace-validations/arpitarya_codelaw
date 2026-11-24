//! W-190: the two rules on JavaScript's `using` declaration, eslint-plugin-
//! unicorn 79.0.0's `no-using-resource-escape` and `prefer-dispose`, written
//! from the plugin's documented behaviour. JavaScript and JSX only: the pinned
//! TypeScript grammar (0.23.2) does not parse `using`.

use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx];

pub(crate) static RULES: &[&dyn NativeRule] = &[&NoUsingResourceEscape, &PreferDispose];

fn finding(n: Node<'_>, message: impl Into<String>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits: Vec::new() }
}

fn is_function(n: Node<'_>) -> bool {
    matches!(n.kind(), "arrow_function" | "function_expression" | "function" | "function_declaration" | "generator_function" | "generator_function_declaration" | "method_definition")
}

/// Names a `using` declaration binds (plain identifiers only).
fn bound_names<'s>(decl: Node<'_>, src: &'s str) -> Vec<&'s str> {
    named_children(decl).into_iter().filter(|c| c.kind() == "variable_declarator").filter_map(|d| d.child_by_field_name("name").filter(|n| n.kind() == "identifier").map(|n| text(n, src))).collect()
}

/// Whether `n` reads one of `names` (an identifier, not a property name, not
/// shadowed by a parameter of the function it sits in).
fn references(n: Node<'_>, names: &[&str], src: &str) -> bool {
    let mut stack = vec![n];
    while let Some(x) = stack.pop() {
        if matches!(x.kind(), "identifier" | "shorthand_property_identifier") && names.contains(&text(x, src)) {
            let mut shadowed = false;
            let mut p = x.parent();
            while let Some(f) = p {
                if is_function(f)
                    && let Some(params) = f.child_by_field_name("parameters").or_else(|| f.child_by_field_name("parameter"))
                {
                    let mut ps = vec![params];
                    while let Some(q) = ps.pop() {
                        if q.kind() == "identifier" && text(q, src) == text(x, src) {
                            shadowed = true;
                        }
                        ps.extend(named_children(q));
                    }
                }
                p = f.parent();
            }
            if !shadowed {
                return true;
            }
        }
        let mut c = x.walk();
        stack.extend(x.children(&mut c));
    }
    false
}

/// What of `expr` leaves the scope: the resource itself, or a function that
/// captures one. Pushes the offending nodes.
fn escapes<'t>(expr: Node<'t>, scope: Node<'t>, names: &[&str], src: &str, out: &mut Vec<(Node<'t>, bool)>) {
    match expr.kind() {
        "parenthesized_expression" => named_children(expr).into_iter().for_each(|c| escapes(c, scope, names, src, out)),
        "identifier" => {
            let t = text(expr, src);
            if names.contains(&t) {
                out.push((expr, false));
            } else if let Some(f) = local_function(scope, t, src)
                && references(f, names, src)
            {
                out.push((expr, true));
            }
        }
        "arrow_function" | "function_expression" | "function" | "generator_function" => {
            if references(expr, names, src) {
                out.push((expr, true));
            }
        }
        "array" => named_children(expr).into_iter().for_each(|c| escapes(c, scope, names, src, out)),
        "object" => {
            for c in named_children(expr) {
                match c.kind() {
                    "pair" => {
                        if let Some(v) = c.child_by_field_name("value") {
                            escapes(v, scope, names, src, out);
                        }
                    }
                    "method_definition" => {
                        if references(c, names, src) {
                            out.push((c, true));
                        }
                    }
                    "shorthand_property_identifier" if names.contains(&text(c, src)) => out.push((c, false)),
                    _ => {}
                }
            }
        }
        "ternary_expression" => {
            for f in ["consequence", "alternative"] {
                if let Some(c) = expr.child_by_field_name(f) {
                    escapes(c, scope, names, src, out);
                }
            }
        }
        "binary_expression" => {
            let op = expr.child_by_field_name("operator").map(|o| text(o, src)).unwrap_or("");
            if matches!(op, "&&" | "||" | "??") {
                for f in ["left", "right"] {
                    if let Some(c) = expr.child_by_field_name(f) {
                        escapes(c, scope, names, src, out);
                    }
                }
            }
        }
        "sequence_expression" => {
            if let Some(last) = named_children(expr).into_iter().last() {
                escapes(last, scope, names, src, out);
            }
        }
        _ => {}
    }
}

/// A function the scope declares under `name`: a function declaration, or a
/// `const` initialised directly with a function.
fn local_function<'t>(scope: Node<'t>, name: &str, src: &str) -> Option<Node<'t>> {
    for s in named_children(scope) {
        match s.kind() {
            "function_declaration" | "generator_function_declaration" => {
                if s.child_by_field_name("name").is_some_and(|n| text(n, src) == name) {
                    return Some(s);
                }
            }
            "lexical_declaration" if s.child(0).is_some_and(|k| text(k, src) == "const") => {
                for d in named_children(s) {
                    if d.kind() == "variable_declarator"
                        && d.child_by_field_name("name").is_some_and(|n| text(n, src) == name)
                        && let Some(v) = d.child_by_field_name("value")
                        && matches!(v.kind(), "arrow_function" | "function_expression" | "function")
                    {
                        return Some(v);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Every `return` of the scope's own body (not of a function nested in it).
fn returns<'t>(scope: Node<'t>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    let mut stack = named_children(scope);
    while let Some(n) = stack.pop() {
        if is_function(n) || n.kind() == "class_declaration" {
            continue;
        }
        if n.kind() == "return_statement" {
            out.push(n);
        }
        stack.extend(named_children(n));
    }
    out
}

pub struct NoUsingResourceEscape;

impl NativeRule for NoUsingResourceEscape {
    fn id(&self) -> &'static str {
        "js/no-using-resource-escape"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["using_declaration"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let decl = node.get_inner_node();
        let src = ctx.source;
        let Some(scope) = decl.parent() else { return };
        let names = bound_names(decl, src);
        if names.is_empty() {
            return;
        }
        let mut hits: Vec<(Node<'_>, bool)> = Vec::new();
        for r in returns(scope) {
            if let Some(arg) = named_children(r).into_iter().find(|c| c.kind() != "comment") {
                escapes(arg, scope, &names, src, &mut hits);
            }
        }
        if scope.kind() == "program" {
            for s in named_children(scope).into_iter().filter(|s| s.kind() == "export_statement") {
                if s.child_by_field_name("source").is_some() {
                    continue;
                }
                for c in named_children(s) {
                    match c.kind() {
                        "export_clause" => {
                            for sp in named_children(c).into_iter().filter(|x| x.kind() == "export_specifier") {
                                if let Some(local) = sp.child_by_field_name("name")
                                    && names.contains(&text(local, src))
                                {
                                    hits.push((local, false));
                                } else if let Some(local) = sp.child_by_field_name("name")
                                    && let Some(f) = local_function(scope, text(local, src), src)
                                    && references(f, &names, src)
                                {
                                    hits.push((local, true));
                                }
                            }
                        }
                        "lexical_declaration" | "variable_declaration" => {
                            for d in named_children(c).into_iter().filter(|d| d.kind() == "variable_declarator") {
                                if let Some(v) = d.child_by_field_name("value") {
                                    escapes(v, scope, &names, src, &mut hits);
                                }
                            }
                        }
                        "decorator" | "comment" => {}
                        _ => {
                            if s.child_by_field_name("value").is_some_and(|v| v.id() == c.id()) {
                                escapes(c, scope, &names, src, &mut hits);
                            }
                        }
                    }
                }
            }
        }
        hits.sort_by_key(|(n, _)| n.start_byte());
        hits.dedup_by_key(|(n, _)| n.start_byte());
        for (n, captures) in hits {
            let what = text(n, src);
            let message = if captures {
                "This function captures a resource declared with `using`, which is disposed when its scope exits; run the work inside the function, or have the function own the resource".to_owned()
            } else {
                format!("`{what}` is declared with `using` and is disposed when its scope exits; returning or exporting it hands later code a disposed resource: return what it produces instead")
            };
            out.push(finding(n, message));
        }
    }
}

/// Methods a `finally` calls to let go of a resource.
const DISPOSERS: &[&str] = &["close", "dispose", "destroy", "end", "release", "disconnect", "unlock", "free"];

pub struct PreferDispose;

impl NativeRule for PreferDispose {
    fn id(&self) -> &'static str {
        "js/prefer-dispose"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["try_statement"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let t = node.get_inner_node();
        let src = ctx.source;
        let kids = named_children(t);
        if kids.iter().any(|c| c.kind() == "catch_clause") {
            return;
        }
        let (Some(body), Some(fin)) = (t.child_by_field_name("body"), kids.iter().find(|c| c.kind() == "finally_clause").and_then(|f| f.child_by_field_name("body").or_else(|| named_children(*f).into_iter().find(|b| b.kind() == "statement_block")))) else { return };
        // What the `finally` does: one disposer call per statement.
        let mut disposed: Vec<(&str, bool)> = Vec::new();
        for s in named_children(fin).into_iter().filter(|s| s.kind() != "comment") {
            let Some(mut e) = (s.kind() == "expression_statement").then(|| named_children(s).into_iter().next()).flatten() else { return };
            let awaited = e.kind() == "await_expression";
            if awaited {
                let Some(inner) = named_children(e).into_iter().next() else { return };
                e = inner;
            }
            let Some(callee) = (e.kind() == "call_expression").then(|| e.child_by_field_name("function")).flatten().filter(|c| c.kind() == "member_expression") else { return };
            let (Some(obj), Some(prop)) = (callee.child_by_field_name("object"), callee.child_by_field_name("property")) else { return };
            if obj.kind() != "identifier" || !DISPOSERS.contains(&text(prop, src)) || named_children(e.child_by_field_name("arguments").unwrap_or(e)).iter().any(|a| a.kind() != "comment" && e.child_by_field_name("arguments").is_some()) {
                return;
            }
            disposed.push((text(obj, src), awaited));
        }
        if disposed.is_empty() {
            return;
        }
        // The `const` declarations just before the `try`.
        let mut run: Vec<Node<'_>> = Vec::new();
        let mut p = t.prev_named_sibling();
        while let Some(s) = p {
            let single_const = s.kind() == "lexical_declaration" && s.child(0).is_some_and(|k| text(k, src) == "const") && {
                let ds: Vec<_> = named_children(s).into_iter().filter(|d| d.kind() == "variable_declarator").collect();
                ds.len() == 1 && ds[0].child_by_field_name("name").is_some_and(|n| n.kind() == "identifier") && ds[0].child_by_field_name("value").is_some()
            };
            if !single_const {
                break;
            }
            run.push(s);
            p = s.prev_named_sibling();
        }
        run.reverse();
        let name_of = |s: Node<'_>| named_children(s).into_iter().find(|d| d.kind() == "variable_declarator").and_then(|d| d.child_by_field_name("name")).map(|n| text(n, src)).unwrap_or("");
        let Some(first) = run.iter().position(|s| disposed.iter().any(|(n, _)| *n == name_of(*s))) else { return };
        let range = &run[first..];
        let names: Vec<&str> = range.iter().map(|s| name_of(*s)).collect();
        // Disposed in reverse order of declaration, each once, nothing else.
        let want: Vec<&str> = names.iter().rev().copied().filter(|n| disposed.iter().any(|(d, _)| d == n)).collect();
        if disposed.iter().map(|(n, _)| *n).collect::<Vec<_>>() != want {
            return;
        }
        // The names must not be read after the `try`: the block would end their scope.
        let mut later = t.next_named_sibling();
        while let Some(s) = later {
            if references(s, &names, src) {
                return;
            }
            later = s.next_named_sibling();
        }
        let line_start = src[..range[0].start_byte()].rfind('\n').map_or(0, |i| i + 1);
        let indent: String = src[line_start..range[0].start_byte()].chars().take_while(|c| c.is_whitespace()).collect();
        let mut text_out = String::from("{\n");
        for s in range {
            let n = name_of(*s);
            let is_disposed = disposed.iter().find(|(d, _)| *d == n);
            let raw = text(*s, src);
            let rest = raw.trim_start().strip_prefix("const").unwrap_or(raw);
            match is_disposed {
                Some((_, true)) => text_out.push_str(&format!("{indent}  await using{rest}\n")),
                Some((_, false)) => text_out.push_str(&format!("{indent}  using{rest}\n")),
                None => text_out.push_str(&format!("{indent}  const{rest}\n")),
            }
        }
        let inner = text(body, src).trim();
        let inner = inner.strip_prefix('{').and_then(|b| b.strip_suffix('}')).unwrap_or(inner).trim_matches(['\n', '\r']);
        text_out.push_str(inner);
        text_out.push_str(&format!("\n{indent}}}"));
        let start = range[0].start_byte();
        let edit = TextEdit::new(start, t.end_byte(), text_out);
        let names_list = disposed.iter().rev().map(|(n, _)| format!("`{n}`")).collect::<Vec<_>>().join(", ");
        out.push(NativeFinding { start: t.start_byte(), end: t.start_byte() + 3, message: format!("This `try`/`finally` only disposes {names_list}: declare with `using` (`await using` for an awaited disposal) instead, if the value has `Symbol.dispose`"), edits: vec![edit] });
    }
}

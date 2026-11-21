//! `js/id-denylist`, `js/id-match` and `js/id-match-unicorn`: the names a
//! project forbids, or requires to match a pattern. ESLint 10.13.0's
//! `id-denylist` and `id-match` and eslint-plugin-unicorn 79.0.0's
//! `id-match`, read and reimplemented on tree-sitter nodes: the identifier
//! kinds below stand for ESTree's `Identifier` and `PrivateIdentifier`.

use super::{at, opt_bool, opt_str, opt_strings, option_problem, regex_of};
use crate::util::text;
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const KINDS: &[&str] = &["identifier", "property_identifier", "shorthand_property_identifier", "shorthand_property_identifier_pattern", "private_property_identifier", "type_identifier"];

fn is_function_decl(kind: &str) -> bool {
    matches!(kind, "function_declaration" | "generator_function_declaration")
}

/// A `type_identifier` is a name only where it declares a class.
fn counts(n: Node<'_>) -> bool {
    if n.kind() != "type_identifier" {
        return true;
    }
    n.parent().is_some_and(|p| matches!(p.kind(), "class_declaration" | "abstract_class_declaration" | "class") && p.child_by_field_name("name").is_some_and(|x| x.id() == n.id()))
}

/// Whether the identifier is a reference to a name no file declares (a
/// language or environment global): the project has no say over those.
fn is_global_reference(n: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    ctx.semantic.and_then(|m| m.reference_at(n.start_byte())).is_some_and(|r| r.symbol.is_none())
}

fn in_jsx_attribute(n: Node<'_>) -> bool {
    n.kind() == "property_identifier" && n.parent().is_some_and(|p| p.kind() == "jsx_attribute")
}

/// `n` is directly the callee or an argument of a call or `new`.
fn call_operand(n: Node<'_>) -> bool {
    let Some(p) = n.parent() else { return false };
    match p.kind() {
        "call_expression" | "new_expression" => true,
        "arguments" => p.parent().is_some_and(|g| matches!(g.kind(), "call_expression" | "new_expression")),
        _ => false,
    }
}

/// A member expression is assigned to: `a.b = 1`, `[a.b] = x`, `({ k: a.b } = x)`.
fn is_assignment_target(member: Node<'_>) -> bool {
    let Some(p) = member.parent() else { return false };
    match p.kind() {
        "assignment_expression" | "augmented_assignment_expression" | "assignment_pattern" => p.child_by_field_name("left").is_some_and(|l| l.id() == member.id()),
        "array_pattern" | "rest_pattern" => true,
        "pair_pattern" => p.child_by_field_name("value").is_some_and(|v| v.id() == member.id()),
        _ => false,
    }
}

fn restricted_message(n: Node<'_>, src: &str) -> String {
    let t = text(n, src);
    if n.kind() == "private_property_identifier" && !t.starts_with('#') { format!("Identifier '#{t}' is restricted.") } else { format!("Identifier '{t}' is restricted.") }
}

pub struct IdDenylist;

impl NativeRule for IdDenylist {
    fn id(&self) -> &'static str {
        "js/id-denylist"
    }
    fn languages(&self) -> &'static [Language] {
        super::JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        KINDS
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn needs_options(&self) -> &'static [&'static str] {
        &["names"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        if !counts(n) || in_jsx_attribute(n) {
            return;
        }
        let name = text(n, ctx.source).trim_start_matches('#');
        let names = opt_strings(ctx, "names");
        if !names.iter().any(|d| d == name) {
            return;
        }
        let Some(p) = n.parent() else { return };
        if n.kind() == "private_property_identifier" {
            out.push(at(n, restricted_message(n, ctx.source)));
            return;
        }
        // `a.b`: the property is checked only when it is assigned to.
        if p.kind() == "member_expression" && p.child_by_field_name("property").is_some_and(|x| x.id() == n.id()) {
            if is_assignment_target(p) {
                out.push(at(n, restricted_message(n, ctx.source)));
            }
            return;
        }
        let renamed_import = match p.kind() {
            "import_specifier" => p.child_by_field_name("alias").is_some() && p.child_by_field_name("name").is_some_and(|x| x.id() == n.id()),
            "export_specifier" => p.child_by_field_name("alias").is_some() && p.child_by_field_name("name").is_some_and(|x| x.id() == n.id()) && p.parent().and_then(|g| g.parent()).is_some_and(|s| s.child_by_field_name("source").is_some()),
            _ => false,
        };
        let destructured_key = p.kind() == "pair_pattern" && p.child_by_field_name("key").is_some_and(|k| k.id() == n.id());
        if call_operand(n) || renamed_import || destructured_key || is_global_reference(n, ctx) {
            return;
        }
        out.push(at(n, restricted_message(n, ctx.source)));
    }
}

/// The options of `id-match` and its unicorn twin.
struct IdMatchOptions<'a> {
    pattern: &'a str,
    properties: bool,
    class_fields: bool,
    only_declarations: bool,
    ignore_destructuring: bool,
}

fn id_match(n: Node<'_>, ctx: &NativeContext<'_>, o: &IdMatchOptions<'_>, unicorn: bool, out: &mut Vec<NativeFinding>) {
    if !counts(n) || in_jsx_attribute(n) {
        return;
    }
    let re = match regex_of(o.pattern) {
        Ok(r) => r,
        Err(e) => return option_problem(n, KINDS, out, format!("`pattern` is not a valid regular expression: {e}")),
    };
    let src = ctx.source;
    let name = text(n, src).trim_start_matches('#');
    let invalid = !re.is_match(name);
    if !invalid {
        return;
    }
    let message = if n.kind() == "private_property_identifier" { format!("Identifier '#{name}' does not match the pattern '{}'.", o.pattern) } else { format!("Identifier '{name}' does not match the pattern '{}'.", o.pattern) };
    let Some(p) = n.parent() else { return };
    let kind = p.kind();
    let is = |field: &str| p.child_by_field_name(field).is_some_and(|x| x.id() == n.id());
    let report = |out: &mut Vec<NativeFinding>| out.push(at(n, message.clone()));

    // A private name is checked wherever it stands; as a class field only with `classFields`.
    if n.kind() == "private_property_identifier" {
        if matches!(kind, "field_definition" | "public_field_definition") && !o.class_fields {
            return;
        }
        return report(out);
    }
    if is_global_reference(n, ctx) {
        return;
    }
    // unicorn's `checkNamedSpecifiers: false` leaves `import { x }` and `export { x } from` alone.
    if unicorn && !opt_bool(ctx, "checkNamedSpecifiers", true) {
        let named = kind == "import_specifier" || (kind == "export_specifier" && p.parent().and_then(|g| g.parent()).is_some_and(|s| s.child_by_field_name("source").is_some()));
        if named {
            return;
        }
    }
    match kind {
        "member_expression" | "subscript_expression" => {
            if !o.properties {
                return;
            }
            let object = is("object");
            let assigned = p.parent().is_some_and(|e| matches!(e.kind(), "assignment_expression" | "augmented_assignment_expression") && e.child_by_field_name("left").is_some_and(|l| l.id() == p.id()));
            let rhs_plain = p.parent().is_some_and(|e| matches!(e.kind(), "assignment_expression" | "augmented_assignment_expression") && e.child_by_field_name("right").is_some_and(|r| r.kind() != "member_expression" && r.kind() != "subscript_expression"));
            // The object's name always; a property's name when it is assigned to.
            if object || assigned || rhs_plain {
                report(out);
            }
        }
        "pair" => {
            // `{ key: value }` in an object literal.
            if !o.properties {
                return;
            }
            if !o.only_declarations {
                report(out);
            }
        }
        "pair_pattern" => {
            // `const { key: value } = x`: the value's name is the new binding; the key is not checked.
            if is("key") {
                return;
            }
            if !(o.ignore_destructuring && p.child_by_field_name("key").is_some_and(|k| text(k, src) == name)) {
                report(out);
            }
        }
        "assignment_pattern" => {
            // A default, `function f(x = 1)` or `[x = 1] = y`: read only with `properties`.
            if !o.properties || !is("left") || o.only_declarations {
                return;
            }
            report(out);
        }
        "import_specifier" => {
            // Only the local name: the alias, or the name when there is none.
            if p.child_by_field_name("alias").is_none() || is("alias") {
                report(out);
            }
        }
        "field_definition" | "public_field_definition" => {
            if o.class_fields && (is("property") || is("name")) {
                report(out);
            }
        }
        "call_expression" | "new_expression" => {}
        "arguments" if call_operand(n) => {}
        _ => {
            let declaring = matches!(kind, "variable_declarator") || is_function_decl(kind) || (kind == "formal_parameters" && p.parent().is_some_and(|g| is_function_decl(g.kind())));
            if !o.only_declarations || declaring {
                report(out);
            }
        }
    }
}

fn shorthand(n: Node<'_>, ctx: &NativeContext<'_>, o: &IdMatchOptions<'_>, unicorn: bool, out: &mut Vec<NativeFinding>) {
    let re = match regex_of(o.pattern) {
        Ok(r) => r,
        Err(e) => return option_problem(n, KINDS, out, format!("`pattern` is not a valid regular expression: {e}")),
    };
    let name = text(n, ctx.source);
    if re.is_match(name) || is_global_reference(n, ctx) {
        return;
    }
    let _ = unicorn;
    let message = format!("Identifier '{name}' does not match the pattern '{}'.", o.pattern);
    match n.kind() {
        // `{ a }` in an object literal: read only with `properties`.
        "shorthand_property_identifier" => {
            if o.properties && !o.only_declarations {
                out.push(at(n, message));
            }
        }
        // `const { a } = x`, `{ a = 1 } = x`: the binding it creates.
        _ => {
            if !o.ignore_destructuring {
                out.push(at(n, message));
            }
        }
    }
}

fn options<'a>(ctx: &'a NativeContext<'_>) -> Option<IdMatchOptions<'a>> {
    Some(IdMatchOptions { pattern: opt_str(ctx, "pattern")?, properties: opt_bool(ctx, "properties", false), class_fields: opt_bool(ctx, "classFields", false), only_declarations: opt_bool(ctx, "onlyDeclarations", false), ignore_destructuring: opt_bool(ctx, "ignoreDestructuring", false) })
}

macro_rules! id_match_rule {
    ($name:ident, $id:literal, $unicorn:literal) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                super::JS
            }
            fn kinds(&self) -> &'static [&'static str] {
                KINDS
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn needs_options(&self) -> &'static [&'static str] {
                &["pattern"]
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                let n = node.get_inner_node();
                let Some(o) = options(ctx) else { return };
                if matches!(n.kind(), "shorthand_property_identifier" | "shorthand_property_identifier_pattern") {
                    shorthand(n, ctx, &o, $unicorn, out);
                } else {
                    id_match(n, ctx, &o, $unicorn, out);
                }
            }
        }
    };
}

id_match_rule!(IdMatch, "js/id-match", false);
id_match_rule!(IdMatchUnicorn, "js/id-match-unicorn", true);

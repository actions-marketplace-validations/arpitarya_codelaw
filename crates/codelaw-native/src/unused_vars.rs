//! `js/no-unused-vars` and `ts/no-unused-vars`: declarations nothing reads,
//! with ESLint's options — `vars`, `args` (`after-used` by default),
//! `caughtErrors`, `ignoreRestSiblings` and the ignore patterns. Parameter
//! order is what a pattern can't see: only the parameters after the last
//! used one are reported.

use crate::util::{named_children, remove_list_item, remove_statement, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{FixClass, Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{ScopeKind, SymbolKind};
use std::collections::BTreeMap;
use tree_sitter::Node;

/// Whether evaluating an initialiser can do nothing observable: a
/// literal, a name, a function, or an array or object of those.
pub(crate) fn pure(n: Node<'_>) -> bool {
    match n.kind() {
        "number" | "string" | "true" | "false" | "null" | "undefined" | "identifier" | "arrow_function" | "function_expression" | "function" | "regex" => true,
        "template_string" => !named_children(n).iter().any(|c| c.kind() == "template_substitution"),
        "parenthesized_expression" | "as_expression" | "satisfies_expression" | "non_null_expression" => n.named_child(0).is_some_and(pure),
        "array" => named_children(n).into_iter().all(pure),
        "object" => named_children(n).into_iter().all(|m| match m.kind() {
            "pair" => m.child_by_field_name("key").is_some_and(|k| k.kind() != "computed_property_name") && m.child_by_field_name("value").is_some_and(pure),
            "shorthand_property_identifier" | "method_definition" => true,
            _ => false,
        }),
        _ => false,
    }
}

/// The edits that remove an unused declaration whose name nothing reads
/// or writes: an import (the statement with its last binding), a
/// variable (its initialiser kept as a statement unless it is pure), a
/// function, an interface or type alias, a catch binding. `None` for a
/// parameter, a destructured name or a class.
fn removal(ident: Node<'_>, src: &str) -> Option<Vec<TextEdit>> {
    let parent = ident.parent()?;
    match parent.kind() {
        "import_specifier" => {
            let list = parent.parent()?;
            if named_children(list).len() > 1 {
                return Some(vec![remove_list_item(parent, src)]);
            }
            let clause = list.parent()?;
            let parts = named_children(clause);
            if parts.len() > 1 {
                let prev = list.prev_named_sibling()?;
                return Some(vec![TextEdit::delete(prev.end_byte(), list.end_byte())]);
            }
            Some(vec![remove_statement(clause.parent()?, src)])
        }
        "import_clause" | "namespace_import" => {
            let clause = if parent.kind() == "namespace_import" { parent.parent()? } else { parent };
            let item = if parent.kind() == "namespace_import" { parent } else { ident };
            if named_children(clause).len() > 1 {
                return Some(vec![remove_list_item(item, src)]);
            }
            Some(vec![remove_statement(clause.parent()?, src)])
        }
        "variable_declarator" if parent.child_by_field_name("name").is_some_and(|n| n.id() == ident.id()) => {
            let decl = parent.parent()?;
            if !matches!(decl.kind(), "lexical_declaration" | "variable_declaration") || decl.parent().is_some_and(|p| p.kind() == "export_statement" || p.kind().starts_with("for")) {
                return None;
            }
            let value = parent.child_by_field_name("value");
            if named_children(decl).len() > 1 {
                return value.is_none_or(pure).then(|| vec![remove_list_item(parent, src)]);
            }
            match value {
                Some(v) if !pure(v) => Some(vec![TextEdit::new(decl.start_byte(), decl.end_byte(), format!("{};", text(v, src)))]),
                _ => Some(vec![remove_statement(decl, src)]),
            }
        }
        "function_declaration" | "generator_function_declaration" | "interface_declaration" | "type_alias_declaration"
            if parent.child_by_field_name("name").is_some_and(|n| n.id() == ident.id()) =>
        {
            if parent.parent().is_some_and(|p| p.kind() == "export_statement") {
                return None;
            }
            Some(vec![remove_statement(parent, src)])
        }
        "catch_clause" => {
            // `catch (err) {` -> `catch {` (an optional catch binding).
            let kw = parent.child(0).filter(|c| c.kind() == "catch")?;
            let body = parent.child_by_field_name("body")?;
            Some(vec![TextEdit::new(kw.end_byte(), body.start_byte(), " ")])
        }
        _ => None,
    }
}

pub struct NoUnusedVars {
    pub id: &'static str,
    pub languages: &'static [Language],
}

fn opt_str<'a>(ctx: &'a NativeContext<'_>, key: &str, default: &'a str) -> &'a str {
    ctx.options.get(key).and_then(|v| v.as_str()).unwrap_or(default)
}

fn opt_regex(ctx: &NativeContext<'_>, key: &str) -> Option<regex::Regex> {
    ctx.options.get(key).and_then(|v| v.as_str()).and_then(|p| regex::Regex::new(p).ok())
}

/// Parameters of a signature without a body are not runtime bindings.
const SIGNATURES: &[&str] =
    &["function_type", "constructor_type", "method_signature", "call_signature", "construct_signature", "function_signature", "abstract_method_signature"];

impl NativeRule for NoUnusedVars {
    /// Removing a whole import statement drops the module's side effects
    /// (it no longer loads), so that finding is a suggestion.
    fn rung_of(&self, finding: &NativeFinding, ctx: &NativeContext<'_>) -> Option<FixClass> {
        finding.edits.iter().any(|e| e.text.is_empty() && ctx.source.get(e.start..e.end).is_some_and(|t| t.trim_start().starts_with("import"))).then_some(FixClass::Suggest)
    }
    fn id(&self) -> &'static str {
        self.id
    }
    fn languages(&self) -> &'static [Language] {
        self.languages
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let root = node.get_inner_node();
        let vars = opt_str(ctx, "vars", "all");
        let args = opt_str(ctx, "args", "after-used");
        let caught = opt_str(ctx, "caughtErrors", "all");
        let rest_siblings = ctx.options.get("ignoreRestSiblings").and_then(|v| v.as_bool()).unwrap_or(false);
        let vars_ignore = opt_regex(ctx, "varsIgnorePattern");
        let args_ignore = opt_regex(ctx, "argsIgnorePattern");
        let caught_ignore = opt_regex(ctx, "caughtErrorsIgnorePattern");
        let array_ignore = opt_regex(ctx, "destructuredArrayIgnorePattern");
        let ignored = |re: &Option<regex::Regex>, name: &str| re.as_ref().is_some_and(|r| r.is_match(name));

        let mut params: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
        for (id, s) in model.symbols.iter().enumerate() {
            let Some(&(start, end)) = s.decls.first() else { continue };
            let Some(ident) = root.descendant_for_byte_range(start, end) else { continue };
            if ident.start_byte() != start {
                continue;
            }
            let parent = ident.parent();
            let parent_kind = parent.map(|p| p.kind()).unwrap_or_default();
            let ambient = {
                let mut a = ident.parent();
                let mut found = false;
                while let Some(n) = a {
                    if n.kind() == "ambient_declaration" {
                        found = true;
                        break;
                    }
                    a = n.parent();
                }
                found
            };
            if ambient {
                continue;
            }
            match s.kind {
                SymbolKind::Param => {
                    let scope = &model.scopes[s.scope];
                    let owner = root.descendant_for_byte_range(scope.start, scope.end).map(|n| n.kind()).unwrap_or_default();
                    if SIGNATURES.contains(&owner) {
                        continue;
                    }
                    params.entry(s.scope).or_default().push((start, id));
                    continue;
                }
                SymbolKind::TypeParam => continue,
                // A function or class expression's own name.
                SymbolKind::Function | SymbolKind::Class if matches!(parent_kind, "function_expression" | "function" | "generator_function" | "class") => continue,
                _ => {}
            }
            if !model.is_unused(id) {
                continue;
            }
            if s.kind == SymbolKind::CatchParam {
                if caught == "none" || ignored(&caught_ignore, &s.name) {
                    continue;
                }
            } else {
                if vars == "local" && model.scopes[s.scope].kind == ScopeKind::Module {
                    continue;
                }
                if ignored(&vars_ignore, &s.name) {
                    continue;
                }
                if parent_kind == "array_pattern" && ignored(&array_ignore, &s.name) {
                    continue;
                }
                if rest_siblings
                    && let Some(p) = parent
                    && p.kind() == "object_pattern"
                    && named_children(p).iter().any(|c| c.kind() == "rest_pattern")
                {
                    continue;
                }
            }
            let assigned = s.refs.iter().any(|&r| model.references[r].write)
                || parent.is_some_and(|p| p.kind() == "variable_declarator" && p.child_by_field_name("value").is_some());
            let what = if assigned { "is assigned a value but never used" } else { "is defined but never used" };
            let edits = if s.refs.is_empty() { removal(ident, ctx.source).unwrap_or_default() } else { Vec::new() };
            out.push(NativeFinding { start, end, message: format!("`{}` {what}", s.name), edits });
        }
        if args == "none" {
            return;
        }
        for (_, mut list) in params {
            list.sort();
            let last_used = list.iter().rposition(|(_, id)| !model.is_unused(*id));
            for (i, (start, id)) in list.iter().enumerate() {
                if !model.is_unused(*id) {
                    continue;
                }
                if args == "after-used" && last_used.is_some_and(|u| i < u) {
                    continue;
                }
                let s = model.symbol(*id);
                if ignored(&args_ignore, &s.name) {
                    continue;
                }
                out.push(NativeFinding { start: *start, end: s.decls[0].1, message: format!("`{}` is defined but never used", s.name), edits: Vec::new() });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(language: Language, src: &str) -> Vec<String> {
        let rule = NoUnusedVars { id: "js/no-unused-vars", languages: &[Language::Javascript, Language::Typescript] };
        crate::run(&rule, language, src).into_iter().map(|f| f.message).collect()
    }

    #[test]
    fn vars_args_and_catch() {
        let src = "import { a, b } from 'm';\nconst c = 1, d = a;\nfunction f(x, y, z) { return y; }\nf();\nlet e; e = 2;\ntry {} catch (err) {}\nexport { d };\n";
        assert_eq!(found(Language::Javascript, src), [
            "`b` is defined but never used",
            "`c` is assigned a value but never used",
            "`e` is assigned a value but never used",
            "`err` is defined but never used",
            "`z` is defined but never used",
        ]);
    }

    #[test]
    fn expressions_and_signatures_are_left_alone() {
        let src = "export const g = function named() {};\nexport type F = (unusedParam: number) => void;\ninterface Unused {}\nexport class K { constructor(private s: string) {} m(a: number): void; m(a: any) { return a; } }\ndeclare const amb: number;\n";
        assert_eq!(found(Language::Typescript, src), ["`Unused` is defined but never used"]);
    }
}

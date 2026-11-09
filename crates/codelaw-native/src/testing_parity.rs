//! The `testing` pack's native rules (W-139, the tail of slice 14): the
//! rows a `match:` rule cannot express, each reimplemented from the plugin's
//! documented behaviour at its pinned version.
//!
//! - `testing/prefer-importing-jest-globals` (eslint-plugin-jest 29.17.0):
//!   every Jest global a file uses without an import, collected into one
//!   `@jest/globals` import.
//! - `testing/prefer-importing-vitest-globals` (@vitest/eslint-plugin 1.6.28):
//!   the same for Vitest's globals that a file *calls*.
//! - `testing/valid-mock-module-path` (eslint-plugin-jest): `jest.mock('./x')`
//!   names a file that exists. It reads the project's files (SR-ARCHITECTURE
//!   decision 15b).
//! - `testing/playwright-no-unsafe-references` and
//!   `testing/playwright-prefer-native-locators` (eslint-plugin-playwright
//!   2.12.1).

use crate::util::{named_children, text, unquote};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::SemanticModel;
use std::collections::BTreeSet;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

pub const RULES: &[&dyn NativeRule] = &[&PreferImportingJestGlobals, &PreferImportingVitestGlobals, &ValidMockModulePath, &PlaywrightNoUnsafeReferences, &PlaywrightPreferNativeLocators];

fn finding(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

/// Every named node under `root`, in source order.
fn all_nodes(root: Node<'_>) -> Vec<Node<'_>> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        out.push(n);
        let mut kids = named_children(n);
        kids.reverse();
        stack.extend(kids);
    }
    out
}

/// A top-level statement of the program, comments and a hashbang left out.
fn statements(program: Node<'_>) -> Vec<Node<'_>> {
    named_children(program).into_iter().filter(|c| !matches!(c.kind(), "comment" | "hash_bang_line")).collect()
}

/// A plain string literal's value (no template, no substitution).
fn plain_string<'s>(n: Node<'_>, src: &'s str) -> Option<&'s str> {
    match n.kind() {
        "string" => Some(unquote(text(n, src))),
        "template_string" if named_children(n).iter().all(|c| c.kind() != "template_substitution") => Some(unquote(text(n, src))),
        _ => None,
    }
}

/// Whether the identifier at `n` names nothing declared in the file: a
/// global, as a test runner provides it.
fn is_global(model: &SemanticModel, n: Node<'_>) -> bool {
    model.reference_at(n.start_byte()).is_some_and(|r| r.symbol.is_none())
}

/// Whether the identifier at `n` is an import from `module`.
fn imported_from(model: &SemanticModel, n: Node<'_>, module: &str) -> bool {
    model.reference_at(n.start_byte()).and_then(|r| r.symbol).is_some_and(|s| model.imported_from(s, module, None))
}

/// The module specifier of a top-level `import … from '<source>'`.
fn import_source<'s>(stmt: Node<'_>, src: &'s str) -> Option<&'s str> {
    (stmt.kind() == "import_statement").then(|| stmt.child_by_field_name("source")).flatten().map(|s| unquote(text(s, src)))
}

/// `import type …`, or a specifier with `type`: a form this rule does not merge into.
fn is_type_import(stmt: Node<'_>, src: &str) -> bool {
    let t = text(stmt, src);
    t.starts_with("import type") || named_children(stmt).iter().any(|c| c.kind() == "import_clause" && text(*c, src).contains("type "))
}

/// The insertion that starts a new line of imports: before the first
/// statement, or after a leading `'use strict'` directive.
fn new_line_edit(program: Node<'_>, src: &str, line: &str) -> Option<TextEdit> {
    let stmts = statements(program);
    let first = *stmts.first()?;
    if first.kind() == "expression_statement" && first.named_child(0).is_some_and(|s| s.kind() == "string" && unquote(text(s, src)) == "use strict") {
        return Some(TextEdit::insert(first.end_byte(), format!("\n{line}")));
    }
    Some(TextEdit::insert(first.start_byte(), format!("{line}\n")))
}

// ------------------------------------------------------------ Jest calls

const DESCRIBE_NAMES: &[&str] = &["describe", "fdescribe", "xdescribe"];
const TEST_NAMES: &[&str] = &["it", "test", "fit", "xit", "xtest"];
const HOOK_NAMES: &[&str] = &["beforeAll", "beforeEach", "afterAll", "afterEach"];

/// The call chains Jest's runner defines (`describe.only.each`, `it.concurrent.failing`, …).
const VALID_CHAINS: &[&str] = &[
    "afterAll", "afterEach", "beforeAll", "beforeEach", "describe", "describe.each", "describe.only", "describe.only.each", "describe.skip", "describe.skip.each", "fdescribe", "fdescribe.each", "xdescribe", "xdescribe.each", "it", "it.concurrent", "it.concurrent.failing", "it.concurrent.each", "it.concurrent.failing.each",
    "it.concurrent.failing.only.each", "it.concurrent.failing.skip.each", "it.concurrent.only.each", "it.concurrent.skip.each", "it.each", "it.failing", "it.failing.each", "it.only", "it.only.each", "it.only.failing", "it.only.failing.each", "it.skip", "it.skip.each", "it.skip.failing", "it.skip.failing.each", "it.todo",
    "fit", "fit.each", "fit.failing", "fit.failing.each", "xit", "xit.each", "xit.failing", "xit.failing.each", "test", "test.concurrent", "test.concurrent.failing", "test.concurrent.each", "test.concurrent.failing.each", "test.concurrent.failing.only.each", "test.concurrent.failing.skip.each",
    "test.concurrent.only.each", "test.concurrent.skip.each", "test.each", "test.failing", "test.failing.each", "test.only", "test.only.each", "test.only.failing", "test.only.failing.each", "test.skip", "test.skip.each", "test.skip.failing", "test.skip.failing.each", "test.todo", "xtest", "xtest.each",
    "xtest.failing", "xtest.failing.each",
];

/// A tagged template: a call whose argument is a template string.
fn is_tagged(call: Node<'_>) -> bool {
    call.kind() == "call_expression" && call.child_by_field_name("arguments").is_some_and(|a| a.kind() == "template_string")
}

/// The accessors of a callee, head first: `a.b['c']()` is `a`, `b`, `'c'`.
fn chain<'t>(n: Node<'t>, src: &str) -> Option<Vec<Node<'t>>> {
    match n.kind() {
        "identifier" | "property_identifier" => Some(vec![n]),
        "string" => Some(vec![n]),
        "member_expression" => {
            let mut head = chain(n.child_by_field_name("object")?, src)?;
            head.push(n.child_by_field_name("property")?);
            Some(head)
        }
        "subscript_expression" => {
            let mut head = chain(n.child_by_field_name("object")?, src)?;
            let index = n.child_by_field_name("index")?;
            plain_string(index, src)?;
            head.push(index);
            Some(head)
        }
        "call_expression" => chain(n.child_by_field_name("function")?, src),
        _ => None,
    }
}

fn accessor<'s>(n: Node<'_>, src: &'s str) -> &'s str {
    if n.kind() == "string" { unquote(text(n, src)) } else { text(n, src) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum JestKind {
    Hook,
    Describe,
    Test,
    Expect,
    Jest,
}

impl JestKind {
    fn name(self) -> &'static str {
        match self {
            JestKind::Hook => "hook",
            JestKind::Describe => "describe",
            JestKind::Test => "test",
            JestKind::Expect => "expect",
            JestKind::Jest => "jest",
        }
    }
}

/// What a call is, when it is a call of a Jest global: the global's name,
/// its kind and the identifier that names it.
fn jest_call<'t>(call: Node<'t>, src: &str) -> Option<(String, JestKind, Node<'t>)> {
    if call.kind() != "call_expression" || is_tagged(call) {
        return None;
    }
    let callee = call.child_by_field_name("function")?;
    let links = chain(callee, src)?;
    let head = *links.first()?;
    if head.kind() != "identifier" {
        return None;
    }
    let last = accessor(*links.last()?, src);
    if last == "each" && callee.kind() != "call_expression" {
        return None;
    }
    let name = text(head, src);
    let kind = match name {
        "expect" => JestKind::Expect,
        "jest" => JestKind::Jest,
        n if DESCRIBE_NAMES.contains(&n) => JestKind::Describe,
        n if TEST_NAMES.contains(&n) => JestKind::Test,
        n if HOOK_NAMES.contains(&n) => JestKind::Hook,
        _ => return None,
    };
    let joined: Vec<&str> = links.iter().map(|l| accessor(*l, src)).collect();
    if !matches!(kind, JestKind::Expect | JestKind::Jest) && !VALID_CHAINS.contains(&joined.join(".").as_str()) {
        return None;
    }
    if kind == JestKind::Expect {
        // Only a complete `expect(…)[.modifier…].matcher(…)` chain is a Jest call.
        let mut modifiers: Vec<&str> = Vec::new();
        let mut matcher = false;
        for m in &links[1..] {
            let access = m.parent()?;
            let called = access.parent().is_some_and(|p| p.kind() == "call_expression" && p.child_by_field_name("function").is_some_and(|f| f.id() == access.id()));
            if called {
                matcher = true;
                break;
            }
            let word = accessor(*m, src);
            let ok = match modifiers.len() {
                0 => matches!(word, "not" | "resolves" | "rejects"),
                1 => word == "not" && matches!(modifiers[0], "resolves" | "rejects"),
                _ => false,
            };
            if !ok {
                return None;
            }
            modifiers.push(word);
        }
        if !matcher {
            return None;
        }
    } else {
        // Every link but the last is a member access, and the call is the top of its chain.
        if links[..links.len() - 1].iter().any(|l| !l.parent().is_some_and(|p| matches!(p.kind(), "member_expression" | "subscript_expression"))) {
            return None;
        }
        if call.parent().is_some_and(|p| matches!(p.kind(), "call_expression" | "member_expression")) {
            return None;
        }
    }
    Some((name.to_owned(), kind, head))
}

fn is_module(program: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    let stmts = statements(program);
    if stmts.iter().any(|s| matches!(s.kind(), "import_statement" | "export_statement")) {
        return true;
    }
    let path = ctx.file.path();
    if path.ends_with(".cjs") || path.ends_with(".cts") {
        return false;
    }
    let src = ctx.source;
    let commonjs = all_nodes(program).into_iter().any(|n| {
        n.kind() == "call_expression" && n.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "require") || n.kind() == "member_expression" && text(n, src) == "module.exports"
    });
    !commonjs
}

// ---------------------------------------------------- prefer-importing-jest-globals

pub struct PreferImportingJestGlobals;

impl NativeRule for PreferImportingJestGlobals {
    fn id(&self) -> &'static str {
        "testing/prefer-importing-jest-globals"
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
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let program = node.get_inner_node();
        let src = ctx.source;
        let types: Vec<String> = match ctx.options.get("types").and_then(|v| v.as_array()) {
            Some(a) => a.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect(),
            None => ["hook", "describe", "test", "expect", "jest"].iter().map(|s| s.to_string()).collect(),
        };
        let mut names: Vec<String> = Vec::new();
        let mut reporting: Option<Node<'_>> = None;
        for n in all_nodes(program) {
            let Some((name, kind, head)) = jest_call(n, src) else { continue };
            if !is_global(model, head) || !types.iter().any(|t| t == kind.name()) {
                continue;
            }
            if !names.contains(&name) {
                names.push(name);
            }
            reporting.get_or_insert(head);
        }
        let Some(head) = reporting else { return };
        let module = is_module(program, ctx);
        let edits = fix_edits(program, src, module, &names);
        out.push(finding(head, format!("Import the following Jest functions from '@jest/globals': {}", names.join(", ")), edits));
    }
}

fn globals_line(module: bool, names: &BTreeSet<String>) -> String {
    let list = names.iter().cloned().collect::<Vec<_>>().join(", ");
    if module { format!("import {{ {list} }} from '@jest/globals';") } else { format!("const {{ {list} }} = require('@jest/globals');") }
}

fn fix_edits(program: Node<'_>, src: &str, module: bool, used: &[String]) -> Vec<TextEdit> {
    let mut names: BTreeSet<String> = used.iter().cloned().collect();
    let stmts = statements(program);
    // An existing `import { … } from '@jest/globals'`: merge into it.
    if let Some(imp) = stmts.iter().find(|s| import_source(**s, src) == Some("@jest/globals") && !is_type_import(**s, src)) {
        let clause = named_children(*imp).into_iter().find(|c| c.kind() == "import_clause");
        let mut mergeable = clause.is_some();
        if let Some(clause) = clause {
            for part in named_children(clause) {
                match part.kind() {
                    "identifier" => {
                        names.insert(text(part, src).to_owned());
                    }
                    "named_imports" => {
                        for spec in named_children(part).into_iter().filter(|s| s.kind() == "import_specifier") {
                            let Some(name) = spec.child_by_field_name("name") else { continue };
                            let local = spec.child_by_field_name("alias");
                            names.insert(match local {
                                Some(a) if text(a, src) != text(name, src) => format!("{} as {}", text(name, src), text(a, src)),
                                _ => text(name, src).to_owned(),
                            });
                        }
                    }
                    _ => mergeable = false,
                }
            }
        }
        if mergeable {
            return vec![TextEdit::new(imp.start_byte(), imp.end_byte(), globals_line(true, &names))];
        }
    }
    // An existing `const { … } = require('@jest/globals')`: merge into it.
    for s in &stmts {
        if !matches!(s.kind(), "lexical_declaration" | "variable_declaration") {
            continue;
        }
        let decls: Vec<Node<'_>> = named_children(*s).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
        let [decl] = decls.as_slice() else { continue };
        let (Some(id), Some(value)) = (decl.child_by_field_name("name"), decl.child_by_field_name("value")) else { continue };
        if id.kind() != "object_pattern" || value.kind() != "call_expression" {
            continue;
        }
        let is_require = value.child_by_field_name("function").is_some_and(|f| text(f, src) == "require");
        let arg = value.child_by_field_name("arguments").and_then(|a| named_children(a).into_iter().next());
        if !(is_require && arg.is_some_and(|a| plain_string(a, src) == Some("@jest/globals"))) {
            continue;
        }
        for p in named_children(id) {
            match p.kind() {
                "shorthand_property_identifier_pattern" => {
                    names.insert(text(p, src).to_owned());
                }
                "pair_pattern" => {
                    if let (Some(k), Some(v)) = (p.child_by_field_name("key"), p.child_by_field_name("value")) {
                        let (k, v) = (text(k, src), text(v, src));
                        names.insert(if k == v { k.to_owned() } else { format!("{k}: {v}") });
                    }
                }
                _ => {}
            }
        }
        return vec![TextEdit::new(s.start_byte(), s.end_byte(), globals_line(false, &names))];
    }
    new_line_edit(program, src, &globals_line(module, &names)).into_iter().collect()
}

// -------------------------------------------------- prefer-importing-vitest-globals

const VITEST_GLOBALS: &[&str] = &["suite", "test", "chai", "describe", "it", "expectTypeOf", "assertType", "expect", "assert", "vitest", "vi", "beforeAll", "afterAll", "beforeEach", "afterEach", "onTestFailed", "onTestFinished"];

pub struct PreferImportingVitestGlobals;

impl NativeRule for PreferImportingVitestGlobals {
    fn id(&self) -> &'static str {
        "testing/prefer-importing-vitest-globals"
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
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let program = node.get_inner_node();
        let src = ctx.source;
        let mut hits: Vec<Node<'_>> = Vec::new();
        let mut names: Vec<&str> = Vec::new();
        for n in all_nodes(program) {
            if n.kind() != "call_expression" {
                continue;
            }
            let Some(f) = n.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { continue };
            let name = text(f, src);
            if !VITEST_GLOBALS.contains(&name) || !is_global(model, f) {
                continue;
            }
            if !names.contains(&name) {
                names.push(name);
            }
            hits.push(f);
        }
        if hits.is_empty() {
            return;
        }
        let edits = vitest_edits(program, src, &names);
        for f in hits {
            out.push(finding(f, format!("Import '{}' from 'vitest'", text(f, src)), edits.clone()));
        }
    }
}

fn vitest_edits(program: Node<'_>, src: &str, names: &[&str]) -> Vec<TextEdit> {
    let list = names.join(", ");
    let stmts = statements(program);
    if let Some(imp) = stmts.iter().find(|s| import_source(**s, src) == Some("vitest") && !is_type_import(**s, src)) {
        let clause = named_children(*imp).into_iter().find(|c| c.kind() == "import_clause");
        if let Some(clause) = clause {
            let parts = named_children(clause);
            if !parts.iter().any(|p| p.kind() == "namespace_import") {
                if let Some(named) = parts.iter().find(|p| p.kind() == "named_imports")
                    && let Some(last) = named_children(*named).into_iter().rfind(|s| s.kind() == "import_specifier")
                {
                    return vec![TextEdit::insert(last.end_byte(), format!(", {list}"))];
                }
                if let Some(default) = parts.iter().find(|p| p.kind() == "identifier")
                    && !parts.iter().any(|p| p.kind() == "named_imports")
                {
                    return vec![TextEdit::insert(default.end_byte(), format!(", {{ {list} }}"))];
                }
            }
        }
    }
    for s in &stmts {
        if !matches!(s.kind(), "lexical_declaration" | "variable_declaration") {
            continue;
        }
        let decls: Vec<Node<'_>> = named_children(*s).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
        let [decl] = decls.as_slice() else { continue };
        let (Some(id), Some(value)) = (decl.child_by_field_name("name"), decl.child_by_field_name("value")) else { continue };
        if id.kind() != "object_pattern" || value.kind() != "call_expression" {
            continue;
        }
        let is_require = value.child_by_field_name("function").is_some_and(|f| text(f, src) == "require");
        let arg = value.child_by_field_name("arguments").and_then(|a| named_children(a).into_iter().next());
        if is_require && arg.is_some_and(|a| plain_string(a, src) == Some("vitest")) {
            let props = named_children(id);
            if let Some(last) = props.last().filter(|p| p.kind() != "rest_pattern") {
                return vec![TextEdit::insert(last.end_byte(), format!(", {list}"))];
            }
        }
    }
    new_line_edit(program, src, &format!("import {{ {list} }} from 'vitest';")).into_iter().collect()
}

// ------------------------------------------------------------ valid-mock-module-path

pub struct ValidMockModulePath;

impl NativeRule for ValidMockModulePath {
    fn id(&self) -> &'static str {
        "testing/valid-mock-module-path"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let call = node.get_inner_node();
        let src = ctx.source;
        let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression") else { return };
        let (Some(object), Some(property)) = (f.child_by_field_name("object"), f.child_by_field_name("property")) else { return };
        if object.kind() != "identifier" || text(object, src) != "jest" || !matches!(text(property, src), "mock" | "doMock") {
            return;
        }
        if !is_global(model, object) && !imported_from(model, object, "@jest/globals") {
            return;
        }
        let args = call.child_by_field_name("arguments").map(named_children).unwrap_or_default();
        let Some(first) = args.first().filter(|a| a.kind() == "string") else { return };
        let module = unquote(text(*first, src));
        if !module.starts_with('.') {
            return; // a package: resolving it takes node's lookup, which the project's files do not give
        }
        // `{ virtual: true }` as the third argument: the module is made up.
        if let Some(opts) = args.get(2).filter(|o| o.kind() == "object") {
            let virtual_true = named_children(*opts).into_iter().any(|p| {
                p.kind() == "pair" && p.child_by_field_name("key").is_some_and(|k| text(k, src).trim_matches(['"', '\'']) == "virtual") && p.child_by_field_name("value").is_some_and(|v| !matches!(text(v, src), "false" | "0" | "null" | "undefined" | "''" | "\"\""))
            });
            if virtual_true {
                return;
            }
        }
        let extensions: Vec<String> = match ctx.options.get("moduleFileExtensions").and_then(|v| v.as_array()) {
            Some(a) => a.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect(),
            None => [".js", ".ts", ".tsx", ".jsx", ".json"].iter().map(|s| s.to_string()).collect(),
        };
        let Some(base) = crate::resolve_path(ctx.file.path(), module) else { return };
        let exists = std::iter::once("").chain(extensions.iter().map(String::as_str)).any(|ext| ctx.file.exists(&format!("{base}{ext}")));
        if !exists {
            out.push(finding(call, format!("Module path {} does not exist or is not exported", text(*first, src)), vec![]));
        }
    }
}

// ------------------------------------------------------- playwright-no-unsafe-references

fn page_name_full(s: &str) -> bool {
    s.starts_with("page") || s.starts_with("frame") || s.ends_with("Page") || s.ends_with("Frame")
}

fn page_name_prefix(s: &str) -> bool {
    s.starts_with("page") || s.starts_with("frame")
}

/// The name a chain ends in, looking through calls and member accesses.
fn dig(n: Node<'_>, src: &str) -> bool {
    match n.kind() {
        "member_expression" => n.child_by_field_name("property").is_some_and(|p| dig(p, src)),
        "call_expression" => n.child_by_field_name("function").is_some_and(|f| dig(f, src)),
        "identifier" | "property_identifier" => page_name_full(text(n, src)),
        "non_null_expression" | "parenthesized_expression" => n.named_child(0).is_some_and(|c| dig(c, src)),
        _ => false,
    }
}

/// `<page-or-frame>.<method>(…)`: the receiver is named like a Playwright page or frame.
fn is_page_method(call: Node<'_>, method: &str, src: &str) -> bool {
    let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression") else { return false };
    if f.child_by_field_name("property").is_none_or(|p| text(p, src) != method) {
        return false;
    }
    let Some(obj) = f.child_by_field_name("object") else { return false };
    if obj.kind() == "member_expression" {
        let (Some(inner), Some(prop)) = (obj.child_by_field_name("object"), obj.child_by_field_name("property")) else { return false };
        let name = text(prop, src);
        return if inner.kind() == "this" { page_name_full(name) } else { page_name_prefix(name) };
    }
    dig(obj, src)
}

pub struct PlaywrightNoUnsafeReferences;

impl NativeRule for PlaywrightNoUnsafeReferences {
    fn id(&self) -> &'static str {
        "testing/playwright-no-unsafe-references"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let call = node.get_inner_node();
        let src = ctx.source;
        let method = ["evaluate", "addInitScript"].into_iter().find(|m| is_page_method(call, m, src));
        let Some(method) = method else { return };
        let args: Vec<Node<'_>> = call.child_by_field_name("arguments").map(named_children).unwrap_or_default().into_iter().filter(|a| a.kind() != "comment").collect();
        let Some(&func) = args.first().filter(|a| matches!(a.kind(), "arrow_function" | "function_expression" | "function")) else { return };
        let (fs, fe) = (func.start_byte(), func.end_byte());
        // References inside the callback to a binding declared outside it.
        let mut refs: Vec<(usize, usize, String)> = Vec::new();
        for r in &model.references {
            if r.start < fs || r.end > fe || r.type_only {
                continue;
            }
            let Some(sym) = r.symbol else { continue };
            let scope = &model.scopes[model.symbol(sym).scope];
            if scope.start >= fs && scope.end <= fe {
                continue;
            }
            refs.push((r.start, r.end, r.name.clone()));
        }
        if refs.is_empty() {
            return;
        }
        refs.sort();
        let mut names: Vec<String> = Vec::new();
        for (_, _, n) in &refs {
            if !names.contains(n) {
                names.push(n.clone());
            }
        }
        let edits = pass_edits(&args, func, &names, src);
        for (start, end, name) in refs {
            out.push(NativeFinding { start, end, message: format!("Unsafe reference to variable \"{name}\" in page.{method}()"), edits: edits.clone() });
        }
    }
}

/// The edits that pass `names` to the callback as its argument, for the two
/// shapes whose result is certain: a callback with no parameters that is
/// the only argument, and one that already destructures an array argument.
fn pass_edits(args: &[Node<'_>], func: Node<'_>, names: &[String], src: &str) -> Vec<TextEdit> {
    let list = names.join(", ");
    let params = func.child_by_field_name("parameters");
    let single = func.child_by_field_name("parameter");
    if args.len() == 1 && single.is_none() && params.is_some_and(|p| named_children(p).is_empty()) {
        let p = params.expect("checked");
        return vec![TextEdit::new(p.start_byte(), p.end_byte(), format!("([{list}])")), TextEdit::insert(args[0].end_byte(), format!(", [{list}]"))];
    }
    if args.len() == 2
        && args[1].kind() == "array"
        && let Some(p) = params
        && let [only] = named_children(p).as_slice()
    {
        let pattern = if matches!(only.kind(), "required_parameter" | "optional_parameter") { only.child_by_field_name("pattern") } else { Some(*only) };
        if let Some(pattern) = pattern.filter(|x| x.kind() == "array_pattern") {
            let (pe, ae) = (named_children(pattern), named_children(args[1]));
            if let (Some(last_p), Some(last_a)) = (pe.last(), ae.last())
                && pe.len() == ae.len()
                && !names.iter().any(|n| pe.iter().any(|e| text(*e, src) == n))
                && last_p.kind() != "rest_pattern"
                && last_a.kind() != "spread_element"
            {
                return vec![TextEdit::insert(last_p.end_byte(), format!(", {list}")), TextEdit::insert(last_a.end_byte(), format!(", {list}"))];
            }
        }
    }
    Vec::new()
}

// ------------------------------------------------------ playwright-prefer-native-locators

pub struct PlaywrightPreferNativeLocators;

/// What `[attr=value]` selects by: the native locator, from the attribute.
fn native_locator(attr: &str, test_id: &str) -> Option<&'static str> {
    Some(match attr {
        "aria-label" => "getByLabel",
        "role" => "getByRole",
        "placeholder" => "getByPlaceholder",
        "alt" => "getByAltText",
        "title" => "getByTitle",
        a if a == test_id => "getByTestId",
        _ => return None,
    })
}

/// `[attr=value]`, `[attr="value"]` or `[attr='value']` and nothing else:
/// the attribute and the bare value.
fn single_attribute(selector: &str) -> Option<(&str, &str)> {
    let inner = selector.strip_prefix('[')?.strip_suffix(']')?;
    let (attr, raw) = inner.split_once('=')?;
    let value = match raw.chars().next()? {
        q @ ('"' | '\'') => raw.strip_prefix(q)?.strip_suffix(q)?,
        _ => raw,
    };
    let clean = !value.is_empty() && !value.chars().any(|c| matches!(c, ']' | '[' | '"' | '\'' | '\\' | '\n' | '='));
    (clean && !attr.is_empty() && attr.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')).then_some((attr, value))
}

impl NativeRule for PlaywrightPreferNativeLocators {
    fn id(&self) -> &'static str {
        "testing/playwright-prefer-native-locators"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let call = node.get_inner_node();
        let src = ctx.source;
        let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression") else { return };
        let Some(prop) = f.child_by_field_name("property").filter(|p| p.kind() == "property_identifier" && text(*p, src) == "locator") else { return };
        let args: Vec<Node<'_>> = call.child_by_field_name("arguments").map(named_children).unwrap_or_default().into_iter().filter(|a| a.kind() != "comment").collect();
        let Some(selector) = args.first().and_then(|a| plain_string(*a, src)) else { return };
        let test_id = ctx.options.get("testIdAttribute").and_then(|v| v.as_str()).unwrap_or("data-testid");
        let Some((attr, value)) = single_attribute(selector) else { return };
        let Some(native) = native_locator(attr, test_id) else { return };
        let edits = if args.len() == 1 { vec![TextEdit::new(prop.start_byte(), call.end_byte(), format!("{native}(\"{value}\")"))] } else { Vec::new() };
        out.push(finding(call, format!("Use {native}() instead"), edits));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;
    use codelaw_core::Language::{Javascript, Typescript};

    fn apply(src: &str, f: &NativeFinding) -> String {
        let mut s = src.to_owned();
        let mut edits = f.edits.clone();
        edits.sort_by_key(|e| std::cmp::Reverse(e.start));
        for e in edits {
            s.replace_range(e.start..e.end, &e.text);
        }
        s
    }

    #[test]
    fn jest_globals_collect_into_one_import() {
        let src = "describe('a', () => {\n  it('b', () => {\n    expect(1).toBe(1);\n    jest.fn();\n  });\n});\n";
        let out = run(&PreferImportingJestGlobals, Javascript, src);
        assert_eq!(out.len(), 1);
        assert_eq!(apply(src, &out[0]), format!("import {{ describe, expect, it, jest }} from '@jest/globals';\n{src}"));
    }

    #[test]
    fn jest_imported_or_local_is_not_global() {
        let src = "import { describe, it, expect } from '@jest/globals';\ndescribe('a', () => { it('b', () => { expect(1).toBe(1); }); });\n";
        assert!(run(&PreferImportingJestGlobals, Javascript, src).is_empty());
        let local = "function it() {}\nit('x', () => {});\n";
        assert!(run(&PreferImportingJestGlobals, Javascript, local).is_empty());
        // `expect(x)` with no matcher is not a Jest call.
        assert!(run(&PreferImportingJestGlobals, Javascript, "expect(1);\n").is_empty());
    }

    #[test]
    fn jest_merges_into_the_existing_import() {
        let src = "import { jest } from '@jest/globals';\nit('b', () => { jest.fn(); });\n";
        let out = run(&PreferImportingJestGlobals, Javascript, src);
        assert_eq!(out.len(), 1);
        assert_eq!(apply(src, &out[0]), "import { it, jest } from '@jest/globals';\nit('b', () => { jest.fn(); });\n");
    }

    #[test]
    fn jest_uses_require_in_commonjs() {
        let src = "const fs = require('fs');\ntest('a', () => {});\n";
        let out = run(&PreferImportingJestGlobals, Javascript, src);
        assert_eq!(apply(src, &out[0]), format!("const {{ test }} = require('@jest/globals');\n{src}"));
    }

    #[test]
    fn vitest_globals_called_without_import() {
        let src = "describe('a', () => {\n  it('b', () => { expect(1).toBe(1); });\n});\n";
        let out = run(&PreferImportingVitestGlobals, Typescript, src);
        assert_eq!(out.len(), 3);
        assert_eq!(apply(src, &out[0]), format!("import {{ describe, it, expect }} from 'vitest';\n{src}"));
        let imported = "import { describe } from 'vitest';\ndescribe('a', () => { it('b', () => {}); });\n";
        let out = run(&PreferImportingVitestGlobals, Typescript, imported);
        assert_eq!(out.len(), 1);
        assert_eq!(apply(imported, &out[0]), "import { describe, it } from 'vitest';\ndescribe('a', () => { it('b', () => {}); });\n");
    }

    #[test]
    fn page_names() {
        assert!(page_name_full("page") && page_name_full("homePage") && page_name_full("frame1") && !page_name_full("browser"));
        assert!(page_name_prefix("pageObject") && !page_name_prefix("homePage"));
    }

    #[test]
    fn unsafe_references_in_evaluate() {
        let src = "const id = 1;\ntest('a', async ({ page }) => {\n  await page.evaluate(() => document.getElementById(id));\n});\n";
        let out = run(&PlaywrightNoUnsafeReferences, Typescript, src);
        assert_eq!(out.len(), 1);
        assert!(out[0].message.contains("\"id\""));
        assert_eq!(apply(src, &out[0]), "const id = 1;\ntest('a', async ({ page }) => {\n  await page.evaluate(([id]) => document.getElementById(id), [id]);\n});\n");
        let safe = "test('a', async ({ page }) => {\n  await page.evaluate((id) => document.getElementById(id), 1);\n});\n";
        assert!(run(&PlaywrightNoUnsafeReferences, Typescript, safe).is_empty());
    }

    #[test]
    fn native_locator_selectors() {
        assert_eq!(single_attribute("[aria-label=\"Save\"]"), Some(("aria-label", "Save")));
        assert_eq!(single_attribute("[role=button]"), Some(("role", "button")));
        assert_eq!(single_attribute("[role=button][name=\"x\"]"), None);
        let src = "page.locator('[placeholder=\"Name\"]');\n";
        let out = run(&PlaywrightPreferNativeLocators, Javascript, src);
        assert_eq!(apply(src, &out[0]), "page.getByPlaceholder(\"Name\");\n");
        assert!(run(&PlaywrightPreferNativeLocators, Javascript, "page.locator('.a');\n").is_empty());
    }

    #[test]
    fn mock_module_paths() {
        use crate::MemoryFiles;
        let src = "jest.mock('./missing');\njest.mock('./present');\njest.mock('pkg');\njest.mock('./ghost', () => ({}), { virtual: true });\n";
        let files = MemoryFiles::new("src/a.test.js", src, &[("src/present.js", "")]);
        let root = codelaw_lang::parse(Javascript, src);
        let model = codelaw_semantic::build(Javascript, &root, src);
        let options = std::collections::BTreeMap::new();
        let data = std::collections::BTreeMap::new();
        let ctx = NativeContext { source: src, language: Javascript, semantic: model.as_ref(), options: &options, data: &data, file: &files };
        let mut out = Vec::new();
        for n in root.root().dfs() {
            if n.kind().as_ref() == "call_expression" {
                ValidMockModulePath.check(n, &ctx, &mut out);
            }
        }
        assert_eq!(out.len(), 1);
        assert!(out[0].message.contains("'./missing'"));
    }
}

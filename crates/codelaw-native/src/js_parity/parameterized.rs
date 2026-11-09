//! W-139 strand W-139-jsfinal: eslint-plugin-sonarjs 4.2.2's
//! `parameterized-tests` (S5976; behaviour only: SonarJS is source-available,
//! nothing here is copied), written again. Three or more tests of one block
//! whose bodies are the same code except for a few literal values are one
//! parameterized test.

use super::files::package_json;
use super::kit::srule;
use super::{JS, span};
use crate::js_ast::arguments;
use crate::util::{children, named_children, text};
use codelaw_semantic::{Imported, SemanticModel};
use std::collections::BTreeSet;
use tree_sitter::Node;

const MIN_SIMILAR_TESTS: usize = 3;
const MIN_STATEMENTS: usize = 2;
const MAX_PARAMETERS: usize = 3;

const SUPPORTED_MODULES: &[&str] = &["bun:test", "jest", "@jest/globals", "vitest", "@playwright/test"];
const UNSUPPORTED_MODULES: &[&str] = &["cypress", "jasmine", "jasmine-core", "jasmine-node", "karma-jasmine", "mocha"];

struct Active {
    jest: bool,
    playwright: bool,
    vitest: bool,
    bun: bool,
}

fn imports_module(m: &SemanticModel, modules: &[&str]) -> bool {
    m.imports.iter().any(|i| modules.contains(&i.source.as_str()))
}

fn depends_on(ctx: &crate::NativeContext<'_>, names: &[&str]) -> bool {
    let Some((_, pkg)) = package_json(ctx, ctx.file.path(), false) else { return false };
    ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"].iter().any(|k| pkg.get(k).and_then(|d| d.as_object()).is_some_and(|d| names.iter().any(|n| d.contains_key(*n))))
}

fn active(ctx: &crate::NativeContext<'_>, m: &SemanticModel) -> Active {
    let one = |mods: &[&str]| imports_module(m, mods) || depends_on(ctx, mods);
    Active { jest: one(&["jest", "@jest/globals"]), playwright: one(&["@playwright/test"]), vitest: one(&["vitest"]), bun: one(&["bun:test"]) }
}

/// A callee as its base identifier and the property names after it.
fn callee_parts<'t, 's>(callee: Node<'t>, src: &'s str) -> Option<(Node<'t>, Vec<&'s str>)> {
    let mut mods = Vec::new();
    let mut cur = callee;
    loop {
        match cur.kind() {
            "identifier" => {
                mods.reverse();
                return Some((cur, mods));
            }
            "member_expression" => {
                if super::kit::optional(cur) {
                    return None;
                }
                let p = cur.child_by_field_name("property").filter(|p| p.kind() == "property_identifier")?;
                mods.push(text(p, src));
                cur = cur.child_by_field_name("object")?;
            }
            _ => return None,
        }
    }
}

fn concrete_modifier(modifier: &str, a: &Active) -> bool {
    modifier == "only" || (a.jest && matches!(modifier, "concurrent" | "failing")) || (a.playwright && matches!(modifier, "fail" | "slow")) || (a.vitest && matches!(modifier, "concurrent" | "fails" | "sequential"))
}

/// The test construct a call is: `Some(true)` for a concrete test (`it(…)`, `test.only(…)`),
/// `Some(false)` for one that is not (`it.each`, `test.skip`), `None` for no test call.
fn classify(m: &SemanticModel, call: Node<'_>, a: &Active, globals_ok: bool, src: &str) -> Option<bool> {
    let (base, mods) = callee_parts(call.child_by_field_name("function")?, src)?;
    let name = text(base, src);
    let sym = m.reference_at(base.start_byte()).and_then(|r| r.symbol);
    let modifiers: Vec<&str> = match sym.and_then(|s| m.import_of(s)) {
        Some(imp) if SUPPORTED_MODULES.contains(&imp.source.as_str()) && !imp.type_only => match &imp.imported {
            Imported::Named(n) if matches!(n.as_str(), "it" | "test") => mods,
            Imported::Default | Imported::Namespace if mods.first().is_some_and(|f| matches!(*f, "it" | "test")) => mods[1..].to_vec(),
            _ => return None,
        },
        Some(_) => return None,
        None => {
            if sym.is_none() && globals_ok && matches!(name, "it" | "test") && (a.jest || a.vitest) {
                mods
            } else {
                return None;
            }
        }
    };
    Some(modifiers.iter().all(|x| concrete_modifier(x, a)))
}

fn is_callback(n: Node<'_>) -> bool {
    matches!(n.kind(), "function_expression" | "arrow_function" | "function" | "generator_function")
}

struct Test<'t> {
    title: Node<'t>,
    body: Vec<Node<'t>>,
}

struct Frame<'t> {
    disabled: bool,
    tests: Vec<Test<'t>>,
}

// ---- literal differences ------------------------------------------------------------

#[derive(PartialEq, Clone, Copy)]
enum Cat {
    Str,
    Num,
    Bool,
    Null,
    Big,
}

fn literal_cat(n: Node<'_>, src: &str) -> Option<Cat> {
    match n.kind() {
        "string" => Some(Cat::Str),
        "number" => Some(if text(n, src).ends_with('n') { Cat::Big } else { Cat::Num }),
        "true" | "false" => Some(Cat::Bool),
        "null" => Some(Cat::Null),
        _ => None,
    }
}

/// A literal's value as a comparable string (the same value in two spellings compares equal).
fn literal_value(n: Node<'_>, cat: Cat, src: &str) -> String {
    match cat {
        Cat::Str => crate::util::unquote(text(n, src)).to_owned(),
        Cat::Num => super::kit::number(n, src).map_or_else(|| text(n, src).to_owned(), |v| format!("{v:?}")),
        _ => text(n, src).to_owned(),
    }
}

struct Collector<'t> {
    nodes: Vec<Node<'t>>,
    current: Vec<Node<'t>>,
}

fn same_shape<'t>(a: Node<'t>, b: Node<'t>, src: &str, current: &mut Vec<Node<'t>>) -> bool {
    if let (Some(ca), Some(cb)) = (literal_cat(a, src), literal_cat(b, src)) {
        if a.kind() == b.kind() && text(a, src) == text(b, src) {
            return true;
        }
        if ca != cb {
            return false;
        }
        if literal_value(a, ca, src) == literal_value(b, cb, src) {
            return true;
        }
        if !current.iter().any(|x| x.id() == a.id()) {
            current.push(a);
        }
        return true;
    }
    if a.kind() != b.kind() {
        return false;
    }
    let ka: Vec<Node<'t>> = children(a).into_iter().filter(|c| c.kind() != "comment").collect();
    let kb: Vec<Node<'t>> = children(b).into_iter().filter(|c| c.kind() != "comment").collect();
    if ka.is_empty() && kb.is_empty() {
        return text(a, src) == text(b, src);
    }
    ka.len() == kb.len() && ka.iter().zip(&kb).all(|(x, y)| same_shape(*x, *y, src, current))
}

fn matches<'t>(c: &mut Collector<'t>, base: &[Node<'t>], other: &[Node<'t>], src: &str) -> bool {
    base.len() == other.len() && base.iter().zip(other).all(|(x, y)| same_shape(*x, *y, src, &mut c.current))
}

fn accept<'t>(c: &mut Collector<'t>, base_len: usize) -> bool {
    let mut all: BTreeSet<usize> = c.nodes.iter().map(|n| n.id()).collect();
    all.extend(c.current.iter().map(|n| n.id()));
    if all.len() > MAX_PARAMETERS || base_len <= all.len() {
        return false;
    }
    for n in std::mem::take(&mut c.current) {
        if !c.nodes.iter().any(|x| x.id() == n.id()) {
            c.nodes.push(n);
        }
    }
    true
}

fn check_similar(tests: &[Test<'_>], src: &str, out: &mut Vec<crate::NativeFinding>) {
    if tests.len() < MIN_SIMILAR_TESTS {
        return;
    }
    let mut handled: BTreeSet<usize> = BTreeSet::new();
    for (i, test) in tests.iter().enumerate() {
        if handled.contains(&i) {
            continue;
        }
        let mut c = Collector { nodes: Vec::new(), current: Vec::new() };
        let mut equivalent = Vec::new();
        for (j, other) in tests.iter().enumerate().skip(i + 1) {
            if handled.contains(&j) {
                continue;
            }
            if matches(&mut c, &test.body, &other.body, src) && accept(&mut c, test.body.len()) {
                equivalent.push(j);
            }
            c.current.clear();
        }
        if equivalent.len() + 1 < MIN_SIMILAR_TESTS || c.nodes.is_empty() {
            continue;
        }
        handled.insert(i);
        handled.extend(equivalent.iter().copied());
        out.push(span(test.title.start_byte(), test.title.end_byte(), format!("Replace these {} tests with a single Parameterized one.", equivalent.len() + 1), vec![]));
    }
}

// ---- the walk --------------------------------------------------------------------------

struct Walk<'a, 't> {
    m: &'a SemanticModel,
    act: Active,
    globals_ok: bool,
    src: &'a str,
    frames: Vec<Frame<'t>>,
    fns: Vec<Node<'t>>,
    callbacks: BTreeSet<usize>,
    out: Vec<crate::NativeFinding>,
}

impl<'t> Walk<'_, 't> {
    fn visit(&mut self, n: Node<'t>) {
        match n.kind() {
            "statement_block" => {
                let is_test_body = self.fns.last().is_some_and(|f| self.callbacks.contains(&f.id()) && f.child_by_field_name("body").is_some_and(|b| b.id() == n.id()));
                let disabled = self.frames.last().is_some_and(|f| f.disabled) || is_test_body;
                self.frames.push(Frame { disabled, tests: Vec::new() });
                self.children(n);
                if let Some(f) = self.frames.pop()
                    && !f.disabled
                {
                    check_similar(&f.tests, self.src, &mut self.out);
                }
                return;
            }
            "call_expression" => self.call(n),
            _ => {}
        }
        let callback = is_callback(n);
        if callback {
            self.fns.push(n);
        }
        self.children(n);
        if callback {
            self.callbacks.remove(&n.id());
            self.fns.pop();
        }
    }

    fn children(&mut self, n: Node<'t>) {
        for c in children(n) {
            self.visit(c);
        }
    }

    fn call(&mut self, call: Node<'t>) {
        let Some(concrete) = classify(self.m, call, &self.act, self.globals_ok, self.src) else { return };
        let args = arguments(call);
        let Some(cb) = args.iter().copied().find(|a| is_callback(*a)) else { return };
        self.callbacks.insert(cb.id());
        if !concrete {
            return;
        }
        let Some(title) = args.first().copied() else { return };
        let static_title = match title.kind() {
            "string" => true,
            "template_string" => !named_children(title).iter().any(|c| c.kind() == "template_substitution"),
            _ => false,
        };
        let Some(body) = cb.child_by_field_name("body").filter(|b| b.kind() == "statement_block") else { return };
        let stmts: Vec<Node<'t>> = named_children(body).into_iter().filter(|c| c.kind() != "comment").collect();
        if !static_title || stmts.len() < MIN_STATEMENTS {
            return;
        }
        if let Some(f) = self.frames.last_mut()
            && !f.disabled
        {
            f.tests.push(Test { title, body: stmts });
        }
    }
}

srule!(ParameterizedTests, "js/parameterized-tests", JS, &["program"], |n, ctx, m, out| {
    let a = active(ctx, m);
    if !(a.jest || a.playwright || a.vitest || a.bun) {
        return;
    }
    let mut w = Walk { m, act: a, globals_ok: !imports_module(m, UNSUPPORTED_MODULES), src: ctx.source, frames: vec![Frame { disabled: false, tests: Vec::new() }], fns: Vec::new(), callbacks: BTreeSet::new(), out: Vec::new() };
    w.children(n);
    if let Some(f) = w.frames.pop() {
        check_similar(&f.tests, ctx.source, &mut w.out);
    }
    out.extend(w.out);
});

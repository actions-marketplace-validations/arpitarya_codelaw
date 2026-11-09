//! W-139 strand W-139-jsfinal: ESLint 10.13.0's `require-atomic-updates`,
//! read from its source and tests and written again on a walk of each
//! `async` or generator function's own code (ESLint reads the code-path
//! graph; the rule needs only what flows forward through it, and this walk
//! carries the same two sets along the same edges).
//!
//! Along every path the walk keeps the variables read since the last
//! `await`/`yield` (fresh) and those read before it (outdated); a read makes
//! a variable fresh again, an `await` or a `yield` makes every fresh one
//! outdated, and paths that meet take the union. An assignment to a
//! variable that closures (or other functions) can reach, or to a property
//! of one, is reported when its right-hand side is done and the variable is
//! outdated. Like ESLint's graph, a loop's back edge carries nothing, and a
//! `catch` clause starts from where the `try` block first can throw, where
//! it ends, and every `throw` in it.

use super::kit::{operator, optional};
use super::scope::{ancestors, enclosing_fn, fn_like};
use super::{JS, span};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{SemanticModel, SymbolId, SymbolKind};
use std::collections::BTreeSet;
use tree_sitter::Node;

#[derive(Clone, Default)]
struct State {
    fresh: BTreeSet<SymbolId>,
    outdated: BTreeSet<SymbolId>,
}

impl State {
    fn join(&mut self, other: &State) {
        self.fresh.extend(other.fresh.iter().copied());
        self.outdated.extend(other.outdated.iter().copied());
    }
    fn read(&mut self, v: SymbolId) {
        self.fresh.insert(v);
        self.outdated.remove(&v);
    }
    fn suspend(&mut self) {
        let fresh = std::mem::take(&mut self.fresh);
        self.outdated.extend(fresh);
    }
}

fn join_all(states: impl IntoIterator<Item = State>) -> Option<State> {
    let mut it = states.into_iter();
    let mut first = it.next()?;
    for s in it {
        first.join(&s);
    }
    Some(first)
}

#[derive(PartialEq, Clone, Copy)]
enum Position {
    Try,
    Catch,
    Finally,
}

struct TryCtx {
    position: Position,
    has_finalizer: bool,
    thrown: Vec<State>,
}

struct Target {
    label: Option<String>,
    is_loop: bool,
    is_switch: bool,
    breaks: Vec<State>,
    continues: Vec<State>,
}

struct Walk<'a, 't> {
    m: &'a SemanticModel,
    src: &'a str,
    root: Node<'t>,
    allow_properties: bool,
    targets: Vec<Target>,
    tries: Vec<TryCtx>,
    out: Vec<NativeFinding>,
    pending_label: Option<String>,
}

/// A registered assignment target: the variable and the node that names it.
struct Registered<'t> {
    var: SymbolId,
    ident: Node<'t>,
}

fn is_function_scope(n: Node<'_>) -> bool {
    fn_like(n.kind()) || n.kind() == "class_static_block"
}

fn function_of(n: Node<'_>) -> Option<Node<'_>> {
    ancestors(n).find(|a| is_function_scope(*a))
}

impl<'a, 't> Walk<'a, 't> {
    /// The nearest `try` where a throw goes: one in its block, or in its
    /// `catch` when it has a `finally`.
    fn throw_ctx(&mut self) -> Option<&mut TryCtx> {
        self.tries.iter_mut().rev().find(|c| c.position == Position::Try || (c.has_finalizer && c.position == Position::Catch))
    }

    fn first_throwable(&mut self, st: &State) {
        if let Some(c) = self.throw_ctx()
            && c.position == Position::Try
            && c.thrown.is_empty()
        {
            c.thrown.push(st.clone());
        }
    }

    // ---- references --------------------------------------------------------------

    /// Whether a variable is reached only from the function that declares it.
    fn local_without_escape(&self, var: SymbolId, member_access: bool) -> bool {
        let s = self.m.symbol(var);
        if member_access && s.kind == SymbolKind::Param {
            return false;
        }
        let Some(&(ds, de)) = s.decls.first() else { return false };
        let Some(id) = self.root.descendant_for_byte_range(ds, de) else { return false };
        let home = function_of(id).map(|f| f.id());
        s.refs.iter().map(|&r| &self.m.references[r]).all(|r| self.root.descendant_for_byte_range(r.start, r.end).and_then(function_of).map(|f| f.id()) == home)
    }

    /// The right-hand side an identifier is assigned, or the one of the assignment whose
    /// target is a member chain starting at the identifier.
    fn write_expr(&self, id: Node<'t>, write: bool) -> Option<(Node<'t>, Node<'t>)> {
        // (assignment-or-loop node, its right-hand side)
        let assignment_of = |n: Node<'t>| -> Option<(Node<'t>, Node<'t>)> {
            let p = n.parent()?;
            match p.kind() {
                "assignment_expression" | "augmented_assignment_expression" if p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()) => Some((p, p.child_by_field_name("right")?)),
                "for_in_statement" if p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()) => Some((p, p.child_by_field_name("right")?)),
                _ => None,
            }
        };
        if write {
            // A target of an assignment, possibly inside a destructuring pattern.
            let mut cur = id;
            while let Some(p) = cur.parent() {
                if matches!(p.kind(), "array_pattern" | "object_pattern" | "pair_pattern" | "rest_pattern" | "assignment_pattern" | "object_assignment_pattern" | "parenthesized_expression") {
                    cur = p;
                } else {
                    break;
                }
            }
            return assignment_of(cur);
        }
        let mut node = id;
        while let Some(p) = node.parent() {
            if matches!(p.kind(), "member_expression" | "subscript_expression") && p.child_by_field_name("object").is_some_and(|o| o.id() == node.id()) {
                node = p;
            } else {
                break;
            }
        }
        assignment_of(node)
    }

    /// An identifier reference: marks the read, and gives the assignment target to check later.
    fn identifier(&mut self, id: Node<'t>, st: &mut State) -> Option<(Registered<'t>, Node<'t>, Node<'t>)> {
        let r = self.m.reference_at(id.start_byte())?;
        let var = r.symbol?;
        let member_access = id.parent().is_some_and(|p| matches!(p.kind(), "member_expression" | "subscript_expression"));
        let we = self.write_expr(id, r.write && !r.read || (r.write && matches!(id.parent().map(|p| p.kind()), Some("assignment_expression" | "augmented_assignment_expression" | "for_in_statement" | "array_pattern" | "object_pattern" | "pair_pattern" | "assignment_pattern" | "object_assignment_pattern" | "rest_pattern"))));
        let plain_assign = we.is_some_and(|(p, _)| p.kind() == "assignment_expression");
        // `foo.bar = x` and `foo = x` do not read `foo`.
        if r.read && !plain_assign && !(we.is_some_and(|(p, _)| p.kind() == "for_in_statement")) {
            st.read(var);
        }
        let (assign, right) = we?;
        if right.parent().is_none_or(|p| p.id() != assign.id()) || self.local_without_escape(var, member_access) {
            return None;
        }
        Some((Registered { var, ident: id }, assign, right))
    }

    fn report(&mut self, reg: &Registered<'t>, assign: Node<'t>) {
        let name = self.m.symbol(reg.var).name.clone();
        let left = assign.child_by_field_name("left");
        if left.is_some_and(|l| l.id() == reg.ident.id()) {
            self.out.push(span(assign.start_byte(), assign.end_byte(), format!("Possible race condition: `{name}` might be reassigned based on an outdated value of `{name}`."), vec![]));
        } else if !self.allow_properties {
            let target = left.map_or("", |l| text(l, self.src));
            self.out.push(span(assign.start_byte(), assign.end_byte(), format!("Possible race condition: `{target}` might be assigned based on an outdated state of `{name}`."), vec![]));
        }
    }

    // ---- expressions ----------------------------------------------------------------------

    fn expr(&mut self, n: Node<'t>, st: &mut State) {
        match n.kind() {
            k if is_function_scope(n) && k != "class_static_block" => return,
            "comment" => return,
            "identifier" | "shorthand_property_identifier" | "shorthand_property_identifier_pattern" => {
                self.identifier(n, st);
                self.first_throwable(st);
                return;
            }
            "await_expression" | "yield_expression" => {
                for c in named_children(n) {
                    self.expr(c, st);
                }
                if n.kind() == "yield_expression" {
                    self.first_throwable(st);
                }
                st.suspend();
                return;
            }
            "assignment_expression" | "augmented_assignment_expression" => {
                self.assignment(n, st);
                return;
            }
            "binary_expression" if matches!(operator(n, self.src), Some("&&" | "||" | "??")) => {
                if let (Some(l), Some(r)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) {
                    self.expr(l, st);
                    let mut right = st.clone();
                    self.expr(r, &mut right);
                    st.join(&right);
                    return;
                }
            }
            "ternary_expression" => {
                let (Some(c), Some(a), Some(b)) = (n.child_by_field_name("condition"), n.child_by_field_name("consequence"), n.child_by_field_name("alternative")) else { return };
                self.expr(c, st);
                let mut then = st.clone();
                self.expr(a, &mut then);
                let mut other = st.clone();
                self.expr(b, &mut other);
                then.join(&other);
                *st = then;
                return;
            }
            "member_expression" | "subscript_expression" | "call_expression" => {
                if optional(n) {
                    // `a?.b`, `a?.[b]`, `a?.()`: what follows the `?.` may be skipped.
                    let object = n.child_by_field_name("object").or_else(|| n.child_by_field_name("function"));
                    for c in named_children(n) {
                        if object.is_some_and(|o| o.id() == c.id()) {
                            self.expr(c, st);
                        }
                    }
                    let mut rest = st.clone();
                    for c in named_children(n) {
                        if !object.is_some_and(|o| o.id() == c.id()) {
                            self.expr(c, &mut rest);
                        }
                    }
                    self.first_throwable(&rest);
                    st.join(&rest);
                    return;
                }
                for c in named_children(n) {
                    self.expr(c, st);
                }
                self.first_throwable(st);
                return;
            }
            "new_expression" => {
                for c in named_children(n) {
                    self.expr(c, st);
                }
                self.first_throwable(st);
                return;
            }
            "assignment_pattern" | "object_assignment_pattern" => {
                // A default value runs only when the value is missing.
                if let Some(l) = n.child_by_field_name("left") {
                    self.expr(l, st);
                }
                if let Some(r) = n.child_by_field_name("right") {
                    let mut with = st.clone();
                    self.expr(r, &mut with);
                    st.join(&with);
                }
                return;
            }
            "class" | "class_declaration" => {
                for c in named_children(n) {
                    if c.kind() == "class_heritage" {
                        self.expr(c, st);
                    }
                }
                return;
            }
            "statement_block" => {
                // A static block: its own path, not this one.
                return;
            }
            _ => {}
        }
        for c in named_children(n) {
            self.expr(c, st);
        }
    }

    fn assignment(&mut self, n: Node<'t>, st: &mut State) {
        let (Some(left), Some(right)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return };
        let mut regs = Vec::new();
        self.targets_of(left, st, &mut regs);
        let logical = matches!(operator(n, self.src), Some("&&=" | "||=" | "??="));
        if logical {
            let mut branch = st.clone();
            self.expr(right, &mut branch);
            self.check(&regs, n, &branch);
            st.join(&branch);
        } else {
            self.expr(right, st);
            self.check(&regs, n, st);
        }
    }

    /// The identifiers of an assignment's left side, visited in order.
    fn targets_of(&mut self, left: Node<'t>, st: &mut State, regs: &mut Vec<(Registered<'t>, Node<'t>)>) {
        match left.kind() {
            "identifier" | "shorthand_property_identifier_pattern" | "shorthand_property_identifier" => {
                if let Some((reg, assign, _)) = self.identifier(left, st) {
                    regs.push((reg, assign));
                }
            }
            "member_expression" | "subscript_expression" => {
                // The base identifier is the registered one; everything else is a read.
                if let Some((base, _)) = self.chain_base(left) {
                    if let Some((reg, assign, _)) = self.identifier(base, st) {
                        regs.push((reg, assign));
                    }
                    self.rest_of_chain(left, base, st);
                } else {
                    self.expr(left, st);
                }
            }
            "array_pattern" | "object_pattern" | "pair_pattern" | "rest_pattern" | "parenthesized_expression" => {
                for c in named_children(left) {
                    self.targets_of(c, st, regs);
                }
            }
            "assignment_pattern" | "object_assignment_pattern" => {
                if let Some(l) = left.child_by_field_name("left") {
                    self.targets_of(l, st, regs);
                }
                if let Some(r) = left.child_by_field_name("right") {
                    let mut with = st.clone();
                    self.expr(r, &mut with);
                    st.join(&with);
                }
            }
            _ => self.expr(left, st),
        }
    }

    fn chain_base(&self, mut n: Node<'t>) -> Option<(Node<'t>, ())> {
        loop {
            match n.kind() {
                "member_expression" | "subscript_expression" => n = n.child_by_field_name("object")?,
                "identifier" => return Some((n, ())),
                _ => return None,
            }
        }
    }

    /// Visit the parts of a member chain after its base identifier (the computed keys).
    fn rest_of_chain(&mut self, chain: Node<'t>, base: Node<'t>, st: &mut State) {
        if chain.id() == base.id() {
            return;
        }
        if let Some(o) = chain.child_by_field_name("object") {
            self.rest_of_chain(o, base, st);
        }
        for c in named_children(chain) {
            if chain.child_by_field_name("object").is_some_and(|o| o.id() == c.id()) {
                continue;
            }
            self.expr(c, st);
        }
        self.first_throwable(st);
    }

    fn check(&mut self, regs: &[(Registered<'t>, Node<'t>)], assign: Node<'t>, st: &State) {
        for (reg, a) in regs {
            if a.id() == assign.id() && st.outdated.contains(&reg.var) {
                self.report(reg, assign);
            }
        }
    }

    // ---- statements -----------------------------------------------------------------------------

    fn block(&mut self, n: Node<'t>, mut st: State) -> Option<State> {
        for c in named_children(n) {
            st = self.stmt(c, st)?;
        }
        Some(st)
    }

    fn jump(&mut self, label: Option<&str>, want_loop_for_continue: bool, is_break: bool, st: State) {
        let found = self.targets.iter_mut().rev().find(|t| match label {
            Some(l) => t.label.as_deref() == Some(l),
            None => if is_break { t.is_loop || t.is_switch } else { t.is_loop },
        });
        let _ = want_loop_for_continue;
        if let Some(t) = found {
            if is_break { t.breaks.push(st) } else { t.continues.push(st) }
        }
    }

    fn stmt(&mut self, n: Node<'t>, mut st: State) -> Option<State> {
        let label = self.pending_label.take();
        match n.kind() {
            "comment" | "empty_statement" | "function_declaration" | "generator_function_declaration" | "import_statement" | "debugger_statement" | "interface_declaration" | "type_alias_declaration" | "enum_declaration" => Some(st),
            "statement_block" => self.block(n, st),
            "expression_statement" | "lexical_declaration" | "variable_declaration" => {
                for c in named_children(n) {
                    self.expr(c, &mut st);
                }
                Some(st)
            }
            "export_statement" => {
                for c in named_children(n) {
                    st = self.stmt(c, st)?;
                }
                Some(st)
            }
            "class_declaration" => {
                self.expr(n, &mut st);
                Some(st)
            }
            "return_statement" => {
                for c in named_children(n) {
                    self.expr(c, &mut st);
                }
                None
            }
            "throw_statement" => {
                for c in named_children(n) {
                    self.expr(c, &mut st);
                }
                if let Some(c) = self.throw_ctx() {
                    c.thrown.push(st);
                }
                None
            }
            "break_statement" => {
                let l = n.child_by_field_name("label").map(|l| text(l, self.src).to_owned());
                self.jump(l.as_deref(), false, true, st);
                None
            }
            "continue_statement" => {
                let l = n.child_by_field_name("label").map(|l| text(l, self.src).to_owned());
                self.jump(l.as_deref(), true, false, st);
                None
            }
            "labeled_statement" => {
                let l = n.child_by_field_name("label").map(|l| text(l, self.src).to_owned());
                let body = n.child_by_field_name("body")?;
                if matches!(body.kind(), "for_statement" | "while_statement" | "do_statement" | "for_in_statement" | "switch_statement") {
                    self.pending_label = l;
                    return self.stmt(body, st);
                }
                self.targets.push(Target { label: l, is_loop: false, is_switch: false, breaks: Vec::new(), continues: Vec::new() });
                let end = self.stmt(body, st);
                let t = self.targets.pop()?;
                join_all(end.into_iter().chain(t.breaks))
            }
            "if_statement" => {
                if let Some(c) = n.child_by_field_name("condition") {
                    self.expr(c, &mut st);
                }
                let then = n.child_by_field_name("consequence").and_then(|c| self.stmt(c, st.clone()));
                let other = match n.child_by_field_name("alternative") {
                    Some(a) => {
                        let inner = named_children(a).into_iter().find(|c| c.kind() != "comment");
                        inner.and_then(|c| self.stmt(c, st.clone()))
                    }
                    None => Some(st),
                };
                join_all(then.into_iter().chain(other))
            }
            "while_statement" => {
                if let Some(c) = n.child_by_field_name("condition") {
                    self.expr(c, &mut st);
                }
                self.targets.push(Target { label, is_loop: true, is_switch: false, breaks: Vec::new(), continues: Vec::new() });
                if let Some(b) = n.child_by_field_name("body") {
                    self.stmt(b, st.clone());
                }
                let t = self.targets.pop()?;
                join_all(std::iter::once(st).chain(t.breaks))
            }
            "do_statement" => {
                self.targets.push(Target { label, is_loop: true, is_switch: false, breaks: Vec::new(), continues: Vec::new() });
                let end = n.child_by_field_name("body").and_then(|b| self.stmt(b, st));
                let t = self.targets.pop()?;
                let mut test = join_all(end.into_iter().chain(t.continues))?;
                if let Some(c) = n.child_by_field_name("condition") {
                    self.expr(c, &mut test);
                }
                join_all(std::iter::once(test).chain(t.breaks))
            }
            "for_statement" => {
                if let Some(i) = n.child_by_field_name("initializer") {
                    st = self.stmt(i, st)?;
                }
                let cond = n.child_by_field_name("condition").filter(|c| !matches!(c.kind(), "empty_statement" | ";"));
                if let Some(c) = cond {
                    let c = if c.kind() == "expression_statement" { c.named_child(0).unwrap_or(c) } else { c };
                    self.expr(c, &mut st);
                }
                self.targets.push(Target { label, is_loop: true, is_switch: false, breaks: Vec::new(), continues: Vec::new() });
                let end = n.child_by_field_name("body").and_then(|b| self.stmt(b, st.clone()));
                let t = self.targets.pop()?;
                if let Some(mut upd) = join_all(end.into_iter().chain(t.continues))
                    && let Some(u) = n.child_by_field_name("increment")
                {
                    self.expr(u, &mut upd);
                }
                if cond.is_some() { join_all(std::iter::once(st).chain(t.breaks)) } else { join_all(t.breaks) }
            }
            "for_in_statement" => {
                let (Some(left), Some(right)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return Some(st) };
                let mut regs = Vec::new();
                self.targets_of(left, &mut st, &mut regs);
                self.expr(right, &mut st);
                self.check(&regs, n, &st);
                self.targets.push(Target { label, is_loop: true, is_switch: false, breaks: Vec::new(), continues: Vec::new() });
                if let Some(b) = n.child_by_field_name("body") {
                    self.stmt(b, st.clone());
                }
                let t = self.targets.pop()?;
                join_all(std::iter::once(st).chain(t.breaks))
            }
            "switch_statement" => {
                if let Some(d) = n.child_by_field_name("value") {
                    self.expr(d, &mut st);
                }
                let Some(body) = n.child_by_field_name("body") else { return Some(st) };
                self.targets.push(Target { label, is_loop: false, is_switch: true, breaks: Vec::new(), continues: Vec::new() });
                let mut tests = st.clone();
                let mut fall: Option<State> = None;
                let mut has_default = false;
                for case in named_children(body) {
                    let entry_tests = tests.clone();
                    let mut entered = entry_tests.clone();
                    if case.kind() == "switch_case" {
                        if let Some(v) = case.child_by_field_name("value") {
                            self.expr(v, &mut tests);
                            entered = tests.clone();
                        }
                    } else {
                        has_default = true;
                    }
                    let mut entry = entered;
                    if let Some(f) = &fall {
                        entry.join(f);
                    }
                    let mut cur = Some(entry);
                    for c in named_children(case) {
                        if case.child_by_field_name("value").is_some_and(|v| v.id() == c.id()) {
                            continue;
                        }
                        cur = cur.and_then(|s| self.stmt(c, s));
                    }
                    fall = cur;
                }
                let t = self.targets.pop()?;
                let mut ends: Vec<State> = t.breaks;
                ends.extend(fall);
                if !has_default {
                    ends.push(tests);
                }
                join_all(ends)
            }
            "try_statement" => self.try_stmt(n, st),
            _ => {
                for c in named_children(n) {
                    self.expr(c, &mut st);
                }
                Some(st)
            }
        }
    }

    fn try_stmt(&mut self, n: Node<'t>, st: State) -> Option<State> {
        let handler = n.child_by_field_name("handler");
        let finalizer = n.child_by_field_name("finalizer");
        self.tries.push(TryCtx { position: Position::Try, has_finalizer: finalizer.is_some(), thrown: Vec::new() });
        let try_end = n.child_by_field_name("body").and_then(|b| self.stmt(b, st));
        let mut ctx = self.tries.pop()?;
        let mut normal: Vec<State> = Vec::new();
        if let Some(h) = handler {
            // The catch clause starts where the block first could throw, where it ends, and at each throw.
            let mut entries = std::mem::take(&mut ctx.thrown);
            entries.extend(try_end.iter().cloned());
            normal.extend(try_end);
            if let Some(mut entry) = join_all(entries) {
                if let Some(p) = h.child_by_field_name("parameter") {
                    self.expr(p, &mut entry);
                }
                self.tries.push(TryCtx { position: Position::Catch, has_finalizer: finalizer.is_some(), thrown: Vec::new() });
                let end = h.child_by_field_name("body").and_then(|b| self.stmt(b, entry));
                self.tries.pop();
                normal.extend(end);
            }
        } else {
            normal.extend(try_end);
        }
        let after = join_all(normal);
        match finalizer {
            Some(f) => {
                let body = f.child_by_field_name("body").or_else(|| f.named_child(0)).unwrap_or(f);
                self.tries.push(TryCtx { position: Position::Finally, has_finalizer: false, thrown: Vec::new() });
                let end = after.and_then(|s| self.stmt(body, s));
                self.tries.pop();
                end
            }
            None => after,
        }
    }
}

pub struct RequireAtomicUpdates;

impl NativeRule for RequireAtomicUpdates {
    fn id(&self) -> &'static str {
        "js/require-atomic-updates"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["function_declaration", "function_expression", "function", "arrow_function", "method_definition", "generator_function", "generator_function_declaration"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(m) = ctx.semantic else { return };
        let f = node.get_inner_node();
        let resumable = crate::js_ast::is_async(f) || matches!(f.kind(), "generator_function" | "generator_function_declaration") || (f.kind() == "method_definition" && children(f).iter().any(|c| !c.is_named() && c.kind() == "*"));
        if !resumable {
            return;
        }
        let Some(body) = f.child_by_field_name("body") else { return };
        let allow = ctx.options.get("allowProperties").and_then(serde_json::Value::as_bool).unwrap_or(false);
        let mut w = Walk { m, src: ctx.source, root: crate::js_ast::root_of(f), allow_properties: allow, targets: Vec::new(), tries: Vec::new(), out: Vec::new(), pending_label: None };
        if body.kind() == "statement_block" {
            w.block(body, State::default());
        } else {
            w.expr(body, &mut State::default());
        }
        let _ = enclosing_fn;
        out.extend(w.out);
    }
}

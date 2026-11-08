//! The flow pass of SR-PY-INFERENCE: *is this name bound on every path that
//! reaches this read?* Run on one Python scope (a module or a function) and
//! only when a rule asks, never as part of building the model.
//!
//! It walks the statements of the scope in execution order, keeping for each
//! name whether it is definitely bound, bound on some paths, or not at all.
//! Branches join, loops are walked twice (the second walk starts from the
//! join of the first walk's end and the entry), `try` handlers start from the
//! names the body binds with a chance that the binding never ran, and a
//! statement that cannot fall through (`return`, `raise`, `break`, `continue`,
//! `sys.exit()`, a call to a function annotated `NoReturn`) ends its path.
//! Nested functions, lambdas, class bodies and comprehension bodies are
//! evaluated later or elsewhere and are not read. It decides nothing about
//! conditions' values: `if FALSE:` is a branch like any other.

use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

/// What a read of a name found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowKind {
    /// No path to the read binds the name.
    Unassigned,
    /// Some paths bind it and the others skip a branch of an `if`.
    Maybe,
    /// The name is a `for` target read after a loop that may run zero times.
    LoopVariable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowUse {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub kind: FlowKind,
}

/// What the pass cannot decide from the tree.
pub struct FlowConfig<'a> {
    /// Whether a call to this (last-segment) name never returns.
    pub noreturn: &'a dyn Fn(&str) -> bool,
    /// Whether a name is a Python builtin: a module-level read of one before
    /// the module binds it finds the builtin.
    pub builtin: &'a dyn Fn(&str) -> bool,
    /// Whether a `for` iterable is statically known to yield at least once.
    pub nonempty: &'a dyn for<'t> Fn(Node<'t>) -> bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum St {
    Full,
    /// Bound on some paths. `guard` is the `if` condition whose true branch
    /// binds it; a read under the same condition is not reported.
    Partial { if_origin: bool, zero_iter: bool, guard: Option<usize> },
    /// Bound inside a `try` body: an `except` handler may run before it.
    TryOnly,
}

#[derive(Clone, Debug, Default)]
struct State {
    vars: BTreeMap<String, St>,
    dead: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Join {
    If,
    Other,
}

fn join(kind: Join, states: Vec<State>) -> State {
    let mut live: Vec<State> = states.into_iter().filter(|s| !s.dead).collect();
    match live.len() {
        0 => return State { vars: BTreeMap::new(), dead: true },
        1 => return live.pop().expect("one live state"),
        _ => {}
    }
    let names: BTreeSet<&String> = live.iter().flat_map(|s| s.vars.keys()).collect();
    let mut out = State::default();
    for name in names {
        let parts: Vec<Option<St>> = live.iter().map(|s| s.vars.get(name).copied()).collect();
        if parts.iter().all(|p| *p == Some(St::Full)) {
            out.vars.insert(name.clone(), St::Full);
            continue;
        }
        if parts.iter().all(|p| matches!(p, None | Some(St::TryOnly))) {
            if parts.contains(&Some(St::TryOnly)) {
                out.vars.insert(name.clone(), St::TryOnly);
            }
            continue;
        }
        let mut if_origin = kind == Join::If;
        let mut zero_iter = false;
        let mut guard = None;
        for p in &parts {
            match p {
                Some(St::Partial { if_origin: i, zero_iter: z, guard: g }) => {
                    if_origin |= *i;
                    zero_iter |= *z;
                    if g.is_some() {
                        guard = guard.or(*g);
                    }
                }
                Some(St::Full) | None | Some(St::TryOnly) => {}
            }
        }
        out.vars.insert(name.clone(), St::Partial { if_origin, zero_iter, guard });
    }
    out
}

struct LoopCtx {
    breaks: Vec<State>,
    continues: Vec<State>,
}

struct Flow<'s, 'c> {
    src: &'s str,
    cfg: &'c FlowConfig<'c>,
    locals: BTreeSet<String>,
    uses: Vec<FlowUse>,
    module: bool,
    report: bool,
    loops: Vec<LoopCtx>,
    conditions: Vec<String>,
    /// Conditions of the `if` branches being walked.
    active: Vec<usize>,
}

fn named<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut c = n.walk();
    n.named_children(&mut c).filter(|x| x.kind() != "comment").collect()
}

fn alternatives<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut c = n.walk();
    n.children_by_field_name("alternative", &mut c).collect()
}

fn block_of<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    match n.kind() {
        "block" | "module" => named(n),
        _ => n.child_by_field_name("body").or_else(|| n.child_by_field_name("consequence")).map(named).unwrap_or_default(),
    }
}

fn is_scope_boundary(kind: &str) -> bool {
    matches!(kind, "function_definition" | "class_definition" | "lambda")
}

/// Names a pattern of `case` binds (captures, `as` aliases, `*rest`).
fn pattern_binds(n: Node<'_>, src: &str, out: &mut Vec<(String, usize, usize)>) {
    match n.kind() {
        "dotted_name" => {
            if n.named_child_count() == 1 {
                let t = &src[n.byte_range()];
                if t != "_" {
                    out.push((t.to_owned(), n.start_byte(), n.end_byte()));
                }
            }
        }
        "class_pattern" => {
            for c in named(n).into_iter().skip(1) {
                pattern_binds(c, src, out);
            }
        }
        "keyword_pattern" => {
            for c in named(n).into_iter().skip(1) {
                pattern_binds(c, src, out);
            }
        }
        "as_pattern" => {
            for c in named(n) {
                if c.kind() == "identifier" {
                    out.push((src[c.byte_range()].to_owned(), c.start_byte(), c.end_byte()));
                } else {
                    pattern_binds(c, src, out);
                }
            }
        }
        "splat_pattern" => {
            for c in named(n) {
                if c.kind() == "identifier" && &src[c.byte_range()] != "_" {
                    out.push((src[c.byte_range()].to_owned(), c.start_byte(), c.end_byte()));
                }
            }
        }
        "string" | "integer" | "float" | "true" | "false" | "none" | "concatenated_string" | "identifier" => {}
        _ => {
            for c in named(n) {
                pattern_binds(c, src, out);
            }
        }
    }
}

/// The names bound by an assignment target.
fn target_names(n: Node<'_>, src: &str, out: &mut Vec<String>) {
    match n.kind() {
        "identifier" => out.push(src[n.byte_range()].to_owned()),
        "pattern_list" | "tuple_pattern" | "list_pattern" | "tuple" | "list" | "list_splat_pattern" | "list_splat" | "parenthesized_expression" => {
            for c in named(n) {
                target_names(c, src, out);
            }
        }
        _ => {}
    }
}

/// Every name the scope binds, by the statements of the scope itself.
fn collect_locals(scope: Node<'_>, src: &str, out: &mut BTreeSet<String>, declared: &mut BTreeSet<String>) {
    fn walk(n: Node<'_>, src: &str, out: &mut BTreeSet<String>, declared: &mut BTreeSet<String>) {
        for c in named(n) {
            match c.kind() {
                "function_definition" | "class_definition" => {
                    if let Some(name) = c.child_by_field_name("name") {
                        out.insert(src[name.byte_range()].to_owned());
                    }
                    continue;
                }
                "lambda" => continue,
                "assignment" => {
                    if let Some(l) = c.child_by_field_name("left") {
                        let mut v = Vec::new();
                        target_names(l, src, &mut v);
                        out.extend(v);
                    }
                }
                "augmented_assignment" => {
                    if let Some(l) = c.child_by_field_name("left") {
                        let mut v = Vec::new();
                        target_names(l, src, &mut v);
                        out.extend(v);
                    }
                }
                "for_statement" | "for_in_clause" => {
                    if c.kind() == "for_statement"
                        && let Some(l) = c.child_by_field_name("left")
                    {
                        let mut v = Vec::new();
                        target_names(l, src, &mut v);
                        out.extend(v);
                    }
                }
                "named_expression" => {
                    if let Some(l) = c.child_by_field_name("name") {
                        out.insert(src[l.byte_range()].to_owned());
                    }
                }
                "as_pattern" if c.parent().is_some_and(|p| matches!(p.kind(), "with_item" | "with_clause" | "except_clause" | "except_group_clause")) => {
                    for t in named(c) {
                        if t.kind() == "as_pattern_target" {
                            let mut v = Vec::new();
                            for x in named(t) {
                                target_names(x, src, &mut v);
                            }
                            out.extend(v);
                        }
                    }
                }
                "import_statement" | "import_from_statement" | "future_import_statement" => {
                    for name in import_names(c, src) {
                        out.insert(name);
                    }
                }
                "global_statement" | "nonlocal_statement" => {
                    for id in named(c) {
                        if id.kind() == "identifier" {
                            declared.insert(src[id.byte_range()].to_owned());
                        }
                    }
                }
                "case_clause" => {
                    for p in named(c) {
                        if p.kind() == "case_pattern" {
                            let mut v = Vec::new();
                            pattern_binds(p, src, &mut v);
                            out.extend(v.into_iter().map(|x| x.0));
                        }
                    }
                }
                // A comprehension's walrus binds in this scope; its own loop
                // variables do not.
                "list_comprehension" | "set_comprehension" | "dictionary_comprehension" | "generator_expression" => {
                    let mut stack = vec![c];
                    while let Some(x) = stack.pop() {
                        for y in named(x) {
                            if y.kind() == "named_expression"
                                && let Some(l) = y.child_by_field_name("name")
                            {
                                out.insert(src[l.byte_range()].to_owned());
                            }
                            if !is_scope_boundary(y.kind()) {
                                stack.push(y);
                            }
                        }
                    }
                    continue;
                }
                _ => {}
            }
            walk(c, src, out, declared);
        }
    }
    walk(scope, src, out, declared);
}

fn import_names(n: Node<'_>, src: &str) -> Vec<String> {
    let mut v = Vec::new();
    let module = n.child_by_field_name("module_name");
    for c in named(n) {
        if Some(c) == module {
            continue;
        }
        match c.kind() {
            "dotted_name" => {
                let first = if n.kind() == "import_statement" { c.named_child(0) } else { c.named_child(c.named_child_count().saturating_sub(1) as u32) };
                if let Some(f) = first {
                    v.push(src[f.byte_range()].to_owned());
                }
            }
            "aliased_import" => {
                if let Some(a) = c.child_by_field_name("alias") {
                    v.push(src[a.byte_range()].to_owned());
                }
            }
            _ => {}
        }
    }
    v
}

fn pure_value(n: Node<'_>) -> bool {
    match n.kind() {
        "integer" | "float" | "true" | "false" | "none" | "identifier" => true,
        "string" => named(n).iter().all(|c| c.kind() != "interpolation"),
        "tuple" | "list" => named(n).into_iter().all(pure_value),
        _ => false,
    }
}

impl<'s, 'c> Flow<'s, 'c> {
    fn text(&self, n: Node<'_>) -> &'s str {
        &self.src[n.byte_range()]
    }

    fn intern(&mut self, cond: Node<'_>) -> usize {
        let t = self.text(cond).split_whitespace().collect::<Vec<_>>().join(" ");
        match self.conditions.iter().position(|c| *c == t) {
            Some(i) => i,
            None => {
                self.conditions.push(t);
                self.conditions.len() - 1
            }
        }
    }

    fn bind(&self, name: &str, st: &mut State) {
        st.vars.insert(name.to_owned(), St::Full);
    }

    fn read(&mut self, n: Node<'_>, st: &mut State) {
        let name = self.text(n);
        if !self.locals.contains(name) {
            return;
        }
        let kind = match st.vars.get(name) {
            Some(St::Full) => return,
            None if self.module && (self.cfg.builtin)(name) => return,
            None | Some(St::TryOnly) => FlowKind::Unassigned,
            Some(St::Partial { zero_iter: true, .. }) => FlowKind::LoopVariable,
            Some(St::Partial { guard: Some(g), .. }) if self.active.contains(g) => return,
            Some(St::Partial { if_origin: true, .. }) => FlowKind::Maybe,
            Some(St::Partial { .. }) => return,
        };
        if self.report {
            self.uses.push(FlowUse { name: name.to_owned(), start: n.start_byte(), end: n.end_byte(), kind });
        }
        // Once per path: the name is as good as bound after a report.
        st.vars.insert(name.to_owned(), St::Full);
    }

    fn bind_target(&mut self, n: Node<'_>, st: &mut State) {
        match n.kind() {
            "identifier" => {
                let name = self.text(n);
                self.bind(name, st);
            }
            "pattern_list" | "tuple_pattern" | "list_pattern" | "tuple" | "list" | "list_splat_pattern" | "list_splat" | "parenthesized_expression" => {
                for c in named(n) {
                    self.bind_target(c, st);
                }
            }
            // `a.b = …`, `a[i] = …`: reads of `a` and `i`.
            _ => self.expr(n, st),
        }
    }

    /// Evaluate an expression in order: reads are checked, `:=` binds.
    fn expr(&mut self, n: Node<'_>, st: &mut State) {
        match n.kind() {
            "identifier" => self.read(n, st),
            "lambda" | "type" | "comment" => {}
            "attribute" => {
                if let Some(o) = n.child_by_field_name("object") {
                    self.expr(o, st);
                }
            }
            "keyword_argument" => {
                if let Some(v) = n.child_by_field_name("value") {
                    self.expr(v, st);
                }
            }
            "named_expression" => {
                if let Some(v) = n.child_by_field_name("value") {
                    self.expr(v, st);
                }
                if let Some(name) = n.child_by_field_name("name") {
                    let t = self.text(name);
                    self.bind(t, st);
                }
            }
            "list_comprehension" | "set_comprehension" | "dictionary_comprehension" | "generator_expression" => {
                // Only the first iterable is evaluated here; a walrus in the
                // rest binds in this scope (and is treated as bound).
                if let Some(first) = named(n).into_iter().find(|c| c.kind() == "for_in_clause")
                    && let Some(r) = first.child_by_field_name("right")
                {
                    self.expr(r, st);
                }
                let mut stack = vec![n];
                while let Some(x) = stack.pop() {
                    for y in named(x) {
                        if y.kind() == "named_expression"
                            && let Some(l) = y.child_by_field_name("name")
                        {
                            let t = self.text(l);
                            self.bind(t, st);
                        }
                        if !is_scope_boundary(y.kind()) {
                            stack.push(y);
                        }
                    }
                }
            }
            "boolean_operator" => {
                if let Some(l) = n.child_by_field_name("left") {
                    self.expr(l, st);
                }
                if let Some(r) = n.child_by_field_name("right") {
                    let mut other = st.clone();
                    self.expr(r, &mut other);
                    *st = join(Join::If, vec![st.clone(), other]);
                    // The right operand's reads were checked; keep what the
                    // left side already bound.
                }
            }
            "conditional_expression" => {
                let parts = named(n);
                if let [value, cond, other] = parts.as_slice() {
                    self.expr(*cond, st);
                    let mut a = st.clone();
                    let mut b = st.clone();
                    self.expr(*value, &mut a);
                    self.expr(*other, &mut b);
                    *st = join(Join::If, vec![a, b]);
                }
            }
            "string" => {
                for c in named(n) {
                    if c.kind() == "interpolation"
                        && let Some(e) = c.child_by_field_name("expression")
                    {
                        self.expr(e, st);
                    }
                }
            }
            "call" => {
                for c in named(n) {
                    self.expr(c, st);
                }
            }
            _ => {
                for c in named(n) {
                    self.expr(c, st);
                }
            }
        }
    }

    /// A condition's states on the path where it held and where it did not:
    /// `a and (b := f())` has bound `b` where it held.
    fn cond(&mut self, n: Node<'_>, st: &State) -> (State, State) {
        let n = if n.kind() == "parenthesized_expression" { n.named_child(0).unwrap_or(n) } else { n };
        match n.kind() {
            "boolean_operator" => {
                let is_and = n.child_by_field_name("operator").is_some_and(|o| o.kind() == "and");
                let (Some(l), Some(r)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else {
                    let mut s = st.clone();
                    self.expr(n, &mut s);
                    return (s.clone(), s);
                };
                let (lt, lf) = self.cond(l, st);
                if is_and {
                    let (rt, rf) = self.cond(r, &lt);
                    (rt, join(Join::If, vec![lf, rf]))
                } else {
                    let (rt, rf) = self.cond(r, &lf);
                    (join(Join::If, vec![lt, rt]), rf)
                }
            }
            "not_operator" => match n.child_by_field_name("argument") {
                Some(a) => {
                    let (t, f) = self.cond(a, st);
                    (f, t)
                }
                None => (st.clone(), st.clone()),
            },
            _ => {
                let mut s = st.clone();
                self.expr(n, &mut s);
                (s.clone(), s)
            }
        }
    }

    fn block(&mut self, stmts: Vec<Node<'_>>, st: &mut State) {
        for s in stmts {
            if st.dead {
                break;
            }
            self.stmt(s, st);
        }
    }

    fn terminating_call(&self, e: Node<'_>) -> bool {
        let call = match e.kind() {
            "call" => e,
            "await" => match e.named_child(0) {
                Some(c) if c.kind() == "call" => c,
                _ => return false,
            },
            _ => return false,
        };
        let Some(f) = call.child_by_field_name("function") else { return false };
        let full = self.text(f);
        if matches!(full, "sys.exit" | "exit" | "quit" | "os._exit" | "os.abort" | "_exit") {
            return true;
        }
        let leaf = full.rsplit('.').next().unwrap_or(full);
        (self.cfg.noreturn)(leaf)
    }

    fn stmt(&mut self, n: Node<'_>, st: &mut State) {
        match n.kind() {
            "expression_statement" => {
                for c in named(n) {
                    match c.kind() {
                        "assignment" => self.assignment(c, st),
                        "augmented_assignment" => {
                            if let Some(r) = c.child_by_field_name("right") {
                                // `x += …` reads x first.
                                if let Some(l) = c.child_by_field_name("left") {
                                    self.expr(l, st);
                                }
                                self.expr(r, st);
                            }
                            if let Some(l) = c.child_by_field_name("left") {
                                self.bind_target(l, st);
                            }
                        }
                        _ => {
                            self.expr(c, st);
                            if self.terminating_call(c) {
                                st.dead = true;
                            }
                        }
                    }
                }
            }
            "assignment" => self.assignment(n, st),
            "return_statement" | "raise_statement" => {
                for c in named(n) {
                    self.expr(c, st);
                }
                st.dead = true;
            }
            "break_statement" => {
                if let Some(l) = self.loops.last_mut() {
                    l.breaks.push(st.clone());
                }
                st.dead = true;
            }
            "continue_statement" => {
                if let Some(l) = self.loops.last_mut() {
                    l.continues.push(st.clone());
                }
                st.dead = true;
            }
            "assert_statement" => {
                for c in named(n) {
                    self.expr(c, st);
                }
            }
            "import_statement" | "import_from_statement" | "future_import_statement" => {
                for name in import_names(n, self.src) {
                    self.bind(&name, st);
                }
            }
            "function_definition" => {
                if let Some(p) = n.child_by_field_name("parameters") {
                    for prm in named(p) {
                        if let Some(v) = prm.child_by_field_name("value") {
                            self.expr(v, st);
                        }
                    }
                }
                if let Some(name) = n.child_by_field_name("name") {
                    let t = self.text(name);
                    self.bind(t, st);
                }
            }
            "class_definition" => {
                if let Some(s) = n.child_by_field_name("superclasses") {
                    self.expr(s, st);
                }
                if let Some(name) = n.child_by_field_name("name") {
                    let t = self.text(name);
                    self.bind(t, st);
                }
            }
            "decorated_definition" => {
                for c in named(n) {
                    match c.kind() {
                        "decorator" => {
                            for e in named(c) {
                                self.expr(e, st);
                            }
                        }
                        _ => self.stmt(c, st),
                    }
                }
            }
            "if_statement" => self.if_statement(n, st),
            "for_statement" => self.for_statement(n, st),
            "while_statement" => self.while_statement(n, st),
            "try_statement" => self.try_statement(n, st),
            "with_statement" => {
                for item in named(n).into_iter().filter(|c| c.kind() == "with_clause") {
                    for it in named(item) {
                        let value = if it.kind() == "with_item" { it.child_by_field_name("value").unwrap_or(it) } else { it };
                        self.with_item(value, st);
                    }
                }
                if let Some(b) = n.child_by_field_name("body") {
                    self.block(block_of(b), st);
                }
            }
            "match_statement" => self.match_statement(n, st),
            "global_statement" | "nonlocal_statement" | "pass_statement" | "delete_statement" | "type_alias_statement" | "comment" => {}
            _ => {
                for c in named(n) {
                    self.expr(c, st);
                }
            }
        }
    }

    fn with_item(&mut self, value: Node<'_>, st: &mut State) {
        if value.kind() == "as_pattern" {
            for c in named(value) {
                if c.kind() == "as_pattern_target" {
                    continue;
                }
                self.expr(c, st);
            }
            for c in named(value) {
                if c.kind() == "as_pattern_target" {
                    for t in named(c) {
                        self.bind_target(t, st);
                    }
                }
            }
        } else {
            self.expr(value, st);
        }
    }

    fn assignment(&mut self, n: Node<'_>, st: &mut State) {
        if let Some(r) = n.child_by_field_name("right") {
            // `a = b = 1` nests: the inner assignment evaluates and binds first.
            if r.kind() == "assignment" {
                self.assignment(r, st);
            } else {
                self.expr(r, st);
            }
            if let Some(l) = n.child_by_field_name("left") {
                self.bind_target(l, st);
            }
        } else if let Some(l) = n.child_by_field_name("left") {
            // `x: int` declares but does not bind; attribute and subscript
            // targets still read their object.
            if l.kind() != "identifier" {
                self.expr(l, st);
            }
        }
    }

    fn if_statement(&mut self, n: Node<'_>, st: &mut State) {
        let mut ends: Vec<State> = Vec::new();
        let mut cond = n.child_by_field_name("condition");
        let mut body = n.child_by_field_name("consequence");
        let mut alts = alternatives(n).into_iter();
        let mut has_else = false;
        let mut first_guard: Option<usize> = None;
        // The state on the path where every condition so far was false.
        let mut fallthrough = st.clone();
        loop {
            let mut branch = fallthrough.clone();
            if let Some(c) = cond {
                let (t, f) = self.cond(c, &fallthrough);
                branch = t;
                fallthrough = f;
            }
            let guard = cond.map(|c| self.intern(c));
            if first_guard.is_none() {
                first_guard = guard;
            }
            if let Some(g) = guard {
                self.active.push(g);
            }
            if let Some(b) = body {
                self.block(block_of(b), &mut branch);
            }
            if guard.is_some() {
                self.active.pop();
            }
            ends.push(branch);
            match alts.next() {
                Some(a) if a.kind() == "elif_clause" => {
                    cond = a.child_by_field_name("condition");
                    body = a.child_by_field_name("consequence");
                }
                Some(a) => {
                    has_else = true;
                    let mut branch = fallthrough.clone();
                    if let Some(b) = a.child_by_field_name("body") {
                        self.block(block_of(b), &mut branch);
                    }
                    ends.push(branch);
                    break;
                }
                None => break,
            }
        }
        if !has_else {
            ends.push(fallthrough);
        }
        // Names only the first branch binds are guarded by its condition.
        let guarded: Vec<String> = match (first_guard, ends.first()) {
            (Some(_), Some(first)) if !first.dead => first
                .vars
                .iter()
                .filter(|(k, v)| **v == St::Full && !matches!(st.vars.get(*k), Some(St::Full)) && !ends.iter().skip(1).any(|e| !e.dead && e.vars.get(*k) == Some(&St::Full)))
                .map(|(k, _)| k.clone())
                .collect(),
            _ => Vec::new(),
        };
        *st = join(Join::If, ends);
        if let Some(g) = first_guard {
            for name in guarded {
                if let Some(St::Partial { guard, .. }) = st.vars.get_mut(&name) {
                    *guard = Some(g);
                }
            }
        }
    }

    fn loop_body(&mut self, target: Option<Node<'_>>, cond: Option<Node<'_>>, body: Option<Node<'_>>, entry: &State) -> (State, LoopCtx, State) {
        // Pass 1, silent: what the end of an iteration leaves behind.
        let saved = self.report;
        self.report = false;
        self.loops.push(LoopCtx { breaks: Vec::new(), continues: Vec::new() });
        let mut s = entry.clone();
        if let Some(t) = target {
            self.bind_target(t, &mut s);
        }
        if let Some(b) = body {
            self.block(block_of(b), &mut s);
        }
        let first = self.loops.pop().expect("loop context");
        self.report = saved;
        let mut back = vec![s];
        back.extend(first.continues);
        let mut head_parts = vec![entry.clone()];
        head_parts.extend(back);
        let head = join(Join::Other, head_parts);
        // Pass 2, reporting, from the head state.
        let mut h = head.clone();
        if let Some(c) = cond {
            self.expr(c, &mut h);
        }
        self.loops.push(LoopCtx { breaks: Vec::new(), continues: Vec::new() });
        let mut s2 = h.clone();
        if let Some(t) = target {
            self.bind_target(t, &mut s2);
        }
        if let Some(b) = body {
            self.block(block_of(b), &mut s2);
        }
        let ctx = self.loops.pop().expect("loop context");
        (head, ctx, h)
    }

    fn finish_loop(&mut self, head: State, ctx: LoopCtx, else_block: Option<Node<'_>>, infinite: bool, st: &mut State) {
        let mut normal = head;
        if infinite {
            normal.dead = true;
        }
        if let Some(e) = else_block
            && !normal.dead
            && let Some(b) = e.child_by_field_name("body")
        {
            self.block(block_of(b), &mut normal);
        }
        let mut parts = vec![normal];
        parts.extend(ctx.breaks);
        *st = join(Join::Other, parts);
    }

    fn for_statement(&mut self, n: Node<'_>, st: &mut State) {
        let target = n.child_by_field_name("left");
        let iter = n.child_by_field_name("right");
        if let Some(i) = iter {
            self.expr(i, st);
        }
        let known_nonempty = iter.is_some_and(|i| (self.cfg.nonempty)(i));
        let (mut head, ctx, _) = self.loop_body(target, None, n.child_by_field_name("body"), st);
        // A loop that may run zero times leaves its target unbound.
        if let Some(t) = target {
            let mut names = Vec::new();
            target_names(t, self.src, &mut names);
            for name in names {
                let before = st.vars.get(&name).copied();
                if before == Some(St::Full) {
                    continue;
                }
                if known_nonempty {
                    head.vars.insert(name, St::Full);
                } else if let Some(St::Partial { zero_iter, .. }) = head.vars.get_mut(&name) {
                    *zero_iter = true;
                } else if !head.dead {
                    head.vars.insert(name, St::Partial { if_origin: false, zero_iter: true, guard: None });
                }
            }
        }
        let alt = alternatives(n).into_iter().next();
        self.finish_loop(head, ctx, alt, false, st);
    }

    fn while_statement(&mut self, n: Node<'_>, st: &mut State) {
        let cond = n.child_by_field_name("condition");
        if let Some(c) = cond {
            self.expr(c, st);
        }
        let infinite = cond.is_some_and(|c| matches!(c.kind(), "true") || (c.kind() == "integer" && self.text(c) != "0"));
        let (head, ctx, _) = self.loop_body(None, cond, n.child_by_field_name("body"), st);
        let alt = alternatives(n).into_iter().next();
        self.finish_loop(head, ctx, alt, infinite, st);
    }

    fn try_statement(&mut self, n: Node<'_>, st: &mut State) {
        let body = n.child_by_field_name("body").map(block_of).unwrap_or_default();
        let entry = st.clone();
        // The names the body binds, and whether the first one certainly ran.
        let mut bound = BTreeSet::new();
        let mut declared = BTreeSet::new();
        if let Some(b) = n.child_by_field_name("body") {
            collect_locals(b, self.src, &mut bound, &mut declared);
        }
        let mut safe: BTreeSet<String> = BTreeSet::new();
        if let Some(first) = body.first()
            && first.kind() == "expression_statement"
            && let Some(a) = first.named_child(0)
            && a.kind() == "assignment"
            && a.child_by_field_name("right").is_some_and(pure_value)
            && let Some(l) = a.child_by_field_name("left")
        {
            let mut v = Vec::new();
            target_names(l, self.src, &mut v);
            safe.extend(v);
        }
        let mut body_end = entry.clone();
        self.block(body.clone(), &mut body_end);
        let mut handler_entry = entry.clone();
        for name in &bound {
            if !self.locals.contains(name) {
                continue;
            }
            match handler_entry.vars.get(name) {
                Some(St::Full) => {}
                _ => {
                    handler_entry.vars.insert(name.clone(), if safe.contains(name) { St::Full } else { St::TryOnly });
                }
            }
        }
        let mut parts = Vec::new();
        let mut finally = None;
        let mut else_block = None;
        let mut had_handler = false;
        for c in named(n) {
            match c.kind() {
                "except_clause" | "except_group_clause" => {
                    had_handler = true;
                    let mut h = handler_entry.clone();
                    let mut block = None;
                    for x in named(c) {
                        match x.kind() {
                            "block" => block = Some(x),
                            "as_pattern" => {
                                for y in named(x) {
                                    if y.kind() == "as_pattern_target" {
                                        for t in named(y) {
                                            self.bind_target(t, &mut h);
                                        }
                                    } else {
                                        self.expr(y, &mut h);
                                    }
                                }
                            }
                            _ => self.expr(x, &mut h),
                        }
                    }
                    if let Some(b) = block {
                        self.block(block_of(b), &mut h);
                    }
                    parts.push(h);
                }
                "else_clause" => else_block = Some(c),
                "finally_clause" => finally = Some(c),
                _ => {}
            }
        }
        if let Some(e) = else_block
            && let Some(b) = e.child_by_field_name("body")
        {
            self.block(block_of(b), &mut body_end);
        }
        parts.push(body_end);
        let _ = had_handler;
        let mut after = join(Join::Other, parts);
        if let Some(f) = finally {
            let blk = f.child_by_field_name("body").or_else(|| named(f).into_iter().find(|c| c.kind() == "block"));
            if let Some(b) = blk {
                self.block(block_of(b), &mut after);
            }
        }
        *st = after;
    }

    fn match_statement(&mut self, n: Node<'_>, st: &mut State) {
        if let Some(s) = n.child_by_field_name("subject") {
            self.expr(s, st);
        }
        let Some(body) = n.child_by_field_name("body") else { return };
        let mut ends = Vec::new();
        let mut irrefutable = false;
        for case in named(body).into_iter().filter(|c| c.kind() == "case_clause") {
            let mut s = st.clone();
            let mut guard = false;
            for p in named(case) {
                match p.kind() {
                    "case_pattern" => {
                        let mut v = Vec::new();
                        pattern_binds(p, self.src, &mut v);
                        for (name, _, _) in &v {
                            self.bind(name, &mut s);
                        }
                        let t = self.text(p).trim();
                        if t == "_" || (v.len() == 1 && v[0].0 == t) {
                            irrefutable = true;
                        }
                    }
                    "if_clause" => {
                        guard = true;
                        for e in named(p) {
                            self.expr(e, &mut s);
                        }
                    }
                    _ => {}
                }
            }
            if guard {
                irrefutable = false;
            }
            if let Some(b) = case.child_by_field_name("consequence").or_else(|| named(case).into_iter().find(|c| c.kind() == "block")) {
                self.block(block_of(b), &mut s);
            }
            ends.push(s);
        }
        if !irrefutable {
            ends.push(st.clone());
        }
        *st = join(Join::If, ends);
    }
}

/// Walk one scope — a `module` or a `function_definition` node — and report
/// every read of a name the scope binds that the walk finds not bound (or
/// bound on only some paths) at that point.
pub fn analyze(scope: Node<'_>, src: &str, cfg: &FlowConfig<'_>) -> Vec<FlowUse> {
    let mut locals = BTreeSet::new();
    let mut declared = BTreeSet::new();
    let (stmts, mut entry) = if scope.kind() == "function_definition" {
        let body = scope.child_by_field_name("body").map(|b| (b, block_of(b)));
        let mut st = State::default();
        if let Some(p) = scope.child_by_field_name("parameters") {
            let mut stack = named(p);
            while let Some(x) = stack.pop() {
                match x.kind() {
                    "identifier" => {
                        locals.insert(src[x.byte_range()].to_owned());
                        st.vars.insert(src[x.byte_range()].to_owned(), St::Full);
                    }
                    "default_parameter" | "typed_default_parameter" | "typed_parameter" => {
                        if let Some(nm) = x.child_by_field_name("name").or_else(|| x.named_child(0)) {
                            stack.push(nm);
                        }
                    }
                    "list_splat_pattern" | "dictionary_splat_pattern" | "tuple_pattern" => stack.extend(named(x)),
                    _ => {}
                }
            }
        }
        match body {
            Some((b, s)) => {
                collect_locals(b, src, &mut locals, &mut declared);
                (s, st)
            }
            None => return Vec::new(),
        }
    } else {
        collect_locals(scope, src, &mut locals, &mut declared);
        (block_of(scope), State::default())
    };
    // A wildcard import may bind any name.
    if stmts.iter().any(|s| s.kind() == "import_from_statement" && named(*s).iter().any(|c| c.kind() == "wildcard_import")) {
        return Vec::new();
    }
    for d in &declared {
        locals.remove(d);
    }
    let mut flow = Flow { src, cfg, locals, uses: Vec::new(), module: scope.kind() == "module", report: true, loops: Vec::new(), conditions: Vec::new(), active: Vec::new() };
    flow.block(stmts, &mut entry);
    flow.uses
}

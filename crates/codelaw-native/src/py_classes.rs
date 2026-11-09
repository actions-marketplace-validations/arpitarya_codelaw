//! W-174, the in-file class-hierarchy group of SR-PY-INFERENCE: Pylint 4.1.3
//! checks that need to know which class derives from which and what a base
//! defines, answered from the classes of **one file**. The table below holds
//! a file's module-level classes with their bases by name; a base the file
//! imports makes the ancestry *open*, and a rule that needs the whole
//! ancestry (an override, an abstract method, a missing `__init__` call)
//! skips such a class. Cross-file bases wait on the project graph.
//! Behaviour only, from Pylint's functional tests; written fresh.

use crate::python_util::{call_args, decorator_target, decorators, kids, py_rule};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use std::collections::{BTreeMap, BTreeSet};
use codelaw_core::TextEdit;
use tree_sitter::Node;

struct Method<'t> {
    name: &'t str,
    node: Node<'t>,
    decorators: Vec<&'t str>,
    is_async: bool,
}

struct Class<'t> {
    name: &'t str,
    node: Node<'t>,
    name_node: Node<'t>,
    /// Dotted text of each plain base.
    bases: Vec<&'t str>,
    /// A base is a call, a subscript or a starred expression.
    opaque_base: bool,
    /// A base is something other than a plain or a subscripted name.
    opaque_other: bool,
    /// The names inside subscripted bases: `Base` for `Base[T]`.
    sub_bases: Vec<&'t str>,
    metaclass: Option<&'t str>,
    decorators: Vec<&'t str>,
    methods: Vec<Method<'t>>,
    body: Vec<Node<'t>>,
}

struct Table<'t> {
    classes: Vec<Class<'t>>,
    by_name: BTreeMap<&'t str, usize>,
    src: &'t str,
}

const ENUMS: &[&str] = &["Enum", "IntEnum", "StrEnum", "Flag", "IntFlag", "ReprEnum", "enum.Enum", "enum.IntEnum", "enum.StrEnum", "enum.Flag", "enum.IntFlag"];

fn leaf(dotted: &str) -> &str {
    dotted.rsplit('.').next().unwrap_or(dotted)
}

impl<'t> Table<'t> {
    fn build(root: Node<'t>, src: &'t str) -> Table<'t> {
        let mut classes = Vec::new();
        for s in named_children(root) {
            let def = if s.kind() == "decorated_definition" { s.child_by_field_name("definition").unwrap_or(s) } else { s };
            if def.kind() != "class_definition" {
                continue;
            }
            let Some(name_node) = def.child_by_field_name("name") else { continue };
            let mut c = Class {
                name: text(name_node, src),
                node: def,
                name_node,
                bases: Vec::new(),
                opaque_base: false,
                opaque_other: false,
                sub_bases: Vec::new(),
                metaclass: None,
                decorators: decorators(def).into_iter().map(|d| text(decorator_target(d), src)).collect(),
                methods: Vec::new(),
                body: Vec::new(),
            };
            if let Some(sup) = def.child_by_field_name("superclasses") {
                for b in kids(sup) {
                    match b.kind() {
                        "identifier" | "attribute" => c.bases.push(text(b, src)),
                        "keyword_argument" => {
                            if b.child_by_field_name("name").is_some_and(|n| text(n, src) == "metaclass") {
                                c.metaclass = b.child_by_field_name("value").map(|v| text(v, src));
                            }
                        }
                        "subscript" => {
                            c.opaque_base = true;
                            match b.child_by_field_name("value") {
                                Some(v) if matches!(v.kind(), "identifier" | "attribute") => c.sub_bases.push(text(v, src)),
                                _ => c.opaque_other = true,
                            }
                        }
                        _ => {
                            c.opaque_base = true;
                            c.opaque_other = true;
                        }
                    }
                }
            }
            if let Some(body) = def.child_by_field_name("body") {
                c.body = kids(body);
                for st in &c.body {
                    let f = if st.kind() == "decorated_definition" { st.child_by_field_name("definition").unwrap_or(*st) } else { *st };
                    if f.kind() == "function_definition"
                        && let Some(n) = f.child_by_field_name("name")
                    {
                        c.methods.push(Method {
                            name: text(n, src),
                            node: f,
                            decorators: decorators(f).into_iter().map(|d| text(decorator_target(d), src)).collect(),
                            is_async: text(f, src).starts_with("async"),
                        });
                    }
                }
            }
            classes.push(c);
        }
        let mut by_name: BTreeMap<&str, usize> = BTreeMap::new();
        let mut dup: BTreeSet<&str> = BTreeSet::new();
        for (i, c) in classes.iter().enumerate() {
            if by_name.insert(c.name, i).is_some() {
                dup.insert(c.name);
            }
        }
        for d in dup {
            by_name.remove(d);
        }
        Table { classes, by_name, src }
    }

    fn base_index(&self, base: &str) -> Option<usize> {
        self.by_name.get(base).copied()
    }

    /// The in-file ancestors of class `i`, nearest first (depth first, in
    /// base order), and whether every base was either in the file or
    /// `object`/`ABC`.
    fn ancestry(&self, i: usize) -> (Vec<usize>, bool) {
        let mut out = Vec::new();
        let mut closed = !self.classes[i].opaque_base;
        let mut stack = vec![i];
        let mut seen = BTreeSet::new();
        while let Some(c) = stack.pop() {
            if !seen.insert(c) {
                continue;
            }
            if c != i {
                out.push(c);
            }
            let cls = &self.classes[c];
            closed &= !cls.opaque_base;
            for b in cls.bases.iter().rev() {
                match self.base_index(b) {
                    Some(j) => stack.push(j),
                    None if matches!(leaf(b), "object" | "ABC") => {}
                    None => closed = false,
                }
            }
        }
        (out, closed)
    }

    fn is_enum(&self, i: usize) -> bool {
        let c = &self.classes[i];
        c.bases.iter().any(|b| ENUMS.contains(b) || self.base_index(b).is_some_and(|j| j != i && self.is_enum(j)))
    }

    fn is_abc(&self, i: usize) -> bool {
        let (anc, _) = self.ancestry(i);
        std::iter::once(i).chain(anc).any(|j| {
            let c = &self.classes[j];
            c.bases.iter().any(|b| leaf(b) == "ABC") || c.metaclass.is_some_and(|m| leaf(m) == "ABCMeta")
        })
    }

    fn method<'a>(&'a self, i: usize, name: &str) -> Option<&'a Method<'t>> {
        self.classes[i].methods.iter().rev().find(|m| m.name == name)
    }

    /// C3 linearization over the file's classes; a base the file does not
    /// hold is a leaf. `None` when no consistent order exists.
    fn mro(&self, i: usize, stack: &mut Vec<usize>) -> Option<Vec<String>> {
        if stack.contains(&i) {
            return None;
        }
        stack.push(i);
        let c = &self.classes[i];
        let mut lists: Vec<Vec<String>> = Vec::new();
        for b in &c.bases {
            match self.base_index(b) {
                Some(j) => lists.push(self.mro(j, stack)?),
                None => lists.push(vec![(*b).to_owned()]),
            }
        }
        lists.push(c.bases.iter().map(|b| (*b).to_owned()).collect());
        stack.pop();
        let mut out = vec![c.name.to_owned()];
        loop {
            lists.retain(|l| !l.is_empty());
            if lists.is_empty() {
                return Some(out);
            }
            let cand = lists.iter().map(|l| l[0].clone()).find(|h| !lists.iter().any(|l| l[1..].contains(h)))?;
            out.push(cand.clone());
            for l in lists.iter_mut() {
                if l.first() == Some(&cand) {
                    l.remove(0);
                }
            }
        }
    }

    fn has_decorator(&self, ds: &[&str], name: &str) -> bool {
        ds.iter().any(|d| leaf(d) == name)
    }
}

fn at(n: Node<'_>, message: String) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message, edits: Vec::new() }
}

// ---------------------------------------------------------------- parameters

struct Params<'t> {
    /// Positional parameters, including positional-only ones, in order.
    positional: Vec<&'t str>,
    defaults: usize,
    posonly: usize,
    kwonly: Vec<&'t str>,
    vararg: bool,
    kwarg: bool,
}

fn params<'t>(def: Node<'t>, src: &'t str) -> Params<'t> {
    let mut p = Params { positional: Vec::new(), defaults: 0, posonly: 0, kwonly: Vec::new(), vararg: false, kwarg: false };
    let Some(list) = def.child_by_field_name("parameters") else { return p };
    let mut after_star = false;
    for x in kids(list) {
        let name = |n: Node<'t>| -> &'t str {
            match n.kind() {
                "identifier" => text(n, src),
                _ => n.child_by_field_name("name").or_else(|| n.named_child(0)).map_or("", |c| text(c, src)),
            }
        };
        // `*args: T` and `**kwargs: T` are a `typed_parameter` wrapping the splat.
        let kind = match x.kind() {
            "typed_parameter" => x.named_child(0).map_or("typed_parameter", |c| match c.kind() {
                k @ ("list_splat_pattern" | "dictionary_splat_pattern") => k,
                _ => "typed_parameter",
            }),
            k => k,
        };
        match kind {
            "positional_separator" => p.posonly = p.positional.len(),
            "keyword_separator" => after_star = true,
            "list_splat_pattern" => {
                p.vararg = true;
                after_star = true;
            }
            "dictionary_splat_pattern" => p.kwarg = true,
            "identifier" | "typed_parameter" => {
                if after_star {
                    p.kwonly.push(name(x));
                } else {
                    p.positional.push(name(x));
                }
            }
            "default_parameter" | "typed_default_parameter" => {
                if after_star {
                    p.kwonly.push(name(x));
                } else {
                    p.positional.push(name(x));
                    p.defaults += 1;
                }
            }
            _ => {}
        }
    }
    p
}

fn is_static(m: &Method<'_>) -> bool {
    m.decorators.iter().any(|d| leaf(d) == "staticmethod")
}

// ---------------------------------------------------------------- rules

fn module_table<'t>(n: Node<'t>, ctx: &NativeContext<'t>) -> Table<'t> {
    Table::build(n, ctx.source)
}

fn inconsistent_mro<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    for (i, c) in t.classes.iter().enumerate() {
        if c.bases.len() < 2 || t.mro(i, &mut Vec::new()).is_some() {
            continue;
        }
        out.push(at(c.name_node, format!("Cannot create a consistent method resolution order (MRO) for bases {}", c.bases.join(", "))));
    }
}

fn bad_super_call<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for c in &t.classes {
        let mut stack = vec![c.node];
        while let Some(x) = stack.pop() {
            for k in named_children(x) {
                if k.kind() == "class_definition" {
                    continue;
                }
                stack.push(k);
                if k.kind() != "call" || !k.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "super") {
                    continue;
                }
                let a = call_args(k, src);
                let [first, second] = a.positional.as_slice() else { continue };
                if first.kind() != "identifier" || second.kind() != "identifier" {
                    continue;
                }
                let arg = text(*first, src);
                if arg != c.name && t.base_index(arg).is_some() {
                    out.push(NativeFinding {
                        start: first.start_byte(),
                        end: first.end_byte(),
                        message: format!("Bad first argument `{arg}` given to `super()`: this method belongs to `{}`", c.name),
                        edits: vec![codelaw_core::TextEdit::new(first.start_byte(), first.end_byte(), c.name)],
                    });
                }
            }
        }
    }
}

fn init_body<'a, 't>(c: &'a Class<'t>) -> Option<&'a Method<'t>> {
    c.methods.iter().rev().find(|m| m.name == "__init__")
}

fn calls_in<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for k in named_children(n) {
        if k.kind() == "call" {
            out.push(k);
        }
        if !matches!(k.kind(), "function_definition" | "class_definition" | "lambda") {
            calls_in(k, out);
        }
    }
}

fn non_parent_init_called<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for (i, c) in t.classes.iter().enumerate() {
        let Some(init) = init_body(c) else { continue };
        let (anc, closed) = t.ancestry(i);
        if !closed {
            continue;
        }
        let mut calls = Vec::new();
        if let Some(b) = init.node.child_by_field_name("body") {
            calls_in(b, &mut calls);
        }
        for k in calls {
            let Some(f) = k.child_by_field_name("function").filter(|f| f.kind() == "attribute") else { continue };
            let (Some(o), Some(a)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { continue };
            if o.kind() != "identifier" || text(a, src) != "__init__" {
                continue;
            }
            let target = text(o, src);
            let Some(j) = t.base_index(target) else { continue };
            if j != i && !anc.contains(&j) {
                out.push(at(k, format!("`__init__` method from a non direct base class `{target}` is called")));
            }
        }
    }
}

fn super_init_not_called<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for (i, c) in t.classes.iter().enumerate() {
        let Some(init) = init_body(c) else { continue };
        let (anc, _) = t.ancestry(i);
        let Some(&base) = anc.iter().find(|&&j| t.method(j, "__init__").is_some()) else { continue };
        let mut calls = Vec::new();
        if let Some(b) = init.node.child_by_field_name("body") {
            calls_in(b, &mut calls);
        }
        let delegates = calls.iter().any(|k| {
            k.child_by_field_name("function").is_some_and(|f| {
                let ft = text(f, src);
                ft == "super" || ft.ends_with(".__init__") || ft.ends_with(".__init__")
            })
        });
        if !delegates && let Some(nm) = init.node.child_by_field_name("name") {
            out.push(at(nm, format!("`__init__` method from base class `{}` is not called", t.classes[base].name)));
        }
    }
}

fn subclassed_final_class<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    for c in &t.classes {
        for b in &c.bases {
            if let Some(j) = t.base_index(b)
                && t.has_decorator(&t.classes[j].decorators, "final")
            {
                out.push(at(c.name_node, format!("Class `{}` is a subclass of a class decorated with `typing.final`: `{b}`", c.name)));
            }
        }
    }
}

fn overridden_final_method<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    for (i, c) in t.classes.iter().enumerate() {
        let (anc, _) = t.ancestry(i);
        for m in &c.methods {
            for &j in &anc {
                if let Some(bm) = t.method(j, m.name) {
                    if t.has_decorator(&bm.decorators, "final")
                        && let Some(nm) = m.node.child_by_field_name("name")
                    {
                        out.push(at(nm, format!("Method `{}` overrides a final method of class `{}`", m.name, t.classes[j].name)));
                    }
                    break;
                }
            }
        }
    }
}

fn members(t: &Table<'_>, i: usize) -> bool {
    t.classes[i].body.iter().any(|s| {
        s.kind() == "expression_statement"
            && s.named_child(0).is_some_and(|a| {
                a.kind() == "assignment" && a.child_by_field_name("left").is_some_and(|l| l.kind() == "identifier" && !text(l, t.src).starts_with('_'))
            })
    })
}

fn invalid_enum_extension<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    for c in &t.classes {
        for b in &c.bases {
            if let Some(j) = t.base_index(b)
                && t.is_enum(j)
                && members(&t, j)
            {
                out.push(at(c.name_node, format!("Enum class `{b}` defines members and cannot be extended")));
            }
        }
    }
}

/// The abstract methods class `i` still has: each name's first definition
/// along the ancestry decides.
fn abstract_methods(t: &Table<'_>, i: usize) -> Vec<(String, usize)> {
    let (anc, _) = t.ancestry(i);
    let mut decided: BTreeSet<&str> = BTreeSet::new();
    let mut out = Vec::new();
    for j in std::iter::once(i).chain(anc) {
        for m in &t.classes[j].methods {
            if !decided.insert(m.name) {
                continue;
            }
            if t.has_decorator(&m.decorators, "abstractmethod") || t.has_decorator(&m.decorators, "abstractproperty") {
                out.push((m.name.to_owned(), j));
            }
        }
    }
    out
}

fn abstract_class_instantiated<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    let abstract_names: BTreeMap<&str, Vec<(String, usize)>> = t
        .classes
        .iter()
        .enumerate()
        .filter(|(i, _)| t.is_abc(*i) && t.ancestry(*i).1)
        .filter_map(|(i, c)| {
            let a = abstract_methods(&t, i);
            (!a.is_empty()).then_some((c.name, a))
        })
        .collect();
    if abstract_names.is_empty() {
        return;
    }
    let mut stack = vec![n];
    while let Some(x) = stack.pop() {
        for k in named_children(x) {
            stack.push(k);
            if k.kind() != "call" {
                continue;
            }
            let Some(f) = k.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { continue };
            let name = text(f, src);
            if abstract_names.contains_key(name) && t.base_index(name).is_some() {
                out.push(at(k, format!("Abstract class `{name}` with abstract methods instantiated")));
            }
        }
    }
}

fn abstract_method<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    for (i, c) in t.classes.iter().enumerate() {
        let (anc, closed) = t.ancestry(i);
        // A class that declares itself abstract may leave methods abstract.
        let declares = c.bases.iter().any(|b| leaf(b) == "ABC" || leaf(b) == "Protocol") || c.metaclass.is_some_and(|m| leaf(m) == "ABCMeta");
        if anc.is_empty() || !closed || declares || !t.is_abc(i) || c.methods.iter().any(|m| t.has_decorator(&m.decorators, "abstractmethod")) {
            continue;
        }
        for (name, owner) in abstract_methods(&t, i) {
            out.push(at(c.name_node, format!("Method `{name}` is abstract in class `{}` but is not overridden in child class `{}`", t.classes[owner].name, c.name)));
        }
    }
}

fn method_hidden<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for c in &t.classes {
        // `self.name = …` in any method of the class.
        let mut assigned: BTreeMap<&str, usize> = BTreeMap::new();
        for m in &c.methods {
            let selves = params(m.node, src).positional.first().copied();
            let Some(selfname) = selves else { continue };
            let mut stack = vec![m.node];
            while let Some(x) = stack.pop() {
                for k in named_children(x) {
                    if matches!(k.kind(), "class_definition" | "lambda") {
                        continue;
                    }
                    stack.push(k);
                    if k.kind() == "assignment"
                        && let Some(l) = k.child_by_field_name("left")
                        && l.kind() == "attribute"
                        && l.child_by_field_name("object").is_some_and(|o| text(o, src) == selfname)
                        && let Some(a) = l.child_by_field_name("attribute")
                    {
                        assigned.entry(text(a, src)).or_insert(src[..k.start_byte()].matches('\n').count() + 1);
                    }
                }
            }
        }
        for m in &c.methods {
            if m.decorators.iter().any(|d| matches!(leaf(d), "property" | "setter" | "getter" | "deleter" | "cached_property" | "overload")) {
                continue;
            }
            if let (Some(line), Some(nm)) = (assigned.get(m.name), m.node.child_by_field_name("name")) {
                out.push(at(nm, format!("An attribute defined in line {line} hides method `{}`", m.name)));
            }
        }
    }
}

fn kind_of(m: &Method<'_>) -> (&'static str, bool) {
    let prop = m.decorators.iter().any(|d| matches!(leaf(d), "property" | "cached_property"));
    (if prop { "property" } else { "method" }, m.is_async)
}

fn invalid_overridden_method<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    for (i, c) in t.classes.iter().enumerate() {
        let (anc, closed) = t.ancestry(i);
        if !closed {
            continue;
        }
        for m in &c.methods {
            if m.name.starts_with("__") && m.name.ends_with("__") {
                continue;
            }
            let Some(base) = anc.iter().find_map(|&j| t.method(j, m.name)) else { continue };
            let (bk, ba) = kind_of(base);
            let (ok, oa) = kind_of(m);
            let Some(nm) = m.node.child_by_field_name("name") else { continue };
            if bk != ok {
                out.push(at(nm, format!("Method `{}` was expected to be `{bk}`, found it instead as `{ok}`", m.name)));
            } else if bk == "method" && ba != oa {
                let (want, got) = if ba { ("async", "non-async") } else { ("non-async", "async") };
                out.push(at(nm, format!("Method `{}` was expected to be `{want}`, found it instead as `{got}`", m.name)));
            }
        }
    }
}

const EXEMPT: &[&str] = &["__init__", "__new__", "__init_subclass__", "__post_init__", "__class_getitem__", "__call__"];

/// What an override check reports: the parameter counts, the names, or the
/// defaults.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Differ,
    Renamed,
    Signature,
}

/// For each method that overrides an in-file ancestor's: what differs.
fn overrides<'t>(t: &Table<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>, mode: Mode) {
    let src = ctx.source;
    for (i, c) in t.classes.iter().enumerate() {
        let (anc, closed) = t.ancestry(i);
        if !closed {
            continue;
        }
        for m in &c.methods {
            if EXEMPT.contains(&m.name) || is_static(m) || m.decorators.iter().any(|d| matches!(leaf(d), "property" | "setter" | "overload" | "wraps")) {
                continue;
            }
            let Some((bj, base)) = anc.iter().find_map(|&j| t.method(j, m.name).map(|b| (j, b))) else { continue };
            if is_static(base) || base.decorators.iter().any(|d| matches!(leaf(d), "property" | "overload")) {
                continue;
            }
            let (bp, op) = (params(base.node, src), params(m.node, src));
            if bp.vararg || op.vararg {
                continue;
            }
            let bpos: Vec<&str> = bp.positional.iter().skip(1).copied().collect();
            let opos: Vec<&str> = op.positional.iter().skip(1).copied().collect();
            let Some(nm) = m.node.child_by_field_name("name") else { continue };
            let base_name = format!("{}.{}", t.classes[bj].name, m.name);
            let own_name = format!("{}.{}", c.name, m.name);
            let required = |p: &Params<'_>| p.positional.len().saturating_sub(1) - p.defaults.min(p.positional.len().saturating_sub(1));
            let differs = opos.len() < bpos.len()
                || required(&op) > bpos.len()
                || (opos.len() > bpos.len() && !op.kwarg && required(&op) > required(&bp))
                || bp.kwonly.iter().any(|k| !op.kwonly.contains(k) && !op.kwarg);
            if differs {
                if mode == Mode::Differ {
                    out.push(at(
                        nm,
                        format!(
                            "Number of parameters was {} in `{base_name}` and is now {} in overriding `{own_name}` method",
                            bpos.len(),
                            opos.len()
                        ),
                    ));
                }
                continue;
            }
            if mode == Mode::Signature {
                // The same parameters, but the override asks for one the base made optional.
                let first_default = |p: &Params<'_>| p.positional.len() - p.defaults.min(p.positional.len());
                let (bf, of) = (first_default(&bp).saturating_sub(1), first_default(&op).saturating_sub(1));
                let n = bpos.len().min(opos.len());
                if (0..n).any(|k| k >= bf && k < of) {
                    out.push(at(nm, format!("Signature of `{own_name}` differs from the overridden `{base_name}`: a parameter the base makes optional is required")));
                }
                continue;
            }
            if mode == Mode::Renamed && let Some((b, o)) = bpos.iter().zip(opos.iter()).find(|(b, o)| b != o && !b.starts_with('_') && !o.starts_with('_')) {
                out.push(at(nm, format!("Parameter `{b}` has been renamed to `{o}` in overriding `{own_name}` method")));
            }
        }
    }
}

fn arguments_differ<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    overrides(&t, ctx, out, Mode::Differ)
}

fn arguments_renamed<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    overrides(&t, ctx, out, Mode::Renamed)
}


fn signature_differs<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    overrides(&t, ctx, out, Mode::Signature)
}

// ------------------------------------------------- attributes set on `self`

#[derive(Clone, Copy, PartialEq, Eq)]
enum Role {
    Store,
    /// `self.x: int` with no value declares nothing.
    AnnOnly,
    Read,
}

/// Whether an attribute node is assigned, only annotated, or read.
fn attr_role(a: Node<'_>) -> Role {
    let mut cur = a;
    loop {
        let Some(p) = cur.parent() else { return Role::Read };
        match p.kind() {
            "assignment" => {
                if p.child_by_field_name("left").is_some_and(|l| l.id() == cur.id()) {
                    return match (p.child_by_field_name("right"), p.child_by_field_name("type")) {
                        (Some(_), _) => Role::Store,
                        (None, Some(_)) => Role::AnnOnly,
                        (None, None) => Role::Read,
                    };
                }
                return Role::Read;
            }
            "pattern_list" | "tuple_pattern" | "list_pattern" | "list_splat_pattern" => cur = p,
            "for_statement" | "for_in_clause" => {
                return if p.child_by_field_name("left").is_some_and(|l| l.id() == cur.id()) { Role::Store } else { Role::Read };
            }
            "as_pattern_target" => return Role::Store,
            _ => return Role::Read,
        }
    }
}

struct SelfAttr<'t> {
    name: &'t str,
    node: Node<'t>,
    role: Role,
}

/// Every `<selfname>.attr` inside a method, nested functions, lambdas and
/// classes left out, in source order.
fn self_attrs<'t>(method: Node<'t>, selfname: &str, src: &'t str) -> Vec<SelfAttr<'t>> {
    let mut out = Vec::new();
    let mut stack = vec![method];
    while let Some(x) = stack.pop() {
        for k in named_children(x) {
            if matches!(k.kind(), "function_definition" | "class_definition" | "lambda") {
                continue;
            }
            stack.push(k);
            if k.kind() == "attribute"
                && k.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && text(o, src) == selfname)
                && let Some(a) = k.child_by_field_name("attribute")
            {
                out.push(SelfAttr { name: text(a, src), node: k, role: attr_role(k) });
            }
        }
    }
    out.sort_by_key(|a| a.node.start_byte());
    out
}

fn line_of(src: &str, pos: usize) -> usize {
    src[..pos].matches('\n').count() + 1
}

/// The first parameter of a method that is an instance method.
fn self_name<'t>(m: &Method<'t>, src: &'t str) -> Option<&'t str> {
    if m.decorators.iter().any(|d| matches!(leaf(d), "staticmethod" | "classmethod")) {
        return None;
    }
    params(m.node, src).positional.first().copied()
}

/// Names a class body binds: methods, nested classes, assignments and
/// annotations.
fn class_level_names<'t>(c: &Class<'t>, src: &'t str) -> BTreeSet<&'t str> {
    let mut out = BTreeSet::new();
    for st in &c.body {
        let f = if st.kind() == "decorated_definition" { st.child_by_field_name("definition").unwrap_or(*st) } else { *st };
        match f.kind() {
            "function_definition" | "class_definition" => {
                if let Some(nm) = f.child_by_field_name("name") {
                    out.insert(text(nm, src));
                }
            }
            "expression_statement" => {
                for a in named_children(f) {
                    if a.kind() == "assignment"
                        && let Some(l) = a.child_by_field_name("left")
                    {
                        let mut ids = vec![l];
                        while let Some(x) = ids.pop() {
                            if x.kind() == "identifier" {
                                out.insert(text(x, src));
                            } else {
                                ids.extend(named_children(x));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn is_mixin(name: &str) -> bool {
    name.to_ascii_lowercase().ends_with("mixin")
}

/// Whether `n` sits in the body of a `try` that catches `AttributeError`
/// (or everything) before it reaches `stop`.
fn guarded_by_attribute_error(n: Node<'_>, stop: Node<'_>, src: &str) -> bool {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        if p.id() == stop.id() {
            return false;
        }
        if p.kind() == "try_statement"
            && p.child_by_field_name("body").is_some_and(|b| b.start_byte() <= n.start_byte() && n.end_byte() <= b.end_byte())
        {
            let catches = named_children(p).into_iter().filter(|c| c.kind() == "except_clause").any(|e| {
                let head = text(e, src).lines().next().unwrap_or("");
                let head = head.trim_start_matches("except").trim_start_matches('*').trim();
                head.starts_with(':') || head.contains("AttributeError") || head.starts_with("Exception") || head.starts_with("BaseException")
            });
            if catches {
                return true;
            }
        }
        cur = p;
    }
    false
}

fn access_member_before_definition<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for (i, c) in t.classes.iter().enumerate() {
        if is_mixin(c.name) {
            continue;
        }
        let (anc, closed) = t.ancestry(i);
        if !closed {
            continue;
        }
        // Names the class or an ancestor binds at class level, and the
        // attributes an ancestor's methods assign: neither is "before".
        let mut known: BTreeSet<&str> = BTreeSet::new();
        for &j in std::iter::once(&i).chain(anc.iter()) {
            known.extend(class_level_names(&t.classes[j], src));
        }
        for &j in &anc {
            for m in &t.classes[j].methods {
                if let Some(s) = self_name(m, src) {
                    known.extend(self_attrs(m.node, s, src).iter().filter(|a| a.role == Role::Store).map(|a| a.name));
                }
            }
        }
        // For each method: its accesses; and which methods assign each name.
        let per_method: Vec<(usize, &str, Vec<SelfAttr<'t>>)> = c
            .methods
            .iter()
            .enumerate()
            .filter_map(|(k, m)| self_name(m, src).map(|s| (k, s, self_attrs(m.node, s, src))))
            .collect();
        let mut assigners: BTreeMap<&str, BTreeSet<usize>> = BTreeMap::new();
        for (k, _, accs) in &per_method {
            for a in accs.iter().filter(|a| a.role == Role::Store) {
                assigners.entry(a.name).or_default().insert(*k);
            }
        }
        for (k, selfname, accs) in &per_method {
            let m = &c.methods[*k];
            // The first assignment of each name in this method.
            let mut first_store: BTreeMap<&str, usize> = BTreeMap::new();
            for a in accs.iter().filter(|a| a.role == Role::Store) {
                first_store.entry(a.name).or_insert(a.node.start_byte());
            }
            let mut reported: BTreeSet<&str> = BTreeSet::new();
            for a in accs.iter().filter(|a| a.role == Role::Read) {
                let Some(&d) = first_store.get(a.name) else { continue };
                if known.contains(a.name)
                    || a.node.start_byte() >= d
                    || line_of(src, a.node.start_byte()) >= line_of(src, d)
                    || assigners.get(a.name).is_some_and(|s| s.len() != 1)
                    || !reported.insert(a.name)
                    || guarded_by_attribute_error(a.node, m.node, src)
                {
                    continue;
                }
                out.push(at(a.node, format!("Access to member `{}` before its definition on line {}", a.name, line_of(src, d))));
            }
            if m.name != "__init__" {
                continue;
            }
            // `self.method()` in `__init__` before the attributes the method reads are assigned.
            let mut calls = Vec::new();
            if let Some(b) = m.node.child_by_field_name("body") {
                calls_in(b, &mut calls);
            }
            for call in calls {
                let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "attribute") else { continue };
                let (Some(o), Some(an)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { continue };
                if o.kind() != "identifier" || text(o, src) != *selfname {
                    continue;
                }
                let Some(cm) = c.methods.iter().rev().find(|x| x.name == text(an, src)) else { continue };
                let Some(cs) = self_name(cm, src) else { continue };
                if std::ptr::eq(cm, m) {
                    continue;
                }
                let called = self_attrs(cm.node, cs, src);
                let mut done: BTreeSet<&str> = BTreeSet::new();
                for r in called.iter().filter(|a| a.role == Role::Read) {
                    let Some(&d) = first_store.get(r.name) else { continue };
                    let assigned_first = called.iter().any(|a| a.role == Role::Store && a.name == r.name && a.node.start_byte() < r.node.start_byte());
                    if known.contains(r.name)
                        || assigned_first
                        || d <= call.start_byte()
                        || line_of(src, d) <= line_of(src, call.start_byte())
                        || assigners.get(r.name).is_some_and(|s| s.len() != 1)
                        || !done.insert(r.name)
                    {
                        continue;
                    }
                    out.push(at(f, format!("Access to member `{}` before its definition on line {}", r.name, line_of(src, d))));
                }
            }
        }
    }
}

// ------------------------------------------------- iterators and context managers

fn has_yield(n: Node<'_>) -> bool {
    named_children(n).into_iter().any(|k| match k.kind() {
        "yield" => true,
        "function_definition" | "class_definition" | "lambda" => false,
        _ => has_yield(k),
    })
}

fn returns_of<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for k in named_children(n) {
        match k.kind() {
            "return_statement" => out.push(k),
            "function_definition" | "class_definition" | "lambda" => {}
            _ => returns_of(k, out),
        }
    }
}

impl<'t> Table<'t> {
    /// Whether class `i` and every ancestor are in the file (or `object`)
    /// and none defines any of `names`.
    fn lacks_all(&self, i: usize, names: &[&str]) -> bool {
        let (anc, closed) = self.ancestry(i);
        closed && !std::iter::once(i).chain(anc).any(|j| names.iter().any(|n| self.method(j, n).is_some()))
    }
}

enum Value {
    NotIterator(String),
    Unknown,
}

fn non_iterator_returned<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for (i, c) in t.classes.iter().enumerate() {
        let Some(m) = c.methods.iter().rev().find(|m| m.name == "__iter__") else { continue };
        let Some(body) = m.node.child_by_field_name("body") else { continue };
        if m.is_async || has_yield(body) {
            continue;
        }
        let selfname = params(m.node, src).positional.first().copied().unwrap_or("self");
        let mut rets = Vec::new();
        returns_of(body, &mut rets);
        let values: Vec<Value> = rets
            .iter()
            .map(|r| {
                let Some(v) = r.named_child(0).map(crate::python_util::unparen) else { return Value::Unknown };
                match v.kind() {
                    "list" | "dictionary" | "set" | "tuple" | "integer" | "float" | "string" | "concatenated_string" | "true" | "false" | "list_comprehension" | "dictionary_comprehension" | "set_comprehension" => {
                        Value::NotIterator(v.kind().to_owned())
                    }
                    "identifier" if text(v, src) == selfname => {
                        if t.lacks_all(i, &["__next__", "next"]) { Value::NotIterator(c.name.to_owned()) } else { Value::Unknown }
                    }
                    "call" => match v.child_by_field_name("function").filter(|f| f.kind() == "identifier").and_then(|f| t.base_index(text(f, src))) {
                        Some(j) if t.classes[j].metaclass.is_none() && t.lacks_all(j, &["__next__", "next"]) => Value::NotIterator(t.classes[j].name.to_owned()),
                        _ => Value::Unknown,
                    },
                    "identifier" => match t.base_index(text(v, src)) {
                        // The class itself: an iterator only through its metaclass.
                        Some(j) if t.classes[j].metaclass.is_none() && t.ancestry(j).1 => Value::NotIterator(format!("class {}", t.classes[j].name)),
                        _ => Value::Unknown,
                    },
                    _ => Value::Unknown,
                }
            })
            .collect();
        if let Some(Value::NotIterator(what)) = values.first()
            && values.iter().all(|v| matches!(v, Value::NotIterator(_)))
            && let Some(nm) = m.node.child_by_field_name("name")
        {
            out.push(at(nm, format!("`__iter__` returns a non-iterator ({what}): it has no `__next__`")));
        }
    }
}

const NOT_CONTEXT_BUILTINS: &[&str] = &["int", "str", "float", "bool", "list", "dict", "set", "frozenset", "tuple", "slice", "range", "bytes", "complex", "object"];

fn not_async_context_manager<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    if !text(n, src).starts_with("async") {
        return;
    }
    let t = module_table(n_root(n), ctx);
    // Module-level functions decorated with `contextmanager`.
    let root = n_root(n);
    for item in named_children(n).into_iter().flat_map(|c| if c.kind() == "with_clause" { named_children(c) } else { vec![c] }) {
        if item.kind() != "with_item" {
            continue;
        }
        let Some(v) = item.child_by_field_name("value").or_else(|| item.named_child(0)) else { continue };
        let v = crate::python_util::unparen(v);
        let kind = match v.kind() {
            "integer" => Some("int".to_owned()),
            "float" => Some("float".to_owned()),
            "string" | "concatenated_string" => Some("str".to_owned()),
            "list" | "list_comprehension" => Some("list".to_owned()),
            "dictionary" | "dictionary_comprehension" => Some("dict".to_owned()),
            "set" | "set_comprehension" => Some("set".to_owned()),
            "tuple" => Some("tuple".to_owned()),
            "true" | "false" => Some("bool".to_owned()),
            "call" => v.child_by_field_name("function").filter(|f| f.kind() == "identifier").and_then(|f| {
                let name = text(f, src);
                if let Some(j) = t.base_index(name) {
                    let cls = &t.classes[j];
                    if cls.metaclass.is_none() && !is_mixin(cls.name) && t.ancestry(j).1 {
                        let (anc, _) = t.ancestry(j);
                        let has = |m: &str| std::iter::once(j).chain(anc.iter().copied()).any(|x| t.method(x, m).is_some());
                        return (!(has("__aenter__") && has("__aexit__"))).then(|| name.to_owned());
                    }
                    return None;
                }
                if let Some(def) = module_function(root, name, src) {
                    let cm = decorators(def).into_iter().any(|d| {
                        let d = decorator_target(d);
                        crate::python_util::qualified_name(d, model, src).is_some_and(|q| q == "contextlib.contextmanager")
                    });
                    return (cm && !text(def, src).starts_with("async")).then(|| "generator".to_owned());
                }
                (NOT_CONTEXT_BUILTINS.contains(&name) && crate::python_util::qualified_name(f, model, src).is_some_and(|q| q == format!("builtins.{name}"))).then(|| name.to_owned())
            }),
            _ => None,
        };
        if let Some(k) = kind {
            out.push(at(v, format!("Async context manager `{k}` does not implement `__aenter__` and `__aexit__`")));
        }
    }
}

fn n_root(mut n: Node<'_>) -> Node<'_> {
    while let Some(p) = n.parent() {
        n = p;
    }
    n
}

/// A module-level `def name`, unique in the file.
fn module_function<'t>(root: Node<'t>, name: &str, src: &str) -> Option<Node<'t>> {
    let mut found = None;
    for s in named_children(root) {
        let def = if s.kind() == "decorated_definition" { s.child_by_field_name("definition").unwrap_or(s) } else { s };
        if def.kind() == "function_definition" && def.child_by_field_name("name").is_some_and(|n| text(n, src) == name) {
            if found.is_some() {
                return None;
            }
            found = Some(def);
        }
    }
    found
}

// ------------------------------------------------- __slots__

enum Slots {
    Absent,
    Unknown,
    Names(Vec<String>),
}

fn own_slots(c: &Class<'_>, src: &str) -> Slots {
    let mut found: Option<Node<'_>> = None;
    for st in &c.body {
        if st.kind() != "expression_statement" {
            continue;
        }
        for a in named_children(*st) {
            if a.kind() == "assignment" && a.child_by_field_name("left").is_some_and(|l| l.kind() == "identifier" && text(l, src) == "__slots__") {
                if found.is_some() {
                    return Slots::Unknown;
                }
                if let Some(r) = a.child_by_field_name("right") {
                    found = Some(r);
                }
            }
        }
    }
    let Some(v) = found else { return Slots::Absent };
    let v = crate::python_util::unparen(v);
    let one = |x: Node<'_>| crate::python_util::string_value(x, src).filter(|s| !s.is_empty());
    match v.kind() {
        "string" | "concatenated_string" => one(v).map_or(Slots::Unknown, |s| Slots::Names(vec![s])),
        "tuple" | "list" | "set" => {
            let items: Vec<Option<String>> = kids(v).into_iter().map(one).collect();
            if items.iter().all(|x| x.is_some()) { Slots::Names(items.into_iter().flatten().collect()) } else { Slots::Unknown }
        }
        _ => Slots::Unknown,
    }
}

/// Class `i` and its ancestors when every one is in the file (or one of the
/// slot-less library bases), so the instance layout is known.
fn slot_chain(t: &Table<'_>, i: usize) -> Option<Vec<usize>> {
    let mut out = Vec::new();
    let mut stack = vec![i];
    while let Some(c) = stack.pop() {
        if out.contains(&c) {
            continue;
        }
        out.push(c);
        let cls = &t.classes[c];
        if cls.opaque_other || cls.metaclass.is_some() {
            return None;
        }
        for b in cls.bases.iter().chain(cls.sub_bases.iter()) {
            match t.base_index(b) {
                Some(j) => stack.push(j),
                None if matches!(leaf(b), "object" | "Generic" | "Protocol" | "ABC") => {}
                None => return None,
            }
        }
    }
    Some(out)
}

/// The slot names of a chain, and whether `__dict__` is among them; `None`
/// when a class in it has no literal `__slots__`.
fn chain_slots(t: &Table<'_>, chain: &[usize]) -> Option<(BTreeSet<String>, bool)> {
    let mut names = BTreeSet::new();
    for &j in chain {
        match own_slots(&t.classes[j], t.src) {
            Slots::Names(v) => names.extend(v),
            _ => return None,
        }
    }
    let dict = names.contains("__dict__");
    Some((names, dict))
}

fn is_classvar(ann: Node<'_>, src: &str) -> bool {
    let a = if ann.kind() == "type" { ann.named_child(0).unwrap_or(ann) } else { ann };
    let base = match a.kind() {
        "generic_type" | "subscript" => a.named_child(0).unwrap_or(a),
        _ => a,
    };
    leaf(text(base, src)) == "ClassVar"
}

fn declare_non_slot<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for (i, c) in t.classes.iter().enumerate() {
        let Slots::Names(own) = own_slots(c, src) else { continue };
        if own.is_empty() {
            continue;
        }
        let Some(chain) = slot_chain(&t, i) else { continue };
        let Some((names, dict)) = chain_slots(&t, &chain) else { continue };
        if dict {
            continue;
        }
        for st in &c.body {
            if st.kind() != "expression_statement" {
                continue;
            }
            for a in named_children(*st) {
                if a.kind() != "assignment" || a.child_by_field_name("right").is_some() {
                    continue;
                }
                let (Some(l), Some(ann)) = (a.child_by_field_name("left"), a.child_by_field_name("type")) else { continue };
                if l.kind() != "identifier" || is_classvar(ann, src) {
                    continue;
                }
                let nm = text(l, src);
                if nm != "__slots__" && !names.contains(nm) {
                    out.push(at(l, format!("No such name `{nm}` in `__slots__`")));
                }
            }
        }
    }
}

// ------------------------------------------------- Flag members

fn int_literal(n: Node<'_>, src: &str) -> Option<u64> {
    if n.kind() != "integer" {
        return None;
    }
    let s = text(n, src).replace('_', "").to_ascii_lowercase();
    if let Some(h) = s.strip_prefix("0x") {
        u64::from_str_radix(h, 16).ok()
    } else if let Some(o) = s.strip_prefix("0o") {
        u64::from_str_radix(o, 8).ok()
    } else if let Some(b) = s.strip_prefix("0b") {
        u64::from_str_radix(b, 2).ok()
    } else {
        s.parse().ok()
    }
}

fn implicit_flag_alias<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    let src = ctx.source;
    for c in &t.classes {
        if !c.bases.iter().any(|b| matches!(*b, "Flag" | "IntFlag" | "enum.Flag" | "enum.IntFlag")) {
            continue;
        }
        // The members that are an integer literal, in order.
        let mut members: Vec<(&str, u64, Node<'_>)> = Vec::new();
        for st in &c.body {
            if st.kind() != "expression_statement" {
                continue;
            }
            for a in named_children(*st) {
                if a.kind() == "assignment"
                    && let (Some(l), Some(r)) = (a.child_by_field_name("left"), a.child_by_field_name("right"))
                    && l.kind() == "identifier"
                    && let Some(v) = int_literal(r, src)
                    && v != 0
                {
                    members.push((text(l, src), v, l));
                }
            }
        }
        // Single-bit values are the base; a multi-bit value joins it only
        // when it shares no bit with the base.
        let mut base: Vec<(&str, u64)> = Vec::new();
        for (name, v, _) in &members {
            if v.is_power_of_two() && !base.iter().any(|b| b.1 == *v) {
                base.push((name, *v));
            }
        }
        for (name, v, node) in &members {
            if v.is_power_of_two() || base.iter().any(|b| b.1 == *v) {
                continue;
            }
            let shared: Vec<&(&str, u64)> = base.iter().filter(|b| b.1 & v != 0).collect();
            if shared.is_empty() {
                base.push((name, *v));
                continue;
            }
            let list: Vec<String> = shared.iter().map(|b| format!("`{}`", b.0)).collect();
            out.push(at(*node, format!("Flag member `{name}` shares bit positions with {}", list.join(", "))));
        }
    }
}

// ------------------------------------------------- match class patterns

const SELF_MATCHING: &[&str] = &["bool", "bytearray", "bytes", "dict", "float", "frozenset", "int", "list", "set", "str", "tuple"];

/// The `__match_args__` of class `i`: its own tuple of strings, else the
/// nearest ancestor's, else a plain dataclass's fields. `None` when it
/// cannot be known.
fn match_args(t: &Table<'_>, i: usize, depth: usize) -> Option<Vec<String>> {
    let src = t.src;
    let c = &t.classes[i];
    if depth > 8 || c.opaque_base || c.metaclass.is_some() {
        return None;
    }
    for st in &c.body {
        if st.kind() != "expression_statement" {
            continue;
        }
        for a in named_children(*st) {
            if a.kind() == "assignment" && a.child_by_field_name("left").is_some_and(|l| l.kind() == "identifier" && text(l, src) == "__match_args__") {
                let v = crate::python_util::unparen(a.child_by_field_name("right")?);
                if v.kind() != "tuple" {
                    return None;
                }
                return kids(v).into_iter().map(|x| crate::python_util::string_value(x, src)).collect();
            }
        }
    }
    let plain_dataclass = c.decorators.iter().any(|d| leaf(d) == "dataclass");
    if plain_dataclass {
        if !c.bases.iter().all(|b| leaf(b) == "object") {
            return None;
        }
        let mut fields = Vec::new();
        for st in &c.body {
            if st.kind() != "expression_statement" {
                continue;
            }
            for a in named_children(*st) {
                let (Some(l), Some(ann)) = (a.child_by_field_name("left"), a.child_by_field_name("type")) else { continue };
                if a.kind() != "assignment" || l.kind() != "identifier" {
                    continue;
                }
                let at_ = text(ann, src);
                if at_.contains("KW_ONLY") {
                    break;
                }
                if is_classvar(ann, src) {
                    continue;
                }
                if let Some(r) = a.child_by_field_name("right") {
                    let rt = text(r, src);
                    if rt.starts_with("field(") && (rt.contains("kw_only=True") || rt.contains("init=False")) {
                        continue;
                    }
                }
                fields.push(text(l, src).to_owned());
            }
        }
        return Some(fields);
    }
    // An ancestor's, nearest first; an unknown base before it ends the search.
    for b in &c.bases {
        if SELF_MATCHING.contains(&leaf(b)) || matches!(leaf(b), "NamedTuple" | "tuple") {
            return None;
        }
        if let Some(j) = t.base_index(b) {
            return match_args(t, j, depth + 1);
        }
        if leaf(b) != "object" {
            return None;
        }
    }
    None
}

fn class_patterns<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for k in named_children(n) {
        if k.kind() == "class_pattern" {
            out.push(k);
        }
        class_patterns(k, out);
    }
}

fn pattern_parts<'t>(p: Node<'t>) -> (Option<Node<'t>>, Vec<Node<'t>>) {
    let ks = kids(p);
    let name = ks.first().copied().filter(|n| n.kind() == "dotted_name");
    // A keyword pattern (`y=2`) is a `case_pattern` wrapping a `keyword_pattern`.
    let positional = ks.iter().skip(1).copied().filter(|n| n.kind() == "case_pattern" && !kids(*n).iter().any(|c| c.kind() == "keyword_pattern")).collect();
    (name, positional)
}

fn too_many_positional_sub_patterns<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    if !ctx.source.contains("case ") {
        return;
    }
    let t = module_table(n, ctx);
    let src = ctx.source;
    let mut pats = Vec::new();
    class_patterns(n, &mut pats);
    for p in pats {
        let (Some(name_node), positional) = pattern_parts(p) else { continue };
        let name = text(name_node, src);
        let expected = if SELF_MATCHING.contains(&name) && t.base_index(name).is_none() {
            1
        } else if let Some(j) = t.base_index(name) {
            match match_args(&t, j, 0) {
                Some(a) => a.len(),
                None => continue,
            }
        } else {
            continue;
        };
        if positional.len() > expected {
            out.push(at(p, format!("`{name}` expects {expected} positional sub-patterns (given {})", positional.len())));
        }
    }
}

fn match_class_positional_attributes<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    if !ctx.source.contains("case ") {
        return;
    }
    let t = module_table(n, ctx);
    let src = ctx.source;
    let mut pats = Vec::new();
    class_patterns(n, &mut pats);
    for p in pats {
        let (Some(name_node), positional) = pattern_parts(p) else { continue };
        let name = text(name_node, src);
        let Some(j) = t.base_index(name) else { continue };
        let Some(args) = match_args(&t, j, 0) else { continue };
        if positional.is_empty() || positional.len() > args.len() {
            continue;
        }
        let names: Vec<String> = args.iter().take(positional.len()).map(|a| format!("`{a}`")).collect();
        let edits: Vec<TextEdit> = positional
            .iter()
            .zip(args.iter())
            .map(|(pat, a)| TextEdit::new(pat.start_byte(), pat.start_byte(), format!("{a}=")))
            .collect();
        out.push(NativeFinding {
            start: p.start_byte(),
            end: p.end_byte(),
            message: format!("Use keyword attributes instead of positional ones ({})", names.join(", ")),
            edits,
        });
    }
}

py_rule!(SignatureDiffers, "py/signature-differs", ["module"], false, signature_differs);
py_rule!(AccessMemberBeforeDefinition, "py/access-member-before-definition", ["module"], false, access_member_before_definition);
py_rule!(NonIteratorReturned, "py/non-iterator-returned", ["module"], false, non_iterator_returned);
py_rule!(NotAsyncContextManager, "py/not-async-context-manager", ["with_statement"], true, not_async_context_manager);
py_rule!(DeclareNonSlot, "py/declare-non-slot", ["module"], false, declare_non_slot);
py_rule!(ImplicitFlagAlias, "py/implicit-flag-alias", ["module"], false, implicit_flag_alias);
py_rule!(TooManyPositionalSubPatterns, "py/too-many-positional-sub-patterns", ["module"], false, too_many_positional_sub_patterns);
py_rule!(MatchClassPositionalAttributes, "py/match-class-positional-attributes", ["module"], false, match_class_positional_attributes);
py_rule!(InconsistentMro, "py/inconsistent-mro", ["module"], false, inconsistent_mro);
py_rule!(BadSuperCall, "py/bad-super-call", ["module"], false, bad_super_call);
py_rule!(NonParentInitCalled, "py/non-parent-init-called", ["module"], false, non_parent_init_called);
py_rule!(SuperInitNotCalled, "py/super-init-not-called", ["module"], false, super_init_not_called);
py_rule!(SubclassedFinalClass, "py/subclassed-final-class", ["module"], false, subclassed_final_class);
py_rule!(OverriddenFinalMethod, "py/overridden-final-method", ["module"], false, overridden_final_method);
py_rule!(InvalidEnumExtension, "py/invalid-enum-extension", ["module"], false, invalid_enum_extension);
py_rule!(AbstractClassInstantiated, "py/abstract-class-instantiated", ["module"], false, abstract_class_instantiated);
py_rule!(AbstractMethod, "py/abstract-method", ["module"], false, abstract_method);
py_rule!(MethodHidden, "py/method-hidden", ["module"], false, method_hidden);
py_rule!(InvalidOverriddenMethod, "py/invalid-overridden-method", ["module"], false, invalid_overridden_method);
py_rule!(ArgumentsDiffer, "py/arguments-differ", ["module"], false, arguments_differ);
py_rule!(ArgumentsRenamed, "py/arguments-renamed", ["module"], false, arguments_renamed);

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &SignatureDiffers,
    &AccessMemberBeforeDefinition,
    &NonIteratorReturned,
    &NotAsyncContextManager,
    &DeclareNonSlot,
    &ImplicitFlagAlias,
    &TooManyPositionalSubPatterns,
    &MatchClassPositionalAttributes,
    &InconsistentMro,
    &BadSuperCall,
    &NonParentInitCalled,
    &SuperInitNotCalled,
    &SubclassedFinalClass,
    &OverriddenFinalMethod,
    &InvalidEnumExtension,
    &AbstractClassInstantiated,
    &AbstractMethod,
    &MethodHidden,
    &InvalidOverriddenMethod,
    &ArgumentsDiffer,
    &ArgumentsRenamed,
];

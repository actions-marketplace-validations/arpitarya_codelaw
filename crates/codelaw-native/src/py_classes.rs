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
                        _ => c.opaque_base = true,
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

/// For each method that overrides an in-file ancestor's: what differs.
fn overrides<'t>(
    t: &Table<'t>,
    ctx: &NativeContext<'t>,
    out: &mut Vec<NativeFinding>,
    renamed: bool,
) {
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
                if !renamed {
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
            if renamed && let Some((b, o)) = bpos.iter().zip(opos.iter()).find(|(b, o)| b != o && !b.starts_with('_') && !o.starts_with('_')) {
                out.push(at(nm, format!("Parameter `{b}` has been renamed to `{o}` in overriding `{own_name}` method")));
            }
        }
    }
}

fn arguments_differ<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    overrides(&t, ctx, out, false)
}

fn arguments_renamed<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let t = module_table(n, ctx);
    overrides(&t, ctx, out, true)
}

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

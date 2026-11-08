//! W-174, the scope-facts group of SR-PY-INFERENCE: Python rows that ask a
//! question of the per-file semantic model beyond *which binding does this
//! name resolve to* — whether an enclosing function binds a `nonlocal`
//! name, which class-scope binding a read in the class body picks up,
//! whether a plain string names variables in scope, whether a local with a
//! leading underscore is read, and which private class members are read
//! inside their class. Upstream behaviour: Ruff 0.16.9 (`PLE0117`, `A003`,
//! `RUF027`, `RUF052`) and Pylint 4.1.3 (`unused-private-member`),
//! behaviour only, written fresh.

use crate::python_util::{call_args, decorators, hit, kids, py_rule, unparen};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding};
use codelaw_core::TextEdit;
use codelaw_semantic::{PYTHON_BUILTINS, ScopeKind, SemanticModel, SymbolKind};
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

const KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except",
    "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try",
    "while", "with", "yield",
];

fn line_of(src: &str, offset: usize) -> usize {
    src[..offset.min(src.len())].bytes().filter(|b| *b == b'\n').count() + 1
}

// ------------------------------------------------------------ PLE0117

/// A `del name` in `func`'s own body (not a nested definition's): Ruff counts
/// it as a binding of the name.
fn deletes(func: Node<'_>, name: &str, src: &str) -> bool {
    fn walk(n: Node<'_>, name: &str, src: &str) -> bool {
        for c in named_children(n) {
            match c.kind() {
                "function_definition" | "class_definition" | "lambda" => continue,
                "delete_statement"
                    if children(c).iter().any(|t| matches!(t.kind(), "identifier") && text(*t, src) == name)
                        || named_children(c).iter().flat_map(|e| named_children(*e)).any(|t| t.kind() == "identifier" && text(t, src) == name) =>
                {
                    return true;
                }
                _ => {}
            }
            if walk(c, name, src) {
                return true;
            }
        }
        false
    }
    func.child_by_field_name("body").is_some_and(|b| walk(b, name, src))
}

fn nonlocal_without_binding(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let scope = model.scope_at(n.start_byte());
    if model.scopes[scope].kind != ScopeKind::Function {
        return;
    }
    for id in named_children(n).into_iter().filter(|c| c.kind() == "identifier") {
        let name = text(id, src);
        if model.nonlocal_bound(scope, name) {
            continue;
        }
        // `nonlocal __class__` in a method reads the compiler's cell.
        let mut in_class = false;
        let mut del_bound = false;
        let mut cur = n.parent();
        let mut first = true;
        while let Some(p) = cur {
            match p.kind() {
                "class_definition" => in_class = true,
                "function_definition" => {
                    if !first && deletes(p, name, src) {
                        del_bound = true;
                    }
                    first = false;
                }
                _ => {}
            }
            cur = p.parent();
        }
        if del_bound || (name == "__class__" && in_class) {
            continue;
        }
        out.push(hit(id, format!("Nonlocal name `{name}` found without binding")));
    }
}

py_rule!(NonlocalWithoutBinding, "py/nonlocal-without-binding", ["nonlocal_statement"], true, nonlocal_without_binding);

// ------------------------------------------------------------ A003

fn has_future_annotations(model: &SemanticModel) -> bool {
    model.imports.iter().any(|i| i.source == "__future__" && matches!(&i.imported, codelaw_semantic::Imported::Named(n) if n == "annotations"))
}

fn overridden(def: Node<'_>, src: &str) -> bool {
    decorators(def).iter().any(|d| {
        let t = text(crate::python_util::decorator_target(*d), src);
        t == "override" || t.ends_with(".override")
    })
}

fn builtin_attribute_shadowing(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let Some(class_scope) = model.scope_starting_at(ScopeKind::Class, n.start_byte()) else { return };
    let future = has_future_annotations(model);
    for (name, ids) in &model.scopes[class_scope].bindings {
        if !PYTHON_BUILTINS.contains(&name.as_str()) {
            continue;
        }
        for &id in ids {
            let sym = model.symbol(id);
            let kind = match sym.kind {
                SymbolKind::Variable => "class attribute",
                SymbolKind::Function => "method",
                _ => continue,
            };
            let first = sym.decls.iter().copied().min().unwrap_or((0, 0));
            if sym.kind == SymbolKind::Function
                && let Some(def) = n.descendant_for_byte_range(first.0, first.1).and_then(|d| d.parent())
                && def.kind() == "function_definition"
                && overridden(def, src)
            {
                continue;
            }
            let line = line_of(src, first.0);
            for &r in &sym.refs {
                let r = &model.references[r];
                if !r.read || r.start < first.1 {
                    continue;
                }
                // Reads in the class body, its decorators, defaults and
                // annotations; a method body sees through the class.
                let scope = &model.scopes[r.scope];
                let direct = r.scope == class_scope || (scope.kind == ScopeKind::TypeParams && scope.parent == Some(class_scope));
                if !direct {
                    continue;
                }
                if future && in_annotation(n, r.start) {
                    continue;
                }
                out.push(NativeFinding {
                    start: r.start,
                    end: r.end,
                    message: format!("Python builtin is shadowed by {kind} `{name}` from line {line}"),
                    edits: Vec::new(),
                });
            }
        }
    }
}

/// Whether the identifier at `at` sits in a type annotation.
fn in_annotation(root: Node<'_>, at: usize) -> bool {
    let mut cur = root.descendant_for_byte_range(at, at + 1);
    while let Some(c) = cur {
        if c.kind() == "type" {
            return true;
        }
        if matches!(c.kind(), "block" | "class_definition") {
            return false;
        }
        cur = c.parent();
    }
    false
}

py_rule!(BuiltinAttributeShadowing, "py/builtin-attribute-shadowing", ["class_definition"], true, builtin_attribute_shadowing);

// ------------------------------------------------------------ RUF027

fn is_gettext(call: Node<'_>, src: &str) -> bool {
    let Some(f) = call.child_by_field_name("function") else { return false };
    match f.kind() {
        "identifier" => matches!(text(f, src), "gettext" | "ngettext" | "_"),
        "attribute" => f.child_by_field_name("attribute").is_some_and(|a| matches!(text(a, src), "gettext" | "ngettext")),
        _ => false,
    }
}

const LOG_METHODS: &[&str] = &["debug", "info", "warn", "warning", "error", "critical", "log", "fatal", "exception"];

/// `logging.info(…)`, `logger.warning(…)`, `self.log.error(…)`: the logging
/// module formats its message lazily, so an f-string is not wanted there.
fn is_logger_call(call: Node<'_>, src: &str) -> bool {
    let Some(f) = call.child_by_field_name("function") else { return false };
    if f.kind() != "attribute" {
        return false;
    }
    let (Some(obj), Some(attr)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { return false };
    if !LOG_METHODS.contains(&text(attr, src).to_ascii_lowercase().as_str()) {
        return false;
    }
    let leaf = match obj.kind() {
        "identifier" => text(obj, src),
        "attribute" => obj.child_by_field_name("attribute").map_or("", |a| text(a, src)),
        _ => return false,
    };
    let leaf = leaf.trim_start_matches('_').to_ascii_lowercase();
    leaf == "logging" || leaf == "log" || leaf.ends_with("logger") || leaf.ends_with("_log")
}

/// `{name}` placeholders of a plain string whose text, read as an f-string,
/// parses: the names of the simple interpolations, or `None` when the text
/// is not an f-string or an expression is beyond a pre-3.12 f-string.
fn placeholders(literal: &str) -> Option<Vec<String>> {
    if literal.bytes().filter(|b| matches!(b, b'{' | b'}')).count() < 2 {
        return None;
    }
    let source = format!("f{literal}\n");
    let root = codelaw_lang::parse(codelaw_core::Language::Python, &source);
    let top = root.root().get_inner_node();
    if top.has_error() {
        return None;
    }
    let string = top.named_child(0).and_then(|s| s.named_child(0))?;
    if string.kind() != "string" {
        return None;
    }
    let mut names = Vec::new();
    for interp in named_children(string).into_iter().filter(|c| c.kind() == "interpolation") {
        let expr = interp.child_by_field_name("expression")?;
        let t = &source[expr.byte_range()];
        if t.contains('\\') || t.contains('"') || t.contains('\'') {
            return None;
        }
        if expr.kind() == "identifier" {
            names.push(t.to_owned());
        }
        if let Some(spec) = named_children(interp).into_iter().find(|c| c.kind() == "format_specifier")
            && spec.has_error()
        {
            return None;
        }
    }
    Some(names)
}

fn missing_fstring(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let prefix: String = text(n, src).chars().take_while(|c| c.is_ascii_alphabetic()).collect::<String>().to_ascii_lowercase();
    if prefix.contains(['f', 't', 'b']) {
        return;
    }
    // A bare string statement (a docstring) is not formatted.
    let whole = n.parent().filter(|p| p.kind() == "concatenated_string").unwrap_or(n);
    if whole.parent().is_some_and(|p| p.kind() == "expression_statement") {
        return;
    }
    let mut arg_names: BTreeSet<String> = BTreeSet::new();
    let mut cur = n.parent();
    while let Some(p) = cur {
        match p.kind() {
            // Annotations are type definitions.
            "type" => return,
            "call" => {
                if is_gettext(p, src) || is_logger_call(p, src) {
                    return;
                }
                // `"…".format(…)`: a method called on the literal itself.
                if let Some(f) = p.child_by_field_name("function")
                    && f.kind() == "attribute"
                    && f.child_by_field_name("object").is_some_and(|o| o.id() == whole.id() || o.id() == n.id())
                {
                    return;
                }
                let a = call_args(p, src);
                for (k, _, _) in &a.keywords {
                    arg_names.insert(k.clone());
                }
                for v in &a.positional {
                    if v.kind() == "identifier" {
                        arg_names.insert(text(*v, src).to_owned());
                    }
                }
            }
            _ => {}
        }
        cur = p.parent();
    }
    let Some(names) = placeholders(text(n, src)) else { return };
    if names.is_empty() {
        return;
    }
    let scope = model.scope_at(n.start_byte());
    for name in &names {
        if arg_names.contains(name.as_str()) {
            return;
        }
        match model.lookup(scope, name, n.start_byte()) {
            Some(_) => {}
            None => return,
        }
    }
    out.push(NativeFinding {
        start: n.start_byte(),
        end: n.end_byte(),
        message: "Possible f-string without an `f` prefix".to_owned(),
        edits: vec![TextEdit::new(n.start_byte(), n.start_byte(), "f")],
    });
}

py_rule!(MissingFStringSyntax, "py/missing-f-string-syntax", ["string"], true, missing_fstring);

// ------------------------------------------------------------ RUF052

/// Ruff's default `dummy-variable-rgx`, `^(_+|(_+[a-zA-Z0-9_]*[a-zA-Z0-9]+?))$`,
/// without the `_`, `__` and dunder names the rule leaves alone.
fn is_dummy_name(name: &str) -> bool {
    if !name.starts_with('_') || name.chars().all(|c| c == '_') {
        return false;
    }
    if name.starts_with("__") && name.ends_with("__") {
        return false;
    }
    name.chars().last().is_some_and(|c| c.is_ascii_alphanumeric())
}

/// The call that makes an assignment a declaration of a type or a class
/// (`T = TypeVar("T")`, `Point = namedtuple(…)`, `C = type("C", …)`):
/// Ruff binds those as classes, not variables.
fn declares_a_type(value: Node<'_>, src: &str) -> bool {
    let v = unparen(value);
    if v.kind() != "call" {
        return false;
    }
    let Some(f) = v.child_by_field_name("function") else { return false };
    let leaf = match f.kind() {
        "identifier" => text(f, src),
        "attribute" => f.child_by_field_name("attribute").map_or("", |a| text(a, src)),
        _ => return false,
    };
    match leaf {
        "TypeVar" | "ParamSpec" | "TypeVarTuple" | "NewType" | "NamedTuple" | "namedtuple" | "TypedDict" | "Enum" | "IntEnum" | "StrEnum" | "Flag"
        | "IntFlag" => true,
        "type" => call_args(v, src).positional.len() == 3,
        _ => false,
    }
}

/// Whether `name` is free to rename a local of `scope` to: bound nowhere in
/// `scope` or above it, no builtin, no keyword, and not rebound in any
/// scope that reads the variable.
fn name_is_free(model: &SemanticModel, scope: usize, sym: usize, name: &str) -> bool {
    if name.is_empty() || KEYWORDS.contains(&name) || PYTHON_BUILTINS.contains(&name) {
        return false;
    }
    let mut cur = Some(scope);
    while let Some(sc) = cur {
        if model.scopes[sc].bindings.contains_key(name) {
            return false;
        }
        cur = model.scopes[sc].parent;
    }
    for &r in &model.symbol(sym).refs {
        let mut cur = Some(model.references[r].scope);
        while let Some(sc) = cur {
            if sc == scope {
                break;
            }
            if model.scopes[sc].bindings.contains_key(name) || model.scopes[sc].redirects.contains_key(name) {
                return false;
            }
            cur = model.scopes[sc].parent;
        }
    }
    true
}

fn used_dummy_variable(_n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    for (scope_id, scope) in model.scopes.iter().enumerate() {
        if !matches!(scope.kind, ScopeKind::Function | ScopeKind::Comprehension) {
            continue;
        }
        for (name, ids) in &scope.bindings {
            if !is_dummy_name(name) {
                continue;
            }
            for &id in ids {
                let sym = model.symbol(id);
                if sym.kind != SymbolKind::Variable || !sym.refs.iter().any(|&r| model.references[r].read) {
                    continue;
                }
                let Some(&(ds, de)) = sym.decls.first() else { continue };
                // A name bound by `T = TypeVar("T")` and the like.
                let declares = _n
                    .descendant_for_byte_range(ds, de)
                    .and_then(|d| crate::util::ancestor(d, &["assignment"]))
                    .is_some_and(|a| a.child_by_field_name("right").is_some_and(|r| declares_a_type(r, src)));
                if declares {
                    continue;
                }
                let trimmed = name.trim_start_matches('_');
                let new_name = [trimmed.to_owned(), format!("{trimmed}_")]
                    .into_iter()
                    .filter(|c| !c.is_empty())
                    .find(|c| !is_dummy_name(c) && c.chars().all(|ch| ch.is_alphanumeric() || ch == '_') && name_is_free(model, scope_id, id, c));
                let mut edits = Vec::new();
                if let Some(new_name) = &new_name {
                    let nonlocal = sym.refs.iter().any(|&r| model.scopes[model.references[r].scope].redirects.contains_key(name));
                    if !nonlocal {
                        let mut spans: BTreeSet<(usize, usize)> = sym.decls.iter().copied().collect();
                        spans.extend(sym.refs.iter().map(|&r| (model.references[r].start, model.references[r].end)));
                        edits = spans.into_iter().map(|(s, e)| TextEdit::new(s, e, new_name.clone())).collect();
                    }
                }
                out.push(NativeFinding { start: ds, end: de, message: format!("Local dummy variable `{name}` is accessed"), edits });
            }
        }
    }
}

py_rule!(UsedDummyVariable, "py/used-dummy-variable", ["module"], true, used_dummy_variable);

// ------------------------------------------------------------ unused-private-member

#[derive(Clone)]
struct Member {
    name: String,
    /// What the finding spans.
    start: usize,
    end: usize,
    /// `(self, x)` for a function.
    signature: Option<String>,
    /// A function defined inside a method: that method's span and name.
    owner: Option<(usize, usize, String)>,
}

fn is_private(name: &str) -> bool {
    name.starts_with("__") && !name.ends_with("__") && name.len() > 2
}

/// The names that stand for the instance or the class inside `class_name`'s
/// methods: the first parameter, `type(self)` and `self.__class__` aside.
fn self_names(def: Node<'_>, src: &str) -> Vec<String> {
    let Some(params) = def.child_by_field_name("parameters") else { return Vec::new() };
    kids(params)
        .into_iter()
        .next()
        .map(|p| match p.kind() {
            "identifier" => text(p, src).to_owned(),
            _ => p.named_child(0).map(|c| text(c, src).to_owned()).unwrap_or_default(),
        })
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}

fn signature(def: Node<'_>, src: &str) -> String {
    def.child_by_field_name("parameters").map_or_else(|| "()".to_owned(), |p| text(p, src).split_whitespace().collect::<Vec<_>>().join(" "))
}

/// Whether `obj` (the object of an attribute) names the class or an
/// instance of it, as `selves` and `class_name` say.
fn is_self_like(obj: Node<'_>, selves: &[String], class_name: &str, src: &str) -> bool {
    let obj = unparen(obj);
    match obj.kind() {
        "identifier" => {
            let t = text(obj, src);
            t == class_name || selves.iter().any(|s| s == t)
        }
        // `type(self)`
        "call" => {
            obj.child_by_field_name("function").is_some_and(|f| f.kind() == "identifier" && text(f, src) == "type")
                && call_args(obj, src).positional.first().is_some_and(|a| a.kind() == "identifier" && selves.iter().any(|s| s == text(*a, src)))
        }
        // `self.__class__`
        "attribute" => {
            obj.child_by_field_name("attribute").is_some_and(|a| text(a, src) == "__class__")
                && obj.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && selves.iter().any(|s| s == text(o, src)))
        }
        _ => false,
    }
}

struct Scan<'s> {
    src: &'s str,
    class_name: &'s str,
    defs: Vec<Member>,
    used: BTreeSet<String>,
    /// The methods being walked, innermost last.
    methods: Vec<Node<'s>>,
}

impl<'s> Scan<'s> {
    fn walk(&mut self, n: Node<'s>, selves: &[String], in_class_body: bool) {
        for c in named_children(n) {
            match c.kind() {
                "class_definition" => continue,
                "decorated_definition" => {
                    if let Some(d) = c.child_by_field_name("definition") {
                        self.def(d, selves, in_class_body);
                    }
                    for dec in decorators_of(c) {
                        self.walk(dec, selves, in_class_body);
                    }
                    continue;
                }
                "function_definition" => {
                    self.def(c, selves, in_class_body);
                    continue;
                }
                "assignment" | "augmented_assignment" => {
                    self.assignment(c, selves, in_class_body);
                }
                "attribute" => {
                    // A load of `self.__x` (a store is handled by its assignment).
                    if let (Some(obj), Some(attr)) = (c.child_by_field_name("object"), c.child_by_field_name("attribute"))
                        && is_private(text(attr, self.src))
                        && is_self_like(obj, selves, self.class_name, self.src)
                        && !is_store(c)
                    {
                        self.used.insert(text(attr, self.src).to_owned());
                    }
                }
                "identifier" => {
                    let t = text(c, self.src);
                    let member_name = c.parent().is_some_and(|p| {
                        (p.kind() == "attribute" && p.child_by_field_name("attribute").is_some_and(|a| a.id() == c.id()))
                            || (p.kind() == "keyword_argument" && p.child_by_field_name("name").is_some_and(|a| a.id() == c.id()))
                    });
                    if is_private(t) && !is_store(c) && !member_name {
                        self.used.insert(t.to_owned());
                    }
                }
                _ => {}
            }
            self.walk(c, selves, in_class_body);
        }
    }

    fn def(&mut self, d: Node<'s>, selves: &[String], in_class_body: bool) {
        let Some(name) = d.child_by_field_name("name") else { return };
        let t = text(name, self.src);
        if is_private(t) {
            self.defs.push(Member {
                name: t.to_owned(),
                start: d.start_byte(),
                end: name.end_byte(),
                signature: Some(signature(d, self.src)),
                owner: if in_class_body {
                    None
                } else {
                    self.methods.last().map(|m| (m.start_byte(), m.end_byte(), m.child_by_field_name("name").map_or(String::new(), |n| text(n, self.src).to_owned())))
                },
            });
        }
        // Parameters' defaults and decorators read names of the enclosing body.
        if let Some(p) = d.child_by_field_name("parameters") {
            self.walk(p, selves, in_class_body);
        }
        let inner: Vec<String> = if in_class_body { self_names(d, self.src) } else { selves.to_vec() };
        if let Some(b) = d.child_by_field_name("body") {
            self.methods.push(d);
            self.walk(b, &inner, false);
            self.methods.pop();
        }
    }

    fn assignment(&mut self, a: Node<'s>, selves: &[String], in_class_body: bool) {
        let Some(left) = a.child_by_field_name("left") else { return };
        let augmented = a.kind() == "augmented_assignment";
        match left.kind() {
            "identifier" if in_class_body && is_private(text(left, self.src)) && a.parent().is_some_and(|p| p.kind() == "expression_statement") => {
                if augmented {
                    self.used.insert(text(left, self.src).to_owned());
                } else {
                    self.defs.push(Member { name: text(left, self.src).to_owned(), start: left.start_byte(), end: left.end_byte(), signature: None, owner: None });
                }
            }
            "attribute" => {
                if let (Some(obj), Some(attr)) = (left.child_by_field_name("object"), left.child_by_field_name("attribute"))
                    && is_private(text(attr, self.src))
                    && is_self_like(obj, selves, self.class_name, self.src)
                {
                    if augmented {
                        self.used.insert(text(attr, self.src).to_owned());
                    } else {
                        self.defs.push(Member { name: text(attr, self.src).to_owned(), start: left.start_byte(), end: left.end_byte(), signature: None, owner: None });
                    }
                }
            }
            _ => {}
        }
    }
}

fn decorators_of(decorated: Node<'_>) -> Vec<Node<'_>> {
    kids(decorated).into_iter().filter(|c| c.kind() == "decorator").collect()
}

/// A node that is the target of an assignment, not a read.
fn is_store(n: Node<'_>) -> bool {
    n.parent().is_some_and(|p| p.kind() == "assignment" && p.child_by_field_name("left").is_some_and(|l| l.id() == n.id()))
}

fn unused_private_member(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(name_node) = n.child_by_field_name("name") else { return };
    let class_name = text(name_node, src);
    let Some(body) = n.child_by_field_name("body") else { return };
    let mut scan = Scan { src, class_name, defs: Vec::new(), used: BTreeSet::new(), methods: Vec::new() };
    scan.walk(body, &[], true);
    // A nested private function is used only inside its own method.
    let mut findings: BTreeMap<usize, NativeFinding> = BTreeMap::new();
    for m in &scan.defs {
        let used = match &m.owner {
            None => scan.used.contains(&m.name),
            Some((from, to, _)) => {
                // The names read in the method that defines it.
                let mut stack = vec![body.descendant_for_byte_range(*from, *to).unwrap_or(body)];
                let mut seen = false;
                while let Some(x) = stack.pop() {
                    for c in named_children(x) {
                        if c.kind() == "identifier" && text(c, src) == m.name && !(c.start_byte() >= m.start && c.end_byte() <= m.end) && !is_store(c) {
                            seen = true;
                        }
                        if c.kind() != "class_definition" {
                            stack.push(c);
                        }
                    }
                }
                seen
            }
        };
        if used {
            continue;
        }
        let path = match &m.owner {
            Some((_, _, method)) => format!("{class_name}.{method}.{}", m.name),
            None => format!("{class_name}.{}", m.name),
        };
        let shown = match &m.signature {
            Some(sig) => format!("{path}{sig}"),
            None => path,
        };
        findings.insert(m.start, NativeFinding { start: m.start, end: m.end, message: format!("Unused private member `{shown}`"), edits: Vec::new() });
    }
    out.extend(findings.into_values());
}

py_rule!(UnusedPrivateMember, "py/unused-private-member", ["class_definition"], false, unused_private_member);

pub(crate) static RULES: &[&dyn crate::NativeRule] =
    &[&NonlocalWithoutBinding, &BuiltinAttributeShadowing, &MissingFStringSyntax, &UsedDummyVariable, &UnusedPrivateMember];

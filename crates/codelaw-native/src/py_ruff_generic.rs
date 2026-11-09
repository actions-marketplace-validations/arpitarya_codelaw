//! W-139 strand W-139-ruff: the rows that turn a module's `TypeVar`s into
//! something the language says better — PEP 695 type parameters (UP046,
//! UP047) and `Self` (PYI019) — and PYI025's alias for `collections.abc.Set`.
//! Behaviour is Ruff 0.16.9's (MIT; read from its source and tests over the
//! network, research only, L5); each rule is written fresh. A `TypeVar` is
//! recognised only when the file assigns it (`T = TypeVar("T", …)`).

use crate::python_util::{call_args, decorators, hit, kids, qualified_name, string_parts, PY};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

fn root_of(n: Node<'_>) -> Node<'_> {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        cur = p;
    }
    cur
}

fn is_typing(q: &str, name: &str) -> bool {
    q == format!("typing.{name}") || q == format!("typing_extensions.{name}")
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    TypeVar,
    TypeVarTuple,
    ParamSpec,
}

enum Restriction {
    Bound(String),
    Constraints(Vec<String>),
    AnyStr,
}

struct Tv {
    name: String,
    kind: Kind,
    restriction: Option<Restriction>,
    default: bool,
}

impl Tv {
    fn display(&self) -> String {
        let star = match self.kind {
            Kind::TypeVar => "",
            Kind::TypeVarTuple => "*",
            Kind::ParamSpec => "**",
        };
        let r = match &self.restriction {
            None => String::new(),
            Some(Restriction::Bound(b)) => format!(": {b}"),
            Some(Restriction::Constraints(c)) => format!(": ({})", c.join(", ")),
            Some(Restriction::AnyStr) => ": (bytes, str)".to_owned(),
        };
        format!("{star}{}{r}", self.name)
    }
}

/// The `TypeVar`, `TypeVarTuple` or `ParamSpec` the identifier `id` names:
/// a module-level `X = TypeVar("X", …)`.
fn typevar_at(id: Node<'_>, ctx: &NativeContext<'_>) -> Option<Tv> {
    let model = ctx.semantic?;
    let src = ctx.source;
    let sym_id = model.symbol_at(id.start_byte())?;
    let sym = model.symbol(sym_id);
    if sym.decls.len() != 1 {
        return None;
    }
    let (s, e) = sym.decls[0];
    let decl = root_of(id).descendant_for_byte_range(s, e)?;
    let assign = decl.parent().filter(|p| p.kind() == "assignment" && field(*p, "left") == Some(decl))?;
    let call = field(assign, "right").filter(|c| c.kind() == "call")?;
    let q = qualified_name(field(call, "function")?, model, src)?;
    let kind = if is_typing(&q, "TypeVar") {
        Kind::TypeVar
    } else if is_typing(&q, "TypeVarTuple") {
        Kind::TypeVarTuple
    } else if is_typing(&q, "ParamSpec") {
        Kind::ParamSpec
    } else {
        return None;
    };
    let args = call_args(call, src);
    let first = args.positional.first().filter(|f| f.kind() == "string")?;
    string_parts(*first, src)?;
    let bound = args.keyword("bound").map(|b| text(b, src).to_owned());
    let restriction = match bound {
        Some(b) => Some(Restriction::Bound(b)),
        None if args.positional.len() > 1 => Some(Restriction::Constraints(args.positional.iter().skip(1).map(|a| text(*a, src).to_owned()).collect())),
        None => None,
    };
    Some(Tv { name: text(id, src).to_owned(), kind, restriction, default: args.keyword("default").is_some() })
}

/// Every type variable named under `n`; `skipped` when a name there is not one.
fn collect_typevars(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<Tv>, skipped: &mut bool) {
    match n.kind() {
        "identifier" => {
            if let Some(model) = ctx.semantic
                && qualified_name(n, model, ctx.source).is_some_and(|q| is_typing(&q, "AnyStr"))
            {
                out.push(Tv { name: "AnyStr".into(), kind: Kind::TypeVar, restriction: Some(Restriction::AnyStr), default: false });
            } else if let Some(t) = typevar_at(n, ctx) {
                out.push(t);
            } else {
                *skipped = true;
            }
        }
        "attribute" => {
            if let Some(o) = field(n, "object") {
                collect_typevars(o, ctx, out, skipped);
            }
        }
        "keyword_argument" => {
            if let Some(v) = field(n, "value") {
                collect_typevars(v, ctx, out, skipped);
            }
        }
        "string" | "integer" | "float" | "none" | "true" | "false" | "ellipsis" => {}
        _ => {
            for c in named_children(n) {
                collect_typevars(c, ctx, out, skipped);
            }
        }
    }
}

/// The variables in order of first use, or `None` when the set is empty or
/// cannot be written as type parameters.
fn usable(vars: Vec<Tv>) -> Option<Vec<Tv>> {
    if vars.is_empty() {
        return None;
    }
    if vars.iter().any(|v| v.kind != Kind::TypeVar && v.restriction.is_some()) || vars.iter().any(|v| v.default) {
        return None;
    }
    let mut seen: Vec<String> = Vec::new();
    for v in &vars {
        if seen.contains(&v.name) {
            return None;
        }
        seen.push(v.name.clone());
    }
    Some(vars)
}

fn type_params_text(vars: &[Tv]) -> String {
    format!("[{}]", vars.iter().map(Tv::display).collect::<Vec<_>>().join(", "))
}

/// Whether `n` sits in a function or class body, below the one `n` itself
/// defines.
fn nested(n: Node<'_>) -> bool {
    let mut p = n.parent();
    while let Some(x) = p {
        if matches!(x.kind(), "function_definition" | "class_definition") {
            return true;
        }
        p = x.parent();
    }
    false
}

fn has_comment(n: Node<'_>) -> bool {
    let mut stack = vec![n];
    while let Some(c) = stack.pop() {
        if c.kind() == "comment" {
            return true;
        }
        stack.extend(named_children(c));
    }
    false
}

// ------------------------------------------------------------------ UP046

fn up046(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(model) = ctx.semantic else { return };
    if nested(n) || field(n, "type_parameters").is_some() {
        return;
    }
    let Some(list) = field(n, "superclasses") else { return };
    let items = kids(list);
    let positional: Vec<Node<'_>> = items.iter().copied().filter(|c| !matches!(c.kind(), "keyword_argument" | "dictionary_splat")).collect();
    let Some((idx, generic)) = positional.iter().enumerate().find(|(_, c)| {
        c.kind() == "subscript" && field(**c, "value").and_then(|v| qualified_name(v, model, src)).is_some_and(|q| is_typing(&q, "Generic"))
    }) else {
        return;
    };
    let generic = *generic;
    let finding_start = generic.start_byte();
    let finding_end = generic.end_byte();
    let name = field(n, "name").map_or("", |x| text(x, src));
    let message = format!("Generic class `{name}` uses `Generic` subclass instead of type parameters");
    let mut vars = Vec::new();
    let mut skipped = false;
    let mut c = generic.walk();
    for s in generic.children_by_field_name("subscript", &mut c) {
        collect_typevars(s, ctx, &mut vars, &mut skipped);
    }
    let mut edits = Vec::new();
    if idx == positional.len() - 1 && !skipped && !has_comment(list) {
        let Some(vars) = usable(vars) else { return };
        let Some(name_node) = field(n, "name") else { return };
        edits.push(TextEdit::new(name_node.end_byte(), name_node.end_byte(), type_params_text(&vars)));
        let pos = items.iter().position(|c| *c == generic).unwrap_or(0);
        if items.len() == 1 {
            edits.push(TextEdit::new(list.start_byte(), list.end_byte(), ""));
        } else if let Some(next) = items.get(pos + 1) {
            edits.push(TextEdit::new(generic.start_byte(), next.start_byte(), ""));
        } else if let Some(prev) = pos.checked_sub(1).and_then(|p| items.get(p)) {
            edits.push(TextEdit::new(prev.end_byte(), generic.end_byte(), ""));
        }
    } else if !skipped && idx == positional.len() - 1 && usable(vars).is_none() {
        return;
    }
    out.push(NativeFinding { start: finding_start, end: finding_end, message, edits });
}

// ------------------------------------------------------------------ UP047

fn up047(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    if nested(n) || field(n, "type_parameters").is_some() {
        return;
    }
    let Some(params) = field(n, "parameters") else { return };
    let Some(name) = field(n, "name") else { return };
    let mut vars: Vec<Tv> = Vec::new();
    for p in kids(params) {
        if let Some(t) = field(p, "type").filter(|_| matches!(p.kind(), "typed_parameter" | "typed_default_parameter")) {
            let mut skipped = false;
            collect_typevars(t, ctx, &mut vars, &mut skipped);
        }
    }
    let mut seen: Vec<String> = Vec::new();
    vars.retain(|v| {
        if seen.contains(&v.name) {
            false
        } else {
            seen.push(v.name.clone());
            true
        }
    });
    let Some(vars) = usable(vars) else { return };
    let edits = vec![TextEdit::new(name.end_byte(), name.end_byte(), type_params_text(&vars))];
    out.push(NativeFinding { start: name.start_byte(), end: params.end_byte(), message: format!("Generic function `{}` should use type parameters", text(name, src)), edits });
}

// ------------------------------------------------------------------ PYI019

fn enclosing_class(def: Node<'_>) -> Option<Node<'_>> {
    let mut p = def.parent();
    if p.is_some_and(|x| x.kind() == "decorated_definition") {
        p = p.and_then(|x| x.parent());
    }
    p?.parent().filter(|c| c.kind() == "class_definition")
}

/// The custom type variable a method annotates its first parameter with:
/// the identifier of the annotation, and whether it is `type[T]`.
fn self_typevar<'t>(def: Node<'t>, ctx: &NativeContext<'_>) -> Option<(Node<'t>, Node<'t>, Tv)> {
    let src = ctx.source;
    let class = enclosing_class(def)?;
    let names = crate::python_util::decorators(def).into_iter().map(|d| text(d, src).to_owned()).collect::<Vec<_>>();
    if names.iter().any(|d| d == "staticmethod" || d.ends_with("abstractmethod") || d.ends_with("overload")) {
        return None;
    }
    if let Some(sup) = field(class, "superclasses")
        && kids(sup).iter().any(|b| matches!(text(*b, src), "type" | "ABCMeta" | "EnumMeta" | "EnumType" | "abc.ABCMeta"))
    {
        return None;
    }
    let fname = text(field(def, "name")?, src);
    let is_class_method = names.iter().any(|d| d == "classmethod") || matches!(fname, "__new__" | "__init_subclass__" | "__class_getitem__");
    let params = field(def, "parameters")?;
    let first = kids(params).into_iter().next().filter(|p| p.kind() == "typed_parameter")?;
    let ann = field(first, "type")?;
    let mut expr = ann.named_child(0)?;
    if is_class_method {
        if expr.kind() != "generic_type" && expr.kind() != "subscript" {
            return None;
        }
        let head = if expr.kind() == "generic_type" { expr.named_child(0)? } else { field(expr, "value")? };
        if text(head, src) != "type" {
            return None;
        }
        let inner = if expr.kind() == "generic_type" { named_children(expr).into_iter().find(|c| c.kind() == "type_parameter")?.named_child(0)?.named_child(0)? } else { field(expr, "subscript")? };
        expr = inner;
    }
    if expr.kind() != "identifier" {
        return None;
    }
    let tv = typevar_at(expr, ctx)?;
    Some((first, expr, tv))
}

fn collect_methods<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    if n.kind() == "function_definition" {
        out.push(n);
    }
    for c in named_children(n) {
        collect_methods(c, out);
    }
}

fn identifiers_named<'t>(n: Node<'t>, name: &str, src: &str, out: &mut Vec<Node<'t>>, skip: (usize, usize)) {
    if n.kind() == "identifier" && text(n, src) == name && !(n.start_byte() >= skip.0 && n.end_byte() <= skip.1) {
        // Not the name after a dot, nor a keyword argument's name.
        let attr = n.parent().is_some_and(|p| p.kind() == "attribute" && field(p, "attribute") == Some(n));
        if !attr {
            out.push(n);
        }
    }
    for c in named_children(n) {
        identifiers_named(c, name, src, out, skip);
    }
}

fn mentions_in_string(n: Node<'_>, name: &str, src: &str) -> bool {
    if n.kind() == "string" {
        let t = text(n, src);
        return t.match_indices(name).any(|(i, _)| {
            let before = t[..i].chars().next_back().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
            let after = t[i + name.len()..].chars().next().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
            before && after
        });
    }
    named_children(n).into_iter().any(|c| mentions_in_string(c, name, src))
}

fn self_import_edit(root: Node<'_>, src: &str) -> Option<Option<TextEdit>> {
    // `Some(None)`: `Self` is already imported from typing; `None`: the name is taken.
    let mut last_import_end = None;
    let mut merge: Option<usize> = None;
    for s in kids(root) {
        match s.kind() {
            "import_from_statement" | "future_import_statement" => {
                let m = field(s, "module_name").map_or("__future__", |m| text(m, src));
                let mut c = s.walk();
                let names: Vec<&str> = s.children_by_field_name("name", &mut c).map(|c| text(c, src)).collect();
                if matches!(m, "typing" | "typing_extensions") && names.contains(&"Self") {
                    return Some(None);
                }
                if m == "typing" && merge.is_none() && s.kind() == "import_from_statement" && !children(s).iter().any(|c| c.kind() == "wildcard_import") {
                    merge = s.children_by_field_name("name", &mut s.walk()).last().map(|l| l.end_byte());
                }
                last_import_end = Some(s.end_byte());
            }
            "import_statement" => last_import_end = Some(s.end_byte()),
            _ => {}
        }
    }
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "identifier" && text(n, src) == "Self" {
            return None;
        }
        stack.extend(named_children(n));
    }
    if let Some(end) = merge {
        return Some(Some(TextEdit::new(end, end, ", Self")));
    }
    let at = match last_import_end {
        Some(e) => src[e..].find('\n').map_or(src.len(), |i| e + i + 1),
        None => {
            let first = kids(root).into_iter().next();
            match first {
                Some(f) if f.kind() == "expression_statement" && f.named_child(0).is_some_and(|c| c.kind() == "string") => src[f.end_byte()..].find('\n').map_or(src.len(), |i| f.end_byte() + i + 1),
                _ => 0,
            }
        }
    };
    let text_ = if at == 0 { "from typing import Self\n\n".to_owned() } else { "from typing import Self\n".to_owned() };
    Some(Some(TextEdit::new(at, at, text_)))
}

fn pyi019(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some((param, ann_expr, tv)) = self_typevar(n, ctx) else { return };
    let Some(ann) = field(param, "type") else { return };
    let header_end = field(n, "return_type").map_or_else(|| field(n, "parameters").map_or(n.end_byte(), |p| p.end_byte()), |r| r.end_byte());
    let name_end = field(n, "name").map_or(n.start_byte(), |x| x.end_byte());
    let message = format!("Use `Self` instead of custom TypeVar `{}`", tv.name);
    let mut edits = Vec::new();
    let strings = mentions_in_string(n, &tv.name, src);
    let root = root_of(n);
    if !strings && let Some(import) = self_import_edit(root, src) {
        // Delete the annotation, then name `Self` wherever the variable is read in the method.
        let pname = param.named_child(0).map_or(param.start_byte(), |x| x.end_byte());
        edits.push(TextEdit::new(pname, ann.end_byte(), ""));
        let mut refs = Vec::new();
        identifiers_named(n, &tv.name, src, &mut refs, (param.start_byte(), ann.end_byte()));
        for r in refs {
            edits.push(TextEdit::new(r.start_byte(), r.end_byte(), "Self"));
        }
        // The import goes with the first method of the file that needs it.
        let mut methods = Vec::new();
        collect_methods(root, &mut methods);
        let earlier = methods.iter().any(|m| m.start_byte() < n.start_byte() && self_typevar(*m, ctx).is_some());
        if let (Some(i), false) = (import, earlier) {
            edits.push(i);
        }
    }
    let _ = ann_expr;
    out.push(NativeFinding { start: name_end, end: header_end, message, edits });
}

// ------------------------------------------------------------------ PYI025

fn pyi025(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(model) = ctx.semantic else { return };
    if field(n, "module_name").is_none_or(|m| text(m, src).split_whitespace().collect::<String>() != "collections.abc") {
        return;
    }
    let mut c = n.walk();
    for name in n.children_by_field_name("name", &mut c) {
        if name.kind() != "dotted_name" || text(name, src) != "Set" {
            continue;
        }
        let mut edits = Vec::new();
        let taken = {
            let mut stack = vec![root_of(n)];
            let mut found = false;
            while let Some(x) = stack.pop() {
                if x.kind() == "identifier" && text(x, src) == "AbstractSet" {
                    found = true;
                    break;
                }
                stack.extend(named_children(x));
            }
            found
        };
        if !taken && let Some(id) = model.declared_at(name.start_byte()).or_else(|| name.named_child(0).and_then(|i| model.declared_at(i.start_byte()))) {
            edits.push(TextEdit::new(name.start_byte(), name.end_byte(), "Set as AbstractSet"));
            for r in &model.symbol(id).refs {
                let r = &model.references[*r];
                edits.push(TextEdit::new(r.start, r.end, "AbstractSet"));
            }
        }
        out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: "Alias `Set` to `AbstractSet` when importing it from `collections.abc`".into(), edits });
    }
}

macro_rules! gen_rule {
    ($name:ident, $id:literal, $kind:literal, $f:path) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &[$kind]
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                $f(node.get_inner_node(), ctx, out)
            }
        }
    };
}

gen_rule!(NonPep695GenericClass, "py/non-pep695-generic-class", "class_definition", up046);
gen_rule!(NonPep695GenericFunction, "py/non-pep695-generic-function", "function_definition", up047);
gen_rule!(CustomTypeVarForSelf, "py/custom-type-var-for-self", "function_definition", pyi019);
gen_rule!(UnaliasedCollectionsAbcSetImport, "py/unaliased-collections-abc-set-import", "import_from_statement", pyi025);

pub static RULES: &[&dyn NativeRule] = &[&NonPep695GenericClass, &NonPep695GenericFunction, &CustomTypeVarForSelf, &UnaliasedCollectionsAbcSetImport];

#[allow(dead_code)]
fn unused(n: Node<'_>) -> NativeFinding {
    let _ = (children(n), decorators(n));
    hit(n, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        crate::run(rule, Language::Python, src)
    }

    #[test]
    fn generic_class() {
        let src = "from typing import Generic, TypeVar\nT = TypeVar(\"T\", bound=int)\nclass A(Generic[T]):\n    pass\n";
        let r = run(&NonPep695GenericClass, src);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].edits[0].text, "[T: int]");
        let src = "from typing import Generic, TypeVar\nT = TypeVar(\"T\")\ndef f(x: T) -> T: ...\n";
        assert_eq!(run(&NonPep695GenericFunction, src).len(), 1);
    }

    #[test]
    fn self_type() {
        let src = "from typing import TypeVar\n_S = TypeVar(\"_S\", bound=\"A\")\nclass A:\n    def m(self: _S) -> _S:\n        return self\n";
        let r = run(&CustomTypeVarForSelf, src);
        assert_eq!(r.len(), 1);
        assert!(r[0].edits.len() >= 3);
    }

    #[test]
    fn abstract_set() {
        let src = "from collections.abc import Set\nx: Set[int]\n";
        let r = run(&UnaliasedCollectionsAbcSetImport, src);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].edits.len(), 2);
    }
}

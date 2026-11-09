//! W-174, the value-type group of SR-PY-INFERENCE: Ruff 0.16.9 and Pylint
//! 4.1.3 checks that need the type of a value, answered only where one file
//! states it — a literal, a `dict()` or `set()` call, a comprehension, an
//! annotation, or a call to a function this file defines — for a name bound
//! once in its scope. A value that comes out of a call to code elsewhere is
//! unknown, and unknown never reports. Written fresh from the upstream
//! behaviour.

use crate::python_util::{call_args, decorators, kids, py_rule, qualified_name, unparen};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::TextEdit;
use codelaw_semantic::{ScopeKind, SemanticModel, SymbolKind};
use tree_sitter::Node;

fn at(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

// ---------------------------------------------------------------- bindings

/// Where a name's only binding gets its type from.
struct Origin<'t> {
    value: Option<Node<'t>>,
    annotation: Option<Node<'t>>,
}

/// The single binding of the name read at `id`, when the scope binds it once
/// and never assigns it again.
fn only_origin<'t>(id: Node<'t>, model: &SemanticModel, root: Node<'t>) -> Option<(usize, Origin<'t>)> {
    let sym_id = model.reference_at(id.start_byte()).and_then(|r| r.symbol)?;
    let sym = model.symbol(sym_id);
    if sym.decls.len() != 1 || !matches!(sym.kind, SymbolKind::Variable | SymbolKind::Param) {
        return None;
    }
    if sym.refs.iter().any(|&r| model.references[r].write) {
        return None;
    }
    let (ds, de) = sym.decls[0];
    let decl = root.descendant_for_byte_range(ds, de)?;
    let parent = decl.parent()?;
    let origin = match parent.kind() {
        "assignment" if parent.child_by_field_name("left").is_some_and(|l| l.id() == decl.id()) => {
            Origin { value: parent.child_by_field_name("right"), annotation: parent.child_by_field_name("type") }
        }
        "typed_parameter" | "typed_default_parameter" => Origin { value: None, annotation: parent.child_by_field_name("type") },
        "as_pattern_target" => {
            let pat = parent.parent()?;
            Origin { value: named_children(pat).into_iter().next(), annotation: None }
        }
        _ => return None,
    };
    Some((sym_id, origin))
}

/// Every name an annotation mentions, qualified through the file's imports.
fn annotation_names(a: Node<'_>, model: &SemanticModel, src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![a];
    while let Some(x) = stack.pop() {
        match x.kind() {
            "identifier" | "attribute" => {
                if let Some(q) = qualified_name(x, model, src) {
                    out.push(q);
                }
            }
            _ => stack.extend(named_children(x)),
        }
    }
    out
}

/// The qualified name of an annotation's base: `dict` for `dict[str, int]`.
fn annotation_base<'t>(a: Node<'t>) -> Node<'t> {
    let a = if a.kind() == "type" { a.named_child(0).unwrap_or(a) } else { a };
    match a.kind() {
        "generic_type" | "subscript" => a.named_child(0).unwrap_or(a),
        _ => a,
    }
}

fn call_target<'t>(n: Node<'t>) -> Option<Node<'t>> {
    let n = unparen(n);
    (n.kind() == "call").then(|| n.child_by_field_name("function")).flatten()
}

fn is_dict_origin(o: &Origin<'_>, model: &SemanticModel, src: &str) -> bool {
    let by_annotation = o.annotation.is_some_and(|a| {
        qualified_name(annotation_base(a), model, src).is_some_and(|q| {
            matches!(q.as_str(), "builtins.dict" | "typing.Dict" | "typing.DefaultDict" | "collections.defaultdict" | "collections.OrderedDict" | "typing.OrderedDict")
        })
    });
    let by_value = o.value.is_some_and(|v| {
        let v = unparen(v);
        matches!(v.kind(), "dictionary" | "dictionary_comprehension")
            || call_target(v).is_some_and(|f| {
                qualified_name(f, model, src).is_some_and(|q| matches!(q.as_str(), "builtins.dict" | "collections.defaultdict" | "collections.OrderedDict"))
            })
    });
    by_annotation || by_value
}

fn is_set_origin(o: &Origin<'_>, model: &SemanticModel, src: &str) -> bool {
    let by_annotation = o.annotation.is_some_and(|a| {
        qualified_name(annotation_base(a), model, src).is_some_and(|q| matches!(q.as_str(), "builtins.set" | "typing.Set"))
    });
    let by_value = o.value.is_some_and(|v| {
        let v = unparen(v);
        matches!(v.kind(), "set" | "set_comprehension")
            || call_target(v).is_some_and(|f| qualified_name(f, model, src).is_some_and(|q| q == "builtins.set"))
    });
    by_annotation || by_value
}

fn tuple_of_two(n: Node<'_>) -> bool {
    n.kind() == "tuple" && kids(n).len() == 2
}

// ---------------------------------------------------------------- dict-iter-missing-items

fn dict_iter_missing_items(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let (Some(left), Some(iter)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return };
    if !matches!(left.kind(), "pattern_list" | "tuple_pattern") || kids(left).len() != 2 || iter.kind() != "identifier" {
        return;
    }
    let mut top = n;
    while let Some(p) = top.parent() {
        top = p;
    }
    let Some((_, origin)) = only_origin(iter, model, top) else { return };
    if !is_dict_origin(&origin, model, src) {
        return;
    }
    // Keys that are all pairs: iterating the dict gives them.
    let pairs: Vec<Node<'_>> = match origin.value.map(unparen) {
        Some(v) if v.kind() == "dictionary" => kids(v).into_iter().filter(|p| p.kind() == "pair").collect(),
        _ => Vec::new(),
    };
    if !pairs.is_empty() && pairs.iter().all(|p| p.child_by_field_name("key").is_some_and(|k| tuple_of_two(unparen(k)))) {
        return;
    }
    if pairs.is_empty()
        && origin.annotation.is_some_and(|a| {
            let t = text(a, src).replace(' ', "");
            t.contains("[tuple[") || t.contains("[Tuple[")
        })
    {
        return;
    }
    let name = text(iter, src);
    out.push(at(iter, format!("Unpacking a dictionary in iteration without calling `.items()`: `{name}`"), vec![TextEdit::new(iter.start_byte(), iter.end_byte(), format!("{name}.items()"))]));
}

py_rule!(DictIterMissingItems, "py/dict-iter-missing-items", ["for_statement"], true, dict_iter_missing_items);

// ---------------------------------------------------------------- modified-iterating-set

fn modified_iterating_set(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let (Some(iter), Some(body)) = (n.child_by_field_name("right"), n.child_by_field_name("body")) else { return };
    if iter.kind() != "identifier" {
        return;
    }
    let mut top = n;
    while let Some(p) = top.parent() {
        top = p;
    }
    let Some((sym, origin)) = only_origin(iter, model, top) else { return };
    if !is_set_origin(&origin, model, src) {
        return;
    }
    let mut stack = vec![body];
    while let Some(x) = stack.pop() {
        for k in named_children(x) {
            if matches!(k.kind(), "function_definition" | "class_definition" | "lambda") {
                continue;
            }
            stack.push(k);
            if k.kind() != "call" {
                continue;
            }
            let Some(f) = k.child_by_field_name("function").filter(|f| f.kind() == "attribute") else { continue };
            let (Some(o), Some(a)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { continue };
            if o.kind() == "identifier"
                && matches!(text(a, src), "add" | "clear" | "discard" | "pop" | "remove")
                && model.reference_at(o.start_byte()).and_then(|r| r.symbol) == Some(sym)
            {
                let name = text(iter, src);
                out.push(NativeFinding {
                    start: n.start_byte(),
                    end: n.end_byte(),
                    message: format!("Iterated set `{name}` is modified within the `for` loop"),
                    edits: vec![TextEdit::new(iter.start_byte(), iter.end_byte(), format!("{name}.copy()"))],
                });
                return;
            }
        }
    }
}

py_rule!(ModifiedIteratingSet, "py/modified-iterating-set", ["for_statement"], true, modified_iterating_set);

// ---------------------------------------------------------------- bad-string-format-type

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Atom {
    Str,
    Bytes,
    Container,
    NoneOrEllipsis,
    Int,
    Float,
    Complex,
}

/// The type a literal-ish expression has, or `None` when it is unknown.
fn atom(n: Node<'_>, src: &str) -> Option<Atom> {
    let n = unparen(n);
    match n.kind() {
        "string" | "concatenated_string" => {
            let prefix: String = text(n, src).chars().take_while(|c| c.is_ascii_alphabetic()).collect::<String>().to_ascii_lowercase();
            Some(if prefix.contains('b') { Atom::Bytes } else { Atom::Str })
        }
        "integer" | "true" | "false" => Some(Atom::Int),
        "float" => Some(Atom::Float),
        "list" | "tuple" | "dictionary" | "set" | "list_comprehension" | "set_comprehension" | "dictionary_comprehension" | "generator_expression" => Some(Atom::Container),
        "none" | "ellipsis" => Some(Atom::NoneOrEllipsis),
        "unary_operator" => {
            let a = atom(n.child_by_field_name("argument")?, src)?;
            matches!(a, Atom::Int | Atom::Float).then_some(a)
        }
        _ => {
            if text(n, src).ends_with(['j', 'J']) && n.kind() == "integer" {
                Some(Atom::Complex)
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Conv {
    Any,
    Integer,
    Number,
    Float,
}

fn conv_of(c: char) -> Conv {
    match c {
        'n' | 'd' => Conv::Number,
        'b' | 'c' | 'o' | 'x' | 'X' => Conv::Integer,
        'e' | 'E' | 'f' | 'F' | 'g' | 'G' | '%' => Conv::Float,
        _ => Conv::Any,
    }
}

fn compatible(conv: char, a: Atom, value: Node<'_>, src: &str) -> bool {
    let c = conv_of(conv);
    if conv == 'c'
        && let Some(s) = crate::python_util::string_value(unparen(value), src)
        && s.chars().count() == 1
    {
        return true;
    }
    match a {
        Atom::Int => true,
        Atom::Float => matches!(c, Conv::Any | Conv::Float | Conv::Number),
        _ => c == Conv::Any,
    }
}

/// The conversions of a `%` format string and each one's mapping key, or
/// `None` when it uses `*` (the arguments then are not one per conversion).
fn conversions(f: &str) -> Option<Vec<(Option<String>, char)>> {
    let mut out = Vec::new();
    let b: Vec<char> = f.chars().collect();
    let mut i = 0;
    while i < b.len() {
        if b[i] != '%' {
            i += 1;
            continue;
        }
        i += 1;
        let mut key = None;
        if b.get(i) == Some(&'(') {
            let end = b[i..].iter().position(|c| *c == ')')? + i;
            key = Some(b[i + 1..end].iter().collect());
            i = end + 1;
        }
        while b.get(i).is_some_and(|c| "#0- +".contains(*c)) {
            i += 1;
        }
        if b.get(i) == Some(&'*') {
            return None;
        }
        while b.get(i).is_some_and(|c| c.is_ascii_digit()) {
            i += 1;
        }
        if b.get(i) == Some(&'.') {
            i += 1;
            if b.get(i) == Some(&'*') {
                return None;
            }
            while b.get(i).is_some_and(|c| c.is_ascii_digit()) {
                i += 1;
            }
        }
        while b.get(i).is_some_and(|c| "hlL".contains(*c)) {
            i += 1;
        }
        let conv = *b.get(i)?;
        i += 1;
        if conv != '%' {
            out.push((key, conv));
        }
    }
    Some(out)
}

fn bad_string_format_type(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    if n.child_by_field_name("operator").is_none_or(|o| o.kind() != "%") {
        return;
    }
    let (Some(left), Some(right)) = (n.child_by_field_name("left"), n.child_by_field_name("right")) else { return };
    if !matches!(left.kind(), "string" | "concatenated_string") || crate::python_util::is_fstring(left, src) {
        return;
    }
    let Some(fmt) = crate::python_util::string_value(left, src) else { return };
    let Some(convs) = conversions(&fmt) else { return };
    let right = unparen(right);
    let ok = match right.kind() {
        "tuple" => {
            let vals = kids(right);
            if vals.iter().any(|v| matches!(v.kind(), "list_splat")) || convs.len() > vals.len() {
                true
            } else {
                convs.iter().zip(vals.iter()).all(|((_, c), v)| atom(*v, src).is_none_or(|a| compatible(*c, a, *v, src)))
            }
        }
        "dictionary" => {
            let pairs: Vec<Node<'_>> = kids(right).into_iter().filter(|p| p.kind() == "pair").collect();
            if convs.len() > pairs.len() {
                true
            } else {
                pairs.iter().all(|p| {
                    let (Some(k), Some(v)) = (p.child_by_field_name("key"), p.child_by_field_name("value")) else { return true };
                    let Some(key) = crate::python_util::string_value(k, src) else { return true };
                    match convs.iter().find(|(mk, _)| mk.as_deref() == Some(key.as_str())) {
                        Some((_, c)) => atom(v, src).is_none_or(|a| compatible(*c, a, v, src)),
                        None => true,
                    }
                })
            }
        }
        _ => {
            if convs.len() != 1 {
                true
            } else {
                atom(right, src).is_none_or(|a| compatible(convs[0].1, a, right, src))
            }
        }
    };
    if !ok {
        out.push(at(n, "Format type does not match argument type", Vec::new()));
    }
}

py_rule!(BadStringFormatType, "py/bad-string-format-type", ["binary_operator"], false, bad_string_format_type);

// ---------------------------------------------------------------- comparison-with-callable

fn plain_function<'t>(id: Node<'t>, model: &SemanticModel) -> bool {
    let Some(sym) = model.reference_at(id.start_byte()).and_then(|r| r.symbol) else { return false };
    let s = model.symbol(sym);
    s.kind == SymbolKind::Function && s.decls.len() == 1 && model.scopes[s.scope].kind == ScopeKind::Module
}

fn is_decorated_def(id: Node<'_>, model: &SemanticModel, root: Node<'_>) -> bool {
    let Some(sym) = model.reference_at(id.start_byte()).and_then(|r| r.symbol) else { return true };
    let (ds, de) = model.symbol(sym).decls[0];
    root.descendant_for_byte_range(ds, de).and_then(|d| d.parent()).is_none_or(|def| def.kind() != "function_definition" || !decorators(def).is_empty())
}

fn comparison_with_callable(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let operands = kids(n).into_iter().filter(|c| !matches!(c.kind(), "==" | "!=")).collect::<Vec<_>>();
    let ops: Vec<&str> = children_ops(n, src);
    if operands.len() != 2 || ops.len() != 1 || !matches!(ops[0], "==" | "!=") {
        return;
    }
    let mut top = n;
    while let Some(p) = top.parent() {
        top = p;
    }
    for (f, other) in [(operands[0], operands[1]), (operands[1], operands[0])] {
        if f.kind() == "identifier"
            && plain_function(f, model)
            && !is_decorated_def(f, model, top)
            && atom(other, src).is_some()
        {
            out.push(at(n, "Comparing against a callable, did you omit the parenthesis?", Vec::new()));
            return;
        }
    }
}

fn children_ops<'s>(n: Node<'_>, src: &'s str) -> Vec<&'s str> {
    crate::util::children(n).into_iter().filter(|c| !c.is_named()).map(|c| &src[c.byte_range()]).collect()
}

py_rule!(ComparisonWithCallable, "py/comparison-with-callable", ["comparison_operator"], true, comparison_with_callable);

// ---------------------------------------------------------------- positional-only-arguments-expected

fn positional_only_arguments_expected(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let Some(f) = n.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { return };
    if !plain_function(f, model) {
        return;
    }
    let mut top = n;
    while let Some(p) = top.parent() {
        top = p;
    }
    let Some(sym) = model.reference_at(f.start_byte()).and_then(|r| r.symbol) else { return };
    let (ds, de) = model.symbol(sym).decls[0];
    let Some(def) = top.descendant_for_byte_range(ds, de).and_then(|d| d.parent()).filter(|d| d.kind() == "function_definition") else { return };
    let Some(params) = def.child_by_field_name("parameters") else { return };
    let mut names: Vec<&str> = Vec::new();
    let mut posonly: Vec<&str> = Vec::new();
    let mut kwarg = false;
    for p in kids(params) {
        match p.kind() {
            "positional_separator" => posonly = names.clone(),
            "dictionary_splat_pattern" => kwarg = true,
            "identifier" => names.push(text(p, src)),
            // `**kwargs: T` is a `typed_parameter` wrapping the splat; `*args: T` is not a named parameter either.
            "typed_parameter" if p.named_child(0).is_some_and(|c| c.kind() == "dictionary_splat_pattern") => kwarg = true,
            "typed_parameter" if p.named_child(0).is_some_and(|c| c.kind() == "list_splat_pattern") => {}
            "typed_parameter" | "default_parameter" | "typed_default_parameter" => {
                if let Some(nm) = p.child_by_field_name("name").or_else(|| p.named_child(0)) {
                    names.push(text(nm, src));
                }
            }
            _ => {}
        }
    }
    if posonly.is_empty() || kwarg {
        return;
    }
    let a = call_args(n, src);
    let bad: Vec<&str> = a.keywords.iter().map(|(k, _, _)| k.as_str()).filter(|k| posonly.contains(k)).collect();
    if !bad.is_empty() {
        out.push(at(n, format!("`{}()` got some positional-only arguments passed as keyword arguments: {}", text(f, src), bad.iter().map(|b| format!("`{b}`")).collect::<Vec<_>>().join(", ")), Vec::new()));
    }
}

py_rule!(PositionalOnlyArgumentsExpected, "py/positional-only-arguments-expected", ["call"], true, positional_only_arguments_expected);

// ---------------------------------------------------------------- ASYNC212

const HTTPX_BLOCKING: &[&str] = &["close", "delete", "get", "head", "options", "patch", "post", "put", "request", "send", "stream"];

fn blocking_http_call_httpx(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let Some(f) = n.child_by_field_name("function").filter(|f| f.kind() == "attribute") else { return };
    let (Some(obj), Some(attr)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { return };
    if obj.kind() != "identifier" || !HTTPX_BLOCKING.contains(&text(attr, src)) {
        return;
    }
    // The innermost function around the call is `async def`.
    let mut cur = n.parent();
    let mut in_async = false;
    while let Some(p) = cur {
        match p.kind() {
            "function_definition" => {
                in_async = text(p, src).starts_with("async");
                break;
            }
            "lambda" | "class_definition" => break,
            _ => {}
        }
        cur = p.parent();
    }
    if !in_async {
        return;
    }
    let mut top = n;
    while let Some(p) = top.parent() {
        top = p;
    }
    let Some((_, origin)) = only_origin(obj, model, top) else { return };
    let by_annotation = origin.annotation.is_some_and(|a| annotation_names(a, model, src).iter().any(|q| q == "httpx.Client"));
    let by_value = origin.value.is_some_and(|v| call_target(v).is_some_and(|c| qualified_name(c, model, src).is_some_and(|q| q == "httpx.Client")));
    if by_annotation || by_value {
        out.push(at(f, format!("Blocking httpx method {}.{}() in async context, use httpx.AsyncClient", text(obj, src), text(attr, src)), Vec::new()));
    }
}

py_rule!(BlockingHttpCallHttpxInAsyncFunction, "py/blocking-http-call-httpx-in-async-function", ["call"], true, blocking_http_call_httpx);


// ---------------------------------------------------------------- literal types

fn root_of(mut n: Node<'_>) -> Node<'_> {
    while let Some(p) = n.parent() {
        n = p;
    }
    n
}

/// A module-level `def name` or `class name`, unique in the file.
fn module_def<'t>(root: Node<'t>, name: &str, kind: &str, src: &str) -> Option<Node<'t>> {
    let mut found = None;
    for s in named_children(root) {
        let def = if s.kind() == "decorated_definition" { s.child_by_field_name("definition").unwrap_or(s) } else { s };
        if def.kind() == kind && def.child_by_field_name("name").is_some_and(|n| text(n, src) == name) {
            if found.is_some() {
                return None;
            }
            found = Some(def);
        }
    }
    found
}

const BUILTIN_TYPES: &[&str] = &["int", "float", "str", "bool", "list", "dict", "set", "frozenset", "tuple", "bytes", "complex", "object", "bytearray"];

/// The type name a value's expression gives, where the file states it:
/// a literal, a comprehension, a ternary whose arms agree, a call of a
/// builtin type or of a class of this file, a call of a function of this
/// file whose every `return` states the same type. `None` for anything else
/// and for `None` itself.
fn literal_type(n: Node<'_>, model: &SemanticModel, src: &str, root: Node<'_>, depth: usize) -> Option<String> {
    let n = unparen(n);
    let t = |s: &str| Some(s.to_owned());
    match n.kind() {
        "integer" => t("int"),
        "float" => t("float"),
        "true" | "false" => t("bool"),
        "string" | "concatenated_string" => {
            let bytes = if n.kind() == "string" { crate::python_util::string_parts(n, src).is_some_and(|(p, _)| p.contains('b')) } else { false };
            t(if bytes { "bytes" } else { "str" })
        }
        "list" | "list_comprehension" => t("list"),
        "dictionary" | "dictionary_comprehension" => t("dict"),
        "set" | "set_comprehension" => t("set"),
        "tuple" => t("tuple"),
        "generator_expression" => t("generator"),
        "unary_operator" => {
            let op = n.child_by_field_name("operator").map(|o| text(o, src))?;
            let arg = n.child_by_field_name("argument")?;
            if op == "-" && matches!(arg.kind(), "integer" | "float") { literal_type(arg, model, src, root, depth) } else { None }
        }
        "conditional_expression" => {
            let ks = kids(n);
            let (a, b) = (ks.first()?, ks.last()?);
            let (x, y) = (literal_type(*a, model, src, root, depth)?, literal_type(*b, model, src, root, depth)?);
            (x == y).then_some(x)
        }
        "call" => {
            let f = n.child_by_field_name("function").filter(|f| f.kind() == "identifier")?;
            let name = text(f, src);
            if module_def(root, name, "class_definition", src).is_some() {
                return Some(name.to_owned());
            }
            if let Some(def) = module_def(root, name, "function_definition", src) {
                if depth > 2 || text(def, src).starts_with("async") {
                    return None;
                }
                let body = def.child_by_field_name("body")?;
                let mut rets = Vec::new();
                collect_returns(body, &mut rets);
                let mut ty: Option<String> = None;
                for r in &rets {
                    let v = r.named_child(0)?;
                    let this = literal_type(v, model, src, root, depth + 1)?;
                    if ty.as_ref().is_some_and(|x| *x != this) {
                        return None;
                    }
                    ty = Some(this);
                }
                return if has_yield_in(body) { None } else { ty };
            }
            (BUILTIN_TYPES.contains(&name) && qualified_name(f, model, src).is_some_and(|q| q == format!("builtins.{name}"))).then(|| name.to_owned())
        }
        _ => None,
    }
}

fn collect_returns<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for k in named_children(n) {
        match k.kind() {
            "return_statement" => out.push(k),
            "function_definition" | "class_definition" | "lambda" => {}
            _ => collect_returns(k, out),
        }
    }
}

fn has_yield_in(n: Node<'_>) -> bool {
    named_children(n).into_iter().any(|k| match k.kind() {
        "yield" => true,
        "function_definition" | "class_definition" | "lambda" => false,
        _ => has_yield_in(k),
    })
}

// ---------------------------------------------------------------- redefined-variable-type

type Types = std::collections::BTreeMap<String, String>;

/// Walk the statements of one scope in order, keeping the type each name or
/// `obj.attr` target last got from a literal assignment. Branches start from
/// the state before them; after them a name holds what the last branch that
/// assigned it gave it.
fn walk_types<'t>(stmts: Vec<Node<'t>>, state: &mut Types, cx: &TypeCx<'t, '_>, out: &mut Vec<NativeFinding>) {
    for st in stmts {
        let st = if st.kind() == "decorated_definition" { st.child_by_field_name("definition").unwrap_or(st) } else { st };
        match st.kind() {
            "expression_statement" => {
                for a in named_children(st) {
                    if a.kind() == "assignment" {
                        assign_types(a, state, cx, out);
                    }
                }
            }
            "if_statement" => {
                let mut branches: Vec<Vec<Node<'t>>> = Vec::new();
                if let Some(c) = st.child_by_field_name("consequence") {
                    branches.push(kids(c));
                }
                for k in named_children(st) {
                    match k.kind() {
                        "elif_clause" => branches.push(k.child_by_field_name("consequence").map(kids).unwrap_or_default()),
                        "else_clause" => branches.push(k.child_by_field_name("body").map(kids).unwrap_or_default()),
                        _ => {}
                    }
                }
                branch_types(branches, state, cx, out);
            }
            "for_statement" | "while_statement" => {
                let mut branches = vec![st.child_by_field_name("body").map(kids).unwrap_or_default()];
                if let Some(e) = named_children(st).into_iter().find(|k| k.kind() == "else_clause") {
                    branches.push(e.child_by_field_name("body").map(kids).unwrap_or_default());
                }
                branch_types(branches, state, cx, out);
            }
            "try_statement" => {
                if let Some(b) = st.child_by_field_name("body") {
                    walk_types(kids(b), state, cx, out);
                }
                let mut branches = Vec::new();
                for k in named_children(st) {
                    match k.kind() {
                        "except_clause" | "except_group_clause" => {
                            branches.push(kids(k).into_iter().find(|c| c.kind() == "block").map(kids).unwrap_or_default());
                        }
                        _ => {}
                    }
                }
                branch_types(branches, state, cx, out);
                for k in named_children(st) {
                    if matches!(k.kind(), "else_clause" | "finally_clause") {
                        let b = k.child_by_field_name("body").or_else(|| kids(k).into_iter().find(|c| c.kind() == "block"));
                        if let Some(b) = b {
                            walk_types(kids(b), state, cx, out);
                        }
                    }
                }
            }
            "with_statement" => {
                if let Some(b) = st.child_by_field_name("body") {
                    walk_types(kids(b), state, cx, out);
                }
            }
            "match_statement" => {
                let branches: Vec<Vec<Node<'t>>> = named_children(st)
                    .into_iter()
                    .flat_map(|k| if k.kind() == "block" { kids(k) } else { vec![k] })
                    .filter(|k| k.kind() == "case_clause")
                    .map(|k| k.child_by_field_name("consequence").map(kids).unwrap_or_default())
                    .collect();
                branch_types(branches, state, cx, out);
            }
            "function_definition" | "class_definition" => {
                if let Some(b) = st.child_by_field_name("body") {
                    let mut fresh = Types::new();
                    walk_types(kids(b), &mut fresh, cx, out);
                }
            }
            _ => {}
        }
    }
}

fn branch_types<'t>(branches: Vec<Vec<Node<'t>>>, state: &mut Types, cx: &TypeCx<'t, '_>, out: &mut Vec<NativeFinding>) {
    let pre = state.clone();
    let mut merged = pre.clone();
    for b in branches {
        let mut s = pre.clone();
        walk_types(b, &mut s, cx, out);
        for (k, v) in s {
            if pre.get(&k) != Some(&v) {
                merged.insert(k, v);
            }
        }
    }
    *state = merged;
}

struct TypeCx<'t, 'm> {
    model: &'m SemanticModel,
    src: &'t str,
    root: Node<'t>,
}

fn assign_types<'t>(a: Node<'t>, state: &mut Types, cx: &TypeCx<'t, '_>, out: &mut Vec<NativeFinding>) {
    let (Some(l), Some(r)) = (a.child_by_field_name("left"), a.child_by_field_name("right")) else { return };
    if a.child_by_field_name("type").is_some() {
        return;
    }
    // `a = b = value`: the inner assignment is the right side.
    let value = if r.kind() == "assignment" {
        assign_types(r, state, cx, out);
        let mut v = r;
        while let Some(x) = v.child_by_field_name("right") {
            v = x;
            if v.kind() != "assignment" {
                break;
            }
        }
        v
    } else {
        r
    };
    let key = match l.kind() {
        "identifier" => {
            let k = text(l, cx.src);
            if k == "_" {
                return;
            }
            k.to_owned()
        }
        "attribute" if l.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier") => text(l, cx.src).to_owned(),
        _ => return,
    };
    let Some(ty) = literal_type(value, cx.model, cx.src, cx.root, 0) else { return };
    if let Some(prev) = state.get(&key)
        && *prev != ty
    {
        out.push(at(a, format!("Redefinition of `{key}` type from `{prev}` to `{ty}`"), Vec::new()));
    }
    state.insert(key, ty);
}

fn redefined_variable_type(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(model) = ctx.semantic else { return };
    let cx = TypeCx { model, src: ctx.source, root: n };
    let mut state = Types::new();
    walk_types(kids(n), &mut state, &cx, out);
}

py_rule!(RedefinedVariableType, "py/redefined-variable-type", ["module"], true, redefined_variable_type);

// ---------------------------------------------------------------- str.format fields

const ATTRS_INT: &[&str] = &["as_integer_ratio", "bit_count", "bit_length", "conjugate", "denominator", "from_bytes", "from_number", "imag", "is_integer", "numerator", "real", "to_bytes"];
const ATTRS_FLOAT: &[&str] = &["as_integer_ratio", "conjugate", "from_number", "fromhex", "hex", "imag", "is_integer", "real"];
const ATTRS_STR: &[&str] = &["capitalize", "casefold", "center", "count", "encode", "endswith", "expandtabs", "find", "format", "format_map", "index", "isalnum", "isalpha", "isascii", "isdecimal", "isdigit", "isidentifier", "islower", "isnumeric", "isprintable", "isspace", "istitle", "isupper", "join", "ljust", "lower", "lstrip", "maketrans", "partition", "removeprefix", "removesuffix", "replace", "rfind", "rindex", "rjust", "rpartition", "rsplit", "rstrip", "split", "splitlines", "startswith", "strip", "swapcase", "title", "translate", "upper", "zfill"];
const ATTRS_BYTES: &[&str] = &["capitalize", "center", "count", "decode", "endswith", "expandtabs", "find", "fromhex", "hex", "index", "isalnum", "isalpha", "isascii", "isdigit", "islower", "isspace", "istitle", "isupper", "join", "ljust", "lower", "lstrip", "maketrans", "partition", "removeprefix", "removesuffix", "replace", "rfind", "rindex", "rjust", "rpartition", "rsplit", "rstrip", "split", "splitlines", "startswith", "strip", "swapcase", "title", "translate", "upper", "zfill"];
const ATTRS_LIST: &[&str] = &["append", "clear", "copy", "count", "extend", "index", "insert", "pop", "remove", "reverse", "sort"];
const ATTRS_TUPLE: &[&str] = &["count", "index"];
const ATTRS_DICT: &[&str] = &["clear", "copy", "fromkeys", "get", "items", "keys", "pop", "popitem", "setdefault", "update", "values"];
const ATTRS_SET: &[&str] = &["add", "clear", "copy", "difference", "difference_update", "discard", "intersection", "intersection_update", "isdisjoint", "issubset", "issuperset", "pop", "remove", "symmetric_difference", "symmetric_difference_update", "union", "update"];


fn attrs_of(ty: &str) -> Option<&'static [&'static str]> {
    Some(match ty {
        "int" | "bool" => ATTRS_INT,
        "float" => ATTRS_FLOAT,
        "str" => ATTRS_STR,
        "bytes" => ATTRS_BYTES,
        "list" => ATTRS_LIST,
        "tuple" => ATTRS_TUPLE,
        "dict" => ATTRS_DICT,
        "set" => ATTRS_SET,
        _ => return None,
    })
}

#[derive(Debug)]
enum FKey {
    Auto(usize),
    Index(usize),
    Name(String),
}

#[derive(Debug)]
enum FStep {
    Attr(String),
    Item(String),
}

#[derive(Debug)]
struct FField {
    key: FKey,
    steps: Vec<FStep>,
    /// The field as written: `a.b[0]`.
    written: String,
}

/// The replacement fields of a `str.format` string, nested ones in a format
/// spec included. `None` when the string is not well formed.
fn format_fields(s: &str) -> Option<Vec<FField>> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut auto = 0;
    fields_in(&chars, &mut out, &mut auto, 0)?;
    Some(out)
}

fn fields_in(chars: &[char], out: &mut Vec<FField>, auto: &mut usize, depth: usize) -> Option<()> {
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '{' if chars.get(i + 1) == Some(&'{') && depth == 0 => i += 2,
            '}' if chars.get(i + 1) == Some(&'}') && depth == 0 => i += 2,
            '}' => return None,
            '{' => {
                let start = i + 1;
                let mut level = 1;
                i += 1;
                while i < chars.len() && level > 0 {
                    match chars[i] {
                        '{' => level += 1,
                        '}' => level -= 1,
                        _ => {}
                    }
                    i += 1;
                }
                if level > 0 {
                    return None;
                }
                let body: &[char] = &chars[start..i - 1];
                // The name ends at `!` or `:` outside square brackets.
                let mut j = 0;
                let mut in_br = false;
                while j < body.len() {
                    match body[j] {
                        '[' => in_br = true,
                        ']' => in_br = false,
                        '!' | ':' if !in_br => break,
                        _ => {}
                    }
                    j += 1;
                }
                let name: String = body[..j].iter().collect();
                out.push(parse_field(&name, auto)?);
                if let Some(c) = body.get(j)
                    && *c == ':'
                {
                    fields_in(&body[j + 1..], out, auto, depth + 1)?;
                } else if body.get(j) == Some(&'!') {
                    let rest = &body[j..];
                    if let Some(k) = rest.iter().position(|c| *c == ':') {
                        fields_in(&rest[k + 1..], out, auto, depth + 1)?;
                    }
                }
            }
            _ => i += 1,
        }
    }
    Some(())
}

fn parse_field(name: &str, auto: &mut usize) -> Option<FField> {
    let cs: Vec<char> = name.chars().collect();
    let mut i = 0;
    while i < cs.len() && cs[i] != '.' && cs[i] != '[' {
        i += 1;
    }
    let head: String = cs[..i].iter().collect();
    let key = if head.is_empty() {
        *auto += 1;
        FKey::Auto(*auto - 1)
    } else if head.chars().all(|c| c.is_ascii_digit()) {
        FKey::Index(head.parse().ok()?)
    } else {
        FKey::Name(head)
    };
    let mut steps = Vec::new();
    while i < cs.len() {
        if cs[i] == '.' {
            let st = i + 1;
            i = st;
            while i < cs.len() && cs[i] != '.' && cs[i] != '[' {
                i += 1;
            }
            if i == st {
                return None;
            }
            steps.push(FStep::Attr(cs[st..i].iter().collect()));
        } else if cs[i] == '[' {
            let st = i + 1;
            while i < cs.len() && cs[i] != ']' {
                i += 1;
            }
            if i >= cs.len() {
                return None;
            }
            if i == st {
                return None;
            }
            steps.push(FStep::Item(cs[st..i].iter().collect()));
            i += 1;
        } else {
            return None;
        }
    }
    Some(FField { key, steps, written: name.to_owned() })
}

/// What an argument is, where the file says so.
enum Shape<'t> {
    Scalar(&'static str),
    Str(usize),
    Seq(&'static str, Vec<Node<'t>>),
    Dict(Vec<(String, Node<'t>)>),
    Set,
    /// A builtin class passed as a value: not subscriptable.
    Class,
    Other(&'static str),
}

fn shape_of<'t>(n: Node<'t>, ctx: &NativeContext<'t>, model: &SemanticModel, depth: usize) -> Option<Shape<'t>> {
    let src = ctx.source;
    let mut n = unparen(n);
    if n.kind() == "identifier" {
        if qualified_name(n, model, src).is_some_and(|q| matches!(q.as_str(), "builtins.object" | "builtins.int" | "builtins.str" | "builtins.float" | "builtins.bool" | "builtins.bytes" | "builtins.complex")) {
            return Some(Shape::Class);
        }
        if depth > 0 {
            return None;
        }
        let (_, o) = only_origin(n, model, root_of(n))?;
        n = unparen(o.value?);
    }
    Some(match n.kind() {
        "integer" => Shape::Scalar("int"),
        "float" => Shape::Scalar("float"),
        "true" | "false" => Shape::Scalar("bool"),
        "none" => Shape::Scalar("None"),
        "string" | "concatenated_string" => {
            let v = crate::python_util::string_value(n, src);
            match v {
                Some(s) => Shape::Str(s.chars().count()),
                None => Shape::Other("str"),
            }
        }
        "list" | "tuple" => {
            let items = kids(n);
            if items.iter().any(|k| matches!(k.kind(), "list_splat" | "parenthesized_list_splat")) {
                Shape::Other(if n.kind() == "list" { "list" } else { "tuple" })
            } else {
                Shape::Seq(if n.kind() == "list" { "list" } else { "tuple" }, items)
            }
        }
        "dictionary" => {
            let mut pairs = Vec::new();
            for p in kids(n) {
                if p.kind() != "pair" {
                    return Some(Shape::Other("dict"));
                }
                let (k, v) = (p.child_by_field_name("key")?, p.child_by_field_name("value")?);
                let key = match k.kind() {
                    "integer" => text(k, src).to_owned(),
                    "string" => crate::python_util::string_value(k, src).map(|s| format!("'{s}'"))?,
                    _ => return Some(Shape::Other("dict")),
                };
                pairs.push((key, v));
            }
            Shape::Dict(pairs)
        }
        "set" => Shape::Set,
        _ => return None,
    })
}

fn format_attrs(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>, want_index: bool) {
    let Some(model) = ctx.semantic else { return };
    let src = ctx.source;
    let Some(f) = n.child_by_field_name("function").filter(|f| f.kind() == "attribute") else { return };
    if !f.child_by_field_name("attribute").is_some_and(|a| text(a, src) == "format") {
        return;
    }
    let Some(recv) = f.child_by_field_name("object") else { return };
    if !matches!(recv.kind(), "string" | "concatenated_string") {
        return;
    }
    let Some(fmt) = crate::python_util::string_value(recv, src) else { return };
    let a = call_args(n, src);
    if a.starred {
        return;
    }
    let Some(fields) = format_fields(&fmt) else { return };
    for fld in fields {
        if fld.steps.is_empty() {
            continue;
        }
        let arg = match &fld.key {
            FKey::Auto(i) | FKey::Index(i) => a.positional.get(*i).copied(),
            FKey::Name(k) => a.keywords.iter().find(|x| x.0 == *k).map(|x| x.1),
        };
        let Some(arg) = arg else { continue };
        let Some(mut cur) = shape_of(arg, ctx, model, 0) else { continue };
        for step in &fld.steps {
            match step {
                FStep::Attr(name) => {
                    if !want_index && !name.starts_with("__") {
                        let ty = match &cur {
                            Shape::Scalar(t) => attrs_of(t).map(|_| *t),
                            Shape::Str(_) | Shape::Other("str") => Some("str"),
                            Shape::Seq(t, _) => Some(*t),
                            Shape::Dict(_) | Shape::Other("dict") => Some("dict"),
                            Shape::Set => Some("set"),
                            Shape::Other(t) => attrs_of(t).map(|_| *t),
                            Shape::Class => None,
                        };
                        if let Some(t) = ty
                            && let Some(table) = attrs_of(t)
                            && !table.contains(&name.as_str())
                        {
                            out.push(at(n, format!("Missing format attribute `{name}` in format specifier `{}`", fld.written), Vec::new()));
                        }
                    }
                    break;
                }
                FStep::Item(key) => {
                    let bad = |out: &mut Vec<NativeFinding>| {
                        if want_index {
                            out.push(at(n, format!("Using invalid lookup key `{key}` in format specifier `{}`", fld.written), Vec::new()));
                        }
                    };
                    let idx = key.parse::<usize>().ok();
                    match cur {
                        Shape::Scalar(_) | Shape::Class | Shape::Set => {
                            bad(out);
                            break;
                        }
                        Shape::Str(len) => {
                            if idx.is_none_or(|i| i >= len) {
                                bad(out);
                                break;
                            }
                            cur = Shape::Str(1);
                        }
                        Shape::Seq(_, items) => match idx {
                            Some(i) if i < items.len() => match shape_of(items[i], ctx, model, 1) {
                                Some(s) => cur = s,
                                None => break,
                            },
                            _ => {
                                bad(out);
                                break;
                            }
                        },
                        Shape::Dict(pairs) => {
                            let want = if idx.is_some() { key.clone() } else { format!("'{key}'") };
                            match pairs.iter().find(|p| p.0 == want) {
                                Some((_, v)) => match shape_of(*v, ctx, model, 1) {
                                    Some(s) => cur = s,
                                    None => break,
                                },
                                None => {
                                    bad(out);
                                    break;
                                }
                            }
                        }
                        Shape::Other(_) => break,
                    }
                }
            }
        }
    }
}

fn invalid_format_index(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    format_attrs(n, ctx, out, true)
}

fn missing_format_attribute(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    format_attrs(n, ctx, out, false)
}

py_rule!(InvalidFormatIndex, "py/invalid-format-index", ["call"], true, invalid_format_index);
py_rule!(MissingFormatAttribute, "py/missing-format-attribute", ["call"], true, missing_format_attribute);

// ---------------------------------------------------------------- consider-using-namedtuple-or-dataclass

/// A dict key as a field name: a string literal, or a name or attribute of a
/// string constant this file assigns.
fn key_name(k: Node<'_>, src: &str, root: Node<'_>) -> Option<String> {
    let valid = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_') && !s.starts_with(|c: char| c.is_ascii_digit());
    match k.kind() {
        "string" => crate::python_util::string_value(k, src).filter(|s| valid(s)),
        "identifier" => {
            let name = text(k, src);
            for st in named_children(root) {
                if st.kind() == "expression_statement"
                    && let Some(a) = st.named_child(0).filter(|a| a.kind() == "assignment")
                    && a.child_by_field_name("left").is_some_and(|l| l.kind() == "identifier" && text(l, src) == name)
                    && let Some(r) = a.child_by_field_name("right")
                {
                    return crate::python_util::string_value(r, src).filter(|s| valid(s));
                }
            }
            None
        }
        "attribute" => {
            let (cls, attr) = (text(k.child_by_field_name("object")?, src), text(k.child_by_field_name("attribute")?, src));
            let def = module_def(root, cls, "class_definition", src)?;
            for st in kids(def.child_by_field_name("body")?) {
                if st.kind() == "expression_statement"
                    && let Some(a) = st.named_child(0).filter(|a| a.kind() == "assignment")
                    && a.child_by_field_name("left").is_some_and(|l| l.kind() == "identifier" && text(l, src) == attr)
                    && let Some(r) = a.child_by_field_name("right")
                {
                    return crate::python_util::string_value(r, src).filter(|s| valid(s));
                }
            }
            None
        }
        _ => None,
    }
}

fn consider_namedtuple_or_dataclass(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(d) = n.child_by_field_name("right").map(unparen).filter(|d| d.kind() == "dictionary") else { return };
    // Module level, or annotated `Final` anywhere.
    let final_annotated = n.child_by_field_name("type").is_some_and(|t| text(t, src).contains("Final"));
    let mut p = n.parent();
    let mut in_def = false;
    while let Some(x) = p {
        if matches!(x.kind(), "function_definition" | "class_definition" | "lambda") {
            in_def = true;
            break;
        }
        p = x.parent();
    }
    if in_def && !final_annotated {
        return;
    }
    let root = root_of(n);
    let entries = kids(d);
    if entries.len() < 2 || entries.iter().any(|e| e.kind() != "pair") {
        return;
    }
    let values: Vec<Node<'_>> = entries.iter().filter_map(|e| e.child_by_field_name("value")).map(unparen).collect();
    let first = values[0].kind();
    if !matches!(first, "dictionary" | "list" | "tuple") || values.iter().any(|v| v.kind() != first) {
        return;
    }
    let eligible = if first == "dictionary" {
        let mut common: Option<std::collections::BTreeSet<String>> = None;
        for v in &values {
            let mut keys = std::collections::BTreeSet::new();
            let ps = kids(*v);
            if ps.is_empty() {
                return;
            }
            for p in ps {
                let Some(k) = (p.kind() == "pair").then(|| p.child_by_field_name("key")).flatten().and_then(|k| key_name(k, src, root)) else { return };
                keys.insert(k);
            }
            common = Some(match common {
                None => keys,
                Some(c) => c.intersection(&keys).cloned().collect(),
            });
        }
        common.is_some_and(|c| !c.is_empty())
    } else {
        let len = kids(values[0]).len();
        len > 0
            && values.iter().all(|v| {
                let items = kids(*v);
                items.len() == len && !items.iter().any(|i| i.kind() == "dictionary")
            })
    };
    if eligible {
        out.push(at(d, "Consider using a namedtuple or dataclass for the dictionary's values", Vec::new()));
    }
}

py_rule!(ConsiderUsingNamedtupleOrDataclass, "py/consider-using-namedtuple-or-dataclass", ["assignment"], false, consider_namedtuple_or_dataclass);

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &DictIterMissingItems,
    &ModifiedIteratingSet,
    &BadStringFormatType,
    &ComparisonWithCallable,
    &PositionalOnlyArgumentsExpected,
    &BlockingHttpCallHttpxInAsyncFunction,
    &RedefinedVariableType,
    &InvalidFormatIndex,
    &MissingFormatAttribute,
    &ConsiderUsingNamedtupleOrDataclass,
];

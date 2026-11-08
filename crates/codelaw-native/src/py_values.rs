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

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &DictIterMissingItems,
    &ModifiedIteratingSet,
    &BadStringFormatType,
    &ComparisonWithCallable,
    &PositionalOnlyArgumentsExpected,
    &BlockingHttpCallHttpxInAsyncFunction,
];

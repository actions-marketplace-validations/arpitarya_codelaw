//! Python framework and security rows of W-139's tail, each Ruff 0.16.9's
//! documented behaviour reimplemented:
//!
//! - `sec-py/suspicious-subprocess-import` (S404),
//!   `sec-py/suspicious-httpoxy-import` (S412),
//!   `sec-py/suspicious-pyghmi-import` (S415),
//!   `sec-py/paramiko-call` (S601) and
//!   `sec-py/start-process-with-no-shell` (S606): calls and imports
//!   resolved through the file's imports;
//! - `airflow/airflow-task-branch-as-short-circuit` (AIR004),
//!   `airflow/airflow-task-implicit-multiple-outputs` (AIR202),
//!   `airflow/airflow3-incompatible-function-signature` (AIR303);
//! - `fastapi/fast-api-unused-path-parameter` (FAST003).

use crate::python_util::{call_args, decorators, kids, qualified_name, string_parts, PY};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{SemanticModel, SymbolKind};
use tree_sitter::Node;

pub const RULES: &[&dyn NativeRule] = &[
    &SuspiciousSubprocessImport,
    &SuspiciousHttpoxyImport,
    &SuspiciousPyghmiImport,
    &ParamikoCall,
    &StartProcessWithNoShell,
    &TaskBranchAsShortCircuit,
    &TaskImplicitMultipleOutputs,
    &IncompatibleFunctionSignature,
    &UnusedPathParameter,
];

fn hit(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

/// The root of the tree a node is in.
fn root_of(n: Node<'_>) -> Node<'_> {
    let mut r = n;
    while let Some(p) = r.parent() {
        r = p;
    }
    r
}

/// Whether the node is under an `if TYPE_CHECKING:` (or `typing.TYPE_CHECKING`) block.
fn in_type_checking(n: Node<'_>, src: &str) -> bool {
    let mut cur = n.parent();
    while let Some(p) = cur {
        if p.kind() == "if_statement"
            && p.child_by_field_name("condition").is_some_and(|c| matches!(text(c, src), "TYPE_CHECKING" | "typing.TYPE_CHECKING" | "t.TYPE_CHECKING"))
            && p.child_by_field_name("consequence").is_some_and(|b| b.start_byte() <= n.start_byte() && n.end_byte() <= b.end_byte())
        {
            return true;
        }
        cur = p.parent();
    }
    false
}

/// The definition a name binds exactly once in the file: its kind and the
/// declaring identifier.
fn only_binding<'t>(model: &SemanticModel, root: Node<'t>, ident: Node<'_>) -> Option<(SymbolKind, Node<'t>)> {
    let id = model.reference_at(ident.start_byte())?.symbol?;
    let sym = model.symbol(id);
    if sym.decls.len() != 1 {
        return None;
    }
    let (s, e) = sym.decls[0];
    Some((sym.kind, root.descendant_for_byte_range(s, e)?))
}

/// The function or class definition a name refers to, when it is bound once.
fn definition_of<'t>(model: &SemanticModel, root: Node<'t>, ident: Node<'_>, kind: &str) -> Option<Node<'t>> {
    let (_, decl) = only_binding(model, root, ident)?;
    let def = decl.parent()?;
    (def.kind() == kind && def.child_by_field_name("name").is_some_and(|n| n.id() == decl.id())).then_some(def)
}

/// The expression a name was assigned, when it is bound once by `name = value`.
fn assigned_value<'t>(model: &SemanticModel, root: Node<'t>, ident: Node<'_>) -> Option<Node<'t>> {
    let (kind, decl) = only_binding(model, root, ident)?;
    if kind != SymbolKind::Variable {
        return None;
    }
    let a = decl.parent().filter(|p| p.kind() == "assignment")?;
    (a.child_by_field_name("left").is_some_and(|l| l.id() == decl.id())).then(|| a.child_by_field_name("right")).flatten()
}

/// The expression of an annotation: the `type` wrapper stripped.
fn annotation_expr(t: Node<'_>) -> Node<'_> {
    if t.kind() == "type" { t.named_child(0).unwrap_or(t) } else { t }
}

/// The head of an annotation: `dict` of `dict[str, int]`.
fn annotation_head(t: Node<'_>) -> Node<'_> {
    let e = annotation_expr(t);
    match e.kind() {
        "generic_type" => e.named_child(0).unwrap_or(e),
        "subscript" => e.child_by_field_name("value").unwrap_or(e),
        _ => e,
    }
}

/// The type arguments of `Annotated[…]`, when the annotation is that.
fn annotated_parts<'t>(t: Node<'t>, model: &SemanticModel, src: &str) -> Option<Vec<Node<'t>>> {
    let e = annotation_expr(t);
    let qn = qualified_name(annotation_head(t), model, src)?;
    if !matches!(qn.as_str(), "typing.Annotated" | "typing_extensions.Annotated") {
        return None;
    }
    match e.kind() {
        "generic_type" => {
            let tp = named_children(e).into_iter().find(|c| c.kind() == "type_parameter")?;
            Some(named_children(tp).into_iter().filter(|c| c.kind() != "comment").map(annotation_expr).collect())
        }
        "subscript" => Some(named_children(e).into_iter().skip(1).filter(|c| c.kind() != "comment").map(annotation_expr).collect()),
        _ => None,
    }
}

/// A call decorator or a bare decorator's expression: `@x.y(…)` and `@x.y` give `x.y`.
fn callable_of(d: Node<'_>) -> Node<'_> {
    if d.kind() == "call" { d.child_by_field_name("function").unwrap_or(d) } else { d }
}

// ------------------------------------------------------------------ parameters

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParamKind {
    PositionalOnly,
    Regular,
    KeywordOnly,
    VarArgs,
}

struct Param<'t> {
    name: &'t str,
    kind: ParamKind,
    default: Option<Node<'t>>,
    annotation: Option<Node<'t>>,
    node: Node<'t>,
}

fn params_of<'t>(def: Node<'t>, src: &'t str) -> Vec<Param<'t>> {
    let Some(list) = def.child_by_field_name("parameters") else { return Vec::new() };
    let mut out: Vec<Param<'t>> = Vec::new();
    let mut keyword_only = false;
    for c in kids(list) {
        let (name, default, annotation) = match c.kind() {
            "identifier" => (text(c, src), None, None),
            "typed_parameter" => (c.named_child(0).filter(|n| n.kind() == "identifier").map(|n| text(n, src)).unwrap_or(""), None, c.child_by_field_name("type")),
            "default_parameter" => (c.child_by_field_name("name").map(|n| text(n, src)).unwrap_or(""), c.child_by_field_name("value"), None),
            "typed_default_parameter" => (c.child_by_field_name("name").map(|n| text(n, src)).unwrap_or(""), c.child_by_field_name("value"), c.child_by_field_name("type")),
            "positional_separator" => {
                for p in out.iter_mut() {
                    if p.kind == ParamKind::Regular {
                        p.kind = ParamKind::PositionalOnly;
                    }
                }
                continue;
            }
            "keyword_separator" => {
                keyword_only = true;
                continue;
            }
            "list_splat_pattern" | "dictionary_splat_pattern" => {
                if c.kind() == "list_splat_pattern" {
                    keyword_only = true;
                }
                out.push(Param { name: "", kind: ParamKind::VarArgs, default: None, annotation: None, node: c });
                continue;
            }
            _ => continue,
        };
        out.push(Param { name, kind: if keyword_only { ParamKind::KeywordOnly } else { ParamKind::Regular }, default, annotation, node: c });
    }
    out
}

/// The function definition and the decorators of a `decorated_definition`.
fn decorated_function(n: Node<'_>) -> Option<Node<'_>> {
    n.child_by_field_name("definition").filter(|d| d.kind() == "function_definition")
}

/// The statements a block holds, for the walk below.
fn walk_returns<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    for c in named_children(n) {
        match c.kind() {
            "function_definition" | "class_definition" | "lambda" => {}
            "return_statement" => out.push(c),
            _ => walk_returns(c, out),
        }
    }
}

/// The `return` statements of a function body, nested functions and classes left out.
fn returns_of(def: Node<'_>) -> Vec<Node<'_>> {
    let mut out = Vec::new();
    if let Some(b) = def.child_by_field_name("body") {
        walk_returns(b, &mut out);
    }
    out
}

// ------------------------------------------------------------ suspicious imports

/// Whether a statement imports `module` itself (`import module`, `from module import …`).
fn imports_module<'t>(stmt: Node<'t>, src: &str, module: &str) -> Option<Node<'t>> {
    match stmt.kind() {
        "import_statement" => named_children(stmt).into_iter().filter(|c| matches!(c.kind(), "dotted_name" | "aliased_import")).map(|c| if c.kind() == "aliased_import" { c.child_by_field_name("name").unwrap_or(c) } else { c }).find(|c| text(*c, src) == module),
        "import_from_statement" => stmt.child_by_field_name("module_name").filter(|m| m.kind() == "dotted_name" && text(*m, src) == module),
        _ => None,
    }
}

/// The names a `from … import …` statement binds, as written.
fn imported_names<'s>(stmt: Node<'_>, src: &'s str) -> Vec<&'s str> {
    let module = stmt.child_by_field_name("module_name").map(|m| m.id());
    named_children(stmt)
        .into_iter()
        .filter(|c| Some(c.id()) != module && matches!(c.kind(), "dotted_name" | "aliased_import"))
        .map(|c| if c.kind() == "aliased_import" { c.child_by_field_name("name").unwrap_or(c) } else { c })
        .map(|c| text(c, src))
        .collect()
}

fn skip_import(stmt: Node<'_>, ctx: &NativeContext<'_>) -> bool {
    ctx.file.path().ends_with(".pyi") || in_type_checking(stmt, ctx.source)
}

macro_rules! import_rule {
    ($name:ident, $id:literal, $message:literal, |$stmt:ident, $src:ident| $find:expr) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &["import_statement", "import_from_statement"]
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                let $stmt = node.get_inner_node();
                let $src = ctx.source;
                if skip_import($stmt, ctx) {
                    return;
                }
                let found: Option<Node<'_>> = $find;
                if let Some(at) = found {
                    out.push(hit(at, $message, vec![]));
                }
            }
        }
    };
}

import_rule!(SuspiciousSubprocessImport, "sec-py/suspicious-subprocess-import", "`subprocess` module is possibly insecure", |stmt, src| imports_module(stmt, src, "subprocess"));

import_rule!(SuspiciousPyghmiImport, "sec-py/suspicious-pyghmi-import", "An IPMI-related module is being imported. Prefer an encrypted protocol over IPMI.", |stmt, src| imports_module(stmt, src, "pyghmi"));

import_rule!(
    SuspiciousHttpoxyImport,
    "sec-py/suspicious-httpoxy-import",
    "`httpoxy` is a set of vulnerabilities that affect application code running in CGI, or CGI-like environments. The use of CGI for web applications should be avoided",
    |stmt, src| {
        if stmt.kind() != "import_from_statement" {
            None
        } else {
            let module = stmt.child_by_field_name("module_name").filter(|m| m.kind() == "dotted_name");
            module.filter(|m| {
                let names = imported_names(stmt, src);
                match text(*m, src) {
                    "wsgiref.handlers" => names.contains(&"CGIHandler"),
                    "twisted.web.twcgi" => names.contains(&"CGIScript"),
                    _ => false,
                }
            })
        }
    }
);

// ------------------------------------------------------------------ calls

const OS_NO_SHELL: &[&str] = &[
    "os.execl", "os.execle", "os.execlp", "os.execlpe", "os.execv", "os.execve", "os.execvp", "os.execvpe", "os.spawnl", "os.spawnle", "os.spawnlp", "os.spawnlpe", "os.spawnv", "os.spawnve", "os.spawnvp", "os.spawnvpe", "os.startfile",
];

macro_rules! call_rule {
    ($name:ident, $id:literal, $message:literal, |$q:ident| $test:expr) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &["call"]
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                let Some(model) = ctx.semantic else { return };
                let call = node.get_inner_node();
                let Some(f) = call.child_by_field_name("function") else { return };
                let Some($q) = qualified_name(f, model, ctx.source) else { return };
                if $test {
                    out.push(hit(f, $message, vec![]));
                }
            }
        }
    };
}

call_rule!(ParamikoCall, "sec-py/paramiko-call", "Possible shell injection via Paramiko call; check inputs are properly sanitized", |q| q == "paramiko.exec_command");

call_rule!(StartProcessWithNoShell, "sec-py/start-process-with-no-shell", "Starting a process without a shell", |q| OS_NO_SHELL.contains(&q.as_str()));

// -------------------------------------------------------------- AIR004, AIR202

fn is_task_module(q: &str) -> bool {
    matches!(q, "airflow.decorators.task" | "airflow.sdk.task")
}

/// `@task.<variant>` or `@task.<variant>()` with `task` from Airflow.
fn is_task_variant(d: Node<'_>, model: &SemanticModel, src: &str, variants: &dyn Fn(&str) -> bool) -> bool {
    let e = callable_of(d);
    if e.kind() != "attribute" {
        return false;
    }
    let (Some(obj), Some(attr)) = (e.child_by_field_name("object"), e.child_by_field_name("attribute")) else { return false };
    variants(text(attr, src)) && qualified_name(obj, model, src).is_some_and(|q| is_task_module(&q))
}

/// Two or more `return`s, exactly one of which returns a non-empty list.
fn could_be_short_circuit(def: Node<'_>) -> bool {
    let rets = returns_of(def);
    if rets.len() < 2 {
        return false;
    }
    rets.iter().filter(|r| r.named_child(0).is_some_and(|v| v.kind() == "list" && !kids(v).is_empty())).count() == 1
}

pub struct TaskBranchAsShortCircuit;

impl NativeRule for TaskBranchAsShortCircuit {
    fn id(&self) -> &'static str {
        "airflow/airflow-task-branch-as-short-circuit"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["decorated_definition", "call"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        if n.kind() == "decorated_definition" {
            let Some(def) = decorated_function(n) else { return };
            if decorators(def).iter().any(|d| is_task_variant(*d, model, src, &|v| v == "branch")) && could_be_short_circuit(def) {
                out.push(hit(n, "`@task.branch` can be replaced with `@task.short_circuit`", vec![]));
            }
            return;
        }
        let Some(f) = n.child_by_field_name("function") else { return };
        let Some(q) = qualified_name(f, model, src) else { return };
        if !matches!(q.as_str(), "airflow.operators.python.BranchPythonOperator" | "airflow.operators.python_operator.BranchPythonOperator" | "airflow.providers.standard.operators.python.BranchPythonOperator") {
            return;
        }
        let args = call_args(n, src);
        let Some(callable) = args.keyword("python_callable").filter(|v| v.kind() == "identifier") else { return };
        let Some(def) = definition_of(model, root_of(n), callable, "function_definition") else { return };
        if could_be_short_circuit(def) {
            out.push(hit(f, "`BranchPythonOperator` can be replaced with `ShortCircuitOperator`", vec![]));
        }
    }
}

const TASK_VARIANTS: &[&str] = &["python", "virtualenv", "external_python", "branch", "branch_virtualenv", "branch_external_python", "short_circuit", "docker", "kubernetes", "pyspark"];

/// A decorator Airflow infers `multiple_outputs` for: `@task` or `@task.<variant>`.
fn is_supported_task_decorator(d: Node<'_>, model: &SemanticModel, src: &str) -> bool {
    let e = callable_of(d);
    if qualified_name(e, model, src).is_some_and(|q| is_task_module(&q)) {
        return true;
    }
    is_task_variant(d, model, src, &|v| TASK_VARIANTS.contains(&v))
}

const MAPPING_TYPES: &[&str] = &[
    "typing.Dict", "typing.Mapping", "typing.MutableMapping", "typing.OrderedDict", "typing.DefaultDict", "typing.Counter", "typing.ChainMap", "typing.TypedDict", "typing_extensions.Dict", "typing_extensions.Mapping", "typing_extensions.MutableMapping", "typing_extensions.OrderedDict", "typing_extensions.DefaultDict", "typing_extensions.Counter",
    "typing_extensions.ChainMap", "typing_extensions.TypedDict", "builtins.dict", "collections.abc.Mapping", "collections.abc.MutableMapping", "collections.OrderedDict", "collections.defaultdict", "collections.Counter", "collections.ChainMap",
];

/// Whether a class (by name) derives from `TypedDict`, through classes of this file.
fn is_typed_dict_class(model: &SemanticModel, root: Node<'_>, ident: Node<'_>, src: &str, depth: usize) -> bool {
    if depth > 5 {
        return false;
    }
    let Some(class) = definition_of(model, root, ident, "class_definition") else { return false };
    let Some(bases) = class.child_by_field_name("superclasses") else { return false };
    kids(bases).into_iter().any(|b| qualified_name(b, model, src).is_some_and(|q| matches!(q.as_str(), "typing.TypedDict" | "typing_extensions.TypedDict")) || b.kind() == "identifier" && is_typed_dict_class(model, root, b, src, depth + 1))
}

fn annotation_is_mapping(def: Node<'_>, model: &SemanticModel, src: &str) -> bool {
    let Some(ret) = def.child_by_field_name("return_type") else { return false };
    let head = annotation_head(ret);
    qualified_name(head, model, src).is_some_and(|q| MAPPING_TYPES.contains(&q.as_str())) || head.kind() == "identifier" && is_typed_dict_class(model, root_of(def), head, src, 0)
}

fn body_returns_dict(def: Node<'_>, model: &SemanticModel, src: &str) -> bool {
    returns_of(def).into_iter().filter_map(|r| r.named_child(0)).any(|v| match v.kind() {
        "dictionary" | "dictionary_comprehension" => true,
        "call" => v.child_by_field_name("function").and_then(|f| qualified_name(f, model, src)).is_some_and(|q| q == "builtins.dict"),
        _ => false,
    })
}

/// The edit that adds `keyword` to a decorator: before the first keyword, after the last
/// argument, or inside empty parentheses; a bare decorator becomes a call.
fn add_keyword(d: Node<'_>, keyword: &str, src: &str) -> TextEdit {
    if d.kind() != "call" {
        return TextEdit::insert(d.end_byte(), format!("({keyword})"));
    }
    let args = call_args(d, src);
    if let Some((_, _, first)) = args.keywords.first() {
        return TextEdit::insert(first.start_byte(), format!("{keyword}, "));
    }
    if let Some(last) = args.list.map(kids).and_then(|k| k.last().copied()) {
        return TextEdit::insert(last.end_byte(), format!(", {keyword}"));
    }
    match args.list {
        Some(list) => TextEdit::insert(list.start_byte() + 1, keyword.to_owned()),
        None => TextEdit::insert(d.end_byte(), format!("({keyword})")),
    }
}

pub struct TaskImplicitMultipleOutputs;

impl NativeRule for TaskImplicitMultipleOutputs {
    fn id(&self) -> &'static str {
        "airflow/airflow-task-implicit-multiple-outputs"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["decorated_definition"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        let Some(def) = decorated_function(n) else { return };
        let Some(d) = decorators(def).into_iter().find(|d| is_supported_task_decorator(*d, model, src)) else { return };
        if d.kind() == "call" && call_args(d, src).keyword("multiple_outputs").is_some() {
            return;
        }
        let mapping = annotation_is_mapping(def, model, src);
        if !mapping && !body_returns_dict(def, model, src) {
            return;
        }
        let keyword = if mapping { "multiple_outputs=True" } else { "multiple_outputs=False" };
        let at = d.parent().filter(|p| p.kind() == "decorator").unwrap_or(d);
        out.push(hit(at, "`@task`-decorated function relies on `multiple_outputs` inference", vec![add_keyword(d, keyword, src)]));
    }
}

// -------------------------------------------------------------------- AIR303

/// Whether a class (by its definition node) derives from `BaseOperatorLink`.
fn derives_operator_link(model: &SemanticModel, class: Node<'_>, src: &str, depth: usize) -> bool {
    if depth > 5 {
        return false;
    }
    let Some(bases) = class.child_by_field_name("superclasses") else { return false };
    kids(bases).into_iter().any(|b| {
        qualified_name(b, model, src).is_some_and(|q| {
            let parts: Vec<&str> = q.split('.').collect();
            parts.len() >= 3 && parts[0] == "airflow" && matches!(parts[1], "models" | "sdk") && parts[parts.len() - 1] == "BaseOperatorLink"
        }) || b.kind() == "identifier" && definition_of(model, root_of(class), b, "class_definition").is_some_and(|c| derives_operator_link(model, c, src, depth + 1))
    })
}

fn is_dict_expression(e: Node<'_>, model: &SemanticModel, src: &str) -> bool {
    match e.kind() {
        "dictionary" | "dictionary_comprehension" => true,
        "call" => e.child_by_field_name("function").and_then(|f| qualified_name(f, model, src)).is_some_and(|q| q == "builtins.dict"),
        "identifier" => assigned_value(model, root_of(e), e).is_some_and(|v| matches!(v.kind(), "dictionary" | "dictionary_comprehension") || v.kind() == "call" && v.child_by_field_name("function").and_then(|f| qualified_name(f, model, src)).is_some_and(|q| q == "builtins.dict")),
        _ => false,
    }
}

pub struct IncompatibleFunctionSignature;

impl NativeRule for IncompatibleFunctionSignature {
    fn id(&self) -> &'static str {
        "airflow/airflow3-incompatible-function-signature"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call", "function_definition"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        if n.kind() == "function_definition" {
            self.check_get_link(n, model, src, out);
            return;
        }
        let Some(f) = n.child_by_field_name("function") else { return };
        if f.kind() == "attribute" {
            let (Some(obj), Some(attr)) = (f.child_by_field_name("object"), f.child_by_field_name("attribute")) else { return };
            if text(attr, src) != "create_asset" {
                return;
            }
            let class = if obj.kind() == "call" {
                obj.child_by_field_name("function").and_then(|g| qualified_name(g, model, src))
            } else if obj.kind() == "identifier" {
                assigned_value(model, root_of(n), obj).filter(|v| v.kind() == "call").and_then(|v| v.child_by_field_name("function")).and_then(|g| qualified_name(g, model, src))
            } else {
                None
            };
            let positional = n.child_by_field_name("arguments").is_some_and(|a| kids(a).iter().any(|c| !matches!(c.kind(), "keyword_argument" | "dictionary_splat") && c.kind() != "comment"));
            if class.as_deref() == Some("airflow.lineage.hook.HookLineageCollector") && positional {
                out.push(hit(attr, "`create_asset` signature is changed in Airflow 3.0", vec![]));
            }
            return;
        }
        let Some(q) = qualified_name(f, model, src) else { return };
        if !matches!(q.as_str(), "airflow.Dataset" | "airflow.datasets.Dataset" | "airflow.sdk.Asset") {
            return;
        }
        let args = call_args(n, src);
        if let Some(second) = args.positional.get(1)
            && is_dict_expression(*second, model, src)
        {
            let name = q.rsplit('.').next().unwrap_or("");
            out.push(hit(f, format!("`{name}` signature is changed in Airflow 3.0"), vec![]));
        }
    }
}

impl IncompatibleFunctionSignature {
    fn check_get_link(&self, def: Node<'_>, model: &SemanticModel, src: &str, out: &mut Vec<NativeFinding>) {
        if def.child_by_field_name("name").is_none_or(|nm| text(nm, src) != "get_link") {
            return;
        }
        // A method of a class that derives from `BaseOperatorLink`.
        let Some(class) = def.parent().filter(|b| b.kind() == "block").and_then(|b| b.parent()).filter(|c| c.kind() == "class_definition") else { return };
        if !derives_operator_link(model, class, src, 0) {
            return;
        }
        let params = params_of(def, src);
        let positional: Vec<&Param<'_>> = params.iter().filter(|p| matches!(p.kind, ParamKind::PositionalOnly | ParamKind::Regular)).collect();
        let valid = match positional.len() {
            2 => params.iter().any(|p| p.kind == ParamKind::KeywordOnly && p.name == "ti_key"),
            3 => positional[2].name == "ti_key",
            _ => false,
        };
        if !valid && let Some(name) = def.child_by_field_name("name") {
            out.push(hit(name, "`get_link` signature is changed in Airflow 3.0", vec![]));
        }
    }
}

// -------------------------------------------------------------------- FAST003

const ROUTE_METHODS: &[&str] = &["get", "post", "put", "delete", "patch", "options", "head", "trace"];

/// The path parameters of a route path: `{id}` and `{id:int}`, with their offsets
/// (the braces included) in the path text.
fn path_params(path: &str) -> Vec<(&str, usize, usize)> {
    let b = path.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let ident = |s: &[u8]| !s.is_empty() && (s[0].is_ascii_alphabetic() || s[0] == b'_') && s.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_');
    while i < b.len() {
        if b[i] == b'{'
            && let Some(close) = path[i..].find('}')
        {
            let inner = &path[i + 1..i + close];
            let (name, conv) = match inner.split_once(':') {
                Some((n, c)) => (n, Some(c)),
                None => (inner, None),
            };
            if ident(name.as_bytes()) && conv.is_none_or(|c| ident(c.as_bytes())) {
                out.push((name, i, i + close + 1));
                i += close + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn is_fastapi_app_type(q: &str) -> bool {
    matches!(q, "fastapi.FastAPI" | "fastapi.APIRouter" | "fastapi.applications.FastAPI" | "fastapi.routing.APIRouter")
}

/// Whether a name is a FastAPI app or router: assigned from `FastAPI(…)`/`APIRouter(…)`,
/// or annotated as one.
fn is_fastapi_app(model: &SemanticModel, root: Node<'_>, ident: Node<'_>, src: &str) -> bool {
    let Some((kind, decl)) = only_binding(model, root, ident) else { return false };
    let Some(parent) = decl.parent() else { return false };
    match (kind, parent.kind()) {
        (SymbolKind::Variable, "assignment") => {
            let by_call = parent.child_by_field_name("right").filter(|r| r.kind() == "call").and_then(|r| r.child_by_field_name("function")).and_then(|f| qualified_name(f, model, src)).is_some_and(|q| is_fastapi_app_type(&q));
            by_call || parent.child_by_field_name("type").map(annotation_head).and_then(|h| qualified_name(h, model, src)).is_some_and(|q| is_fastapi_app_type(&q))
        }
        (SymbolKind::Param, "typed_parameter" | "typed_default_parameter") => parent.child_by_field_name("type").map(annotation_head).and_then(|h| qualified_name(h, model, src)).is_some_and(|q| is_fastapi_app_type(&q)),
        _ => false,
    }
}

/// What a `Depends(…)` resolves to in this file.
enum Dependency<'t> {
    Unknown,
    Multiple,
    Names(Vec<&'t str>),
}

fn names_of_params<'t>(def: Node<'t>, src: &'t str) -> Vec<&'t str> {
    params_of(def, src).into_iter().filter(|p| matches!(p.kind, ParamKind::Regular | ParamKind::KeywordOnly)).map(|p| p.name).collect()
}

/// A class's parameter names for `method` (`__init__` or `__call__`): `self` skipped; a
/// Pydantic model's are its annotated fields.
fn class_params<'t>(model: &SemanticModel, class: Node<'t>, src: &'t str, method: &str) -> Dependency<'t> {
    let is_pydantic = class.child_by_field_name("superclasses").is_some_and(|b| kids(b).into_iter().any(|e| qualified_name(e, model, src).is_some_and(|q| q == "pydantic.BaseModel")));
    let Some(body) = class.child_by_field_name("body") else { return Dependency::Unknown };
    if is_pydantic {
        let names = kids(body)
            .into_iter()
            .filter_map(|s| s.named_child(0).filter(|a| s.kind() == "expression_statement" && a.kind() == "assignment" && a.child_by_field_name("type").is_some()))
            .filter_map(|a| a.child_by_field_name("left").filter(|l| l.kind() == "identifier"))
            .map(|l| text(l, src))
            .collect();
        return Dependency::Names(names);
    }
    let found = kids(body).into_iter().map(|s| if s.kind() == "decorated_definition" { s.child_by_field_name("definition").unwrap_or(s) } else { s }).find(|s| s.kind() == "function_definition" && s.child_by_field_name("name").is_some_and(|n| text(n, src) == method));
    match found {
        Some(m) => Dependency::Names(names_of_params(m, src).into_iter().skip(1).collect()),
        None => Dependency::Unknown,
    }
}

fn depends_call<'t>(e: Node<'t>, model: &SemanticModel, src: &str) -> Option<Node<'t>> {
    (e.kind() == "call" && e.child_by_field_name("function").and_then(|f| qualified_name(f, model, src)).is_some_and(|q| q == "fastapi.Depends")).then_some(e)
}

/// The `dependency` argument of a `Depends(…)` call: the first positional, or the keyword.
fn dependency_arg<'t>(call: Node<'t>, src: &str) -> Option<Node<'t>> {
    let args = call_args(call, src);
    args.positional.first().copied().or_else(|| args.keyword("dependency"))
}

fn dependency_from_call<'t>(call: Node<'t>, model: &SemanticModel, src: &'t str) -> Option<Dependency<'t>> {
    let arg = dependency_arg(call, src)?;
    let root = root_of(call);
    match arg.kind() {
        "identifier" => dependency_from_name(arg, model, root, src),
        "call" => {
            let f = arg.child_by_field_name("function").filter(|f| f.kind() == "identifier")?;
            match definition_of(model, root, f, "class_definition") {
                Some(class) => Some(class_params(model, class, src, "__call__")),
                None => Some(Dependency::Unknown),
            }
        }
        _ => None,
    }
}

fn dependency_from_name<'t>(name: Node<'t>, model: &SemanticModel, root: Node<'t>, src: &'t str) -> Option<Dependency<'t>> {
    if let Some(def) = definition_of(model, root, name, "function_definition") {
        return Some(Dependency::Names(names_of_params(def, src)));
    }
    if let Some(class) = definition_of(model, root, name, "class_definition") {
        return Some(class_params(model, class, src, "__init__"));
    }
    Some(Dependency::Unknown)
}

fn dependency_of<'t>(p: &Param<'t>, model: &SemanticModel, src: &'t str) -> Option<Dependency<'t>> {
    if let Some(call) = p.default.and_then(|d| depends_call(d, model, src))
        && let Some(dep) = dependency_from_call(call, model, src)
    {
        return Some(dep);
    }
    let parts = annotated_parts(p.annotation?, model, src)?;
    let mut found = parts.iter().skip(1).filter_map(|m| {
        let call = depends_call(*m, model, src)?;
        let empty = call.child_by_field_name("arguments").is_none_or(|a| kids(a).is_empty());
        if empty {
            let first = parts.first().filter(|f| f.kind() == "identifier")?;
            dependency_from_name(*first, model, root_of(call), src)
        } else {
            dependency_from_call(call, model, src)
        }
    });
    let first = found.next()?;
    if found.next().is_some() { Some(Dependency::Multiple) } else { Some(first) }
}

/// The alias an `Annotated[T, Path(alias="x")]` parameter goes by.
fn path_alias<'t>(p: &Param<'t>, model: &SemanticModel, src: &'t str) -> Option<String> {
    let parts = annotated_parts(p.annotation?, model, src)?;
    let path = parts.get(1).filter(|c| c.kind() == "call")?;
    if !path.child_by_field_name("function").and_then(|f| qualified_name(f, model, src)).is_some_and(|q| q == "fastapi.Path") {
        return None;
    }
    let alias = call_args(*path, src).keyword("alias")?;
    string_parts(alias, src).map(|(_, v)| v)
}

/// The edit that adds a parameter to a function: after the last regular parameter without a
/// default, else before the first regular one, else inside empty parentheses.
fn add_parameter(def: Node<'_>, name: &str, src: &str) -> Option<TextEdit> {
    let params = params_of(def, src);
    let regular: Vec<&Param<'_>> = params.iter().filter(|p| p.kind == ParamKind::Regular).collect();
    if let Some(last) = regular.iter().rfind(|p| p.default.is_none()) {
        return Some(TextEdit::insert(last.node.end_byte(), format!(", {name}")));
    }
    if let Some(first) = regular.first() {
        if params.iter().any(|p| p.kind == ParamKind::PositionalOnly && p.default.is_some()) {
            return None;
        }
        return Some(TextEdit::insert(first.node.start_byte(), format!("{name}, ")));
    }
    let list = def.child_by_field_name("parameters")?;
    params.is_empty().then(|| TextEdit::insert(list.start_byte() + 1, name.to_owned()))
}

pub struct UnusedPathParameter;

impl NativeRule for UnusedPathParameter {
    fn id(&self) -> &'static str {
        "fastapi/fast-api-unused-path-parameter"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["decorated_definition"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        let Some(def) = decorated_function(n) else { return };
        let root = root_of(n);
        // The first route decorator: `<app>.<method>("<path>", …)`.
        let route = decorators(def).into_iter().find(|d| {
            d.kind() == "call"
                && d.child_by_field_name("function").is_some_and(|f| {
                    f.kind() == "attribute" && f.child_by_field_name("attribute").is_some_and(|a| ROUTE_METHODS.contains(&text(a, src))) && f.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && is_fastapi_app(model, root, o, src))
                })
        });
        let Some(route) = route else { return };
        let Some(path_node) = call_args(route, src).positional.first().copied().filter(|p| p.kind() == "string") else { return };
        let Some((prefix, path)) = string_parts(path_node, src) else { return };
        if prefix.contains('b') || prefix.contains('f') {
            return;
        }
        let quote = if text(path_node, src)[prefix.len()..].starts_with("\"\"\"") || text(path_node, src)[prefix.len()..].starts_with("'''") { 3 } else { 1 };
        let content_start = path_node.start_byte() + prefix.len() + quote;
        let params = params_of(def, src);
        let mut named: Vec<String> = Vec::new();
        for p in params.iter().filter(|p| matches!(p.kind, ParamKind::Regular | ParamKind::KeywordOnly)) {
            named.push(path_alias(p, model, src).unwrap_or_else(|| p.name.to_owned()));
            let Some(dep) = dependency_of(p, model, src) else { continue };
            match dep {
                Dependency::Names(ns) => named.extend(ns.into_iter().map(str::to_owned)),
                // A dependency this rule cannot read: it may take any path parameter.
                Dependency::Unknown | Dependency::Multiple => return,
            }
        }
        let fname = def.child_by_field_name("name").map(|n| text(n, src)).unwrap_or("");
        for (param, s, e) in path_params(&path) {
            if named.iter().any(|n| n == param) {
                continue;
            }
            let positional = params.iter().any(|p| p.kind == ParamKind::PositionalOnly && p.name == param);
            let message = if positional { format!("Parameter `{param}` appears in route path, but only as a positional-only argument in `{fname}` signature") } else { format!("Parameter `{param}` appears in route path, but not in `{fname}` signature") };
            let edits = if !positional && param != "__debug__" { add_parameter(def, param, src).into_iter().collect() } else { Vec::new() };
            out.push(NativeFinding { start: content_start + s, end: content_start + e, message, edits });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;
    use codelaw_core::Language::Python;

    fn lines(rule: &dyn NativeRule, src: &str) -> Vec<String> {
        run(rule, Python, src).into_iter().map(|f| f.message).collect()
    }

    fn fixed(rule: &dyn NativeRule, src: &str) -> String {
        let f = run(rule, Python, src).remove(0);
        let mut s = src.to_owned();
        let mut edits = f.edits;
        edits.sort_by_key(|e| std::cmp::Reverse(e.start));
        for e in edits {
            s.replace_range(e.start..e.end, &e.text);
        }
        s
    }

    #[test]
    fn imports() {
        assert_eq!(lines(&SuspiciousSubprocessImport, "import subprocess\nfrom subprocess import run\nimport subprocess as sp\nimport os\n").len(), 3);
        assert!(lines(&SuspiciousSubprocessImport, "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n    import subprocess\n").is_empty());
        assert_eq!(lines(&SuspiciousHttpoxyImport, "from wsgiref.handlers import CGIHandler\nfrom wsgiref.handlers import x\nfrom twisted.web.twcgi import CGIScript\n").len(), 2);
        assert_eq!(lines(&SuspiciousPyghmiImport, "import pyghmi\nfrom pyghmi import x\nimport pyghmi.ipmi\n").len(), 2);
    }

    #[test]
    fn calls() {
        assert_eq!(lines(&StartProcessWithNoShell, "import os\nfrom os import execv\nos.execv('a', [])\nexecv('a', [])\nos.system('a')\n").len(), 2);
        assert_eq!(lines(&ParamikoCall, "import paramiko\nparamiko.exec_command('x')\nclient.exec_command('x')\n").len(), 1);
    }

    #[test]
    fn branch_as_short_circuit() {
        let src = "from airflow.decorators import task\n\n@task.branch\ndef f():\n    if c:\n        return ['a']\n    return []\n";
        assert_eq!(lines(&TaskBranchAsShortCircuit, src).len(), 1);
        let two = "from airflow.decorators import task\n\n@task.branch\ndef f():\n    if c:\n        return ['a']\n    return ['b']\n";
        assert!(lines(&TaskBranchAsShortCircuit, two).is_empty());
        let op = "from airflow.operators.python import BranchPythonOperator\n\ndef g():\n    if c:\n        return ['a']\n    return []\n\nt = BranchPythonOperator(task_id='t', python_callable=g)\n";
        assert_eq!(lines(&TaskBranchAsShortCircuit, op).len(), 1);
    }

    #[test]
    fn multiple_outputs() {
        let src = "from airflow.decorators import task\n\n@task\ndef f() -> dict[str, int]:\n    return {}\n";
        assert_eq!(fixed(&TaskImplicitMultipleOutputs, src), "from airflow.decorators import task\n\n@task(multiple_outputs=True)\ndef f() -> dict[str, int]:\n    return {}\n");
        let call = "from airflow.decorators import task\n\n@task(retries=1)\ndef f():\n    return dict(a=1)\n";
        assert_eq!(fixed(&TaskImplicitMultipleOutputs, call), "from airflow.decorators import task\n\n@task(multiple_outputs=False, retries=1)\ndef f():\n    return dict(a=1)\n");
        let given = "from airflow.decorators import task\n\n@task(multiple_outputs=True)\ndef f():\n    return {}\n";
        assert!(lines(&TaskImplicitMultipleOutputs, given).is_empty());
    }

    #[test]
    fn function_signatures() {
        let src = "from airflow.sdk import Asset\nAsset('a', {'k': 1})\nAsset('a', extra={'k': 1})\n";
        assert_eq!(lines(&IncompatibleFunctionSignature, src).len(), 1);
        let link = "from airflow.models import BaseOperatorLink\nclass L(BaseOperatorLink):\n    def get_link(self, operator, dttm):\n        return ''\nclass M(BaseOperatorLink):\n    def get_link(self, operator, *, ti_key):\n        return ''\n";
        assert_eq!(lines(&IncompatibleFunctionSignature, link).len(), 1);
    }

    #[test]
    fn path_parameters() {
        assert_eq!(path_params("/a/{id}/b/{name:str}/{9x}"), vec![("id", 3, 7), ("name", 10, 20)]);
    }

    #[test]
    fn unused_path_parameter() {
        let src = "from fastapi import FastAPI\napp = FastAPI()\n\n@app.get('/items/{item_id}')\nasync def read(q: str):\n    return q\n";
        let out = run(&UnusedPathParameter, Python, src);
        assert_eq!(out.len(), 1);
        assert_eq!(fixed(&UnusedPathParameter, src), "from fastapi import FastAPI\napp = FastAPI()\n\n@app.get('/items/{item_id}')\nasync def read(q: str, item_id):\n    return q\n");
        let ok = "from fastapi import FastAPI\napp = FastAPI()\n\n@app.get('/items/{item_id}')\nasync def read(item_id: int):\n    return item_id\n";
        assert!(run(&UnusedPathParameter, Python, ok).is_empty());
        let dep = "from fastapi import FastAPI, Depends\napp = FastAPI()\n\ndef common(item_id: int):\n    return item_id\n\n@app.get('/items/{item_id}')\nasync def read(c = Depends(common)):\n    return c\n";
        assert!(run(&UnusedPathParameter, Python, dep).is_empty());
    }
}

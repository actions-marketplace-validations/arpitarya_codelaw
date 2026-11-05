//! Rules of `react` and `ts` that read an ordered list or follow a name
//! (W-139): typescript-eslint's `member-ordering`, `unified-signatures` and
//! `no-restricted-types`; eslint-plugin-react's `jsx-sort-props`,
//! `sort-comp`, `sort-default-props`, `sort-prop-types`, `forbid-dom-props`,
//! `forbid-elements`, `no-unused-prop-types`, `no-unused-state`,
//! `prefer-exact-props` and `no-adjacent-inline-elements` — each its
//! documented behaviour, reimplemented.

use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JSX: &[Language] = &[Language::Javascript, Language::Jsx, Language::Tsx];
const TS: &[Language] = &[Language::Typescript, Language::Tsx];
const ALL: &[Language] = JSX;

fn unquote_key(k: &str) -> String {
    k.trim_matches(|c| c == '"' || c == '\'' || c == '`').to_owned()
}

fn finding(start: usize, end: usize, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

fn opt_strings(ctx: &NativeContext<'_>, key: &str) -> Vec<String> {
    match ctx.options.get(key) {
        Some(serde_json::Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect(),
        _ => Vec::new(),
    }
}

fn opt_bool(ctx: &NativeContext<'_>, key: &str, default: bool) -> bool {
    ctx.options.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

// ---------------------------------------------------------------------------
// ts/member-ordering

/// A class or interface member's rank in typescript-eslint's default order:
/// signatures, fields (static, then instance), static blocks, constructors,
/// accessors, methods (static, then instance); within a kind, public before
/// protected before private before `#private`.
fn member_rank(m: Node<'_>, src: &str) -> Option<(u32, &'static str)> {
    let kids = children(m);
    let is_static = kids.iter().any(|c| c.kind() == "static");
    let access = kids.iter().find(|c| c.kind() == "accessibility_modifier").map(|c| text(*c, src)).unwrap_or("public");
    let private_name = m.child_by_field_name("name").is_some_and(|n| n.kind() == "private_property_identifier");
    let sub = if private_name {
        3
    } else {
        match access {
            "protected" => 1,
            "private" => 2,
            _ => 0,
        }
    };
    let (base, what) = match m.kind() {
        "index_signature" | "call_signature" | "construct_signature" => (0, "signature"),
        "public_field_definition" | "field_definition" | "property_signature" => {
            if is_static {
                (10, "static field")
            } else {
                (20, "instance field")
            }
        }
        "class_static_block" => (30, "static block"),
        "method_definition" | "method_signature" | "abstract_method_signature" => {
            let name = m.child_by_field_name("name").map(|n| text(n, src)).unwrap_or("");
            if name == "constructor" {
                (40, "constructor")
            } else if kids.iter().any(|c| c.kind() == "get" || c.kind() == "set") {
                (50, "accessor")
            } else if is_static {
                (60, "static method")
            } else {
                (70, "instance method")
            }
        }
        _ => return None,
    };
    Some((base + sub, what))
}

pub struct MemberOrdering;

impl NativeRule for MemberOrdering {
    fn id(&self) -> &'static str {
        "ts/member-ordering"
    }
    fn languages(&self) -> &'static [Language] {
        TS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["class_body", "interface_body", "object_type"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        let mut best: Option<(u32, &'static str, Node<'_>)> = None;
        for m in named_children(n) {
            let Some((rank, what)) = member_rank(m, src) else { continue };
            if let Some((b, bw, bn)) = best
                && rank < b
            {
                let name = m.child_by_field_name("name").map(|x| text(x, src)).unwrap_or(what);
                let other = bn.child_by_field_name("name").map(|x| text(x, src)).unwrap_or(bw);
                out.push(finding(m.start_byte(), m.end_byte(), format!("The {what} `{name}` belongs before the {bw} `{other}`"), vec![]));
                continue;
            }
            if best.is_none_or(|(b, _, _)| rank > b) {
                best = Some((rank, what, m));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ts/unified-signatures

/// An overload's name, parameters and return type.
fn overload<'t>(n: Node<'t>, src: &str) -> Option<(String, Vec<Node<'t>>, String)> {
    if !matches!(n.kind(), "function_signature" | "method_signature" | "abstract_method_signature") {
        return None;
    }
    let name = text(n.child_by_field_name("name")?, src).to_owned();
    if n.child_by_field_name("type_parameters").is_some() {
        return None;
    }
    let params = named_children(n.child_by_field_name("parameters")?);
    let ret = n.child_by_field_name("return_type").map(|r| text(r, src).to_owned()).unwrap_or_default();
    Some((name, params, ret))
}

fn param_type(p: Node<'_>, src: &str) -> Option<String> {
    let t = p.child_by_field_name("type")?;
    Some(text(t, src).trim_start_matches(':').trim().to_owned())
}

fn param_head<'s>(p: Node<'_>, src: &'s str) -> &'s str {
    let t = text(p, src);
    match p.child_by_field_name("type") {
        Some(ty) => &t[..ty.start_byte() - p.start_byte()],
        None => t,
    }
}

pub struct UnifiedSignatures;

impl NativeRule for UnifiedSignatures {
    fn id(&self) -> &'static str {
        "ts/unified-signatures"
    }
    fn languages(&self) -> &'static [Language] {
        TS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program", "statement_block", "class_body", "interface_body", "object_type", "export_statement"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        let members: Vec<Node<'_>> = named_children(n).into_iter().map(|c| if c.kind() == "export_statement" { c.named_child(0).unwrap_or(c) } else { c }).collect();
        for w in members.windows(2) {
            let (Some((na, pa, ra)), Some((nb, pb, rb))) = (overload(w[0], src), overload(w[1], src)) else { continue };
            if na != nb || ra != rb {
                continue;
            }
            // Two overloads differing in one parameter's type.
            if pa.len() == pb.len() {
                let diffs: Vec<usize> = (0..pa.len()).filter(|&i| text(pa[i], src) != text(pb[i], src)).collect();
                if let [i] = diffs.as_slice()
                    && param_head(pa[*i], src) == param_head(pb[*i], src)
                    && let (Some(ta), Some(tb)) = (param_type(pa[*i], src), param_type(pb[*i], src))
                {
                    let ty = pa[*i].child_by_field_name("type").unwrap();
                    let mut edits = vec![TextEdit::new(ty.start_byte(), ty.end_byte(), format!(": {ta} | {tb}"))];
                    let (s, _) = line_span(src, w[1].start_byte(), w[1].end_byte());
                    let e = w[1].end_byte() + src[w[1].end_byte()..].find('\n').map(|x| x + 1).unwrap_or(src.len() - w[1].end_byte());
                    edits.push(TextEdit::new(s, e, ""));
                    out.push(finding(pb[*i].start_byte(), pb[*i].end_byte(), format!("These two overloads of `{na}` differ only here: take `{ta} | {tb}` in one signature"), edits));
                    continue;
                }
            }
            // One overload is the other plus a trailing parameter.
            let (short, long, at) = if pa.len() + 1 == pb.len() { (&pa, &pb, &w[1]) } else if pb.len() + 1 == pa.len() { (&pb, &pa, &w[0]) } else { continue };
            if (0..short.len()).all(|i| text(short[i], src) == text(long[i], src)) {
                let extra = long[long.len() - 1];
                if extra.kind() == "required_parameter" && !text(extra, src).starts_with("...") {
                    let _ = at;
                    out.push(finding(extra.start_byte(), extra.end_byte(), format!("These two overloads of `{na}` differ only by this parameter: make it optional in one signature"), vec![]));
                }
            }
        }
    }
}

fn line_span(src: &str, s: usize, e: usize) -> (usize, usize) {
    let ls = src[..s].rfind('\n').map(|x| x + 1).unwrap_or(0);
    let only_ws = src[ls..s].trim().is_empty();
    (if only_ws { ls } else { s }, e)
}

// ---------------------------------------------------------------------------
// ts/no-restricted-types

pub struct NoRestrictedTypes;

impl NativeRule for NoRestrictedTypes {
    fn id(&self) -> &'static str {
        "ts/no-restricted-types"
    }
    fn languages(&self) -> &'static [Language] {
        TS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["type_identifier", "predefined_type", "nested_type_identifier", "generic_type", "object_type", "tuple_type"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let Some(serde_json::Value::Object(types)) = ctx.options.get("types") else { return };
        let t: String = text(n, ctx.source).split_whitespace().collect::<Vec<_>>().join(" ");
        let Some(spec) = types.get(&t) else { return };
        let (msg, with) = match spec {
            serde_json::Value::String(m) => (m.clone(), None),
            serde_json::Value::Bool(true) => (String::new(), None),
            serde_json::Value::Object(o) => (o.get("message").and_then(|m| m.as_str()).unwrap_or("").to_owned(), o.get("fixWith").and_then(|m| m.as_str()).map(str::to_owned)),
            _ => return,
        };
        let edits = with.map(|w| vec![TextEdit::new(n.start_byte(), n.end_byte(), w)]).unwrap_or_default();
        let tail = if msg.is_empty() { String::new() } else { format!(": {msg}") };
        out.push(finding(n.start_byte(), n.end_byte(), format!("Don't use the type `{t}`{tail}"), edits));
    }
}

// ---------------------------------------------------------------------------
// JSX helpers

fn jsx_attrs<'t>(el: Node<'t>) -> Vec<Node<'t>> {
    named_children(el).into_iter().filter(|c| matches!(c.kind(), "jsx_attribute" | "jsx_expression")).collect()
}

fn attr_name<'s>(a: Node<'_>, src: &'s str) -> Option<&'s str> {
    if a.kind() != "jsx_attribute" {
        return None;
    }
    a.named_child(0).map(|n| text(n, src))
}

fn el_name<'s>(el: Node<'_>, src: &'s str) -> Option<&'s str> {
    el.child_by_field_name("name").map(|n| text(n, src))
}

// ---------------------------------------------------------------------------
// react/jsx-sort-props

pub struct JsxSortProps;

impl NativeRule for JsxSortProps {
    fn id(&self) -> &'static str {
        "react/jsx-sort-props"
    }
    fn languages(&self) -> &'static [Language] {
        JSX
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_opening_element", "jsx_self_closing_element"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        let ignore_case = opt_bool(ctx, "ignore_case", false);
        let key = |s: &str| if ignore_case { s.to_lowercase() } else { s.to_owned() };
        // Runs of attributes between spreads.
        let attrs = jsx_attrs(n);
        let mut runs: Vec<Vec<Node<'_>>> = vec![Vec::new()];
        for a in attrs {
            if a.kind() == "jsx_expression" {
                runs.push(Vec::new());
            } else {
                runs.last_mut().unwrap().push(a);
            }
        }
        for run in runs {
            let names: Vec<String> = run.iter().map(|a| key(attr_name(*a, src).unwrap_or(""))).collect();
            let Some(bad) = (1..names.len()).find(|&i| names[i] < names[i - 1]) else { continue };
            let mut sorted: Vec<Node<'_>> = run.clone();
            sorted.sort_by_key(|a| key(attr_name(*a, src).unwrap_or("")));
            let edits = run.iter().zip(&sorted).filter(|(a, b)| a.id() != b.id()).map(|(a, b)| TextEdit::new(a.start_byte(), a.end_byte(), text(*b, src))).collect();
            let a = run[bad];
            out.push(finding(a.start_byte(), a.end_byte(), format!("Props are in alphabetical order: `{}` comes before `{}`", attr_name(a, src).unwrap_or(""), attr_name(run[bad - 1], src).unwrap_or("")), edits));
        }
    }
}

// ---------------------------------------------------------------------------
// react/sort-comp

fn is_component_class(class: Node<'_>, src: &str) -> bool {
    crate::react::is_component_class(class, src)
}

const LIFECYCLE: &[&str] = &[
    "displayName", "propTypes", "contextType", "contextTypes", "childContextTypes", "mixins", "statics", "defaultProps", "constructor", "getDefaultProps", "state", "getInitialState",
    "getChildContext", "getDerivedStateFromProps", "componentWillMount", "UNSAFE_componentWillMount", "componentDidMount", "componentWillReceiveProps", "UNSAFE_componentWillReceiveProps",
    "shouldComponentUpdate", "componentWillUpdate", "UNSAFE_componentWillUpdate", "getSnapshotBeforeUpdate", "componentDidUpdate", "componentDidCatch", "componentWillUnmount",
];

fn comp_rank(m: Node<'_>, src: &str) -> Option<(u32, String)> {
    let name = m.child_by_field_name("name").map(|n| text(n, src).to_owned())?;
    let is_static = children(m).iter().any(|c| c.kind() == "static");
    if let Some(i) = LIFECYCLE.iter().position(|x| *x == name) {
        return Some((100 + i as u32, name));
    }
    if name == "render" {
        return Some((400, name));
    }
    if is_static && m.kind() == "method_definition" {
        return Some((0, name));
    }
    if name.starts_with("render") {
        return Some((300, name));
    }
    Some((200, name))
}

pub struct SortComp;

impl NativeRule for SortComp {
    fn id(&self) -> &'static str {
        "react/sort-comp"
    }
    fn languages(&self) -> &'static [Language] {
        ALL
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["class_declaration", "class"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        if !is_component_class(n, src) {
            return;
        }
        let Some(body) = n.child_by_field_name("body") else { return };
        let mut best: Option<(u32, String)> = None;
        for m in named_children(body) {
            if !matches!(m.kind(), "method_definition" | "public_field_definition" | "field_definition") {
                continue;
            }
            let Some((rank, name)) = comp_rank(m, src) else { continue };
            if let Some((b, bn)) = &best
                && rank < *b
            {
                out.push(finding(m.start_byte(), m.end_byte(), format!("`{name}` belongs before `{bn}` in a component (static methods, lifecycle, other methods, render helpers, `render`)"), vec![]));
                continue;
            }
            if best.as_ref().is_none_or(|(b, _)| rank > *b) {
                best = Some((rank, name));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// react/sort-default-props, react/sort-prop-types

/// `X.<prop> = { … }` or `static <prop> = { … }`: the object.
fn static_object<'t>(n: Node<'t>, src: &str, prop: &str) -> Option<Node<'t>> {
    match n.kind() {
        "assignment_expression" => {
            let l = n.child_by_field_name("left")?;
            let r = n.child_by_field_name("right")?;
            (l.kind() == "member_expression" && l.child_by_field_name("property").is_some_and(|p| text(p, src) == prop) && r.kind() == "object").then_some(r)
        }
        "public_field_definition" | "field_definition" => {
            let name = n.child_by_field_name("name").or_else(|| n.child_by_field_name("property"))?;
            let v = n.child_by_field_name("value")?;
            (text(name, src) == prop && children(n).iter().any(|c| c.kind() == "static") && v.kind() == "object").then_some(v)
        }
        _ => None,
    }
}

fn sort_object(obj: Node<'_>, src: &str, what: &str, ignore_case: bool, out: &mut Vec<NativeFinding>) {
    let pairs: Vec<Node<'_>> = named_children(obj).into_iter().filter(|c| c.kind() != "comment").collect();
    let mut runs: Vec<Vec<Node<'_>>> = vec![Vec::new()];
    for p in pairs {
        if p.kind() == "pair" || p.kind() == "shorthand_property_identifier" {
            runs.last_mut().unwrap().push(p);
        } else {
            runs.push(Vec::new());
        }
    }
    let key = |p: &Node<'_>| {
        let k = if p.kind() == "pair" { p.child_by_field_name("key").map(|k| unquote_key(text(k, src))).unwrap_or_default() } else { text(*p, src).to_owned() };
        if ignore_case { k.to_lowercase() } else { k }
    };
    for run in runs {
        let keys: Vec<String> = run.iter().map(key).collect();
        let Some(bad) = (1..keys.len()).find(|&i| keys[i] < keys[i - 1]) else { continue };
        let mut sorted = run.clone();
        sorted.sort_by_key(key);
        let edits = run.iter().zip(&sorted).filter(|(a, b)| a.id() != b.id()).map(|(a, b)| TextEdit::new(a.start_byte(), a.end_byte(), text(*b, src))).collect();
        let p = run[bad];
        out.push(finding(p.start_byte(), p.end_byte(), format!("{what} are in alphabetical order: `{}` comes before `{}`", keys[bad], keys[bad - 1]), edits));
    }
}

pub struct SortDefaultProps;

impl NativeRule for SortDefaultProps {
    fn id(&self) -> &'static str {
        "react/sort-default-props"
    }
    fn languages(&self) -> &'static [Language] {
        ALL
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["assignment_expression", "public_field_definition", "field_definition"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        if let Some(o) = static_object(node.get_inner_node(), ctx.source, "defaultProps") {
            sort_object(o, ctx.source, "Default props", opt_bool(ctx, "ignore_case", false), out);
        }
    }
}

pub struct SortPropTypes;

impl NativeRule for SortPropTypes {
    fn id(&self) -> &'static str {
        "react/sort-prop-types"
    }
    fn languages(&self) -> &'static [Language] {
        ALL
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["assignment_expression", "public_field_definition", "field_definition"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        if let Some(o) = static_object(node.get_inner_node(), ctx.source, "propTypes") {
            sort_object(o, ctx.source, "Prop types", opt_bool(ctx, "ignore_case", false), out);
        }
    }
}

// ---------------------------------------------------------------------------
// react/forbid-dom-props, react/forbid-elements

pub struct ForbidDomProps;

impl NativeRule for ForbidDomProps {
    fn id(&self) -> &'static str {
        "react/forbid-dom-props"
    }
    fn languages(&self) -> &'static [Language] {
        JSX
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_opening_element", "jsx_self_closing_element"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let forbid = opt_strings(ctx, "forbid");
        if forbid.is_empty() || !el_name(n, ctx.source).is_some_and(|x| x.starts_with(|c: char| c.is_ascii_lowercase()) && !x.contains('.')) {
            return;
        }
        for a in jsx_attrs(n) {
            if let Some(name) = attr_name(a, ctx.source)
                && forbid.iter().any(|f| f == name)
            {
                out.push(finding(a.start_byte(), a.end_byte(), format!("`{name}` is not allowed on DOM elements in this project"), vec![]));
            }
        }
    }
}

pub struct ForbidElements;

impl NativeRule for ForbidElements {
    fn id(&self) -> &'static str {
        "react/forbid-elements"
    }
    fn languages(&self) -> &'static [Language] {
        JSX
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_opening_element", "jsx_self_closing_element"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let forbid = opt_strings(ctx, "forbid");
        let Some(name) = el_name(n, ctx.source) else { return };
        if forbid.iter().any(|f| f == name) {
            let nn = n.child_by_field_name("name").unwrap();
            out.push(finding(nn.start_byte(), nn.end_byte(), format!("`<{name}>` is not allowed in this project"), vec![]));
        }
    }
}

// ---------------------------------------------------------------------------
// react/no-unused-prop-types

/// The props a function component reads: `None` when it passes `props`
/// on whole (a spread, a call, a rest pattern), so any prop may be used.
fn used_props(func: Node<'_>, src: &str) -> Option<Vec<String>> {
    let params = func.child_by_field_name("parameters").map(named_children).or_else(|| func.child_by_field_name("parameter").map(|p| vec![p]))?;
    let first = *params.first()?;
    let pat = if first.kind() == "required_parameter" || first.kind() == "optional_parameter" { first.child_by_field_name("pattern")? } else { first };
    match pat.kind() {
        "object_pattern" => {
            let mut v = Vec::new();
            for c in named_children(pat) {
                match c.kind() {
                    "shorthand_property_identifier_pattern" => v.push(text(c, src).to_owned()),
                    "pair_pattern" | "object_assignment_pattern" => {
                        let k = c.child_by_field_name("key").or_else(|| c.child_by_field_name("left"))?;
                        v.push(text(k, src).to_owned());
                    }
                    _ => return None,
                }
            }
            Some(v)
        }
        "identifier" => {
            let name = text(pat, src);
            let body = func.child_by_field_name("body")?;
            let mut v = Vec::new();
            let mut stack = vec![body];
            while let Some(x) = stack.pop() {
                if x.kind() == "identifier" && text(x, src) == name {
                    let p = x.parent()?;
                    if p.kind() == "member_expression" && p.child_by_field_name("object").is_some_and(|o| o.id() == x.id()) {
                        v.push(text(p.child_by_field_name("property")?, src).to_owned());
                    } else if p.kind() == "variable_declarator" && p.child_by_field_name("value").is_some_and(|o| o.id() == x.id()) && p.child_by_field_name("name").is_some_and(|n| n.kind() == "object_pattern") {
                        let fake = p.child_by_field_name("name")?;
                        for c in named_children(fake) {
                            match c.kind() {
                                "shorthand_property_identifier_pattern" => v.push(text(c, src).to_owned()),
                                "pair_pattern" => v.push(text(c.child_by_field_name("key")?, src).to_owned()),
                                _ => return None,
                            }
                        }
                    } else {
                        return None;
                    }
                }
                let mut c = x.walk();
                stack.extend(x.children(&mut c));
            }
            Some(v)
        }
        _ => None,
    }
}

/// Function components declared at the top of the file, by name.
fn components<'t>(root: Node<'t>, src: &str) -> Vec<(String, Node<'t>)> {
    let mut out = Vec::new();
    for s in named_children(root) {
        let s = if s.kind() == "export_statement" { s.child_by_field_name("declaration").unwrap_or(s) } else { s };
        match s.kind() {
            "function_declaration" => {
                if let Some(n) = s.child_by_field_name("name") {
                    out.push((text(n, src).to_owned(), s));
                }
            }
            "lexical_declaration" | "variable_declaration" => {
                for d in named_children(s) {
                    if let (Some(n), Some(v)) = (d.child_by_field_name("name"), d.child_by_field_name("value"))
                        && matches!(v.kind(), "arrow_function" | "function_expression" | "function")
                    {
                        out.push((text(n, src).to_owned(), v));
                    }
                }
            }
            _ => {}
        }
    }
    out.retain(|(n, _)| n.starts_with(|c: char| c.is_ascii_uppercase()));
    out
}

pub struct NoUnusedPropTypes;

impl NativeRule for NoUnusedPropTypes {
    fn id(&self) -> &'static str {
        "react/no-unused-prop-types"
    }
    fn languages(&self) -> &'static [Language] {
        ALL
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let root = node.get_inner_node();
        let src = ctx.source;
        let comps = components(root, src);
        for s in named_children(root) {
            let Some(e) = s.named_child(0).filter(|_| s.kind() == "expression_statement") else { continue };
            let Some(obj) = static_object(e, src, "propTypes") else { continue };
            let target = e.child_by_field_name("left").and_then(|l| l.child_by_field_name("object")).map(|o| text(o, src).to_owned()).unwrap_or_default();
            let Some((_, func)) = comps.iter().find(|(n, _)| *n == target) else { continue };
            let Some(used) = used_props(*func, src) else { continue };
            for p in named_children(obj) {
                if p.kind() != "pair" {
                    continue;
                }
                let Some(k) = p.child_by_field_name("key") else { continue };
                let key = unquote_key(text(k, src));
                if !used.contains(&key) {
                    let after = src[p.end_byte()..].find(|c: char| !c.is_whitespace()).map(|x| p.end_byte() + x).unwrap_or(p.end_byte());
                    let edits = if src[after..].starts_with(',') {
                        let next = src[after + 1..].find(|c: char| !c.is_whitespace()).map(|x| after + 1 + x).unwrap_or(after + 1);
                        vec![TextEdit::new(p.start_byte(), next, "")]
                    } else {
                        let before = src[..p.start_byte()].rfind(|c: char| !c.is_whitespace()).unwrap_or(0);
                        if src[before..].starts_with(',') { vec![TextEdit::new(before, p.end_byte(), "")] } else { vec![] }
                    };
                    out.push(finding(k.start_byte(), k.end_byte(), format!("The prop `{key}` is declared but `{target}` never reads it"), edits));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// react/no-unused-state

pub struct NoUnusedState;

impl NativeRule for NoUnusedState {
    fn id(&self) -> &'static str {
        "react/no-unused-state"
    }
    fn languages(&self) -> &'static [Language] {
        ALL
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["class_declaration", "class"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        if !is_component_class(n, src) {
            return;
        }
        let Some(body) = n.child_by_field_name("body") else { return };
        // The initial state's keys.
        let mut keys: Vec<(String, Node<'_>)> = Vec::new();
        let mut stack = vec![body];
        let mut whole = false;
        let mut read: Vec<String> = Vec::new();
        while let Some(x) = stack.pop() {
            match x.kind() {
                "public_field_definition" | "field_definition" => {
                    if x.child_by_field_name("name").or_else(|| x.child_by_field_name("property")).is_some_and(|k| text(k, src) == "state")
                        && let Some(v) = x.child_by_field_name("value").filter(|v| v.kind() == "object")
                    {
                        collect_keys(v, src, &mut keys);
                    }
                }
                "assignment_expression" => {
                    if x.child_by_field_name("left").is_some_and(|l| text(l, src) == "this.state")
                        && let Some(v) = x.child_by_field_name("right").filter(|v| v.kind() == "object")
                    {
                        collect_keys(v, src, &mut keys);
                        continue;
                    }
                }
                "member_expression" if text(x, src) == "this.state" => {
                    let p = x.parent();
                    match p {
                        Some(p) if p.kind() == "member_expression" && p.child_by_field_name("object").is_some_and(|o| o.id() == x.id()) => {
                            if let Some(pr) = p.child_by_field_name("property") {
                                read.push(text(pr, src).to_owned());
                            }
                        }
                        Some(p) if p.kind() == "variable_declarator" && p.child_by_field_name("name").is_some_and(|nm| nm.kind() == "object_pattern") => {
                            for c in named_children(p.child_by_field_name("name").unwrap()) {
                                match c.kind() {
                                    "shorthand_property_identifier_pattern" => read.push(text(c, src).to_owned()),
                                    "pair_pattern" => read.extend(c.child_by_field_name("key").map(|k| text(k, src).to_owned())),
                                    _ => whole = true,
                                }
                            }
                        }
                        _ => whole = true,
                    }
                }
                _ => {}
            }
            let mut c = x.walk();
            stack.extend(x.children(&mut c));
        }
        if whole {
            return;
        }
        for (k, node) in keys {
            if !read.contains(&k) {
                out.push(finding(node.start_byte(), node.end_byte(), format!("The state field `{k}` is set but never read"), vec![]));
            }
        }
    }
}

fn collect_keys<'t>(obj: Node<'t>, src: &str, out: &mut Vec<(String, Node<'t>)>) {
    for p in named_children(obj) {
        match p.kind() {
            "pair" => {
                if let Some(k) = p.child_by_field_name("key") {
                    out.push((unquote_key(text(k, src)), k));
                }
            }
            "shorthand_property_identifier" => out.push((text(p, src).to_owned(), p)),
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// react/prefer-exact-props

pub struct PreferExactProps;

impl NativeRule for PreferExactProps {
    fn id(&self) -> &'static str {
        "react/prefer-exact-props"
    }
    fn languages(&self) -> &'static [Language] {
        ALL
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["assignment_expression", "public_field_definition", "field_definition"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let wrappers = opt_strings(ctx, "exact_wrappers");
        let Some(w) = wrappers.first() else { return };
        if let Some(o) = static_object(node.get_inner_node(), ctx.source, "propTypes") {
            let edits = vec![TextEdit::new(o.start_byte(), o.start_byte(), format!("{w}(")), TextEdit::new(o.end_byte(), o.end_byte(), ")")];
            out.push(finding(o.start_byte(), o.end_byte(), format!("Wrap the prop types in `{w}(…)` so a prop nobody declared is an error"), edits));
        }
    }
}

// ---------------------------------------------------------------------------
// react/no-adjacent-inline-elements

const INLINE: &[&str] = &[
    "a", "abbr", "acronym", "b", "bdo", "big", "button", "cite", "code", "dfn", "em", "i", "img", "input", "kbd", "label", "map", "object", "output", "q", "samp", "script", "select", "small",
    "span", "strong", "sub", "sup", "textarea", "tt", "var",
];

fn inline_el(n: Node<'_>, src: &str) -> bool {
    let open = match n.kind() {
        "jsx_element" => n.child_by_field_name("open_tag"),
        "jsx_self_closing_element" => Some(n),
        _ => None,
    };
    open.and_then(|o| el_name(o, src)).is_some_and(|x| INLINE.contains(&x))
}

pub struct NoAdjacentInlineElements;

impl NativeRule for NoAdjacentInlineElements {
    fn id(&self) -> &'static str {
        "react/no-adjacent-inline-elements"
    }
    fn languages(&self) -> &'static [Language] {
        JSX
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_element"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let src = ctx.source;
        let kids: Vec<Node<'_>> = children(n).into_iter().filter(|c| matches!(c.kind(), "jsx_element" | "jsx_self_closing_element")).collect();
        for w in kids.windows(2) {
            if inline_el(w[0], src) && inline_el(w[1], src) && w[0].end_byte() == w[1].start_byte() {
                out.push(finding(w[1].start_byte(), w[1].end_byte(), "Two inline elements touch: nothing separates them when rendered", vec![TextEdit::new(w[0].end_byte(), w[0].end_byte(), " ")]));
            }
        }
    }
}

pub static RULES: &[&dyn NativeRule] = &[
    &MemberOrdering, &UnifiedSignatures, &NoRestrictedTypes, &JsxSortProps, &SortComp, &SortDefaultProps, &SortPropTypes, &ForbidDomProps, &ForbidElements,
    &NoUnusedPropTypes, &NoUnusedState, &PreferExactProps, &NoAdjacentInlineElements,
];

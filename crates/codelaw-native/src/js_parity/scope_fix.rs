//! W-139 strand W-139-js, the scope rows whose upstream rule fixes the
//! code: eslint-plugin-unicorn v79.0.0's rewrites that need to know every
//! reference of a binding before they move or rewrite it. Each fix is
//! offered only in the shapes where it is certain; the rule page names what
//! upstream rewrites beyond them.

use super::scope::{ancestors, enclosing_fn, fn_like};
use super::{JS, finding, span};
use crate::js_ast::{self, arguments, unparen};
use crate::util::{children, named_children, remove_list_item, remove_statement, text};
use crate::{NativeContext, NativeFinding, NativeRule, as_suggestion};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{Reference, SemanticModel, SymbolId, SymbolKind};
use serde_json::Value;
use std::collections::BTreeSet;
use tree_sitter::Node;

macro_rules! srule {
    ($name:ident, $id:literal, $kinds:expr, |$n:ident, $ctx:ident, $m:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                JS
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let Some($m) = $ctx.semantic else { return };
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}

fn same(a: Option<Node<'_>>, b: Node<'_>) -> bool {
    a.is_some_and(|a| a.id() == b.id())
}

fn in_range(pos: usize, outer: Node<'_>) -> bool {
    pos >= outer.start_byte() && pos < outer.end_byte()
}

fn node_at<'t>(root: Node<'t>, start: usize, end: usize) -> Option<Node<'t>> {
    root.descendant_for_byte_range(start, end)
}

fn refs_of(m: &SemanticModel, sym: SymbolId) -> Vec<&Reference> {
    m.symbol(sym).refs.iter().map(|&r| &m.references[r]).collect()
}

fn has_comment(n: Node<'_>) -> bool {
    n.kind() == "comment" || children(n).iter().any(|c| has_comment(*c))
}

fn comment_between(root: Node<'_>, start: usize, end: usize) -> bool {
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        if x.end_byte() <= start || x.start_byte() >= end {
            continue;
        }
        if x.kind() == "comment" {
            return true;
        }
        stack.extend(children(x));
    }
    false
}

fn is_ts(ctx: &NativeContext<'_>) -> bool {
    matches!(ctx.language, Language::Typescript | Language::Tsx)
}

fn indent_of(src: &str, at: usize) -> &str {
    let ls = src[..at].rfind('\n').map_or(0, |i| i + 1);
    let line = &src[ls..];
    &line[..line.len() - line.trim_start_matches([' ', '\t']).len()]
}

fn unit(src: &str, indent: &str) -> &'static str {
    if indent.contains('\t') || src.lines().any(|l| l.starts_with('\t')) { "\t" } else { "  " }
}

fn sole_declarator(decl: Node<'_>) -> Option<Node<'_>> {
    let ds: Vec<Node<'_>> = named_children(decl).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
    (ds.len() == 1).then(|| ds[0])
}

fn decl_kind<'s>(decl: Node<'_>, src: &'s str) -> &'s str {
    decl.child(0).map_or("", |k| text(k, src))
}

fn unresolved(m: &SemanticModel, ident: Node<'_>, name: &str, src: &str) -> bool {
    ident.kind() == "identifier" && text(ident, src) == name && m.reference_at(ident.start_byte()).is_some_and(|r| r.symbol.is_none())
}

fn new_of<'t>(n: Node<'t>, ctor: &str, m: &SemanticModel, src: &str) -> Option<Node<'t>> {
    let n = unparen(n);
    (n.kind() == "new_expression" && n.child_by_field_name("constructor").is_some_and(|c| unresolved(m, c, ctor, src))).then_some(n)
}

fn literal(n: Node<'_>) -> bool {
    match n.kind() {
        "string" | "number" | "true" | "false" | "null" | "regex" => true,
        "template_string" => !named_children(n).iter().any(|c| c.kind() == "template_substitution"),
        _ => false,
    }
}

fn side_effect(n: Node<'_>) -> bool {
    match n.kind() {
        "call_expression" | "new_expression" | "assignment_expression" | "augmented_assignment_expression" | "update_expression" | "await_expression" | "yield_expression" => true,
        "member_expression" | "subscript_expression" => true,
        k if fn_like(k) => false,
        _ => named_children(n).iter().any(|c| side_effect(*c)),
    }
}

fn statement_list(n: Node<'_>) -> bool {
    matches!(n.kind(), "program" | "statement_block" | "class_static_block")
}

// ---- unicorn: prefer-class-fields -----------------------------------------

fn field_name<'t>(member: Node<'t>) -> Option<Node<'t>> {
    match member.kind() {
        "field_definition" => member.child_by_field_name("property"),
        "public_field_definition" => member.child_by_field_name("name"),
        _ => None,
    }
}

srule!(PreferClassFields, "js/prefer-class-fields", &["class_body"], |n, ctx, _m, out| {
    let src = ctx.source;
    let members: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    let Some(ctor) = members.iter().copied().find(|mm| mm.kind() == "method_definition" && mm.child_by_field_name("name").is_some_and(|k| text(k, src) == "constructor") && !children(*mm).iter().any(|c| c.kind() == "static")) else { return };
    let Some(body) = js_ast::body(ctor) else { return };
    let Some(stmt) = named_children(body).into_iter().find(|c| c.kind() != "comment" && c.kind() != "empty_statement") else { return };
    if stmt.kind() != "expression_statement" {
        return;
    }
    let Some(asg) = stmt.named_child(0).filter(|a| a.kind() == "assignment_expression") else { return };
    let (Some(left), Some(right)) = (asg.child_by_field_name("left"), asg.child_by_field_name("right")) else { return };
    if left.kind() != "member_expression" || !left.child_by_field_name("object").is_some_and(|o| o.kind() == "this") || !literal(unparen(right)) {
        return;
    }
    let Some(prop) = left.child_by_field_name("property").filter(|p| matches!(p.kind(), "property_identifier" | "private_property_identifier")) else { return };
    let name = text(prop, src);
    let private = prop.kind() == "private_property_identifier";
    if js_ast::params(ctor).iter().any(|p| p.kind() == "required_parameter" && js_ast::param_name(*p, src) == Some(name) && children(*p).iter().any(|c| matches!(c.kind(), "accessibility_modifier" | "readonly"))) {
        return;
    }
    let is_static = |mm: Node<'_>| children(mm).iter().any(|c| c.kind() == "static");
    let fields: Vec<Node<'_>> = members.iter().copied().filter(|mm| !is_static(*mm) && field_name(*mm).is_some_and(|k| text(k, src) == name)).collect();
    let clash = members.iter().any(|mm| mm.kind() == "method_definition" && mm.id() != ctor.id() && mm.child_by_field_name("name").is_some_and(|k| text(k, src) == name) && (!is_static(*mm) || private)) || (private && members.iter().any(|mm| is_static(*mm) && field_name(*mm).is_some_and(|k| text(k, src) == name)));
    let odd = fields.iter().any(|f| children(*f).iter().any(|c| matches!(c.kind(), "declare" | "abstract" | "!")));
    let value = text(right, src);
    let msg = format!("`this.{name}` gets a constant in the constructor: declare it as a class field, `{name} = {value};`");
    if fields.len() > 1 || clash || odd || has_comment(stmt) {
        out.push(finding(stmt, msg, vec![]));
        return;
    }
    let mut edits = vec![remove_statement(stmt, src)];
    match fields.first() {
        Some(f) if f.child_by_field_name("value").is_some() => {
            let v = f.child_by_field_name("value").unwrap_or(*f);
            out.push(finding(stmt, msg, as_suggestion(vec![remove_statement(stmt, src), TextEdit::new(v.start_byte(), v.end_byte(), value)])));
            return;
        }
        Some(f) => {
            let at = f.child_by_field_name("type").map_or_else(|| field_name(*f).map_or(f.end_byte(), |k| k.end_byte()), |t| t.end_byte());
            edits.push(TextEdit::insert(at, format!(" = {value}")));
        }
        None => {
            let close = n.end_byte() - 1;
            let first = members.first().copied().unwrap_or(ctor);
            let ind = indent_of(src, first.start_byte()).to_owned();
            let line_start = src[..close].rfind('\n').map_or(0, |i| i + 1);
            if src[line_start..close].trim().is_empty() && first.start_position().row > n.start_position().row {
                edits.push(TextEdit::insert(line_start, format!("{ind}{name} = {value};\n")));
            } else {
                edits.push(TextEdit::insert(close, format!("{name} = {value}; ")));
            }
        }
    }
    out.push(finding(stmt, msg, edits));
});

// ---- unicorn: prefer-object-define-properties ------------------------------

fn define_property<'t>(stmt: Node<'t>, src: &str) -> Option<(Node<'t>, [Node<'t>; 3])> {
    if stmt.kind() != "expression_statement" {
        return None;
    }
    let call = stmt.named_child(0).filter(|c| c.kind() == "call_expression")?;
    let f = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && f.child_by_field_name("optional_chain").is_none())?;
    if !f.child_by_field_name("object").is_some_and(|o| text(o, src) == "Object") || !f.child_by_field_name("property").is_some_and(|p| text(p, src) == "defineProperty") {
        return None;
    }
    let args = arguments(call);
    let [t, k, d] = args[..] else { return None };
    if args.iter().any(|a| a.kind() == "spread_element") {
        return None;
    }
    Some((f.child_by_field_name("property")?, [t, k, d]))
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum KeyClass {
    Int,
    Str,
    Sym,
}

fn key_text(k: Node<'_>, src: &str) -> Option<(KeyClass, String, Option<u64>)> {
    let t = text(k, src);
    match k.kind() {
        "string" => {
            let v = crate::util::unquote(t);
            if let Ok(i) = v.parse::<u64>()
                && i.to_string() == v
            {
                return Some((KeyClass::Int, t.to_owned(), Some(i)));
            }
            if v == "__proto__" {
                return Some((KeyClass::Str, format!("[{t}]"), None));
            }
            let ident = !v.is_empty() && v.chars().enumerate().all(|(i, c)| c == '_' || c == '$' || c.is_ascii_alphabetic() || (i > 0 && c.is_ascii_digit()));
            Some((KeyClass::Str, if ident { v.to_owned() } else { t.to_owned() }, None))
        }
        "number" => t.parse::<u64>().ok().map(|i| (KeyClass::Int, t.to_owned(), Some(i))),
        "member_expression" if t.starts_with("Symbol.") => Some((KeyClass::Sym, format!("[{t}]"), None)),
        _ => None,
    }
}

srule!(PreferObjectDefineProperties, "js/prefer-object-define-properties", &["program", "statement_block", "class_static_block"], |n, ctx, _m, out| {
    let src = ctx.source;
    let stmts: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    let mut i = 0;
    while i < stmts.len() {
        let Some((prop, [t, _, _])) = define_property(stmts[i], src) else {
            i += 1;
            continue;
        };
        let target = text(t, src);
        let mut j = i + 1;
        while j < stmts.len() && define_property(stmts[j], src).is_some_and(|(_, [t2, _, _])| text(t2, src) == target) {
            j += 1;
        }
        if j - i < 2 {
            i += 1;
            continue;
        }
        let run = &stmts[i..j];
        let first = run[0];
        let last = run[run.len() - 1];
        let mut entries = Vec::new();
        let mut fixable = !comment_between(n, first.start_byte(), last.end_byte());
        let mut seen = BTreeSet::new();
        let mut prev = KeyClass::Int;
        let mut prev_int: Option<u64> = None;
        let root_name = js_ast::chain_root(t).map(|r| text(r, src).to_owned());
        for (idx, s) in run.iter().enumerate() {
            let Some((_, [_, k, d])) = define_property(*s, src) else { continue };
            let Some((class, key, int)) = key_text(k, src) else {
                fixable = false;
                continue;
            };
            if !seen.insert(key.clone()) || class < prev || (class == KeyClass::Int && prev_int.is_some_and(|p| int.is_some_and(|i| i < p))) || d.start_position().row != d.end_position().row {
                fixable = false;
            }
            if idx > 0 && (side_effect(d) && !matches!(d.kind(), "object") || root_name.as_deref().is_some_and(|r| crate::util::mentions(text(d, src), r))) {
                fixable = false;
            }
            if idx > 0 && d.kind() == "object" && named_children(d).iter().any(|p| p.child_by_field_name("value").is_some_and(side_effect)) {
                fixable = false;
            }
            prev = class;
            if int.is_some() {
                prev_int = int;
            }
            entries.push((key, text(d, src).to_owned()));
        }
        let mut edits = Vec::new();
        if fixable {
            let ind = indent_of(src, first.start_byte()).to_owned();
            let u = unit(src, &ind);
            let body: String = entries.iter().map(|(k, d)| format!("{ind}{u}{k}: {d},\n")).collect();
            edits.push(TextEdit::new(first.start_byte(), last.end_byte(), format!("Object.defineProperties({target}, {{\n{body}{ind}}});")));
        }
        out.push(finding(prop, format!("{} `Object.defineProperty()` calls on `{target}` in a row: define them in one `Object.defineProperties()`", run.len()), edits));
        i = j;
    }
});

// ---- unicorn: prefer-set-has -----------------------------------------------

const ARRAY_PRODUCERS: &[&str] = &["map", "filter", "flat", "flatMap", "sort", "reverse", "splice", "fill", "copyWithin", "toReversed", "toSorted", "toSpliced", "with", "split", "toArray"];

fn array_init(v: Node<'_>, src: &str) -> bool {
    let v = unparen(v);
    match v.kind() {
        "array" => true,
        "new_expression" => v.child_by_field_name("constructor").is_some_and(|c| text(c, src) == "Array"),
        "call_expression" => {
            let Some(f) = v.child_by_field_name("function") else { return false };
            match f.kind() {
                "identifier" => text(f, src) == "Array",
                "member_expression" => {
                    let p = f.child_by_field_name("property").map_or("", |p| text(p, src));
                    let o = f.child_by_field_name("object").map_or("", |o| text(o, src));
                    (o == "Array" && (p == "from" || p == "of")) || ARRAY_PRODUCERS.contains(&p)
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn unique_primitives(v: Node<'_>, src: &str) -> bool {
    let v = unparen(v);
    if v.kind() != "array" {
        return false;
    }
    let mut seen = BTreeSet::new();
    named_children(v).into_iter().filter(|c| c.kind() != "comment").all(|e| matches!(e.kind(), "string" | "number" | "true" | "false" | "null") && text(e, src) != "-0" && seen.insert(text(e, src).to_owned()))
}

/// The edit renaming `x.length`'s property, or `x.includes`', when a
/// reference is one of the uses a `Set` keeps.
enum SetUse {
    Includes(usize, usize),
    Length(usize, usize),
    Kept,
}

fn set_use(id: Node<'_>, src: &str, extra: bool) -> Option<SetUse> {
    let p = id.parent()?;
    if p.kind() == "member_expression" && same(p.child_by_field_name("object"), id) && p.child_by_field_name("optional_chain").is_none() {
        let prop = p.child_by_field_name("property")?;
        let name = text(prop, src);
        let call = p.parent().filter(|c| c.kind() == "call_expression" && same(c.child_by_field_name("function"), p));
        if name == "includes" && call.is_some_and(|c| arguments(c).len() == 1 && arguments(c)[0].kind() != "spread_element") {
            return Some(SetUse::Includes(prop.start_byte(), prop.end_byte()));
        }
        if !extra {
            return None;
        }
        if name == "length" && call.is_none() && !p.parent().is_some_and(|g| matches!(g.kind(), "assignment_expression" | "augmented_assignment_expression" | "update_expression") && g.child(0).is_some_and(|c| c.id() == p.id())) {
            return Some(SetUse::Length(prop.start_byte(), prop.end_byte()));
        }
        if name == "forEach" && call.is_some_and(|c| matches!(&arguments(c)[..], [f] if js_ast::as_function(*f).is_some_and(|f| js_ast::params(f).len() == 1))) {
            return Some(SetUse::Kept);
        }
        return None;
    }
    if !extra {
        return None;
    }
    match p.kind() {
        "for_in_statement" if same(p.child_by_field_name("right"), id) && children(p).iter().any(|c| c.kind() == "of") => Some(SetUse::Kept),
        "spread_element" if p.parent().is_some_and(|g| matches!(g.kind(), "array" | "arguments")) => Some(SetUse::Kept),
        _ => None,
    }
}

srule!(PreferSetHas, "js/prefer-set-has", &["variable_declarator"], |n, ctx, m, out| {
    let src = ctx.source;
    let (Some(name), Some(value)) = (n.child_by_field_name("name"), n.child_by_field_name("value")) else { return };
    if name.kind() != "identifier" || !array_init(value, src) {
        return;
    }
    let Some(decl) = n.parent() else { return };
    if decl.parent().is_some_and(|p| p.kind() == "export_statement") {
        return;
    }
    let min = ctx.options.get("minimumItems").and_then(Value::as_u64).unwrap_or(0) as usize;
    if min > 0 && !(unparen(value).kind() == "array" && !named_children(unparen(value)).iter().any(|c| c.kind() == "spread_element") && named_children(unparen(value)).iter().filter(|c| c.kind() != "comment").count() >= min) {
        return;
    }
    let Some(sym) = m.declared_at(name.start_byte()) else { return };
    let root = js_ast::root_of(n);
    let extra = unique_primitives(value, src);
    let mut edits = Vec::new();
    let mut includes = Vec::new();
    for r in refs_of(m, sym) {
        if r.write {
            return;
        }
        let Some(id) = node_at(root, r.start, r.end) else { return };
        match set_use(id, src, extra) {
            Some(SetUse::Includes(a, b)) => {
                includes.push(id);
                edits.push(TextEdit::new(a, b, "has"));
            }
            Some(SetUse::Length(a, b)) => edits.push(TextEdit::new(a, b, "size")),
            Some(SetUse::Kept) => {}
            None => return,
        }
    }
    if includes.is_empty() {
        return;
    }
    if includes.len() == 1 {
        let home = decl.parent();
        let repeated = ancestors(includes[0]).take_while(|a| !home.is_some_and(|h| h.id() == a.id())).any(|a| fn_like(a.kind()) || matches!(a.kind(), "for_statement" | "for_in_statement" | "while_statement" | "do_statement"));
        if !repeated {
            return;
        }
    }
    edits.push(TextEdit::new(value.start_byte(), value.end_byte(), format!("new Set({})", text(value, src))));
    let mut suggest = false;
    if let Some(t) = n.child_by_field_name("type") {
        let ty = text(t, src).trim_start_matches(':').trim();
        let set = if let Some(e) = ty.strip_prefix("readonly ").and_then(|x| x.strip_suffix("[]")) {
            Some(format!("ReadonlySet<{e}>"))
        } else if let Some(e) = ty.strip_suffix("[]") {
            Some(format!("Set<{e}>"))
        } else if let Some(e) = ty.strip_prefix("ReadonlyArray<").and_then(|x| x.strip_suffix('>')) {
            Some(format!("ReadonlySet<{e}>"))
        } else {
            ty.strip_prefix("Array<").and_then(|x| x.strip_suffix('>')).map(|e| format!("Set<{e}>"))
        };
        match set {
            Some(s) => edits.push(TextEdit::new(t.start_byte(), t.end_byte(), format!(": {s}"))),
            None => suggest = true,
        }
    }
    let edits = if suggest { as_suggestion(edits) } else { edits };
    out.push(finding(name, format!("`{}` is only searched with `includes()`: make it a `Set` and use `has()`", text(name, src)), edits));
});

// ---- unicorn: prefer-export-from ---------------------------------------------

/// The import statement, specifier and the name it imports, for an
/// imported local binding.
fn import_parts<'t>(root: Node<'t>, m: &SemanticModel, sym: SymbolId, src: &str) -> Option<(Node<'t>, Node<'t>, String, Option<Node<'t>>)> {
    let (a, b) = *m.symbol(sym).decls.first()?;
    let id = node_at(root, a, b)?;
    let stmt = ancestors(id).find(|x| x.kind() == "import_statement")?;
    let p = id.parent()?;
    let (imported, spec) = match p.kind() {
        "import_specifier" => (p.child_by_field_name("name").map(|x| text(x, src).to_owned())?, Some(p)),
        "import_clause" => ("default".to_owned(), None),
        "namespace_import" => ("*".to_owned(), None),
        _ => return None,
    };
    let source = stmt.child_by_field_name("source")?;
    Some((stmt, source, imported, spec))
}

srule!(PreferExportFrom, "js/prefer-export-from", &["export_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let root = js_ast::root_of(n);
    // The export site: (local identifier, exported name, the node to remove).
    let mut sites: Vec<(Node<'_>, String, TextEdit, Node<'_>)> = Vec::new();
    if n.child_by_field_name("source").is_some() {
        return;
    }
    if let Some(v) = n.child_by_field_name("value").filter(|v| v.kind() == "identifier") {
        sites.push((v, "default".to_owned(), remove_statement(n, src), n));
    } else if let Some(clause) = named_children(n).into_iter().find(|c| c.kind() == "export_clause") {
        let specs: Vec<Node<'_>> = named_children(clause).into_iter().filter(|s| s.kind() == "export_specifier").collect();
        for s in &specs {
            let Some(local) = s.child_by_field_name("name") else { continue };
            let exported = s.child_by_field_name("alias").map_or_else(|| text(local, src).to_owned(), |a| text(a, src).to_owned());
            let rm = if specs.len() == 1 { remove_statement(n, src) } else { remove_list_item(*s, src) };
            sites.push((local, exported, rm, *s));
        }
    } else if let Some(d) = n.child_by_field_name("declaration").filter(|d| d.kind() == "lexical_declaration" && decl_kind(*d, src) == "const") {
        let Some(dec) = sole_declarator(d) else { return };
        let (Some(name), Some(v)) = (dec.child_by_field_name("name"), dec.child_by_field_name("value")) else { return };
        if name.kind() != "identifier" || v.kind() != "identifier" || dec.child_by_field_name("type").is_some() || n.prev_sibling().is_some_and(|c| c.kind() == "comment" && text(c, src).starts_with("/**")) {
            return;
        }
        if m.declared_at(name.start_byte()).is_some_and(|s| !m.symbol(s).refs.is_empty()) {
            return;
        }
        sites.push((v, text(name, src).to_owned(), remove_statement(n, src), n));
    }
    for (local, exported, rm, at) in sites {
        let Some(sym) = m.symbol_at(local.start_byte()) else { continue };
        if m.symbol(sym).kind != SymbolKind::Import || m.symbol(sym).decls.len() != 1 {
            continue;
        }
        let Some((stmt, source, imported, spec)) = import_parts(root, m, sym, src) else { continue };
        if imported == "*" && exported == "default" {
            continue;
        }
        let ty = if children(stmt).iter().any(|c| c.kind() == "type") || spec.is_some_and(|s| children(s).iter().any(|c| c.kind() == "type")) { "type " } else { "" };
        let line = if imported == "*" {
            format!("export {ty}* as {exported} from {};", text(source, src))
        } else if imported == exported {
            format!("export {ty}{{ {exported} }} from {};", text(source, src))
        } else {
            format!("export {ty}{{ {imported} as {exported} }} from {};", text(source, src))
        };
        let msg = format!("`{}` is imported only to be exported: write `{line}`", text(local, src));
        if has_comment(stmt) || has_comment(n) {
            out.push(finding(at, msg, vec![]));
            continue;
        }
        let mut edits = vec![rm];
        let only_use = m.symbol(sym).refs.len() == 1;
        if only_use {
            let specifiers: Vec<Node<'_>> = {
                let mut v = Vec::new();
                crate::util::descendants(stmt, &|_| false, &mut v);
                v.into_iter().filter(|x| matches!(x.kind(), "import_specifier" | "namespace_import") || (x.kind() == "identifier" && x.parent().is_some_and(|p| p.kind() == "import_clause"))).collect()
            };
            if specifiers.len() == 1 {
                edits.push(remove_statement(stmt, src));
            } else if let Some(s) = spec {
                let named = s.parent().filter(|p| p.kind() == "named_imports");
                if named.is_some_and(|nm| named_children(nm).iter().filter(|c| c.kind() == "import_specifier").count() == 1) {
                    out.push(finding(at, msg, vec![]));
                    continue;
                }
                edits.push(remove_list_item(s, src));
            } else {
                out.push(finding(at, msg, vec![]));
                continue;
            }
        }
        let end = src.len();
        let nl = if src.ends_with('\n') { "" } else { "\n" };
        edits.push(TextEdit::insert(end, format!("{nl}{line}\n")));
        out.push(finding(at, msg, edits));
    }
});

// ---- unicorn: no-declarations-before-early-exit -------------------------------

fn always_exits(n: Node<'_>, src: &str) -> bool {
    crate::util::exits(&n)
        || (n.kind() == "expression_statement" && n.named_child(0).is_some_and(|c| c.kind() == "call_expression" && c.child_by_field_name("function").is_some_and(|f| text(f, src) == "process.exit")))
        || (n.kind() == "statement_block" && named_children(n).iter().any(|c| always_exits(*c, src)))
}

fn guard(n: Node<'_>, src: &str) -> bool {
    if n.kind() != "if_statement" {
        return false;
    }
    let c = n.child_by_field_name("consequence").is_some_and(|c| always_exits(c, src));
    let a = n.child_by_field_name("alternative").and_then(|a| named_children(a).into_iter().find(|x| x.kind() != "comment")).is_some_and(|a| always_exits(a, src));
    c != a
}

fn simple_init(v: Option<Node<'_>>) -> bool {
    v.is_none_or(|v| literal(unparen(v)))
}

fn names_in(n: Node<'_>, src: &str) -> BTreeSet<String> {
    let mut v = Vec::new();
    crate::util::descendants(n, &|_| false, &mut v);
    v.push(n);
    v.into_iter().filter(|x| x.kind() == "identifier").map(|x| text(x, src).to_owned()).collect()
}

srule!(NoDeclarationsBeforeEarlyExit, "js/no-declarations-before-early-exit", &["program", "statement_block"], |n, ctx, m, out| {
    let src = ctx.source;
    let stmts: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    for (i, d) in stmts.iter().enumerate() {
        if d.kind() != "lexical_declaration" {
            continue;
        }
        let Some(j) = (i + 1..stmts.len()).find(|&j| guard(stmts[j], src)) else { continue };
        let g = stmts[j];
        let adjacent = j == i + 1;
        let decls: Vec<Node<'_>> = named_children(*d).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
        for dec in &decls {
            let Some(name) = dec.child_by_field_name("name").filter(|x| x.kind() == "identifier") else { continue };
            let Some(sym) = m.declared_at(name.start_byte()) else { continue };
            let refs = refs_of(m, sym);
            if refs.is_empty() || refs.iter().any(|r| r.start < g.end_byte()) {
                continue;
            }
            let init = dec.child_by_field_name("value");
            if let Some(v) = init {
                let t = text(v, src);
                let callee = v.child_by_field_name("function").map(|f| text(f, src));
                if callee.is_some_and(|c| c.strip_prefix("React.").unwrap_or(c).starts_with("use") && c.len() > 3) || t.contains("await") || t.contains("yield") {
                    continue;
                }
                let shared = !names_in(v, src).is_disjoint(&names_in(g.child_by_field_name("condition").unwrap_or(g), src));
                if shared && (side_effect(v) || g.child_by_field_name("condition").is_some_and(side_effect)) {
                    continue;
                }
            }
            if !adjacent && !simple_init(init) {
                continue;
            }
            let mut edits = Vec::new();
            if adjacent && decls.len() == 1 && simple_init(init) && !comment_between(n, d.start_byte(), g.end_byte()) && !d.prev_sibling().is_some_and(|c| c.kind() == "comment") && !g.next_sibling().is_some_and(|c| c.kind() == "comment" && c.start_position().row == g.end_position().row) {
                let between = &src[d.end_byte()..g.start_byte()];
                edits.push(TextEdit::new(d.start_byte(), g.end_byte(), format!("{}{between}{}", text(g, src), text(*d, src))));
            }
            out.push(finding(name, format!("`{}` is declared before an early exit but only used after it: declare it after the `if`", text(name, src)), edits));
        }
    }
});

// ---- unicorn: no-immediate-mutation ------------------------------------------

fn mentions_sym(n: Node<'_>, m: &SemanticModel, sym: SymbolId) -> bool {
    let mut v = Vec::new();
    crate::util::descendants(n, &|_| false, &mut v);
    v.push(n);
    v.into_iter().any(|x| x.kind() == "identifier" && m.symbol_at(x.start_byte()) == Some(sym))
}

fn list_items(list: Node<'_>) -> Vec<Node<'_>> {
    named_children(list).into_iter().filter(|c| c.kind() != "comment").collect()
}

/// The edit adding `items` to the end of an array or object literal.
fn append(list: Node<'_>, items: &str, src: &str) -> TextEdit {
    let close = list.end_byte() - 1;
    match list_items(list).last() {
        Some(last) => {
            let after = &src[last.end_byte()..close];
            if after.trim_start().starts_with(',') {
                TextEdit::insert(close, format!(" {items}"))
            } else {
                TextEdit::insert(last.end_byte(), format!(", {items}"))
            }
        }
        None => TextEdit::insert(close, items.to_owned()),
    }
}

fn prepend(list: Node<'_>, items: &str) -> TextEdit {
    match list_items(list).first() {
        Some(first) => TextEdit::insert(first.start_byte(), format!("{items}, ")),
        None => TextEdit::insert(list.start_byte() + 1, items.to_owned()),
    }
}

srule!(NoImmediateMutation, "js/no-immediate-mutation", &["program", "statement_block"], |n, ctx, m, out| {
    let src = ctx.source;
    let stmts: Vec<Node<'_>> = named_children(n).into_iter().filter(|c| c.kind() != "comment").collect();
    for w in stmts.windows(2) {
        let (first, next) = (w[0], w[1]);
        // The variable and its initial value.
        let (var, value) = match first.kind() {
            "lexical_declaration" | "variable_declaration" => {
                let Some(last) = named_children(first).into_iter().rfind(|c| c.kind() == "variable_declarator") else { continue };
                match (last.child_by_field_name("name"), last.child_by_field_name("value")) {
                    (Some(nm), Some(v)) if nm.kind() == "identifier" => (nm, v),
                    (Some(nm), None) if nm.kind() == "identifier" && (first.kind() != "lexical_declaration" || decl_kind(first, src) == "let") => {
                        // `let x;` then `x = v;`
                        let declarators: Vec<Node<'_>> = named_children(first).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
                        if declarators.iter().any(|d| d.child_by_field_name("value").is_some()) || next.kind() != "expression_statement" {
                            continue;
                        }
                        let Some(asg) = next.named_child(0).filter(|a| a.kind() == "assignment_expression") else { continue };
                        let (Some(l), Some(r)) = (asg.child_by_field_name("left"), asg.child_by_field_name("right")) else { continue };
                        let Some(dl) = declarators.iter().find(|d| d.child_by_field_name("name").is_some_and(|x| l.kind() == "identifier" && text(x, src) == text(l, src))) else { continue };
                        let Some(sym) = m.declared_at(dl.child_by_field_name("name").map_or(0, |x| x.start_byte())) else { continue };
                        if mentions_sym(r, m, sym) {
                            continue;
                        }
                        let simple = matches!(unparen(r).kind(), "identifier" | "string" | "number" | "true" | "false" | "null" | "arrow_function" | "function_expression") || (matches!(unparen(r).kind(), "array" | "object") && !side_effect(r));
                        let mut edits = Vec::new();
                        if !has_comment(next) && dl.child_by_field_name("type").is_none() {
                            let nm = dl.child_by_field_name("name").unwrap_or(*dl);
                            edits.push(TextEdit::insert(nm.end_byte(), format!(" = {}", text(r, src))));
                            edits.push(remove_statement(next, src));
                            if !simple || is_ts(ctx) {
                                edits = as_suggestion(edits);
                            }
                        }
                        out.push(finding(asg, format!("`{}` is assigned right after it is declared: give it the value in the declaration", text(l, src)), edits));
                        continue;
                    }
                    _ => continue,
                }
            }
            "expression_statement" => {
                let Some(asg) = first.named_child(0).filter(|a| a.kind() == "assignment_expression") else { continue };
                match (asg.child_by_field_name("left"), asg.child_by_field_name("right")) {
                    (Some(l), Some(r)) if l.kind() == "identifier" => (l, r),
                    _ => continue,
                }
            }
            _ => continue,
        };
        let Some(sym) = m.symbol_at(var.start_byte()) else { continue };
        let v = unparen(value);
        let vname = text(var, src);
        if next.kind() != "expression_statement" {
            continue;
        }
        let Some(e) = next.named_child(0) else { continue };
        let comments = has_comment(next) || has_comment(v);
        let on_var = |f: Node<'_>| f.kind() == "member_expression" && f.child_by_field_name("optional_chain").is_none() && f.child_by_field_name("object").is_some_and(|o| o.kind() == "identifier" && m.symbol_at(o.start_byte()) == Some(sym));
        let report = |at_start: usize, at_end: usize, what: &str, edits: Vec<TextEdit>, effects: bool, out: &mut Vec<NativeFinding>| {
            let edits = if comments { vec![] } else if effects { as_suggestion(edits) } else { edits };
            out.push(span(at_start, at_end, format!("`{vname}` is changed right after it is created: {what}"), edits));
        };
        match (v.kind(), e.kind()) {
            ("array", "call_expression") => {
                let Some(f) = e.child_by_field_name("function").filter(|f| on_var(*f)) else { continue };
                let method = f.child_by_field_name("property").map_or("", |p| text(p, src));
                let args = arguments(e);
                if !matches!(method, "push" | "unshift") || args.is_empty() || args.iter().any(|a| mentions_sym(*a, m, sym)) {
                    continue;
                }
                let items = args.iter().map(|a| text(*a, src)).collect::<Vec<_>>().join(", ");
                let effects = args.iter().any(|a| side_effect(*a));
                if method == "unshift" && list_items(v).iter().any(|x| side_effect(*x)) {
                    report(f.start_byte(), f.end_byte(), "put the items in the array literal", vec![], false, out);
                    continue;
                }
                let add = if method == "push" { append(v, &items, src) } else { prepend(v, &items) };
                report(f.start_byte(), f.end_byte(), "put the items in the array literal", vec![add, remove_statement(next, src)], effects, out);
            }
            ("object", "assignment_expression") => {
                let Some(l) = e.child_by_field_name("left").filter(|l| on_var(*l)) else { continue };
                let Some(r) = e.child_by_field_name("right") else { continue };
                let Some(p) = l.child_by_field_name("property").filter(|p| p.kind() == "property_identifier") else { continue };
                if mentions_sym(r, m, sym) {
                    continue;
                }
                let odd = list_items(v).iter().any(|x| x.kind() == "method_definition" || js_ast::member_name(*x, src) == Some("__proto__")) || text(p, src) == "__proto__";
                let end = e.child_by_field_name("right").map_or(e.end_byte(), |r| r.start_byte());
                let op_end = src[..end].trim_end().len();
                let edits = if odd { vec![] } else { vec![append(v, &format!("{}: {}", text(p, src), text(r, src)), src), remove_statement(next, src)] };
                report(l.start_byte(), op_end, "write the property in the object literal", edits, side_effect(r), out);
            }
            ("object", "call_expression") if e.child_by_field_name("function").is_some_and(|f| text(f, src) == "Object.assign") => {
                let args = arguments(e);
                if args.len() < 2 || !(args[0].kind() == "identifier" && m.symbol_at(args[0].start_byte()) == Some(sym)) || args[1].kind() == "spread_element" || args[1..].iter().any(|a| mentions_sym(*a, m, sym)) {
                    continue;
                }
                let f = e.child_by_field_name("function").unwrap_or(e);
                let s = unparen(args[1]);
                let odd = list_items(v).iter().any(|x| x.kind() == "method_definition") || (s.kind() == "object" && list_items(s).iter().any(|x| x.kind() == "method_definition"));
                let items = if s.kind() == "object" { list_items(s).iter().map(|x| text(*x, src)).collect::<Vec<_>>().join(", ") } else { format!("...{}", text(s, src)) };
                let edits = if odd || args.len() > 2 || items.is_empty() { vec![] } else { vec![append(v, &items, src), remove_statement(next, src)] };
                report(f.start_byte(), f.end_byte(), "write the properties in the object literal", edits, side_effect(s), out);
            }
            ("new_expression", "call_expression") => {
                let ctor = v.child_by_field_name("constructor").map_or("", |c| text(c, src));
                let Some(f) = e.child_by_field_name("function").filter(|f| on_var(*f)) else { continue };
                let method = f.child_by_field_name("property").map_or("", |p| text(p, src));
                let args = arguments(e);
                let item = match (ctor, method, &args[..]) {
                    ("Set" | "WeakSet", "add", [x]) => text(*x, src).to_owned(),
                    ("Map" | "WeakMap", "set", [k, x]) => format!("[{}, {}]", text(*k, src), text(*x, src)),
                    _ => continue,
                };
                if args.iter().any(|a| mentions_sym(*a, m, sym) || a.kind() == "spread_element") {
                    continue;
                }
                let cargs = arguments(v);
                let add = match &cargs[..] {
                    [] => v.child_by_field_name("arguments").map(|a| TextEdit::new(a.start_byte(), a.end_byte(), format!("([{item}])"))),
                    [a] if unparen(*a).kind() == "array" => Some(append(unparen(*a), &item, src)),
                    _ => None,
                };
                let Some(add) = add else { continue };
                let effects = args.iter().any(|a| side_effect(*a));
                report(f.start_byte(), f.end_byte(), "pass the entries to the constructor", vec![add, remove_statement(next, src)], effects, out);
            }
            _ => {}
        }
    }
});

// ---- unicorn: consistent-destructuring ------------------------------------------

fn this_boundary(n: Node<'_>) -> Option<usize> {
    ancestors(n).find(|a| (fn_like(a.kind()) && a.kind() != "arrow_function") || matches!(a.kind(), "class_static_block" | "field_definition" | "public_field_definition")).map(|a| a.id())
}

/// The symbol `name` resolves to at `offset`.
fn resolve(m: &SemanticModel, offset: usize, name: &str) -> Option<SymbolId> {
    let mut sc = Some(m.scope_at(offset));
    while let Some(s) = sc {
        if let Some(ids) = m.scopes[s].bindings.get(name)
            && let Some(id) = ids.iter().find(|&&i| m.symbol(i).kind.is_value())
        {
            return Some(*id);
        }
        sc = m.scopes[s].parent;
    }
    None
}

srule!(ConsistentDestructuring, "js/consistent-destructuring", &["lexical_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    if decl_kind(n, src) != "const" {
        return;
    }
    let Some(dec) = sole_declarator(n) else { return };
    let (Some(pat), Some(obj)) = (dec.child_by_field_name("name"), dec.child_by_field_name("value")) else { return };
    if pat.kind() != "object_pattern" || !matches!(obj.kind(), "identifier" | "this") {
        return;
    }
    let mut entries: Vec<(String, Node<'_>)> = Vec::new();
    for c in named_children(pat) {
        match c.kind() {
            "shorthand_property_identifier_pattern" => entries.push((text(c, src).to_owned(), c)),
            "pair_pattern" => {
                if let (Some(k), Some(v)) = (c.child_by_field_name("key"), c.child_by_field_name("value"))
                    && k.kind() == "property_identifier"
                    && v.kind() == "identifier"
                {
                    entries.push((text(k, src).to_owned(), v));
                }
            }
            _ => {}
        }
    }
    if entries.is_empty() {
        return;
    }
    let Some(scope_node) = n.parent() else { return };
    let objsym = (obj.kind() == "identifier").then(|| m.symbol_at(obj.start_byte())).flatten();
    let boundary = this_boundary(n);
    let objtext = text(obj, src);
    let mut stack = vec![scope_node];
    let mut members = Vec::new();
    while let Some(x) = stack.pop() {
        for c in named_children(x) {
            if c.start_byte() >= n.end_byte() && c.kind() == "member_expression" {
                members.push(c);
            }
            stack.push(c);
        }
    }
    members.sort_by_key(|x| x.start_byte());
    for mem in members {
        let Some(o) = mem.child_by_field_name("object") else { continue };
        if text(o, src) != objtext || mem.child_by_field_name("optional_chain").is_some() {
            continue;
        }
        let same_obj = match o.kind() {
            "identifier" => m.symbol_at(o.start_byte()) == objsym,
            "this" => this_boundary(o) == boundary,
            _ => false,
        };
        let Some(p) = mem.child_by_field_name("property").filter(|p| p.kind() == "property_identifier") else { continue };
        let Some((_, bound)) = entries.iter().find(|(k, _)| k == text(p, src)) else { continue };
        if !same_obj {
            continue;
        }
        let parent = mem.parent();
        if parent.is_some_and(|q| matches!(q.kind(), "call_expression" | "new_expression") && q.child(0).is_some_and(|c| c.id() == mem.id()))
            || parent.is_some_and(|q| q.kind() == "unary_expression" && text(q, src).starts_with("delete"))
            || parent.is_some_and(|q| matches!(q.kind(), "assignment_expression" | "augmented_assignment_expression" | "update_expression") && q.child(0).is_some_and(|c| c.id() == mem.id()))
        {
            continue;
        }
        let bname = text(*bound, src);
        let bsym = m.declared_at(bound.start_byte());
        if bsym.is_none() || resolve(m, mem.start_byte(), bname) != bsym {
            continue;
        }
        // Something in between may have changed the object.
        let between = (n.end_byte(), mem.start_byte());
        let changed = objsym.is_some_and(|s| refs_of(m, s).iter().any(|r| r.write && r.start > between.0 && r.start < between.1));
        let fnode = enclosing_fn(mem).map(|f| f.id());
        let mut calls = false;
        let mut writes = false;
        let mut st = vec![scope_node];
        while let Some(x) = st.pop() {
            if x.end_byte() <= between.0 || x.start_byte() >= between.1 {
                continue;
            }
            if x.start_byte() >= between.0 && x.end_byte() <= between.1 {
                if x.kind() == "call_expression" && enclosing_fn(x).map(|f| f.id()) == fnode {
                    calls = true;
                }
                if matches!(x.kind(), "assignment_expression" | "augmented_assignment_expression" | "update_expression") && x.child(0).is_some_and(|l| text(l, src).starts_with(objtext)) {
                    writes = true;
                }
            }
            st.extend(named_children(x));
        }
        let guarded = ancestors(mem).any(|a| a.kind() == "if_statement" && a.child_by_field_name("condition").is_some_and(|c| text(c, src).contains(&format!("in {objtext}"))));
        let in_loop = ancestors(mem).any(|a| matches!(a.kind(), "for_statement" | "for_in_statement" | "while_statement" | "do_statement") && a.start_byte() > n.end_byte() && objsym.is_some_and(|s| refs_of(m, s).iter().any(|r| r.write && in_range(r.start, a))));
        if changed || calls || writes || guarded || in_loop {
            continue;
        }
        let edits = if parent.is_some_and(|q| q.kind() == "member_expression" && same(q.child_by_field_name("object"), mem)) || has_comment(mem) { vec![] } else { vec![TextEdit::new(mem.start_byte(), mem.end_byte(), bname)] };
        out.push(finding(mem, format!("`{}` was destructured into `{bname}` above: use `{bname}`", text(mem, src)), edits));
    }
});

// ---- unicorn: no-blob-to-file ------------------------------------------------------

srule!(NoBlobToFile, "js/no-blob-to-file", &["lexical_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    if decl_kind(n, src) != "const" || !n.parent().is_some_and(statement_list) {
        return;
    }
    let Some(dec) = sole_declarator(n) else { return };
    let (Some(fname), Some(v)) = (dec.child_by_field_name("name"), dec.child_by_field_name("value")) else { return };
    let Some(file) = new_of(v, "File", m, src) else { return };
    let args = arguments(file);
    let [bits, filename] = args[..] else { return };
    let items = list_items(bits);
    let ([b], true) = (&items[..], bits.kind() == "array" && matches!(filename.kind(), "string" | "template_string")) else { return };
    if b.kind() != "identifier" || fname.kind() != "identifier" {
        return;
    }
    let root = js_ast::root_of(n);
    let Some(bsym) = m.symbol_at(b.start_byte()) else { return };
    if m.symbol(bsym).kind != SymbolKind::Const || m.symbol(bsym).decls[0].0 > n.start_byte() {
        return;
    }
    let Some(binit) = js_ast::initial_value(m, root, *b) else { return };
    let blob_ok = new_of(binit, "Blob", m, src).is_some_and(|x| arguments(x).len() <= 1) || new_of(binit, "File", m, src).is_some_and(|x| arguments(x).len() == 2);
    if !blob_ok {
        return;
    }
    let Some(fsym) = m.declared_at(fname.start_byte()) else { return };
    let frefs = refs_of(m, fsym);
    let [r] = frefs[..] else { return };
    let Some(use_) = node_at(root, r.start, r.end) else { return };
    let call = use_.parent().filter(|p| p.kind() == "arguments").and_then(|a| a.parent());
    let Some(call) = call else { return };
    let cargs = arguments(call);
    let Some(f) = call.child_by_field_name("function") else { return };
    let mut extra = None;
    let ok = if text(f, src) == "URL.createObjectURL" && f.child_by_field_name("object").is_some_and(|o| unresolved(m, o, "URL", src)) {
        cargs.len() == 1
    } else if f.kind() == "member_expression" && matches!(f.child_by_field_name("property").map(|p| text(p, src)), Some("append" | "set")) && f.child_by_field_name("optional_chain").is_none() {
        let fd = f.child_by_field_name("object").filter(|o| o.kind() == "identifier");
        let fd_ok = fd.and_then(|o| m.symbol_at(o.start_byte())).is_some_and(|s| m.symbol(s).kind == SymbolKind::Const) && fd.and_then(|o| js_ast::initial_value(m, root, o)).and_then(|x| new_of(x, "FormData", m, src)).is_some_and(|x| arguments(x).is_empty());
        if cargs.len() == 2 {
            extra = Some(format!(", {}", text(filename, src)));
        }
        fd_ok && cargs.get(1).is_some_and(|a| a.id() == use_.id()) && cargs.len() <= 3
    } else {
        false
    };
    if !ok {
        return;
    }
    let mut edits = Vec::new();
    if !comment_between(root, n.start_byte(), call.end_byte()) {
        edits.push(remove_statement(n, src));
        edits.push(TextEdit::new(use_.start_byte(), use_.end_byte(), text(*b, src)));
        if let Some(e) = extra {
            edits.push(TextEdit::insert(use_.end_byte(), e));
        }
    }
    out.push(finding(file, format!("`{}` is wrapped in a `File` only to be passed on: pass the `Blob` itself", text(*b, src)), edits));
});

// ---- unicorn: prefer-promise-with-resolvers ---------------------------------------

srule!(PreferPromiseWithResolvers, "js/prefer-promise-with-resolvers", &["new_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(ctor) = n.child_by_field_name("constructor") else { return };
    if !unresolved(m, ctor, "Promise", src) {
        return;
    }
    let args = arguments(n);
    let [exec] = args[..] else { return };
    let Some(f) = js_ast::as_function(exec).filter(|f| matches!(f.kind(), "arrow_function" | "function_expression" | "function") && !js_ast::is_async(*f)) else { return };
    let Some(body) = js_ast::body(f).filter(|b| b.kind() == "statement_block") else { return };
    let params = js_ast::params(f);
    let pnames: Vec<&str> = params.iter().filter_map(|p| js_ast::param_name(*p, src)).collect();
    if params.is_empty() || params.len() > 2 || pnames.len() != params.len() || (pnames.len() == 2 && pnames[0] == pnames[1]) {
        return;
    }
    let stmts: Vec<Node<'_>> = named_children(body).into_iter().filter(|c| c.kind() != "comment").collect();
    let mut targets: [Option<Node<'_>>; 2] = [None, None];
    for s in &stmts {
        let asg = (s.kind() == "expression_statement").then(|| s.named_child(0)).flatten().filter(|a| a.kind() == "assignment_expression");
        let Some(asg) = asg else { return };
        let (Some(l), Some(r)) = (asg.child_by_field_name("left"), asg.child_by_field_name("right")) else { return };
        if l.kind() != "identifier" || r.kind() != "identifier" || src[l.end_byte()..r.start_byte()].trim() != "=" {
            return;
        }
        let Some(i) = pnames.iter().position(|p| *p == text(r, src)) else { return };
        if targets[i].is_some() || m.symbol_at(l.start_byte()).is_none_or(|s| m.symbol(s).kind == SymbolKind::Param) {
            return;
        }
        targets[i] = Some(l);
    }
    if targets.iter().all(Option::is_none) {
        return;
    }
    let msg = "This `new Promise()` only hands its resolvers out: use `Promise.withResolvers()`";
    let mut edits = Vec::new();
    let decl = n.parent().filter(|p| p.kind() == "variable_declarator").and_then(|d| d.parent().map(|x| (d, x)));
    if let Some((dec, d)) = decl
        && decl_kind(d, src) == "const"
        && sole_declarator(d).is_some()
        && dec.child_by_field_name("name").is_some_and(|x| x.kind() == "identifier")
        && dec.child_by_field_name("type").is_none()
        && !params.iter().any(|p| p.child_by_field_name("type").is_some())
        && !has_comment(d)
    {
        let names: BTreeSet<&str> = targets.iter().flatten().map(|t| text(*t, src)).collect();
        let mut first = d;
        let mut declared: BTreeSet<&str> = BTreeSet::new();
        let mut cur = d.prev_named_sibling();
        while let Some(p) = cur {
            if p.kind() != "lexical_declaration" || decl_kind(p, src) != "let" {
                break;
            }
            let ds: Vec<Node<'_>> = named_children(p).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
            if ds.iter().any(|x| x.child_by_field_name("value").is_some() || x.child_by_field_name("type").is_some()) || !ds.iter().all(|x| x.child_by_field_name("name").is_some_and(|nm| names.contains(text(nm, src)))) {
                break;
            }
            declared.extend(ds.iter().filter_map(|x| x.child_by_field_name("name")).map(|nm| text(nm, src)));
            first = p;
            cur = p.prev_named_sibling();
        }
        let written_elsewhere = targets.iter().flatten().any(|t| m.symbol_at(t.start_byte()).is_some_and(|s| refs_of(m, s).iter().any(|r| r.write && !in_range(r.start, f))));
        if declared == names && !written_elsewhere && first.id() != d.id() && !comment_between(js_ast::root_of(n), first.start_byte(), d.end_byte()) {
            let pname = text(dec.child_by_field_name("name").unwrap_or(dec), src);
            let part = |key: &str, local: &str| if key == local { key.to_owned() } else { format!("{key}: {local}") };
            let mut parts = vec![part("promise", pname)];
            if let Some(t) = targets[0] {
                parts.push(part("resolve", text(t, src)));
            }
            if let Some(t) = targets[1] {
                parts.push(part("reject", text(t, src)));
            }
            let targs = n.child_by_field_name("type_arguments").map_or("", |t| text(t, src));
            edits.push(TextEdit::new(first.start_byte(), d.end_byte(), format!("const {{ {} }} = Promise.withResolvers{targs}();", parts.join(", "))));
        }
    }
    out.push(finding(n, msg, edits));
});

// ---- unicorn: prefer-abort-signal-timeout --------------------------------------------

srule!(PreferAbortSignalTimeout, "js/prefer-abort-signal-timeout", &["lexical_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    if decl_kind(n, src) != "const" || !n.parent().is_some_and(statement_list) {
        return;
    }
    let Some(dec) = sole_declarator(n) else { return };
    let (Some(name), Some(v)) = (dec.child_by_field_name("name"), dec.child_by_field_name("value")) else { return };
    if name.kind() != "identifier" || !new_of(v, "AbortController", m, src).is_some_and(|x| arguments(x).is_empty()) || m.symbols.iter().any(|s| s.name == "AbortSignal") {
        return;
    }
    let Some(next) = n.next_named_sibling().filter(|x| x.kind() == "expression_statement") else { return };
    let Some(call) = next.named_child(0).filter(|c| c.kind() == "call_expression") else { return };
    if !call.child_by_field_name("function").is_some_and(|f| unresolved(m, f, "setTimeout", src)) {
        return;
    }
    let args = arguments(call);
    let [cb, delay] = args[..] else { return };
    let Some(cbf) = js_ast::as_function(cb).filter(|f| js_ast::params(*f).is_empty() && !js_ast::is_async(*f) && !matches!(f.kind(), "generator_function")) else { return };
    let Some(cbody) = js_ast::body(cbf) else { return };
    let inner = if cbody.kind() == "statement_block" {
        let s: Vec<Node<'_>> = named_children(cbody).into_iter().filter(|c| c.kind() != "comment").collect();
        match s[..] {
            [x] if x.kind() == "expression_statement" => x.named_child(0),
            _ => None,
        }
    } else {
        Some(cbody)
    };
    let cname = text(name, src);
    if !inner.is_some_and(|c| c.kind() == "call_expression" && arguments(c).is_empty() && c.child_by_field_name("function").is_some_and(|f| text(f, src) == format!("{cname}.abort"))) {
        return;
    }
    if delay.kind() == "sequence_expression" {
        return;
    }
    if delay.kind() == "number" && !text(delay, src).parse::<u64>().is_ok_and(|d| d <= 2_147_483_647) {
        return;
    }
    let Some(sym) = m.declared_at(name.start_byte()) else { return };
    let root = js_ast::root_of(n);
    let mut signals = Vec::new();
    for r in refs_of(m, sym) {
        let Some(id) = node_at(root, r.start, r.end) else { return };
        if in_range(r.start, next) {
            if in_range(r.start, cbody) && id.parent().is_some_and(|p| p.parent().is_some_and(|c| c.kind() == "call_expression")) && id.parent().and_then(|p| p.child_by_field_name("property")).is_some_and(|p| text(p, src) == "abort") {
                continue;
            }
            return;
        }
        let Some(p) = id.parent().filter(|p| p.kind() == "member_expression" && p.child_by_field_name("property").is_some_and(|x| text(x, src) == "signal")) else { return };
        let gp = p.parent();
        if gp.is_some_and(|g| (g.kind() == "member_expression" && same(g.child_by_field_name("object"), p)) || (matches!(g.kind(), "assignment_expression" | "variable_declarator") && g.child(0).is_some_and(|c| c.id() == p.id()))) {
            return;
        }
        signals.push(p);
    }
    if signals.is_empty() {
        return;
    }
    let new_name = match cname {
        "abortController" => "abortSignal",
        "controller" => "signal",
        other => other,
    };
    let new_name = if new_name != cname && m.symbols.iter().any(|s| s.name == new_name) { cname } else { new_name };
    let mut edits = vec![TextEdit::new(name.start_byte(), name.end_byte(), new_name), TextEdit::new(v.start_byte(), v.end_byte(), format!("AbortSignal.timeout({})", text(delay, src))), remove_statement(next, src)];
    for s in signals {
        edits.push(TextEdit::new(s.start_byte(), s.end_byte(), new_name));
    }
    if comment_between(root, n.start_byte(), next.end_byte()) {
        edits.clear();
    }
    out.push(finding(name, format!("`{cname}` only aborts on a timer: use `AbortSignal.timeout({})`", text(delay, src)), edits));
});

// ---- unicorn: no-for-loop ------------------------------------------------------------

pub(super) fn singular(name: &str) -> String {
    let s = if let Some(b) = name.strip_suffix("ies") {
        format!("{b}y")
    } else if let Some(b) = name.strip_suffix("es").filter(|b| b.ends_with(['x', 's', 'z']) || b.ends_with("ch") || b.ends_with("sh")) {
        b.to_owned()
    } else if let Some(b) = name.strip_suffix('s').filter(|b| !b.ends_with('s') && !b.is_empty()) {
        b.to_owned()
    } else {
        name.to_owned()
    };
    if s == name || s.is_empty() { "element".to_owned() } else { s }
}

srule!(NoForLoop, "js/no-for-loop", &["for_statement"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(init) = n.child_by_field_name("initializer") else { return };
    if !matches!(init.kind(), "lexical_declaration" | "variable_declaration") {
        return;
    }
    let ds: Vec<Node<'_>> = named_children(init).into_iter().filter(|c| c.kind() == "variable_declarator").collect();
    let Some(idecl) = ds.first() else { return };
    let (Some(iname), Some(ival)) = (idecl.child_by_field_name("name"), idecl.child_by_field_name("value")) else { return };
    if iname.kind() != "identifier" || text(ival, src) != "0" || ds.len() > 2 {
        return;
    }
    let i = text(iname, src);
    let len = ds.get(1).and_then(|d| Some((d.child_by_field_name("name")?, d.child_by_field_name("value")?)));
    let Some(mut test) = n.child_by_field_name("condition") else { return };
    if test.kind() == "expression_statement" {
        let Some(t) = test.named_child(0) else { return };
        test = t;
    }
    let (Some(l), Some(op), Some(r)) = (test.child_by_field_name("left"), test.child_by_field_name("operator"), test.child_by_field_name("right")) else { return };
    let bound = match text(op, src) {
        "<" if text(l, src) == i => r,
        ">" if text(r, src) == i => l,
        _ => return,
    };
    let arr_text = match (len, bound.kind()) {
        (Some((ln, lv)), "identifier") if text(ln, src) == text(bound, src) => text(lv, src).strip_suffix(".length").map(str::to_owned),
        (None, "member_expression") => text(bound, src).strip_suffix(".length").map(str::to_owned),
        _ => None,
    };
    let Some(arr) = arr_text.filter(|a| !a.is_empty() && a.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$')) else { return };
    let Some(update) = n.child_by_field_name("increment") else { return };
    let up = text(update, src).replace(' ', "");
    let standard = up == format!("{i}++") || up == format!("++{i}") || up == format!("{i}+=1");
    if !standard && up != format!("{i}={i}+1") && up != format!("{i}=1+{i}") && !(len.is_some() && up.starts_with(&format!("{i}++,"))) {
        return;
    }
    let Some(body) = n.child_by_field_name("body").filter(|b| b.kind() == "statement_block") else { return };
    let Some(isym) = m.declared_at(iname.start_byte()) else { return };
    let irefs = refs_of(m, isym);
    if irefs.iter().any(|r| r.write && in_range(r.start, body)) {
        return;
    }
    // Every use of the array in the body is `arr[i]`.
    let mut accesses = Vec::new();
    let mut all = Vec::new();
    crate::util::descendants(body, &|_| false, &mut all);
    for x in all {
        if x.kind() != "identifier" || text(x, src) != arr {
            continue;
        }
        if accesses.iter().any(|a: &Node<'_>| a.child_by_field_name("object").is_some_and(|o| o.id() == x.id())) {
            continue;
        }
        let Some(p) = x.parent().filter(|p| p.kind() == "subscript_expression" && same(p.child_by_field_name("object"), x) && p.child_by_field_name("index").is_some_and(|ix| text(ix, src) == i)) else { return };
        let gp = p.parent();
        if gp.is_some_and(|g| matches!(g.kind(), "assignment_expression" | "augmented_assignment_expression" | "update_expression") && g.child(0).is_some_and(|c| c.id() == p.id())) {
            return;
        }
        accesses.push(p);
    }
    if accesses.is_empty() {
        return;
    }
    let (open, close) = (children(n).into_iter().find(|c| c.kind() == "("), children(n).into_iter().find(|c| c.kind() == ")"));
    let (Some(open), Some(close)) = (open, close) else { return };
    let report = |edits: Vec<TextEdit>, out: &mut Vec<NativeFinding>| out.push(span(n.start_byte(), close.end_byte(), format!("Use a `for…of` loop over `{arr}` instead of this index loop"), edits));
    // The fix.
    let only_access = irefs.iter().all(|r| !in_range(r.start, body) || accesses.iter().any(|a| a.child_by_field_name("index").is_some_and(|ix| ix.start_byte() == r.start)));
    let first_stmt = named_children(body).into_iter().find(|c| c.kind() != "comment");
    let elem_decl = first_stmt.filter(|s| s.kind() == "lexical_declaration").and_then(|s| sole_declarator(s).map(|d| (s, d))).filter(|(_, d)| d.child_by_field_name("value").is_some_and(|v| accesses.iter().any(|a| a.id() == v.id())));
    let blocked = !standard
        || comment_between(n, open.start_byte(), close.end_byte())
        || irefs.iter().any(|r| !in_range(r.start, n))
        || len.is_some_and(|(ln, _)| m.declared_at(ln.start_byte()).is_some_and(|s| refs_of(m, s).iter().any(|r| !in_range(r.start, test))))
        || accesses.iter().any(|a| a.parent().is_some_and(|g| (g.kind() == "call_expression" && same(g.child_by_field_name("function"), *a)) || (g.kind() == "unary_expression" && text(g, src).starts_with("delete"))));
    if blocked {
        report(vec![], out);
        return;
    }
    let (kind, pattern, elem_node) = match elem_decl {
        Some((s, d)) => {
            if d.child_by_field_name("type").is_some() {
                report(vec![], out);
                return;
            }
            (decl_kind(s, src).to_owned(), text(d.child_by_field_name("name").unwrap_or(d), src).to_owned(), Some(s))
        }
        None => ("const".to_owned(), singular(&arr), None),
    };
    if elem_node.is_none() && (m.symbols.iter().any(|s| s.name == pattern) || crate::util::mentions(text(body, src), &pattern)) {
        report(vec![], out);
        return;
    }
    if let Some(s) = elem_node
        && let Some(es) = sole_declarator(s).and_then(|d| d.child_by_field_name("name")).and_then(|x| m.declared_at(x.start_byte()))
        && refs_of(m, es).iter().any(|r| r.write)
        && accesses.len() > 1
    {
        report(vec![], out);
        return;
    }
    let elem = if elem_node.is_some() && !pattern.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$') {
        // A destructuring pattern can't stand in for `arr[i]` elsewhere.
        if accesses.len() > 1 {
            report(vec![], out);
            return;
        }
        pattern.clone()
    } else {
        pattern.clone()
    };
    let head = if only_access { format!("({kind} {pattern} of {arr})") } else { format!("({kind} [{i}, {pattern}] of {arr}.entries())") };
    let mut edits = vec![TextEdit::new(open.start_byte(), close.end_byte(), head)];
    for a in &accesses {
        if elem_node.is_some_and(|s| in_range(a.start_byte(), s)) {
            continue;
        }
        edits.push(TextEdit::new(a.start_byte(), a.end_byte(), elem.clone()));
    }
    if let Some(s) = elem_node {
        edits.push(remove_statement(s, src));
    }
    report(edits, out);
});

// ---- unicorn: prefer-smaller-scope ------------------------------------------------------

fn boundary_between(n: Node<'_>, outer: Node<'_>) -> bool {
    ancestors(n).take_while(|a| a.id() != outer.id()).any(|a| fn_like(a.kind()) || matches!(a.kind(), "class" | "class_declaration" | "class_static_block" | "with_statement"))
}

srule!(PreferSmallerScope, "js/prefer-smaller-scope", &["lexical_declaration"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(parent) = n.parent().filter(|p| matches!(p.kind(), "program" | "statement_block")) else { return };
    let Some(dec) = sole_declarator(n) else { return };
    let Some(name) = dec.child_by_field_name("name").filter(|x| x.kind() == "identifier") else { return };
    let Some(sym) = m.declared_at(name.start_byte()) else { return };
    if m.symbol(sym).exported || text(parent, src).contains("eval(") || named_children(parent).iter().any(|c| c.kind() == "with_statement") {
        return;
    }
    let root = js_ast::root_of(n);
    let refs = refs_of(m, sym);
    if refs.is_empty() {
        return;
    }
    let kind = decl_kind(n, src);
    let msg = format!("Move `{}` into the block where it is used", text(name, src));
    let typed = dec.child_by_field_name("type").is_some();
    let commented = n.prev_sibling().is_some_and(|c| c.kind() == "comment") || has_comment(n);
    match dec.child_by_field_name("value") {
        None if kind == "let" => {
            let writes: Vec<&&Reference> = refs.iter().filter(|r| r.write).collect();
            let [w] = writes[..] else { return };
            if !refs.iter().any(|r| r.read) {
                return;
            }
            let Some(wid) = node_at(root, w.start, w.end) else { return };
            let Some(asg) = wid.parent().filter(|a| a.kind() == "assignment_expression" && same(a.child_by_field_name("left"), wid) && src[wid.end_byte()..].trim_start().starts_with('=') && !src[wid.end_byte()..].trim_start().starts_with("==")) else { return };
            let Some(stmt) = asg.parent().filter(|s| s.kind() == "expression_statement") else { return };
            let Some(block) = stmt.parent().filter(|b| b.kind() == "statement_block" && b.id() != parent.id()) else { return };
            if !ancestors(block).any(|a| a.id() == parent.id()) || boundary_between(block, parent) || stmt.start_byte() < n.end_byte() {
                return;
            }
            if refs.iter().any(|r| !in_range(r.start, block) || (r.read && r.start < stmt.end_byte()) || node_at(root, r.start, r.end).is_some_and(|x| boundary_between(x, block))) {
                return;
            }
            let rhs = asg.child_by_field_name("right");
            let mut edits = Vec::new();
            if !typed && !commented && !has_comment(stmt) && !rhs.is_some_and(|r| r.kind() == "parenthesized_expression") {
                edits.push(remove_statement(n, src));
                edits.push(TextEdit::insert(stmt.start_byte(), "const "));
            }
            out.push(finding(name, msg, edits));
        }
        Some(v) => {
            if !literal(unparen(v)) || unparen(v).kind() == "regex" {
                return;
            }
            let Some(next) = n.next_named_sibling().filter(|x| x.kind() == "if_statement") else { return };
            let branches = [next.child_by_field_name("consequence"), next.child_by_field_name("alternative").and_then(|a| named_children(a).into_iter().find(|x| x.kind() != "comment"))];
            let Some(branch) = branches.into_iter().flatten().find(|b| b.kind() == "statement_block" && refs.iter().all(|r| in_range(r.start, *b))) else { return };
            if refs.iter().any(|r| node_at(root, r.start, r.end).is_some_and(|x| boundary_between(x, branch))) {
                return;
            }
            let mut edits = Vec::new();
            if !typed && !commented
                && let Some(first) = named_children(branch).into_iter().find(|x| x.kind() != "comment")
            {
                let ind = if first.start_position().row > branch.start_position().row { format!("\n{}", indent_of(src, first.start_byte())) } else { " ".to_owned() };
                edits.push(remove_statement(n, src));
                edits.push(TextEdit::insert(first.start_byte(), format!("{}{}", text(n, src), ind)));
            }
            out.push(finding(name, msg, edits));
        }
        None => {}
    }
});

// ---- unicorn: no-unnecessary-default-arguments ------------------------------------------

fn default_of(p: Node<'_>) -> Option<Node<'_>> {
    match p.kind() {
        "assignment_pattern" => p.child_by_field_name("right"),
        "required_parameter" | "optional_parameter" => p.child_by_field_name("value"),
        _ => None,
    }
}

fn primitive(n: Node<'_>, src: &str) -> Option<String> {
    let n = unparen(n);
    match n.kind() {
        "unary_expression" if matches!(n.child_by_field_name("operator").map(|o| text(o, src)), Some("-" | "+")) => {
            let a = n.child_by_field_name("argument")?;
            (a.kind() == "number").then(|| text(n, src).replace(' ', ""))
        }
        "number" => {
            let t = text(n, src).replace('_', "");
            Some(t.parse::<f64>().map_or(t.clone(), |v| v.to_string()))
        }
        "string" => Some(format!("s:{}", crate::util::unquote(text(n, src)))),
        "template_string" if !named_children(n).iter().any(|c| c.kind() == "template_substitution") => Some(format!("s:{}", text(n, src).trim_matches('`'))),
        "true" | "false" | "null" => Some(text(n, src).to_owned()),
        _ => None,
    }
}

srule!(NoUnnecessaryDefaultArguments, "js/no-unnecessary-default-arguments", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(callee) = n.child_by_field_name("function").filter(|f| f.kind() == "identifier") else { return };
    let Some(sym) = m.symbol_at(callee.start_byte()) else { return };
    let s = m.symbol(sym);
    if s.decls.len() != 1 || refs_of(m, sym).iter().any(|r| r.write) {
        return;
    }
    let root = js_ast::root_of(n);
    let (a, b) = s.decls[0];
    let Some(id) = node_at(root, a, b) else { return };
    let f = match s.kind {
        SymbolKind::Function => id.parent().filter(|p| p.kind() == "function_declaration"),
        SymbolKind::Const => js_ast::initial_value(m, root, id).and_then(js_ast::as_function),
        _ => None,
    };
    let Some(f) = f else { return };
    if f.child_by_field_name("type_parameters").is_some() || crate::util::mentions(text(f, src), "arguments") || src.contains("eval(") {
        return;
    }
    let params = js_ast::params(f);
    let args = arguments(n);
    if args.iter().any(|x| x.kind() == "spread_element") || args.len() > params.len() {
        return;
    }
    let mut first = args.len();
    while first > 0 {
        let k = first - 1;
        let Some(d) = default_of(params[k]).and_then(|d| primitive(d, src)) else { break };
        if primitive(args[k], src).as_deref() != Some(d.as_str()) {
            break;
        }
        first = k;
    }
    if first == args.len() {
        return;
    }
    let Some(list) = n.child_by_field_name("arguments") else { return };
    let start = if first == 0 { list.start_byte() + 1 } else { args[first - 1].end_byte() };
    let end = list.end_byte() - 1;
    let mut edits = if comment_between(list, start, end) { vec![] } else { vec![TextEdit::delete(start, end)] };
    if is_ts(ctx) {
        edits = as_suggestion(edits);
    }
    let removed = args.len() - first;
    out.push(finding(args[first], format!("{} the parameter's default: leave {} out", if removed == 1 { "This argument is" } else { "These arguments are" }, if removed == 1 { "it" } else { "them" }), edits));
});

// ---- unicorn: no-unsafe-promise-all-settled-values ----------------------------------------

fn all_settled(n: Node<'_>, m: &SemanticModel, src: &str) -> bool {
    let n = unparen(n);
    n.kind() == "call_expression"
        && n.child_by_field_name("function").is_some_and(|f| f.kind() == "member_expression" && f.child_by_field_name("optional_chain").is_none() && text(f, src) == "Promise.allSettled" && f.child_by_field_name("object").is_some_and(|o| unresolved(m, o, "Promise", src)))
        && arguments(n).len() == 1
}

/// Whether `n` is an array of settled results: `await Promise.allSettled(x)`,
/// a `.filter()` of one, or a `const` bound to one; or a parameter of a
/// `Promise.allSettled(x).then(cb)` callback, read in that callback.
fn settled(n: Node<'_>, root: Node<'_>, m: &SemanticModel, src: &str, depth: usize) -> bool {
    let n = unparen(n);
    match n.kind() {
        "await_expression" => n.named_child(0).is_some_and(|x| all_settled(x, m, src)),
        "call_expression" => n.child_by_field_name("function").is_some_and(|f| f.kind() == "member_expression" && f.child_by_field_name("property").is_some_and(|p| text(p, src) == "filter") && f.child_by_field_name("object").is_some_and(|o| settled(o, root, m, src, depth))),
        "identifier" if depth < 4 => {
            let Some(sym) = m.symbol_at(n.start_byte()) else { return false };
            let s = m.symbol(sym);
            if s.decls.len() != 1 {
                return false;
            }
            match s.kind {
                SymbolKind::Const => js_ast::initial_value(m, root, n).is_some_and(|v| settled(v, root, m, src, depth + 1)),
                SymbolKind::Param => {
                    let Some(decl) = node_at(root, s.decls[0].0, s.decls[0].1) else { return false };
                    let Some(cb) = enclosing_fn(decl) else { return false };
                    if !js_ast::params(cb).first().is_some_and(|p| p.start_byte() <= decl.start_byte() && p.end_byte() >= decl.end_byte()) || enclosing_fn(n).map(|f| f.id()) != Some(cb.id()) {
                        return false;
                    }
                    let call = cb.parent().filter(|p| p.kind() == "arguments").and_then(|a| a.parent());
                    call.is_some_and(|c| c.child_by_field_name("function").is_some_and(|f| f.kind() == "member_expression" && f.child_by_field_name("property").is_some_and(|p| text(p, src) == "then") && f.child_by_field_name("object").is_some_and(|o| all_settled(o, m, src))))
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// `x.status === 'fulfilled'` (`Some(true)`), or a test that it is not
/// (`Some(false)`), where `status` is `<param>.status` or its destructured name.
fn status_check(t: Node<'_>, status: &[String], src: &str) -> Option<bool> {
    let t = unparen(t);
    if t.kind() != "binary_expression" {
        return None;
    }
    let op = text(t.child_by_field_name("operator")?, src);
    let (l, r) = (t.child_by_field_name("left")?, t.child_by_field_name("right")?);
    let lit = |x: Node<'_>| matches!(x.kind(), "string" | "template_string").then(|| text(x, src).trim_matches(['"', '\'', '`']).to_owned());
    let (val, other) = match (lit(l), lit(r)) {
        (Some(v), None) => (v, r),
        (None, Some(v)) => (v, l),
        _ => return None,
    };
    if !status.iter().any(|s| s == text(other, src)) {
        return None;
    }
    let eq = match op {
        "===" | "==" => true,
        "!==" | "!=" => false,
        _ => return None,
    };
    match val.as_str() {
        "fulfilled" => Some(eq),
        "rejected" => Some(!eq),
        _ => None,
    }
}

fn guarded(read: Node<'_>, cb: Node<'_>, status: &[String], src: &str) -> bool {
    let mut child = read;
    for a in ancestors(read) {
        if a.id() == cb.id() {
            break;
        }
        match a.kind() {
            "ternary_expression" => {
                let c = a.child_by_field_name("condition").and_then(|c| status_check(c, status, src));
                if (same(a.child_by_field_name("consequence"), child) && c == Some(true)) || (same(a.child_by_field_name("alternative"), child) && c == Some(false)) {
                    return true;
                }
            }
            "binary_expression" if same(a.child_by_field_name("right"), child) => {
                let c = a.child_by_field_name("left").and_then(|l| status_check(l, status, src));
                match a.child_by_field_name("operator").map(|o| text(o, src)) {
                    Some("&&") if c == Some(true) => return true,
                    Some("||") if c == Some(false) => return true,
                    _ => {}
                }
            }
            "if_statement" => {
                let c = a.child_by_field_name("condition").and_then(|l| status_check(l, status, src));
                if (same(a.child_by_field_name("consequence"), child) && c == Some(true)) || (a.child_by_field_name("alternative").is_some_and(|alt| alt.id() == child.id()) && c == Some(false)) {
                    return true;
                }
            }
            "statement_block" => {
                // An earlier `if (not fulfilled) return;` in the same block.
                for s in named_children(a) {
                    if s.start_byte() >= child.start_byte() {
                        break;
                    }
                    if s.kind() == "if_statement" && s.child_by_field_name("alternative").is_none() && s.child_by_field_name("condition").and_then(|c| status_check(c, status, src)) == Some(false) && s.child_by_field_name("consequence").is_some_and(|c| always_exits(c, src)) {
                        return true;
                    }
                }
            }
            _ => {}
        }
        child = a;
    }
    false
}

srule!(NoUnsafePromiseAllSettledValues, "js/no-unsafe-promise-all-settled-values", &["call_expression"], |n, ctx, m, out| {
    let src = ctx.source;
    let Some(f) = n.child_by_field_name("function").filter(|f| f.kind() == "member_expression" && f.child_by_field_name("optional_chain").is_none()) else { return };
    if !f.child_by_field_name("property").is_some_and(|p| text(p, src) == "map") || !(1..=2).contains(&arguments(n).len()) {
        return;
    }
    let Some(recv) = f.child_by_field_name("object") else { return };
    let root = js_ast::root_of(n);
    if !settled(recv, root, m, src, 0) {
        return;
    }
    // A `.filter(r => r.status === 'fulfilled')` in the chain already guards it.
    let mut x = unparen(recv);
    while x.kind() == "call_expression" {
        let Some(g) = x.child_by_field_name("function") else { break };
        if g.child_by_field_name("property").is_some_and(|p| text(p, src) == "filter")
            && let Some(cb) = arguments(x).first().and_then(|a| js_ast::as_function(*a))
            && let Some(p) = js_ast::params(cb).first().and_then(|p| js_ast::param_name(*p, src))
        {
            let body = js_ast::body(cb);
            let ret = body.and_then(|b| if b.kind() == "statement_block" { named_children(b).into_iter().find(|s| s.kind() == "return_statement").and_then(|r| r.named_child(0)) } else { Some(b) });
            if ret.and_then(|r| status_check(r, &[format!("{p}.status")], src)) == Some(true) {
                return;
            }
        }
        let Some(o) = g.child_by_field_name("object") else { break };
        x = unparen(o);
    }
    let Some(cb) = arguments(n).first().and_then(|a| js_ast::as_function(*a)) else { return };
    let Some(p) = js_ast::params(cb).first().copied() else { return };
    let pat = if p.kind() == "required_parameter" { p.child_by_field_name("pattern").unwrap_or(p) } else { p };
    let mut reads = Vec::new();
    let mut status = Vec::new();
    let mut shorthand = None;
    match pat.kind() {
        "identifier" => {
            let pn = text(pat, src);
            status.push(format!("{pn}.status"));
            let Some(psym) = m.declared_at(pat.start_byte()) else { return };
            for r in refs_of(m, psym) {
                if let Some(id) = node_at(root, r.start, r.end)
                    && let Some(mem) = id.parent().filter(|q| q.kind() == "member_expression" && same(q.child_by_field_name("object"), id) && q.child_by_field_name("property").is_some_and(|x| text(x, src) == "value"))
                {
                    reads.push(mem);
                }
            }
        }
        "object_pattern" => {
            for c in named_children(pat) {
                let key = match c.kind() {
                    "shorthand_property_identifier_pattern" => Some(c),
                    "pair_pattern" => c.child_by_field_name("key"),
                    _ => None,
                };
                if let Some(k) = key {
                    match text(k, src) {
                        "status" => status.push(text(c.child_by_field_name("value").unwrap_or(c), src).to_owned()),
                        "value" if c.kind() == "shorthand_property_identifier_pattern" => shorthand = Some(c),
                        "value" => reads.push(k),
                        _ => {}
                    }
                }
            }
        }
        _ => return,
    }
    // `({ value }) => …`: reported at the pattern when a use is unguarded.
    if let Some(c) = shorthand
        && let Some(vs) = m.declared_at(c.start_byte())
        && refs_of(m, vs).iter().filter_map(|r| node_at(root, r.start, r.end)).any(|id| !guarded(id, cb, &status, src))
    {
        reads.push(c);
    }
    for r in reads {
        if r.kind() == "shorthand_property_identifier_pattern" || !guarded(r, cb, &status, src) {
            out.push(finding(r, "This reads `value` of a `Promise.allSettled()` result without checking `status`: a rejected result has `reason`, not `value`", vec![]));
        }
    }
});


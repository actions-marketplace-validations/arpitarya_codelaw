//! W-139 strand W-139-js: eslint-plugin-unicorn v79.0.0's
//! `prefer-temporal-conversion`, read from the rule's source and tests and
//! written again. A `Temporal` source is known by the constructor or
//! factory that made it, an annotation `Temporal.X`, or a `const` alias.

use super::values::{Probe, arg_count, bare, const_init, has_comment, is_optional, member_object_text, method};
use super::{JS, finding};
use crate::js_ast::{self, arguments};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule, as_suggestion};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const DATE: &[&str] = &["year", "month", "day"];
const TIME: &[&str] = &["hour", "minute", "second", "millisecond", "microsecond", "nanosecond"];

/// `Temporal.<name>`: the name.
fn temporal_member<'s>(n: Node<'_>, src: &'s str) -> Option<&'s str> {
    let n = bare(n);
    if n.kind() != "member_expression" || is_optional(n) {
        return None;
    }
    let o = n.child_by_field_name("object")?;
    let pr = n.child_by_field_name("property")?;
    (o.kind() == "identifier" && text(o, src) == "Temporal" && pr.kind() == "property_identifier").then(|| text(pr, src))
}

fn annotation_names(t: Node<'_>, src: &str) -> Option<String> {
    match t.kind() {
        "type_annotation" | "parenthesized_type" => annotation_names(t.named_child(0)?, src),
        "nested_type_identifier" => Some(text(t, src).split_whitespace().collect()),
        _ => None,
    }
}

fn is_source<'t>(p: &Probe<'_, 't>, n: Node<'t>, names: &[&str], depth: usize) -> bool {
    let n = bare(n);
    if depth > 6 {
        return false;
    }
    match n.kind() {
        "new_expression" => n.child_by_field_name("constructor").and_then(|c| temporal_member(c, p.src)).is_some_and(|x| names.contains(&x)),
        "call_expression" => {
            if let Some((recv, name)) = method(p.src, n, &["from"])
                && (1..=2).contains(&arg_count(n))
            {
                let _ = name;
                return temporal_member(recv, p.src).is_some_and(|x| names.contains(&x));
            }
            let want = if names.contains(&"ZonedDateTime") { "zonedDateTimeISO" } else { "" };
            let want2 = if names.contains(&"PlainDateTime") { "plainDateTimeISO" } else { "" };
            if let Some((recv, name)) = method(p.src, n, &[want, want2]).filter(|_| !want.is_empty() || !want2.is_empty())
                && arg_count(n) <= 1
                && temporal_member(recv, p.src) == Some("Now")
            {
                return (name == "zonedDateTimeISO" && names.contains(&"ZonedDateTime")) || (name == "plainDateTimeISO" && names.contains(&"PlainDateTime"));
            }
            false
        }
        "identifier" => {
            let Some(sym) = p.m.symbol_at(n.start_byte()) else { return false };
            let s = p.m.symbol(sym);
            if s.decls.len() != 1 {
                return false;
            }
            let Some(id) = p.root.descendant_for_byte_range(s.decls[0].0, s.decls[0].1) else { return false };
            if let Some(d) = id.parent().filter(|d| d.kind() == "variable_declarator")
                && let Some(t) = d.child_by_field_name("type")
                && let Some(full) = annotation_names(t, p.src)
            {
                return names.iter().any(|x| full == format!("Temporal.{x}"));
            }
            if let Some(par) = id.parent().filter(|d| matches!(d.kind(), "required_parameter" | "optional_parameter"))
                && let Some(t) = par.child_by_field_name("type")
            {
                return annotation_names(t, p.src).is_some_and(|full| names.iter().any(|x| full == format!("Temporal.{x}")));
            }
            p.m.symbol(sym).kind == codelaw_semantic::SymbolKind::Const && const_init(id, p.src).is_some_and(|i| is_source(p, i, names, depth + 1))
        }
        "as_expression" | "satisfies_expression" => n.named_child(1).and_then(|t| annotation_names(t, p.src)).is_some_and(|full| names.iter().any(|x| full == format!("Temporal.{x}"))),
        _ => false,
    }
}

/// `value` is `<obj>.<name>` (no optional chain, no computed access).
fn property_of<'t>(n: Node<'t>, name: &str, src: &str) -> Option<Node<'t>> {
    let n = bare(n);
    if n.kind() != "member_expression" || is_optional(n) {
        return None;
    }
    let pr = n.child_by_field_name("property")?;
    (pr.kind() == "property_identifier" && text(pr, src) == name).then(|| n.child_by_field_name("object")).flatten()
}

fn same_reference(a: Node<'_>, b: Node<'_>, src: &str) -> bool {
    let norm = |n: Node<'_>| -> String { text(bare(n), src).split_whitespace().collect() };
    norm(a) == norm(b)
}

fn pure_receiver(n: Node<'_>) -> bool {
    !super::values::has_side_effect(n) && !matches!(bare(n).kind(), "call_expression" | "new_expression")
}

/// The source of a call that rebuilds a `Temporal` value from every field of
/// another: whether the fix is exact.
fn field_reconstruction<'t>(n: Node<'t>, fields: &[&str], src: &str) -> Option<(Node<'t>, bool)> {
    let is_new = n.kind() == "new_expression";
    let entries: Vec<(String, Node<'t>)> = if is_new {
        arguments(n).into_iter().enumerate().map(|(i, v)| (fields.get(i).copied().unwrap_or("calendar").to_owned(), v)).collect()
    } else {
        let a = bare(arguments(n).first().copied()?);
        let mut out = Vec::new();
        for pr in named_children(a).into_iter().filter(|c| c.kind() != "comment") {
            if pr.kind() != "pair" {
                return None;
            }
            let k = pr.child_by_field_name("key")?;
            let name = match k.kind() {
                "property_identifier" => text(k, src).to_owned(),
                "string" => crate::util::string_value(k, src),
                _ => return None,
            };
            out.push((name, pr.child_by_field_name("value")?));
        }
        out
    };
    let has_date = fields.contains(&"year");
    let has_calendar = entries.iter().any(|(k, _)| k == "calendar");
    let mut remaining: Vec<&str> = fields.to_vec();
    if has_date && has_calendar {
        remaining.push("calendar");
    }
    let mut source: Option<Node<'t>> = None;
    for (name, value) in &entries {
        let key = if name == "monthCode" { "month" } else { name.as_str() };
        let pos = remaining.iter().position(|r| *r == key)?;
        remaining.remove(pos);
        let obj = property_of(*value, if name == "calendar" { "calendarId" } else { name }, src)?;
        if !pure_receiver(obj) || source.is_some_and(|s| !same_reference(s, obj, src)) {
            return None;
        }
        source.get_or_insert(obj);
    }
    if !remaining.is_empty() {
        return None;
    }
    Some((source?, !has_date || (!is_new && has_calendar)))
}

pub struct PreferTemporalConversion;
impl NativeRule for PreferTemporalConversion {
    fn id(&self) -> &'static str {
        "js/prefer-temporal-conversion"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression", "new_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let p = Probe::new(model, ctx.source, js_ast::root_of(n));
        let src = ctx.source;
        let args = arguments(n);
        let (ctor, call_method) = if n.kind() == "new_expression" {
            let Some(c) = n.child_by_field_name("constructor") else { return };
            (c, None)
        } else {
            let Some((recv, m)) = method(src, n, &["from", "fromEpochMilliseconds", "fromEpochNanoseconds"]) else { return };
            if args.len() != 1 {
                return;
            }
            (recv, Some(m))
        };
        let Some(target) = temporal_member(ctor, src) else { return };
        let (sources, fields): (&[&str], Vec<&str>) = match target {
            "Instant" => (&["ZonedDateTime"], vec![]),
            "PlainDateTime" => (&["ZonedDateTime"], DATE.iter().chain(TIME).copied().collect()),
            "PlainDate" => (&["ZonedDateTime", "PlainDateTime"], DATE.to_vec()),
            "PlainTime" => (&["ZonedDateTime", "PlainDateTime"], TIME.to_vec()),
            _ => return,
        };
        if args.is_empty() || args.iter().any(|a| a.kind() == "spread_element") || (target == "Instant" && args.len() != 1) || (target != "Instant" && call_method.is_some_and(|m| m != "from")) {
            return;
        }
        let is_from = call_method == Some("from");
        let first = bare(args[0]);
        let found: Option<(Node<'_>, bool)> = if is_from && let Some((recv, _)) = method(src, first, &["toString", "toJSON"]).filter(|_| arg_count(first) == 0) {
            Some((recv, target != "Instant"))
        } else if is_from && first.kind() != "object" {
            Some((args[0], true))
        } else if target != "Instant" {
            field_reconstruction(n, &fields, src)
        } else {
            let ms = call_method == Some("fromEpochMilliseconds");
            property_of(args[0], if ms { "epochMilliseconds" } else { "epochNanoseconds" }, src).map(|o| (o, !ms))
        };
        let Some((source, exact)) = found else { return };
        if !is_source(&p, source, sources, 0) {
            return;
        }
        let meth = format!("to{target}");
        let mut edits = Vec::new();
        if !has_comment(n) {
            let mut rep = format!("{}.{meth}()", member_object_text(source, src));
            let before = src[..n.start_byte()].chars().next_back();
            if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
                rep.insert(0, ' ');
            }
            let at_statement = n.parent().is_some_and(|x| x.kind() == "expression_statement");
            if !(at_statement && rep.starts_with(['(', '[', '`'])) {
                edits.push(TextEdit::new(n.start_byte(), n.end_byte(), rep));
            }
        }
        let edits = if exact { edits } else { as_suggestion(edits) };
        out.push(finding(n, format!("Convert with `.{meth}()` instead of building `Temporal.{target}` again"), edits));
    }
}

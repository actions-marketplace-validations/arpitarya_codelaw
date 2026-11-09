//! W-139 strand W-139-ruff: the format-string rewrites of pyupgrade and flynt
//! — UP030 (explicit positional indices in a `.format` string), UP031
//! (`%` formatting), UP032 (`.format` to an f-string) and FLY002 (a static
//! `join` to an f-string) — on two small parsers, one for `str.format`
//! strings and one for `%` strings. Behaviour is Ruff 0.16.9's (MIT; read
//! from its source and tests over the network, research only, L5); each rule
//! is written fresh.

use crate::python_util::{call_args, kids, qualified_name, string_parts, PY};
use crate::util::{children, named_children, text};
use crate::{as_suggestion, NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use std::collections::BTreeMap;
use tree_sitter::Node;

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

fn span(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

// ------------------------------------------------------- `str.format` strings

#[derive(Debug, Clone, PartialEq, Eq)]
enum Ref {
    Auto,
    Index(usize),
    Keyword(String),
}

#[derive(Debug, Clone)]
enum Access {
    Attr(String),
    Index(usize),
    Key(String),
}

#[derive(Debug)]
struct Fld {
    /// The text of the field's name, the part before `!` or `:`.
    name: String,
    /// Where the name starts in the string's content.
    name_start: usize,
    reference: Ref,
    accesses: Vec<Access>,
    conversion: Option<char>,
    spec: String,
}

#[derive(Debug)]
enum Part {
    Lit(String),
    Fld(Fld),
}

fn is_ident(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(|c| c.is_alphabetic() || c == '_') && cs.all(|c| c.is_alphanumeric() || c == '_')
}

fn parse_field_name(name: &str) -> Option<(Ref, Vec<Access>)> {
    let first_end = name.find(['.', '[']).unwrap_or(name.len());
    let head = &name[..first_end];
    let reference = if head.is_empty() {
        Ref::Auto
    } else if head.chars().all(|c| c.is_ascii_digit()) {
        Ref::Index(head.parse().ok()?)
    } else if is_ident(head) {
        Ref::Keyword(head.to_owned())
    } else {
        return None;
    };
    let mut rest = &name[first_end..];
    let mut accesses = Vec::new();
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix('.') {
            let end = r.find(['.', '[']).unwrap_or(r.len());
            let attr = &r[..end];
            if !is_ident(attr) {
                return None;
            }
            accesses.push(Access::Attr(attr.to_owned()));
            rest = &r[end..];
        } else {
            let r = rest.strip_prefix('[')?;
            let end = r.find(']')?;
            let idx = &r[..end];
            if idx.is_empty() {
                return None;
            }
            if idx.chars().all(|c| c.is_ascii_digit()) {
                accesses.push(Access::Index(idx.parse().ok()?));
            } else {
                accesses.push(Access::Key(idx.to_owned()));
            }
            rest = &r[end + 1..];
        }
    }
    Some((reference, accesses))
}

/// A `str.format` string's literal and field parts. `None` when it is
/// malformed or a field's spec holds another field.
fn parse_format(content: &str) -> Option<Vec<Part>> {
    let b = content.as_bytes();
    let mut parts = Vec::new();
    let mut lit = String::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'{' if b.get(i + 1) == Some(&b'{') => {
                lit.push_str("{{");
                i += 2;
            }
            b'}' if b.get(i + 1) == Some(&b'}') => {
                lit.push_str("}}");
                i += 2;
            }
            b'}' => return None,
            b'{' => {
                if !lit.is_empty() {
                    parts.push(Part::Lit(std::mem::take(&mut lit)));
                }
                let start = i + 1;
                let close = content[start..].find(['{', '}'])? + start;
                if b[close] == b'{' {
                    return None;
                }
                let body = &content[start..close];
                let (before_spec, spec) = match body.find(':') {
                    Some(c) if !body[..c].contains('[') || body[..c].contains(']') => (&body[..c], body[c + 1..].to_owned()),
                    _ => (body, String::new()),
                };
                let (name, conversion) = match before_spec.find('!') {
                    Some(p) => {
                        let conv: Vec<char> = before_spec[p + 1..].chars().collect();
                        if conv.len() != 1 || !matches!(conv[0], 'r' | 's' | 'a') {
                            return None;
                        }
                        (&before_spec[..p], Some(conv[0]))
                    }
                    None => (before_spec, None),
                };
                let (reference, accesses) = parse_field_name(name)?;
                parts.push(Part::Fld(Fld { name: name.to_owned(), name_start: start, reference, accesses, conversion, spec }));
                i = close + 1;
            }
            _ => {
                let ch = content[i..].chars().next()?;
                lit.push(ch);
                i += ch.len_utf8();
            }
        }
    }
    if !lit.is_empty() {
        parts.push(Part::Lit(lit));
    }
    Some(parts)
}

/// A plain string literal node: its prefix (lower case), the opening and
/// closing quotes, and where its content sits.
struct Lit {
    prefix: String,
    quote: String,
    content_start: usize,
    content_end: usize,
}

fn literal_of(n: Node<'_>, src: &str) -> Option<Lit> {
    if n.kind() != "string" {
        return None;
    }
    string_parts(n, src)?;
    let cs = children(n);
    let st = cs.iter().find(|c| c.kind() == "string_start")?;
    let en = cs.iter().rev().find(|c| c.kind() == "string_end")?;
    let opener = text(*st, src);
    let prefix: String = opener.chars().take_while(|c| c.is_ascii_alphabetic()).collect::<String>().to_ascii_lowercase();
    let quote = opener[prefix.len()..].to_owned();
    Some(Lit { prefix, quote, content_start: st.end_byte(), content_end: en.start_byte() })
}

/// The call `"…".format(…)`: the literal and the call's arguments.
fn format_call(call: Node<'_>, src: &str) -> Option<Lit> {
    let func = field(call, "function").filter(|f| f.kind() == "attribute")?;
    if text(field(func, "attribute")?, src) != "format" {
        return None;
    }
    let lit = literal_of(field(func, "object")?, src)?;
    if lit.prefix.contains(['f', 'b', 't']) {
        return None;
    }
    Some(lit)
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

fn has_effect(n: Node<'_>) -> bool {
    let mut stack = vec![n];
    while let Some(c) = stack.pop() {
        if matches!(c.kind(), "call" | "await" | "yield" | "named_expression") {
            return true;
        }
        stack.extend(named_children(c));
    }
    false
}

// ------------------------------------------------------------------ UP030

fn up030(call: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(lit) = format_call(call, src) else { return };
    let content = &src[lit.content_start..lit.content_end];
    let Some(parts) = parse_format(content) else { return };
    let mut indices: Vec<usize> = Vec::new();
    let mut fields: Vec<&Fld> = Vec::new();
    for p in &parts {
        if let Part::Fld(f) = p {
            match &f.reference {
                Ref::Index(i) => indices.push(*i),
                _ => return,
            }
            fields.push(f);
        }
    }
    if indices.is_empty() || (0..indices.len()).any(|i| !indices.contains(&i)) {
        return;
    }
    let args = call_args(call, src);
    let sequential = indices.iter().enumerate().all(|(i, v)| i == *v);
    // The edit: the digits leave the string; arguments move when the order differs.
    let mut edits: Vec<TextEdit> = Vec::new();
    for f in &fields {
        let digits = f.name.chars().take_while(char::is_ascii_digit).count();
        let s = lit.content_start + f.name_start;
        edits.push(TextEdit::new(s, s + digits, ""));
    }
    let mut suggest = false;
    if !sequential {
        if !args.keywords.is_empty() || args.positional.len() != indices.len() || args.starred {
            return;
        }
        for (i, idx) in indices.iter().enumerate() {
            let (dst, srcn) = (args.positional[i], args.positional[*idx]);
            if i != *idx {
                edits.push(TextEdit::new(dst.start_byte(), dst.end_byte(), text(srcn, src)));
                if has_effect(srcn) {
                    suggest = true;
                }
            }
        }
    }
    let edits = if suggest { as_suggestion(edits) } else { edits };
    out.push(span(call, "Use implicit references for positional format fields", edits));
}

// ------------------------------------------------------------------ UP032

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ctx {
    Bare,
    Accessed,
}

fn parenthesize(n: Node<'_>, src: &str, ctx: Ctx) -> bool {
    let t = text(n, src);
    if t.starts_with('(') && t.ends_with(')') && n.kind() == "parenthesized_expression" {
        return false;
    }
    let bad_base = |k: &str| matches!(k, "generator_expression" | "dictionary" | "set" | "set_comprehension" | "dictionary_comprehension");
    match n.kind() {
        "generator_expression" | "dictionary" | "named_expression" | "set" | "set_comprehension" | "dictionary_comprehension" | "lambda" => true,
        "binary_operator" | "unary_operator" | "boolean_operator" | "not_operator" | "comparison_operator" | "conditional_expression" | "await" | "yield" | "list_splat" => ctx == Ctx::Accessed,
        "integer" => ctx == Ctx::Accessed && t.chars().filter(|c| *c != '_').all(|c| c.is_ascii_digit()),
        "subscript" => field(n, "value").is_some_and(|v| bad_base(v.kind())),
        "attribute" => field(n, "object").is_some_and(|v| bad_base(v.kind())),
        "call" => field(n, "function").is_some_and(|v| bad_base(v.kind())),
        _ => false,
    }
}

fn shown(n: Node<'_>, src: &str, ctx: Ctx) -> String {
    let t = text(n, src);
    if parenthesize(n, src, ctx) { format!("({t})") } else { t.to_owned() }
}

fn up032(call: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(lit) = format_call(call, src) else { return };
    if !matches!(lit.prefix.as_str(), "" | "u" | "r") || lit.content_start == lit.content_end {
        return;
    }
    let Some(parts) = parse_format(&src[lit.content_start..lit.content_end]) else { return };
    if parts.iter().any(|p| matches!(p, Part::Fld(f) if f.spec.contains(['{', '}']))) {
        return;
    }
    let args = call_args(call, src);
    if args.starred || (args.positional.is_empty() && args.keywords.is_empty()) {
        return;
    }
    // Every argument is a single line without quotes, and a keyword is named.
    let bad = |n: Node<'_>| {
        let t = text(n, src);
        t.contains(['\'', '"', '\n']) || n.kind() == "list_splat"
    };
    if args.positional.iter().any(|a| bad(*a)) || args.keywords.iter().any(|(_, v, _)| bad(*v)) {
        return;
    }
    if let Some(list) = args.list
        && kids(list).iter().any(|c| c.kind() == "dictionary_splat" || c.kind() == "list_splat")
    {
        return;
    }
    let kw: BTreeMap<&str, Node<'_>> = args.keywords.iter().map(|(k, v, _)| (k.as_str(), *v)).collect();
    let mut auto = 0;
    let mut seen: Vec<(Ref, usize)> = Vec::new();
    let mut used = vec![false; args.positional.len()];
    let mut used_kw: BTreeMap<&str, bool> = kw.keys().map(|k| (*k, false)).collect();
    let mut converted = String::new();
    let mut effect_order: Vec<usize> = Vec::new();
    let trailing = lit.quote.clone();
    for part in &parts {
        match part {
            Part::Lit(l) => converted.push_str(l),
            Part::Fld(f) => {
                let reference = match &f.reference {
                    Ref::Auto => {
                        auto += 1;
                        Ref::Index(auto - 1)
                    }
                    other => other.clone(),
                };
                let (arg, written) = match &reference {
                    Ref::Index(i) => match args.positional.get(*i) {
                        Some(a) => {
                            used[*i] = true;
                            (*a, *i)
                        }
                        None => return,
                    },
                    Ref::Keyword(k) => match kw.get(k.as_str()) {
                        Some(a) => {
                            *used_kw.entry(kw.keys().find(|x| **x == k.as_str()).copied().unwrap_or("")).or_default() = true;
                            (*a, args.positional.len() + args.keywords.iter().position(|(n, _, _)| n == k).unwrap_or(0))
                        }
                        None => return,
                    },
                    Ref::Auto => return,
                };
                let repeated = seen.iter().any(|(r, _)| *r == reference);
                if repeated && has_effect(arg) {
                    return;
                }
                if !repeated {
                    seen.push((reference, written));
                    if has_effect(arg) {
                        effect_order.push(written);
                    }
                }
                let ctx_kind = if f.accesses.is_empty() { Ctx::Bare } else { Ctx::Accessed };
                converted.push('{');
                converted.push_str(&shown(arg, src, ctx_kind));
                for a in &f.accesses {
                    match a {
                        Access::Attr(n) => {
                            converted.push('.');
                            converted.push_str(n);
                        }
                        Access::Index(i) => converted.push_str(&format!("[{i}]")),
                        Access::Key(k) => {
                            let q = if trailing == "\"" { '\'' } else { '"' };
                            converted.push_str(&format!("[{q}{k}{q}]"));
                        }
                    }
                }
                if let Some(c) = f.conversion {
                    converted.push('!');
                    converted.push(c);
                }
                if !f.spec.is_empty() {
                    converted.push(':');
                    converted.push_str(&f.spec);
                }
                converted.push('}');
            }
        }
    }
    if used.iter().any(|u| !u) || used_kw.values().any(|u| !u) {
        return;
    }
    let raw = if lit.prefix == "r" { "r" } else { "" };
    let mut result = format!("{raw}f{}{converted}{}", lit.quote, lit.quote);
    // `return"x"` is valid, `returnf"x"` is not.
    if src[..call.start_byte()].chars().next_back().is_some_and(|c| c.is_ascii_alphabetic()) {
        result.insert(0, ' ');
    }
    // `gettext` takes format-style arguments, not f-strings.
    if let Some(model) = ctx.semantic {
        let mut p = call.parent();
        while let Some(x) = p {
            if x.kind() == "call"
                && field(x, "function").and_then(|f| qualified_name(f, model, src)).is_some_and(|q| q == "django.utils.translation.gettext" || q == "django.utils.translation.gettext_lazy")
            {
                return;
            }
            p = x.parent();
        }
    }
    let edits = if has_comment(call) {
        Vec::new()
    } else {
        let e = vec![TextEdit::new(call.start_byte(), call.end_byte(), result)];
        if effect_order.windows(2).all(|w| w[0] <= w[1]) { e } else { as_suggestion(e) }
    };
    out.push(span(call, "Use f-string instead of `format` call", edits));
}

// ------------------------------------------------------------------ UP031

#[derive(Debug, Default, Clone)]
struct CSpec {
    key: Option<String>,
    left: bool,
    sign: bool,
    alt: bool,
    blank: bool,
    zero: bool,
    width: Option<usize>,
    width_star: bool,
    precision: Option<usize>,
    precision_dot: bool,
    precision_star: bool,
    conv: char,
}

enum CPart {
    Lit(String),
    Spec(CSpec),
}

/// A printf-style string's literal and conversion parts; `None` when it is
/// malformed.
fn parse_printf(content: &str) -> Option<Vec<CPart>> {
    let cs: Vec<char> = content.chars().collect();
    let mut parts = Vec::new();
    let mut lit = String::new();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] != '%' {
            lit.push(cs[i]);
            i += 1;
            continue;
        }
        i += 1;
        if !lit.is_empty() {
            parts.push(CPart::Lit(std::mem::take(&mut lit)));
        }
        let mut s = CSpec::default();
        if cs.get(i) == Some(&'(') {
            let mut depth = 1;
            let mut j = i + 1;
            while j < cs.len() && depth > 0 {
                match cs[j] {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            if depth != 0 {
                return None;
            }
            s.key = Some(cs[i + 1..j - 1].iter().collect());
            i = j;
        }
        while let Some(c) = cs.get(i) {
            match c {
                '-' => s.left = true,
                '+' => s.sign = true,
                '#' => s.alt = true,
                ' ' => s.blank = true,
                '0' => s.zero = true,
                _ => break,
            }
            i += 1;
        }
        if cs.get(i) == Some(&'*') {
            s.width_star = true;
            i += 1;
        } else {
            let st = i;
            while cs.get(i).is_some_and(char::is_ascii_digit) {
                i += 1;
            }
            if i > st {
                s.width = cs[st..i].iter().collect::<String>().parse().ok();
            }
        }
        if cs.get(i) == Some(&'.') {
            s.precision_dot = true;
            i += 1;
            if cs.get(i) == Some(&'*') {
                s.precision_star = true;
                i += 1;
            } else {
                let st = i;
                while cs.get(i).is_some_and(char::is_ascii_digit) {
                    i += 1;
                }
                if i > st {
                    s.precision = cs[st..i].iter().collect::<String>().parse().ok();
                }
            }
        }
        while cs.get(i).is_some_and(|c| matches!(c, 'h' | 'l' | 'L')) {
            i += 1;
        }
        let c = *cs.get(i)?;
        if !"diouxXeEfFgGcrsa%".contains(c) {
            return None;
        }
        s.conv = c;
        i += 1;
        parts.push(CPart::Spec(s));
    }
    if !lit.is_empty() {
        parts.push(CPart::Lit(lit));
    }
    Some(parts)
}

fn has_flags(s: &CSpec) -> bool {
    s.left || s.sign || s.alt || s.blank || s.zero
}

fn convertible(parts: &[CPart], params_is_dict: bool) -> bool {
    for p in parts {
        let CPart::Spec(s) = p else { continue };
        if s.width_star || s.precision_star || matches!(s.conv, 'd' | 'i' | 'u' | 'c') || s.key.as_deref() == Some("") {
            return false;
        }
        let nontrivial = has_flags(s) || s.width.is_some() || s.precision.is_some() || s.precision_dot;
        if nontrivial && matches!(s.conv, '%' | 'a' | 'r') {
            return false;
        }
        if s.width.is_some() && s.conv == 's' {
            return false;
        }
        if params_is_dict && s.key.is_none() {
            return false;
        }
    }
    true
}

fn to_format_spec(s: &CSpec) -> String {
    if s.conv == '%' {
        return "%".to_owned();
    }
    let mut o = String::from("{");
    if let Some(k) = &s.key {
        o.push_str(k);
    }
    let plain = matches!(s.conv, 's' | 'r' | 'a');
    let nontrivial = has_flags(s) || s.width.is_some() || s.precision.is_some() || s.precision_dot;
    if nontrivial || !plain {
        o.push(':');
        if s.left {
            o.push('<');
        }
        if s.sign {
            o.push('+');
        }
        if s.alt {
            o.push('#');
        }
        if s.blank && !s.sign {
            o.push(' ');
        }
        if s.zero && !s.left {
            o.push('0');
        }
        if let Some(w) = s.width {
            o.push_str(&w.to_string());
        }
        if let Some(p) = s.precision {
            if matches!(s.conv, 'b' | 'c' | 'd' | 'o' | 'x' | 'X' | 'n') {
                o.push('0');
            } else {
                o.push('.');
            }
            o.push_str(&p.to_string());
        } else if s.precision_dot {
            o.push_str(".0");
        }
    }
    if !plain {
        o.push(s.conv);
    }
    if matches!(s.conv, 'r' | 'a') {
        o.push('!');
        o.push(s.conv);
    }
    o.push('}');
    o
}

fn curly_escape(s: &str) -> String {
    s.replace('{', "{{").replace('}', "}}")
}

fn up031(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    if field(n, "operator").is_none_or(|o| text(o, src) != "%") {
        return;
    }
    let (Some(left), Some(right)) = (field(n, "left"), field(n, "right")) else { return };
    let Some(lit) = literal_of(left, src) else { return };
    if lit.prefix.contains(['f', 'b', 't']) {
        return;
    }
    let Some(parts) = parse_printf(&src[lit.content_start..lit.content_end]) else { return };
    let params_is_dict = right.kind() == "dictionary";
    if !convertible(&parts, params_is_dict) {
        out.push(span(n, "Use format specifiers instead of percent format", Vec::new()));
        return;
    }
    let (mut positional, mut keyword) = (0, 0);
    for p in &parts {
        if let CPart::Spec(s) = p {
            if s.conv == '%' {
                continue;
            }
            if s.key.is_none() {
                positional += 1;
            } else {
                keyword += 1;
            }
        }
    }
    let rt = text(right, src);
    let params = match right.kind() {
        "string" | "concatenated_string" | "integer" | "float" | "true" | "false" | "none" | "ellipsis" => format!("({rt})"),
        "identifier" | "attribute" | "subscript" | "call" => {
            if keyword > 0 {
                format!("(**{rt})")
            } else if positional > 1 {
                format!("(*{rt})")
            } else {
                format!("({rt})")
            }
        }
        "tuple" => {
            let items = kids(right);
            if items.len() == 1 && !rt.contains('\n') {
                match rt.rfind(',') {
                    Some(c) => format!("{}{}", &rt[..c], &rt[c + 1..]),
                    None => rt.to_owned(),
                }
            } else {
                rt.to_owned()
            }
        }
        "dictionary" => {
            let mut args: Vec<String> = Vec::new();
            let mut seen: Vec<String> = Vec::new();
            for item in kids(right) {
                match item.kind() {
                    "pair" => {
                        let (Some(k), Some(v)) = (field(item, "key"), field(item, "value")) else { return };
                        let Some(kl) = literal_of(k, src) else {
                            out.push(span(n, "Use format specifiers instead of percent format", Vec::new()));
                            return;
                        };
                        let key = &src[kl.content_start..kl.content_end];
                        if !is_ident(key) || seen.iter().any(|s| s == key) || kl.prefix.contains(['b', 'f']) {
                            out.push(span(n, "Use format specifiers instead of percent format", Vec::new()));
                            return;
                        }
                        seen.push(key.to_owned());
                        args.push(format!("{key}={}", text(v, src)));
                    }
                    "dictionary_splat" => args.push(text(item, src).to_owned()),
                    _ => return,
                }
            }
            if args.is_empty() || rt.contains('\n') {
                out.push(span(n, "Use format specifiers instead of percent format", Vec::new()));
                return;
            }
            format!("({})", args.join(", "))
        }
        _ => return,
    };
    let converted: String = parts
        .iter()
        .map(|p| match p {
            CPart::Lit(l) => curly_escape(l),
            CPart::Spec(s) => to_format_spec(s),
        })
        .collect();
    let prefix = lit.prefix.replace('u', "");
    let new = format!("{prefix}{q}{converted}{q}.format{params}", q = lit.quote);
    let edits = if has_comment(n) { Vec::new() } else { as_suggestion(vec![TextEdit::new(n.start_byte(), n.end_byte(), new)]) };
    out.push(span(n, "Use format specifiers instead of percent format", edits));
}

// ------------------------------------------------------------------ FLY002

fn fly002(call: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(func) = field(call, "function").filter(|f| f.kind() == "attribute") else { return };
    if text(field(func, "attribute").unwrap_or(func), src) != "join" {
        return;
    }
    let Some(joiner) = field(func, "object").and_then(|o| literal_of(o, src)) else { return };
    if !joiner.prefix.is_empty() || joiner.quote.len() != 1 {
        return;
    }
    let args = call_args(call, src);
    if args.positional.len() != 1 || !args.keywords.is_empty() || args.starred {
        return;
    }
    let seq = args.positional[0];
    if !matches!(seq.kind(), "list" | "tuple") {
        return;
    }
    let elems = kids(seq);
    if elems.iter().any(|e| e.kind() == "list_splat") {
        return;
    }
    let sep = &src[joiner.content_start..joiner.content_end];
    if sep.contains(['\\', '"', '{', '}', '\n']) {
        return;
    }
    let mut all_lit = !elems.is_empty();
    let mut body = String::new();
    let mut plain = String::new();
    for (i, e) in elems.iter().enumerate() {
        if i > 0 {
            body.push_str(sep);
            plain.push_str(sep);
        }
        match literal_of(*e, src) {
            Some(l) if l.prefix.is_empty() && l.quote.len() == 1 => {
                let c = &src[l.content_start..l.content_end];
                if c.contains(['\\', '"', '{', '}', '\n']) {
                    return;
                }
                body.push_str(c);
                plain.push_str(c);
            }
            _ => {
                all_lit = false;
                let t = text(*e, src);
                if matches!(e.kind(), "string" | "concatenated_string") || t.contains(['\'', '"', '\\', '\n']) {
                    return;
                }
                body.push('{');
                body.push_str(&shown(*e, src, Ctx::Bare));
                body.push('}');
            }
        }
    }
    let new = if all_lit { format!("\"{plain}\"") } else { format!("f\"{body}\"") };
    let mut new = new;
    if src[..call.start_byte()].chars().next_back().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
        new.insert(0, ' ');
    }
    let message = format!("Consider `{}` instead of string join", new.trim_start());
    let edits = if has_comment(call) { Vec::new() } else { vec![TextEdit::new(call.start_byte(), call.end_byte(), new)] };
    out.push(span(call, message, edits));
}

// ------------------------------------------------------------------ SIM905

/// Python's white space for `str.split()`: `str.isspace`'s characters.
fn py_space(c: char) -> bool {
    matches!(c, '\u{9}'..='\u{d}' | '\u{1c}'..='\u{20}' | '\u{85}' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}')
}

/// The value of one string literal's content, escapes decoded; `None` for an
/// escape this does not read (`\N{…}`).
fn cook(content: &str, raw: bool) -> Option<String> {
    if raw {
        return Some(content.to_owned());
    }
    let mut out = String::new();
    let mut it = content.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let n = it.next()?;
        match n {
            '\n' => {}
            '\r' => {
                if it.peek() == Some(&'\n') {
                    it.next();
                }
            }
            '\\' | '\'' | '"' => out.push(n),
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            'a' => out.push('\u{7}'),
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            'v' => out.push('\u{b}'),
            'x' | 'u' | 'U' => {
                let len = match n {
                    'x' => 2,
                    'u' => 4,
                    _ => 8,
                };
                let hex: String = (0..len).filter_map(|_| it.next_if(char::is_ascii_hexdigit)).collect();
                if hex.len() != len {
                    return None;
                }
                out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
            }
            '0'..='7' => {
                let mut v = n.to_digit(8)?;
                for _ in 0..2 {
                    if let Some(d) = it.next_if(|d| ('0'..='7').contains(d)) {
                        v = v * 8 + d.to_digit(8)?;
                    }
                }
                out.push(char::from_u32(v)?);
            }
            'N' => return None,
            other => {
                out.push('\\');
                out.push(other);
            }
        }
    }
    Some(out)
}

/// How the first part of a literal is written.
struct Style {
    raw: bool,
    u: bool,
    quote: char,
}

/// A string literal (or implicit concatenation) read as the value it has and
/// how its first part is written.
fn literal(n: Node<'_>, src: &str) -> Option<(String, Style)> {
    let parts: Vec<Node<'_>> = if n.kind() == "concatenated_string" { kids(n) } else { vec![n] };
    let mut value = String::new();
    let mut first = None;
    for p in parts {
        if p.kind() != "string" {
            return None;
        }
        let (prefix, content) = string_parts(p, src)?;
        if prefix.contains(['f', 'b', 't']) {
            return None;
        }
        value.push_str(&cook(&content, prefix.contains('r'))?);
        if first.is_none() {
            let start = children(p).into_iter().find(|c| c.kind() == "string_start").map(|c| text(c, src).to_owned())?;
            first = Some(Style { raw: prefix.contains('r'), u: prefix.contains('u'), quote: start.chars().last()? });
        }
    }
    Some((value, first?))
}

/// One list element, written in the first part's quote style: a raw string
/// keeps its quote unless the element holds it, then the other quote, and
/// triple quotes when it holds both.
fn element(elt: &str, st: &Style) -> String {
    let prefix = format!("{}{}", if st.u { "u" } else { "" }, if st.raw { "r" } else { "" });
    let quote = st.quote;
    let opposite = if quote == '"' { '\'' } else { '"' };
    if st.raw {
        let (q, n) = if !elt.contains(quote) {
            (quote, 1)
        } else if !elt.contains(opposite) {
            (opposite, 1)
        } else {
            (quote, 3)
        };
        let qs = q.to_string().repeat(n);
        return format!("{prefix}{qs}{elt}{qs}");
    }
    let mut body = String::new();
    for c in elt.chars() {
        match c {
            '\\' => body.push_str("\\\\"),
            '\n' => body.push_str("\\n"),
            '\r' => body.push_str("\\r"),
            '\t' => body.push_str("\\t"),
            c if c == quote => {
                body.push('\\');
                body.push(c);
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => body.push_str(&format!("\\x{:02x}", c as u32)),
            c => body.push(c),
        }
    }
    format!("{prefix}{quote}{body}{quote}")
}

fn split_on_space(s: &str, max: i64, rsplit: bool) -> Vec<String> {
    if max < 0 {
        return s.split(py_space).filter(|p| !p.is_empty()).map(str::to_owned).collect();
    }
    let mut rest = if rsplit { s.trim_end_matches(py_space) } else { s.trim_start_matches(py_space) };
    let mut out: Vec<String> = Vec::new();
    let mut splits = 0i64;
    while !rest.is_empty() {
        if splits >= max {
            out.push(rest.to_owned());
            break;
        }
        splits += 1;
        if rsplit {
            match rest.rsplit_once(py_space) {
                Some((head, tail)) => {
                    out.push(tail.to_owned());
                    rest = head.trim_end_matches(py_space);
                }
                None => {
                    out.push(rest.to_owned());
                    break;
                }
            }
        } else {
            match rest.split_once(py_space) {
                Some((head, tail)) => {
                    out.push(head.to_owned());
                    rest = tail.trim_start_matches(py_space);
                }
                None => {
                    out.push(rest.to_owned());
                    break;
                }
            }
        }
    }
    if rsplit {
        out.reverse();
    }
    out
}

fn split_on(s: &str, sep: &str, max: i64, rsplit: bool) -> Vec<String> {
    let n = usize::try_from(max).ok().map(|m| m.saturating_add(1));
    let mut v: Vec<String> = match (n, rsplit) {
        (Some(n), false) => s.splitn(n, sep).map(str::to_owned).collect(),
        (Some(n), true) => s.rsplitn(n, sep).map(str::to_owned).collect(),
        (None, false) => s.split(sep).map(str::to_owned).collect(),
        (None, true) => s.rsplit(sep).map(str::to_owned).collect(),
    };
    if rsplit {
        v.reverse();
    }
    v
}

/// An integer literal, `-` allowed: `maxsplit`'s value.
fn int_literal(n: Node<'_>, src: &str) -> Option<i64> {
    match n.kind() {
        "integer" => {
            let t = text(n, src).replace('_', "").to_ascii_lowercase();
            let v = if let Some(h) = t.strip_prefix("0x") {
                i64::from_str_radix(h, 16).ok()?
            } else if let Some(o) = t.strip_prefix("0o") {
                i64::from_str_radix(o, 8).ok()?
            } else if let Some(b) = t.strip_prefix("0b") {
                i64::from_str_radix(b, 2).ok()?
            } else {
                t.parse().ok()?
            };
            i32::try_from(v).ok().map(i64::from)
        }
        "unary_operator" => {
            let op = field(n, "operator").map(|o| text(o, src))?;
            if op != "-" {
                return None;
            }
            int_literal(field(n, "argument")?, src).map(|v| -v)
        }
        _ => None,
    }
}

fn sim905(call: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(func) = field(call, "function").filter(|f| f.kind() == "attribute") else { return };
    let Some(method) = field(func, "attribute").map(|a| text(a, src)) else { return };
    if method != "split" && method != "rsplit" {
        return;
    }
    let Some(mut recv) = field(func, "object") else { return };
    while recv.kind() == "parenthesized_expression" {
        let inner: Vec<Node<'_>> = named_children(recv).into_iter().filter(|c| c.kind() != "comment").collect();
        if inner.len() != 1 {
            return;
        }
        recv = inner[0];
    }
    if !matches!(recv.kind(), "string" | "concatenated_string") {
        return;
    }
    let Some((value, style)) = literal(recv, src) else { return };
    let args = call_args(call, src);
    if args.starred || args.positional.len() > 2 || args.keywords.iter().any(|k| k.0 != "sep" && k.0 != "maxsplit") {
        return;
    }
    let (pos_sep, kw_sep) = (args.positional.first().copied(), args.keyword("sep"));
    let (pos_max, kw_max) = (args.positional.get(1).copied(), args.keyword("maxsplit"));
    if (pos_sep.is_some() && kw_sep.is_some()) || (pos_max.is_some() && kw_max.is_some()) {
        return;
    }
    let sep_node = pos_sep.or(kw_sep);
    let max = match pos_max.or(kw_max) {
        None => -1,
        Some(m) => match int_literal(m, src) {
            Some(v) => v,
            None => return,
        },
    };
    let rsplit = method == "rsplit";
    let items = match sep_node {
        None => split_on_space(&value, max, rsplit),
        Some(sn) if sn.kind() == "none" => split_on_space(&value, max, rsplit),
        Some(sn) if matches!(sn.kind(), "string" | "concatenated_string") => {
            let Some((sep, _)) = literal(sn, src) else { return };
            if sep.is_empty() {
                return;
            }
            split_on(&value, &sep, max, rsplit)
        }
        Some(_) => return,
    };
    let list = format!("[{}]", items.iter().map(|i| element(i, &style)).collect::<Vec<_>>().join(", "));
    let mut has_comment = false;
    let mut stack = vec![call];
    while let Some(n) = stack.pop() {
        if n.kind() == "comment" {
            has_comment = true;
        }
        stack.extend(children(n));
    }
    let edit = vec![TextEdit::new(call.start_byte(), call.end_byte(), list)];
    out.push(span(call, format!("Consider using a list literal instead of `str.{method}`"), if has_comment { as_suggestion(edit) } else { edit }));
}

macro_rules! fmt_rule {
    ($name:ident, $id:literal, $kind:literal, $sem:literal, $f:path) => {
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
                $sem
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                $f(node.get_inner_node(), ctx, out)
            }
        }
    };
}

fmt_rule!(FormatLiterals, "py/format-literals", "call", false, up030);
fmt_rule!(PrintfStringFormatting, "py/printf-string-formatting", "binary_operator", false, up031);
fmt_rule!(FString, "py/f-string", "call", true, up032);
fmt_rule!(StaticJoinToFString, "py/static-join-to-f-string", "call", false, fly002);
fmt_rule!(SplitStaticString, "py/split-static-string", "call", false, sim905);

pub static RULES: &[&dyn NativeRule] = &[&FormatLiterals, &PrintfStringFormatting, &FString, &StaticJoinToFString, &SplitStaticString];

#[cfg(test)]
mod tests {
    use super::*;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        crate::run(rule, Language::Python, src)
    }

    #[test]
    fn format_strings() {
        let r = run(&FString, "x = \"{} and {}\".format(a, b)\n");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].edits[0].text, "f\"{a} and {b}\"");
        assert!(run(&FString, "x = \"{} {}\".format(a)\n").is_empty());
        assert!(run(&FString, "x = \"{}\".format(a['k'])\n").is_empty());
        let r = run(&FormatLiterals, "x = \"{0} {1}\".format(a, b)\n");
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn printf() {
        let r = run(&PrintfStringFormatting, "x = \"%s and %s\" % (a, b)\n");
        assert_eq!(r.len(), 1);
        assert!(r[0].edits.iter().any(|e| e.text == "\"{} and {}\".format(a, b)"));
        let r = run(&PrintfStringFormatting, "x = \"%d\" % a\n");
        assert_eq!(r.len(), 1);
        assert!(r[0].edits.is_empty());
    }

    #[test]
    fn join() {
        let r = run(&StaticJoinToFString, "x = \"-\".join([a, b])\n");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].edits[0].text, "f\"{a}-{b}\"");
    }

    /// The text `py/split-static-string` rewrites one expression to, if it does.
    fn split(src: &str) -> Option<String> {
        let code = format!("{src}\n");
        let r = run(&SplitStaticString, &code);
        assert!(r.len() <= 1, "{src}");
        let f = r.first()?;
        let e = f.edits.iter().find(|e| e.start != e.end || !e.text.is_empty())?;
        Some(e.text.clone())
    }

    #[test]
    fn split_static_string_rows() {
        let rows: &[(&str, Option<&str>)] = &[
            ("\"a,b,c,d\".split(\",\")", Some("[\"a\", \"b\", \"c\", \"d\"]")),
            ("\"a,b,c,d\".split(None)", Some("[\"a,b,c,d\"]")),
            ("\"a,b,c,d\".split(\",\", 1)", Some("[\"a\", \"b,c,d\"]")),
            ("\"a,b,c,d\".split(sep=None, maxsplit=1)", Some("[\"a,b,c,d\"]")),
            ("\"a,b,c,d\".split(maxsplit=1, sep=\",\")", Some("[\"a\", \"b,c,d\"]")),
            ("\"a,b,c,d\".split(maxsplit=1.0)", None),
            ("\"VERB AUX PRON ADP DET\".split(\" \")", Some("[\"VERB\", \"AUX\", \"PRON\", \"ADP\", \"DET\"]")),
            ("'   1   2   3   '.split()", Some("['1', '2', '3']")),
            ("'1<>2<>3<4'.split('<>')", Some("['1', '2', '3<4']")),
            ("\" a*a a*a a \".split(\"*\", -1)", Some("[\" a\", \"a a\", \"a a \"]")),
            ("\"\".split()", Some("[]")),
            ("\"/abc/\".split()", Some("[\"/abc/\"]")),
            ("u\"a b\".split()", Some("[u\"a\", u\"b\"]")),
            ("r\"a \\n b\".split()", Some("[r\"a\", r\"\\n\", r\"b\"]")),
            ("(\"a \" \"b\").split()", Some("[\"a\", \"b\"]")),
            ("\"a \" r\"\\n\".split()", Some("[\"a\", \"\\\\n\"]")),
            ("\"a,b,c\".split(',', maxsplit=0)", Some("[\"a,b,c\"]")),
            ("\"a,b,c\".split(',', maxsplit=-0)", Some("[\"a,b,c\"]")),
            ("\"a,b,c\".split(',', maxsplit=-2)", Some("[\"a\", \"b\", \"c\"]")),
            ("\"a,b,c,d\".split(maxsplit=\"hello\")", None),
            ("\"a,b,c,d\".split(maxsplit=-\"hello\")", None),
            ("\"a,b,c,d\".split(sep)", None),
            ("f\"{x}_a\".split(\"_\")", None),
            ("b\"a.b\".split(b\".\")", None),
            ("\"a,b\".rsplit(\",\")", Some("[\"a\", \"b\"]")),
            ("\"a,b,c\".rsplit(\",\", 1)", Some("[\"a,b\", \"c\"]")),
            ("\" x \".split(maxsplit=0)", Some("[\"x \"]")),
            ("\" x \".rsplit(maxsplit=0)", Some("[\" x\"]")),
            ("\" a b c d \".split(maxsplit=2)", Some("[\"a\", \"b\", \"c d \"]")),
            ("\" a b c d \".rsplit(maxsplit=2)", Some("[\" a b\", \"c\", \"d\"]")),
            ("\"a  b\".split(maxsplit=1)", Some("[\"a\", \"b\"]")),
            ("\"S\\x1cP\\x1dL\\x1eI\\x1fT\".split()", Some("[\"S\", \"P\", \"L\", \"I\", \"T\"]")),
            ("\"a,b\".split(\"\")", None),
            ("\"hello\\nworld\".splitlines()", None),
        ];
        for (src, want) in rows {
            assert_eq!(split(src).as_deref(), *want, "{src}");
        }
    }

    #[test]
    fn split_static_string_quotes_and_comments() {
        // Raw strings keep their quote unless the element holds it.
        let src = "r\"\"\"first\n'no need' to escape\n\"swap\" quote style\n\"use' ugly triple quotes\"\"\".split(\"\\n\")";
        assert_eq!(split(src).as_deref(), Some("[r\"first\", r\"'no need' to escape\", r'\"swap\" quote style', r\"\"\"\"use' ugly triple quotes\"\"\"]"));
        // A comment inside the call makes the rewrite a suggestion.
        let r = run(&SplitStaticString, "(\"a,b,c\"\n# comment\n.split(\",\")\n)\n");
        assert_eq!(r.len(), 1);
        assert!(r[0].edits.iter().any(|e| e.start == usize::MAX - 1));
    }
}

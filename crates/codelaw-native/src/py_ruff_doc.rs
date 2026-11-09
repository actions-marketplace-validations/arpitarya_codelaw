//! W-139 strand W-139-ruff: the docstring rows of Ruff's pydocstyle (D207,
//! D208, D209, D214, D215, D402, D405–D417, D420) and pydoclint (DOC102,
//! DOC201, DOC202, DOC402, DOC403, DOC501, DOC502) sets, on one small
//! docstring parser: the docstring of a module, class or function, its lines,
//! and the NumPy- or Google-style sections inside it. Behaviour is Ruff
//! 0.16.9's (MIT; read from its source and tests over the network, research
//! only, L5) and every rule here is written fresh.
//!
//! No convention is configured: a docstring is read as NumPy when it holds a
//! `Parameters` section, as Google when it holds an `Args` section, and
//! otherwise as whichever style finds more sections — Ruff's tie-break.

use crate::python_util::{decorators, kids, qualified_name, statements, string_parts, PY};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::SymbolKind;
use std::collections::{BTreeSet, HashMap};
use tree_sitter::Node;

// ------------------------------------------------------------------ lines

#[derive(Clone, Copy, Debug)]
struct Ln {
    start: usize,
    /// End of the text, without the line terminator.
    end: usize,
    /// End including the terminator.
    full: usize,
}

/// The lines of `src[from..to]`. With `trailing`, a text that ends with a
/// newline also yields one empty line at its end — the line the closing
/// quotes of a docstring sit on.
fn split_lines(src: &str, from: usize, to: usize, trailing: bool) -> Vec<Ln> {
    let mut out = Vec::new();
    let mut pos = from;
    while pos < to {
        match src[pos..to].find('\n') {
            Some(i) => {
                let nl = pos + i;
                let end = if nl > pos && src.as_bytes()[nl - 1] == b'\r' { nl - 1 } else { nl };
                out.push(Ln { start: pos, end, full: nl + 1 });
                pos = nl + 1;
                if pos == to && trailing {
                    out.push(Ln { start: to, end: to, full: to });
                }
            }
            None => {
                out.push(Ln { start: pos, end: to, full: to });
                pos = to;
            }
        }
    }
    out
}

fn lead(s: &str) -> &str {
    let n = s.len() - s.trim_start().len();
    &s[..n]
}

fn clean_space(indent: &str) -> String {
    indent.chars().map(|c| if c.is_whitespace() { c } else { ' ' }).collect()
}

fn leading_words(line: &str) -> &str {
    let line = line.trim();
    match line.find(|c: char| !c.is_alphanumeric() && !c.is_whitespace()) {
        Some(i) => &line[..i],
        None => line,
    }
}

/// The range of `-` (or `=`) a line is made of, when it is only that.
fn find_underline(line: &str, base: usize, ch: char) -> Option<(usize, usize)> {
    let t = line.trim();
    if t.is_empty() || !t.chars().all(|c| c == ch) {
        return None;
    }
    let s = base + line.len() - line.trim_start().len();
    Some((s, s + t.len()))
}

// -------------------------------------------------------------- docstring

pub(crate) struct Ds<'a> {
    pub(crate) src: &'a str,
    pub(crate) def: Node<'a>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) body_start: usize,
    pub(crate) body_end: usize,
    pub(crate) triple: bool,
}

impl<'a> Ds<'a> {
    fn body(&self) -> &'a str {
        &self.src[self.body_start..self.body_end]
    }
    /// What stands on the docstring's first line before its opening quotes.
    fn indentation(&self) -> &'a str {
        let ls = self.src[..self.start].rfind('\n').map_or(0, |i| i + 1);
        &self.src[ls..self.start]
    }
}

/// The docstring of a module, class or function: its first statement, when
/// that is one plain string.
pub(crate) fn docstring_of<'a>(def: Node<'a>, src: &'a str) -> Option<Ds<'a>> {
    let first = match def.kind() {
        "module" => statements(def).into_iter().next()?,
        _ => statements(def.child_by_field_name("body")?).into_iter().next()?,
    };
    if first.kind() != "expression_statement" || first.named_child_count() != 1 {
        return None;
    }
    let s = first.named_child(0)?;
    if s.kind() != "string" {
        return None;
    }
    let (prefix, _) = string_parts(s, src)?;
    if prefix.contains(['f', 'b', 't']) {
        return None;
    }
    let cs = children(s);
    let st = cs.iter().find(|c| c.kind() == "string_start")?;
    let en = cs.iter().rev().find(|c| c.kind() == "string_end")?;
    let opener = text(*st, src).trim_start_matches(|c: char| c.is_ascii_alphabetic());
    let ds = Ds { src, def, start: s.start_byte(), end: s.end_byte(), body_start: st.end_byte(), body_end: en.start_byte(), triple: opener.len() >= 3 };
    if ds.body().trim().is_empty() {
        return None;
    }
    Some(ds)
}

// --------------------------------------------------------------- sections

const NUMPY: &[&str] = &[
    "Attributes", "Examples", "Methods", "Notes", "Raises", "References", "Returns", "See Also", "Warnings", "Warns", "Yields", "Extended Summary",
    "Other Params", "Other Parameters", "Parameters", "Receives", "Short Summary",
];
const GOOGLE: &[&str] = &[
    "Attributes", "Examples", "Methods", "Notes", "Raises", "References", "Returns", "See Also", "Warnings", "Warns", "Yields", "Args", "Arguments",
    "Attention", "Caution", "Danger", "Error", "Example", "Hint", "Important", "Keyword Args", "Keyword Arguments", "Note", "Other Args",
    "Other Arguments", "Return", "Tip", "Todo", "Warning", "Yield",
];
const ALL_KINDS: &[&str] = &[
    "Args", "Arguments", "Attention", "Attributes", "Caution", "Danger", "Error", "Example", "Examples", "Extended Summary", "Hint", "Important",
    "Keyword Args", "Keyword Arguments", "Methods", "Note", "Notes", "Other Args", "Other Arguments", "Other Params", "Other Parameters",
    "Parameters", "Raises", "Receives", "References", "Return", "Returns", "See Also", "Short Summary", "Tip", "Todo", "Warning", "Warnings",
    "Warns", "Yield", "Yields",
];

fn kind_of(name: &str) -> Option<&'static str> {
    let low = name.to_ascii_lowercase();
    ALL_KINDS.iter().copied().find(|k| k.to_ascii_lowercase() == low)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Style {
    Numpy,
    Google,
}

#[derive(Clone, Debug)]
struct Sect {
    kind: &'static str,
    /// Width of the header line's indentation.
    indent: usize,
    name_start: usize,
    name_end: usize,
    /// The header line's start.
    start: usize,
    /// The next section's header line, or the end of the body.
    end: usize,
    /// The header line's end with its terminator.
    full: usize,
}

impl Sect {
    fn name<'a>(&self, src: &'a str) -> &'a str {
        &src[self.name_start..self.name_end]
    }
    fn header_end(&self, src: &str) -> usize {
        let t = &src[self.start..self.full];
        self.start + t.trim_end_matches(['\n', '\r']).len()
    }
    fn after_name<'a>(&self, src: &'a str) -> &'a str {
        &src[self.name_end..self.header_end(src)]
    }
    fn following(&self, src: &str) -> Vec<Ln> {
        split_lines(src, self.full, self.end, true)
    }
}

fn has_parameter(def: Node<'_>, src: &str, name: &str) -> bool {
    def.kind() == "function_definition" && params_of(def, src).iter().any(|p| p.name == name)
}

fn sections_in(ds: &Ds<'_>, style: Style) -> Vec<Sect> {
    let src = ds.src;
    let allowed = if style == Style::Numpy { NUMPY } else { GOOGLE };
    let lines = split_lines(src, ds.body_start, ds.body_end, false);
    let mut out: Vec<Sect> = Vec::new();
    let mut last: Option<Sect> = None;
    if lines.is_empty() {
        return out;
    }
    let mut prev: Option<Ln> = Some(lines[0]);
    let mut directive: Option<usize> = None;
    for i in 1..lines.len() {
        let line = lines[i];
        let text_ = &src[line.start..line.end];
        let indent = lead(text_).len();
        if let Some(active) = directive
            && (text_.trim().is_empty() || indent > active) {
                prev = Some(line);
                continue;
            }
        directive = None;
        if text_.trim_start().starts_with(".. ") {
            directive = Some(indent);
            prev = Some(line);
            continue;
        }
        let words = leading_words(text_);
        if let Some(kind) = kind_of(words).filter(|k| allowed.contains(k)) {
            let nsize = words.len();
            let next = lines.get(i + 1).copied();
            if is_section(ds, text_, indent, nsize, kind, last.as_ref(), prev, next) {
                if let Some(mut l) = last.take() {
                    l.end = line.start;
                    out.push(l);
                }
                last = Some(Sect { kind, indent, name_start: line.start + indent, name_end: line.start + indent + nsize, start: line.start, end: line.start, full: line.full });
            }
        }
        prev = Some(line);
    }
    if let Some(mut l) = last.take() {
        l.end = ds.body_end;
        out.push(l);
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn is_section(ds: &Ds<'_>, line: &str, indent: usize, nsize: usize, kind: &str, prev_sect: Option<&Sect>, prev_line: Option<Ln>, next_line: Option<Ln>) -> bool {
    let src = ds.src;
    let suffix = line[indent + nsize..].trim();
    if !(suffix == ":" || suffix.is_empty()) {
        return false;
    }
    let next_underline = next_line.is_some_and(|n| {
        let t = src[n.start..n.end].trim();
        !t.is_empty() && t.chars().all(|c| c == '-' || c == '=')
    });
    if next_underline {
        return true;
    }
    let prev_text = prev_line.map(|p| src[p.start..p.end].trim());
    let ends_paragraph = prev_text.is_none_or(|t| t.is_empty() || [',', ';', '.', '-', '\\', '/', ']', '}', ')'].iter().any(|c| t.ends_with(*c)));
    if !ends_paragraph {
        return false;
    }
    if let Some(ps) = prev_sect {
        let verbatim = &line[indent..indent + nsize];
        if ps.indent < indent && (kind != verbatim || has_parameter(ds.def, src, kind)) {
            return false;
        }
        if prev_text.is_some_and(str::is_empty) {
            return true;
        }
        if verbatim.chars().next().is_some_and(char::is_lowercase) && kind != verbatim {
            return false;
        }
    }
    true
}

/// The sections Ruff reads the docstring as, with their style.
fn sections_of(ds: &Ds<'_>) -> (Vec<Sect>, Style) {
    let numpy = sections_in(ds, Style::Numpy);
    if numpy.iter().any(|s| matches!(s.kind, "Parameters" | "Other Params" | "Other Parameters")) {
        return (numpy, Style::Numpy);
    }
    let google = sections_in(ds, Style::Google);
    if google.iter().any(|s| matches!(s.kind, "Args" | "Arguments" | "Keyword Args" | "Keyword Arguments" | "Other Args" | "Other Arguments")) {
        return (google, Style::Google);
    }
    if google.len() > numpy.len() {
        (google, Style::Google)
    } else {
        (numpy, Style::Numpy)
    }
}

// ------------------------------------------------------------- parameters

pub(crate) struct Param {
    pub(crate) name: String,
    /// `*`, `**` or nothing.
    pub(crate) stars: usize,
    pub(crate) annotation: Option<String>,
}

pub(crate) fn params_of(def: Node<'_>, src: &str) -> Vec<Param> {
    let mut out = Vec::new();
    let Some(ps) = def.child_by_field_name("parameters") else { return out };
    for c in kids(ps) {
        let (name_node, stars, ann) = match c.kind() {
            "identifier" => (Some(c), 0, None),
            "typed_parameter" => {
                let inner = c.named_child(0);
                let ann = c.child_by_field_name("type").map(|t| text(t, src).to_owned());
                match inner {
                    Some(i) if i.kind() == "list_splat_pattern" => (i.named_child(0), 1, ann),
                    Some(i) if i.kind() == "dictionary_splat_pattern" => (i.named_child(0), 2, ann),
                    other => (other, 0, ann),
                }
            }
            "default_parameter" => (c.child_by_field_name("name"), 0, None),
            "typed_default_parameter" => (c.child_by_field_name("name"), 0, c.child_by_field_name("type").map(|t| text(t, src).to_owned())),
            "list_splat_pattern" => (c.named_child(0), 1, None),
            "dictionary_splat_pattern" => (c.named_child(0), 2, None),
            _ => (None, 0, None),
        };
        if let Some(n) = name_node.filter(|n| n.kind() == "identifier") {
            out.push(Param { name: text(n, src).to_owned(), stars, annotation: ann });
        }
    }
    out
}

pub(crate) fn decorator_names(def: Node<'_>, src: &str) -> Vec<String> {
    decorators(def)
        .into_iter()
        .map(|d| {
            let t = if d.kind() == "call" { d.child_by_field_name("function").unwrap_or(d) } else { d };
            text(t, src).to_owned()
        })
        .collect()
}

pub(crate) fn is_method(def: Node<'_>) -> bool {
    let mut p = def.parent();
    if p.is_some_and(|x| x.kind() == "decorated_definition") {
        p = p.and_then(|x| x.parent());
    }
    p.and_then(|b| b.parent()).is_some_and(|c| c.kind() == "class_definition")
}

fn is_identifier(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(|c| c.is_alphabetic() || c == '_') && cs.all(|c| c.is_alphanumeric() || c == '_')
}

// --------------------------------------------------------------- analysis

type Found = (&'static str, NativeFinding);

fn f(start: usize, end: usize, message: String, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message, edits }
}

fn del(start: usize, end: usize) -> Vec<TextEdit> {
    vec![TextEdit::new(start, end, "")]
}

/// D207, D208, D206-free: how every line of the docstring is indented
/// against the line the docstring starts on.
fn indent_findings(ds: &Ds<'_>, out: &mut Vec<Found>) {
    let src = ds.src;
    let ls = split_lines(src, ds.body_start, ds.body_end, true);
    if ls.len() < 2 {
        return;
    }
    let di = ds.indentation();
    let di_size = di.chars().count();
    let mut over: Vec<Ln> = Vec::new();
    let mut smallest: Option<usize> = Some(usize::MAX);
    let mut last: Option<Ln> = None;
    for (i, line) in ls.iter().enumerate() {
        if last.is_none() || last.is_some_and(|l| src[l.start..l.end].ends_with('\\')) {
            last = Some(*line);
            continue;
        }
        let is_last = i + 1 == ls.len();
        let lt = &src[line.start..line.end];
        let blank = lt.trim().is_empty();
        if !is_last && blank {
            last = Some(*line);
            continue;
        }
        let li = lead(lt);
        let li_size = li.chars().count();
        if (is_last || !blank) && li_size < di_size {
            out.push(("D207", f(line.start, line.start + li.len(), "Docstring is under-indented".into(), vec![TextEdit::new(line.start, line.start + li.len(), clean_space(di))])));
        }
        let closing_only = is_last && blank;
        if !closing_only {
            smallest = smallest.and_then(|s| {
                let o = li_size.saturating_sub(di_size);
                if o > 0 {
                    over.push(*line);
                    Some(s.min(o))
                } else {
                    None
                }
            });
        }
        last = Some(*line);
    }
    if let Some(small) = smallest {
        for line in over {
            let lt = &src[line.start..line.end];
            let li = lead(lt);
            let indent = clean_space(di);
            let edit = if indent.is_empty() {
                TextEdit::new(line.start, line.start + li.len(), "")
            } else {
                let take: usize = lt.chars().take(di_size + small).map(char::len_utf8).sum();
                TextEdit::new(line.start, line.start + take, indent)
            };
            out.push(("D208", f(line.start, line.start + li.len(), "Docstring is over-indented".into(), vec![edit])));
        }
    }
    if let Some(l) = last {
        let lt = &src[l.start..l.end];
        let li = lead(lt);
        if li.chars().count().saturating_sub(di_size) > 0 && li.len() == lt.len() {
            let indent = clean_space(di);
            out.push(("D208", f(l.start, l.start + li.len(), "Docstring is over-indented".into(), vec![TextEdit::new(l.start, l.start + li.len(), indent)])));
        }
    }
}

/// D209: the closing quotes of a multi-line docstring on a line of their own.
fn closing_quotes(ds: &Ds<'_>, out: &mut Vec<Found>) {
    if !ds.triple {
        return;
    }
    let src = ds.src;
    let mut n = 0;
    for l in split_lines(src, ds.body_start, ds.body_end, true) {
        if !src[l.start..l.end].trim().is_empty() {
            n += 1;
        }
        if n > 1 {
            let whole = &src[ds.start..ds.end];
            let last = whole.rsplit('\n').next().unwrap_or("").trim();
            if last != "\"\"\"" && last != "'''" {
                let q = ds.end - 3;
                let spaces = src[..q].chars().rev().take_while(|c| c.is_whitespace() && *c != '\n').map(char::len_utf8).sum::<usize>();
                let text = format!("\n{}", clean_space(ds.indentation()));
                out.push(("D209", f(ds.start, ds.end, "Move the closing quotes of a multi-line docstring to a line of their own".into(), vec![TextEdit::new(q - spaces, q, text)])));
            }
            return;
        }
    }
}

/// D402: the docstring's first line repeats the function's signature.
fn no_signature(ds: &Ds<'_>, out: &mut Vec<Found>) {
    if ds.def.kind() != "function_definition" {
        return;
    }
    let Some(name) = ds.def.child_by_field_name("name") else { return };
    let name = text(name, ds.src);
    let Some(first) = ds.body().trim().lines().next() else { return };
    let hit = first.match_indices(name).any(|(i, _)| {
        let before_ok = first[..i].chars().next_back().is_none_or(|c| matches!(c, ' ' | '\t' | ';' | ','));
        before_ok && first[i + name.len()..].starts_with('(')
    });
    if hit {
        out.push(("D402", f(ds.start, ds.end, format!("The first line of the docstring of `{name}` repeats its signature"), Vec::new())));
    }
}

fn section_findings(ds: &Ds<'_>, secs: &[Sect], style: Style, out: &mut Vec<Found>) {
    let src = ds.src;
    let indentation = ds.indentation();
    for (idx, s) in secs.iter().enumerate() {
        let next = secs.get(idx + 1);
        let name = s.name(src).to_owned();
        let name_end = s.name_end;
        let header_end = s.header_end(src);
        // common section checks
        if s.name(src) != s.kind {
            out.push(("D405", f(s.name_start, name_end, format!("Capitalize the section name `{name}`"), vec![TextEdit::new(s.name_start, name_end, s.kind)])));
        }
        let header = &src[s.start..header_end];
        let lead_len = lead(header).len();
        if lead_len > indentation.len() {
            let cs = clean_space(indentation);
            out.push(("D214", f(s.name_start, name_end, format!("Section `{name}` is over-indented"), vec![TextEdit::new(s.start, s.start + lead_len, cs)])));
        }
        let fl = s.following(src);
        let trailing_blank = fl.iter().rev().take_while(|l| src[l.start..l.end].trim().is_empty()).count();
        if let Some(nx) = next {
            if trailing_blank < 2 {
                out.push(("D410", f(s.name_start, name_end, format!("Missing blank line after section `{name}`"), vec![TextEdit::new(nx.start, nx.start, "\n")])));
            }
        } else if trailing_blank < 2 {
            let del_len = if trailing_blank == 1 { fl.last().map_or(0, |l| l.end - l.start) } else { 0 };
            let at = s.end - del_len;
            let text = format!("{}{}", "\n".repeat(2 - trailing_blank), lead(indentation));
            out.push(("D413", f(s.name_start, name_end, format!("Missing blank line after last section `{name}`"), vec![TextEdit::new(at, s.end, text)])));
        }
        let prev_line_blank = split_lines(src, ds.body_start, s.start, false).last().is_some_and(|l| src[l.start..l.end].trim().is_empty());
        if !prev_line_blank {
            out.push(("D411", f(s.name_start, name_end, format!("Missing blank line before section `{name}`"), vec![TextEdit::new(s.start, s.start, "\n")])));
        }
        blanks_and_underline(ds, s, style, &fl, out);
        // style checks
        let suffix = s.after_name(src);
        match style {
            Style::Numpy => {
                if !suffix.is_empty() {
                    out.push(("D406", f(s.name_start, name_end, format!("Section name `{name}` should end with a newline"), del(name_end, name_end + suffix.len()))));
                }
                // Parameters: D417
            }
            Style::Google => {
                if suffix != ":" {
                    out.push(("D416", f(s.name_start, name_end, format!("Section name `{name}` should end with a colon"), vec![TextEdit::new(name_end, name_end + suffix.len(), ":")])));
                }
            }
        }
    }
}

fn blanks_and_underline(ds: &Ds<'_>, s: &Sect, style: Style, fl: &[Ln], out: &mut Vec<Found>) {
    let src = ds.src;
    let indentation = ds.indentation();
    let name = s.name(src).to_owned();
    let (ns, ne) = (s.name_start, s.name_end);
    let header_end = s.header_end(src);
    let mut i = 0;
    let mut blanks = 0;
    let mut blank_end = s.full;
    while i < fl.len() && src[fl[i].start..fl[i].end].trim().is_empty() {
        blank_end = fl[i].full;
        blanks += 1;
        i += 1;
    }
    let dashes = "-".repeat(name.len());
    let add_underline = |out: &mut Vec<Found>| {
        let ins = format!("\n{}{}", clean_space(indentation), dashes);
        out.push(("D407", f(ns, ne, format!("Missing dashed underline after section `{name}`"), vec![TextEdit::new(header_end, header_end, ins)])));
    };
    let Some(non_blank) = fl.get(i).copied() else {
        if style == Style::Numpy {
            add_underline(out);
        }
        out.push(("D414", f(ns, ne, format!("Section `{name}` has no content"), Vec::new())));
        return;
    };
    i += 1;
    let nb_text = &src[non_blank.start..non_blank.end];
    if let Some((us, ue)) = find_underline(nb_text, non_blank.start, '-') {
        if blanks > 0 {
            out.push(("D408", f(us, ue, format!("Section underline should be in the line following the section's name (`{name}`)"), del(s.full, blank_end))));
        }
        if ue - us != name.len() {
            out.push(("D409", f(us, ue, format!("Section underline should match the length of its name (`{name}`)"), vec![TextEdit::new(us, ue, dashes.clone())])));
        }
        let lsp = lead(nb_text);
        if lsp.len() > indentation.len() {
            let cs = clean_space(indentation);
            out.push(("D215", f(us, ue, format!("Section underline is over-indented (`{name}`)"), vec![TextEdit::new(non_blank.start, non_blank.start + lsp.len(), cs)])));
        }
        match fl.get(i) {
            Some(after) if src[after.start..after.end].trim().is_empty() => {
                i += 1;
                let mut n = 1;
                let mut after_end = after.full;
                while i < fl.len() && src[fl[i].start..fl[i].end].trim().is_empty() {
                    after_end = fl[i].full;
                    n += 1;
                    i += 1;
                }
                if i >= fl.len() {
                    out.push(("D414", f(ns, ne, format!("Section `{name}` has no content"), Vec::new())));
                } else {
                    let sphinx = src[after_end..].lines().next().is_some_and(|l| l.trim_start().starts_with(".. "));
                    if sphinx {
                        if n > 1 {
                            out.push(("D412", f(ns, ne, format!("No blank lines allowed between a section header and its content (`{name}`)"), vec![TextEdit::new(after.start, after_end, "\n")])));
                        }
                    } else {
                        out.push(("D412", f(ns, ne, format!("No blank lines allowed between a section header and its content (`{name}`)"), del(after.start, after_end))));
                    }
                }
            }
            Some(_) => {}
            None => out.push(("D414", f(ns, ne, format!("Section `{name}` has no content"), Vec::new()))),
        }
    } else {
        if style == Style::Numpy {
            if let Some((us, ue)) = find_underline(nb_text, non_blank.start, '=') {
                out.push(("D407", f(us, ue, format!("Missing dashed underline after section `{name}`"), vec![TextEdit::new(us, ue, dashes.clone())])));
            } else {
                add_underline(out);
            }
        }
        if blanks > 0 {
            let sphinx = src[blank_end..].lines().next().is_some_and(|l| l.trim_start().starts_with(".. "));
            if sphinx {
                if blanks > 1 {
                    out.push(("D412", f(ns, ne, format!("No blank lines allowed between a section header and its content (`{name}`)"), vec![TextEdit::new(s.full, blank_end, "\n")])));
                }
            } else {
                out.push(("D412", f(ns, ne, format!("No blank lines allowed between a section header and its content (`{name}`)"), del(s.full, blank_end))));
            }
        }
    }
}

/// D420: a section after one that belongs below it.
fn order_findings(secs: &[Sect], style: Style, src: &str, out: &mut Vec<Found>) {
    let rank = |k: &str| -> Option<usize> {
        match style {
            Style::Numpy => ["Short Summary", "Extended Summary", "Parameters", "Attributes", "Methods", "Returns", "Yields", "Receives", "Other Parameters", "Raises", "Warns", "Warnings", "See Also", "Notes", "References", "Examples"]
                .iter()
                .position(|x| *x == k || (k == "Other Params" && *x == "Other Parameters")),
            Style::Google => match k {
                "Args" | "Arguments" | "Keyword Args" | "Keyword Arguments" | "Other Args" | "Other Arguments" => Some(0),
                "Returns" | "Return" => Some(1),
                "Yields" | "Yield" => Some(2),
                "Raises" => Some(3),
                _ => None,
            },
        }
    };
    let mut max: Option<(usize, String)> = None;
    for s in secs {
        let Some(r) = rank(s.kind) else { continue };
        if let Some((mr, mname)) = &max
            && r < *mr {
                out.push(("D420", f(s.name_start, s.name_end, format!("Section `{}` belongs before `{mname}`", s.name(src)), Vec::new())));
                continue;
            }
        max = Some((r, s.name(src).to_owned()));
    }
}

fn google_args(s: &Sect, src: &str) -> BTreeSet<String> {
    let fl = s.following(src);
    let Some(first) = fl.first() else { return BTreeSet::new() };
    let first_text = &src[first.start..first.end];
    let lead_first = lead(first_text);
    let relevant: Vec<&str> = fl
        .iter()
        .map(|l| &src[l.start..l.end])
        .filter(|t| t.is_empty() || (t.starts_with(lead_first) && find_underline(t, 0, '-').is_none()))
        .collect();
    // dedent by the first line's indentation
    let dedented: Vec<String> = relevant.iter().map(|t| t.strip_prefix(lead_first).unwrap_or(t).to_owned()).collect();
    let joined = dedented.join("\n");
    let mut entries: Vec<String> = Vec::new();
    for line in joined.trim().lines() {
        if line.chars().next().is_none_or(char::is_whitespace) {
            if let Some(l) = entries.last_mut() {
                l.push_str(line);
                l.push('\n');
            }
        } else {
            entries.push(format!("{line}\n"));
        }
    }
    let mut out = BTreeSet::new();
    for e in entries {
        // ^\s*(\*?\*?\w+)\s*(\(.*?\))?\s*:\n?\s*.+
        let t = e.trim_start();
        let name_len = t.chars().take_while(|c| *c == '*').count();
        let rest = &t[name_len..];
        let word: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if word.is_empty() || name_len > 2 {
            continue;
        }
        let mut after = rest[word.len()..].trim_start();
        if after.starts_with('(')
            && let Some(close) = after.find(')') {
                after = after[close + 1..].trim_start();
            }
        if let Some(r) = after.strip_prefix(':')
            && !r.trim().is_empty() {
                out.insert(format!("{}{word}", "*".repeat(name_len)));
            }
    }
    out
}

fn numpy_params(s: &Sect, src: &str) -> BTreeSet<String> {
    let adjusted = {
        let fl = s.following(src);
        fl.iter().map(|l| &src[l.start..l.end]).collect::<Vec<_>>().join("\n").replace("\\\n", "")
    };
    let lines: Vec<&str> = adjusted.split('\n').collect();
    let header = &src[s.start..s.header_end(src)];
    let level = lead(header);
    let mut out = BTreeSet::new();
    for w in lines.windows(2) {
        let (cur, next) = (w[0], w[1]);
        if lead(cur) == level && lead(next).len() > lead(cur).len() && !next.trim().is_empty() {
            let params = match cur.find(':') {
                Some(i) => &cur[..i],
                None => cur.trim(),
            };
            for p in params.split(',') {
                out.insert(p.trim().to_owned());
            }
        }
    }
    out
}

fn undocumented_params(ds: &Ds<'_>, secs: &[Sect], style: Style, out: &mut Vec<Found>) {
    let src = ds.src;
    if ds.def.kind() != "function_definition" {
        return;
    }
    let documented: BTreeSet<String> = match style {
        Style::Numpy => {
            let mut all = BTreeSet::new();
            let mut any = false;
            for s in secs.iter().filter(|s| s.kind == "Parameters") {
                any = true;
                all.extend(numpy_params(s, src));
            }
            if !any {
                return;
            }
            all
        }
        Style::Google => {
            let mut all = BTreeSet::new();
            let mut any = false;
            for s in secs.iter().filter(|s| matches!(s.kind, "Args" | "Arguments" | "Keyword Args" | "Keyword Arguments" | "Other Args" | "Other Arguments")) {
                any = true;
                all.extend(google_args(s, src));
            }
            if !any {
                return;
            }
            all
        }
    };
    let params = params_of(ds.def, src);
    let skip_first = is_method(ds.def) && !decorator_names(ds.def, src).iter().any(|d| d == "staticmethod");
    let mut missing: BTreeSet<String> = BTreeSet::new();
    let mut seen_plain = 0;
    for p in params.iter().filter(|p| p.stars == 0) {
        seen_plain += 1;
        if skip_first && seen_plain == 1 {
            continue;
        }
        if !p.name.starts_with('_') && !documented.contains(&p.name) {
            missing.insert(p.name.clone());
        }
    }
    for p in params.iter().filter(|p| p.stars > 0) {
        let starred = format!("{}{}", "*".repeat(p.stars), p.name);
        if p.stars == 2 && p.annotation.as_deref().is_some_and(|a| a.contains("Unpack")) {
            continue;
        }
        if !p.name.starts_with('_') && !documented.contains(&p.name) && !documented.contains(&starred) {
            missing.insert(starred);
        }
    }
    if missing.is_empty() {
        return;
    }
    let Some(name) = ds.def.child_by_field_name("name") else { return };
    let names: Vec<String> = missing.into_iter().map(|n| format!("`{n}`")).collect();
    out.push(("D417", f(name.start_byte(), name.end_byte(), format!("Missing argument descriptions in the docstring for `{}`: {}", text(name, src), names.join(", ")), Vec::new())));
}

// ------------------------------------------------------------------- DOC

#[derive(Clone, Copy, PartialEq, Eq)]
enum RetKind {
    Value,
    Implicit,
    Explicit,
}

#[derive(Default)]
struct Body {
    returns: Vec<RetKind>,
    yields: Vec<bool>,
    raised: Vec<String>,
}

/// The qualified name an exception expression resolves to: through the
/// file's imports, a builtin, or a class defined in the file.
pub(crate) fn exc_name(n: Node<'_>, ctx: &NativeContext<'_>) -> Option<String> {
    let model = ctx.semantic?;
    if let Some(q) = qualified_name(n, model, ctx.source) {
        return Some(q);
    }
    if n.kind() == "identifier" {
        let r = model.reference_at(n.start_byte())?;
        let id = r.symbol?;
        if model.symbol(id).kind == SymbolKind::Class {
            return Some(text(n, ctx.source).to_owned());
        }
    }
    None
}

fn is_base_exception(q: &str) -> bool {
    matches!(q, "builtins.BaseException" | "builtins.Exception" | "builtins.BaseExceptionGroup" | "builtins.ExceptionGroup")
}

struct Walk<'a, 'b> {
    ctx: &'b NativeContext<'a>,
    body: Body,
    handler: Option<Node<'a>>,
    vars: HashMap<String, Node<'a>>,
}

impl<'a, 'b> Walk<'a, 'b> {
    fn store(&mut self, e: Node<'a>) {
        let items: Vec<Node<'a>> = if matches!(e.kind(), "tuple" | "parenthesized_expression") { kids(e) } else { vec![e] };
        let items: Vec<Node<'a>> = items.into_iter().flat_map(|i| if i.kind() == "tuple" { kids(i) } else { vec![i] }).collect();
        for i in items {
            if let Some(q) = exc_name(i, self.ctx)
                && !is_base_exception(&q) {
                    self.body.raised.push(q);
                }
        }
    }

    fn walk(&mut self, n: Node<'a>) {
        match n.kind() {
            "function_definition" | "class_definition" | "lambda" => return,
            "except_clause" => {
                let saved_h = self.handler;
                let saved_v = std::mem::take(&mut self.vars);
                let first = named_children(n).into_iter().find(|c| c.kind() != "block");
                let (ty, alias) = match first {
                    Some(c) if c.kind() == "as_pattern" => (c.named_child(0), c.child_by_field_name("alias").or_else(|| c.named_child(1))),
                    other => (other, None),
                };
                self.handler = ty;
                if let (Some(t), Some(a)) = (ty, alias) {
                    self.vars.insert(text(a, self.ctx.source).trim().to_owned(), t);
                }
                for c in named_children(n) {
                    self.walk(c);
                }
                self.handler = saved_h;
                self.vars = saved_v;
                return;
            }
            "raise_statement" => {
                let exc = n.named_child(0).filter(|c| Some(*c) != n.child_by_field_name("cause"));
                match exc {
                    Some(e) => {
                        let callee = if e.kind() == "call" { e.child_by_field_name("function").unwrap_or(e) } else { e };
                        if let Some(q) = exc_name(callee, self.ctx) {
                            self.body.raised.push(q);
                        } else if e.kind() == "identifier"
                            && let Some(t) = self.vars.get(text(e, self.ctx.source)).copied() {
                                self.store(t);
                            }
                    }
                    None => {
                        if let Some(h) = self.handler {
                            self.store(h);
                        }
                    }
                }
            }
            "return_statement" => {
                let kind = match n.named_child(0) {
                    None => RetKind::Implicit,
                    Some(v) if v.kind() == "none" => RetKind::Explicit,
                    Some(_) => RetKind::Value,
                };
                self.body.returns.push(kind);
            }
            "yield" => {
                let is_from = children(n).iter().any(|c| c.kind() == "from");
                let val = n.named_child(0);
                if is_from {
                    self.body.yields.push(false);
                } else {
                    self.body.yields.push(val.is_none_or(|v| v.kind() == "none"));
                }
            }
            _ => {}
        }
        for c in named_children(n) {
            self.walk(c);
        }
    }
}

fn body_of<'a>(def: Node<'a>, ctx: &NativeContext<'a>) -> Body {
    let mut w = Walk { ctx, body: Body::default(), handler: None, vars: HashMap::new() };
    if let Some(b) = def.child_by_field_name("body") {
        w.walk(b);
    }
    let mut body = w.body;
    body.raised.sort();
    body.raised.dedup();
    body
}

pub(crate) fn is_stub_fn(def: Node<'_>, src: &str) -> bool {
    if decorator_names(def, src).iter().any(|d| d == "overload" || d.ends_with(".overload")) {
        return true;
    }
    let Some(b) = def.child_by_field_name("body") else { return false };
    statements(b).into_iter().all(|s| match s.kind() {
        "pass_statement" => true,
        "expression_statement" => s.named_child(0).is_some_and(|c| matches!(c.kind(), "string" | "ellipsis")),
        "raise_statement" => s.named_child(0).is_some_and(|e| {
            let t = text(if e.kind() == "call" { e.child_by_field_name("function").unwrap_or(e) } else { e }, src);
            t == "NotImplementedError" || t == "NotImplemented"
        }),
        _ => false,
    })
}

pub(crate) fn is_abstract(def: Node<'_>, src: &str) -> bool {
    decorator_names(def, src).iter().any(|d| {
        let last = d.rsplit('.').next().unwrap_or(d);
        matches!(last, "abstractmethod" | "abstractproperty" | "abstractclassmethod" | "abstractstaticmethod")
    })
}

pub(crate) fn is_property(def: Node<'_>, src: &str) -> bool {
    decorator_names(def, src).iter().any(|d| {
        let last = d.rsplit('.').next().unwrap_or(d);
        matches!(last, "property" | "cached_property" | "setter" | "getter" | "deleter")
    })
}

/// What a `Generator`/`Iterator` return annotation says about its type
/// arguments: the first argument, and whether the return type is `None`.
fn generator_args<'t>(ann: Node<'t>, ctx: &NativeContext<'_>) -> Option<(Option<Node<'t>>, bool)> {
    let model = ctx.semantic?;
    let (head, slice) = if ann.kind() == "generic_type" || ann.kind() == "subscript" {
        (ann.named_child(0)?, Some(ann))
    } else {
        (ann, None)
    };
    let q = qualified_name(head, model, ctx.source)?;
    let name = q.rsplit('.').next()?;
    let module_ok = q.starts_with("typing.") || q.starts_with("typing_extensions.") || q.starts_with("collections.abc.");
    if !module_ok {
        return None;
    }
    let iter_like = matches!(name, "Iterable" | "AsyncIterable" | "Iterator" | "AsyncIterator");
    let gen_like = matches!(name, "Generator" | "AsyncGenerator");
    if !iter_like && !gen_like {
        return None;
    }
    let mut args: Vec<Node<'_>> = Vec::new();
    if let Some(s) = slice {
        for c in named_children(s) {
            if c.kind() == "type_parameter" || c.kind() == "type" {
                args.extend(named_children(c).into_iter().flat_map(|x| if x.kind() == "type" { named_children(x) } else { vec![x] }));
            }
        }
    }
    let first = args.first().copied();
    let none_returned = if iter_like || args.len() < 3 { true } else { args[2].kind() == "none" || text(args[2], ctx.source) == "None" };
    Some((first, none_returned))
}

fn is_none_node(n: Node<'_>, src: &str) -> bool {
    n.kind() == "none" || text(n, src) == "None"
}

fn parse_raises(s: &Sect, src: &str, style: Option<Style>) -> Vec<String> {
    let content = &src[s.full..s.end];
    let google = || {
        let mut out = Vec::new();
        let mut it = content.lines().peekable();
        let Some(first) = it.peek() else { return out };
        let ind = lead(first).to_owned();
        for l in it {
            if let Some(entry) = l.strip_prefix(ind.as_str()) {
                if entry.chars().next().is_some_and(|c| !c.is_whitespace())
                    && let Some(i) = entry.find(':') {
                        let e = entry[..i].trim();
                        if !e.is_empty() {
                            out.push(e.to_owned());
                        }
                    }
            } else if !l.trim().is_empty() {
                break;
            }
        }
        out
    };
    let numpy = || {
        let mut out = Vec::new();
        let mut it = content.lines();
        let Some(dashes) = it.next() else { return out };
        let ind = lead(dashes).to_owned();
        for l in it {
            if let Some(entry) = l.strip_prefix(ind.as_str()) {
                if entry.starts_with("..") {
                    break;
                }
                if entry.chars().next().is_some_and(|c| !c.is_whitespace()) {
                    out.push(entry.trim_end().to_owned());
                }
            }
        }
        out
    };
    let _ = s;
    match style {
        Some(Style::Google) => google(),
        Some(Style::Numpy) => numpy(),
        None => {
            let g = google();
            if g.is_empty() { numpy() } else { g }
        }
    }
}

fn parse_doc_params(s: &Sect, src: &str) -> Vec<(String, usize, usize)> {
    // Entries at the first line's indentation, in either style.
    let content_start = s.full;
    let content = &src[s.full..s.end];
    let google = {
        let mut out = Vec::new();
        if let Some(first) = content.lines().next() {
            let ind = lead(first);
            let mut pos = 0;
            for l in content.split_inclusive('\n') {
                let line = l.trim_end_matches(['\n', '\r']);
                if let Some(entry) = line.strip_prefix(ind)
                    && entry.chars().next().is_some_and(|c| !c.is_whitespace())
                        && let Some((before, _)) = entry.split_once(':')
                            && let Some(p) = before.split_whitespace().next() {
                                let name = p.trim_start_matches('*');
                                if is_identifier(name) {
                                    let st = content_start + pos + ind.len();
                                    out.push((name.to_owned(), st, st + p.len()));
                                }
                            }
                pos += l.len();
            }
        }
        out
    };
    if !google.is_empty() {
        return google;
    }
    let mut out = Vec::new();
    let mut lines = content.split_inclusive('\n');
    let Some(dashes) = lines.next() else { return out };
    let ind = lead(dashes).to_owned();
    let mut pos = dashes.len();
    for l in lines {
        let line = l.trim_end_matches(['\n', '\r']);
        if let Some(entry) = line.strip_prefix(ind.as_str())
            && entry.chars().next().is_some_and(|c| !c.is_whitespace()) {
                let before = entry.split(':').next().unwrap_or("").trim_end();
                let mut off = 0;
                for part in before.split(',') {
                    let t = part.trim();
                    let name = t.trim_start_matches('*');
                    if is_identifier(name) {
                        let st = content_start + pos + ind.len() + off + (part.len() - part.trim_start().len());
                        out.push((name.to_owned(), st, st + t.len()));
                    }
                    off += part.len() + 1;
                }
            }
        pos += l.len();
    }
    out
}

fn doc_findings(ds: &Ds<'_>, secs: &[Sect], ctx: &NativeContext<'_>, out: &mut Vec<Found>) {
    let def = ds.def;
    if def.kind() != "function_definition" || is_stub_fn(def, ds.src) {
        return;
    }
    let src = ds.src;
    // The sections, read for the docstring's own style (Ruff reads both when none is set).
    let sect = |kinds: &[&str]| secs.iter().rev().find(|s| kinds.contains(&s.kind));
    let params_sec = sect(&["Args", "Arguments", "Parameters"]);
    let raises_sec = sect(&["Raises"]);
    let returns_sec = sect(&["Returns"]);
    let yields_sec = sect(&["Yields"]);
    let body = body_of(def, ctx);
    let returns_ann = def.child_by_field_name("return_type");
    let name = def.child_by_field_name("name").map_or("", |n| text(n, src));
    let (ds_start, ds_end) = (ds.start, ds.end);
    let report = |out: &mut Vec<Found>, code: &'static str, message: String, start: usize, end: usize| {
        out.push((code, f(start, end, message, Vec::new())));
    };

    if name != "__new__" && returns_sec.is_none() && !is_property(def, src) && !body.returns.is_empty() {
        match returns_ann {
            Some(r) if !is_none_node(r, src) => {
                let gen_none = !body.yields.is_empty() && generator_args(r, ctx).is_some_and(|(_, none)| none);
                if !gen_none {
                    report(out, "DOC201", "`return` is not documented in the docstring".into(), ds_start, ds_end);
                }
            }
            None if body.returns.contains(&RetKind::Value) => {
                report(out, "DOC201", "`return` is not documented in the docstring".into(), ds_start, ds_end);
            }
            _ => {}
        }
    }
    if yields_sec.is_none() && !body.yields.is_empty() {
        match returns_ann {
            Some(r) if !generator_args(r, ctx).is_some_and(|(first, _)| first.is_none_or(|x| is_none_node(x, src))) => {
                report(out, "DOC402", "`yield` is not documented in the docstring".into(), ds_start, ds_end);
            }
            None if body.yields.iter().any(|none| !none) => {
                report(out, "DOC402", "`yield` is not documented in the docstring".into(), ds_start, ds_end);
            }
            _ => {}
        }
    }
    let documented_raises: Vec<String> = raises_sec.map(|s| parse_raises(s, src, None)).unwrap_or_default();
    for q in &body.raised {
        let last = q.rsplit('.').next().unwrap_or(q);
        if last == "NotImplementedError" {
            continue;
        }
        let segs: Vec<&str> = q.split('.').collect();
        let ok = raises_sec.is_some() && documented_raises.iter().any(|d| {
            let ds_: Vec<&str> = d.split('.').collect();
            segs.ends_with(&ds_)
        });
        if !ok {
            report(out, "DOC501", format!("Raised exception `{last}` is not documented in the docstring"), ds_start, ds_end);
        }
    }
    let ps = params_of(def, src);
    if ps.iter().all(|p| p.stars == 0)
        && let Some(sec) = params_sec {
            for (pname, st, en) in parse_doc_params(sec, src) {
                if !ps.iter().any(|p| p.name == pname) {
                    report(out, "DOC102", format!("Documented parameter `{pname}` is not in the signature"), st, en);
                }
            }
        }
    if !is_abstract(def, src) {
        if returns_sec.is_some() && (body.returns.is_empty() || body.returns.iter().all(|k| *k == RetKind::Implicit)) {
            report(out, "DOC202", "The docstring has a `Returns` section but the function returns nothing".into(), ds_start, ds_end);
        }
        if yields_sec.is_some() && body.yields.is_empty() {
            report(out, "DOC403", "The docstring has a `Yields` section but the function yields nothing".into(), ds_start, ds_end);
        }
        if let Some(_s) = raises_sec {
            let mut extra: Vec<String> = Vec::new();
            for d in &documented_raises {
                let ds_: Vec<&str> = d.split('.').collect();
                if !body.raised.iter().any(|q| q.split('.').collect::<Vec<_>>().ends_with(&ds_)) {
                    extra.push(d.clone());
                }
            }
            if !extra.is_empty() {
                report(out, "DOC502", format!("Documented exception{} not raised: {}", if extra.len() == 1 { " is" } else { "s are" }, extra.iter().map(|e| format!("`{e}`")).collect::<Vec<_>>().join(", ")), ds_start, ds_end);
            }
        }
    }
}

fn analyze(def: Node<'_>, ctx: &NativeContext<'_>, code: &str) -> Vec<Found> {
    let mut out = Vec::new();
    let Some(ds) = docstring_of(def, ctx.source) else { return out };
    match code {
        "D207" | "D208" => indent_findings(&ds, &mut out),
        "D209" => closing_quotes(&ds, &mut out),
        "D402" => no_signature(&ds, &mut out),
        c if c.starts_with("DOC") => {
            let (secs, _) = sections_of(&ds);
            doc_findings(&ds, &secs, ctx, &mut out);
        }
        _ => {
            let (secs, style) = sections_of(&ds);
            section_findings(&ds, &secs, style, &mut out);
            order_findings(&secs, style, ds.src, &mut out);
            undocumented_params(&ds, &secs, style, &mut out);
        }
    }
    out.retain(|(c, _)| *c == code);
    out
}

const DEFS: &[&str] = &["function_definition", "class_definition", "module"];

macro_rules! doc_rule {
    ($name:ident, $id:literal, $code:literal, $sem:literal) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                DEFS
            }
            fn needs_semantic(&self) -> bool {
                $sem
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                for (_, finding) in analyze(node.get_inner_node(), ctx, $code) {
                    out.push(finding);
                }
            }
        }
    };
}

doc_rule!(UnderIndentation, "py/under-indentation", "D207", false);
doc_rule!(OverIndentation, "py/over-indentation", "D208", false);
doc_rule!(NewLineAfterLastParagraph, "py/new-line-after-last-paragraph", "D209", false);
doc_rule!(OverindentedSection, "py/overindented-section", "D214", false);
doc_rule!(OverindentedSectionUnderline, "py/overindented-section-underline", "D215", false);
doc_rule!(SignatureInDocstring, "py/signature-in-docstring", "D402", false);
doc_rule!(NonCapitalizedSectionName, "py/non-capitalized-section-name", "D405", false);
doc_rule!(MissingNewLineAfterSectionName, "py/missing-new-line-after-section-name", "D406", false);
doc_rule!(MissingDashedUnderlineAfterSection, "py/missing-dashed-underline-after-section", "D407", false);
doc_rule!(MissingSectionUnderlineAfterName, "py/missing-section-underline-after-name", "D408", false);
doc_rule!(MismatchedSectionUnderlineLength, "py/mismatched-section-underline-length", "D409", false);
doc_rule!(NoBlankLineAfterSection, "py/no-blank-line-after-section", "D410", false);
doc_rule!(NoBlankLineBeforeSection, "py/no-blank-line-before-section", "D411", false);
doc_rule!(BlankLinesBetweenHeaderAndContent, "py/blank-lines-between-header-and-content", "D412", false);
doc_rule!(MissingBlankLineAfterLastSection, "py/missing-blank-line-after-last-section", "D413", false);
doc_rule!(EmptyDocstringSection, "py/empty-docstring-section", "D414", false);
doc_rule!(MissingSectionNameColon, "py/missing-section-name-colon", "D416", false);
doc_rule!(UndocumentedParam, "py/undocumented-param", "D417", false);
doc_rule!(IncorrectSectionOrder, "py/incorrect-section-order", "D420", false);
doc_rule!(DocstringExtraneousParameter, "py/docstring-extraneous-parameter", "DOC102", true);
doc_rule!(DocstringMissingReturns, "py/docstring-missing-returns", "DOC201", true);
doc_rule!(DocstringExtraneousReturns, "py/docstring-extraneous-returns", "DOC202", true);
doc_rule!(DocstringMissingYields, "py/docstring-missing-yields", "DOC402", true);
doc_rule!(DocstringExtraneousYields, "py/docstring-extraneous-yields", "DOC403", true);
doc_rule!(DocstringMissingException, "py/docstring-missing-exception", "DOC501", true);
doc_rule!(DocstringExtraneousException, "py/docstring-extraneous-exception", "DOC502", true);

pub static RULES: &[&dyn NativeRule] = &[
    &UnderIndentation,
    &OverIndentation,
    &NewLineAfterLastParagraph,
    &OverindentedSection,
    &OverindentedSectionUnderline,
    &SignatureInDocstring,
    &NonCapitalizedSectionName,
    &MissingNewLineAfterSectionName,
    &MissingDashedUnderlineAfterSection,
    &MissingSectionUnderlineAfterName,
    &MismatchedSectionUnderlineLength,
    &NoBlankLineAfterSection,
    &NoBlankLineBeforeSection,
    &BlankLinesBetweenHeaderAndContent,
    &MissingBlankLineAfterLastSection,
    &EmptyDocstringSection,
    &MissingSectionNameColon,
    &UndocumentedParam,
    &IncorrectSectionOrder,
    &DocstringExtraneousParameter,
    &DocstringMissingReturns,
    &DocstringExtraneousReturns,
    &DocstringMissingYields,
    &DocstringExtraneousYields,
    &DocstringMissingException,
    &DocstringExtraneousException,
];

#[cfg(test)]
mod tests {
    use super::*;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        crate::run(rule, Language::Python, src)
    }

    #[test]
    fn google_sections() {
        let src = "def f(a, b):\n    \"\"\"Do it.\n\n    args:\n        a: first.\n    Returns:\n        x\n    \"\"\"\n";
        assert_eq!(run(&NonCapitalizedSectionName, src).len(), 1);
        assert_eq!(run(&NoBlankLineBeforeSection, src).len(), 1);
        assert_eq!(run(&UndocumentedParam, src).len(), 1);
    }

    #[test]
    fn numpy_sections() {
        let src = "def f(a):\n    \"\"\"Do it.\n\n    Parameters\n    ---\n    a : int\n        first\n    \"\"\"\n";
        assert_eq!(run(&MismatchedSectionUnderlineLength, src).len(), 1);
        assert!(run(&UndocumentedParam, src).is_empty());
    }

    #[test]
    fn indentation() {
        let src = "def f():\n    \"\"\"Do it.\n\n  under\n    \"\"\"\n";
        assert_eq!(run(&UnderIndentation, src).len(), 1);
        let src = "def f():\n    \"\"\"Do it.\n\n        over\n        more\n    \"\"\"\n";
        assert_eq!(run(&OverIndentation, src).len(), 2);
    }

    #[test]
    fn pydoclint() {
        let src = "def f(a):\n    \"\"\"Do it.\n\n    Args:\n        a: first\n    \"\"\"\n    if a:\n        raise ValueError\n    return a\n";
        assert_eq!(run(&DocstringMissingReturns, src).len(), 1);
        assert_eq!(run(&DocstringMissingException, src).len(), 1);
        assert!(run(&DocstringExtraneousParameter, src).is_empty());
    }
}


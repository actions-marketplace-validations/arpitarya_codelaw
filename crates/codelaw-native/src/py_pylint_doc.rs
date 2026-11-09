//! W-139 strand W-139-py, round 4: the fifteen rows of Pylint 4.1.3's
//! `docparams` extension — `missing-param-doc`, `missing-type-doc`,
//! `missing-any-param-doc`, `differing-param-doc`, `differing-type-doc`,
//! `useless-param-doc`, `useless-type-doc`, `multiple-constructor-doc`,
//! `missing-raises-doc`, `missing-return-doc`, `missing-return-type-doc`,
//! `missing-yield-doc`, `missing-yield-type-doc`, `redundant-returns-doc` and
//! `redundant-yields-doc`.
//!
//! Behaviour only, read from Pylint's functional tests (`tests/functional/ext/
//! docparams`, GPL-2.0: no code, message or example is taken, L2); every rule
//! here is written fresh. The docstring is read in one of three styles, tried
//! in this order, as Pylint does: Sphinx (`:param x:`), Google (`Args:`),
//! NumPy (`Parameters` over a dashed line). The docstring of a module, class
//! or function comes from `py_ruff_doc`'s locator; the sections are read here,
//! because Pylint's reading differs from Ruff's (the Sphinx field list, the
//! type written beside a name, the `For the other parameters, see` sentence).
//!
//! Options keep Pylint's defaults: `accept_no_param_doc`,
//! `accept_no_raise_doc`, `accept_no_return_doc` and `accept_no_yields_doc`
//! are `true` (a function that documents nothing of the kind is not asked to);
//! `check_private_functions` is `false` (Pylint's `no-docstring-rgx` of `^_`).

use crate::py_ruff_doc::{docstring_of, exc_name, is_abstract, is_property, is_stub_fn, params_of, Ds};
use crate::python_util::{kids, statements, PY};
use crate::util::{children, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use std::collections::{BTreeSet, HashMap};
use tree_sitter::Node;

// ------------------------------------------------------------------ lines

struct L<'a> {
    start: usize,
    end: usize,
    /// End including the terminator.
    full: usize,
    text: &'a str,
    /// Bytes of leading white space.
    ind: usize,
}

fn lines_of(src: &str, from: usize, to: usize) -> Vec<L<'_>> {
    let mut out = Vec::new();
    let mut pos = from;
    while pos < to {
        let (end, full) = match src[pos..to].find('\n') {
            Some(i) => {
                let nl = pos + i;
                (if nl > pos && src.as_bytes()[nl - 1] == b'\r' { nl - 1 } else { nl }, nl + 1)
            }
            None => (to, to),
        };
        let t = &src[pos..end];
        out.push(L { start: pos, end, full, text: t, ind: t.len() - t.trim_start().len() });
        pos = full;
    }
    out
}

fn blank(l: &L<'_>) -> bool {
    l.text.trim().is_empty()
}

/// A documented name with its escapes removed: `\*args` and `raise\_` are
/// written that way in a docstring so Sphinx and Markdown do not read them.
fn norm(name: &str) -> String {
    name.trim().replace('\\', "")
}

fn is_param_name(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '*' | '\\'))
}

// -------------------------------------------------------------- the docs

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Style {
    #[default]
    Unknown,
    Sphinx,
    Google,
    Numpy,
}

/// One documented parameter: its name and the lines that document it.
struct PDoc {
    name: String,
    start: usize,
    end: usize,
}

/// One documented type: the name it is for and the bytes that drop it.
struct TDoc {
    name: String,
    cut: (usize, usize),
}

#[derive(Default)]
struct Doc {
    present: bool,
    style: Style,
    params: Vec<PDoc>,
    types: Vec<TDoc>,
    /// "For the other parameters, see …".
    see: bool,
    returns: bool,
    rtype: bool,
    yields: bool,
    ytype: bool,
    returns_cut: Vec<(usize, usize)>,
    yields_cut: Vec<(usize, usize)>,
    /// A property's description (its first line) and type (`int: …`).
    prop_returns: bool,
    prop_type: bool,
    raises: Vec<String>,
}

impl Doc {
    fn has_params(&self) -> bool {
        !self.params.is_empty() || !self.types.is_empty()
    }
    fn supports_yields(&self) -> bool {
        matches!(self.style, Style::Google | Style::Numpy)
    }
}

/// The text a type is written before, split at the first colon outside
/// brackets and back-quotes that is followed by white space or the end:
/// `list(:class:`X`): the items` is `list(:class:`X`)` and `the items`.
fn split_type(t: &str) -> (Option<&str>, &str) {
    let mut depth = 0i32;
    let mut tick = false;
    for (i, c) in t.char_indices() {
        match c {
            '`' => tick = !tick,
            '(' | '[' | '{' if !tick => depth += 1,
            ')' | ']' | '}' if !tick => depth -= 1,
            ':' if !tick && depth <= 0 && t[i + 1..].chars().next().is_none_or(char::is_whitespace) => {
                return (Some(t[..i].trim()).filter(|s| !s.is_empty()), t[i + 1..].trim());
            }
            _ => {}
        }
    }
    (None, t.trim())
}

fn see_sentence(line: &str) -> bool {
    let l = line.to_lowercase();
    let Some(i) = l.find("for the ") else { return false };
    let mut r = &l[i + 8..];
    if let Some(x) = r.strip_prefix("other ") {
        r = x;
    }
    let Some(x) = r.strip_prefix("param") else { return false };
    let x = x.strip_prefix("eter").unwrap_or(x);
    let x = x.strip_prefix('s').unwrap_or(x);
    let x = x.strip_prefix(',').unwrap_or(x);
    x.trim_start().starts_with("see")
}

fn exception_names(list: &str, out: &mut Vec<String>) {
    for part in list.split(',').flat_map(|p| p.split(" or ")) {
        let n = part.trim().trim_start_matches(['~', '!', '.']).trim();
        if !n.is_empty() && !n.contains(char::is_whitespace) {
            out.push(n.to_owned());
        }
    }
}

// ----------------------------------------------------------------- Sphinx

struct Field<'a> {
    head: &'a str,
    /// The words after the head, with their offsets from the line's start.
    toks: Vec<(usize, &'a str)>,
}

const FIELDS: &[&str] = &[
    "param", "parameter", "arg", "argument", "key", "keyword", "type", "raises", "raise", "except", "exception", "return", "returns", "rtype", "yield",
    "yields", "ytype",
];

fn sphinx_field<'a>(l: &L<'a>) -> Option<Field<'a>> {
    let t = l.text.trim_start();
    let rest = t.strip_prefix(':')?;
    let base = l.ind + 1;
    let mut depth = 0i32;
    let mut tick = false;
    let mut end = None;
    for (i, c) in rest.char_indices() {
        match c {
            '`' => tick = !tick,
            '(' | '[' if !tick => depth += 1,
            ')' | ']' if !tick => depth -= 1,
            ':' if !tick && depth <= 0 => {
                end = Some(i);
                break;
            }
            _ => {}
        }
    }
    let spec = &rest[..end?];
    let mut toks: Vec<(usize, &str)> = Vec::new();
    let b = spec.as_bytes();
    let mut i = 0;
    while i < b.len() {
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        let s = i;
        while i < b.len() && !b[i].is_ascii_whitespace() {
            i += 1;
        }
        if s < i {
            toks.push((base + s, &spec[s..i]));
        }
    }
    let (_, head) = *toks.first()?;
    if !FIELDS.contains(&head) {
        return None;
    }
    toks.remove(0);
    Some(Field { head, toks })
}

/// The end of the field starting at line `i`: its continuation lines are the
/// following non-blank lines indented deeper.
fn field_end(ls: &[L<'_>], i: usize) -> usize {
    let mut j = i + 1;
    while j < ls.len() && !blank(&ls[j]) && ls[j].ind > ls[i].ind {
        j += 1;
    }
    ls[j - 1].full
}

fn parse_sphinx(ls: &[L<'_>], d: &mut Doc) {
    for (i, l) in ls.iter().enumerate() {
        let Some(f) = sphinx_field(l) else { continue };
        let fend = field_end(ls, i);
        match f.head {
            "param" | "parameter" | "arg" | "argument" | "key" | "keyword" => {
                let Some(&(name_off, name)) = f.toks.last() else { continue };
                // Sphinx reads an unescaped `*` as markup: `:param *args:` documents nothing, `:param \*args:` does.
                if name.starts_with('*') {
                    continue;
                }
                let name = norm(name);
                d.params.push(PDoc { name: name.clone(), start: l.start, end: fend });
                if f.toks.len() >= 2 {
                    d.types.push(TDoc { name, cut: (l.start + f.toks[0].0, l.start + name_off) });
                }
            }
            "type" => match f.toks.last() {
                Some(&(_, name)) => d.types.push(TDoc { name: norm(name), cut: (l.start, fend) }),
                None => d.prop_type = true,
            },
            "raises" | "raise" | "except" | "exception" => {
                let joined = f.toks.iter().map(|t| t.1).collect::<Vec<_>>().join(" ");
                exception_names(&joined, &mut d.raises);
            }
            "return" | "returns" => {
                d.returns = true;
                d.returns_cut.push((l.start, fend));
            }
            "rtype" => {
                d.rtype = true;
                d.returns_cut.push((l.start, fend));
            }
            "yield" | "yields" => {
                d.yields = true;
                d.yields_cut.push((l.start, fend));
            }
            "ytype" => {
                d.ytype = true;
                d.yields_cut.push((l.start, fend));
            }
            _ => {}
        }
    }
}

// ----------------------------------------------------------------- Google

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sec {
    Params,
    Raises,
    Returns,
    Yields,
}

fn google_header(t: &str) -> Option<Sec> {
    let name = t.trim().strip_suffix(':')?.trim_end();
    Some(match name {
        "Args" | "Arguments" | "Parameters" | "Keyword Args" | "Keyword Arguments" | "Other Args" | "Other Arguments" | "Other Parameters" => Sec::Params,
        "Raises" | "Raise" => Sec::Raises,
        "Returns" | "Return" => Sec::Returns,
        "Yields" | "Yield" => Sec::Yields,
        _ => return None,
    })
}

/// A Google parameter entry: `name (type): description`. The name and, when
/// a type is written, the bytes that drop it.
fn google_param(l: &L<'_>) -> Option<(String, Option<(usize, usize)>)> {
    let t = &l.text[l.ind..];
    let n = t.find(|c: char| c.is_whitespace() || c == '(' || c == ':').unwrap_or(t.len());
    let name = &t[..n];
    if !is_param_name(name) {
        return None;
    }
    let after = &t[n..];
    let ws = after.len() - after.trim_start().len();
    let r2 = after.trim_start();
    let mut cut = None;
    let rest = if r2.starts_with('(') {
        let mut depth = 0;
        let mut close = None;
        for (i, c) in r2.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let close = close?;
        if !r2[1..close].trim().is_empty() {
            let from = l.start + l.ind + n;
            cut = Some((from, from + ws + close + 1));
        }
        &r2[close + 1..]
    } else {
        r2
    };
    rest.trim_start().starts_with(':').then(|| (norm(name), cut))
}

/// The entries of a section body: each entry line (one at the body's first
/// line's indentation) with the last line of its continuation.
fn entries<'a, 'b>(body: &'b [L<'a>]) -> Vec<(&'b L<'a>, &'b L<'a>)> {
    let nb: Vec<&L<'a>> = body.iter().filter(|l| !blank(l)).collect();
    let Some(first) = nb.first() else { return Vec::new() };
    let ind = first.ind;
    let mut out = Vec::new();
    for (k, l) in nb.iter().enumerate() {
        if l.ind != ind {
            continue;
        }
        let mut last = *l;
        for next in &nb[k + 1..] {
            if next.ind == ind {
                break;
            }
            last = next;
        }
        out.push((*l, last));
    }
    out
}

/// The type and description of a `Returns` or `Yields` body, and the bytes
/// that remove it: `(type, description, from, to)`.
fn return_body<'a>(body: &[L<'a>], numpy: bool) -> (bool, bool, Option<(usize, usize)>) {
    let nb: Vec<&L<'a>> = body.iter().filter(|l| !blank(l)).collect();
    let Some(first) = nb.first() else { return (false, false, None) };
    let last = nb[nb.len() - 1];
    if numpy {
        let t = first.text.trim();
        let ty = match t.split_once(" : ") {
            Some((_, ty)) => ty.trim(),
            None => t,
        };
        let desc = nb.iter().skip(1).any(|l| l.ind > first.ind);
        (!ty.is_empty(), desc, Some((first.start, last.full)))
    } else {
        let (ty, rest) = split_type(first.text.trim());
        let desc = !rest.is_empty() || nb.len() > 1;
        (ty.is_some(), desc, Some((first.start, last.full)))
    }
}

/// The bytes that remove the `Returns` or `Yields` section whose header is
/// line `h` and which ends before line `after`; when only blank lines
/// follow it, the blank line before it goes too.
fn section_cut(ls: &[L<'_>], h: usize, after: usize, to: usize) -> (usize, usize) {
    let last = ls[after..].iter().all(blank);
    let from = if last && h > 0 && blank(&ls[h - 1]) { ls[h - 1].start } else { ls[h].start };
    (from, to)
}

fn parse_google(ls: &[L<'_>], d: &mut Doc) {
    let mut i = 0;
    while i < ls.len() {
        let Some(kind) = google_header(ls[i].text) else {
            i += 1;
            continue;
        };
        let hind = ls[i].ind;
        let mut e = i + 1;
        while e < ls.len() && (blank(&ls[e]) || ls[e].ind > hind) {
            e += 1;
        }
        let body = &ls[i + 1..e];
        match kind {
            Sec::Params => {
                for (l, last) in entries(body) {
                    if let Some((name, cut)) = google_param(l) {
                        d.params.push(PDoc { name: name.clone(), start: l.start, end: last.full });
                        if let Some(cut) = cut {
                            d.types.push(TDoc { name, cut });
                        }
                    }
                }
            }
            Sec::Raises => {
                for (l, _) in entries(body) {
                    if let Some(i) = l.text.find(':') {
                        exception_names(&l.text[..i], &mut d.raises);
                    }
                }
            }
            Sec::Returns | Sec::Yields => {
                let (ty, desc, range) = return_body(body, false);
                let cut = range.map(|(_, to)| section_cut(ls, i, e, to));
                if kind == Sec::Returns {
                    (d.rtype, d.returns) = (d.rtype || ty, d.returns || desc);
                    d.returns_cut.extend(cut.filter(|_| ty || desc));
                } else {
                    (d.ytype, d.yields) = (d.ytype || ty, d.yields || desc);
                    d.yields_cut.extend(cut.filter(|_| ty || desc));
                }
            }
        }
        i = e;
    }
}

// ------------------------------------------------------------------ NumPy

/// A line over a line of dashes is a NumPy section header; what it is called
/// tells which of the four sections Pylint reads it is, if any.
fn numpy_header(ls: &[L<'_>], i: usize) -> Option<Option<Sec>> {
    if blank(&ls[i]) {
        return None;
    }
    let u = ls.get(i + 1)?.text.trim();
    if u.len() < 3 || !u.chars().all(|c| c == '-') {
        return None;
    }
    Some(match ls[i].text.trim().to_lowercase().as_str() {
        "parameters" | "arguments" | "args" | "other parameters" | "keyword arguments" | "keyword args" | "other args" | "other arguments" => Some(Sec::Params),
        "raises" => Some(Sec::Raises),
        "returns" => Some(Sec::Returns),
        "yields" => Some(Sec::Yields),
        _ => None,
    })
}

fn parse_numpy(ls: &[L<'_>], d: &mut Doc) {
    let heads: Vec<usize> = (0..ls.len()).filter(|&i| numpy_header(ls, i).is_some()).collect();
    for (k, &h) in heads.iter().enumerate() {
        let Some(kind) = numpy_header(ls, h).flatten() else { continue };
        let to = heads.get(k + 1).copied().unwrap_or(ls.len());
        let body = &ls[(h + 2).min(to)..to];
        match kind {
            Sec::Params => {
                for (l, last) in entries(body) {
                    let t = l.text.trim();
                    if t.starts_with("..") {
                        continue;
                    }
                    let (names, ty) = match t.find(':') {
                        Some(i) => (&t[..i], t[i + 1..].trim()),
                        None => (t, ""),
                    };
                    let parts: Vec<&str> = names.split(',').map(str::trim).collect();
                    if parts.iter().any(|p| !is_param_name(p)) {
                        continue;
                    }
                    let cut_from = l.start + l.ind + names.trim_end().len();
                    // A bare name is a mention, not documentation: it needs a type or a description.
                    if ty.is_empty() && last.start == l.start {
                        continue;
                    }
                    for p in parts {
                        d.params.push(PDoc { name: norm(p), start: l.start, end: last.full });
                        if !ty.is_empty() {
                            d.types.push(TDoc { name: norm(p), cut: (cut_from, l.end) });
                        }
                    }
                }
            }
            Sec::Raises => {
                for (l, _) in entries(body) {
                    if !l.text.trim().starts_with("..") {
                        exception_names(l.text, &mut d.raises);
                    }
                }
            }
            Sec::Returns | Sec::Yields => {
                let (ty, desc, range) = return_body(body, true);
                let cut = range.map(|(_, end)| section_cut(ls, h, to, end));
                if kind == Sec::Returns {
                    (d.rtype, d.returns) = (d.rtype || ty, d.returns || desc);
                    d.returns_cut.extend(cut.filter(|_| ty || desc));
                } else {
                    (d.ytype, d.yields) = (d.ytype || ty, d.yields || desc);
                    d.yields_cut.extend(cut.filter(|_| ty || desc));
                }
            }
        }
    }
}

/// The docstring read in the first style that finds a section in it.
fn parse(ds: &Ds<'_>) -> Doc {
    let ls = lines_of(ds.src, ds.body_start, ds.body_end);
    let mut d = Doc { present: true, see: ls.iter().any(|l| see_sentence(l.text)), ..Doc::default() };
    d.style = if ls.iter().any(|l| sphinx_field(l).is_some()) {
        Style::Sphinx
    } else if ls.iter().any(|l| google_header(l.text).is_some()) {
        Style::Google
    } else if (0..ls.len()).any(|i| numpy_header(&ls, i).is_some()) {
        Style::Numpy
    } else {
        Style::Unknown
    };
    match d.style {
        Style::Sphinx => parse_sphinx(&ls, &mut d),
        Style::Google => parse_google(&ls, &mut d),
        Style::Numpy => parse_numpy(&ls, &mut d),
        Style::Unknown => {}
    }
    if let Some(first) = ls.iter().find(|l| !blank(l)) {
        let t = first.text.trim();
        d.prop_returns = !t.starts_with(':') && google_header(t).is_none();
        if d.style != Style::Sphinx
            && let (Some(ty), _) = split_type(t)
        {
            d.prop_type = ty.len() <= 60;
        }
    }
    d
}

// ------------------------------------------------------------ exceptions

const BUILTIN_EXCEPTIONS: &[(&str, &str)] = &[
    ("BaseExceptionGroup", "BaseException"),
    ("GeneratorExit", "BaseException"),
    ("KeyboardInterrupt", "BaseException"),
    ("SystemExit", "BaseException"),
    ("Exception", "BaseException"),
    ("ArithmeticError", "Exception"),
    ("AssertionError", "Exception"),
    ("AttributeError", "Exception"),
    ("BufferError", "Exception"),
    ("EOFError", "Exception"),
    ("ExceptionGroup", "Exception"),
    ("ImportError", "Exception"),
    ("LookupError", "Exception"),
    ("MemoryError", "Exception"),
    ("NameError", "Exception"),
    ("OSError", "Exception"),
    ("ReferenceError", "Exception"),
    ("RuntimeError", "Exception"),
    ("StopAsyncIteration", "Exception"),
    ("StopIteration", "Exception"),
    ("SyntaxError", "Exception"),
    ("SystemError", "Exception"),
    ("TypeError", "Exception"),
    ("ValueError", "Exception"),
    ("Warning", "Exception"),
    ("FloatingPointError", "ArithmeticError"),
    ("OverflowError", "ArithmeticError"),
    ("ZeroDivisionError", "ArithmeticError"),
    ("ModuleNotFoundError", "ImportError"),
    ("IndexError", "LookupError"),
    ("KeyError", "LookupError"),
    ("UnboundLocalError", "NameError"),
    ("BlockingIOError", "OSError"),
    ("ChildProcessError", "OSError"),
    ("ConnectionError", "OSError"),
    ("FileExistsError", "OSError"),
    ("FileNotFoundError", "OSError"),
    ("InterruptedError", "OSError"),
    ("IsADirectoryError", "OSError"),
    ("NotADirectoryError", "OSError"),
    ("PermissionError", "OSError"),
    ("ProcessLookupError", "OSError"),
    ("TimeoutError", "OSError"),
    ("BrokenPipeError", "ConnectionError"),
    ("ConnectionAbortedError", "ConnectionError"),
    ("ConnectionRefusedError", "ConnectionError"),
    ("ConnectionResetError", "ConnectionError"),
    ("NotImplementedError", "RuntimeError"),
    ("RecursionError", "RuntimeError"),
    ("PythonFinalizationError", "RuntimeError"),
    ("IndentationError", "SyntaxError"),
    ("TabError", "IndentationError"),
    ("UnicodeError", "ValueError"),
    ("UnicodeDecodeError", "UnicodeError"),
    ("UnicodeEncodeError", "UnicodeError"),
    ("UnicodeTranslateError", "UnicodeError"),
    ("BytesWarning", "Warning"),
    ("DeprecationWarning", "Warning"),
    ("EncodingWarning", "Warning"),
    ("FutureWarning", "Warning"),
    ("ImportWarning", "Warning"),
    ("PendingDeprecationWarning", "Warning"),
    ("ResourceWarning", "Warning"),
    ("RuntimeWarning", "Warning"),
    ("SyntaxWarning", "Warning"),
    ("UnicodeWarning", "Warning"),
    ("UserWarning", "Warning"),
];

/// Names that are the same class as another.
fn canon(n: &str) -> &str {
    match n {
        "IOError" | "EnvironmentError" | "WindowsError" => "OSError",
        other => other,
    }
}

fn is_builtin_exception(n: &str) -> bool {
    n == "BaseException" || BUILTIN_EXCEPTIONS.iter().any(|(c, _)| *c == canon(n)) || canon(n) == "OSError"
}

type Classes = HashMap<String, Vec<String>>;

/// Every class of the file, with the short names of its bases.
fn local_classes(root: Node<'_>, src: &str) -> Classes {
    let mut out = Classes::new();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "class_definition"
            && let Some(name) = n.child_by_field_name("name")
        {
            let bases: Vec<String> = n
                .child_by_field_name("superclasses")
                .map(|s| {
                    named_children(s)
                        .into_iter()
                        .filter(|b| matches!(b.kind(), "identifier" | "attribute"))
                        .map(|b| text(b, src).rsplit('.').next().unwrap_or("").to_owned())
                        .collect()
                })
                .unwrap_or_default();
            out.insert(text(name, src).to_owned(), bases);
        }
        stack.extend(named_children(n));
    }
    out
}

/// `name` and every class it derives from, as far as the file and the
/// built-in exceptions say.
fn lineage(name: &str, classes: &Classes) -> BTreeSet<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut todo = vec![canon(name).to_owned()];
    while let Some(n) = todo.pop() {
        if !seen.insert(n.clone()) {
            continue;
        }
        if let Some(bases) = classes.get(&n) {
            todo.extend(bases.iter().map(|b| canon(b).to_owned()));
        } else if let Some((_, parent)) = BUILTIN_EXCEPTIONS.iter().find(|(c, _)| *c == n) {
            todo.push((*parent).to_owned());
        }
    }
    seen
}

/// The short name of the class an expression names, when the file, the
/// built-ins or the standard library say which class it is. Anything else —
/// a package the checker cannot see — is not read.
fn exc_short(n: Node<'_>, ctx: &NativeContext<'_>) -> Option<String> {
    let q = exc_name(n, ctx)?;
    if let Some(rest) = q.strip_prefix("builtins.") {
        return is_builtin_exception(rest).then(|| canon(rest).to_owned());
    }
    match q.split_once('.') {
        Some((first, _)) => {
            let std = ctx.data.get("python/stdlib").and_then(|v| v.as_array()).is_some_and(|a| a.iter().any(|v| v.as_str() == Some(first)));
            std.then(|| q.rsplit('.').next().unwrap_or("").to_owned())
        }
        None => Some(q),
    }
}

// ------------------------------------------------------------------ scan

#[derive(Default)]
struct Scan {
    value_return: bool,
    yields: bool,
    raised: BTreeSet<String>,
}

struct Walk<'a, 'b> {
    ctx: &'b NativeContext<'a>,
    def: Node<'a>,
    classes: &'b Classes,
    raises: bool,
    out: Scan,
    /// The types of the `except` clause the walk is in; `None` for a bare one.
    handler: Option<Vec<String>>,
    aliases: HashMap<String, Vec<String>>,
    /// Names the function binds to an exception instance: `err = ValueError(...)`.
    locals: HashMap<String, String>,
}

fn handler_types(clause: Node<'_>, ctx: &NativeContext<'_>) -> (Option<Vec<String>>, Option<String>) {
    let first = named_children(clause).into_iter().find(|c| c.kind() != "block");
    let (ty, alias) = match first {
        Some(c) if c.kind() == "as_pattern" => (c.named_child(0), c.child_by_field_name("alias").or_else(|| c.named_child(1))),
        other => (other, None),
    };
    let Some(ty) = ty else { return (None, None) };
    let items: Vec<Node<'_>> = if matches!(ty.kind(), "tuple" | "parenthesized_expression") { kids(ty) } else { vec![ty] };
    let names = items.into_iter().filter_map(|i| exc_short(i, ctx)).collect();
    (Some(names), alias.map(|a| text(a, ctx.source).trim().to_owned()))
}

impl<'a, 'b> Walk<'a, 'b> {
    /// Whether an enclosing `try` of this function handles `exc`.
    fn caught(&self, raise: Node<'a>, exc: &str) -> bool {
        let lineage = lineage(exc, self.classes);
        let mut child = raise;
        let mut p = raise.parent();
        while let Some(n) = p {
            if n.id() == self.def.id() {
                break;
            }
            if n.kind() == "try_statement" && n.child_by_field_name("body").is_some_and(|b| b.id() == child.id()) {
                for c in children(n).into_iter().filter(|c| c.kind() == "except_clause") {
                    match handler_types(c, self.ctx).0 {
                        None => return true,
                        Some(names) => {
                            if names.iter().any(|h| lineage.contains(canon(h)) || h == "BaseException") {
                                return true;
                            }
                        }
                    }
                }
            }
            child = n;
            p = n.parent();
        }
        false
    }

    fn add(&mut self, raise: Node<'a>, name: String) {
        if name != "NotImplementedError" && !self.caught(raise, &name) {
            self.out.raised.insert(name);
        }
    }

    fn walk(&mut self, n: Node<'a>) {
        match n.kind() {
            "function_definition" | "class_definition" | "lambda" => return,
            "except_clause" => {
                let saved = (self.handler.take(), std::mem::take(&mut self.aliases));
                let (types, alias) = handler_types(n, self.ctx);
                if let (Some(t), Some(a)) = (&types, alias) {
                    self.aliases.insert(a, t.clone());
                }
                self.handler = types;
                for c in named_children(n) {
                    self.walk(c);
                }
                (self.handler, self.aliases) = saved;
                return;
            }
            "raise_statement" if self.raises => {
                let exc = n.named_child(0).filter(|c| Some(*c) != n.child_by_field_name("cause"));
                match exc {
                    Some(e) => {
                        let callee = if e.kind() == "call" { e.child_by_field_name("function").unwrap_or(e) } else { e };
                        if let Some(name) = exc_short(callee, self.ctx) {
                            self.add(n, name);
                        } else if e.kind() == "identifier" {
                            let var = text(e, self.ctx.source);
                            if let Some(t) = self.aliases.get(var).cloned() {
                                for name in t {
                                    self.add(n, name);
                                }
                            } else if let Some(name) = self.locals.get(var).cloned() {
                                self.add(n, name);
                            }
                        }
                    }
                    None => {
                        for name in self.handler.clone().unwrap_or_default() {
                            self.add(n, name);
                        }
                    }
                }
            }
            "assignment" if self.raises => {
                if let (Some(l), Some(r)) = (n.child_by_field_name("left"), n.child_by_field_name("right"))
                    && l.kind() == "identifier"
                    && r.kind() == "call"
                    && let Some(name) = r.child_by_field_name("function").and_then(|f| exc_short(f, self.ctx))
                {
                    self.locals.insert(text(l, self.ctx.source).to_owned(), name);
                }
            }
            "return_statement" => {
                if n.named_child(0).is_some_and(|v| v.kind() != "none") {
                    self.out.value_return = true;
                }
            }
            "yield" => self.out.yields = true,
            _ => {}
        }
        for c in named_children(n) {
            self.walk(c);
        }
    }
}

fn scan<'a>(def: Node<'a>, ctx: &NativeContext<'a>, classes: &Classes, raises: bool) -> Scan {
    let mut w = Walk { ctx, def, classes, raises, out: Scan::default(), handler: None, aliases: HashMap::new(), locals: HashMap::new() };
    if let Some(b) = def.child_by_field_name("body") {
        w.walk(b);
    }
    w.out
}

// --------------------------------------------------------------- analysis

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Param,
    Type,
    Any,
    DifferParam,
    DifferType,
    UselessParam,
    UselessType,
    Multiple,
    Raises,
    Ret,
    RetType,
    Yield,
    YieldType,
    RedundantRet,
    RedundantYield,
}

struct Opts {
    accept_param: bool,
    accept_raise: bool,
    accept_return: bool,
    accept_yields: bool,
    check_private: bool,
}

impl Opts {
    fn read(ctx: &NativeContext<'_>) -> Opts {
        let b = |k: &str, d: bool| ctx.options.get(k).and_then(|v| v.as_bool()).unwrap_or(d);
        Opts {
            accept_param: b("accept_no_param_doc", true),
            accept_raise: b("accept_no_raise_doc", true),
            accept_return: b("accept_no_return_doc", true),
            accept_yields: b("accept_no_yields_doc", true),
            check_private: b("check_private_functions", false),
        }
    }
}

fn finding(at: Node<'_>, message: String, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: at.start_byte(), end: at.end_byte(), message, edits }
}

fn class_of(def: Node<'_>) -> Option<Node<'_>> {
    let mut p = def.parent()?;
    if p.kind() == "decorated_definition" {
        p = p.parent()?;
    }
    if p.kind() != "block" {
        return None;
    }
    let c = p.parent()?;
    (c.kind() == "class_definition").then_some(c)
}

fn is_ignored(key: &str) -> bool {
    let n = key.trim_start_matches('*');
    n.starts_with('_') || n.starts_with("ignored_") || n.starts_with("unused_")
}

/// What `# type:` comments say about a function: that it has a return type,
/// and which parameters have a type (`# type: (int, str) -> int` after the
/// signature, or a `# type: int` after each parameter).
#[derive(Default)]
struct TComm {
    ret: bool,
    all: bool,
    typed: BTreeSet<String>,
}

fn param_name<'a>(c: Node<'a>, src: &'a str) -> Option<&'a str> {
    let n = match c.kind() {
        "identifier" => Some(c),
        "typed_parameter" => c.named_child(0).and_then(|i| if i.kind() == "identifier" { Some(i) } else { i.named_child(0) }),
        "default_parameter" | "typed_default_parameter" => c.child_by_field_name("name"),
        "list_splat_pattern" | "dictionary_splat_pattern" => c.named_child(0),
        _ => None,
    }?;
    Some(text(n, src))
}

fn type_comments(def: Node<'_>, src: &str) -> TComm {
    let mut tc = TComm::default();
    let (Some(ps), Some(body)) = (def.child_by_field_name("parameters"), def.child_by_field_name("body")) else { return tc };
    let from = def.child_by_field_name("return_type").map_or(ps.end_byte(), |r| r.end_byte());
    let to = statements(body).first().map_or(body.end_byte(), |s| s.start_byte());
    if from < to
        && let Some(i) = src[from..to].find("# type:")
    {
        let line = src[from + i + 7..].lines().next().unwrap_or("");
        tc.ret = line.contains("->");
        let args = line.split("->").next().unwrap_or("").trim();
        tc.all = args.starts_with('(') && args != "(...)";
    }
    for c in kids(ps) {
        let rest = &src[c.end_byte()..];
        let line = rest.lines().next().unwrap_or("");
        if line.contains("# type:")
            && let Some(n) = param_name(c, src)
        {
            tc.typed.insert(n.to_owned());
        }
    }
    tc
}

fn list(names: &BTreeSet<String>) -> String {
    names.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ")
}

/// A documented `name` stands for the expected `key`: `args` for `*args`.
fn same(key: &str, name: &str) -> bool {
    key == name || key.trim_start_matches('*') == name
}

/// Pylint's reading of one docstring against one parameter list.
fn check_args(doc: &Doc, def: Node<'_>, at: Node<'_>, accept: bool, kind: Kind, ctx: &NativeContext<'_>) -> Vec<NativeFinding> {
    let mut out = Vec::new();
    let params = params_of(def, ctx.source);
    let mut expected: BTreeSet<String> = BTreeSet::new();
    let mut annotated: BTreeSet<String> = BTreeSet::new();
    let mut not_type: BTreeSet<String> = ["self", "cls"].iter().map(|s| (*s).to_owned()).collect();
    let not_param: BTreeSet<String> = not_type.clone();
    for p in &params {
        let key = format!("{}{}", "*".repeat(p.stars), p.name);
        if p.annotation.is_some() {
            annotated.insert(key.clone());
        }
        if p.stars > 0 {
            not_type.insert(key.clone());
        }
        expected.insert(key);
    }
    let ignored: BTreeSet<String> = expected.iter().filter(|k| is_ignored(k)).cloned().collect();
    let documented: BTreeSet<String> = expected.iter().filter(|k| doc.params.iter().any(|p| same(k, &p.name))).cloned().collect();
    let typed_in_doc: BTreeSet<String> = expected.iter().filter(|k| doc.types.iter().any(|t| same(k, &t.name))).cloned().collect();
    let tc = type_comments(def, ctx.source);
    let commented: BTreeSet<String> = expected.iter().filter(|k| tc.all || tc.typed.contains(k.trim_start_matches('*'))).cloned().collect();
    let typed: BTreeSet<String> = typed_in_doc.union(&annotated).chain(commented.iter()).cloned().collect();
    let known = |n: &str| expected.iter().any(|k| same(k, n));
    let tolerate = doc.see || (!doc.has_params() && accept);
    if !tolerate {
        let no_param: BTreeSet<String> = not_param.union(&ignored).cloned().collect();
        let no_type: BTreeSet<String> = not_type.union(&ignored).cloned().collect();
        let missing_param: BTreeSet<String> = expected.iter().filter(|k| !documented.contains(*k) && !no_param.contains(*k)).cloned().collect();
        let missing_type: BTreeSet<String> = expected.iter().filter(|k| !typed.contains(*k) && !no_type.contains(*k)).cloned().collect();
        if missing_param == expected && expected == missing_type && !expected.is_empty() {
            if kind == Kind::Any {
                out.push(finding(at, "No parameter of the function is documented".to_owned(), Vec::new()));
            }
        } else {
            if kind == Kind::Param && !missing_param.is_empty() {
                out.push(finding(at, format!("Parameters {} are not documented in the docstring", list(&missing_param)), Vec::new()));
            }
            if kind == Kind::Type && !missing_type.is_empty() {
                out.push(finding(at, format!("Parameters {} have no type in the docstring or the signature", list(&missing_type)), Vec::new()));
            }
        }
    }
    if kind == Kind::DifferParam {
        let differing: BTreeSet<String> = doc.params.iter().map(|p| p.name.clone()).filter(|n| !known(n) && !not_param.contains(n)).collect();
        if !differing.is_empty() {
            out.push(finding(at, format!("The docstring documents {}, which the signature does not have", list(&differing)), Vec::new()));
        }
    }
    if kind == Kind::DifferType {
        let differing: BTreeSet<String> = doc.types.iter().map(|p| p.name.clone()).filter(|n| !known(n) && !not_type.contains(n)).collect();
        if !differing.is_empty() {
            out.push(finding(at, format!("The docstring types {}, which the signature does not have", list(&differing)), Vec::new()));
        }
    }
    if kind == Kind::UselessParam {
        let useless: BTreeSet<String> = ignored.intersection(&documented).cloned().collect();
        if !useless.is_empty() {
            let edits = doc.params.iter().filter(|p| useless.iter().any(|k| same(k, &p.name))).map(|p| TextEdit::new(p.start, p.end, "")).collect();
            out.push(finding(at, format!("Parameters {} are ignored by name and need no description", list(&useless)), edits));
        }
    }
    if kind == Kind::UselessType {
        let useless: BTreeSet<String> = ignored.intersection(&typed_in_doc).cloned().collect();
        if !useless.is_empty() {
            let edits = doc.types.iter().filter(|t| useless.iter().any(|k| same(k, &t.name))).map(|t| TextEdit::new(t.cut.0, t.cut.1, "")).collect();
            out.push(finding(at, format!("Parameters {} are ignored by name and need no type", list(&useless)), edits));
        }
    }
    out
}

fn function(def: Node<'_>, ctx: &NativeContext<'_>, kind: Kind) -> Vec<NativeFinding> {
    let src = ctx.source;
    let Some(name_node) = def.child_by_field_name("name") else { return Vec::new() };
    let name = text(name_node, src);
    let o = Opts::read(ctx);
    if name.starts_with('_') && !o.check_private {
        return Vec::new();
    }
    if crate::py_ruff_doc::decorator_names(def, src).iter().any(|d| d == "overload" || d.ends_with(".overload")) {
        return Vec::new();
    }
    let ds = docstring_of(def, src);
    let doc = ds.as_ref().map(parse).unwrap_or_default();
    let annotated_return = def.child_by_field_name("return_type").is_some() || type_comments(def, src).ret;
    let no_classes = Classes::new();
    match kind {
        Kind::Param | Kind::Type | Kind::Any | Kind::DifferParam | Kind::DifferType | Kind::UselessParam | Kind::UselessType => {
            let mut out = Vec::new();
            let mut accept = o.accept_param;
            if matches!(name, "__init__" | "__new__")
                && let Some(cls) = class_of(def)
                && let Some(cname) = cls.child_by_field_name("name")
            {
                let cdoc = docstring_of(cls, src).as_ref().map(parse).unwrap_or_default();
                if cdoc.has_params() || cdoc.see {
                    accept = true;
                }
                if cdoc.present {
                    let class_accept = if doc.has_params() || doc.see { true } else { o.accept_param };
                    out.extend(check_args(&cdoc, def, cname, class_accept, kind, ctx));
                }
            }
            if doc.present {
                out.extend(check_args(&doc, def, name_node, accept, kind, ctx));
            }
            out
        }
        Kind::Raises => {
            let mut at = name_node;
            let mut doc = doc;
            if !doc.present {
                // A property setter has no docstring of its own: the getter's is read.
                let Some(getter) = property_getter(def, src) else { return Vec::new() };
                let Some(gds) = docstring_of(getter, src) else { return Vec::new() };
                doc = parse(&gds);
                let Some(gname) = getter.child_by_field_name("name") else { return Vec::new() };
                at = gname;
            }
            if o.accept_raise && doc.raises.is_empty() {
                return Vec::new();
            }
            let classes = local_classes(root_of(def), src);
            let sc = scan(def, ctx, &classes, true);
            let missing: BTreeSet<String> = sc
                .raised
                .iter()
                .filter(|e| {
                    let l = lineage(e, &classes);
                    !doc.raises.iter().any(|d| l.contains(canon(d.rsplit('.').next().unwrap_or(d))) || d.rsplit('.').next() == Some("BaseException"))
                })
                .cloned()
                .collect();
            if missing.is_empty() {
                return Vec::new();
            }
            vec![finding(at, format!("Exceptions {} are raised but not documented", list(&missing)), Vec::new())]
        }
        Kind::Ret | Kind::RetType => {
            if o.accept_return {
                return Vec::new();
            }
            let sc = scan(def, ctx, &no_classes, false);
            if !sc.value_return {
                return Vec::new();
            }
            let prop = is_property(def, src);
            if kind == Kind::Ret && !(doc.returns || (doc.prop_returns && prop)) {
                return vec![finding(name_node, "The value the function returns is not documented".to_owned(), Vec::new())];
            }
            if kind == Kind::RetType && !annotated_return && !(doc.rtype || (doc.prop_type && prop)) {
                return vec![finding(name_node, "The type of the value the function returns is not documented".to_owned(), Vec::new())];
            }
            Vec::new()
        }
        Kind::Yield | Kind::YieldType => {
            if o.accept_yields {
                return Vec::new();
            }
            let sc = scan(def, ctx, &no_classes, false);
            if !sc.yields {
                return Vec::new();
            }
            let (has, has_type) = if doc.supports_yields() { (doc.yields, doc.ytype) } else { (doc.returns, doc.rtype) };
            if kind == Kind::Yield && !has {
                return vec![finding(name_node, "The value the generator yields is not documented".to_owned(), Vec::new())];
            }
            if kind == Kind::YieldType && !annotated_return && !has_type {
                return vec![finding(name_node, "The type of the value the generator yields is not documented".to_owned(), Vec::new())];
            }
            Vec::new()
        }
        Kind::RedundantRet | Kind::RedundantYield => {
            if !doc.present || is_abstract(def, src) || is_stub_fn(def, src) {
                return Vec::new();
            }
            let sc = scan(def, ctx, &no_classes, false);
            let del = |cuts: &[(usize, usize)]| cuts.iter().map(|c| TextEdit::new(c.0, c.1, "")).collect::<Vec<_>>();
            if kind == Kind::RedundantRet {
                if (doc.returns || doc.rtype) && !sc.value_return && !(sc.yields && !doc.supports_yields()) {
                    return vec![finding(name_node, "The docstring documents a returned value the function does not return".to_owned(), del(&doc.returns_cut))];
                }
            } else if doc.supports_yields() && (doc.yields || doc.ytype) && !sc.yields {
                return vec![finding(name_node, "The docstring documents a yielded value the function does not yield".to_owned(), del(&doc.yields_cut))];
            }
            Vec::new()
        }
        Kind::Multiple => Vec::new(),
    }
}

fn root_of(n: Node<'_>) -> Node<'_> {
    let mut r = n;
    while let Some(p) = r.parent() {
        r = p;
    }
    r
}

/// The getter a `@x.setter` function belongs to.
fn property_getter<'a>(def: Node<'a>, src: &str) -> Option<Node<'a>> {
    let name = text(def.child_by_field_name("name")?, src);
    let class = class_of(def)?;
    let body = class.child_by_field_name("body")?;
    for s in statements(body) {
        let f = if s.kind() == "decorated_definition" { s.child_by_field_name("definition")? } else { s };
        if f.kind() == "function_definition"
            && f.id() != def.id()
            && f.child_by_field_name("name").is_some_and(|n| text(n, src) == name)
            && crate::py_ruff_doc::decorator_names(f, src).iter().any(|d| d == "property" || d.ends_with(".cached_property") || d == "cached_property")
        {
            return Some(f);
        }
    }
    None
}

/// A class whose docstring and constructor's docstring both document the
/// constructor's parameters.
fn multiple(cls: Node<'_>, ctx: &NativeContext<'_>) -> Vec<NativeFinding> {
    let src = ctx.source;
    let (Some(name), Some(body)) = (cls.child_by_field_name("name"), cls.child_by_field_name("body")) else { return Vec::new() };
    let cdoc = docstring_of(cls, src).as_ref().map(parse).unwrap_or_default();
    if !cdoc.has_params() {
        return Vec::new();
    }
    for s in statements(body) {
        let f = if s.kind() == "decorated_definition" { s.child_by_field_name("definition").unwrap_or(s) } else { s };
        if f.kind() == "function_definition"
            && f.child_by_field_name("name").is_some_and(|n| matches!(text(n, src), "__init__" | "__new__"))
            && docstring_of(f, src).as_ref().map(parse).is_some_and(|d| d.has_params())
        {
            return vec![finding(name, "The class and its constructor both document the constructor's parameters".to_owned(), Vec::new())];
        }
    }
    Vec::new()
}

fn analyze(node: Node<'_>, ctx: &NativeContext<'_>, kind: Kind) -> Vec<NativeFinding> {
    match node.kind() {
        "class_definition" if kind == Kind::Multiple => multiple(node, ctx),
        "function_definition" => function(node, ctx, kind),
        _ => Vec::new(),
    }
}

macro_rules! doc_rule {
    ($name:ident, $id:literal, $kind:expr, $node:literal, $sem:literal) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                PY
            }
            fn kinds(&self) -> &'static [&'static str] {
                &[$node]
            }
            fn needs_semantic(&self) -> bool {
                $sem
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
                out.extend(analyze(node.get_inner_node(), ctx, $kind));
            }
        }
    };
}

doc_rule!(MissingParamDoc, "py/missing-param-doc", Kind::Param, "function_definition", false);
doc_rule!(MissingTypeDoc, "py/missing-type-doc", Kind::Type, "function_definition", false);
doc_rule!(MissingAnyParamDoc, "py/missing-any-param-doc", Kind::Any, "function_definition", false);
doc_rule!(DifferingParamDoc, "py/differing-param-doc", Kind::DifferParam, "function_definition", false);
doc_rule!(DifferingTypeDoc, "py/differing-type-doc", Kind::DifferType, "function_definition", false);
doc_rule!(UselessParamDoc, "py/useless-param-doc", Kind::UselessParam, "function_definition", false);
doc_rule!(UselessTypeDoc, "py/useless-type-doc", Kind::UselessType, "function_definition", false);
doc_rule!(MultipleConstructorDoc, "py/multiple-constructor-doc", Kind::Multiple, "class_definition", false);
doc_rule!(MissingRaisesDoc, "py/missing-raises-doc", Kind::Raises, "function_definition", true);
doc_rule!(MissingReturnDoc, "py/missing-return-doc", Kind::Ret, "function_definition", false);
doc_rule!(MissingReturnTypeDoc, "py/missing-return-type-doc", Kind::RetType, "function_definition", false);
doc_rule!(MissingYieldDoc, "py/missing-yield-doc", Kind::Yield, "function_definition", false);
doc_rule!(MissingYieldTypeDoc, "py/missing-yield-type-doc", Kind::YieldType, "function_definition", false);
doc_rule!(RedundantReturnsDoc, "py/redundant-returns-doc", Kind::RedundantRet, "function_definition", false);
doc_rule!(RedundantYieldsDoc, "py/redundant-yields-doc", Kind::RedundantYield, "function_definition", false);

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &MissingParamDoc,
    &MissingTypeDoc,
    &MissingAnyParamDoc,
    &DifferingParamDoc,
    &DifferingTypeDoc,
    &UselessParamDoc,
    &UselessTypeDoc,
    &MultipleConstructorDoc,
    &MissingRaisesDoc,
    &MissingReturnDoc,
    &MissingReturnTypeDoc,
    &MissingYieldDoc,
    &MissingYieldTypeDoc,
    &RedundantReturnsDoc,
    &RedundantYieldsDoc,
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn run(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        crate::run(rule, Language::Python, src)
    }

    fn strict(rule: &dyn NativeRule, src: &str) -> Vec<NativeFinding> {
        let mut o = BTreeMap::new();
        for k in ["accept_no_param_doc", "accept_no_raise_doc", "accept_no_return_doc", "accept_no_yields_doc"] {
            o.insert(k.to_owned(), serde_json::Value::Bool(false));
        }
        crate::run_with(rule, Language::Python, src, o)
    }

    #[test]
    fn sphinx_params() {
        let src = "def f(x, y, z):\n    \"\"\"Do it.\n\n    :param x: bla\n    :param int z: bar\n    \"\"\"\n";
        assert_eq!(run(&MissingParamDoc, src).len(), 1);
        assert_eq!(run(&MissingTypeDoc, src).len(), 1);
        assert!(run(&DifferingParamDoc, src).is_empty());
        let none = "def f(x, y):\n    \"\"\"Do it.\"\"\"\n";
        assert!(run(&MissingParamDoc, none).is_empty());
        assert_eq!(strict(&MissingAnyParamDoc, none).len(), 1);
    }

    #[test]
    fn google_and_numpy_params() {
        let g = "def f(x, y: int):\n    \"\"\"Do it.\n\n    Args:\n        x (int): bla\n        z: bar\n    \"\"\"\n";
        assert_eq!(run(&MissingParamDoc, g).len(), 1);
        assert_eq!(run(&DifferingParamDoc, g).len(), 1);
        let n = "def f(x, y):\n    \"\"\"Do it.\n\n    Parameters\n    ----------\n    x : int\n        bla\n    \"\"\"\n";
        assert_eq!(run(&MissingParamDoc, n).len(), 1);
        assert_eq!(run(&MissingTypeDoc, n).len(), 1);
    }

    #[test]
    fn see_sentence_and_ignored_names() {
        let src = "def f(x, y):\n    \"\"\"Do it.\n\n    :param x: bla\n\n    For the other parameters, see :func:`g`.\n    \"\"\"\n";
        assert!(run(&MissingParamDoc, src).is_empty());
        let u = "def f(a, _b):\n    \"\"\"Do it.\n\n    Args:\n        a (int): bla\n        _b (str): ignored\n    \"\"\"\n";
        assert_eq!(run(&UselessParamDoc, u).len(), 1);
        assert_eq!(run(&UselessTypeDoc, u).len(), 1);
        assert_eq!(run(&UselessTypeDoc, u)[0].edits.len(), 1);
    }

    #[test]
    fn returns_and_yields() {
        let src = "def f():\n    \"\"\"Do it.\"\"\"\n    return 1\n";
        assert!(run(&MissingReturnDoc, src).is_empty());
        assert_eq!(strict(&MissingReturnDoc, src).len(), 1);
        assert_eq!(strict(&MissingReturnTypeDoc, src).len(), 1);
        let ok = "def f() -> int:\n    \"\"\"Do it.\n\n    Returns:\n        One.\n    \"\"\"\n    return 1\n";
        assert!(strict(&MissingReturnDoc, ok).is_empty());
        assert!(strict(&MissingReturnTypeDoc, ok).is_empty());
        let y = "def g():\n    \"\"\"Do it.\"\"\"\n    yield 1\n";
        assert_eq!(strict(&MissingYieldDoc, y).len(), 1);
        assert_eq!(strict(&MissingYieldTypeDoc, y).len(), 1);
    }

    #[test]
    fn redundant_docs() {
        let r = "def f():\n    \"\"\"Do it.\n\n    Returns:\n        int: One\n    \"\"\"\n    return None\n";
        assert_eq!(run(&RedundantReturnsDoc, r).len(), 1);
        let gen_sphinx = "def f():\n    \"\"\"Do it.\n\n    :returns: One\n    :rtype: int\n    \"\"\"\n    yield 1\n";
        assert!(run(&RedundantReturnsDoc, gen_sphinx).is_empty());
        let y = "def f():\n    \"\"\"Do it.\n\n    Yields:\n        int: One\n    \"\"\"\n    return 1\n";
        assert_eq!(run(&RedundantYieldsDoc, y).len(), 1);
    }

    #[test]
    fn raises() {
        let src = "def f():\n    \"\"\"Do it.\n\n    Raises:\n        NameError: Never\n    \"\"\"\n    raise RuntimeError('x')\n";
        assert_eq!(run(&MissingRaisesDoc, src).len(), 1);
        let covered = "def f():\n    \"\"\"Do it.\n\n    Raises:\n        Exception: Never\n    \"\"\"\n    raise ValueError('x')\n";
        assert!(run(&MissingRaisesDoc, covered).is_empty());
        let local = "class E(NameError):\n    pass\n\n\ndef f():\n    \"\"\"Do it.\n\n    Raises:\n        NameError: Never\n    \"\"\"\n    raise E()\n";
        assert!(run(&MissingRaisesDoc, local).is_empty());
        let caught = "def f():\n    \"\"\"Do it.\n\n    Raises:\n        NameError: Never\n    \"\"\"\n    try:\n        raise RuntimeError('x')\n    except RuntimeError:\n        pass\n    raise NameError('y')\n";
        assert!(run(&MissingRaisesDoc, caught).is_empty());
        let nodoc = "def f():\n    \"\"\"Do it.\"\"\"\n    raise RuntimeError('x')\n";
        assert!(run(&MissingRaisesDoc, nodoc).is_empty());
        assert_eq!(strict(&MissingRaisesDoc, nodoc).len(), 1);
        let rethrown = "def f():\n    \"\"\"Do it.\n\n    Raises:\n        NameError: Never\n    \"\"\"\n    try:\n        g()\n    except RuntimeError:\n        raise\n";
        assert_eq!(run(&MissingRaisesDoc, rethrown).len(), 1);
    }

    #[test]
    fn constructor_docs() {
        let src = "class C:\n    \"\"\"Class.\n\n    Args:\n        value: one.\n    \"\"\"\n\n    def __init__(self, value):\n        \"\"\"Init.\n\n        Args:\n            value: one.\n        \"\"\"\n";
        assert_eq!(run(&MultipleConstructorDoc, src).len(), 1);
        let module_level = "def __init__(value):\n    \"\"\"Not a constructor.\n\n    Args:\n        value: one.\n    \"\"\"\n";
        assert!(run(&MultipleConstructorDoc, module_level).is_empty());
    }

    /// Pylint's own functional tests, run over every rule here (set `PYLINT_FT`
    /// to the `tests/functional/ext/docparams` folder; not part of the suite).
    #[test]
    #[ignore]
    fn functional_tests() {
        let Ok(root) = std::env::var("PYLINT_FT") else { return };
        let stdlib: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(std::env::var("PYLINT_STDLIB").unwrap()).unwrap()).unwrap();
        let mut data = BTreeMap::new();
        data.insert("python/stdlib".to_owned(), std::sync::Arc::new(stdlib));
        let mut files = Vec::new();
        let mut stack = vec![std::path::PathBuf::from(&root)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(d).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().is_some_and(|x| x == "py") {
                    files.push(p);
                }
            }
        }
        files.sort();
        let ids: Vec<&str> = RULES.iter().map(|r| r.id().trim_start_matches("py/")).collect();
        let mut total_bad = 0;
        for f in files {
            let src = std::fs::read_to_string(&f).unwrap();
            let rc = std::fs::read_to_string(f.with_extension("rc")).unwrap_or_default();
            // Options this module does not have: `docstring-min-length`, and a `no-docstring-rgx` other than the default, `^$` and `^_`-less ones.
            if rc.contains("docstring-min-length=3") || rc.contains("no-docstring-rgx=\n") || rc.contains("(?!__init__$)") || rc.ends_with("no-docstring-rgx=") {
                continue;
            }
            let mut options = BTreeMap::new();
            for line in rc.lines() {
                let line = line.replace(' ', "");
                if let Some((k, v)) = line.split_once('=').or_else(|| line.split_once(':'))
                    && k.starts_with("accept-no-")
                {
                    options.insert(k.replace('-', "_"), serde_json::Value::Bool(v == "yes"));
                }
                if let Some((k, v)) = line.split_once('=')
                    && k == "no-docstring-rgx"
                    && (v == "^$" || v.contains("ONLYVERY"))
                {
                    options.insert("check_private_functions".to_owned(), serde_json::Value::Bool(true));
                }
            }
            let mut want: BTreeSet<(String, usize)> = BTreeSet::new();
            if let Ok(t) = std::fs::read_to_string(f.with_extension("txt")) {
                for l in t.lines() {
                    let mut it = l.split(':');
                    let (Some(id), Some(line)) = (it.next(), it.next()) else { continue };
                    if ids.contains(&id) {
                        want.insert((id.to_owned(), line.parse().unwrap()));
                    }
                }
            }
            let mut got: BTreeSet<(String, usize)> = BTreeSet::new();
            let root_node = codelaw_lang::parse(Language::Python, &src);
            let model = codelaw_semantic::build(Language::Python, &root_node, &src);
            let file = crate::DetachedFile::new("t.py", &src);
            let ctx = NativeContext { source: &src, language: Language::Python, semantic: model.as_ref(), options: &options, data: &data, file: &file };
            for r in RULES {
                let mut out = Vec::new();
                for node in root_node.root().dfs() {
                    if r.kinds().contains(&node.kind().as_ref()) {
                        r.check(node, &ctx, &mut out);
                    }
                }
                for o in out {
                    let line = src[..o.start].matches('\n').count() + 1;
                    got.insert((r.id().trim_start_matches("py/").to_owned(), line));
                }
            }
            if got != want {
                total_bad += 1;
                println!("== {}", f.display());
                for x in want.difference(&got) {
                    println!("   MISSING {} @ {}", x.0, x.1);
                }
                for x in got.difference(&want) {
                    println!("   EXTRA   {} @ {}", x.0, x.1);
                }
            }
        }
        println!("files differing: {total_bad}");
    }
}

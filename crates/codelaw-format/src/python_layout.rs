//! The blank-line rules of Ruff's docstring group that the Ruff formatter
//! leaves to a linter (SR-FORMATTER decision 10, W-179).
//!
//! Ruff's formatter already writes the layout `D201` (no blank line before a
//! function docstring), `D211` (none before a class docstring; `D203`, its
//! opposite, is never produced), `D204` (one blank line after a class
//! docstring), `D210` (no space around a docstring's text) and `EXE004` (no
//! whitespace before a shebang) ask for, with Black's own output. Three
//! cases are left, and this pass writes them on its output:
//!
//! - **`D202`**: a blank line between a function's docstring and the first
//!   statement after it is removed. One blank line stays before a nested
//!   function or class, which the formatter itself writes there, so the two
//!   never disagree (Ruff's own `D202` allows it too, for `def` and `class`
//!   written right after the blank line).
//! - **`D201` and `D211` after a comment**: `def f():` / `# note` / blank /
//!   docstring keeps its blank line in Ruff's formatter; here the blank
//!   lines right before the docstring go.
//!
//! The pass only removes blank lines, so it cannot change a token, and it
//! leaves a `# fmt: off` ... `# fmt: on` region and a line ending in
//! `# fmt: skip` as written.

use ruff_python_ast::statement_visitor::{StatementVisitor, walk_stmt};
use ruff_python_ast::{Stmt, helpers::is_docstring_stmt};
use ruff_text_size::Ranged;
use std::ops::Range;

/// `text` with the blank lines of `D202`, `D201` and `D211` taken out.
pub(crate) fn apply(text: &str) -> String {
    if !text.contains("\n\n") && !text.contains("\n\r\n") {
        return text.to_owned();
    }
    let Ok(parsed) = ruff_python_parser::parse_module(text) else { return text.to_owned() };
    let lines = Lines::new(text);
    let mut found = Found { lines: &lines, remove: Vec::new() };
    found.visit_body(&parsed.syntax().body);
    if found.remove.is_empty() {
        return text.to_owned();
    }
    found.remove.sort_by_key(|r| r.start);
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for r in found.remove {
        if r.start < at {
            continue;
        }
        out.push_str(&text[at..r.start]);
        at = r.end;
    }
    out.push_str(&text[at..]);
    out
}

/// The lines of a text: where each starts and whether it is blank, and the
/// lines a `# fmt: off` region or `# fmt: skip` keeps as written.
struct Lines<'t> {
    text: &'t str,
    starts: Vec<usize>,
    blank: Vec<bool>,
    kept: Vec<bool>,
    len: usize,
}

impl<'t> Lines<'t> {
    fn new(text: &'t str) -> Self {
        let mut starts = Vec::new();
        let mut blank = Vec::new();
        let mut kept = Vec::new();
        let mut off = false;
        let mut at = 0;
        for line in text.split_inclusive('\n') {
            let body = line.trim();
            starts.push(at);
            blank.push(body.is_empty());
            let command = body.strip_prefix('#').map(|c| c.trim());
            let (turns_off, turns_on) = match command {
                Some(c) => {
                    let c = c.strip_prefix("fmt:").or_else(|| c.strip_prefix("yapf:")).map(str::trim);
                    (matches!(c, Some("off" | "disable")), matches!(c, Some("on" | "enable")))
                }
                None => (false, false),
            };
            if turns_off {
                off = true;
            }
            kept.push(off || body.contains("fmt: skip") || body.contains("fmt:skip"));
            if turns_on {
                off = false;
            }
            at += line.len();
        }
        Lines { text, starts, blank, kept, len: text.len() }
    }

    /// The line holding byte `offset`.
    fn of(&self, offset: usize) -> usize {
        self.starts.partition_point(|s| *s <= offset).saturating_sub(1)
    }

    fn range(&self, first: usize, end: usize) -> Range<usize> {
        self.starts[first]..self.starts.get(end).copied().unwrap_or(self.len)
    }
}

struct Found<'l> {
    lines: &'l Lines<'l>,
    remove: Vec<Range<usize>>,
}

impl Found<'_> {
    /// `body[0]` is a docstring: its blank lines before (always) and, in a
    /// function with more after it, behind.
    fn docstring(&mut self, body: &[Stmt], function: bool) {
        let Some(doc) = body.first().filter(|s| is_docstring_stmt(s)) else { return };
        let l = self.lines;
        let (start, end) = (usize::from(doc.start()), usize::from(doc.end()));
        let (first, last) = (l.of(start), l.of(end.saturating_sub(1).max(start)));
        // The docstring starts its own line: not `def f(): """x"""`.
        if l.text[l.starts[first]..start].trim().is_empty() {
            let mut from = first;
            while from > 0 && l.blank[from - 1] && !l.kept[from - 1] {
                from -= 1;
            }
            if from < first && !l.kept[first] {
                self.remove.push(l.range(from, first));
            }
        }
        if !function || body.len() < 2 || l.kept[last] {
            return;
        }
        let mut to = last + 1;
        while to < l.blank.len() && l.blank[to] && !l.kept[to] {
            to += 1;
        }
        let blank = to - (last + 1);
        // One blank line before a nested function or class is the formatter's.
        let nested = matches!(body[1], Stmt::FunctionDef(_) | Stmt::ClassDef(_));
        if blank > 0 && !(blank == 1 && nested) && to < l.blank.len() {
            self.remove.push(l.range(last + 1, to));
        }
    }
}

impl<'a> StatementVisitor<'a> for Found<'_> {
    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        match stmt {
            Stmt::FunctionDef(f) => self.docstring(&f.body, true),
            Stmt::ClassDef(c) => self.docstring(&c.body, false),
            _ => {}
        }
        walk_stmt(self, stmt);
    }
}

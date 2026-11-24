//! Global stylesheets and the files they load with (W-199, SR-MODULE-GRAPH
//! decision 8a): three queries over plain CSS and SCSS — a class a script
//! writes that no stylesheet declares, a class a stylesheet declares that
//! nothing uses, a `var(--x)` no stylesheet declares — each read from what
//! the project's own files say: a JS `import "./x.css"`, an HTML
//! `<link href>`, a stylesheet's `@import` / `@use` / `@forward`.
//!
//! What can load with a file is what its *entries* reach: the file's
//! importers up to the files nobody imports (an app's `main.jsx`, a page),
//! and everything those load, so a global `index.css` imported by `main.jsx`
//! counts for every component under it. A stylesheet the graph cannot read
//! — another host's, a package's, a Sass partial it cannot find, Tailwind's
//! directives, a class built by interpolation — makes the whole scope
//! *unreadable*, and the queries stay silent for it rather than guess.

use crate::facts::{FactLang, Span};
use crate::link::Graph;
use crate::modules::{is_ident, mask};
use crate::query::{GraphFinding, GraphRule, Primitive, file_name, languages_match};
use crate::resolve::{Target, dirname, join, normalize};
use codelaw_core::{Language, TextEdit};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// What a stylesheet says about the project, besides its classes
/// (`ModuleFacts` holds those).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetFacts {
    /// `@import`, `@use` and `@forward` specifiers, in order (another host's,
    /// a data URL and Sass's own `sass:` modules are not here).
    pub imports: Vec<String>,
    /// An import of another host or a data URL.
    pub remote: bool,
    /// A directive that makes classes the file does not spell (`@tailwind`,
    /// `@plugin`, `@theme`, `@source`, `@config`, `@utility`).
    pub opaque: bool,
    /// Custom properties declared (`--x: …`) or registered (`@property --x`).
    pub props: Vec<String>,
    /// `var(--x …)` reads.
    pub uses: Vec<PropUse>,
    /// A custom property named by interpolation (`--#{$x}`): the names
    /// declared cannot be listed.
    pub dynamic_props: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropUse {
    pub name: String,
    pub span: Span,
    /// `var(--x, fallback)`: valid whether or not `--x` is declared.
    pub fallback: bool,
}

/// Which of the three queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetCheck {
    /// `sheets: { classes: undeclared }`.
    UndeclaredClasses,
    /// `sheets: { classes: unused }`.
    UnusedClasses,
    /// `sheets: { properties: undeclared }`.
    UndeclaredProperties,
}

const OPAQUE_AT: [&str; 6] = ["tailwind", "plugin", "theme", "source", "config", "utility"];

/// Read a stylesheet's imports, custom properties and `var()` reads from
/// its text (not its tree: see `modules.rs`).
pub fn scan(text: &str, scss: bool) -> SheetFacts {
    let masked = mask(text, scss);
    let m = masked.as_bytes();
    let mut out = SheetFacts::default();
    let mut i = 0;
    while i < m.len() {
        match m[i] {
            b'@' => {
                let mut j = i + 1;
                while j < m.len() && (m[j].is_ascii_alphabetic() || m[j] == b'-') {
                    j += 1;
                }
                let name = &masked[i + 1..j];
                if matches!(name, "import" | "use" | "forward") {
                    read_imports(text, m, j, &mut out);
                } else if OPAQUE_AT.contains(&name) {
                    out.opaque = true;
                }
                i = j.max(i + 1);
            }
            b'-' if m.get(i + 1) == Some(&b'-') && (i == 0 || !is_ident(m[i - 1])) => {
                let mut j = i + 2;
                while j < m.len() && is_ident(m[j]) {
                    j += 1;
                }
                if m.get(j) == Some(&b'#') && m.get(j + 1) == Some(&b'{') {
                    out.dynamic_props = true;
                }
                if j > i + 2 {
                    let name = &text[i..j];
                    let before = masked[..i].trim_end();
                    let mut k = j;
                    while k < m.len() && m[k].is_ascii_whitespace() {
                        k += 1;
                    }
                    if before.len() >= 4 && before[before.len() - 4..].eq_ignore_ascii_case("var(") {
                        out.uses.push(PropUse { name: name.to_owned(), span: (i, j), fallback: m.get(k) == Some(&b',') });
                    } else if before.ends_with("@property") || m.get(k) == Some(&b':') {
                        out.props.push(name.to_owned());
                    }
                }
                i = j.max(i + 2);
            }
            _ => i += 1,
        }
    }
    out.props.sort();
    out.props.dedup();
    out
}

/// The specifiers after `@import` / `@use` / `@forward` (from `p`).
fn read_imports(text: &str, m: &[u8], mut p: usize, out: &mut SheetFacts) {
    loop {
        while p < m.len() && m[p].is_ascii_whitespace() {
            p += 1;
        }
        let spec = if m[p..].get(..4).is_some_and(|b| b.eq_ignore_ascii_case(b"url(")) {
            p += 4;
            // The masked text blanks an unquoted URL: skip blanks in the text itself.
            while p < m.len() && text.as_bytes()[p].is_ascii_whitespace() {
                p += 1;
            }
            if matches!(m.get(p), Some(b'"' | b'\'')) {
                read_string(text, m, &mut p)
            } else {
                let end = text[p..].find(')').map_or(m.len(), |o| p + o);
                let s = text[p..end].trim().to_owned();
                p = end;
                Some(s)
            }
        } else if matches!(m.get(p), Some(b'"' | b'\'')) {
            read_string(text, m, &mut p)
        } else {
            None
        };
        let Some(spec) = spec else { return };
        if spec.starts_with("http://") || spec.starts_with("https://") || spec.starts_with("//") || spec.starts_with("data:") {
            out.remote = true;
        } else if !spec.starts_with("sass:") && !spec.is_empty() {
            out.imports.push(spec);
        }
        // Sass's `@import "a", "b";`.
        while p < m.len() && m[p] != b',' && m[p] != b';' && m[p] != b'{' && m[p] != b'\n' {
            p += 1;
        }
        if m.get(p) != Some(&b',') {
            return;
        }
        p += 1;
    }
}

fn read_string(text: &str, m: &[u8], p: &mut usize) -> Option<String> {
    let q = m[*p];
    let start = *p + 1;
    let end = start + m[start..].iter().position(|&b| b == q)?;
    *p = end + 1;
    Some(text[start..end].to_owned())
}

/// The stylesheet an `@import` / `@use` specifier names, when it is one of
/// the project: relative to the importing file, with Sass's extensionless,
/// `_partial` and `index` spellings.
fn resolve_sheet(graph: &Graph, from: &str, spec: &str) -> Option<String> {
    let spec = spec.split(['?', '#']).next().unwrap_or(spec);
    let rel = if spec.starts_with('.') || spec.starts_with('/') { spec.to_owned() } else { format!("./{spec}") };
    let base = join(dirname(from), &rel).or_else(|| normalize(spec.trim_start_matches('/')))?;
    let (dir, name) = base.rsplit_once('/').unwrap_or(("", &base));
    let at = |d: &str, n: &str| if d.is_empty() { n.to_owned() } else { format!("{d}/{n}") };
    let mut candidates = vec![base.clone()];
    if !name.ends_with(".css") && !name.ends_with(".scss") && !name.ends_with(".sass") {
        for ext in [".scss", ".css"] {
            candidates.push(format!("{base}{ext}"));
            candidates.push(at(dir, &format!("_{name}{ext}")));
            candidates.push(format!("{base}/index{ext}"));
            candidates.push(format!("{base}/_index{ext}"));
        }
    } else {
        candidates.push(at(dir, &format!("_{name}")));
    }
    candidates.into_iter().find(|c| graph.modules.get(c).is_some_and(|m| m.facts.lang == Some(FactLang::Css)))
}

fn looks_like_sheet(spec: &str) -> bool {
    let p = spec.split(['?', '#']).next().unwrap_or(spec);
    [".css", ".scss", ".sass", ".less", ".styl", ".pcss"].iter().any(|e| p.ends_with(e))
}

/// What loads what, and what the graph cannot read.
struct Scopes<'g> {
    graph: &'g Graph,
    /// A file → the files it loads: its imports, a page's scripts and links,
    /// a stylesheet's `@import`s.
    edges: BTreeMap<&'g str, BTreeSet<String>>,
    parents: BTreeMap<String, BTreeSet<String>>,
    /// Files that load a stylesheet the graph cannot read.
    unreadable: BTreeSet<String>,
    closures: BTreeMap<String, BTreeSet<String>>,
}

impl<'g> Scopes<'g> {
    fn new(graph: &'g Graph) -> Self {
        let mut s = Scopes { graph, edges: BTreeMap::new(), parents: BTreeMap::new(), unreadable: BTreeSet::new(), closures: BTreeMap::new() };
        for m in graph.modules.values() {
            let mut to = BTreeSet::new();
            let mut unreadable = false;
            match m.facts.lang {
                Some(FactLang::Js) => {
                    for sp in m.facts.specifiers.iter().filter(|x| !x.type_only) {
                        let spec = sp.spec.split('?').next().unwrap_or(&sp.spec);
                        let files: Vec<String> = graph.resolver.resolve_js(&m.path, spec).into_iter().filter_map(|t| if let Target::File(f) = t { Some(f) } else { None }).filter(|f| graph.modules.contains_key(f)).collect();
                        if files.is_empty() && looks_like_sheet(spec) {
                            unreadable = true;
                        }
                        to.extend(files);
                    }
                }
                Some(FactLang::Css) => {
                    let sheet = m.facts.sheet.as_ref();
                    unreadable = sheet.is_some_and(|x| x.remote || x.opaque);
                    for spec in sheet.iter().flat_map(|x| &x.imports) {
                        match resolve_sheet(graph, &m.path, spec) {
                            Some(f) => {
                                to.insert(f);
                            }
                            None => unreadable = true,
                        }
                    }
                }
                Some(FactLang::Html) => {
                    unreadable = m.facts.markup_opaque;
                    for l in &m.facts.links {
                        let spec = if l.starts_with('/') || l.starts_with('.') { l.clone() } else { format!("./{l}") };
                        let found = join(dirname(&m.path), &spec).or_else(|| normalize(l.trim_start_matches('/'))).map(|f| graph.resolver.probe(&f).unwrap_or(f)).filter(|f| graph.modules.contains_key(f));
                        match found {
                            Some(f) => {
                                to.insert(f);
                            }
                            None if looks_like_sheet(l) => unreadable = true,
                            None => {}
                        }
                    }
                }
                _ => {}
            }
            to.remove(&m.path);
            for t in &to {
                s.parents.entry(t.clone()).or_default().insert(m.path.clone());
            }
            if unreadable {
                s.unreadable.insert(m.path.clone());
            }
            s.edges.insert(m.path.as_str(), to);
        }
        s
    }

    /// The files nobody loads that reach `f` (itself when nobody loads it).
    fn roots(&self, f: &str) -> Vec<String> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut todo = vec![f.to_owned()];
        let mut roots = Vec::new();
        while let Some(x) = todo.pop() {
            if !seen.insert(x.clone()) {
                continue;
            }
            match self.parents.get(&x) {
                Some(p) if !p.is_empty() => todo.extend(p.iter().cloned()),
                _ => roots.push(x),
            }
        }
        // Every file loads another in a cycle: take them all.
        if roots.is_empty() { seen.into_iter().collect() } else { roots }
    }

    fn closure(&mut self, root: &str) -> &BTreeSet<String> {
        if !self.closures.contains_key(root) {
            let mut seen = BTreeSet::new();
            let mut todo = vec![root.to_owned()];
            while let Some(x) = todo.pop() {
                if !seen.insert(x.clone()) {
                    continue;
                }
                if let Some(to) = self.edges.get(x.as_str()) {
                    todo.extend(to.iter().cloned());
                }
            }
            self.closures.insert(root.to_owned(), seen);
        }
        &self.closures[root]
    }

    /// Everything that loads with `f`: what each of its entries reaches.
    fn scope(&mut self, f: &str) -> Scope {
        let mut files = BTreeSet::new();
        for r in self.roots(f) {
            files.extend(self.closure(&r).iter().cloned());
        }
        let sheets: Vec<String> = files.iter().filter(|p| self.graph.modules.get(*p).is_some_and(|m| m.facts.lang == Some(FactLang::Css))).cloned().collect();
        let readable = files.iter().all(|p| !self.unreadable.contains(p)) && sheets.iter().all(|p| self.graph.modules.get(p).and_then(|m| m.facts.module.as_ref()).is_none_or(|x| !x.incomplete));
        Scope { files, readable }
    }
}

struct Scope {
    files: BTreeSet<String>,
    /// Every stylesheet in it was read whole.
    readable: bool,
}

impl Scope {
    /// The global (not CSS Module) stylesheets in it.
    fn global_sheets<'g>(&'g self, graph: &'g Graph) -> impl Iterator<Item = &'g crate::link::Module> + 'g {
        self.files.iter().filter_map(|p| graph.modules.get(p)).filter(|m| m.facts.lang == Some(FactLang::Css) && !graph.config.is_css_module(&m.path))
    }
}

fn finding(file: &str, span: Span, captures: &[(&str, &str)], edits: Vec<TextEdit>) -> GraphFinding {
    GraphFinding { file: file.to_owned(), span, captures: captures.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect(), edits, uncertain: None, message: None }
}

/// Run one of the three `sheets:` queries.
pub fn run(rule: &GraphRule, graph: &Graph, languages: &[Language], check: SheetCheck) -> Vec<GraphFinding> {
    let mut scopes = Scopes::new(graph);
    match check {
        SheetCheck::UndeclaredClasses => undeclared_classes(graph, &mut scopes, languages),
        SheetCheck::UnusedClasses => unused_classes(rule, graph, &mut scopes, languages),
        SheetCheck::UndeclaredProperties => undeclared_properties(graph, &mut scopes, languages),
    }
}

/// A class a script writes in `className` that no stylesheet loading with
/// the file declares.
fn undeclared_classes(graph: &Graph, scopes: &mut Scopes<'_>, languages: &[Language]) -> Vec<GraphFinding> {
    let mut out = Vec::new();
    for m in graph.modules.values() {
        if m.facts.lang != Some(FactLang::Js) || m.facts.class_uses.is_empty() || !languages_match(&m.path, languages) {
            continue;
        }
        let scope = scopes.scope(&m.path);
        let declared: BTreeSet<&str> = scope.global_sheets(graph).filter_map(|s| s.facts.module.as_ref()).flat_map(|x| x.classes.iter().map(|c| c.name.as_str())).collect();
        // No stylesheet loads with the file: nothing is known of its classes.
        if !scope.readable || scope.global_sheets(graph).next().is_none() {
            continue;
        }
        for u in &m.facts.class_uses {
            if !declared.contains(u.name.as_str()) {
                out.push(finding(&m.path, u.span, &[("CLASS", &u.name), ("FILE", file_name(&m.path))], Vec::new()));
            }
        }
    }
    out
}

/// A class a global stylesheet declares that no file loading with it uses.
fn unused_classes(rule: &GraphRule, graph: &Graph, scopes: &mut Scopes<'_>, languages: &[Language]) -> Vec<GraphFinding> {
    // sheet → the files whose scope holds it
    let mut users: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for m in graph.modules.values().filter(|m| matches!(m.facts.lang, Some(FactLang::Js | FactLang::Html))) {
        for s in scopes.scope(&m.path).global_sheets(graph) {
            users.entry(s.path.clone()).or_default().push(m.path.clone());
        }
    }
    let removes = rule.fix.otherwise.contains(&Primitive::RemoveRule);
    let mut out = Vec::new();
    for (sheet, files) in &users {
        let Some(m) = graph.modules.get(sheet) else { continue };
        let Some(facts) = m.facts.module.as_ref().filter(|f| !f.incomplete) else { continue };
        if !languages_match(&m.path, languages) || m.entry.is_some() || graph.is_kept_path(&m.path) || graph.in_opaque_package(&m.path).is_some() {
            continue;
        }
        let mut tokens: BTreeSet<&str> = BTreeSet::new();
        let mut prefixes: BTreeSet<&str> = BTreeSet::new();
        for f in files.iter().filter_map(|p| graph.modules.get(p)) {
            tokens.extend(f.facts.class_tokens.iter().map(String::as_str));
            prefixes.extend(f.facts.class_prefixes.iter().map(String::as_str));
        }
        // `@apply` and `composes` in the sheets themselves name classes too.
        for f in scopes.scope(sheet).files.iter().filter_map(|p| graph.modules.get(p)).filter(|f| f.facts.lang == Some(FactLang::Css)) {
            tokens.extend(f.facts.class_tokens.iter().map(String::as_str));
        }
        for c in &facts.classes {
            if tokens.contains(c.name.as_str()) || prefixes.iter().any(|p| c.name.starts_with(p)) {
                continue;
            }
            let edits = match (removes, c.rule) {
                (true, Some((a, b))) => vec![TextEdit::delete(a, b)],
                _ => Vec::new(),
            };
            out.push(finding(&m.path, c.span, &[("CLASS", &c.name), ("FILE", file_name(&m.path))], edits));
        }
    }
    out.sort_by(|a, b| (&a.file, a.span).cmp(&(&b.file, b.span)));
    out
}

/// A `var(--x)` without a fallback that no stylesheet loading with the file
/// declares, nor any script or page of the scope sets.
fn undeclared_properties(graph: &Graph, scopes: &mut Scopes<'_>, languages: &[Language]) -> Vec<GraphFinding> {
    let mut out = Vec::new();
    for m in graph.modules.values() {
        let Some(sheet) = m.facts.sheet.as_ref().filter(|s| s.uses.iter().any(|u| !u.fallback)) else { continue };
        if m.facts.lang != Some(FactLang::Css) || !languages_match(&m.path, languages) {
            continue;
        }
        // A stylesheet nothing loads is unjudged: whoever loads it may declare.
        if scopes.parents.get(&m.path).is_none_or(BTreeSet::is_empty) {
            continue;
        }
        let scope = scopes.scope(&m.path);
        let files: Vec<&crate::link::Module> = scope.files.iter().filter_map(|p| graph.modules.get(p)).collect();
        if !scope.readable || files.iter().any(|f| f.facts.sheet.as_ref().is_some_and(|s| s.dynamic_props)) {
            continue;
        }
        let declared: BTreeSet<&str> = files.iter().flat_map(|f| f.facts.sheet.iter().flat_map(|s| s.props.iter()).chain(f.facts.props_set.iter())).map(String::as_str).collect();
        for u in sheet.uses.iter().filter(|u| !u.fallback && !declared.contains(u.name.as_str())) {
            out.push(finding(&m.path, u.span, &[("PROPERTY", &u.name), ("FILE", file_name(&m.path))], Vec::new()));
        }
    }
    out
}

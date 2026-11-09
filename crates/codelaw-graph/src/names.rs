//! The `graph:` queries that read what another module exports
//! (SR-PROJECT-GRAPH decision 6, W-175): an import against its target's
//! export names, the target's default export's own name, a JSDoc
//! `@deprecated` on an export, `export *` conflicts, import cycles, and
//! modules with no exports or exports nothing imports.
//!
//! None of them adds a fact beyond `FileFacts::default_name`, and each runs
//! only when a rule that names it is on — the graph is built for project
//! rules alone, and these queries then walk the linked modules once.

use crate::facts::{ExportFact, FactLang, ImportKind, SpecForm, Span};
use crate::link::{Graph, Module, is_test_file};
use crate::query::{GraphFinding, GraphRule, Primitive, file_name, find_decl, fix_edits, languages_match, reexported, resolve_from};
use crate::resolve::Target;
use codelaw_core::{Language, TextEdit};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// What an import is checked against in the module it names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportTarget {
    /// `names: unexported`: a named import the module does not export.
    NamedMissing,
    /// `default: missing`: a default import of a module with no default export.
    DefaultMissing,
    /// `namespace: unexported`: `ns.x` where the module exports no `x`.
    NamespaceMember,
    /// `default_name: named`: a default import bound to a name the module also exports by name.
    NamedAsDefault,
    /// `default_member: named`: `foo.bar` off a default import, `bar` being a named export.
    NamedAsDefaultMember,
    /// `default_name: differs`: a default import bound to another name than the export's own.
    RenameDefault,
    /// `deprecated: jsdoc`: a name the module marks `@deprecated`, imported or used.
    Deprecated,
}

/// The export names of a module, `export *` followed.
#[derive(Clone, Debug, Default)]
struct Names {
    set: BTreeSet<String>,
    /// Every `export *` was read: a name missing from `set` is missing.
    complete: bool,
}

/// The file an import specifier of `from` names, when it is one of the project.
fn target_file(graph: &Graph, from: &str, spec: &str) -> Option<String> {
    resolve_from(graph, from, spec).into_iter().find_map(|t| if let Target::File(f) = t { Some(f) } else { None })
}

/// Whether a module is written as an ES module: ambiguous files (CommonJS,
/// scripts) are never judged by what they export.
fn is_esm(m: &Module) -> bool {
    !m.facts.exports.is_empty() || m.facts.specifiers.iter().any(|s| matches!(s.form, SpecForm::Import | SpecForm::Export))
}

fn exported_names(graph: &Graph, path: &str, visiting: &mut Vec<String>) -> Option<Names> {
    let m = graph.modules.get(path)?;
    if m.facts.lang != Some(FactLang::Js) || !is_esm(m) || m.facts.exports.iter().any(|e| e.name == "=") {
        return None;
    }
    if visiting.iter().any(|v| v == path) {
        return Some(Names { set: BTreeSet::new(), complete: true });
    }
    visiting.push(path.to_owned());
    let mut out = Names { set: BTreeSet::new(), complete: true };
    for e in &m.facts.exports {
        if e.name != "*" {
            out.set.insert(e.name.clone());
            continue;
        }
        let Some(src) = &e.source else { continue };
        let inner = target_file(graph, path, src).and_then(|t| exported_names(graph, &t, visiting));
        match inner {
            Some(n) => {
                out.set.extend(n.set.into_iter().filter(|x| x != "default"));
                out.complete &= n.complete;
            }
            None => out.complete = false,
        }
    }
    visiting.pop();
    Some(out)
}

/// Lazily computed export names, one entry per module a run looks at.
struct Cache<'g> {
    graph: &'g Graph,
    names: BTreeMap<String, Option<Names>>,
}

impl<'g> Cache<'g> {
    fn new(graph: &'g Graph) -> Self {
        Cache { graph, names: BTreeMap::new() }
    }

    fn names(&mut self, path: &str) -> Option<&Names> {
        let graph = self.graph;
        self.names.entry(path.to_owned()).or_insert_with(|| exported_names(graph, path, &mut Vec::new())).as_ref()
    }
}

fn finding(file: &str, span: Span, captures: &[(&str, &str)], message: Option<String>, edits: Vec<TextEdit>) -> GraphFinding {
    GraphFinding { file: file.to_owned(), span, captures: captures.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect(), edits, uncertain: None, message }
}

fn is_ident(s: &str) -> bool {
    s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$') && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// The one export name nearest in spelling to `name`, when one is near.
fn nearest(name: &str, names: &BTreeSet<String>) -> Option<String> {
    let max = (name.len() / 4).clamp(1, 2);
    let mut near: Vec<(usize, &String)> = names.iter().filter(|n| is_ident(n) && n.as_str() != name && n.as_str() != "default").map(|n| (codelaw_core::similar::levenshtein(name, n), n)).filter(|(d, _)| *d <= max).collect();
    near.sort();
    let best = near.first()?.0;
    let mut best_names = near.iter().filter(|(d, _)| *d == best);
    let first = best_names.next()?.1;
    best_names.next().is_none().then(|| first.clone())
}

/// Run an `import:` query against what the imported modules export.
pub fn names(rule: &GraphRule, graph: &Graph, languages: &[Language], check: ImportTarget) -> Vec<GraphFinding> {
    let mut cache = Cache::new(graph);
    let mut out = Vec::new();
    for m in graph.modules.values() {
        if m.facts.lang != Some(FactLang::Js) || !languages_match(&m.path, languages) {
            continue;
        }
        let suggests = rule.fix.otherwise.contains(&Primitive::NearestExport);
        for imp in m.facts.imports.iter().filter(|i| !i.type_only) {
            let Some(target) = target_file(graph, &m.path, &imp.source) else { continue };
            if target == m.path {
                continue;
            }
            let Some(local) = imp.local.as_deref() else { continue };
            let src = imp.source.as_str();
            let file = file_name(&target);
            match (check, &imp.kind) {
                (ImportTarget::NamedMissing, ImportKind::Named(n)) => {
                    let Some(have) = cache.names(&target).filter(|h| h.complete) else { continue };
                    if have.set.contains(n) {
                        continue;
                    }
                    let mut edits = Vec::new();
                    let len = imp.span.1 - imp.span.0;
                    if suggests && is_ident(n)
                        && let Some(near) = nearest(n, &have.set)
                    {
                        // Keep the local name: `{ near as local }`.
                        if local == n && len == n.len() {
                            edits.push(TextEdit::new(imp.span.0, imp.span.1, format!("{near} as {n}")));
                        } else if len == n.len() + 4 + local.len() {
                            edits.push(TextEdit::new(imp.span.0, imp.span.0 + n.len(), near));
                        }
                    }
                    out.push(finding(&m.path, imp.span, &[("NAME", n), ("SOURCE", src), ("FILE", file)], None, edits));
                }
                (ImportTarget::DefaultMissing, ImportKind::Default) => {
                    let Some(have) = cache.names(&target).filter(|h| h.complete) else { continue };
                    if !have.set.contains("default") {
                        out.push(finding(&m.path, imp.span, &[("SOURCE", src), ("FILE", file)], None, Vec::new()));
                    }
                }
                (ImportTarget::NamespaceMember, ImportKind::Namespace) => {
                    let Some(have) = cache.names(&target).filter(|h| h.complete).cloned() else { continue };
                    if have.set.is_empty() {
                        out.push(finding(&m.path, imp.span, &[("SOURCE", src), ("FILE", file)], Some(format!("No names are exported by `{src}`")), Vec::new()));
                        continue;
                    }
                    for mem in imp.members.iter().flatten().filter(|x| !have.set.contains(&x.name)) {
                        let mut edits = Vec::new();
                        if suggests && is_ident(&mem.name) && mem.span.1 - mem.span.0 == mem.name.len()
                            && let Some(near) = nearest(&mem.name, &have.set)
                        {
                            edits.push(TextEdit::new(mem.span.0, mem.span.1, near));
                        }
                        out.push(finding(&m.path, mem.span, &[("NAME", &mem.name), ("NAMESPACE", local), ("SOURCE", src), ("FILE", file)], None, edits));
                    }
                }
                (ImportTarget::NamedAsDefault, ImportKind::Default) => {
                    let Some(tm) = graph.modules.get(&target) else { continue };
                    let Some(have) = cache.names(&target) else { continue };
                    if local == "default" || !have.set.contains("default") || !have.set.contains(local) {
                        continue;
                    }
                    let own = |name: &str| tm.facts.exports.iter().find(|e| e.name == name);
                    if let (Some(d), Some(n)) = (own("default"), own(local))
                        && same_thing(d, n)
                    {
                        continue;
                    }
                    out.push(finding(&m.path, imp.span, &[("NAME", local), ("SOURCE", src), ("FILE", file)], None, Vec::new()));
                }
                (ImportTarget::NamedAsDefaultMember, ImportKind::Default) => {
                    let Some(have) = cache.names(&target).cloned() else { continue };
                    for mem in imp.members.iter().flatten().filter(|x| x.name != "default" && have.set.contains(&x.name)) {
                        out.push(finding(&m.path, mem.span, &[("NAME", &mem.name), ("LOCAL", local), ("SOURCE", src), ("FILE", file)], None, Vec::new()));
                    }
                }
                (ImportTarget::RenameDefault, ImportKind::Default | ImportKind::Named(_)) => {
                    if matches!(&imp.kind, ImportKind::Named(n) if n != "default") {
                        continue;
                    }
                    let Some(tm) = graph.modules.get(&target).filter(|t| is_esm(t)) else { continue };
                    let Some(own) = tm.facts.default_name.as_deref() else { continue };
                    if own != local {
                        out.push(finding(&m.path, imp.span, &[("NAME", own), ("LOCAL", local), ("SOURCE", src), ("FILE", file)], None, Vec::new()));
                    }
                }
                (ImportTarget::Deprecated, kind) => {
                    deprecated_import(graph, &mut cache, &m.path, imp, kind, &target, &mut out);
                }
                _ => {}
            }
        }
        if check == ImportTarget::NamedMissing {
            reexport_names(graph, &mut cache, m, &mut out);
        }
    }
    out
}

/// Two export facts that export one binding: `export default foo` beside `export { foo }`.
fn same_thing(a: &ExportFact, b: &ExportFact) -> bool {
    (a.local.is_some() && a.local == b.local) || (a.source.is_some() && a.source == b.source && a.imported == b.imported)
}

/// `export { a } from "./x"`: the name must be one `./x` exports.
fn reexport_names(graph: &Graph, cache: &mut Cache<'_>, m: &Module, out: &mut Vec<GraphFinding>) {
    for e in &m.facts.exports {
        let (Some(src), Some(imported)) = (&e.source, &e.imported) else { continue };
        if imported == "*" || e.name == "*" {
            continue;
        }
        let Some(target) = target_file(graph, &m.path, src).filter(|t| *t != m.path) else { continue };
        let Some(have) = cache.names(&target).filter(|h| h.complete) else { continue };
        if !have.set.contains(imported) {
            out.push(finding(&m.path, e.span, &[("NAME", imported), ("SOURCE", src), ("FILE", file_name(&target))], None, Vec::new()));
        }
    }
}

fn deprecation(graph: &Graph, target: &str, name: &str) -> Option<String> {
    let (module, decl) = find_decl(graph, &Target::File(target.to_owned()), name, 0)?;
    let d = graph.modules.get(&module)?.facts.decls.iter().find(|d| d.name == decl)?;
    d.deprecated.clone()
}

fn deprecated_message(note: &str) -> String {
    if note.is_empty() { "Deprecated.".to_owned() } else { format!("Deprecated: {note}") }
}

fn deprecated_import(graph: &Graph, cache: &mut Cache<'_>, from: &str, imp: &crate::facts::ImportFact, kind: &ImportKind, target: &str, out: &mut Vec<GraphFinding>) {
    let _ = cache;
    match kind {
        ImportKind::Named(_) | ImportKind::Default => {
            let name = if let ImportKind::Named(n) = kind { n.as_str() } else { "default" };
            let Some(note) = deprecation(graph, target, name) else { return };
            let message = deprecated_message(&note);
            let caps = [("NAME", name), ("NOTE", note.as_str())];
            out.push(finding(from, imp.span, &caps, Some(message.clone()), Vec::new()));
            for u in &imp.uses {
                out.push(finding(from, *u, &caps, Some(message.clone()), Vec::new()));
            }
        }
        ImportKind::Namespace => {
            for mem in imp.members.iter().flatten() {
                let Some(note) = deprecation(graph, target, &mem.name) else { continue };
                out.push(finding(from, mem.span, &[("NAME", &mem.name), ("NOTE", &note)], Some(deprecated_message(&note)), Vec::new()));
            }
        }
        _ => {}
    }
}

/// `graph: { export: { star: conflicts } }`: a name two `export *` (or an
/// `export *` and a named export) of one module both bring in.
pub fn export_star(graph: &Graph, languages: &[Language]) -> Vec<GraphFinding> {
    let mut cache = Cache::new(graph);
    let mut out = Vec::new();
    for m in graph.modules.values() {
        if m.facts.lang != Some(FactLang::Js) || !languages_match(&m.path, languages) {
            continue;
        }
        let stars: Vec<&ExportFact> = m.facts.exports.iter().filter(|e| e.name == "*" && e.source.is_some()).collect();
        if stars.is_empty() {
            continue;
        }
        // name → the statements that bring it in
        let mut who: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        for e in m.facts.exports.iter().filter(|e| e.name != "*" && e.name != "default" && e.name != "=") {
            // A type and a value of one name are two exports.
            let is_type = e.local.as_deref().is_some_and(|l| m.facts.decls.iter().any(|d| d.name == l && matches!(d.kind, crate::facts::DeclKind::Type | crate::facts::DeclKind::Interface)));
            if !is_type {
                who.entry(e.name.clone()).or_default().push(e.span);
            }
        }
        let mut empty: Vec<(&ExportFact, String)> = Vec::new();
        for e in &stars {
            let src = e.source.as_deref().unwrap_or_default();
            let Some(have) = target_file(graph, &m.path, src).filter(|t| *t != m.path).and_then(|t| cache.names(&t).cloned()) else { continue };
            let named: Vec<&String> = have.set.iter().filter(|n| *n != "default").collect();
            if named.is_empty() && have.complete {
                empty.push((e, src.to_owned()));
            }
            for n in named {
                who.entry(n.clone()).or_default().push(e.span);
            }
        }
        let mut seen: BTreeSet<(Span, String)> = BTreeSet::new();
        for (name, spans) in &who {
            let distinct: BTreeSet<&Span> = spans.iter().collect();
            if distinct.len() < 2 {
                continue;
            }
            for span in distinct {
                if seen.insert((*span, name.clone())) {
                    out.push(finding(&m.path, *span, &[("NAME", name)], None, Vec::new()));
                }
            }
        }
        for (e, src) in empty {
            out.push(finding(&m.path, e.span, &[("SOURCE", &src)], Some(format!("`{src}` has no named exports to re-export")), Vec::new()));
        }
    }
    out
}

/// `graph: { cycle: … }`: an import whose target imports its way back.
pub fn cycles(graph: &Graph, languages: &[Language], forms: &[SpecForm]) -> Vec<GraphFinding> {
    // Edges between modules of the project, from the specifiers written.
    let mut index: BTreeMap<&str, usize> = BTreeMap::new();
    let mut paths: Vec<&str> = Vec::new();
    for m in graph.modules.values().filter(|m| m.facts.lang == Some(FactLang::Js)) {
        index.insert(&m.path, paths.len());
        paths.push(&m.path);
    }
    let mut edges: Vec<Vec<(usize, usize, Span)>> = vec![Vec::new(); paths.len()];
    for (i, p) in paths.iter().enumerate() {
        let m = &graph.modules[*p];
        for s in m.facts.specifiers.iter().filter(|s| forms.contains(&s.form) && !s.type_only) {
            if s.form == SpecForm::Import {
                let mut of = m.facts.imports.iter().filter(|x| x.source == s.spec).peekable();
                if of.peek().is_some() && of.all(|x| x.type_only) {
                    continue;
                }
            }
            let Some(&t) = target_file(graph, p, &s.spec).as_deref().and_then(|t| index.get(t)) else { continue };
            if t != i {
                edges[i].push((t, 0, s.span));
            }
        }
    }
    let component = strongly_connected(&edges);
    let mut out = Vec::new();
    for (i, p) in paths.iter().enumerate() {
        if !languages_match(p, languages) {
            continue;
        }
        let mut seen: BTreeSet<Span> = BTreeSet::new();
        for &(t, _, span) in &edges[i] {
            if component[t] != component[i] || !seen.insert(span) {
                continue;
            }
            // The way back, shortest first.
            let route = way_back(&edges, &component, t, i);
            let names: Vec<&str> = route.iter().map(|&x| file_name(paths[x])).collect();
            let message = if names.len() > 1 { format!("Dependency cycle via {}", names.join(" -> ")) } else { "Dependency cycle detected".to_owned() };
            out.push(finding(p, span, &[("ROUTE", &names.join(" -> "))], Some(message), Vec::new()));
        }
    }
    out
}

fn way_back(edges: &[Vec<(usize, usize, Span)>], component: &[usize], from: usize, to: usize) -> Vec<usize> {
    let mut prev: BTreeMap<usize, usize> = BTreeMap::new();
    let mut queue = VecDeque::from([from]);
    prev.insert(from, from);
    while let Some(n) = queue.pop_front() {
        if n == to {
            break;
        }
        for &(next, _, _) in &edges[n] {
            if component[next] == component[from] && !prev.contains_key(&next) {
                prev.insert(next, n);
                queue.push_back(next);
            }
        }
    }
    let mut route = vec![to];
    let mut at = to;
    while at != from {
        let Some(&p) = prev.get(&at) else { break };
        route.push(p);
        at = p;
    }
    route.reverse();
    route
}

/// Tarjan's strongly connected components, iterative: the component of each node.
fn strongly_connected(edges: &[Vec<(usize, usize, Span)>]) -> Vec<usize> {
    let n = edges.len();
    let (mut index, mut low) = (vec![usize::MAX; n], vec![0usize; n]);
    let (mut on, mut comp) = (vec![false; n], vec![usize::MAX; n]);
    let (mut stack, mut next, mut count) = (Vec::new(), 0usize, 0usize);
    for root in 0..n {
        if index[root] != usize::MAX {
            continue;
        }
        let mut work: Vec<(usize, usize)> = vec![(root, 0)];
        while let Some(&mut (v, ref mut i)) = work.last_mut() {
            if *i == 0 {
                index[v] = next;
                low[v] = next;
                next += 1;
                stack.push(v);
                on[v] = true;
            }
            if *i < edges[v].len() {
                let w = edges[v][*i].0;
                *i += 1;
                if index[w] == usize::MAX {
                    work.push((w, 0));
                } else if on[w] {
                    low[v] = low[v].min(index[w]);
                }
                continue;
            }
            work.pop();
            if let Some(&(parent, _)) = work.last() {
                low[parent] = low[parent].min(low[v]);
            }
            if low[v] == index[v] {
                while let Some(w) = stack.pop() {
                    on[w] = false;
                    comp[w] = count;
                    if w == v {
                        break;
                    }
                }
                count += 1;
            }
        }
    }
    comp
}

/// What the other modules read from each module: every name, the names an
/// `export *` passes on, or these.
#[derive(Default)]
struct Used {
    all: bool,
    all_named: bool,
    names: BTreeSet<String>,
}

fn reads(graph: &Graph) -> BTreeMap<String, Used> {
    let mut used: BTreeMap<String, Used> = BTreeMap::new();
    let mut stars: Vec<(String, String)> = Vec::new();
    for m in graph.modules.values().filter(|m| m.facts.lang == Some(FactLang::Js)) {
        for imp in &m.facts.imports {
            let Some(t) = target_file(graph, &m.path, &imp.source) else { continue };
            let u = used.entry(t).or_default();
            match &imp.kind {
                ImportKind::Named(n) => {
                    u.names.insert(n.clone());
                }
                ImportKind::Default => {
                    u.names.insert("default".into());
                }
                ImportKind::Namespace => match &imp.members {
                    Some(members) => u.names.extend(members.iter().map(|x| x.name.clone())),
                    None => u.all = true,
                },
                _ => {}
            }
        }
        for e in &m.facts.exports {
            let (Some(src), Some(imported)) = (&e.source, &e.imported) else { continue };
            let Some(t) = target_file(graph, &m.path, src) else { continue };
            if e.name == "*" {
                stars.push((m.path.clone(), t));
            } else if imported == "*" {
                used.entry(t).or_default().all = true;
            } else {
                used.entry(t).or_default().names.insert(imported.clone());
            }
        }
        // A literal `import()` or `require()` may read any name.
        for s in m.facts.specifiers.iter().filter(|s| matches!(s.form, SpecForm::Dynamic | SpecForm::Require)) {
            if let Some(t) = target_file(graph, &m.path, &s.spec) {
                used.entry(t).or_default().all = true;
            }
        }
    }
    // `export * from` passes the names on when the re-exporting module is itself read.
    loop {
        let mut changed = false;
        for (from, to) in &stars {
            let from_read = graph.modules.get(from).is_some_and(|m| read_from_outside(graph, m)) || used.get(from).is_some_and(|u| u.all || u.all_named || !u.names.is_empty());
            if from_read {
                let u = used.entry(to.clone()).or_default();
                if !u.all_named {
                    u.all_named = true;
                    changed = true;
                }
            }
        }
        if !changed {
            return used;
        }
    }
}

/// An entry point, or a module of a package whose public surface is read by others.
fn read_from_outside(graph: &Graph, m: &Module) -> bool {
    m.entry.is_some() || m.all_used.is_some() || graph.in_opaque_package(&m.path).is_some()
}

/// `graph: { module: { exports: [missing, unused] } }`.
pub fn modules(rule: &GraphRule, graph: &Graph, languages: &[Language], missing: bool, unused: bool) -> Vec<GraphFinding> {
    let used = if unused { reads(graph) } else { BTreeMap::new() };
    let reexports = if unused { reexported(graph) } else { BTreeSet::new() };
    let mut out = Vec::new();
    for m in graph.modules.values() {
        if m.facts.lang != Some(FactLang::Js) || !languages_match(&m.path, languages) {
            continue;
        }
        // Entry points, the public API of a package, kept paths and tests are read from outside.
        if read_from_outside(graph, m) || graph.is_kept_path(&m.path) || is_test_file(&m.path) || m.facts.dynamic.is_some() {
            continue;
        }
        let exports: Vec<&ExportFact> = m.facts.exports.iter().filter(|e| e.name != "=").collect();
        if missing && exports.is_empty() {
            let at = m.facts.imports.iter().map(|i| i.span.0).chain(m.facts.decls.iter().map(|d| d.span.0)).min().unwrap_or(0);
            out.push(finding(&m.path, (at, at), &[("FILE", file_name(&m.path))], Some("No exports found".to_owned()), Vec::new()));
        }
        if !unused {
            continue;
        }
        let read = used.get(&m.path);
        if read.is_some_and(|u| u.all) {
            continue;
        }
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for e in exports.iter().filter(|e| e.name != "*") {
            let name = e.name.as_str();
            if !seen.insert(name) || read.is_some_and(|u| u.names.contains(name) || (u.all_named && name != "default")) {
                continue;
            }
            let decl = e.local.as_deref().and_then(|l| m.facts.decls.iter().find(|d| d.name == l));
            let (span, edits) = match decl {
                Some(d) => (d.span, if reexports.contains(&(m.path.clone(), d.name.clone())) { Vec::new() } else { fix_edits(&rule.fix, d, false) }),
                None => (e.span, Vec::new()),
            };
            out.push(finding(&m.path, span, &[("NAME", name), ("FILE", file_name(&m.path))], None, edits));
        }
    }
    out
}

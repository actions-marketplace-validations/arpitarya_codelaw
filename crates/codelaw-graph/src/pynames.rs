//! The Python queries (SR-PROJECT-GRAPH decision 12, W-185): an import read
//! against the project's files, the names a module binds, and a class's
//! ancestors across files. Each runs only when a rule that names it is on,
//! once per run, over modules already linked.
//!
//! Nothing here guesses. A module the project does not hold (the standard
//! library, an installed package) is never judged; a module whose names the
//! facts cannot all know (`__getattr__`, `globals()`, a `__path__` change,
//! a `from x import *` of a module outside the project, a dynamic `__all__`)
//! is never judged by them.

use crate::facts::{FactLang, ImportKind, PyClass, Span};
use crate::link::{Graph, Module};
use crate::names::finding;
use crate::query::{GraphFinding, file_name, languages_match};
use crate::resolve::{Target, dirname, join};
use codelaw_core::Language;
use std::collections::{BTreeMap, BTreeSet};

/// What a `graph: python: imports:` query reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyImportCheck {
    /// `unresolved`: an import of a module of the project that is not there.
    Unresolved,
    /// `unexported`: `from m import x` where `m` binds no `x` and has no submodule `x`.
    Unexported,
    /// `beyond_top`: a relative import with more dots than the package has levels.
    BeyondTop,
    /// `wildcard_unused`: names a `from m import *` brings that the file never reads.
    WildcardUnused,
    /// `private`: a `_name`, or a module with a `_part`, imported from another
    /// top-level package (W-186, Ruff `PLC2701`).
    PrivateName,
}

/// The number of leading dots of a Python module spelling (`..a.b` is 2).
pub fn level(source: &str) -> usize {
    source.chars().take_while(|c| *c == '.').count()
}

fn is_init(path: &str) -> bool {
    matches!(file_name(path), "__init__.py" | "__init__.pyi")
}

/// How many folders, from the file's own folder up, hold an `__init__.py`:
/// the levels a relative import of the file can climb (Pylint's reading —
/// a folder without `__init__.py` ends the package).
pub fn package_depth(graph: &Graph, file: &str) -> usize {
    let mut dir = dirname(file).to_owned();
    let mut depth = 0;
    loop {
        let has_init = ["__init__.py", "__init__.pyi"].iter().any(|n| join(&dir, n).is_some_and(|p| graph.resolver.exists(&p)));
        if !has_init {
            return depth;
        }
        depth += 1;
        if dir.is_empty() {
            return depth;
        }
        dir = dirname(&dir).to_owned();
    }
}

fn resolve(graph: &Graph, from: &str, module: &str) -> Option<Target> {
    graph.resolver.resolve_py(from, module)
}

fn python_modules<'g>(graph: &'g Graph, languages: &'g [Language]) -> impl Iterator<Item = &'g Module> {
    graph.modules.values().filter(move |m| m.facts.lang == Some(FactLang::Python) && m.facts.py.is_some() && languages_match(&m.path, languages))
}

/// The names `from <module> import *` brings, `None` when they cannot be known.
fn star_names(graph: &Graph, path: &str, memo: &mut BTreeMap<String, Option<Names>>) -> Option<BTreeSet<String>> {
    let tm = graph.modules.get(path)?;
    let py = tm.facts.py.as_ref()?;
    if py.all_dynamic {
        return None;
    }
    if let Some(all) = &py.all {
        return Some(all.iter().cloned().collect());
    }
    let names = module_names(graph, path, memo, &mut Vec::new())?;
    Some(names.set.into_iter().filter(|n| !n.starts_with('_')).collect())
}

/// The names a module binds at its top level, `from x import *` followed.
#[derive(Clone, Debug, Default)]
pub(crate) struct Names {
    pub set: BTreeSet<String>,
    /// Every name the module can bind is in `set`.
    pub complete: bool,
}

pub(crate) fn module_names(graph: &Graph, path: &str, memo: &mut BTreeMap<String, Option<Names>>, visiting: &mut Vec<String>) -> Option<Names> {
    if let Some(done) = memo.get(path) {
        return done.clone();
    }
    let m = graph.modules.get(path)?;
    let py = m.facts.py.as_ref()?;
    if visiting.iter().any(|v| v == path) {
        return Some(Names { set: BTreeSet::new(), complete: true });
    }
    visiting.push(path.to_owned());
    let mut out = Names { set: py.names.iter().cloned().collect(), complete: !py.open };
    for imp in m.facts.imports.iter().filter(|i| i.kind == ImportKind::Star) {
        let inner = match resolve(graph, path, &imp.source) {
            Some(Target::File(t)) if t != path => {
                let tm = graph.modules.get(&t).and_then(|x| x.facts.py.as_ref());
                match tm {
                    Some(tp) if tp.all_dynamic => None,
                    Some(tp) if tp.all.is_some() => tp.all.clone().map(|a| (a.into_iter().collect::<BTreeSet<_>>(), true)),
                    Some(_) => module_names(graph, &t, memo, visiting).map(|n| (n.set.into_iter().filter(|x| !x.starts_with('_')).collect(), n.complete)),
                    None => None,
                }
            }
            _ => None,
        };
        match inner {
            Some((set, complete)) => {
                out.set.extend(set);
                out.complete &= complete;
            }
            None => out.complete = false,
        }
    }
    visiting.pop();
    memo.insert(path.to_owned(), Some(out.clone()));
    Some(out)
}

/// A name `n` of a package target that is one of its submodules.
fn is_submodule(graph: &Graph, target: &str, name: &str) -> bool {
    graph.resolver.py_submodule(&Target::File(target.to_owned()), name).is_some()
}

/// Run a `python: imports:` query.
pub fn imports(graph: &Graph, languages: &[Language], check: PyImportCheck) -> Vec<GraphFinding> {
    let mut memo = BTreeMap::new();
    let mut out = Vec::new();
    for m in python_modules(graph, languages) {
        let py = m.facts.py.as_ref().expect("python_modules keeps modules with facts");
        let depth = package_depth(graph, &m.path);
        let mut seen: BTreeSet<Span> = BTreeSet::new();
        for imp in &m.facts.imports {
            let lv = level(&imp.source);
            match check {
                PyImportCheck::BeyondTop => {
                    if lv > depth && seen.insert(imp.stmt) {
                        out.push(finding(&m.path, imp.stmt, &[("SOURCE", &imp.source)], Some(format!("Attempted relative import beyond top-level package (`{}`)", imp.source)), Vec::new()));
                    }
                }
                PyImportCheck::Unresolved => {
                    if imp.guarded || py.path_hack || lv > depth || imp.kind == ImportKind::SideEffect || !seen.insert(imp.stmt) {
                        continue;
                    }
                    if unresolved(graph, &m.path, &imp.source, lv) {
                        out.push(finding(&m.path, imp.stmt, &[("SOURCE", &imp.source)], None, Vec::new()));
                    }
                }
                PyImportCheck::Unexported => {
                    let ImportKind::Named(n) = &imp.kind else { continue };
                    if imp.guarded || lv > depth {
                        continue;
                    }
                    let Some(Target::File(t)) = resolve(graph, &m.path, &imp.source) else { continue };
                    if t == m.path || graph.modules.get(&t).is_none_or(|x| x.facts.lang != Some(FactLang::Python)) {
                        continue;
                    }
                    let Some(have) = module_names(graph, &t, &mut memo, &mut Vec::new()).filter(|h| h.complete) else { continue };
                    if have.set.contains(n) || is_submodule(graph, &t, n) {
                        continue;
                    }
                    out.push(finding(&m.path, imp.span, &[("NAME", n), ("SOURCE", &imp.source), ("FILE", file_name(&t))], None, Vec::new()));
                }
                PyImportCheck::PrivateName => {
                    if imp.type_checking || imp.type_only || lv > 0 || imp.kind == ImportKind::SideEffect {
                        continue;
                    }
                    let private = |s: &str| s.starts_with('_') && !(s.starts_with("__") && s.ends_with("__"));
                    let module_part = imp.source.split('.').find(|p| private(p));
                    let name = match &imp.kind {
                        ImportKind::Named(n) if private(n) => Some(n.as_str()),
                        _ => None,
                    };
                    let Some(shown) = name.or(module_part) else { continue };
                    // The same top-level package may use its own private names.
                    let mine = dotted(graph, &m.path);
                    if mine.split('.').next() == imp.source.split('.').next() || !seen.insert(imp.span) {
                        continue;
                    }
                    out.push(finding(&m.path, imp.span, &[("NAME", shown), ("SOURCE", &imp.source)], None, Vec::new()));
                }
                PyImportCheck::WildcardUnused => {
                    if imp.kind != ImportKind::Star || is_init(&m.path) || lv > depth {
                        continue;
                    }
                    let Some(Target::File(t)) = resolve(graph, &m.path, &imp.source) else { continue };
                    if t == m.path {
                        continue;
                    }
                    let Some(brought) = star_names(graph, &t, &mut memo) else { continue };
                    let unused: Vec<&String> = brought.iter().filter(|n| py.free.binary_search(n).is_err() && m.facts.class_tokens.binary_search(n).is_err()).collect();
                    if unused.is_empty() {
                        continue;
                    }
                    let listed: Vec<&str> = unused.iter().take(10).map(|n| n.as_str()).collect();
                    let more = if unused.len() > 10 { format!(" and {} more", unused.len() - 10) } else { String::new() };
                    let message = format!("Unused {} from wildcard import of `{}`", listed.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ") + &more, imp.source);
                    out.push(finding(&m.path, imp.stmt, &[("SOURCE", &imp.source), ("NAMES", &listed.join(", "))], Some(message), Vec::new()));
                }
            }
        }
    }
    out
}

/// Whether an import names a module of the project that is not there: a
/// relative import whose target is missing, or an absolute one whose first
/// part is a module or a regular package of the project and whose rest is
/// not. A first part the project does not hold (the standard library, an
/// installed package, a namespace folder) is not judged.
fn unresolved(graph: &Graph, from: &str, source: &str, lv: usize) -> bool {
    if lv > 0 {
        return resolve(graph, from, source).is_none();
    }
    let mut parts = source.split('.');
    let first = parts.next().unwrap_or_default();
    if parts.next().is_none() {
        return false;
    }
    let Some(Target::File(head)) = resolve(graph, from, first) else { return false };
    // A package that changes its own `__path__` takes modules from elsewhere.
    if graph.modules.get(&head).and_then(|x| x.facts.py.as_ref()).is_some_and(|p| p.open) {
        return false;
    }
    resolve(graph, from, source).is_none()
}

/// The import edges of a Python module for cycles: each project module a
/// statement imports, and the submodule `from pkg import sub` names. An
/// import under `if TYPE_CHECKING:` is not run, so it is not an edge.
pub(crate) fn import_edges(graph: &Graph, m: &Module) -> Vec<(String, Span)> {
    let depth = package_depth(graph, &m.path);
    let mut seen: BTreeSet<(String, Span)> = BTreeSet::new();
    let mut out = Vec::new();
    for imp in m.facts.imports.iter().filter(|i| !i.type_checking && i.kind != ImportKind::SideEffect) {
        if level(&imp.source) > depth {
            continue;
        }
        let Some(Target::File(t)) = resolve(graph, &m.path, &imp.source) else { continue };
        let mut targets = vec![t.clone()];
        if let ImportKind::Named(n) = &imp.kind
            && let Some(Target::File(sub)) = graph.resolver.py_submodule(&Target::File(t), n)
        {
            targets.push(sub);
        }
        for t in targets {
            if t != m.path && seen.insert((t.clone(), imp.stmt)) {
                out.push((t, imp.stmt));
            }
        }
    }
    out
}

/// A module's dotted name, or its file name.
pub(crate) fn dotted(graph: &Graph, path: &str) -> String {
    graph.resolver.py_module_name(path).unwrap_or_else(|| file_name(path).to_owned())
}

/// One base of a class.
enum Base {
    Class(String, String),
    External(String),
    Ignored,
}

/// The class `name` a module defines or re-exports (`from .a import name`),
/// the file it is in and its name there.
fn find_class(graph: &Graph, file: &str, name: &str, depth: usize) -> Option<(String, String)> {
    let m = graph.modules.get(file)?;
    let py = m.facts.py.as_ref()?;
    if py.classes.iter().any(|c| c.top && c.name == name) {
        return Some((file.to_owned(), name.to_owned()));
    }
    if depth >= 8 {
        return None;
    }
    for imp in &m.facts.imports {
        let hit = match &imp.kind {
            ImportKind::Named(n) if imp.local.as_deref() == Some(name) => Some(n.as_str()),
            ImportKind::Star => Some(name),
            _ => None,
        };
        let Some(wanted) = hit else { continue };
        if let Some(Target::File(t)) = resolve(graph, file, &imp.source)
            && t != file
            && let Some(found) = find_class(graph, &t, wanted, depth + 1)
        {
            return Some(found);
        }
    }
    None
}

/// A module's name as a base, through `import a.b`, `import a.b as m` and
/// `from pkg import mod`: the file of the dotted path `a.b` as `file` writes it.
fn module_of(graph: &Graph, file: &str, segments: &[&str]) -> Option<String> {
    let m = graph.modules.get(file)?;
    let first = segments[0];
    for imp in m.facts.imports.iter().filter(|i| i.local.as_deref() == Some(first)) {
        match &imp.kind {
            ImportKind::Module => {
                let source_first = imp.source.split('.').next().unwrap_or_default();
                // `import a.b` binds `a`; `import a.b as m` binds `m` to `a.b`.
                let path = if source_first == first && imp.source.contains('.') || imp.source == first {
                    segments.join(".")
                } else {
                    std::iter::once(imp.source.as_str()).chain(segments[1..].iter().copied()).collect::<Vec<_>>().join(".")
                };
                if let Some(Target::File(t)) = resolve(graph, file, &path) {
                    return Some(t);
                }
            }
            ImportKind::Named(n) => {
                let mut at = graph.resolver.py_submodule(&resolve(graph, file, &imp.source)?, n)?;
                for seg in &segments[1..] {
                    at = graph.resolver.py_submodule(&at, seg)?;
                }
                if let Target::File(t) = at {
                    return Some(t);
                }
            }
            _ => {}
        }
    }
    None
}

fn resolve_base(graph: &Graph, file: &str, text: &str) -> Base {
    let segments: Vec<&str> = text.split('.').collect();
    let m = &graph.modules[file];
    let py = m.facts.py.as_ref().expect("a class has python facts");
    if segments.len() == 1 {
        let name = segments[0];
        if py.classes.iter().any(|c| c.top && c.name == name) {
            return Base::Class(file.to_owned(), name.to_owned());
        }
        if let Some(imp) = m.facts.imports.iter().find(|i| i.local.as_deref() == Some(name)) {
            if let ImportKind::Named(n) = &imp.kind
                && let Some(Target::File(t)) = resolve(graph, file, &imp.source)
                && let Some((f, c)) = find_class(graph, &t, n, 0)
            {
                return Base::Class(f, c);
            }
            return Base::External(text.to_owned());
        }
        // The builtins Pylint does not count as ancestors.
        if matches!(name, "object" | "tuple" | "dict" | "list" | "set") && py.names.binary_search(&name.to_owned()).is_err() {
            return Base::Ignored;
        }
        return Base::External(text.to_owned());
    }
    let (last, head) = segments.split_last().expect("two or more segments");
    if let Some(t) = module_of(graph, file, head)
        && let Some((f, c)) = find_class(graph, &t, last, 0)
    {
        return Base::Class(f, c);
    }
    Base::External(text.to_owned())
}

fn class_of<'g>(graph: &'g Graph, file: &str, name: &str) -> Option<&'g PyClass> {
    graph.modules.get(file)?.facts.py.as_ref()?.classes.iter().find(|c| c.top && c.name == name)
}

/// Every ancestor of a class the project's files let be read, and the bases
/// from outside the project, each counted once.
fn ancestors(graph: &Graph, file: &str, class: &PyClass) -> usize {
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    let mut external: BTreeSet<String> = BTreeSet::new();
    let own = (file.to_owned(), class.name.clone());
    let mut stack: Vec<(String, Vec<String>)> = vec![(file.to_owned(), class.bases.clone())];
    while let Some((f, bases)) = stack.pop() {
        for b in bases {
            match resolve_base(graph, &f, &b) {
                Base::Class(cf, cn) => {
                    let key = (cf.clone(), cn.clone());
                    if key != own && seen.insert(key)
                        && let Some(c) = class_of(graph, &cf, &cn)
                    {
                        stack.push((cf, c.bases.clone()));
                    }
                }
                Base::External(t) => {
                    external.insert(t);
                }
                Base::Ignored => {}
            }
        }
    }
    seen.len() + external.len()
}

/// `graph: { python: { classes: ancestors } }`: a class with more ancestors
/// than `[graph] max_ancestors` (7, Pylint's `max-parents`).
pub fn too_many_ancestors(graph: &Graph, languages: &[Language]) -> Vec<GraphFinding> {
    let max = graph.config.max_ancestors.unwrap_or(7);
    let mut out = Vec::new();
    for m in python_modules(graph, languages) {
        let py = m.facts.py.as_ref().expect("python_modules keeps modules with facts");
        for c in py.classes.iter().filter(|c| !c.bases.is_empty()) {
            let count = ancestors(graph, &m.path, c);
            if count > max {
                let (n, mx) = (count.to_string(), max.to_string());
                out.push(finding(&m.path, c.span, &[("NAME", &c.name), ("COUNT", &n), ("MAX", &mx)], None, Vec::new()));
            }
        }
    }
    out
}

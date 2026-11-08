//! The graph check of a multi-file fix (SR-MULTI-FILE-FIXES decision 2,
//! safeguard 2): every reference of the project that resolves to nothing,
//! one key each. The engine builds the graph before and after a fix; a key
//! the fix adds is a reference it left dangling, and the fix is refused.
//!
//! The references the graph knows:
//! - a relative import, `export … from` or `import()` that names no file;
//! - a named or default import of a project module that exports no such
//!   name (a module with re-exports, or none at all, is not judged);
//! - `styles.x` where the CSS Module defines no `.x`;
//! - a class token of markup or a script that no stylesheet defines,
//!   counted only while some stylesheet defines classes.

use crate::facts::{FactLang, ImportKind, SpecForm};
use crate::link::Graph;
use crate::query::{ImportCheck, Query, run};
use crate::resolve::Target;
use codelaw_core::Language;
use std::collections::BTreeSet;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

/// Every reference of the graph that resolves to nothing, as a key.
pub fn dangling(graph: &Graph) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let all = crate::query::GraphRule { query: Query::Imports { forms: vec![SpecForm::Import, SpecForm::Export, SpecForm::Dynamic, SpecForm::Require], check: ImportCheck::Missing }, fix: Default::default(), certainty_from_dynamic: false };
    for f in run(&all, graph, JS) {
        let spec = f.captures.iter().find(|(k, _)| k == "SOURCE").map(|(_, v)| v.as_str()).unwrap_or_default();
        out.insert(format!("import\0{}\0{spec}", f.file));
    }
    let classes = crate::query::GraphRule { query: Query::UndefinedModuleClasses, fix: Default::default(), certainty_from_dynamic: false };
    for f in run(&classes, graph, JS) {
        let class = f.captures.iter().find(|(k, _)| k == "CLASS").map(|(_, v)| v.as_str()).unwrap_or_default();
        out.insert(format!("module-class\0{}\0{class}", f.file));
    }
    imported_names(graph, &mut out);
    class_tokens(graph, &mut out);
    out
}

/// A named or default import of a project module that exports no such name.
fn imported_names(graph: &Graph, out: &mut BTreeSet<String>) {
    for m in graph.modules.values().filter(|m| m.facts.lang == Some(FactLang::Js)) {
        for imp in &m.facts.imports {
            let name = match &imp.kind {
                ImportKind::Named(n) => n.as_str(),
                ImportKind::Default => "default",
                _ => continue,
            };
            for t in graph.resolver.resolve_js(&m.path, &imp.source) {
                let Target::File(file) = t else { continue };
                let Some(target) = graph.modules.get(&file).filter(|t| t.facts.lang == Some(FactLang::Js)) else { continue };
                let f = &target.facts;
                // Re-exports, CommonJS and dynamic modules are not judged.
                if f.dynamic.is_some() || f.exports.is_empty() || f.exports.iter().any(|e| e.source.is_some()) {
                    continue;
                }
                if !f.exports.iter().any(|e| e.name == name) {
                    out.insert(format!("name\0{}\0{}\0{name}", m.path, imp.source));
                }
            }
        }
    }
}

/// A class token no stylesheet defines, while some stylesheet defines
/// classes: a class a rename moved from under its users.
fn class_tokens(graph: &Graph, out: &mut BTreeSet<String>) {
    let defined: BTreeSet<&str> = graph
        .modules
        .values()
        .filter(|m| m.facts.lang == Some(FactLang::Css) && !graph.config.is_css_module(&m.path))
        .flat_map(|m| m.facts.css_classes.iter().map(|c| c.name.as_str()))
        .collect();
    if defined.is_empty() {
        return;
    }
    // Only a token spelled like a class name can be one.
    let class_like = |t: &str| t.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '-') && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    for m in graph.modules.values().filter(|m| matches!(m.facts.lang, Some(FactLang::Js | FactLang::Html))) {
        for t in m.facts.class_tokens.iter().filter(|t| class_like(t)) {
            if !defined.contains(t.as_str()) {
                out.insert(format!("class\0{}\0{t}", m.path));
            }
        }
    }
}

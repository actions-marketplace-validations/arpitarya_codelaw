//! W-139 strand W-139-py, the import rows of Pylint 4.1.3 that one file
//! answers: an import that shadows another, and imports of one package split
//! by another. Behaviour only, from Pylint's functional tests; written fresh.

use crate::python_util::{kids, py_rule};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

/// One name an import statement binds, and what it takes it from.
struct Bound<'t> {
    name: &'t str,
    /// The imported entity: `pathlib.Path` for `from pathlib import Path`,
    /// `a` for `import a.b`, `a.b` for `import a.b as c`.
    source: String,
}

fn dotted(n: Node<'_>, src: &str) -> String {
    text(n, src).split_whitespace().collect()
}

fn bindings<'t>(imp: Node<'t>, src: &'t str) -> Vec<Bound<'t>> {
    let mut out = Vec::new();
    match imp.kind() {
        "import_statement" => {
            for k in kids(imp) {
                match k.kind() {
                    "dotted_name" => {
                        let full = dotted(k, src);
                        let first = full.split('.').next().unwrap_or("");
                        let name = text(k.named_child(0).unwrap_or(k), src);
                        out.push(Bound { name, source: first.to_owned() });
                    }
                    "aliased_import" => {
                        if let (Some(n), Some(a)) = (k.child_by_field_name("name"), k.child_by_field_name("alias")) {
                            out.push(Bound { name: text(a, src), source: dotted(n, src) });
                        }
                    }
                    _ => {}
                }
            }
        }
        "import_from_statement" => {
            let Some(module) = imp.child_by_field_name("module_name") else { return out };
            let m = dotted(module, src);
            for k in named_children(imp).into_iter().filter(|k| k.id() != module.id()) {
                match k.kind() {
                    "dotted_name" => out.push(Bound { name: text(k, src), source: format!("{m}.{}", dotted(k, src)) }),
                    "aliased_import" => {
                        if let (Some(n), Some(a)) = (k.child_by_field_name("name"), k.child_by_field_name("alias")) {
                            out.push(Bound { name: text(a, src), source: format!("{m}.{}", dotted(n, src)) });
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    out
}

fn line_of(src: &str, pos: usize) -> usize {
    src[..pos].matches('\n').count() + 1
}

/// The statement lists of a file: the module and every block, reached
/// through statements only (an expression holds none).
fn blocks<'t>(n: Node<'t>, out: &mut Vec<Node<'t>>) {
    out.push(n);
    for k in named_children(n) {
        if matches!(
            k.kind(),
            "block" | "if_statement" | "elif_clause" | "else_clause" | "for_statement" | "while_statement" | "try_statement" | "except_clause" | "except_group_clause" | "finally_clause" | "with_statement" | "match_statement" | "case_clause" | "function_definition" | "class_definition" | "decorated_definition"
        ) {
            blocks(k, out);
        }
    }
}

fn shadowed_import<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let mut all = Vec::new();
    blocks(n, &mut all);
    for b in all.into_iter().filter(|b| matches!(b.kind(), "module" | "block")) {
        // The first import of each name in this statement list: its entity and line.
        let mut seen: BTreeMap<&str, (String, usize)> = BTreeMap::new();
        for st in kids(b) {
            if !matches!(st.kind(), "import_statement" | "import_from_statement") {
                continue;
            }
            for bound in bindings(st, src) {
                match seen.get(bound.name) {
                    Some((source, line)) if *source != bound.source => {
                        out.push(NativeFinding {
                            start: st.start_byte(),
                            end: st.end_byte(),
                            message: format!("`{}` shadows the import of `{source}` on line {line}", bound.name),
                            edits: Vec::new(),
                        });
                    }
                    Some(_) => {}
                    None => {
                        seen.insert(bound.name, (bound.source.clone(), line_of(src, st.start_byte())));
                    }
                }
            }
        }
    }
}

py_rule!(ShadowedImport, "py/shadowed-import", ["module"], false, shadowed_import);

// ---------------------------------------------------------------- ungrouped-imports

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Group {
    Std,
    Ext,
    Local,
}

struct Imp<'t> {
    node: Node<'t>,
    package: String,
    from: bool,
    group: Group,
    guarded: bool,
}

/// Whether the import sits in a block no run executes whole: `if
/// TYPE_CHECKING:` and the `os.name` / `sys.platform` guards.
fn guarded(n: Node<'_>, src: &str) -> bool {
    let mut p = n.parent();
    while let Some(x) = p {
        if x.kind() == "if_statement"
            && let Some(c) = x.child_by_field_name("condition")
        {
            let t = text(c, src);
            if t.contains("TYPE_CHECKING") || t.contains("os.name") || t.contains("sys.platform") {
                return true;
            }
        }
        p = x.parent();
    }
    false
}

fn collect_imports<'t>(n: Node<'t>, src: &'t str, stdlib: &BTreeSet<&str>, out: &mut Vec<Imp<'t>>) {
    for k in named_children(n) {
        match k.kind() {
            "function_definition" | "class_definition" | "lambda" => continue,
            "import_statement" | "import_from_statement" => {
                let (module, from) = if k.kind() == "import_statement" {
                    (kids(k).first().map(|f| if f.kind() == "aliased_import" { f.child_by_field_name("name").unwrap_or(*f) } else { *f }), false)
                } else {
                    (k.child_by_field_name("module_name"), true)
                };
                let Some(module) = module else { continue };
                let name = dotted(module, src);
                let relative = name.starts_with('.');
                let package = if relative { String::new() } else { name.split('.').next().unwrap_or("").to_owned() };
                let group = if relative {
                    Group::Local
                } else if stdlib.contains(package.as_str()) {
                    Group::Std
                } else {
                    Group::Ext
                };
                out.push(Imp { node: k, package, from, group, guarded: guarded(k, src) });
            }
            _ => collect_imports(k, src, stdlib, out),
        }
    }
}

fn ungrouped_imports<'t>(n: Node<'t>, ctx: &NativeContext<'t>, out: &mut Vec<NativeFinding>) {
    let src = ctx.source;
    let Some(table) = ctx.data.get("python/stdlib").and_then(|v| v.as_array()) else { return };
    let stdlib: BTreeSet<&str> = table.iter().filter_map(|v| v.as_str()).collect();
    let mut imports = Vec::new();
    collect_imports(n, src, &stdlib, &mut imports);
    // Standard library first, then the rest, then relative imports, each in source order.
    imports.sort_by_key(|i| i.group);
    let mut met_import: BTreeSet<String> = BTreeSet::new();
    let mut met_from: BTreeSet<String> = BTreeSet::new();
    let mut current: Option<String> = None;
    for i in &imports {
        let met = if i.from { &mut met_from } else { &mut met_import };
        if let Some(c) = &current
            && *c != i.package
            && met.contains(&i.package)
            && !i.guarded
        {
            out.push(NativeFinding {
                start: i.node.start_byte(),
                end: i.node.end_byte(),
                message: format!("Imports from package `{}` are not grouped", i.package),
                edits: Vec::new(),
            });
        }
        current = Some(i.package.clone());
        met.insert(i.package.clone());
    }
}

py_rule!(UngroupedImports, "py/ungrouped-imports", ["module"], false, ungrouped_imports);

pub(crate) static RULES: &[&dyn NativeRule] = &[&ShadowedImport, &UngroupedImports];

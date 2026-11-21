//! W-187: Pylint's five `deprecated-*` rows (W4901 module, W4902 method,
//! W4903 argument, W4904 class, W4906 attribute), built on the data package
//! `@codelaw/data-python-deprecations` / `codelaw-data-python-deprecations`
//! (L6 decision 15; L11 D-54), generated from CPython's library reference:
//! per standard-library name, the Python version that deprecates it and the
//! one that removes it. The table comes through `FileContext::data_table`,
//! read only when one of these rules is on; without the package every rule
//! reports nothing. Behaviour only from Pylint's documentation; written
//! fresh.
//!
//! A name is reported when the project's Python target (SR-CONFIG decision
//! 11f, through `FileContext::python_target`) is at or past the version that
//! deprecates it, as Pylint's `py-version` does: a name deprecated in 3.12 is
//! fine in a project that still runs on 3.11. An unknown target reports
//! nothing (SR-RULE-FORMAT decision 2d). Names are read as the file's
//! imports resolve them (`qualified_name`): a method called on a variable
//! whose class is not known is not reported.

use crate::python_util::{call_args, hit, kids, py_rule, qualified_name};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use serde_json::Value;
use tree_sitter::Node;

/// `"3.12"` as `[3, 12]`.
fn version(s: &str) -> Option<Vec<u64>> {
    s.split('.').map(|p| p.parse().ok()).collect()
}

fn at_least(target: &[u64], v: &[u64]) -> bool {
    let part = |x: &[u64], i: usize| x.get(i).copied().unwrap_or(0);
    (0..target.len().max(v.len())).map(|i| part(target, i).cmp(&part(v, i))).find(|o| o.is_ne()).is_none_or(|o| o.is_ge())
}

/// What the table says of a name, when the target has reached its
/// deprecation: `since` and, if the documentation gives one, `removed`.
struct Hit {
    since: String,
    removed: Option<String>,
}

fn hit_from(since: &str, removed: Option<&str>, target: &[u64]) -> Option<Hit> {
    at_least(target, &version(since)?).then(|| Hit { since: since.to_owned(), removed: removed.map(str::to_owned) })
}

fn hit_of(row: &Value, target: &[u64]) -> Option<Hit> {
    let a = row.as_array()?;
    hit_from(a.first()?.as_str()?, a.get(1).and_then(Value::as_str), target)
}

fn when(h: &Hit) -> String {
    match &h.removed {
        Some(r) => format!("deprecated since Python {} and removed in {r}", h.since),
        None => format!("deprecated since Python {}", h.since),
    }
}

/// The table and the target, for a rule that asks for both.
struct Ask {
    table: std::sync::Arc<Value>,
    target: Vec<u64>,
}

fn ask(ctx: &NativeContext<'_>, rule: &str) -> Option<Ask> {
    let table = ctx.file.data_table("python-deprecations")?;
    let target = ctx.file.python_target(rule)?;
    Some(Ask { table, target })
}

impl Ask {
    fn row(&self, section: &str, name: &str) -> Option<Hit> {
        hit_of(self.table.get(section)?.get(name)?, &self.target)
    }
}

// ----------------------------------------------------------------- module

/// The dotted name an `import` names, without `as`.
fn module_name(n: Node<'_>, src: &str) -> String {
    text(n, src).split_whitespace().collect()
}

fn deprecated_module(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(a) = ask(ctx, "py/deprecated-module") else { return };
    let src = ctx.source;
    let mut names: Vec<Node<'_>> = Vec::new();
    match n.kind() {
        "import_statement" => {
            for k in kids(n) {
                match k.kind() {
                    "dotted_name" => names.push(k),
                    "aliased_import" => names.extend(k.child_by_field_name("name")),
                    _ => {}
                }
            }
        }
        "import_from_statement" => names.extend(n.child_by_field_name("module_name").filter(|m| m.kind() == "dotted_name")),
        _ => {}
    }
    for m in names {
        let full = module_name(m, src);
        // `import distutils.core` is the module `distutils`'s: the longest known prefix.
        let parts: Vec<&str> = full.split('.').collect();
        let found = (1..=parts.len()).rev().find_map(|i| {
            let prefix = parts[..i].join(".");
            a.row("modules", &prefix).map(|h| (prefix, h))
        });
        if let Some((name, h)) = found {
            out.push(hit(m, format!("The module `{name}` is {}", when(&h))));
        }
    }
}
py_rule!(DeprecatedModule, "py/deprecated-module", ["import_statement", "import_from_statement"], false, deprecated_module);

// ------------------------------------------------- class and attribute

/// Names an import-from or an attribute chain resolves to, against one section.
fn deprecated_name(section: &'static str, rule: &'static str, noun: &'static str, n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(a) = ask(ctx, rule) else { return };
    let src = ctx.source;
    match n.kind() {
        "import_from_statement" => {
            let Some(module) = n.child_by_field_name("module_name").filter(|m| m.kind() == "dotted_name") else { return };
            let module = module_name(module, src);
            for k in named_children(n).into_iter().filter(|k| n.child_by_field_name("module_name").is_none_or(|m| m.id() != k.id())) {
                let name = match k.kind() {
                    "dotted_name" => k,
                    "aliased_import" => match k.child_by_field_name("name") {
                        Some(x) => x,
                        None => continue,
                    },
                    _ => continue,
                };
                let full = format!("{module}.{}", module_name(name, src));
                if let Some(h) = a.row(section, &full) {
                    out.push(hit(k, format!("The {noun} `{full}` is {}", when(&h))));
                }
            }
        }
        "attribute" => {
            let Some(model) = ctx.semantic else { return };
            let Some(q) = qualified_name(n, model, src) else { return };
            if let Some(h) = a.row(section, &q) {
                out.push(hit(n, format!("The {noun} `{q}` is {}", when(&h))));
            }
        }
        _ => {}
    }
}

fn deprecated_class(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    deprecated_name("classes", "py/deprecated-class", "class", n, ctx, out);
}
py_rule!(DeprecatedClass, "py/deprecated-class", ["import_from_statement", "attribute"], true, deprecated_class);

fn deprecated_attribute(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    deprecated_name("attributes", "py/deprecated-attribute", "attribute", n, ctx, out);
}
py_rule!(DeprecatedAttribute, "py/deprecated-attribute", ["import_from_statement", "attribute"], true, deprecated_attribute);

// ------------------------------------------------- method and argument

/// The qualified name a call's callee resolves to: the file's imports, or a
/// class built on the spot (`threading.Thread(…).getName()`).
fn callee(call: Node<'_>, ctx: &NativeContext<'_>) -> Option<String> {
    let model = ctx.semantic?;
    let f = call.child_by_field_name("function")?;
    if let Some(q) = qualified_name(f, model, ctx.source) {
        return Some(q);
    }
    if f.kind() == "attribute" {
        let obj = f.child_by_field_name("object")?;
        if obj.kind() == "call" {
            let class = qualified_name(obj.child_by_field_name("function")?, model, ctx.source)?;
            return Some(format!("{class}.{}", text(f.child_by_field_name("attribute")?, ctx.source)));
        }
    }
    None
}

fn deprecated_method(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(a) = ask(ctx, "py/deprecated-method") else { return };
    let Some(q) = callee(n, ctx) else { return };
    if let Some(h) = a.row("functions", &q)
        && let Some(f) = n.child_by_field_name("function")
    {
        out.push(hit(f, format!("The function `{q}` is {}", when(&h))));
    }
}
py_rule!(DeprecatedMethod, "py/deprecated-method", ["call"], true, deprecated_method);

fn deprecated_argument(n: Node<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
    let Some(a) = ask(ctx, "py/deprecated-argument") else { return };
    let Some(q) = callee(n, ctx) else { return };
    let Some(rows) = a.table.get("arguments").and_then(|t| t.get(&q)).and_then(Value::as_array) else { return };
    let args = call_args(n, ctx.source);
    for row in rows {
        let Some(r) = row.as_array() else { continue };
        let (Some(name), pos, Some(since)) = (r.first().and_then(Value::as_str), r.get(1).and_then(Value::as_u64), r.get(2).and_then(Value::as_str)) else { continue };
        let Some(h) = hit_from(since, r.get(3).and_then(Value::as_str), &a.target) else { continue };
        // By keyword; by position only while no `*args` makes the position uncertain.
        let at = args.keywords.iter().find(|k| k.0 == name).map(|k| k.2).or_else(|| match pos {
            Some(p) if !args.starred => args.positional.get(p as usize).copied(),
            _ => None,
        });
        if let Some(arg) = at {
            out.push(hit(arg, format!("The argument `{name}` of `{q}` is {}", when(&h))));
        }
    }
}
py_rule!(DeprecatedArgument, "py/deprecated-argument", ["call"], true, deprecated_argument);

pub(crate) static RULES: &[&dyn NativeRule] = &[&DeprecatedModule, &DeprecatedClass, &DeprecatedAttribute, &DeprecatedMethod, &DeprecatedArgument];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_target_reaches_a_version_by_number_not_by_text() {
        assert!(at_least(&[3, 12], &[3, 9]));
        assert!(at_least(&[3, 9], &[3, 9]));
        assert!(!at_least(&[3, 9], &[3, 12]));
        assert!(at_least(&[3, 10], &[3, 9]), "3.10 is past 3.9");
        assert!(at_least(&[3], &[3, 0]));
    }

    #[test]
    fn a_row_is_a_hit_only_from_its_version() {
        let row = serde_json::json!(["3.12", null]);
        assert!(hit_of(&row, &[3, 11]).is_none());
        let h = hit_of(&row, &[3, 12]).unwrap();
        assert_eq!(when(&h), "deprecated since Python 3.12");
        let gone = hit_of(&serde_json::json!(["3.6", "3.12"]), &[3, 13]).unwrap();
        assert_eq!(when(&gone), "deprecated since Python 3.6 and removed in 3.12");
    }
}

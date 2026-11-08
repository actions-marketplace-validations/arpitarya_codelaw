//! W-175: the queries that read another module's exports, on virtual repos.

use crate::*;
use codelaw_core::Language;
use serde_json::json;
use std::path::Path;

fn graph(files: &[(&str, &str)]) -> Graph {
    let mut inputs = Vec::new();
    let mut manifests = Vec::new();
    for (p, t) in files {
        let name = p.rsplit('/').next().unwrap();
        if is_manifest(name) {
            manifests.push(Manifest { path: p.to_string(), text: t.to_string() });
        }
        let facts = Language::from_path(Path::new(p)).and_then(|l| extract(l, t));
        inputs.push(GraphInput { path: p.to_string(), facts });
    }
    Graph::build(inputs, &manifests, GraphConfig::default())
}

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

/// (file, the text the finding covers, its edits as start, end, text)
type Hit = (String, String, Vec<(usize, usize, String)>);

fn run_query(files: &[(&str, &str)], g: serde_json::Value, fix: Option<serde_json::Value>) -> Vec<Hit> {
    let rule = compile(&g, fix.as_ref()).unwrap();
    let gr = graph(files);
    run(&rule, &gr, JS)
        .into_iter()
        .map(|f| {
            let text = files.iter().find(|(p, _)| *p == f.file).unwrap().1;
            let at = text.get(f.span.0..f.span.1).unwrap_or("").to_owned();
            (f.file.clone(), at, f.edits.iter().map(|e| (e.start, e.end, e.text.clone())).collect())
        })
        .collect()
}

fn hits(files: &[(&str, &str)], g: serde_json::Value) -> Vec<String> {
    run_query(files, g, None).into_iter().map(|(f, at, _)| format!("{f}:{at}")).collect()
}

#[test]
fn a_named_import_must_be_exported() {
    let files = [
        ("src/lib.js", "export const alpha = 1;\nexport function beta() {}\nexport * from './more';\n"),
        ("src/more.js", "export const gamma = 3;\n"),
        ("src/app.js", "import { alpha, gamma, alphx, nope as other } from './lib';\nexport { beta2 } from './lib';\nconsole.log(alpha, gamma, alphx, other, beta2);\n"),
    ];
    let q = json!({"import": {"names": "unexported"}});
    let got = hits(&files, q.clone());
    assert_eq!(got, vec!["src/app.js:alphx", "src/app.js:nope as other", "src/app.js:export { beta2 } from './lib';"]);
    // The nearest name keeps the local binding.
    let fix = json!({"do": ["nearest-export"]});
    let with_fix = run_query(&files, q, Some(fix));
    let alphx = &with_fix[0];
    assert_eq!(alphx.2, vec![(files[2].1.find("alphx").unwrap(), files[2].1.find("alphx").unwrap() + 5, "alpha as alphx".to_owned())]);
    assert!(with_fix[1].2.is_empty(), "`nope` is near nothing");
}

#[test]
fn what_is_not_known_is_not_reported() {
    // CommonJS, an unresolved package and an unreadable `export *` say nothing.
    let files = [
        ("src/cjs.js", "module.exports = { a: 1 };\n"),
        ("src/open.js", "export * from 'somewhere';\nexport const here = 1;\n"),
        ("src/app.js", "import { a } from './cjs';\nimport { elsewhere } from './open';\nimport { zip } from 'zip';\nimport { here } from './open';\nconsole.log(a, elsewhere, zip, here);\n"),
    ];
    assert!(hits(&files, json!({"import": {"names": "unexported"}})).is_empty());
    assert!(hits(&files, json!({"import": {"default": "missing"}})).is_empty());
}

#[test]
fn a_default_import_needs_a_default_export() {
    let files = [
        ("src/a.js", "export const a = 1;\n"),
        ("src/b.js", "export default 2;\n"),
        ("src/c.js", "export * from './b';\n"),
        ("src/app.js", "import a from './a';\nimport b from './b';\nimport c from './c';\nconsole.log(a, b, c);\n"),
    ];
    // `export *` does not pass the default on.
    assert_eq!(hits(&files, json!({"import": {"default": "missing"}})), vec!["src/app.js:a", "src/app.js:c"]);
}

#[test]
fn a_namespace_member_must_be_exported() {
    let files = [
        ("src/lib.js", "export const one = 1;\nexport const two = 2;\n"),
        ("src/empty.js", "import './lib';\n"),
        ("src/app.js", "import * as lib from './lib';\nimport * as none from './empty';\nconsole.log(lib.one, lib.three, lib.twoo);\nconst { two, four } = lib;\nconsole.log(none, two, four);\n"),
    ];
    let q = json!({"import": {"namespace": "unexported"}});
    assert_eq!(hits(&files, q.clone()), vec!["src/app.js:three", "src/app.js:twoo", "src/app.js:four", "src/app.js:* as none"]);
    let fixed = run_query(&files, q, Some(json!({"do": ["nearest-export"]})));
    let twoo = fixed.iter().find(|f| f.1 == "twoo").unwrap();
    assert_eq!(twoo.2[0].2, "two");
    assert!(fixed.iter().find(|f| f.1 == "three").unwrap().2.is_empty());
}

#[test]
fn a_default_import_named_like_a_named_export() {
    let files = [
        ("src/lib.js", "export const foo = 1;\nexport default function main() {}\n"),
        ("src/same.js", "function same() {}\nexport { same };\nexport default same;\n"),
        ("src/app.js", "import foo from './lib';\nimport same from './same';\nimport other from './lib';\nconsole.log(foo, same, other);\n"),
    ];
    // `same` is one binding exported twice; `other` is not an export's name.
    assert_eq!(hits(&files, json!({"import": {"default_name": "named"}})), vec!["src/app.js:foo"]);
}

#[test]
fn a_member_of_a_default_import_that_is_a_named_export() {
    let files = [
        ("src/lib.js", "export const find = 1;\nexport default { find, other: 2 };\n"),
        ("src/app.js", "import lib from './lib';\nconsole.log(lib.find, lib.other, lib.default);\nconst { find, other } = lib;\nconsole.log(find, other);\n"),
    ];
    assert_eq!(hits(&files, json!({"import": {"default_member": "named"}})), vec!["src/app.js:find", "src/app.js:find"]);
}

#[test]
fn a_default_import_goes_by_the_exports_own_name() {
    let files = [
        ("src/fn.js", "export default function make() {}\n"),
        ("src/id.js", "const widget = 1;\nexport default widget;\n"),
        ("src/call.js", "import { connect } from 'x';\nfunction Panel() {}\nexport default connect(Panel);\n"),
        ("src/anon.js", "export default () => 1;\n"),
        ("src/list.js", "const item = 1;\nexport { item as default };\n"),
        ("src/app.js", "import make from './fn';\nimport thing from './id';\nimport Wrapped from './call';\nimport anon from './anon';\nimport row from './list';\nimport { default as made } from './fn';\nconsole.log(make, thing, Wrapped, anon, row, made);\n"),
    ];
    assert_eq!(hits(&files, json!({"import": {"default_name": "differs"}})), vec!["src/app.js:thing", "src/app.js:Wrapped", "src/app.js:row", "src/app.js:default as made"]);
}

#[test]
fn a_deprecated_export_is_reported_where_it_is_imported_and_used() {
    let files = [
        ("src/lib.js", "/** @deprecated use fresh */\nexport function old() {}\nexport function fresh() {}\n/** @deprecated */\nexport default function legacy() {}\n"),
        ("src/app.js", "import legacy, { old, fresh } from './lib';\nimport * as lib from './lib';\nold();\nfresh();\nlegacy();\nlib.old();\n"),
    ];
    let q = json!({"import": {"deprecated": "jsdoc"}});
    let rule = compile(&q, None).unwrap();
    let g = graph(&files);
    let got = run(&rule, &g, JS);
    let messages: Vec<String> = got.iter().map(|f| format!("{} {}", &files[1].1[f.span.0..f.span.1], f.render(""))).collect();
    assert!(messages.contains(&"old Deprecated: use fresh".to_owned()), "{messages:?}");
    assert!(messages.contains(&"legacy Deprecated.".to_owned()), "{messages:?}");
    // the import, the call, and `lib.old`; `fresh` is not deprecated
    assert_eq!(got.iter().filter(|f| f.render("").contains("use fresh")).count(), 3);
    assert!(!messages.iter().any(|m| m.starts_with("fresh")));
}

#[test]
fn a_cycle_is_reported_at_each_import_on_it() {
    let files = [
        ("src/a.js", "import { b } from './b';\nexport const a = b;\n"),
        ("src/b.js", "import { c } from './c';\nexport const b = c;\n"),
        ("src/c.js", "import { a } from './a';\nexport const c = a;\nimport('./d');\n"),
        ("src/d.js", "export const d = 1;\n"),
        ("src/t.ts", "import type { T } from './u';\nexport const t = 1;\n"),
        ("src/u.ts", "import { t } from './t';\nexport type T = typeof t;\n"),
    ];
    let q = json!({"cycle": {}});
    let rule = compile(&q, None).unwrap();
    let g = graph(&files);
    let got = run(&rule, &g, JS);
    let at: Vec<&str> = got.iter().map(|f| f.file.as_str()).collect();
    assert_eq!(at, vec!["src/a.js", "src/b.js", "src/c.js"], "the type-only import closes no cycle");
    assert_eq!(got[0].render(""), "Dependency cycle via b.js -> c.js -> a.js");
    // `form: [require]` alone sees no edge.
    let none = compile(&json!({"cycle": {"form": ["require"]}}), None).unwrap();
    assert!(run(&none, &g, JS).is_empty());
}

#[test]
fn export_star_conflicts_across_modules() {
    let files = [
        ("src/a.js", "export const x = 1;\nexport const only_a = 1;\n"),
        ("src/b.js", "export const x = 2;\n"),
        ("src/none.js", "export default 1;\n"),
        ("src/index.js", "export * from './a';\nexport * from './b';\nexport * from './none';\nexport const only_a = 0;\n"),
        ("src/ok.js", "export * from './a';\nexport const mine = 1;\n"),
    ];
    let q = json!({"export": {"star": "conflicts"}});
    let rule = compile(&q, None).unwrap();
    let g = graph(&files);
    let got = run(&rule, &g, JS);
    let found: Vec<(String, String)> = got.iter().map(|f| (f.file.clone(), files[3].1.get(f.span.0..f.span.1).unwrap_or("").to_owned())).collect();
    assert!(found.iter().all(|(f, _)| f == "src/index.js"));
    let texts: Vec<&str> = found.iter().map(|(_, t)| t.as_str()).collect();
    assert!(texts.contains(&"export * from './a';") && texts.contains(&"export * from './b';"), "{texts:?}");
    assert!(texts.contains(&"export const only_a = 0;"), "an own export against a star one: {texts:?}");
    assert!(got.iter().any(|f| f.render("").contains("no named exports")));
}

#[test]
fn modules_with_no_exports_or_exports_nothing_reads() {
    let files = [
        ("src/main.js", "import { used } from './lib';\nimport * as ns from './space';\nimport './script';\nconsole.log(used, ns.read);\n"),
        ("src/lib.js", "export const used = 1;\nexport const idle = 2;\nexport default 3;\n"),
        ("src/space.js", "export const read = 1;\nexport const unread = 2;\n"),
        ("src/script.js", "console.log('side effect');\n"),
        ("src/barrel.js", "export * from './lib';\n"),
    ];
    let both = json!({"module": {"exports": ["missing", "unused"]}});
    let fix = json!([{"when": {"local_references": ">0"}, "do": ["strip-export-keyword"]}, {"else": null, "do": ["remove-declaration"]}]);
    let rule = compile(&both, Some(&fix)).unwrap();
    let g = graph(&files);
    let found = |r: &GraphRule| -> Vec<String> {
        let mut v: Vec<String> = run(r, &g, JS).into_iter().map(|f| format!("{} {}", f.file, f.message.clone().or_else(|| f.captures.first().map(|c| c.1.clone())).unwrap_or_default())).collect();
        v.sort();
        v
    };
    // No entry point reads `main.js` or `script.js`, so both lack exports; `default` of lib is read by nobody.
    assert_eq!(found(&rule), vec!["src/lib.js default", "src/lib.js idle", "src/main.js No exports found", "src/script.js No exports found", "src/space.js unread"]);
    // `export *` passes the names of lib on, as barrel is itself unread: `default` stays unread.
    let only_unused = compile(&json!({"module": {"exports": ["unused"]}}), Some(&fix)).unwrap();
    assert!(!found(&only_unused).iter().any(|f| f.contains("No exports")));
    // The unused `idle` is removed; the file is the only edit's owner.
    let idle = run(&only_unused, &g, JS).into_iter().find(|f| f.captures.first().is_some_and(|c| c.1 == "idle")).unwrap();
    assert_eq!(idle.edits.len(), 1);
}

#[test]
fn the_new_queries_take_only_their_own_fix() {
    let bad = compile(&json!({"cycle": {}}), Some(&json!({"do": ["nearest-export"]})));
    assert!(bad.is_err());
    let ok = compile(&json!({"import": {"names": "unexported"}}), Some(&json!({"do": ["nearest-export"]})));
    assert!(ok.is_ok());
    assert!(compile(&json!({"import": {"names": "unexported", "default": "missing"}}), None).is_err());
}

/// L8 evidence for the one fact W-175 adds (`default_name`): the time of
/// `extract` on a module-sized file. Run with `--ignored --nocapture`.
#[test]
#[ignore]
fn the_default_name_fact_costs_nothing_measurable() {
    let mut text = String::from("import { connect } from 'x';\n");
    for i in 0..400 {
        text.push_str(&format!("export function f{i}(a, b) {{ return a + b + {i}; }}\nexport const c{i} = {{ k: {i} }};\n"));
    }
    text.push_str("export default connect(f1);\n");
    let start = std::time::Instant::now();
    let mut names = 0;
    for _ in 0..50 {
        names += extract(Language::Javascript, &text).unwrap().default_name.is_some() as usize;
    }
    eprintln!("extract x50 of an 800-statement file: {:?} ({names} default names)", start.elapsed());
}

//! W-185: the Python queries, on virtual repos.

use crate::*;
use codelaw_core::Language;
use serde_json::json;
use std::path::Path;

fn graph_with(files: &[(&str, &str)], config: GraphConfig) -> Graph {
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
    Graph::build(inputs, &manifests, config)
}

/// `file:text covered by the finding`, and the message when it is the query's own.
fn hits_with(files: &[(&str, &str)], g: serde_json::Value, config: GraphConfig) -> Vec<String> {
    let rule = compile(&g, None).unwrap();
    let gr = graph_with(files, config);
    run(&rule, &gr, &[Language::Python])
        .into_iter()
        .map(|f| {
            let text = files.iter().find(|(p, _)| *p == f.file).unwrap().1;
            let at = text.get(f.span.0..f.span.1).unwrap_or("").to_owned();
            format!("{}:{}", f.file, at.lines().next().unwrap_or(""))
        })
        .collect()
}

fn hits(files: &[(&str, &str)], g: serde_json::Value) -> Vec<String> {
    hits_with(files, g, GraphConfig::default())
}

fn messages(files: &[(&str, &str)], g: serde_json::Value) -> Vec<String> {
    let rule = compile(&g, None).unwrap();
    let gr = graph_with(files, GraphConfig::default());
    run(&rule, &gr, &[Language::Python]).into_iter().map(|f| f.render("$SOURCE|$NAME")).collect()
}

fn q(kind: &str, value: &str) -> serde_json::Value {
    json!({"python": {kind: value}})
}

#[test]
fn a_cycle_through_python_imports() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/a.py", "from pkg import b\n\ndef fa():\n    return b.fb\n"),
        ("pkg/b.py", "from .a import fa\n\ndef fb():\n    return fa\n"),
        ("pkg/c.py", "import pkg.a\n"),
    ];
    let g = json!({"cycle": {"form": ["import"]}});
    assert_eq!(hits(&files, g.clone()), vec!["pkg/a.py:from pkg import b", "pkg/b.py:from .a import fa"]);
    let m = messages(&files, g);
    assert_eq!(m[0], "Cyclic import (pkg.a -> pkg.b -> pkg.a)");
}

#[test]
fn a_type_checking_import_closes_no_cycle() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/a.py", "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n    from pkg.b import B\n"),
        ("pkg/b.py", "from pkg.a import x\n"),
    ];
    assert!(hits(&files, json!({"cycle": {"form": ["import"]}})).is_empty());
    // An import in the `else` branch runs.
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/a.py", "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n    pass\nelse:\n    from pkg.b import B\n"),
        ("pkg/b.py", "from pkg.a import x\n"),
    ];
    assert_eq!(hits(&files, json!({"cycle": {"form": ["import"]}})).len(), 2);
}

#[test]
fn an_import_of_a_missing_module_of_the_project() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/real.py", "X = 1\n"),
        ("pkg/app.py", "import pkg.real\nimport pkg.gone\nfrom pkg.missing import a, b\nfrom .nothing import c\nfrom . import real\nimport os.path\nimport requests\nimport requests.adapters\n"),
    ];
    let got = hits(&files, q("imports", "unresolved"));
    assert_eq!(got, vec!["pkg/app.py:import pkg.gone", "pkg/app.py:from pkg.missing import a, b", "pkg/app.py:from .nothing import c"]);
}

#[test]
fn an_import_the_project_cannot_judge_is_not_reported() {
    let files = [
        ("pkg/__init__.py", ""),
        ("ns/inner.py", "X = 1\n"),
        ("pkg/app.py", concat!(
            "try:\n    import pkg.optional\nexcept ImportError:\n    pass\n",
            "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n    from pkg.typed import T\n",
            "import ns.other\n",
        )),
        ("pkg/hack.py", "import sys\nsys.path.insert(0, 'x')\nimport pkg.late\n"),
    ];
    // `ns` is a folder without `__init__.py`: other path entries may add to it.
    assert!(hits(&files, q("imports", "unresolved")).is_empty());
}

#[test]
fn a_src_layout_and_a_sibling_script_resolve() {
    let files = [
        ("pyproject.toml", "[project]\nname = \"x\"\n"),
        ("src/lib/__init__.py", ""),
        ("src/lib/core.py", "A = 1\n"),
        ("tests/test_core.py", "import lib.core\nfrom lib import core\nimport helper\nimport lib.core.deep\n"),
        ("tests/helper.py", "H = 1\n"),
    ];
    assert_eq!(hits(&files, q("imports", "unresolved")), vec!["tests/test_core.py:import lib.core.deep"]);
}

#[test]
fn a_name_a_module_does_not_bind() {
    let files = [
        ("pkg/__init__.py", "from .sub import sub_fn\nimport pkg.deep\n"),
        ("pkg/sub.py", "def sub_fn(): ...\nclass K: ...\nif True:\n    COND = 1\nfor loop_var in []:\n    pass\n"),
        ("pkg/deep.py", "D = 1\n"),
        ("pkg/app.py", "from pkg.sub import sub_fn, K, COND, loop_var, nope\nfrom pkg import sub_fn, deep, sub, absent\nfrom . import K2\nfrom os import path, nothing\n"),
    ];
    let m = messages(&files, q("imports", "unexported"));
    let got = hits(&files, q("imports", "unexported"));
    assert_eq!(got, vec!["pkg/app.py:nope", "pkg/app.py:absent", "pkg/app.py:K2"]);
    assert_eq!(m.len(), 3);
}

#[test]
fn a_module_with_unknown_names_is_not_judged() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/lazy.py", "def __getattr__(name):\n    return name\n"),
        ("pkg/glob.py", "globals()['made'] = 1\n"),
        ("pkg/star.py", "from os.path import *\n"),
        ("pkg/app.py", "from pkg.lazy import anything\nfrom pkg.glob import made\nfrom pkg.star import join\n"),
    ];
    assert_eq!(hits(&files, q("imports", "unexported")), Vec::<String>::new());
}

#[test]
fn a_star_import_of_the_project_brings_its_names() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/base.py", "def one(): ...\ndef two(): ...\n"),
        ("pkg/wide.py", "from pkg.base import *\nX = 1\n"),
        ("pkg/app.py", "from pkg.wide import one, X, three\n"),
    ];
    assert_eq!(hits(&files, q("imports", "unexported")), vec!["pkg/app.py:three"]);
}

#[test]
fn a_relative_import_beyond_the_top_level() {
    let files = [
        ("top/__init__.py", ""),
        ("top/mid/__init__.py", ""),
        ("top/mid/leaf.py", "from . import x\nfrom .. import y\nfrom ... import z\nfrom ...far import w\n"),
        ("script.py", "from . import sibling\n"),
        ("loose/mod.py", "from .x import y\n"),
    ];
    assert_eq!(
        hits(&files, q("imports", "beyond_top")),
        vec!["loose/mod.py:from .x import y", "script.py:from . import sibling", "top/mid/leaf.py:from ... import z", "top/mid/leaf.py:from ...far import w"]
    );
    // Beyond the top level is that rule's finding alone, not an unresolved import too.
    assert!(hits(&files, q("imports", "unresolved")).iter().all(|h| !h.starts_with("script.py") && !h.starts_with("loose/")));
}

#[test]
fn unused_names_of_a_wildcard_import() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/consts.py", "ALPHA = 1\nBETA = 2\nGAMMA = 3\n_hidden = 4\n"),
        ("pkg/listed.py", "__all__ = ['one', 'two']\ndef one(): ...\ndef two(): ...\ndef three(): ...\n"),
        ("pkg/app.py", "from pkg.consts import *\nfrom pkg.listed import *\n\nprint(ALPHA, one)\n"),
        ("pkg/all_used.py", "from pkg.consts import *\n\nprint(ALPHA, BETA, GAMMA)\n"),
        ("pkg/stringy.py", "from pkg.consts import *\n\nx: 'BETA' = 0\nprint(ALPHA, GAMMA)\n"),
        ("pkg/__init__star.py", "X = 1\n"),
    ];
    let m = messages(&files, q("imports", "wildcard_unused"));
    assert_eq!(m, vec!["Unused `BETA`, `GAMMA` from wildcard import of `pkg.consts`", "Unused `two` from wildcard import of `pkg.listed`"]);
    // `__init__.py` re-exports on purpose.
    let files = [("pkg/__init__.py", "from pkg.consts import *\n"), ("pkg/consts.py", "A = 1\n")];
    assert!(hits(&files, q("imports", "wildcard_unused")).is_empty());
}

#[test]
fn a_wildcard_import_of_a_module_outside_the_project_is_not_judged() {
    let files = [
        ("app.py", "from os.path import *\nfrom tkinter import *\nfrom dyn import *\n"),
        ("dyn.py", "__all__ = ['a'] + other.__all__\n"),
    ];
    assert!(hits(&files, q("imports", "wildcard_unused")).is_empty());
}

#[test]
fn ancestors_across_files() {
    let files = [
        ("pkg/__init__.py", ""),
        ("pkg/base.py", "class A: ...\nclass B(A): ...\nclass C(B): ...\n"),
        ("pkg/more.py", "from pkg.base import C\nclass D(C): ...\nclass E(D): ...\n"),
        ("pkg/mix.py", "import pkg.more\nimport pkg.base as base\nfrom pkg import more\nclass F(pkg.more.E): ...\nclass G(F, base.A): ...\nclass H(G, more.D): ...\n"),
    ];
    // H's ancestors: G, F, E, D, C, B, A — seven.
    let cfg = GraphConfig { max_ancestors: Some(6), ..Default::default() };
    assert_eq!(hits_with(&files, json!({"python": {"classes": "ancestors"}}), cfg.clone()), vec!["pkg/mix.py:H"]);
    let rule = compile(&json!({"python": {"classes": "ancestors"}}), None).unwrap();
    let gr = graph_with(&files, cfg);
    let f = run(&rule, &gr, &[Language::Python]);
    assert_eq!(f[0].captures.iter().find(|(k, _)| k == "COUNT").unwrap().1, "7");
    // The default is Pylint's seven: seven are not too many.
    assert!(hits(&files, json!({"python": {"classes": "ancestors"}})).is_empty());
}

#[test]
fn an_outside_base_counts_once_and_object_not_at_all() {
    let files = [("a.py", "class P(object, Exception, ValueError, Exception): ...\nclass Q(P, dict): ...\n")];
    let cfg = GraphConfig { max_ancestors: Some(2), ..Default::default() };
    let got = hits_with(&files, json!({"python": {"classes": "ancestors"}}), cfg);
    // P has Exception and ValueError; Q has P as well.
    assert_eq!(got, vec!["a.py:Q"]);
}

#[test]
fn a_class_that_names_itself_does_not_loop() {
    let files = [("a.py", "class A(B): ...\nclass B(A): ...\n")];
    let cfg = GraphConfig { max_ancestors: Some(0), ..Default::default() };
    assert_eq!(hits_with(&files, json!({"python": {"classes": "ancestors"}}), cfg).len(), 2);
}

#[test]
fn the_facts_do_not_change_with_the_cache() {
    // A cached facts value reads back as the one extracted.
    let text = "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n    from a import B\nimport c\nclass K(B, c.D): ...\n";
    let f = extract(Language::Python, text).unwrap();
    let back: FileFacts = serde_json::from_str(&serde_json::to_string(&f).unwrap()).unwrap();
    assert_eq!(f, back);
    let py = f.py.unwrap();
    assert_eq!(py.classes[0].bases, vec!["B", "c.D"]);
    assert!(f.imports[1].type_checking && f.imports[1].guarded && !f.imports[0].guarded && !f.imports[2].guarded);
}

/// L8 measurement (W-185), not a check: `CODELAW_PY_SAMPLE=<dir> cargo test
/// -p codelaw-graph --release measure_python_facts -- --ignored --nocapture`.
#[test]
#[ignore]
fn measure_python_facts() {
    use std::time::Instant;
    let Ok(dir) = std::env::var("CODELAW_PY_SAMPLE") else { return };
    let mut files = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(dir)];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                if !p.ends_with("test") && !p.ends_with("site-packages") {
                    stack.push(p);
                }
            } else if p.extension().is_some_and(|x| x == "py")
                && let Ok(t) = std::fs::read_to_string(&p)
            {
                files.push(t);
            }
        }
    }
    let bytes: usize = files.iter().map(String::len).sum();
    let t = Instant::now();
    let mut n = 0;
    for f in &files {
        n += extract(Language::Python, f).map(|x| x.imports.len()).unwrap_or(0);
    }
    let extract_all = t.elapsed();
    let t = Instant::now();
    for f in &files {
        let root = codelaw_lang::parse(Language::Python, f);
        let _ = codelaw_semantic::build(Language::Python, &root, f);
    }
    let parse_and_model = t.elapsed();
    let t = Instant::now();
    let mut size = 0;
    for f in &files {
        if let Some(x) = extract(Language::Python, f) {
            size += serde_json::to_string(&x).unwrap().len();
            size -= serde_json::to_string(&FileFacts { py: None, ..x }).unwrap().len();
        }
    }
    println!("{} files, {} KiB of source, {} imports: extract {:?}, parse+semantic model alone {:?}; py facts add {} KiB of cache over {} KiB", files.len(), bytes / 1024, n, extract_all, parse_and_model, size / 1024, 0);
    let _ = t;
}

#[test]
fn star_imports_that_reach_each_other_are_followed_once_each() {
    // Six modules star-import the five others, and each re-exports a name
    // none of them defines: the search for it tried every star at every level
    // to the depth cap, 5^16 steps (W-185: a few Python `__init__` modules
    // took minutes in `graph/no-unused-exports`). A (module, name) is now
    // entered once.
    let mut files: Vec<(String, String)> = vec![("pkg/__init__.py".into(), String::new())];
    for i in 0..6 {
        let mut t = String::new();
        for j in (0..6).filter(|j| *j != i) {
            t.push_str(&format!("from .m{j} import *\n"));
        }
        t.push_str(&format!("from .m{} import ghost\n__all__ = ['ghost', 'f{i}']\n\n\ndef f{i}():\n    return 1\n", (i + 1) % 6));
        files.push((format!("pkg/m{i}.py"), t));
    }
    let refs: Vec<(&str, &str)> = files.iter().map(|(p, t)| (p.as_str(), t.as_str())).collect();
    let rule = compile(&json!({"symbol": {"exported": true, "kind": ["function"]}, "when": {"external_references": 0}}), None).unwrap();
    let gr = graph_with(&refs, GraphConfig::default());
    let started = std::time::Instant::now();
    let _ = run(&rule, &gr, &[Language::Python]);
    assert!(started.elapsed().as_secs() < 20, "took {:?}", started.elapsed());
}

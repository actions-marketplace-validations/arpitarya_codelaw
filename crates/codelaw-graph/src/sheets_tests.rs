//! W-199: global stylesheets against the files that load with them, and
//! `@private` / `@package` imports, on virtual repos.

use crate::*;
use codelaw_core::Language;
use serde_json::json;
use std::path::Path;

fn graph(files: &[(&str, &str)]) -> Graph {
    let mut inputs = Vec::new();
    for (p, t) in files {
        let facts = Language::from_path(Path::new(p)).and_then(|l| extract(l, t));
        inputs.push(GraphInput { path: p.to_string(), facts });
    }
    Graph::build(inputs, &[], GraphConfig::default())
}

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const CSS: &[Language] = &[Language::Css, Language::Scss];

fn hits(files: &[(&str, &str)], g: serde_json::Value, languages: &[Language]) -> Vec<String> {
    let rule = compile(&g, None).unwrap();
    run(&rule, &graph(files), languages)
        .into_iter()
        .map(|f| {
            let text = files.iter().find(|(p, _)| *p == f.file).unwrap().1;
            format!("{}:{}", f.file, text.get(f.span.0..f.span.1).unwrap_or(""))
        })
        .collect()
}

fn undeclared(files: &[(&str, &str)]) -> Vec<String> {
    hits(files, json!({"sheets": {"classes": "undeclared"}}), JS)
}
fn unused(files: &[(&str, &str)]) -> Vec<String> {
    hits(files, json!({"sheets": {"classes": "unused"}}), CSS)
}
fn props(files: &[(&str, &str)]) -> Vec<String> {
    hits(files, json!({"sheets": {"properties": "undeclared"}}), CSS)
}

#[test]
fn a_class_no_loaded_stylesheet_declares_is_reported() {
    let files = [
        ("src/Card.css", ".card { padding: 1rem; }\n.card-title { color: red; }\n"),
        ("src/Card.jsx", "import './Card.css';\nexport const C = () => <div className=\"card card-titel\"><b className={'card-title'} /></div>;\n"),
    ];
    assert_eq!(undeclared(&files), vec!["src/Card.jsx:card-titel"]);
}

#[test]
fn the_stylesheets_of_the_entry_count_for_a_component() {
    // `index.css` is loaded by main.jsx, which loads App, which loads Card.
    let files = [
        ("src/index.css", ".container { margin: 0 auto; }\n"),
        ("src/Card.css", ".card { padding: 1rem; }\n"),
        ("src/Card.jsx", "import './Card.css';\nexport const C = () => <div className=\"container card\" />;\n"),
        ("src/App.jsx", "import { C } from './Card';\nexport const App = () => <C />;\n"),
        ("src/main.jsx", "import './index.css';\nimport { App } from './App';\nApp;\n"),
    ];
    assert!(undeclared(&files).is_empty());
}

#[test]
fn an_unreadable_stylesheet_or_none_at_all_is_silent() {
    let jsx = "export const C = () => <div className=\"nope\" />;\n";
    // Nothing loads a stylesheet with the file.
    assert!(undeclared(&[("a.css", ".x{}"), ("a.jsx", jsx)]).is_empty());
    // A package's stylesheet, another host's, Tailwind's, a missing partial.
    for sheet in ["@import \"bootstrap/dist/css/bootstrap.css\";\n.x{}", "@import url(https://cdn.example/x.css);\n.x{}", "@tailwind base;\n.x{}", "@use \"missing\";\n.x{}"] {
        let files = [("a.css", sheet), ("a.jsx", "import './a.css';\nexport const C = () => <div className=\"nope\" />;\n")];
        assert!(undeclared(&files).is_empty(), "{sheet}");
    }
    let files = [("a.css", ".x{}"), ("a.jsx", "import './a.css';\nimport 'bootstrap/dist/css/bootstrap.css';\nexport const C = () => <div className=\"nope\" />;\n")];
    assert!(undeclared(&files).is_empty());
    // An interpolated selector hides class names.
    let files = [("a.scss", "$n: a;\n.#{$n} { color: red; }\n"), ("a.jsx", "import './a.scss';\nexport const C = () => <div className=\"nope\" />;\n")];
    assert!(undeclared(&files).is_empty());
}

#[test]
fn only_static_strings_and_class_shaped_words_are_read() {
    let files = [
        ("a.css", ".a { color: red; }\n"),
        ("a.jsx", "import './a.css';\nconst x = 'b';\nexport const C = () => <div className={`a ${x}`} class={x} className=\"md:w-1/2 [&>p]:x a\" />;\n"),
    ];
    assert!(undeclared(&files).is_empty());
}

#[test]
fn css_modules_are_not_global_stylesheets() {
    let files = [("a.module.css", ".x { color: red; }\n"), ("a.jsx", "import s from './a.module.css';\nexport const C = () => <div className=\"x\" />;\n")];
    assert!(undeclared(&files).is_empty());
}

#[test]
fn a_page_links_stylesheets_to_its_scripts() {
    let files = [
        ("public/site.css", ".site { color: red; }\n"),
        ("public/index.html", "<link rel=\"stylesheet\" href=\"site.css\"><script src=\"app.jsx\"></script>"),
        ("public/app.jsx", "import './own.css';\nexport const C = () => <div className=\"site own gone\" />;\n"),
        ("public/own.css", ".own { color: blue; }\n"),
    ];
    assert_eq!(undeclared(&files), vec!["public/app.jsx:gone"]);
    // A page with an inline style or another host's stylesheet is not read.
    let opaque = [files[0], ("public/index.html", "<style>.x{}</style><script src=\"app.jsx\"></script>"), files[2], files[3]];
    assert!(undeclared(&opaque).is_empty());
}

#[test]
fn a_class_nothing_in_scope_uses_is_reported_and_dynamic_names_keep_it() {
    let files = [
        ("src/Card.css", ".card { padding: 1rem; }\n.legacy { color: red; }\n.btn-big { color: red; }\n"),
        ("src/Card.jsx", "import './Card.css';\nexport const C = (p) => <div className=\"card\" data-x={`btn-${p.size}`} />;\n"),
    ];
    assert_eq!(unused(&files), vec!["src/Card.css:legacy"]);
    // Nobody loads the sheet: unjudged.
    assert!(unused(&[files[0]]).is_empty());
}

#[test]
fn a_class_used_by_any_file_under_the_entry_is_used() {
    let files = [
        ("src/index.css", ".container { margin: 0 auto; }\n.dead { color: red; }\n"),
        ("src/Card.jsx", "export const C = () => <div className=\"container\" />;\n"),
        ("src/main.jsx", "import './index.css';\nimport { C } from './Card';\nC;\n"),
    ];
    assert_eq!(unused(&files), vec!["src/index.css:dead"]);
}

#[test]
fn unused_classes_leave_modules_and_incomplete_sheets_alone() {
    let files = [("a.module.css", ".x{}"), ("a.scss", "$n: a;\n.#{$n} {}\n.y {}\n"), ("a.jsx", "import './a.scss';\nimport s from './a.module.css';\ns;\n")];
    assert!(unused(&files).is_empty());
}

#[test]
fn a_custom_property_nobody_declares_is_reported() {
    let files = [
        ("src/a.css", ".a { color: var(--brand); border: 1px solid var(--edge, red); margin: var(--gap); }\n"),
        ("src/b.css", ":root { --brand: red; }\n@property --gap { syntax: '<length>'; inherits: false; initial-value: 0; }\n"),
        ("src/main.jsx", "import './a.css';\nimport './b.css';\n"),
    ];
    assert!(props(&files).is_empty());
    let files = [
        ("src/a.css", ".a { color: var(--brand); border: 1px solid var(--edge, red); }\n"),
        ("src/main.jsx", "import './a.css';\n"),
    ];
    assert_eq!(props(&files), vec!["src/a.css:--brand"]);
}

#[test]
fn a_script_or_page_that_sets_a_property_declares_it() {
    let files = [
        ("a.css", ".a { color: var(--brand); }\n"),
        ("main.jsx", "import './a.css';\ndocument.documentElement.style.setProperty('--brand', 'red');\n"),
    ];
    assert!(props(&files).is_empty());
    let files = [("a.css", ".a { color: var(--brand); }\n"), ("index.html", "<link rel=\"stylesheet\" href=\"a.css\"><body style=\"--brand: red\">")];
    assert!(props(&files).is_empty());
}

#[test]
fn custom_properties_are_silent_when_unjudgeable() {
    // Nothing loads the sheet; another host's stylesheet; an interpolated name.
    assert!(props(&[("a.css", ".a { color: var(--x); }\n")]).is_empty());
    let files = [("a.css", "@import \"theme/vars.css\";\n.a { color: var(--x); }\n"), ("m.jsx", "import './a.css';\n")];
    assert!(props(&files).is_empty());
    let files = [("a.scss", "$n: x;\n:root { --#{$n}: 1; }\n.a { color: var(--x); }\n"), ("m.jsx", "import './a.scss';\n")];
    assert!(props(&files).is_empty());
}

#[test]
fn sass_imports_find_partials() {
    let files = [
        ("src/_vars.scss", ":root { --brand: red; }\n.p { color: red; }\n"),
        ("src/a.scss", "@use \"vars\";\n.a { color: var(--brand); }\n"),
        ("src/m.jsx", "import './a.scss';\nexport const C = () => <i className=\"a p q\" />;\n"),
    ];
    assert!(props(&files).is_empty());
    assert_eq!(undeclared(&files), vec!["src/m.jsx:q"]);
}

fn private(files: &[(&str, &str)]) -> Vec<String> {
    hits(files, json!({"import": {"visibility": "restricted"}}), JS)
}

#[test]
fn private_and_package_exports_stay_in_their_folders() {
    let files = [
        ("src/lib/impl.js", "/** @private */\nexport const secret = 1;\n/** @package */\nexport const shared = 2;\nexport const open = 3;\n"),
        ("src/lib/peer.js", "import { secret, shared, open } from './impl';\nsecret; shared; open;\n"),
        ("src/lib/deep/child.js", "import { secret, shared } from '../impl';\nsecret; shared;\n"),
        ("src/app.js", "import { secret, shared, open } from './lib/impl';\nsecret; shared; open;\n"),
    ];
    assert_eq!(private(&files), vec!["src/app.js:secret", "src/app.js:shared", "src/lib/deep/child.js:secret"]);
}

#[test]
fn private_names_are_followed_through_reexports_and_index_files_import_freely() {
    let files = [
        ("src/lib/impl.js", "/** @package */\nexport const shared = 2;\n"),
        ("src/lib/index.js", "export { shared } from './impl';\nimport { shared as s } from './impl';\ns;\n"),
        ("src/app.js", "import { shared } from './lib';\nimport * as lib from './lib';\nshared; lib.shared;\n"),
    ];
    assert_eq!(private(&files), vec!["src/app.js:shared", "src/app.js:shared"]);
}

#[test]
fn a_tag_is_a_whole_word() {
    let files = [
        ("src/lib/impl.js", "/** @packageDocumentation */\nexport const a = 1;\n"),
        ("src/app.js", "import { a } from './lib/impl';\na;\n"),
    ];
    assert!(private(&files).is_empty());
}

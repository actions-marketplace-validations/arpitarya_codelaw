use super::*;

fn fmt(language: Language, path: &str, text: &str) -> String {
    format(language, path, text, &FormatOptions::default()).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// One messy input per engine and the layout SR-FORMATTER's defaults give.
#[test]
fn every_engine_formats_with_the_default_style() {
    let cases: &[(Language, &str, &str, &str)] = &[
        (Language::Javascript, "a.js", "const a = {b:1,c:'x'}\nfoo( a )", "const a = { b: 1, c: \"x\" };\nfoo(a);\n"),
        (Language::Jsx, "a.jsx", "const x = <div   className='a'>hi</div>", "const x = <div className=\"a\">hi</div>;\n"),
        (Language::Typescript, "a.ts", "function f(a:number,b:string):void{}", "function f(a: number, b: string): void {}\n"),
        (Language::Tsx, "a.tsx", "const C = (p:{a:number}) => <b>{p.a}</b>", "const C = (p: { a: number }) => <b>{p.a}</b>;\n"),
        (Language::Json, "a.json", "{\n\"a\":1,\"b\":[1,2]}", "{\n  \"a\": 1,\n  \"b\": [1, 2]\n}\n"),
        (Language::Css, "a.css", "a{color:red;margin:0}", "a {\n  color: red;\n  margin: 0;\n}\n"),
        (Language::Python, "a.py", "x = {  'a':1 }\ndef f(a,b):\n  return a", "x = {\"a\": 1}\n\n\ndef f(a, b):\n    return a\n"),
        (Language::Yaml, "a.yaml", "a:   1\nb:\n    - x\n    - y\n", "a: 1\nb:\n  - x\n  - y\n"),
        (Language::Toml, "a.toml", "a=1\n[t]\nb  =  \"x\"\n", "a = 1\n[t]\nb = \"x\"\n"),
        (Language::Markdown, "a.md", "# T\n\n* a\n* b\n", "# T\n\n- a\n- b\n"),
        (Language::Html, "a.html", "<div><p>hi</p></div>\n", "<div><p>hi</p></div>\n"),
    ];
    let mut wrong = Vec::new();
    for (language, path, input, want) in cases {
        let got = fmt(*language, path, input);
        if got != *want {
            wrong.push(format!("{path}:\n--- got\n{got}--- want\n{want}"));
        }
        assert_eq!(fmt(*language, path, &got), got, "{path}: not idempotent");
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn embedded_code_goes_to_its_own_engine() {
    let md = fmt(Language::Markdown, "a.md", "# T\n\n```js\nconst a={b:1}\n```\n\n```py\nx = {  'a':1 }\n```\n");
    assert_eq!(md, "# T\n\n```js\nconst a = { b: 1 };\n```\n\n```py\nx = {\"a\": 1}\n```\n");
    let html = fmt(Language::Html, "a.html", "<script>const a={b:1}</script>\n<style>a{color:red}</style>\n");
    assert!(html.contains("const a = { b: 1 };"), "{html}");
    assert!(html.contains("color: red;"), "{html}");
}

#[test]
fn a_file_that_does_not_parse_is_an_error_not_a_rewrite() {
    for (language, path, text) in [
        (Language::Javascript, "a.js", "const = ;"),
        (Language::Python, "a.py", "def f(:\n"),
        (Language::Json, "a.json", "{\"a\": }"),
        (Language::Toml, "a.toml", "a = = 1"),
        (Language::Yaml, "a.yaml", "a: [1, 2\n"),
        (Language::Css, "a.css", "a { color: red; "),
    ] {
        let r = format(language, path, text, &FormatOptions::default());
        assert!(matches!(r, Err(FormatError::Syntax(_))), "{path}: {r:?}");
    }
}

#[test]
fn options_reach_the_engines() {
    let o = FormatOptions { quote_style: QuoteStyle::Single, semicolons: Semicolons::AsNeeded, indent_width: 4, ..Default::default() };
    assert_eq!(format(Language::Javascript, "a.js", "if (a) {b(\"x\")}", &o).unwrap(), "if (a) {\n    b('x')\n}\n");
    let o = FormatOptions { python_line_width: 20, ..Default::default() };
    assert_eq!(format(Language::Python, "a.py", "f(aaaaaaaa, bbbbbbbbb, ccccccc)\n", &o).unwrap(), "f(\n    aaaaaaaa,\n    bbbbbbbbb,\n    ccccccc,\n)\n");
    let o = FormatOptions { end_of_line: Some(EndOfLine::Crlf), ..Default::default() };
    assert_eq!(format(Language::Json, "a.json", "{\n\"a\":1,\"b\":2}", &o).unwrap(), "{\r\n  \"a\": 1,\r\n  \"b\": 2\r\n}\r\n");
}

/// SR-ARCHITECTURE decision 9: unset, `end_of_line` keeps each file's
/// dominant line ending, in every engine; set, it changes it.
#[test]
fn line_endings_are_kept_unless_set() {
    let o = FormatOptions::default();
    for (l, path, text) in [
        (Language::Javascript, "a.js", "let a=1\r\nlet b=2\r\n"),
        (Language::Python, "a.py", "a=1\r\nb=2\r\n"),
        (Language::Json, "a.json", "{\"a\":1,\r\n\"b\":2}\r\n"),
        (Language::Yaml, "a.yaml", "a:  1\r\nb:  2\r\n"),
        (Language::Toml, "a.toml", "a=1\r\nb=2\r\n"),
        (Language::Markdown, "a.md", "# T\r\n* a\r\n"),
        (Language::Css, "a.css", "a{color:red}\r\nb{color:blue}\r\n"),
        (Language::Html, "a.html", "<p>x</p>\r\n<p>y</p>\r\n"),
    ] {
        let out = format(l, path, text, &o).unwrap();
        assert_eq!(out.matches('\n').count(), out.matches("\r\n").count(), "{path}: {out:?}");
        let lf = format(l, path, text, &FormatOptions { end_of_line: Some(EndOfLine::Lf), ..Default::default() }).unwrap();
        assert!(!lf.contains('\r'), "{path}: {lf:?}");
    }
    assert_eq!(format(Language::Javascript, "a.js", "let a=1\n", &o).unwrap(), "let a = 1;\n");
}

#[test]
fn the_data_check_catches_a_changed_value() {
    assert!(engines::same_meaning(Language::Json, "a.json", "{\"a\": 1}", "{\"a\":1}"));
    assert!(!engines::same_meaning(Language::Json, "a.json", "{\"a\": 1}", "{\"a\": 2}"));
    assert!(!engines::same_meaning(Language::Toml, "a.toml", "a = 1", "a = \"1\""));
    assert!(!engines::same_meaning(Language::Yaml, "a.yaml", "a: 1\n", "a: '1'\n"));
}

#[test]
fn every_language_has_an_engine() {
    for l in Language::ALL {
        assert!(!engine(l).is_empty());
    }
}

/// SR-FORMATTER decision 6: in every engine, `codelaw-format-ignore` and
/// `prettier-ignore` leave the next node byte-identical, the node after it
/// is still formatted, and the marker itself is kept as written.
#[test]
fn ignore_markers_skip_the_next_node_in_every_engine() {
    // (language, path, the comment with `{}` for the marker, the ignored node, a node after it, how that one formats)
    let cases: &[(Language, &str, &str, &str, &str, &str)] = &[
        (Language::Javascript, "a.js", "// {}\n", "const a   =   [1,2,3];\n", "const b={c:1}\n", "const b = { c: 1 };\n"),
        (Language::Jsx, "a.jsx", "// {}\n", "const x = <div   a='1'>hi</div>;\n", "const y = <b   c='2'/>\n", "const y = <b c=\"2\" />;\n"),
        (Language::Typescript, "a.ts", "/* {} */\n", "let t:number   =  1;\n", "let u:number=2\n", "let u: number = 2;\n"),
        (Language::Tsx, "a.tsx", "// {}\n", "let t:number   =  1;\n", "let u:number=2\n", "let u: number = 2;\n"),
        (Language::Css, "a.css", "/* {} */\n", "a{color:red}\n", "b{color:blue}\n", "b {\n  color: blue;\n}\n"),
        (Language::Python, "a.py", "# {}\n", "x = [  1,2,\n  3 ]\n", "y = {  'a':1 }\n", "y = {\"a\": 1}\n"),
        // pretty_yaml: a marker before a block's first entry keeps the whole block.
        (Language::Yaml, "a.yaml", "x: 0\n# {}\n", "a:   [1,   2]\n", "b:    1\n", "b: 1\n"),
        (Language::Toml, "a.toml", "# {}\n", "a  =   [1,2]\n", "b=1\n", "b = 1\n"),
        (Language::Markdown, "a.md", "<!-- {} -->\n", "| a | b |\n|-|-|\n| 1 | 2 |\n", "\n* x\n", "\n- x\n"),
        (Language::Html, "a.html", "<!-- {} -->\n", "<div   ><p>hi</p>   </div>\n", "<p   >x</p>\n", "<p>x</p>\n"),
    ];
    let mut wrong = Vec::new();
    for (language, path, form, node, after, after_formatted) in cases {
        for marker in ["codelaw-format-ignore", "prettier-ignore"] {
            let comment = form.replace("{}", marker);
            let input = format!("{comment}{node}{after}");
            let got = match format(*language, path, &input, &FormatOptions::default()) {
                Ok(g) => g,
                Err(e) => {
                    wrong.push(format!("{path} {marker}: {e}"));
                    continue;
                }
            };
            if !got.starts_with(&format!("{comment}{node}")) || !got.ends_with(after_formatted) {
                wrong.push(format!("{path} {marker}:\n--- input\n{input}--- got\n{got}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// JSON allows comments (decision 2's parser options), and so the markers.
#[test]
fn ignore_markers_in_json() {
    for marker in ["codelaw-format-ignore", "prettier-ignore"] {
        let input = format!("{{\n  // {marker}\n  \"a\":   [1,2],\n  \"b\":[3,4]\n}}\n");
        let got = fmt(Language::Json, "a.json", &input);
        assert!(got.contains(&format!("// {marker}\n  \"a\":   [1,2],")), "{got}");
        assert!(got.contains("\"b\": [3, 4]"), "{got}");
    }
}

/// A marker nested in an ignored node, a marker with nothing after it,
/// and the engines' own directives next to codelaw's.
#[test]
fn ignore_markers_edge_cases() {
    // Python: a marker before a compound statement keeps all of it; a
    // marker at the end of a block skips nothing; `# fmt: off` still works.
    let py = "# codelaw-format-ignore\ndef f( a ):\n    # prettier-ignore\n    return [ a ]\n\n\nx = 1\n# fmt: off\ny  =  2\n# fmt: on\nz  =  3\n";
    let got = fmt(Language::Python, "a.py", py);
    assert!(got.starts_with("# codelaw-format-ignore\ndef f( a ):\n    # prettier-ignore\n    return [ a ]\n"), "{got}");
    assert!(got.contains("y  =  2\n") && got.ends_with("z = 3\n"), "{got}");
    let tail = "def g():\n    x  =  1\n    # codelaw-format-ignore\ny  =  2\n";
    assert_eq!(fmt(Language::Python, "a.py", tail), "def g():\n    x = 1\n    # codelaw-format-ignore\n\n\ny = 2\n");
    // JS: Biome's own directive is untouched beside codelaw's.
    let js = "// biome-ignore format: mine\nconst a  =  1;\n// codelaw-format-ignore\nconst b  =  2;\nconst c  =  3;\n";
    assert_eq!(fmt(Language::Javascript, "a.js", js), "// biome-ignore format: mine\nconst a  =  1;\n// codelaw-format-ignore\nconst b  =  2;\nconst c = 3;\n");
    // TOML: a marker inside an array keeps one element.
    let toml = "a = [\n  # prettier-ignore\n  {x=1},\n  {y=2},\n]\n";
    let got = fmt(Language::Toml, "a.toml", toml);
    assert!(got.contains("{x=1}") && got.contains("{ y = 2 }"), "{got}");
    // A marker in a string is not a marker.
    assert_eq!(fmt(Language::Javascript, "a.js", "const s='// prettier-ignore'\n"), "const s = \"// prettier-ignore\";\n");
}

/// W-141: a comment that starts its own line keeps it, as Prettier writes it,
/// even when the source puts the next rule or declaration right after it.
#[test]
fn a_css_comment_on_its_own_line_stays_there() {
    let cases: &[(Language, &str, &str, &str)] = &[
        (Language::Css, "a.css", "/*!\n * banner\n */.a{color:red}\n", "/*!\n * banner\n */\n.a {\n  color: red;\n}\n"),
        (Language::Css, "a.css", "/* one */ .b{color:red}\n", "/* one */\n.b {\n  color: red;\n}\n"),
        (Language::Css, "a.css", "@media print{\n/* x */.e{color:red}}\n", "@media print {\n  /* x */\n  .e {\n    color: red;\n  }\n}\n"),
        (Language::Css, "a.css", ".f{\n/* in */color:red}\n", ".f {\n  /* in */\n  color: red;\n}\n"),
        // A comment after a node on the same line stays a trailing comment.
        (Language::Css, "a.css", ".c{color:red}/* tail */\n.d{color:blue}\n", ".c {\n  color: red;\n} /* tail */\n.d {\n  color: blue;\n}\n"),
        // Inside a selector list the comment is not statement level: untouched.
        (Language::Css, "a.css", "a,\n/* x */ b{color:red}\n", "a,\n/* x */ b {\n  color: red;\n}\n"),
        // A comment followed by another comment is left as it is.
        (Language::Scss, "a.scss", ".a {\n  b: c;\n  /*! */ // why\n}\n", ".a {\n  b: c;\n  /*! */ // why\n}\n"),
        (Language::Scss, "a.scss", "// see https://x.y/*z\n/* c */.a{b:url(//x/*y)}\n", "// see https://x.y/*z\n/* c */\n.a {\n  b: url(//x/*y);\n}\n"),
    ];
    let mut wrong = Vec::new();
    for (language, path, input, want) in cases {
        let got = fmt(*language, path, input);
        if got != *want {
            wrong.push(format!("{input:?}:\n--- got\n{got}--- want\n{want}"));
        }
        assert_eq!(fmt(*language, path, &got), got, "{input:?}: not idempotent");
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// W-141: void elements end in ` />`, as Prettier writes them.
#[test]
fn html_void_elements_are_self_closed() {
    let got = fmt(Language::Html, "a.html", "<head><meta charset=\"utf-8\"><link rel=\"x\" href=\"y\"></head>\n<p>a<br>b</p>\n");
    assert!(got.contains("<meta charset=\"utf-8\" />") && got.contains("<link rel=\"x\" href=\"y\" />") && got.contains("<br />"), "{got}");
    assert_eq!(fmt(Language::Html, "a.html", &got), got, "not idempotent");
}

/// W-141: a `style` attribute and a `<script>` body as Prettier writes them.
#[test]
fn html_style_attributes_and_scripts_match_prettier() {
    let input = "<html>\n<body>\n<p style=\"color:red;display : inline;\">x</p>\n<script>\nconst fileName = img.currentSrc?.substring(img.currentSrc.lastIndexOf(\"/\"));\n</script>\n</body>\n</html>\n";
    let want = "<html>\n  <body>\n    <p style=\"color: red; display: inline\">x</p>\n    <script>\n      const fileName = img.currentSrc?.substring(\n        img.currentSrc.lastIndexOf(\"/\"),\n      );\n    </script>\n  </body>\n</html>\n";
    let got = fmt(Language::Html, "a.html", input);
    assert_eq!(got, want);
    assert_eq!(fmt(Language::Html, "a.html", &got), got, "not idempotent");
    // A value that does not parse, or holds a comment, is left as written.
    for kept in ["<p style=\"{{ x }}\">a</p>\n", "<p style=\"color: red; /* c */\">a</p>\n"] {
        assert_eq!(fmt(Language::Html, "a.html", kept), kept);
    }
}

// TEMP-W141-MEASURE (removed before landing)
#[test]
#[ignore]
fn temp_measure_corpus() {
    let Ok(dir) = std::env::var("W141_DIR") else { return };
    for e in std::fs::read_dir(&dir).unwrap() {
        let p = e.unwrap().path();
        let Some(lang) = Language::from_path(&p) else { continue };
        let text = std::fs::read_to_string(&p).unwrap();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        match format(lang, &name, &text, &FormatOptions::default()) {
            Ok(out) => std::fs::write(&p, out).unwrap(),
            Err(_) => println!("{name}\n  format/failed"),
        }
    }
}

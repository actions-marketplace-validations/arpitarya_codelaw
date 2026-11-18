//! W-182: Ruff 0.16.9's RUF001, RUF002 and RUF003 — `py/ambiguous-unicode-
//! character-string`, `-docstring` and `-comment` — written from the rule's
//! behaviour (MIT; read for research only, L5).
//!
//! A non-ASCII character Unicode lists as easy to mistake for one ASCII
//! character (Cyrillic `а` for `a`, `’` for `'`) is reported in a string, in
//! a docstring or in a comment, with the ASCII character as the suggestion.
//! The table is the data package `@codelaw/data-confusables` (L6 decision 15;
//! L11 D-54), asked for through `FileContext::data_table`: a file with only
//! ASCII text never asks, and without the package the rules report nothing.
//!
//! Which characters count follows Ruff, which follows VS Code: in a run of
//! letters and digits (a word), the confusable characters are reported when
//! the word holds an ASCII character, or when every character of it is
//! confusable; a word with a non-ASCII character that is not confusable is a
//! word of another script and is left alone. A confusable character that is
//! not a letter or digit (`’`, `×`, a no-break space) is always reported.

use crate::python_util::{statements, PY};
use crate::util::children;
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use tree_sitter::Node;

/// What a rule reads: the table and the characters the project allows.
struct Table<'a> {
    value: &'a Value,
    allowed: Vec<char>,
}

impl Table<'_> {
    /// The ASCII character `c` is mistaken for, with the two names.
    fn confusable(&self, c: char) -> Option<(char, &str, &str)> {
        if c.is_ascii() || self.allowed.contains(&c) {
            return None;
        }
        let from = self.value.get("from")?.as_array()?;
        let at = from.binary_search_by(|v| v.as_u64().cmp(&Some(u64::from(c)))).ok()?;
        let to = self.value.get("to")?.as_array()?.get(at)?.as_str()?;
        let name = self.value.get("names")?.as_array()?.get(at)?.as_str()?;
        let target_name = self.value.get("target_names")?.get(to)?.as_str()?;
        Some((to.chars().next()?, name, target_name))
    }
}

/// One confusable character found: where, and what to say.
struct Candidate {
    at: usize,
    c: char,
    to: char,
    name: String,
    to_name: String,
}

/// The confusable characters of `text` that Ruff reports (see the module
/// doc); `at` of each is its byte offset in `text`.
fn scan(text: &str, table: &Table<'_>) -> Vec<Candidate> {
    let mut out = Vec::new();
    let mut word: Vec<Candidate> = Vec::new();
    let (mut ascii, mut other) = (false, false);
    let flush = |word: &mut Vec<Candidate>, out: &mut Vec<Candidate>, ascii: bool, other: bool| {
        // A word is reported when it holds an ASCII character, or when it is
        // nothing but confusable characters.
        if ascii || !other {
            out.append(word);
        }
        word.clear();
    };
    for (at, c) in text.char_indices() {
        let found = table.confusable(c).map(|(to, name, to_name)| Candidate { at, c, to, name: name.to_owned(), to_name: to_name.to_owned() });
        if !c.is_alphanumeric() {
            flush(&mut word, &mut out, ascii, other);
            (ascii, other) = (false, false);
            out.extend(found);
        } else if c.is_ascii() {
            ascii = true;
        } else if let Some(f) = found {
            word.push(f);
        } else {
            other = true;
        }
    }
    flush(&mut word, &mut out, ascii, other);
    out
}

fn findings(text: &str, base: usize, label: &str, table: &Table<'_>, out: &mut Vec<NativeFinding>) {
    for c in scan(text, table) {
        let (start, end) = (base + c.at, base + c.at + c.c.len_utf8());
        out.push(NativeFinding {
            start,
            end,
            message: format!("{label} contains ambiguous `{}` ({}). Did you mean `{}` ({})?", c.c, c.name, c.to, c.to_name),
            edits: vec![TextEdit::new(start, end, c.to.to_string())],
        });
    }
}

/// The table, when the package is installed and `text` has anything to
/// judge: ASCII text has no ambiguous character, and does not ask.
fn table_for<'a>(text: &str, ctx: &NativeContext<'_>, hold: &'a mut Option<std::sync::Arc<Value>>) -> Option<Table<'a>> {
    if text.is_ascii() {
        return None;
    }
    *hold = Some(ctx.file.data_table("confusables")?);
    let allowed = ctx.options.get("allowed_confusables").and_then(Value::as_array).map(|a| a.iter().filter_map(|v| v.as_str()).flat_map(str::chars).collect()).unwrap_or_default();
    Some(Table { value: hold.as_deref()?, allowed })
}

/// Whether a `string` node is the docstring of its module, class or function:
/// the first statement of that body, alone on its statement.
fn is_docstring(s: Node<'_>) -> bool {
    let Some(mut stmt) = s.parent() else { return false };
    if stmt.kind() == "concatenated_string" {
        let Some(p) = stmt.parent() else { return false };
        stmt = p;
    }
    if stmt.kind() != "expression_statement" || stmt.named_child_count() != 1 {
        return false;
    }
    let Some(body) = stmt.parent() else { return false };
    let owner_ok = match body.kind() {
        "module" => true,
        "block" => body.parent().is_some_and(|p| matches!(p.kind(), "function_definition" | "class_definition")),
        _ => false,
    };
    owner_ok && statements(body).first().is_some_and(|f| f.id() == stmt.id())
}

/// The runs of a string's text that are judged one at a time: the whole
/// literal, or — for an f-string — each run between its replacement fields.
fn runs(s: Node<'_>, src: &str) -> Vec<(usize, usize)> {
    let head = src[s.start_byte()..].chars().take_while(|c| *c != '"' && *c != '\'').collect::<String>().to_ascii_lowercase();
    if !head.contains(['f', 't']) {
        return vec![(s.start_byte(), s.end_byte())];
    }
    let mut out = Vec::new();
    let mut run: Option<(usize, usize)> = None;
    for c in children(s) {
        if c.kind() == "interpolation" {
            out.extend(run.take());
        } else {
            run = Some(match run {
                Some((a, _)) => (a, c.end_byte()),
                None => (c.start_byte(), c.end_byte()),
            });
        }
    }
    out.extend(run);
    out
}

struct Ambiguous {
    id: &'static str,
    label: &'static str,
    /// Docstrings only (RUF002), strings that are not docstrings (RUF001), or comments.
    scope: Scope,
}

#[derive(PartialEq)]
enum Scope {
    String,
    Docstring,
    Comment,
}

impl NativeRule for Ambiguous {
    fn id(&self) -> &'static str {
        self.id
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        match self.scope {
            Scope::Comment => &["comment"],
            _ => &["string"],
        }
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        if self.scope != Scope::Comment && is_docstring(n) != (self.scope == Scope::Docstring) {
            return;
        }
        let mut hold = None;
        if self.scope == Scope::Comment {
            let text = &ctx.source[n.start_byte()..n.end_byte()];
            if let Some(t) = table_for(text, ctx, &mut hold) {
                findings(text, n.start_byte(), self.label, &t, out);
            }
            return;
        }
        for (a, b) in runs(n, ctx.source) {
            let text = &ctx.source[a..b];
            if let Some(t) = table_for(text, ctx, &mut hold) {
                findings(text, a, self.label, &t, out);
            }
        }
    }
}

pub static RULES: &[&dyn NativeRule] = &[
    &Ambiguous { id: "py/ambiguous-unicode-character-string", label: "String", scope: Scope::String },
    &Ambiguous { id: "py/ambiguous-unicode-character-docstring", label: "Docstring", scope: Scope::Docstring },
    &Ambiguous { id: "py/ambiguous-unicode-character-comment", label: "Comment", scope: Scope::Comment },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn table() -> Value {
        // а, о (Cyrillic), ’, × and the no-break space.
        serde_json::json!({
            "schema": 1, "version": "x",
            "from": [160, 215, 1072, 1086, 8217],
            "to": [" ", "x", "a", "o", "'"],
            "names": ["NO-BREAK SPACE", "MULTIPLICATION SIGN", "CYRILLIC SMALL LETTER A", "CYRILLIC SMALL LETTER O", "RIGHT SINGLE QUOTATION MARK"],
            "target_names": {" ": "SPACE", "x": "LATIN SMALL LETTER X", "a": "LATIN SMALL LETTER A", "o": "LATIN SMALL LETTER O", "'": "APOSTROPHE"}
        })
    }

    struct WithTable(crate::DetachedFile);
    impl crate::FileContext for WithTable {
        fn path(&self) -> &str {
            self.0.path()
        }
        fn bytes(&self) -> &[u8] {
            self.0.bytes()
        }
        fn exists(&self, p: &str) -> bool {
            self.0.exists(p)
        }
        fn is_dir(&self, p: &str) -> bool {
            self.0.is_dir(p)
        }
        fn read(&self, p: &str) -> Option<String> {
            self.0.read(p)
        }
        fn list(&self, p: &str) -> Option<Vec<String>> {
            self.0.list(p)
        }
        fn data_table(&self, short: &str) -> Option<std::sync::Arc<Value>> {
            (short == "confusables").then(|| std::sync::Arc::new(table()))
        }
    }

    fn run(rule: &dyn NativeRule, src: &str, options: BTreeMap<String, Value>) -> Vec<NativeFinding> {
        let root = codelaw_lang::parse(Language::Python, src);
        let data = BTreeMap::new();
        let file = WithTable(crate::DetachedFile::new("example.py", src));
        let ctx = NativeContext { source: src, language: Language::Python, semantic: None, options: &options, data: &data, file: &file };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            if rule.kinds().contains(&node.kind().as_ref()) {
                rule.check(node, &ctx, &mut out);
            }
        }
        out
    }

    fn texts(src: &str, rule: &dyn NativeRule) -> Vec<String> {
        run(rule, src, BTreeMap::new()).iter().map(|f| src[f.start..f.end].to_owned()).collect()
    }

    #[test]
    fn a_word_is_judged_whole() {
        let t = table();
        let table = Table { value: &t, allowed: Vec::new() };
        let seen = |s: &str| scan(s, &table).iter().map(|c| c.c).collect::<String>();
        // an ASCII letter beside it: reported
        assert_eq!(seen("pаss"), "а");
        // nothing but confusables: reported
        assert_eq!(seen("оо"), "оо");
        // a letter of the same script that is not confusable: the word is Cyrillic
        assert_eq!(seen("об"), "");
        // not a letter: always reported
        assert_eq!(seen("it’s 2 × 3\u{a0}"), "’×\u{a0}");
        assert_eq!(seen("plain ascii"), "");
    }

    #[test]
    fn strings_docstrings_and_comments_are_told_apart() {
        let src = "\"\"\"Mоdule doc.\"\"\"\n\nx = 'pаss'\n\n\ndef f():\n    \"\"\"Dо it.\"\"\"\n    y = \"fine\"  # a ‘comment’ with оne\n    return y\n";
        let [s, d, c] = [&RULES[0], &RULES[1], &RULES[2]];
        assert_eq!(texts(src, *s), vec!["а"]);
        assert_eq!(texts(src, *d), vec!["о", "о"]);
        assert_eq!(texts(src, *c), vec!["’", "о"]);
    }

    #[test]
    fn an_f_string_is_judged_outside_its_fields() {
        let src = "x = f\"{a} pаss {b}\"\n";
        assert_eq!(texts(src, &Ambiguous { id: "", label: "String", scope: Scope::String }), vec!["а"]);
        let f = run(RULES[0], "x = 'it’s'\n", BTreeMap::new());
        assert_eq!(f[0].message, "String contains ambiguous `’` (RIGHT SINGLE QUOTATION MARK). Did you mean `'` (APOSTROPHE)?");
        assert_eq!(f[0].edits[0].text, "'");
    }

    #[test]
    fn allowed_confusables_are_left_alone() {
        let mut o = BTreeMap::new();
        o.insert("allowed_confusables".to_owned(), serde_json::json!(["’"]));
        assert!(run(RULES[0], "x = 'it’s'\n", o).is_empty());
    }

    #[test]
    fn an_ascii_file_never_asks_for_the_table() {
        struct Refuses;
        impl crate::FileContext for Refuses {
            fn path(&self) -> &str {
                "a.py"
            }
            fn bytes(&self) -> &[u8] {
                b""
            }
            fn exists(&self, _: &str) -> bool {
                false
            }
            fn is_dir(&self, _: &str) -> bool {
                false
            }
            fn read(&self, _: &str) -> Option<String> {
                None
            }
            fn list(&self, _: &str) -> Option<Vec<String>> {
                None
            }
            fn data_table(&self, _: &str) -> Option<std::sync::Arc<Value>> {
                panic!("an ASCII text asked for the table")
            }
        }
        let src = "x = 'plain'  # plain\n";
        let root = codelaw_lang::parse(Language::Python, src);
        let (options, data) = (BTreeMap::new(), BTreeMap::new());
        let ctx = NativeContext { source: src, language: Language::Python, semantic: None, options: &options, data: &data, file: &Refuses };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            for r in RULES {
                if r.kinds().contains(&node.kind().as_ref()) {
                    r.check(node.clone(), &ctx, &mut out);
                }
            }
        }
        assert!(out.is_empty());
    }
}

//! W-139 strand W-139-jsfinal: the small kit the last JavaScript rules share
//! — a rule on the per-file semantic model, and the few syntax questions
//! more than one of them asks.

use crate::util::{children, text};
use tree_sitter::Node;

/// A native rule that needs the semantic model: the body runs only when the
/// model was built.
macro_rules! srule {
    ($name:ident, $id:literal, $langs:expr, $kinds:expr, |$n:ident, $ctx:ident, $m:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl crate::NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [codelaw_core::Language] {
                $langs
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn needs_semantic(&self) -> bool {
                true
            }
            fn check(&self, node: codelaw_lang::SyntaxNode<'_>, $ctx: &crate::NativeContext<'_>, $out: &mut Vec<crate::NativeFinding>) {
                let Some($m) = $ctx.semantic else { return };
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}
pub(super) use srule;

/// `(x)` as `x`.
pub(super) fn strip_parens(mut n: Node<'_>) -> Node<'_> {
    while n.kind() == "parenthesized_expression" {
        match n.named_child(0) {
            Some(x) => n = x,
            None => break,
        }
    }
    n
}

/// Whether a member or call carries `?.`.
pub(super) fn optional(n: Node<'_>) -> bool {
    children(n).iter().any(|c| c.kind() == "optional_chain" || c.kind() == "?.")
}

/// The operator of a binary or assignment expression.
pub(super) fn operator<'s>(n: Node<'_>, src: &'s str) -> Option<&'s str> {
    n.child_by_field_name("operator").map(|o| text(o, src))
}

/// The value of a numeric literal, when it is a plain number.
pub(super) fn number(n: Node<'_>, src: &str) -> Option<f64> {
    if n.kind() != "number" {
        return None;
    }
    let t: String = text(n, src).chars().filter(|c| *c != '_').collect();
    let lower = t.to_ascii_lowercase();
    for (prefix, radix) in [("0x", 16), ("0o", 8), ("0b", 2)] {
        if let Some(rest) = lower.strip_prefix(prefix) {
            return u64::from_str_radix(rest, radix).ok().map(|v| v as f64);
        }
    }
    lower.parse::<f64>().ok()
}

/// Whether a comment sits inside `[start, end)`.
pub(super) fn comment_in(root: Node<'_>, start: usize, end: usize) -> bool {
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        if x.end_byte() <= start || x.start_byte() >= end {
            continue;
        }
        if x.kind() == "comment" {
            return true;
        }
        stack.extend(children(x));
    }
    false
}

#[cfg(test)]
mod tests {
    use super::super::{abort_any, atomic, names, parameterized, params, zip};
    use crate::{DetachedFile, MemoryFiles, NativeContext, NativeFinding, NativeRule};
    use codelaw_core::Language;
    use std::collections::BTreeMap;

    fn run_in(rule: &dyn NativeRule, src: &str, options: &[(&str, serde_json::Value)], file: &dyn crate::FileContext) -> Vec<NativeFinding> {
        let language = Language::Javascript;
        let root = codelaw_lang::parse(language, src);
        let model = codelaw_semantic::build(language, &root, src);
        let options: BTreeMap<String, serde_json::Value> = options.iter().map(|(k, v)| ((*k).to_owned(), v.clone())).collect();
        let data = BTreeMap::new();
        let ctx = NativeContext { source: src, language, semantic: model.as_ref(), options: &options, data: &data, file };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            if rule.kinds().contains(&node.kind().as_ref()) {
                rule.check(node, &ctx, &mut out);
            }
        }
        out
    }

    fn run(rule: &dyn NativeRule, src: &str, options: &[(&str, serde_json::Value)]) -> Vec<NativeFinding> {
        run_in(rule, src, options, &DetachedFile::new("example.js", src))
    }

    #[test]
    fn name_replacements_reads_the_filename() {
        let src = "const value = 1;\n";
        let f = run_in(&names::NameReplacements, src, &[], &MemoryFiles::new("src/utils.js", src, &[]));
        assert_eq!(f.len(), 1);
        assert!(f[0].message.contains("`utils.js` should be named `utilities.js`"), "{}", f[0].message);
        let off = run_in(&names::NameReplacements, src, &[("checkFilenames", serde_json::json!(false))], &MemoryFiles::new("src/utils.js", src, &[]));
        assert!(off.is_empty());
        let own = run_in(&names::NameReplacements, src, &[], &MemoryFiles::new("docs/utils.md", src, &[]));
        assert!(own.is_empty(), "a host file that is not JavaScript has no filename check");
    }

    #[test]
    fn name_replacements_options() {
        let src = "const cb = 1;\nconst tmp = 2;\nuse(cb, tmp);\n";
        assert_eq!(run(&names::NameReplacements, src, &[]).len(), 2);
        let custom = [("replacements", serde_json::json!({ "cb": false })), ("allowList", serde_json::json!({ "tmp": true }))];
        assert!(run(&names::NameReplacements, src, &custom).is_empty());
        let only = [("extendDefaultReplacements", serde_json::json!(false)), ("replacements", serde_json::json!({ "tmp": { "scratch": true } }))];
        let f = run(&names::NameReplacements, src, &only);
        assert_eq!(f.len(), 1);
        assert!(f[0].message.contains("`scratch`"));
        let props = run(&names::NameReplacements, "const o = { err: 1 };\no.cb = 2;\n", &[("checkProperties", serde_json::json!(true))]);
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn no_unnecessary_parameters_minimum_call_count() {
        let src = "function log(message, level) { return [message, level]; }\nlog('a', 'info');\n";
        assert!(run(&params::NoUnnecessaryParameters, src, &[]).is_empty());
        let f = run(&params::NoUnnecessaryParameters, src, &[("minimumCallCount", serde_json::json!(1))]);
        assert_eq!(f.len(), 2, "both parameters get the same value at the one call");
        assert!(f.iter().any(|x| x.message.contains("`level`")));
    }

    #[test]
    fn prefer_iterator_zip_and_abort_signal_any_need_the_model() {
        let src = "for (let i = 0; i < Math.min(a.length, b.length); i++) { use(a[i], b[i]); }\n";
        assert_eq!(run(&zip::PreferIteratorZip, src, &[]).len(), 1);
        let src = "const abortController = new AbortController();\nfirstSignal.addEventListener('abort', () => abortController.abort());\nsecondSignal.addEventListener('abort', () => abortController.abort());\nrun(abortController.signal);\n";
        assert_eq!(run(&abort_any::PreferAbortSignalAny, src, &[]).len(), 1);
    }

    #[test]
    fn parameterized_tests_follow_the_package_json() {
        let tests = "it('a', () => {\n  const v = f(1);\n  expect(v).toBe(2);\n  expect(g(v)).toBe(true);\n});\nit('b', () => {\n  const v = f(2);\n  expect(v).toBe(3);\n  expect(g(v)).toBe(true);\n});\nit('c', () => {\n  const v = f(3);\n  expect(v).toBe(4);\n  expect(g(v)).toBe(true);\n});\n";
        assert!(run(&parameterized::ParameterizedTests, tests, &[]).is_empty(), "no framework, no report");
        let with = MemoryFiles::new("src/a.test.js", tests, &[("package.json", r#"{ "devDependencies": { "vitest": "^3" } }"#)]);
        assert_eq!(run_in(&parameterized::ParameterizedTests, tests, &[], &with).len(), 1);
        let mocha = format!("import 'mocha';\n{tests}");
        let with = MemoryFiles::new("src/a.test.js", &mocha, &[("package.json", r#"{ "devDependencies": { "vitest": "^3" } }"#)]);
        assert!(run_in(&parameterized::ParameterizedTests, &mocha, &[], &with).is_empty(), "the globals belong to Mocha");
    }

    #[test]
    fn require_atomic_updates_allow_properties() {
        let src = "const foo = {};\nasync function x() { const a = foo.bar; await y(); foo.bar = a; foo.baz = foo.bar + await z(); }\n";
        assert!(!run(&atomic::RequireAtomicUpdates, src, &[]).is_empty());
        assert!(run(&atomic::RequireAtomicUpdates, src, &[("allowProperties", serde_json::json!(true))]).is_empty());
        let var = "let foo;\nasync function x() { foo = foo + await z(); }\n";
        assert_eq!(run(&atomic::RequireAtomicUpdates, var, &[("allowProperties", serde_json::json!(true))]).len(), 1);
    }

    #[test]
    fn locale_order_of_replacements() {
        let f = run(&names::NameReplacements, "let dst;\nuse(dst);\n", &[]);
        assert_eq!(f.len(), 1);
        assert!(f[0].message.contains("`daylightSavingTime`, `destination`, `distribution`"), "{}", f[0].message);
    }
}

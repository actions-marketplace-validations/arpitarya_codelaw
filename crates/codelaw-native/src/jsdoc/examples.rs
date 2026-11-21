//! `js/jsdoc-check-examples` (W-178): an `@example` whose code does not parse.
//!
//! eslint-plugin-jsdoc's `check-examples` runs the example through ESLint and
//! reports what the project's rules find; here that is the engine's job (the
//! example is a region, `[lint] jsdoc_examples`, SR-LINTER decision 3), so
//! this rule reads the one thing no other rule can: that the example is not
//! code. An example is read as the doc comment holds it: the lines up to the
//! next tag, a fenced block's code when it has a fence, a `<caption>` line
//! left out. A fragment that is not a whole program (`add(1,`, `...`) does not
//! parse and is reported; `ignore` lists the regular expressions of examples
//! left alone.

use super::{find, opt_str_list};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;

pub struct CheckExamples;

impl NativeRule for CheckExamples {
    fn id(&self) -> &'static str {
        "js/jsdoc-check-examples"
    }
    fn languages(&self) -> &'static [Language] {
        super::JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["comment"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let ignore: Vec<regex::Regex> = opt_str_list(ctx.options, "ignore").iter().filter_map(|p| regex::Regex::new(p).ok()).collect();
        for e in codelaw_lang::jsdoc_examples(&node, ctx.source, ctx.language) {
            if e.parses {
                continue;
            }
            let code = e.region.text.trim();
            if ignore.iter().any(|r| r.is_match(code)) {
                continue;
            }
            // From the first character of code, not its margin.
            let start = e.region.text.find(|c: char| !c.is_whitespace()).and_then(|i| e.region.to_host(i)).unwrap_or(e.region.host_start);
            let language = format!("{:?}", e.region.language);
            out.push(find(start, e.region.host_end.max(start), format!("This `@example` is not valid {language}; fix it, or mark it as not code"), Vec::new()));
        }
    }
}

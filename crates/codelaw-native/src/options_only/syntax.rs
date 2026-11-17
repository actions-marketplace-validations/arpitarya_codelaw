//! `js/no-restricted-syntax`: the syntax a project forbids. ESLint 10.13.0's
//! rule takes ESTree selectors; a tree-sitter tree has no ESTree, so an
//! entry is an ast-grep pattern (`for ($K in $O) $$$B`: the same patterns a
//! project rule's `match:` takes) or a tree-sitter node kind.

use super::{JS, at, opt_strings, option_problem};
use crate::{NativeContext, NativeFinding, NativeRule};
use ast_grep_core::matcher::{MatcherExt, Pattern};
use codelaw_core::Language;
use codelaw_lang::{Grammar, SyntaxNode};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

fn compiled(pattern: &str, grammar: Grammar) -> Result<Arc<Pattern>, String> {
    static CACHE: LazyLock<Mutex<HashMap<(String, Grammar), Result<Arc<Pattern>, String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    cache.entry((pattern.to_owned(), grammar)).or_insert_with(|| Pattern::try_new(pattern, grammar).map(Arc::new).map_err(|e| e.to_string())).clone()
}

enum Restriction {
    Pattern(Arc<Pattern>, String),
    Kind(String, String),
}

pub struct NoRestrictedSyntax;

impl NativeRule for NoRestrictedSyntax {
    fn id(&self) -> &'static str {
        "js/no-restricted-syntax"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn needs_options(&self) -> &'static [&'static str] {
        &["patterns", "kinds"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let grammar = Grammar::for_language(ctx.language);
        let root = node.get_inner_node();
        let messages = ctx.options.get("messages").and_then(Value::as_object);
        let message_for = |what: &str| messages.and_then(|m| m.get(what)).and_then(Value::as_str).map_or_else(|| format!("Using '{what}' is not allowed."), str::to_owned);
        let mut restrictions: Vec<Restriction> = Vec::new();
        for p in opt_strings(ctx, "patterns") {
            match compiled(&p, grammar) {
                Ok(c) => restrictions.push(Restriction::Pattern(c, message_for(&p))),
                Err(err) => option_problem(root, &["program"], out, format!("`patterns` entry `{p}` is not a valid pattern: {err}")),
            }
        }
        for k in opt_strings(ctx, "kinds") {
            let message = message_for(&k);
            restrictions.push(Restriction::Kind(k, message));
        }
        if restrictions.is_empty() {
            return;
        }
        for n in node.dfs() {
            let inner = n.get_inner_node();
            for r in &restrictions {
                match r {
                    Restriction::Pattern(p, message) => {
                        if p.match_node(n.clone()).is_some() {
                            out.push(at(inner, message.clone()));
                        }
                    }
                    Restriction::Kind(k, message) => {
                        if inner.kind() == k.as_str() {
                            out.push(at(inner, message.clone()));
                        }
                    }
                }
            }
        }
    }
}

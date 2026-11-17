//! W-176: the options-only rules — rules that check nothing until a project
//! sets the option that says what to check (SR-RULE-FORMAT decision 2d).
//!
//! What every rule here shares: [`NativeRule::needs_options`] names the
//! options it reads, and the engine dispatches it to no node while none is
//! set, so a rule that is on but unconfigured costs nothing and says
//! nothing — no finding, no warning about the missing option. A value that
//! cannot be used (a pattern that does not compile) is reported once, on the
//! file's first node of the rule's kinds: that is a mistake in the option,
//! not a missing one.

mod comments;
mod functions;
mod globals;
mod ids;
mod imports;
mod jsdoc;
mod strings;
mod syntax;

use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use tree_sitter::Node;

pub(super) const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

pub(crate) static RULES: &[&dyn NativeRule] = &[
    &ids::IdDenylist,
    &ids::IdMatch,
    &ids::IdMatchUnicorn,
    &syntax::NoRestrictedSyntax,
    &globals::NoImplicitGlobals,
    &comments::CommentRegex,
    &comments::FileHeader,
    &functions::ConsistentFunctionStyle,
    &imports::ImportPreferNamespaceImport,
    &imports::ImportNoRestrictedPaths,
    &imports::NoRestrictedImport,
    &imports::NoRestrictedRequire,
    &strings::StringContent,
    &jsdoc::JsdocMatchName,
    &jsdoc::JsdocNoMissingSyntax,
    &jsdoc::JsdocNoRestrictedSyntax,
    &jsdoc::JsdocRequireTags,
    &jsdoc::JsdocTextEscaping,
];

pub(super) fn finding(start: usize, end: usize, message: impl Into<String>, edits: Vec<codelaw_core::TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

pub(super) fn at(n: Node<'_>, message: impl Into<String>) -> NativeFinding {
    finding(n.start_byte(), n.end_byte(), message, Vec::new())
}

pub(super) fn opt_str<'a>(ctx: &'a NativeContext<'_>, key: &str) -> Option<&'a str> {
    ctx.options.get(key).and_then(Value::as_str).filter(|s| !s.is_empty())
}

pub(super) fn opt_bool(ctx: &NativeContext<'_>, key: &str, default: bool) -> bool {
    ctx.options.get(key).and_then(Value::as_bool).unwrap_or(default)
}

pub(super) fn opt_strings(ctx: &NativeContext<'_>, key: &str) -> Vec<String> {
    match ctx.options.get(key) {
        Some(Value::Array(v)) => v.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect(),
        Some(Value::String(s)) if !s.is_empty() => vec![s.clone()],
        _ => Vec::new(),
    }
}

/// A regular expression, compiled once per distinct text (a rule is asked
/// about many nodes with the same option). `Err` carries the compile error.
pub(super) fn regex_of(pattern: &str) -> Result<Arc<Regex>, String> {
    static CACHE: LazyLock<Mutex<HashMap<String, Result<Arc<Regex>, String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(r) = cache.get(pattern) {
        return r.clone();
    }
    let r = Regex::new(pattern).map(Arc::new).map_err(|e| e.to_string().lines().last().unwrap_or("invalid").trim().to_owned());
    cache.insert(pattern.to_owned(), r.clone());
    r
}

/// A string of a project's list as a matcher: `/body/flags` is a regular
/// expression (`i` is the only flag read), any other text is itself.
pub(super) enum TextMatcher {
    Exact(String),
    Re(Arc<Regex>),
}

impl TextMatcher {
    pub fn parse(s: &str) -> Result<TextMatcher, String> {
        if let Some(rest) = s.strip_prefix('/')
            && let Some(close) = rest.rfind('/')
        {
            let (body, flags) = (&rest[..close], &rest[close + 1..]);
            if !body.is_empty() && flags.chars().all(|c| c.is_ascii_alphabetic()) {
                let body = if flags.contains('i') { format!("(?i){body}") } else { body.to_owned() };
                return regex_of(&body).map(TextMatcher::Re);
            }
        }
        Ok(TextMatcher::Exact(s.to_owned()))
    }

    pub fn is_match(&self, s: &str) -> bool {
        match self {
            TextMatcher::Exact(t) => t == s,
            TextMatcher::Re(r) => r.is_match(s),
        }
    }
}

/// Whether `node` is the first node of `kinds` in its file: where a problem
/// with an option, which belongs to the whole file, is reported once.
pub(super) fn first_of_kinds(node: Node<'_>, kinds: &[&str]) -> bool {
    let mut root = node;
    while let Some(p) = root.parent() {
        root = p;
    }
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if kinds.contains(&n.kind()) {
            return n.id() == node.id();
        }
        let mut c = n.walk();
        let kids: Vec<Node<'_>> = n.children(&mut c).collect();
        stack.extend(kids.into_iter().rev());
    }
    false
}

/// An option problem as a finding on the file's first node of `kinds`.
pub(super) fn option_problem(node: Node<'_>, kinds: &[&str], out: &mut Vec<NativeFinding>, message: impl Into<String>) {
    if first_of_kinds(node, kinds) {
        out.push(at(node, message));
    }
}

/// A rule with `needs_options`, over `kinds`, in the JavaScript family.
macro_rules! orule {
    ($name:ident, $id:literal, $needs:expr, $kinds:expr, $semantic:expr, |$n:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl crate::NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [codelaw_core::Language] {
                super::JS
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn needs_semantic(&self) -> bool {
                $semantic
            }
            fn needs_options(&self) -> &'static [&'static str] {
                $needs
            }
            fn check(&self, node: codelaw_lang::SyntaxNode<'_>, $ctx: &crate::NativeContext<'_>, $out: &mut Vec<crate::NativeFinding>) {
                let $n = node.get_inner_node();
                $body
            }
        }
    };
}
pub(super) use orule;

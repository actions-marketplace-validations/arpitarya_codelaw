use ast_grep_core::Language as AgLanguage;
use ast_grep_core::matcher::{Pattern, PatternBuilder, PatternError};
use ast_grep_core::tree_sitter::{LanguageExt, StrDoc, TSLanguage};
use ast_grep_language as agl;
use codelaw_core::Language;
use std::borrow::Cow;

/// One tree-sitter grammar. JSX parses with the JavaScript grammar; TSX has
/// its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Grammar {
    JavaScript,
    TypeScript,
    Tsx,
    Python,
    Json,
    Yaml,
    Toml,
    Markdown,
    /// Markdown's inline grammar (links, emphasis, code spans), run on the
    /// block grammar's `inline` nodes.
    MarkdownInline,
    Css,
    Scss,
    Html,
}

impl Grammar {
    pub fn for_language(language: Language) -> Grammar {
        match language {
            Language::Javascript | Language::Jsx => Grammar::JavaScript,
            Language::Typescript => Grammar::TypeScript,
            Language::Tsx => Grammar::Tsx,
            Language::Python => Grammar::Python,
            Language::Json => Grammar::Json,
            Language::Yaml => Grammar::Yaml,
            Language::Toml => Grammar::Toml,
            Language::Markdown => Grammar::Markdown,
            Language::Css => Grammar::Css,
            Language::Scss => Grammar::Scss,
            Language::Html => Grammar::Html,
        }
    }

    /// Every grammar a language's rules compile for: the primary one, then
    /// any that parse its embedded parts.
    pub fn all_for_language(language: Language) -> &'static [Grammar] {
        match language {
            Language::Markdown => &[Grammar::Markdown, Grammar::MarkdownInline],
            Language::Javascript | Language::Jsx => &[Grammar::JavaScript],
            Language::Typescript => &[Grammar::TypeScript],
            Language::Tsx => &[Grammar::Tsx],
            Language::Python => &[Grammar::Python],
            Language::Json => &[Grammar::Json],
            Language::Yaml => &[Grammar::Yaml],
            Language::Toml => &[Grammar::Toml],
            Language::Css => &[Grammar::Css],
            Language::Scss => &[Grammar::Scss],
            Language::Html => &[Grammar::Html],
        }
    }
}

/// Delegates one method to the ast-grep language for each built-in grammar.
macro_rules! delegate {
    ($self:ident, $g:ident => $e:expr, toml => $t:expr) => {
        match $self {
            Grammar::JavaScript => { let $g = agl::JavaScript; $e }
            Grammar::TypeScript => { let $g = agl::TypeScript; $e }
            Grammar::Tsx => { let $g = agl::Tsx; $e }
            Grammar::Python => { let $g = agl::Python; $e }
            Grammar::Json => { let $g = agl::Json; $e }
            Grammar::Yaml => { let $g = agl::Yaml; $e }
            Grammar::Markdown | Grammar::MarkdownInline => { let $g = agl::Markdown; $e }
            Grammar::Css => { let $g = agl::Css; $e }
            Grammar::Html => { let $g = agl::Html; $e }
            Grammar::Toml => $t,
            // SCSS is codelaw's own grammar, patterned like CSS: it borrows
            // CSS's pattern syntax (`$` is not an SCSS identifier start in
            // patterns, so the CSS expando applies).
            Grammar::Scss => { let $g = agl::Css; $e }
        }
    };
}

fn toml_language() -> TSLanguage {
    tree_sitter_toml_ng::LANGUAGE.into()
}

/// `$` is not legal in a TOML bare key; `_` is, so patterns use it.
const TOML_EXPANDO: char = '_';

fn toml_pre_process(query: &str) -> Cow<'_, str> {
    // Same rule as ast-grep's built-in expando languages: `$A`, `$$A`,
    // `$$$` and `$_` become the expando character.
    let mut out = String::with_capacity(query.len());
    let mut dollars = 0;
    for c in query.chars() {
        if c == '$' {
            dollars += 1;
            continue;
        }
        let replace = matches!(c, 'A'..='Z' | '_') || dollars == 3;
        let sigil = if replace { TOML_EXPANDO } else { '$' };
        out.extend(std::iter::repeat_n(sigil, dollars));
        dollars = 0;
        out.push(c);
    }
    let sigil = if dollars == 3 { TOML_EXPANDO } else { '$' };
    out.extend(std::iter::repeat_n(sigil, dollars));
    Cow::Owned(out)
}

/// Run `f` on a copy of `name` that is followed by a NUL byte (W-169).
///
/// Tree-sitter looks a node kind or field name up with
/// `strncmp(symbol, name, len)`. The `strncmp` linked into the musl binaries
/// (maturin builds them with zig) reads `name[len]` when the first `len`
/// bytes agree, one past the end of the string; when the string's allocation
/// ends at the end of a page — musl's allocator does that — the read faults
/// and the process dies with SIGSEGV. Every lookup goes through here, so the
/// byte past `len` is always this copy's NUL.
fn with_nul<R>(name: &str, f: impl FnOnce(&str) -> R) -> R {
    const STACK: usize = 128;
    let n = name.len();
    if n < STACK {
        let mut buf = [0u8; STACK];
        buf[..n].copy_from_slice(name.as_bytes());
        f(std::str::from_utf8(&buf[..n]).expect("a copy of a str"))
    } else {
        let mut buf = String::with_capacity(n + 1);
        buf.push_str(name);
        buf.push('\0');
        f(&buf[..n])
    }
}

impl AgLanguage for Grammar {
    fn kind_to_id(&self, kind: &str) -> u16 {
        with_nul(kind, |k| self.get_ts_language().id_for_node_kind(k, true))
    }

    fn field_to_id(&self, field: &str) -> Option<u16> {
        with_nul(field, |f| self.get_ts_language().field_id_for_name(f)).map(|f| f.get())
    }

    fn expando_char(&self) -> char {
        delegate!(self, g => g.expando_char(), toml => TOML_EXPANDO)
    }

    fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str> {
        delegate!(self, g => g.pre_process_pattern(query), toml => toml_pre_process(query))
    }

    fn build_pattern(&self, builder: &PatternBuilder) -> Result<Pattern, PatternError> {
        builder.build(|src| StrDoc::try_new(src, *self))
    }
}

impl LanguageExt for Grammar {
    fn get_ts_language(&self) -> TSLanguage {
        if *self == Grammar::MarkdownInline {
            return tree_sitter_md::INLINE_LANGUAGE.into();
        }
        if *self == Grammar::Scss {
            return tree_sitter_scss::LANGUAGE.into();
        }
        delegate!(self, g => g.get_ts_language(), toml => toml_language())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// W-169: the name tree-sitter reads is a copy whose next byte is a NUL,
    /// short or long, so a `strncmp` that reads one past `len` reads that.
    #[test]
    fn a_name_lookup_reads_a_nul_terminated_copy() {
        for name in ["identifier", "x", "", &"k".repeat(127), &"k".repeat(128), &"k".repeat(300)] {
            let (same, len, next) = with_nul(name, |c| {
                // SAFETY: `with_nul` hands out a prefix of a buffer one byte
                // longer than it, so `c.len()` is in bounds of that buffer.
                let next = unsafe { *c.as_ptr().add(c.len()) };
                (c == name, c.len(), next)
            });
            assert!(same && len == name.len(), "the copy of a {}-byte name", name.len());
            assert_eq!(next, 0, "the byte after a {}-byte name", name.len());
        }
    }

    #[test]
    fn lookups_still_find_kinds_and_fields() {
        assert_ne!(Grammar::JavaScript.kind_to_id("identifier"), 0);
        assert_eq!(Grammar::JavaScript.kind_to_id("no_such_kind"), 0);
        assert!(Grammar::JavaScript.field_to_id("body").is_some());
        assert!(Grammar::JavaScript.field_to_id("no_such_field").is_none());
    }
}

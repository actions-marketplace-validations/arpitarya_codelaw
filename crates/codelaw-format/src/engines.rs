//! One function per engine of SR-FORMATTER decision 2, and the parse and
//! data checks the verifier uses.

use crate::FormatError;
use crate::markers;
use crate::options::{self, FormatOptions};
use codelaw_core::Language;
use std::borrow::Cow;
use std::path::Path;

fn engine_error(e: impl std::fmt::Display) -> FormatError {
    FormatError::Engine(e.to_string())
}

/// Format with the language's engine, its ignore markers honoured
/// (decision 6, `markers`).
pub(crate) fn format(language: Language, path: &str, text: &str, o: &FormatOptions) -> Result<String, FormatError> {
    use markers::{BIOME, PRETTIER};
    match language {
        Language::Javascript | Language::Jsx | Language::Typescript | Language::Tsx => {
            markers::native(text, &["//", "/*"], BIOME, |t| biome_js(language, path, t, o, o.line_width))
        }
        Language::Json => markers::native(text, &["//", "/*"], BIOME, |t| biome_json(t, o, o.line_width)),
        Language::Css => markers::native(text, &["/*"], PRETTIER, |t| crate::oxc::css(t, o, o.line_width, false)),
        Language::Scss => markers::native(text, &["//", "/*"], PRETTIER, |t| crate::oxc::css(t, o, o.line_width, true)),
        Language::Python => markers::python(text, |t| ruff(path, t, o, o.python_line_width)),
        Language::Yaml => markers::native(text, &["#"], PRETTIER, |t| crate::oxc::yaml(t, o, o.line_width)),
        Language::Toml => markers::toml(text, |t| toml_fmt(t, o, o.line_width)),
        Language::Markdown => markers::native(text, &["<!--"], PRETTIER, |t| crate::oxc::markdown(t, o)),
        Language::Html => markers::native(text, &["<!--"], PRETTIER, |t| html(t, o)),
    }
}

/// The parse error, if the engine's own parser rejects `text`.
pub(crate) fn parse_error(language: Language, path: &str, text: &str) -> Option<String> {
    let bad = match language {
        Language::Javascript | Language::Jsx | Language::Typescript | Language::Tsx => js_parse(language, path, text).0.has_errors(),
        Language::Json => biome_json_parser::parse_json(text, json_parser_options()).has_errors(),
        Language::Css => return crate::oxc::css_parse_error(text, false),
        Language::Scss => return crate::oxc::css_parse_error(text, true),
        Language::Python => ruff_python_parser::parse_module(text).is_err(),
        Language::Yaml => return crate::oxc::yaml_parse_error(text),
        Language::Toml => !taplo::parser::parse(text).errors.is_empty(),
        Language::Markdown => false,
        Language::Html => return markup_fmt::format_text(text, markup_fmt::Language::Html, &Default::default(), |c, _| Ok(c.into())).err().map(|e| e.to_string()),
    };
    bad.then(|| format!("{} rejects it", crate::engine(language)))
}

/// For the data languages, whether both texts hold the same data. A text
/// the data reader cannot read is not compared.
pub(crate) fn same_meaning(language: Language, _path: &str, before: &str, after: &str) -> bool {
    match language {
        Language::Json => match (serde_json::from_str::<serde_json::Value>(before), serde_json::from_str::<serde_json::Value>(after)) {
            (Ok(a), Ok(b)) => a == b,
            (Ok(_), Err(_)) => false,
            _ => true,
        },
        Language::Toml => match (before.parse::<toml::Table>(), after.parse::<toml::Table>()) {
            (Ok(a), Ok(b)) => a == b,
            (Ok(_), Err(_)) => false,
            _ => true,
        },
        Language::Yaml => match (serde_saphyr::from_multiple::<serde_json::Value>(before), serde_saphyr::from_multiple::<serde_json::Value>(after)) {
            (Ok(a), Ok(b)) => a == b,
            (Ok(_), Err(_)) => false,
            _ => true,
        },
        _ => true,
    }
}

// ---- Biome: JS, TS, JSX, TSX, JSON ----

use biome_formatter::{IndentStyle as BIndentStyle, IndentWidth, LineEnding, LineWidth, QuoteStyle as BQuoteStyle};
use biome_languages::JsFileSource;

fn js_source(language: Language, path: &str) -> JsFileSource {
    let lower = path.to_ascii_lowercase();
    match language {
        Language::Typescript if lower.ends_with(".d.ts") || lower.ends_with(".d.mts") || lower.ends_with(".d.cts") => JsFileSource::d_ts(),
        Language::Typescript => JsFileSource::ts(),
        Language::Tsx => JsFileSource::tsx(),
        // `.js` may hold JSX, as Prettier's and Biome's own defaults allow.
        _ => JsFileSource::jsx(),
    }
}

/// Parse JavaScript as a module, then — if that fails and the file is not
/// `.mjs` — as a script, as Prettier's Babel parser does: a script may
/// hold what strict mode forbids (`071`, `with`, `delete x`, a parameter
/// named `eval`). TypeScript is parsed once, as a module (W-141).
fn js_parse(language: Language, path: &str, text: &str) -> (biome_js_parser::Parse<biome_js_parser::AnyJsRoot>, JsFileSource) {
    let source = js_source(language, path);
    let parsed = biome_js_parser::parse(text, source, biome_js_parser::JsParserOptions::default());
    let module_only = path.to_ascii_lowercase().ends_with(".mjs");
    if !parsed.has_errors() || !matches!(language, Language::Javascript | Language::Jsx) || module_only {
        return (parsed, source);
    }
    let script = source.with_module_kind(biome_languages::javascript::ModuleKind::Script);
    let retry = biome_js_parser::parse(text, script, biome_js_parser::JsParserOptions::default());
    if retry.has_errors() { (parsed, source) } else { (retry, script) }
}

fn biome_indent(o: &FormatOptions) -> BIndentStyle {
    match o.indent_style {
        options::IndentStyle::Space => BIndentStyle::Space,
        options::IndentStyle::Tab => BIndentStyle::Tab,
    }
}

fn biome_eol(o: &FormatOptions) -> LineEnding {
    match o.eol() {
        options::EndOfLine::Lf => LineEnding::Lf,
        options::EndOfLine::Crlf => LineEnding::Crlf,
    }
}

fn biome_widths(o: &FormatOptions, width: u16) -> Result<(IndentWidth, LineWidth), FormatError> {
    let indent = IndentWidth::try_from(o.indent_width).map_err(engine_error)?;
    let line = LineWidth::try_from(width).map_err(engine_error)?;
    Ok((indent, line))
}

fn biome_js(language: Language, path: &str, text: &str, o: &FormatOptions, width: u16) -> Result<String, FormatError> {
    use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, QuoteProperties, Semicolons, trailing_commas::TrailingCommas};
    let (parsed, source) = js_parse(language, path, text);
    if parsed.has_errors() {
        return Err(FormatError::Syntax("biome_js_parser rejects it".into()));
    }
    let (indent, line) = biome_widths(o, width)?;
    let options = JsFormatOptions::new(source)
        .with_indent_style(biome_indent(o))
        .with_indent_width(indent)
        .with_line_width(line)
        .with_line_ending(biome_eol(o))
        .with_quote_style(match o.quote_style {
            options::QuoteStyle::Double => BQuoteStyle::Double,
            options::QuoteStyle::Single => BQuoteStyle::Single,
        })
        .with_semicolons(match o.semicolons {
            options::Semicolons::Always => Semicolons::Always,
            options::Semicolons::AsNeeded => Semicolons::AsNeeded,
        })
        .with_trailing_commas(match o.trailing_commas {
            options::TrailingCommas::All => TrailingCommas::All,
            options::TrailingCommas::Es5 => TrailingCommas::Es5,
            options::TrailingCommas::None => TrailingCommas::None,
        })
        .with_arrow_parentheses(match o.arrow_parens {
            options::ArrowParens::Always => ArrowParentheses::Always,
            options::ArrowParens::Avoid => ArrowParentheses::AsNeeded,
        })
        .with_bracket_spacing(o.bracket_spacing.into())
        .with_jsx_quote_style(match o.jsx_quote_style {
            options::QuoteStyle::Double => BQuoteStyle::Double,
            options::QuoteStyle::Single => BQuoteStyle::Single,
        })
        .with_quote_properties(match o.quote_props {
            options::QuoteProps::AsNeeded => QuoteProperties::AsNeeded,
            options::QuoteProps::Preserve => QuoteProperties::Preserve,
        });
    let formatted = biome_js_formatter::format_node(options, &parsed.syntax(), Vec::new()).map_err(engine_error)?;
    Ok(formatted.print().map_err(engine_error)?.into_code())
}

fn json_parser_options() -> biome_json_parser::JsonParserOptions {
    // Prettier's `json` parser accepts comments and trailing commas; so
    // does this, and the output drops the commas.
    biome_json_parser::JsonParserOptions::default().with_allow_comments().with_allow_trailing_commas()
}

fn biome_json(text: &str, o: &FormatOptions, width: u16) -> Result<String, FormatError> {
    use biome_json_formatter::context::JsonFormatOptions;
    let parsed = biome_json_parser::parse_json(text, json_parser_options());
    if parsed.has_errors() {
        return Err(FormatError::Syntax("biome_json_parser rejects it".into()));
    }
    let (indent, line) = biome_widths(o, width)?;
    let options =
        JsonFormatOptions::new().with_indent_style(biome_indent(o)).with_indent_width(indent).with_line_width(line).with_line_ending(biome_eol(o));
    let formatted = biome_json_formatter::format_node(options, &parsed.syntax()).map_err(engine_error)?;
    Ok(formatted.print().map_err(engine_error)?.into_code())
}

// ---- Ruff: Python ----

fn ruff(path: &str, text: &str, o: &FormatOptions, width: u16) -> Result<String, FormatError> {
    use ruff_python_formatter::{FormatModuleError, PyFormatOptions};
    let indent = ruff_formatter::IndentWidth::try_from(o.python_indent_width).map_err(engine_error)?;
    let line = ruff_formatter::LineWidth::try_from(width).map_err(engine_error)?;
    let options = PyFormatOptions::from_extension(Path::new(if path.is_empty() { "x.py" } else { path }))
        .with_indent_width(indent)
        .with_line_width(line)
        .with_indent_style(match o.indent_style {
            options::IndentStyle::Space => ruff_formatter::IndentStyle::Space,
            options::IndentStyle::Tab => ruff_formatter::IndentStyle::Tab,
        })
        .with_line_ending(match o.eol() {
            options::EndOfLine::Lf => ruff_formatter::printer::LineEnding::LineFeed,
            options::EndOfLine::Crlf => ruff_formatter::printer::LineEnding::CarriageReturnLineFeed,
        });
    match ruff_python_formatter::format_module_source(text, options) {
        Ok(printed) => Ok(printed.into_code()),
        Err(FormatModuleError::ParseError(e)) => Err(FormatError::Syntax(e.error.to_string())),
        Err(e) => Err(engine_error(e)),
    }
}

// ---- taplo: TOML ----

fn toml_fmt(text: &str, o: &FormatOptions, width: u16) -> Result<String, FormatError> {
    let parsed = taplo::parser::parse(text);
    if let Some(e) = parsed.errors.first() {
        return Err(FormatError::Syntax(e.message.clone()));
    }
    let options = taplo::formatter::Options {
        column_width: width as usize,
        indent_string: match o.indent_style {
            options::IndentStyle::Space => " ".repeat(o.indent_width as usize),
            options::IndentStyle::Tab => "\t".into(),
        },
        crlf: o.eol() == options::EndOfLine::Crlf,
        ..Default::default()
    };
    Ok(taplo::formatter::format_syntax(parsed.into_syntax(), options))
}

// ---- Embedded blocks: Markdown fences (`oxc::markdown`), HTML embeds ----

/// The language a Markdown fence tag or an HTML embed names.
fn tag_language(tag: &str) -> Option<(Language, &'static str)> {
    let tag = tag.split([' ', ',', '{']).next().unwrap_or_default().to_ascii_lowercase();
    Some(match tag.as_str() {
        "js" | "javascript" | "mjs" | "cjs" | "jsx" => (Language::Jsx, "x.jsx"),
        "ts" | "typescript" | "mts" | "cts" => (Language::Typescript, "x.ts"),
        "tsx" => (Language::Tsx, "x.tsx"),
        "json" | "jsonc" | "json5" => (Language::Json, "x.json"),
        "css" => (Language::Css, "x.css"),
        "scss" => (Language::Scss, "x.scss"),
        "py" | "python" | "python3" => (Language::Python, "x.py"),
        "pyi" => (Language::Python, "x.pyi"),
        "yaml" | "yml" => (Language::Yaml, "x.yaml"),
        "toml" => (Language::Toml, "x.toml"),
        "html" => (Language::Html, "x.html"),
        _ => return None,
    })
}

/// Format an embedded block; `None` leaves it as written (a block that
/// does not parse is not the file's failure, as in Prettier).
pub(crate) fn embedded(tag: &str, code: &str, o: &FormatOptions, width: u16) -> Option<String> {
    let (language, path) = tag_language(tag)?;
    let mut inner = o.clone();
    inner.line_width = width.max(20);
    if language == Language::Python {
        inner.python_line_width = width.max(20);
    }
    crate::format(language, path, code, &inner).ok()
}

// ---- markup_fmt: HTML ----

/// A `style=""` attribute as Prettier writes it (W-141): each declaration
/// formatted as CSS, joined by `; `, with no `;` after the last. `None`
/// leaves the attribute as written — a value that does not parse, or one
/// with a comment.
fn style_attribute(code: &str, o: &FormatOptions) -> Option<String> {
    if code.trim().is_empty() || code.contains("/*") {
        return None;
    }
    let mut inner = o.clone();
    inner.line_width = 320;
    let rule = crate::format(Language::Css, "x.css", &format!("a{{{code}}}"), &inner).ok()?;
    let body = rule.strip_prefix("a {\n")?.trim_end().strip_suffix('}')?;
    let mut declarations = Vec::new();
    for line in body.lines() {
        declarations.push(line.trim().strip_suffix(';')?);
    }
    Some(declarations.join("; "))
}

/// The doctype as Prettier prints it (W-141): `<!doctype html>` for the
/// HTML5 doctype; any other keeps its keyword as written, its value's first
/// `html` lower-cased and its whitespace runs made one space. Comments and
/// whitespace may come before it. (Prettier keeps the keyword of
/// `<!DOCTYPE HTML>` and lower-cases only the value, which its own second
/// run then changes again; codelaw's output must be stable, decision 4.)
fn prettier_doctype(text: &str) -> Option<String> {
    let mut at = 0;
    loop {
        let rest = text[at..].trim_start_matches(|c: char| c.is_whitespace() || c == '\u{feff}');
        at = text.len() - rest.len();
        if rest.starts_with("<!--") {
            at += rest.find("-->")? + 3;
        } else {
            break;
        }
    }
    let rest = &text[at..];
    if rest.len() < 9 || !rest.is_char_boundary(9) || !rest[2..9].eq_ignore_ascii_case("doctype") || !rest.starts_with("<!") {
        return None;
    }
    let end = rest.find('>')?;
    let value = rest[9..end].split_whitespace().collect::<Vec<_>>().join(" ");
    let keyword = if value.eq_ignore_ascii_case("html") { "doctype" } else { &rest[2..9] };
    let value = match value.get(..4) {
        Some(head) if head.eq_ignore_ascii_case("html") && value[4..].chars().next().is_none_or(|c| !c.is_alphanumeric() && c != '_') => {
            format!("html{}", &value[4..])
        }
        _ => value,
    };
    let doctype = format!("<!{keyword} {value}>");
    (doctype != rest[..=end]).then(|| format!("{}{doctype}{}", &text[..at], &rest[end + 1..]))
}

/// The index just past `needle` (ASCII, any case) at or after `from`.
fn find_ci(text: &str, needle: &str, from: usize) -> Option<usize> {
    let hay = text.as_bytes();
    let n = needle.as_bytes();
    (from..=hay.len().checked_sub(n.len())?).find(|&i| hay[i..i + n.len()].eq_ignore_ascii_case(n)).map(|i| i + n.len())
}

/// `<script>` and `<style>` whose body is only whitespace, emptied
/// (W-141): Prettier prints them `<script></script>`, markup_fmt on two
/// lines. Comments, `<textarea>` bodies and a node after an ignore marker
/// are left alone.
fn empty_blank_embeds(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let mut out = String::new();
    let mut copied = 0;
    let mut at = 0;
    while let Some(lt) = lower[at..].find('<').map(|i| i + at) {
        let tail = &lower[lt..];
        if tail.starts_with("<!--") {
            at = lt + tail.find("-->").map_or(tail.len(), |e| e + 3);
            continue;
        }
        let Some(name) = ["script", "style", "textarea"]
            .into_iter()
            .find(|n| tail[1..].starts_with(n) && tail[1 + n.len()..].starts_with(|c: char| c == '>' || c == '/' || c.is_ascii_whitespace()))
        else {
            at = lt + 1;
            continue;
        };
        // The end of the opening tag, past quoted attribute values.
        let mut quote = None;
        let Some(open_end) = tail.char_indices().find_map(|(i, c)| match (quote, c) {
            (Some(q), c) if c == q => {
                quote = None;
                None
            }
            (Some(_), _) => None,
            (None, '"' | '\'') => {
                quote = Some(c);
                None
            }
            (None, '>') => Some(lt + i + 1),
            _ => None,
        }) else {
            break;
        };
        let close = format!("</{name}");
        let Some(body_end) = find_ci(&lower, &close, open_end).map(|e| e - close.len()) else { break };
        let body = &text[open_end..body_end];
        let ignored = text[..lt].trim_end().strip_suffix("-->").and_then(|t| t.rfind("<!--").map(|c| &t[c + 4..])).is_some_and(|c| c.trim().starts_with(markers::PRETTIER));
        if name != "textarea" && !body.is_empty() && body.trim().is_empty() && !ignored {
            out.push_str(&text[copied..open_end]);
            copied = body_end;
        }
        at = body_end;
    }
    (copied > 0).then(|| out + &text[copied..])
}

fn html(text: &str, o: &FormatOptions) -> Result<String, FormatError> {
    let doctype = prettier_doctype(text);
    let text = doctype.as_deref().unwrap_or(text);
    let emptied = empty_blank_embeds(text);
    let text = emptied.as_deref().unwrap_or(text);
    let mut options = markup_fmt::config::FormatOptions::default();
    options.layout.print_width = o.line_width as usize;
    options.layout.indent_width = o.indent_width as usize;
    options.layout.use_tabs = o.indent_style == options::IndentStyle::Tab;
    // Prettier's HTML style: indented `<script>`/`<style>` bodies, a
    // lower-case doctype, and `<!-- prettier-ignore -->` (decision 6).
    options.language.script_indent = true;
    options.language.style_indent = true;
    // The doctype keyword is `prettier_doctype`'s.
    options.language.doctype_keyword_case = markup_fmt::config::DoctypeKeywordCase::Ignore;
    // Prettier writes every void element as `<br />` (W-141); html-validate's
    // `void-style` leaves the spelling to the formatter (packs/html README).
    options.language.html_void_self_closing = Some(true);
    // An empty element's closing tag stays on its line: Prettier breaks the
    // attributes of `<button a b c></button>`, never the `></button>`.
    options.language.closing_tag_line_break_for_empty = markup_fmt::config::ClosingTagLineBreakForEmpty::Never;
    options.language.ignore_comment_directive = markers::PRETTIER.into();
    options.layout.line_break = match o.eol() {
        options::EndOfLine::Lf => markup_fmt::config::LineBreak::Lf,
        options::EndOfLine::Crlf => markup_fmt::config::LineBreak::Crlf,
    };
    markup_fmt::format_text(text, markup_fmt::Language::Html, &options, |code, hints| {
        if hints.attr && hints.ext == "css" {
            return Ok(style_attribute(code, o).map_or(Cow::Borrowed(code), Cow::Owned));
        }
        // markup_fmt hands a `<style>` body the width left after its
        // indentation but a `<script>` body the whole line; Prettier fits
        // both inside the line (W-141). A script's `indent_level` already
        // counts the body's own indent.
        let mut width = hints.print_width;
        if !hints.attr && !matches!(hints.ext, "css" | "scss" | "less" | "sass") {
            width = width.saturating_sub(hints.indent_level as usize * o.indent_width as usize);
        }
        let width = u16::try_from(width).unwrap_or(u16::MAX);
        Ok(match embedded(hints.ext, code, o, width) {
            Some(s) => Cow::Owned(s),
            None => Cow::Borrowed(code),
        })
    })
    .map_err(|e| match e {
        markup_fmt::FormatError::Syntax(s) => FormatError::Syntax(s.to_string()),
        other => engine_error(other),
    })
}

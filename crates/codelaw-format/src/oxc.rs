//! Oxc's native formatters (SR-LAW-11 D-52, ratified 2026-11-05; W-141):
//! `oxc_formatter_css` for CSS and SCSS, `oxc_formatter_yaml`,
//! `oxc_formatter_markdown`. Each run has its own arena; nothing is shared
//! between files, so the parallel walk needs no locking.

use crate::FormatError;
use crate::options::{self, FormatOptions};
use oxc_allocator::{Allocator, ArenaVec};
use oxc_formatter_core::{
    DispatchRequest, DispatchResponse, EmbeddedIr, FormatDispatcher, FormatSession, IndentStyle, IndentWidth, InputKind, LineEnding, LineWidth,
    SessionServices, push_text_with_literal_lines,
};
use std::sync::Arc;

/// The layout every Oxc formatter takes, from codelaw's options.
struct Layout {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_width: LineWidth,
    line_ending: LineEnding,
}

fn layout(o: &FormatOptions, width: u16) -> Result<Layout, FormatError> {
    Ok(Layout {
        indent_style: match o.indent_style {
            options::IndentStyle::Space => IndentStyle::Space,
            options::IndentStyle::Tab => IndentStyle::Tab,
        },
        indent_width: IndentWidth::try_from(o.indent_width).map_err(|e| FormatError::Engine(e.to_string()))?,
        line_width: LineWidth::try_from(width).map_err(|e| FormatError::Engine(e.to_string()))?,
        line_ending: match o.eol() {
            options::EndOfLine::Lf => LineEnding::Lf,
            options::EndOfLine::Crlf => LineEnding::Crlf,
        },
    })
}

fn single_quote(o: &FormatOptions) -> bool {
    o.quote_style == options::QuoteStyle::Single
}

/// CSS, or SCSS with `scss`.
pub(crate) fn css(text: &str, o: &FormatOptions, width: u16, scss: bool) -> Result<String, FormatError> {
    use oxc_formatter_css::{CssFormatOptions, CssVariant, TrailingCommas};
    let l = layout(o, width)?;
    let options = CssFormatOptions {
        indent_style: l.indent_style,
        indent_width: l.indent_width,
        line_width: l.line_width,
        line_ending: l.line_ending,
        variant: if scss { CssVariant::Scss } else { CssVariant::Css },
        single_quote: single_quote(o).into(),
        trailing_commas: if o.trailing_commas == options::TrailingCommas::None { TrailingCommas::Never } else { TrailingCommas::Always },
        sort_tailwindcss: false,
    };
    let allocator = Allocator::default();
    let formatted = oxc_formatter_css::format(&allocator, text, options).map_err(|e| FormatError::Syntax(e.to_string()))?;
    Ok(formatted.print().map_err(|e| FormatError::Engine(e.to_string()))?.into_code())
}

pub(crate) fn yaml(text: &str, o: &FormatOptions, width: u16) -> Result<String, FormatError> {
    use oxc_formatter_yaml::{TrailingCommas, YamlFormatOptions};
    let l = layout(o, width)?;
    let options = YamlFormatOptions {
        indent_style: l.indent_style,
        indent_width: l.indent_width,
        line_width: l.line_width,
        line_ending: l.line_ending,
        single_quote: single_quote(o).into(),
        bracket_spacing: o.bracket_spacing.into(),
        trailing_commas: if o.trailing_commas == options::TrailingCommas::None { TrailingCommas::Never } else { TrailingCommas::Always },
        ..Default::default()
    };
    let allocator = Allocator::default();
    let formatted = oxc_formatter_yaml::format(&allocator, text, options).map_err(|e| FormatError::Syntax(e.to_string()))?;
    Ok(formatted.print().map_err(|e| FormatError::Engine(e.to_string()))?.into_code())
}

/// Markdown. With `markdown_code_blocks`, a fenced block in a language
/// codelaw formats is handed to that language's engine (decision 2's
/// table), and front matter to YAML's, as Prettier does.
pub(crate) fn markdown(text: &str, o: &FormatOptions) -> Result<String, FormatError> {
    use oxc_formatter_markdown::{MarkdownFormatOptions, format_with_session};
    let l = layout(o, o.line_width)?;
    let options = MarkdownFormatOptions {
        indent_style: l.indent_style,
        indent_width: l.indent_width,
        line_width: l.line_width,
        line_ending: l.line_ending,
        single_quote: single_quote(o).into(),
        ..Default::default()
    };
    let allocator = Allocator::default();
    let services = SessionServices { dispatcher: o.markdown_code_blocks.then(|| dispatcher(o.clone(), l.indent_width)), ..Default::default() };
    let session = FormatSession::with_services(&allocator, InputKind::PhysicalFile, services);
    let formatted = format_with_session(&session, text, options).map_err(|e| FormatError::Syntax(e.to_string()))?;
    Ok(formatted.print().map_err(|e| FormatError::Engine(e.to_string()))?.into_code())
}

/// Formats an embedded block with codelaw's engine for its language and
/// gives it back as text: Oxc keeps each line at the fence's column. A
/// block in a language codelaw does not format, or one that does not
/// parse, is kept as written.
fn dispatcher(o: FormatOptions, indent_width: IndentWidth) -> FormatDispatcher {
    Arc::new(move |session: &FormatSession<'_>, request: DispatchRequest<'_>| {
        let Some(code) = crate::engines::embedded(request.language, request.text, &o, o.line_width) else {
            return Ok(DispatchResponse::PreserveOriginal);
        };
        let code = session.allocator().alloc_str(code.trim_end_matches(['\n', '\r']));
        let mut ir = ArenaVec::new_in(&session.allocator());
        push_text_with_literal_lines(&mut ir, code, indent_width);
        Ok(DispatchResponse::Formatted(EmbeddedIr { ir }))
    })
}

/// The parse error, if Oxc's CSS parser rejects `text` as a stylesheet.
pub(crate) fn css_parse_error(text: &str, scss: bool) -> Option<String> {
    use oxc_formatter_css::{CssFormatOptions, CssRoot, CssVariant};
    let options = CssFormatOptions { variant: if scss { CssVariant::Scss } else { CssVariant::Css }, ..Default::default() };
    let allocator = Allocator::default();
    oxc_formatter_css::parse_for_format(&allocator, text, options, CssRoot::Stylesheet).err().map(|e| e.to_string())
}

/// The parse error, if Oxc's YAML parser rejects `text`.
pub(crate) fn yaml_parse_error(text: &str) -> Option<String> {
    let allocator = Allocator::default();
    oxc_formatter_yaml::parse_for_format(&allocator, text).err().map(|e| e.to_string())
}

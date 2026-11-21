//! `js/comment-regex` and `js/file-header`: comments a project forbids, and
//! the header it requires at the top of a file. eslint-plugin-sonarjs 4.2.2's
//! `comment-regex` (S124) and `file-header` (S1451), behaviour only (LGPL-3.0,
//! L2): both are written from their documented behaviour and tests.

use super::{at, finding, opt_bool, opt_str, option_problem, regex_of};
use crate::util::text;
use codelaw_core::TextEdit;

/// A comment's text without its delimiters.
fn comment_body(t: &str) -> &str {
    if let Some(b) = t.strip_prefix("//") {
        b
    } else if let Some(b) = t.strip_prefix("/*") {
        b.strip_suffix("*/").unwrap_or(b)
    } else {
        t
    }
}

super::orule!(CommentRegex, "js/comment-regex", &["regularExpression"], &["comment"], false, |n, ctx, out| {
    let Some(pattern) = opt_str(ctx, "regularExpression") else { return };
    let re = match regex_of(pattern) {
        Ok(r) => r,
        Err(e) => return option_problem(n, &["comment"], out, format!("`regularExpression` is not a valid regular expression: {e}")),
    };
    if re.is_match(comment_body(text(n, ctx.source))) {
        out.push(at(n, opt_str(ctx, "message").unwrap_or("The regular expression matches this comment.")));
    }
});

super::orule!(FileHeader, "js/file-header", &["headerFormat"], &["program"], false, |n, ctx, out| {
    let Some(header) = opt_str(ctx, "headerFormat") else { return };
    let src = ctx.source;
    if src.trim().is_empty() {
        return;
    }
    // A shebang line is not part of the header.
    let body_start = if src.starts_with("#!") { src.find('\n').map_or(src.len(), |i| i + 1) } else { 0 };
    let rest = &src[body_start..];
    let regex = opt_bool(ctx, "isRegularExpression", false);
    let ok = if regex {
        match regex_of(&format!("^(?:{header})")) {
            Ok(r) => r.is_match(rest),
            Err(e) => return option_problem(n, &["program"], out, format!("`headerFormat` is not a valid regular expression: {e}")),
        }
    } else {
        rest.starts_with(header)
    };
    if ok {
        return;
    }
    let line_end = rest.find('\n').map_or(src.len(), |i| body_start + i);
    // A plain header is text: offer to put it first.
    let edits = if regex {
        Vec::new()
    } else {
        let sep = if header.ends_with('\n') { "" } else { "\n" };
        vec![TextEdit::new(body_start, body_start, format!("{header}{sep}"))]
    };
    out.push(finding(body_start, line_end.max(body_start), "Add or update the header of this file.", edits));
});

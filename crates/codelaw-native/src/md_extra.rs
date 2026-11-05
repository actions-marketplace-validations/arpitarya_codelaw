//! More markdownlint rules (W-139): MD026, MD028, MD043, MD044, MD054.

use crate::data_md::{heading_text, headings, inline_root, non_text_ranges, preorder, string_list};
use crate::util::text;
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;

const MD: &[Language] = &[Language::Markdown];

macro_rules! md_rule {
    ($name:ident, $id:literal, $kinds:expr, |$node:ident, $ctx:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                MD
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn check(&self, node: SyntaxNode<'_>, $ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let $node = node.get_inner_node();
                $body
            }
        }
    };
}

fn finding(start: usize, end: usize, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message: message.into(), edits }
}

md_rule!(NoTrailingPunctuation, "md/no-trailing-punctuation", &["document"], |doc, ctx, out| {
    let src = ctx.source;
    let punct = ctx.options.get("punctuation").and_then(|v| v.as_str()).unwrap_or(".,;:!。，；：！").to_owned();
    for h in headings(doc) {
        let Some(c) = h.content else { continue };
        let raw = &src[c.byte_range()];
        let mut body = raw.trim_end();
        if !h.setext {
            let t = body.trim_end_matches('#');
            if t.len() != body.len() && (t.is_empty() || t.ends_with(' ')) {
                body = t.trim_end();
            }
        }
        let Some(last) = body.chars().last() else { continue };
        let entity = last == ';' && body.rfind('&').is_some_and(|k| body[k + 1..body.len() - 1].chars().all(|x| x.is_ascii_alphanumeric() || x == '#') && k + 2 < body.len());
        if punct.contains(last) && !entity {
            let at = c.start_byte() + body.len() - last.len_utf8();
            out.push(finding(h.node.start_byte(), c.start_byte() + body.len(), format!("The heading ends with `{last}`: drop it"), vec![TextEdit::delete(at, at + last.len_utf8())]));
        }
    }
});

md_rule!(NoBlanksBlockquote, "md/no-blanks-blockquote", &["document"], |doc, ctx, out| {
    let src = ctx.source;
    for n in preorder(doc) {
        let mut c = n.walk();
        let kids: Vec<_> = n.named_children(&mut c).collect();
        for w in kids.windows(2) {
            if w[0].kind() != "block_quote" || w[1].kind() != "block_quote" {
                continue;
            }
            let (a, b) = (w[0].end_byte(), w[1].start_byte());
            let gap = &src[a..b];
            if !gap.trim().is_empty() || !gap.contains('\n') {
                continue;
            }
            // Each blank line between the two quotes gets a `>`.
            let mut edits = Vec::new();
            let mut at = a;
            for line in gap.split_inclusive('\n') {
                if line.trim().is_empty() && line.ends_with('\n') && at > 0 && src.as_bytes()[at - 1] == b'\n' {
                    edits.push(TextEdit::insert(at, ">"));
                }
                at += line.len();
            }
            out.push(finding(w[1].start_byte(), w[1].start_byte() + 1, "A blank line splits this blockquote from the one above: join them with `>` or separate them with text", edits));
        }
    }
});

md_rule!(RequiredHeadings, "md/required-headings", &["document"], |doc, ctx, out| {
    let want = string_list(ctx, "headings", &[]);
    if want.is_empty() {
        return;
    }
    let src = ctx.source;
    let have: Vec<String> = headings(doc).iter().map(|h| format!("{} {}", "#".repeat(h.level), heading_text(h, src))).collect();
    // Match in order; `*` matches any run of headings, `+` one or more.
    let mut i = 0;
    let mut k = 0;
    while k < want.len() {
        let w = &want[k];
        if w == "*" || w == "+" {
            let next = want.get(k + 1);
            let min = if w == "+" { 1 } else { 0 };
            let mut skipped = 0;
            while i < have.len() && next.is_none_or(|n| !have[i].eq_ignore_ascii_case(n)) {
                i += 1;
                skipped += 1;
            }
            if skipped < min {
                break;
            }
            k += 1;
            continue;
        }
        if i < have.len() && have[i].eq_ignore_ascii_case(w) {
            i += 1;
            k += 1;
        } else {
            break;
        }
    }
    if k < want.len() || (i < have.len() && want.last().is_none_or(|w| w != "*" && w != "+")) {
        let missing = want.get(k).cloned().unwrap_or_else(|| "no more headings".into());
        out.push(finding(doc.start_byte(), doc.start_byte(), format!("The headings don't follow the required outline: expected `{missing}`"), vec![]));
    }
});

md_rule!(ProperNames, "md/proper-names", &["document"], |doc, ctx, out| {
    let names = string_list(ctx, "names", &[]);
    if names.is_empty() {
        return;
    }
    let src = ctx.source;
    let code_blocks = ctx.options.get("code_blocks").and_then(|v| v.as_bool()).unwrap_or(true);
    let mut skip: Vec<(usize, usize)> = Vec::new();
    for n in preorder(doc) {
        match n.kind() {
            "fenced_code_block" | "indented_code_block" | "html_block" if !code_blocks || n.kind() == "html_block" => skip.push((n.start_byte(), n.end_byte())),
            "inline" => skip.extend(non_text_ranges(n)),
            "front_matter" | "minus_metadata" | "plus_metadata" => skip.push((n.start_byte(), n.end_byte())),
            _ => {}
        }
    }
    let lower = src.to_lowercase();
    if lower.len() != src.len() {
        return; // case mapping changed byte offsets
    }
    for name in &names {
        let l = name.to_lowercase();
        let mut from = 0;
        while let Some(o) = lower[from..].find(&l) {
            let s = from + o;
            let e = s + l.len();
            from = e;
            let word = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
            if word(src[..s].chars().last()) || word(src[e..].chars().next()) || &src[s..e] == name.as_str() || skip.iter().any(|(a, b)| s < *b && e > *a) {
                continue;
            }
            // A link target or a URL is not prose.
            if src[..s].ends_with(['/', '.', '@', '#']) || src[e..].starts_with(['.', '/']) && src[e..].chars().nth(1).is_some_and(|c| c.is_alphanumeric()) {
                continue;
            }
            out.push(finding(s, e, format!("Write `{}` as `{name}`", &src[s..e]), vec![TextEdit::new(s, e, name.clone())]));
        }
    }
});

md_rule!(LinkImageStyle, "md/link-image-style", &["inline"], |root, ctx, out| {
    if !inline_root(root) {
        return;
    }
    let allow = |k: &str| ctx.options.get(k).and_then(|v| v.as_bool()).unwrap_or(true);
    let src = ctx.source;
    for n in preorder(root) {
        let (style, key) = match n.kind() {
            "uri_autolink" | "email_autolink" => ("autolink", "autolink"),
            "inline_link" | "image" => {
                let t = text(n, src);
                let dest = t.rsplit("](").next().unwrap_or("").trim_end_matches(')');
                let label = t.trim_start_matches('!').trim_start_matches('[').split("](").next().unwrap_or("");
                if allow("inline") && !(label == dest && !allow("url_inline")) {
                    continue;
                }
                if label == dest && !allow("url_inline") { ("inline link whose text is its URL", "url_inline") } else { ("inline", "inline") }
            }
            "full_reference_link" => ("full reference", "full"),
            "collapsed_reference_link" => ("collapsed reference", "collapsed"),
            "shortcut_link" => ("shortcut reference", "shortcut"),
            _ => continue,
        };
        if !allow(key) {
            out.push(finding(n.start_byte(), n.end_byte(), format!("This project doesn't use {style} links or images"), vec![]));
        }
    }
});

pub static RULES: &[&dyn NativeRule] = &[&NoTrailingPunctuation, &NoBlanksBlockquote, &RequiredHeadings, &ProperNames, &LinkImageStyle];

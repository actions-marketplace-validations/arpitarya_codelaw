//! A JSDoc block comment, read the way the plugin's comment parser reads it:
//! lines with their `*` delimiter taken off, a description, and tags — a tag
//! starts on a line whose content starts with `@`, outside a fenced code
//! block, and runs to the next tag; its `{type}` (braces balanced, across
//! lines), its name (an optional `[name=default]` too) and its description
//! follow. Every position is a byte offset into the file.

/// Tags whose first word is not a name (so it stays in the description).
const NO_NAMES: &[&str] = &[
    "access", "author", "default", "defaultvalue", "description", "example", "exception", "file", "fileoverview", "overview", "kind", "license", "return", "returns", "since", "summary", "throws", "version", "variation",
];
/// Tags with no `{type}`: a brace after them is text.
const NO_TYPES: &[&str] = &["default", "defaultvalue", "description", "example", "file", "fileoverview", "license", "overview", "see", "summary"];

#[derive(Clone, Debug)]
pub struct Line {
    /// Start of the line (the comment's start, for the first line).
    pub start: usize,
    /// End of the line, before its line break (the comment's end, for the last).
    pub end: usize,
    /// Start of the content: after `/**` or `*` and the spaces that follow it.
    pub body_start: usize,
    /// End of the content: before `*/` and trailing spaces.
    pub body_end: usize,
    /// The line holds the closing `*/`.
    pub closes: bool,
    /// Inside a fenced code block (a description line, never a tag).
    pub fenced: bool,
}

#[derive(Clone, Debug)]
pub struct Tag {
    /// The tag's name without `@`.
    pub tag: String,
    /// Offset of `@`.
    pub at: usize,
    pub tag_end: usize,
    /// The first and last lines the tag spans (blank lines before the next tag included).
    pub first_line: usize,
    pub last_line: usize,
    /// The `{type}`: offset of `{`, of the first and last type byte, and of `}`.
    pub type_open: Option<usize>,
    pub type_start: usize,
    pub type_end: usize,
    /// The name token, with its offsets (`[name=default]` kept whole).
    pub name: String,
    pub name_start: usize,
    pub name_end: usize,
    /// Whether the name was written `[name]` / `[name=default]`.
    pub optional: bool,
    /// The default value after `=`.
    pub default: Option<String>,
    /// Where the description starts on the tag's first line.
    pub desc_start: usize,
    /// The line the description starts on.
    pub desc_line: usize,
    /// The unclosed `[` of an optional name, when there is one.
    pub name_problem: Option<String>,
}

impl Tag {
    pub fn type_text<'a>(&self, src: &'a str) -> Option<&'a str> {
        self.type_open.map(|_| &src[self.type_start..self.type_end])
    }
}

#[derive(Clone, Debug)]
pub struct Block<'s> {
    pub src: &'s str,
    pub start: usize,
    pub end: usize,
    pub lines: Vec<Line>,
    pub tags: Vec<Tag>,
}

/// Whether a comment's text is a JSDoc block: `/**` then not `*` and not the
/// end (`/***` and `/**/` are not).
pub fn is_jsdoc(text: &str) -> bool {
    text.starts_with("/**") && text.len() >= 5 && text.as_bytes()[3] != b'*' && text.ends_with("*/")
}

impl<'s> Block<'s> {
    pub fn parse(src: &'s str, start: usize, end: usize) -> Option<Block<'s>> {
        let text = &src[start..end];
        if !is_jsdoc(text) {
            return None;
        }
        let mut lines = split_lines(src, start, end);
        let mut in_fence = false;
        for l in &mut lines {
            let body = &src[l.body_start..l.body_end];
            if body.starts_with("```") {
                in_fence = !in_fence;
                l.fenced = true;
            } else {
                l.fenced = in_fence;
            }
        }
        let mut b = Block { src, start, end, lines, tags: Vec::new() };
        b.read_tags();
        Some(b)
    }

    pub fn body(&self, i: usize) -> &'s str {
        let l = &self.lines[i];
        &self.src[l.body_start..l.body_end]
    }

    pub fn line_index_at(&self, offset: usize) -> usize {
        self.lines.iter().position(|l| offset <= l.end).unwrap_or(self.lines.len() - 1)
    }

    fn read_tags(&mut self) {
        let mut starts: Vec<usize> = Vec::new();
        for (i, l) in self.lines.iter().enumerate() {
            if l.fenced {
                continue;
            }
            let body = &self.src[l.body_start..l.body_end];
            if body.starts_with('@') && body.len() > 1 && !body[1..].starts_with(char::is_whitespace) {
                starts.push(i);
            }
        }
        for (n, &i) in starts.iter().enumerate() {
            let last = starts.get(n + 1).map_or(self.lines.len() - 1, |&nx| nx - 1);
            let t = self.read_tag(i, last);
            self.tags.push(t);
        }
    }

    fn read_tag(&self, first: usize, last: usize) -> Tag {
        let src = self.src;
        let l = &self.lines[first];
        let at = l.body_start;
        let body = &src[at..l.body_end];
        let tag_len = body.find(|c: char| c.is_whitespace()).unwrap_or(body.len());
        let tag = body[1..tag_len].to_owned();
        let mut pos = at + tag_len;
        let tag_end = pos;
        let mut t = Tag {
            tag,
            at,
            tag_end,
            first_line: first,
            last_line: last,
            type_open: None,
            type_start: pos,
            type_end: pos,
            name: String::new(),
            name_start: pos,
            name_end: pos,
            optional: false,
            default: None,
            desc_start: pos,
            desc_line: first,
            name_problem: None,
        };
        let skip_ws = |p: usize| -> usize { p + src[p..].len() - src[p..].trim_start_matches([' ', '\t']).len() };
        // The region the tag's own line (and the lines it continues onto) may read from.
        let limit = self.lines[last].body_end;
        pos = skip_ws(pos);
        if !NO_TYPES.contains(&t.tag.as_str()) && src[pos..limit.max(pos)].starts_with('{') {
            // Balanced braces, possibly across lines.
            let mut depth = 0usize;
            let mut i = pos;
            let bytes = src.as_bytes();
            let mut close = None;
            while i < limit {
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            close = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if let Some(c) = close {
                t.type_open = Some(pos);
                t.type_start = pos + 1;
                t.type_end = c;
                pos = skip_ws(c + 1);
            }
        }
        let cur = self.line_index_at(pos).max(first);
        let line_end = self.lines[cur].body_end.max(pos);
        t.desc_line = cur;
        if !NO_NAMES.contains(&t.tag.as_str()) && pos < line_end {
            let rest = &src[pos..line_end];
            if let Some(first_ch) = rest.chars().next() {
                if matches!(t.tag.as_str(), "template" | "typeParam") {
                    // A comma-separated list of names, each maybe `[name=default]`.
                    let rb = rest.as_bytes();
                    let mut i = 0;
                    loop {
                        if rb[i] == b'[' {
                            let mut depth = 0;
                            while i < rb.len() {
                                match rb[i] {
                                    b'[' => depth += 1,
                                    b']' => {
                                        depth -= 1;
                                        if depth == 0 {
                                            i += 1;
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                                i += 1;
                            }
                        } else {
                            while i < rb.len() && !rb[i].is_ascii_whitespace() && rb[i] != b',' {
                                i += 1;
                            }
                        }
                        let mut j = i;
                        while j < rb.len() && (rb[j] == b' ' || rb[j] == b'\t') {
                            j += 1;
                        }
                        if j < rb.len() && rb[j] == b',' {
                            i = j + 1;
                            while i < rb.len() && (rb[i] == b' ' || rb[i] == b'\t') {
                                i += 1;
                            }
                            if i >= rb.len() {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                    t.name_start = pos;
                    t.name_end = pos + i;
                    t.name = rest[..i].to_owned();
                    pos += i;
                } else if first_ch == '[' {
                    let mut depth = 0i32;
                    let mut quote: Option<char> = None;
                    let mut end = None;
                    for (k, c) in rest.char_indices() {
                        match (quote, c) {
                            (Some(q), c) if c == q => quote = None,
                            (Some(_), _) => {}
                            (None, '"' | '\'' | '`') => quote = Some(c),
                            (None, '[') => depth += 1,
                            (None, ']') => {
                                depth -= 1;
                                if depth == 0 {
                                    end = Some(k + 1);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    match end {
                        Some(e) => {
                            t.name_start = pos;
                            t.name_end = pos + e;
                            t.optional = true;
                            let inner = &rest[1..e - 1];
                            let (nm, def) = match inner.find('=') {
                                Some(k) => (inner[..k].trim(), Some(inner[k + 1..].trim().to_owned())),
                                None => (inner.trim(), None),
                            };
                            t.name = nm.to_owned();
                            t.default = def;
                            pos += e;
                        }
                        None => {
                            // An unclosed bracket: the rest of the line is the name.
                            t.name_start = pos;
                            t.name_end = pos + rest.len();
                            t.name = rest.trim_end().to_owned();
                            t.name_problem = Some(format!("unterminated optional name `{}`", rest.trim_end()));
                            pos += rest.len();
                        }
                    }
                } else {
                    let len = rest.find(char::is_whitespace).unwrap_or(rest.len());
                    t.name_start = pos;
                    t.name_end = pos + len;
                    t.name = rest[..len].to_owned();
                    pos += len;
                }
            }
        }
        t.desc_start = skip_ws(pos).min(line_end).max(pos);
        t
    }

    /// The description of a tag, as the plugin gives it: the text after the
    /// tag, type and name on its first line, then the following lines'
    /// content, joined by line breaks; trailing blank lines are not part of it.
    pub fn tag_description(&self, t: &Tag) -> String {
        let mut parts: Vec<String> = Vec::new();
        let dl = &self.lines[t.desc_line];
        parts.push(self.src[t.desc_start.min(dl.body_end)..dl.body_end].to_owned());
        for i in t.desc_line + 1..=t.last_line {
            parts.push(self.body(i).to_owned());
        }
        while parts.last().is_some_and(|p| p.trim().is_empty()) {
            parts.pop();
        }
        parts.join("\n").trim().to_owned()
    }

    /// Line indexes of the block's description: the lines before the first tag.
    pub fn description_lines(&self) -> std::ops::Range<usize> {
        0..self.tags.first().map_or(self.lines.len(), |t| t.first_line)
    }

    /// The description text, lines joined by `\n`, the opening line's text included.
    pub fn description(&self) -> String {
        let r = self.description_lines();
        let mut parts: Vec<&str> = r.map(|i| self.body(i)).collect();
        while parts.last().is_some_and(|p| p.trim().is_empty()) {
            parts.pop();
        }
        while parts.first().is_some_and(|p| p.trim().is_empty()) {
            parts.remove(0);
        }
        parts.join("\n")
    }

    /// Whether line `i` has no content (a blank line, or the opening or closing line alone).
    pub fn is_blank(&self, i: usize) -> bool {
        self.body(i).trim().is_empty()
    }
}

fn split_lines(src: &str, start: usize, end: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut ls = start;
    let n_total = end;
    loop {
        let nl = src[ls..n_total].find('\n').map(|k| ls + k);
        let mut le = nl.unwrap_or(n_total);
        if le > ls && src.as_bytes()[le - 1] == b'\r' {
            le -= 1;
        }
        let first = ls == start;
        let last = nl.is_none();
        // The delimiter.
        let p = if first {
            start + 3
        } else {
            let t = src[ls..le].trim_start_matches([' ', '\t']);
            let ws = (le - ls) - t.len();
            if t.starts_with('*') && !t.starts_with("*/") { ls + ws + 1 } else { ls + ws }
        };
        // The content.
        let mut body_end = le;
        let mut closes = false;
        if last {
            // The closing `*/` is the end of the comment.
            body_end = le.saturating_sub(2).max(p);
            closes = true;
        }
        let bs = {
            let rest = &src[p..body_end.max(p)];
            p + (rest.len() - rest.trim_start_matches([' ', '\t']).len())
        };
        let be = {
            let seg = &src[bs.min(body_end)..body_end.max(bs.min(body_end))];
            bs.min(body_end) + seg.trim_end().len()
        };
        let body_start = bs.min(be);
        lines.push(Line { start: ls, end: le, body_start, body_end: be.max(body_start), closes, fenced: false });
        match nl {
            Some(k) => ls = k + 1,
            None => break,
        }
    }
    lines
}

/// An inline tag `{@name text}` found in `text`; offsets are `base` + position.
#[derive(Clone, Debug)]
pub struct Inline {
    pub tag: String,
    pub start: usize,
    pub end: usize,
    /// The text after the name, trimmed.
    pub text: String,
}

pub fn inline_tags(text: &str, base: usize) -> Vec<Inline> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut i = 0;
    while i + 2 < b.len() {
        if b[i] == b'{' && b[i + 1] == b'@' {
            let name_end = text[i + 2..].find(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_')).map_or(text.len(), |k| i + 2 + k);
            if name_end > i + 2 {
                // Balanced to the closing brace.
                let mut depth = 0i32;
                let mut close = None;
                for (k, c) in text[i..].char_indices() {
                    match c {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                close = Some(i + k);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(c) = close {
                    out.push(Inline { tag: text[i + 2..name_end].to_owned(), start: base + i, end: base + c + 1, text: text[name_end..c].trim().to_owned() });
                    i = c + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

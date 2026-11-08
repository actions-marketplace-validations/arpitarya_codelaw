//! Renames that reach other files (SR-MULTI-FILE-FIXES): a name of one
//! kind — a CSS class, an id, a custom property, keyframes, a layer, a
//! container, a custom media query, an SCSS variable, mixin or placeholder,
//! a CSS Module class — renamed where it is defined and in every file of the
//! project that uses it, as one fix.
//!
//! The uses are found per kind in the files that can hold one, read through
//! [`FileContext`](crate::FileContext) (so a cached result is reused only
//! while every file it read is unchanged, SR-CACHE): stylesheets by their
//! statements, markup by its attributes and its `<style>` and `<script>`
//! blocks, scripts by their string literals. A name the new one already
//! takes refuses the rename. The engine's safeguards then check the result
//! across files (every file parses, no reference left dangling, all or
//! nothing).
//!
//! **Certain** (`auto`, SR-MULTI-FILE-FIXES safeguard 4) only when every use
//! is resolved: an SCSS member whose every other user reaches the file
//! through `@use`, `@forward` or `@import`. A use in markup or a string
//! literal is a guess, so the rename is `suggest`.

use crate::NativeContext;
use crate::css_syntax::{Kind as CssKind, SKind, all_simples, text};
use codelaw_core::{FileEdit, Language, TextEdit};

/// The kinds of name a rename follows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NameKind {
    Class,
    Id,
    CustomProperty,
    Keyframes,
    Layer,
    Container,
    CustomMedia,
    ScssVariable,
    ScssMixin,
    ScssPlaceholder,
    /// A class of a CSS Module: its selectors, and `styles.name` in the
    /// scripts that import the module.
    ModuleClass,
}

/// A rename's edits: this file's (in the offsets of `ctx.source`), the
/// other files' (and this file's own, in the host's offsets, when the rule
/// runs in an embedded region), and whether every use is resolved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Plan {
    pub own: Vec<TextEdit>,
    pub files: Vec<FileEdit>,
    pub certain: bool,
}

impl Plan {
    /// The finding's edits, as [`crate::in_files`] writes them; marked a
    /// suggestion unless every use is resolved.
    pub fn edits(self) -> Vec<TextEdit> {
        let certain = self.certain;
        let e = crate::in_files(self.own, self.files);
        if certain { e } else { crate::as_suggestion(e) }
    }
}

/// The most project files a rename reads; past it the finding offers no
/// fix (L8: a rename reads every file that may use the name).
const MAX_FILES: usize = 4000;

/// A name in kebab-case: `navBar`, `nav_bar` and `NavBar` are `nav-bar`.
pub(crate) fn kebab(name: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == ' ' || c == '-' {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            continue;
        }
        if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && (chars[i - 1].is_ascii_lowercase() || chars[i - 1].is_ascii_digit());
            let next_lower = chars.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            let prev_upper = i > 0 && chars[i - 1].is_ascii_uppercase();
            if !out.is_empty() && !out.ends_with('-') && (prev_lower || (prev_upper && next_lower)) {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out.trim_end_matches('-').to_owned()
}

/// Whether a name is kebab-case: lowercase words of letters and digits.
pub(crate) fn is_kebab(name: &str) -> bool {
    let mut parts = name.split('-');
    parts.next().is_some_and(|p| p.chars().next().is_some_and(|c| c.is_ascii_lowercase()) && p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
        && parts.all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
}

/// A name in camelCase: `nav-bar` and `nav_bar` are `navBar`.
pub(crate) fn camel(name: &str) -> String {
    let mut out = String::new();
    let mut up = false;
    for c in name.chars() {
        if c == '-' || c == '_' {
            up = !out.is_empty();
            continue;
        }
        if up {
            out.extend(c.to_uppercase());
            up = false;
        } else if out.is_empty() {
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// A pattern rule's fix for one name: `old` renamed to its `convert`ed
/// spelling where it matches `pattern`, here and in every file that uses
/// it; no edits when there is no safe rename.
pub(crate) fn renamed(ctx: &NativeContext<'_>, kind: NameKind, old: &str, pattern: &regex::Regex, convert: fn(&str) -> String) -> Vec<TextEdit> {
    conventional(old, pattern, convert).and_then(|new| rename(ctx, kind, old, &new)).map(Plan::edits).unwrap_or_default()
}

/// [`kebab`], keeping a leading `--` (a dashed container name).
pub(crate) fn kebab_dashed(name: &str) -> String {
    match name.strip_prefix("--") {
        Some(rest) => format!("--{}", kebab(rest)),
        None => kebab(name),
    }
}

/// The name a pattern rule renames `old` to: the case convention of the
/// rule's `pattern`, when `convert` of `old` matches it and differs.
pub(crate) fn conventional(old: &str, pattern: &regex::Regex, convert: fn(&str) -> String) -> Option<String> {
    let new = convert(old);
    (!new.is_empty() && new != old && pattern.is_match(&new)).then_some(new)
}

fn is_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' || c == b'_' || c >= 0x80
}

/// Every whole-name occurrence of `needle` in `hay[from..to]`: not inside a
/// longer name.
fn occurrences(hay: &str, from: usize, to: usize, needle: &str) -> Vec<usize> {
    let b = hay.as_bytes();
    let mut out = Vec::new();
    if needle.is_empty() || to > hay.len() {
        return out;
    }
    let mut at = from;
    while let Some(i) = hay[at..to].find(needle) {
        let s = at + i;
        let e = s + needle.len();
        let before = s == 0 || !is_name_char(b[s - 1]) || needle.starts_with(['$', '%', '-', '.', '#']);
        // A name preceded by `-` or a name char is part of a longer name
        // unless the needle carries its own sigil.
        let before = before && !(needle.as_bytes()[0].is_ascii_alphanumeric() && s > 0 && is_name_char(b[s - 1]));
        let after = e >= b.len() || !is_name_char(b[e]);
        if before && after {
            out.push(s);
        }
        at = e;
    }
    out
}

/// The comments and strings of a stylesheet, which no use is in: byte
/// ranges. SCSS `//` comments are blanked by the caller.
fn css_skipped(src: &str, strings: bool) -> Vec<(usize, usize)> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            let e = src[i + 2..].find("*/").map_or(b.len(), |o| i + 2 + o + 2);
            out.push((i, e));
            i = e;
        } else if strings && (b[i] == b'"' || b[i] == b'\'') {
            let e = crate::css_syntax::skip_string(b, i);
            out.push((i, e));
            i = e.max(i + 1);
        } else {
            i += 1;
        }
    }
    out
}

fn outside(skipped: &[(usize, usize)], at: usize) -> bool {
    !skipped.iter().any(|&(s, e)| s <= at && at < e)
}

/// The uses of a name of `kind` in one stylesheet's text (`scss`: read as
/// SCSS), as byte ranges of the name itself, sigils left out.
fn css_uses(src: &str, scss: bool, module: bool, kind: NameKind, name: &str) -> Vec<(usize, usize)> {
    let view = if scss { crate::css_syntax::scss_view(src) } else { src.to_owned() };
    let sheet = crate::css_syntax::parse_with(&view, scss, module);
    let mut out: Vec<(usize, usize)> = Vec::new();
    let push_in = |out: &mut Vec<(usize, usize)>, from: usize, to: usize, needle: &str, skip: usize| {
        let skipped = css_skipped(&view, true);
        for s in occurrences(&view, from, to, needle) {
            if outside(&skipped, s) {
                out.push((s + skip, s + needle.len()));
            }
        }
    };
    match kind {
        NameKind::Class | NameKind::Id | NameKind::ModuleClass => {
            let want = if kind == NameKind::Id { SKind::Id } else { SKind::Class };
            for i in sheet.walk() {
                let n = &sheet.nodes[i];
                if n.kind == CssKind::Rule && !sheet.in_keyframes(i, &view) {
                    let mut simples = Vec::new();
                    all_simples(&view, &sheet.selectors(i, &view), &mut simples);
                    for x in simples.iter().filter(|x| x.kind == want) {
                        if text(&view, x.name) == name {
                            out.push(x.name);
                        }
                    }
                }
                // `@extend .name` and CSS Modules' `composes: name`.
                if want == SKind::Class && n.kind == CssKind::AtRule && sheet.at_name(i, &view) == "extend" {
                    push_in(&mut out, n.params.0, n.params.1, &format!(".{name}"), 1);
                }
                if kind == NameKind::ModuleClass && n.kind == CssKind::Decl && text(&view, n.name) == "composes" {
                    let to = view[n.params.0..n.params.1].find(" from ").map_or(n.params.1, |o| n.params.0 + o);
                    push_in(&mut out, n.params.0, to, name, 0);
                }
            }
        }
        NameKind::CustomProperty | NameKind::CustomMedia => {
            let needle = format!("--{name}");
            let queries: Vec<(usize, usize)> = sheet
                .walk()
                .into_iter()
                .filter(|&i| sheet.nodes[i].kind == CssKind::AtRule && matches!(sheet.at_name(i, &view).as_str(), "media" | "custom-media"))
                .map(|i| sheet.nodes[i].params)
                .collect();
            let skipped = css_skipped(&view, true);
            for s in occurrences(&view, 0, view.len(), &needle) {
                let in_query = queries.iter().any(|&(a, b)| a <= s && s < b);
                if outside(&skipped, s) && in_query == (kind == NameKind::CustomMedia) {
                    out.push((s + 2, s + needle.len()));
                }
            }
        }
        NameKind::Keyframes | NameKind::Layer | NameKind::Container => {
            for i in sheet.walk() {
                let n = &sheet.nodes[i];
                let (from, to) = match (kind, n.kind) {
                    (NameKind::Keyframes, CssKind::AtRule) if sheet.at_name(i, &view) == "keyframes" => n.params,
                    (NameKind::Keyframes, CssKind::Decl) if matches!(crate::css_syntax::unprefixed(&text(&view, n.name).to_ascii_lowercase()), "animation" | "animation-name") => n.params,
                    (NameKind::Layer, CssKind::AtRule) if sheet.at_name(i, &view) == "layer" => n.params,
                    (NameKind::Layer, CssKind::AtRule) if sheet.at_name(i, &view) == "import" => {
                        match view[n.params.0..n.params.1].find("layer(") {
                            Some(o) => (n.params.0 + o + 6, view[n.params.0 + o..n.params.1].find(')').map_or(n.params.1, |c| n.params.0 + o + c)),
                            None => continue,
                        }
                    }
                    (NameKind::Container, CssKind::AtRule) if sheet.at_name(i, &view) == "container" => n.params,
                    (NameKind::Container, CssKind::Decl) if matches!(text(&view, n.name).to_ascii_lowercase().as_str(), "container-name" | "container") => {
                        let to = view[n.params.0..n.params.1].find('/').map_or(n.params.1, |o| n.params.0 + o);
                        (n.params.0, to)
                    }
                    _ => continue,
                };
                push_in(&mut out, from, to, name, 0);
            }
        }
        NameKind::ScssVariable | NameKind::ScssPlaceholder => {
            let sigil = if kind == NameKind::ScssVariable { "$" } else { "%" };
            // A variable is read inside `#{…}` in strings too.
            let skipped = css_skipped(&view, kind == NameKind::ScssPlaceholder);
            for s in occurrences(&view, 0, view.len(), &format!("{sigil}{name}")) {
                if outside(&skipped, s) {
                    out.push((s + 1, s + 1 + name.len()));
                }
            }
        }
        NameKind::ScssMixin => {
            let skipped = css_skipped(&view, true);
            let b = view.as_bytes();
            for at in ["@mixin", "@include"] {
                let mut from = 0;
                while let Some(o) = view[from..].find(at) {
                    let s = from + o;
                    from = s + at.len();
                    if !outside(&skipped, s) {
                        continue;
                    }
                    let mut j = from;
                    while j < b.len() && b[j].is_ascii_whitespace() {
                        j += 1;
                    }
                    let e = crate::css_syntax::ident_end(&view, j);
                    let mut word = (j, e);
                    // `@include ns.name`.
                    if b.get(e) == Some(&b'.') {
                        word = (e + 1, crate::css_syntax::ident_end(&view, e + 1));
                    }
                    if &view[word.0..word.1] == name {
                        out.push(word);
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The uses in a script's string literals (and, for a CSS Module class,
/// its `styles.name` reads).
fn script_uses(src: &str, language: Language, kind: NameKind, name: &str, module_locals: &[String]) -> Vec<(usize, usize)> {
    let root = codelaw_lang::parse(language, src);
    let mut out = Vec::new();
    for n in root.root().dfs() {
        let k = n.kind();
        let r = n.range();
        match kind {
            NameKind::ModuleClass => {
                if k == "member_expression"
                    && let (Some(obj), Some(prop)) = (n.field("object"), n.field("property"))
                    && module_locals.iter().any(|l| obj.text() == l.as_str())
                    && prop.text() == name
                {
                    let pr = prop.range();
                    out.push((pr.start, pr.end));
                }
                if k == "subscript_expression"
                    && let (Some(obj), Some(idx)) = (n.field("object"), n.field("index"))
                    && module_locals.iter().any(|l| obj.text() == l.as_str())
                    && idx.kind() == "string"
                    && idx.text().len() == name.len() + 2
                    && &idx.text()[1..idx.text().len() - 1] == name
                {
                    let ir = idx.range();
                    out.push((ir.start + 1, ir.end - 1));
                }
            }
            _ if k == "string_fragment" => {
                let frag = &src[r.clone()];
                match kind {
                    NameKind::Class => out.extend(occurrences(src, r.start, r.end, name).into_iter().map(|s| (s, s + name.len()))),
                    NameKind::Id => {
                        if frag == name {
                            out.push((r.start, r.end));
                        } else {
                            out.extend(occurrences(src, r.start, r.end, &format!("#{name}")).into_iter().map(|s| (s + 1, s + 1 + name.len())));
                        }
                    }
                    NameKind::CustomProperty => out.extend(occurrences(src, r.start, r.end, &format!("--{name}")).into_iter().map(|s| (s + 2, s + 2 + name.len()))),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The attributes of markup that name a class or an id.
const ID_REFS: &[&str] = &["for", "headers", "list", "form", "aria-labelledby", "aria-describedby", "aria-controls", "aria-owns", "aria-activedescendant", "aria-details", "aria-errormessage", "aria-flowto", "popovertarget", "commandfor", "anchor"];

/// The uses in markup: attribute values, and the `<style>` and `<script>`
/// blocks read as a stylesheet and a script.
fn html_uses(src: &str, kind: NameKind, name: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let lower = src.to_ascii_lowercase();
    for (open, close, css) in [("<style", "</style", true), ("<script", "</script", false)] {
        let mut from = 0;
        while let Some(o) = lower[from..].find(open) {
            let s = from + o;
            let Some(gt) = lower[s..].find('>').map(|g| s + g + 1) else { break };
            let end = lower[gt..].find(close).map_or(src.len(), |c| gt + c);
            let body = &src[gt..end];
            let found = if css {
                css_uses(body, false, false, kind, name)
            } else if matches!(kind, NameKind::Class | NameKind::Id | NameKind::CustomProperty) && !lower[s..gt].contains("type=\"text/template\"") {
                script_uses(body, Language::Javascript, kind, name, &[])
            } else {
                Vec::new()
            };
            out.extend(found.into_iter().map(|(a, b)| (a + gt, b + gt)));
            from = end.max(gt);
        }
    }
    if !matches!(kind, NameKind::Class | NameKind::Id | NameKind::CustomProperty) {
        return out;
    }
    let attrs = regex::Regex::new(r#"(?is)\s([a-z][a-z0-9:-]*)\s*=\s*("([^"]*)"|'([^']*)')"#).expect("a valid pattern");
    for c in attrs.captures_iter(src) {
        let attr = c.get(1).map(|m| m.as_str().to_ascii_lowercase()).unwrap_or_default();
        let Some(v) = c.get(3).or_else(|| c.get(4)) else { continue };
        let (vs, ve) = (v.start(), v.end());
        // Inside `<style>` or `<script>`: read above.
        match kind {
            NameKind::Class if attr == "class" || attr == "classname" => out.extend(occurrences(src, vs, ve, name).into_iter().map(|s| (s, s + name.len()))),
            NameKind::Id if attr == "id" || ID_REFS.contains(&attr.as_str()) => out.extend(occurrences(src, vs, ve, name).into_iter().map(|s| (s, s + name.len()))),
            NameKind::Id if attr == "href" || attr == "xlink:href" => {
                if src[vs..ve] == format!("#{name}") {
                    out.push((vs + 1, ve));
                }
            }
            NameKind::CustomProperty if attr == "style" => out.extend(occurrences(src, vs, ve, &format!("--{name}")).into_iter().map(|s| (s + 2, s + 2 + name.len()))),
            _ => {}
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Which files can hold a use of a name of `kind`.
fn reads(kind: NameKind, language: Language) -> bool {
    use Language::*;
    match kind {
        NameKind::Class | NameKind::Id | NameKind::CustomProperty => matches!(language, Css | Scss | Html | Javascript | Jsx | Typescript | Tsx),
        NameKind::Keyframes | NameKind::Layer | NameKind::Container | NameKind::CustomMedia => matches!(language, Css | Scss | Html),
        NameKind::ScssVariable | NameKind::ScssMixin | NameKind::ScssPlaceholder => matches!(language, Scss),
        NameKind::ModuleClass => matches!(language, Javascript | Jsx | Typescript | Tsx),
    }
}

/// The uses of a name of `kind` in one file.
fn uses_in(path: &str, src: &str, language: Language, kind: NameKind, name: &str, ctx: &NativeContext<'_>, module: Option<&str>) -> Vec<(usize, usize)> {
    match language {
        Language::Css | Language::Scss => {
            let is_module = crate::css_syntax::module_path(path, ctx.options.get("modules"));
            // A module's classes are its own; a global class is not a module's.
            match kind {
                NameKind::ModuleClass if Some(path) != module => Vec::new(),
                NameKind::Class if is_module => Vec::new(),
                _ => css_uses(src, language == Language::Scss, is_module, kind, name),
            }
        }
        Language::Html => html_uses(src, kind, name),
        l if l.is_js_family() => {
            let locals = match (kind, module) {
                (NameKind::ModuleClass, Some(m)) => module_imports(path, src, m),
                (NameKind::ModuleClass, None) => return Vec::new(),
                _ => Vec::new(),
            };
            if kind == NameKind::ModuleClass && locals.is_empty() {
                return Vec::new();
            }
            script_uses(src, l, kind, name, &locals)
        }
        _ => Vec::new(),
    }
}

/// The local names a script imports the CSS Module `module` as.
fn module_imports(path: &str, src: &str, module: &str) -> Vec<String> {
    let re = regex::Regex::new(r#"import\s+(?:\*\s+as\s+)?([A-Za-z_$][\w$]*)\s+from\s+['"]([^'"]+)['"]"#).expect("a valid pattern");
    re.captures_iter(src)
        .filter(|c| c[2].starts_with('.') && crate::resolve_path(path, &c[2]).as_deref() == Some(module))
        .map(|c| c[1].to_owned())
        .collect()
}

/// Whether an SCSS file reaches `target` through `@use`, `@forward` or
/// `@import`: the partial `_x.scss`, `x.scss` and `x/_index.scss` forms.
fn scss_reaches(path: &str, src: &str, target: &str) -> bool {
    let re = regex::Regex::new(r#"@(?:use|forward|import)\s+['"]([^'"]+)['"]"#).expect("a valid pattern");
    re.captures_iter(src).any(|c| {
        let spec = &c[1];
        let (dir, file) = match spec.rsplit_once('/') {
            Some((d, f)) => (format!("{d}/"), f.to_owned()),
            None => (String::new(), spec.to_owned()),
        };
        let stem = file.trim_end_matches(".scss");
        [format!("{dir}{stem}.scss"), format!("{dir}_{stem}.scss"), format!("{dir}{stem}/_index.scss"), format!("{dir}{stem}/index.scss")]
            .iter()
            .any(|cand| crate::resolve_path(path, cand).as_deref() == Some(target))
    })
}

/// The rename of `old` to `new`, a name of `kind`, in this file and every
/// file of the project that uses it; `None` when there is no safe one: the
/// project is not known, a file already uses `new`, or there are too many
/// files to read.
pub(crate) fn rename(ctx: &NativeContext<'_>, kind: NameKind, old: &str, new: &str) -> Option<Plan> {
    let own_path = ctx.file.path().to_owned();
    let host_text = String::from_utf8_lossy(ctx.file.bytes());
    let host = host_text.strip_prefix('\u{feff}').unwrap_or(&host_text).to_owned();
    plan(ctx, kind, old, new, &own_path, &host)
}

fn plan(ctx: &NativeContext<'_>, kind: NameKind, old: &str, new: &str, own_path: &str, host: &str) -> Option<Plan> {
    let own_language = Language::from_path(std::path::Path::new(own_path)).unwrap_or(ctx.language);
    let module = (kind == NameKind::ModuleClass).then_some(own_path);
    let mut files: Vec<String> = ctx
        .file
        .project_files()?
        .into_iter()
        .filter(|p| p != own_path && Language::from_path(std::path::Path::new(p)).is_some_and(|l| reads(kind, l)))
        .collect();
    files.sort();
    if files.len() > MAX_FILES {
        return None;
    }
    let mut certain = matches!(kind, NameKind::ScssVariable | NameKind::ScssMixin | NameKind::ScssPlaceholder);
    let own_uses = uses_in(own_path, host, own_language, kind, old, ctx, module);
    if own_uses.is_empty() || !uses_in(own_path, host, own_language, kind, new, ctx, module).is_empty() {
        return None;
    }
    let mut out = Plan::default();
    let edits = |uses: &[(usize, usize)]| -> Vec<TextEdit> { uses.iter().map(|&(s, e)| TextEdit::new(s, e, new)).collect() };
    if ctx.source == host {
        out.own = edits(&own_uses);
    } else {
        out.files.push(FileEdit::new(own_path, edits(&own_uses)));
    }
    for path in files {
        let language = Language::from_path(std::path::Path::new(&path)).expect("filtered");
        // A file that names neither spelling has no use of either.
        let Some(src) = ctx.file.read(&path) else { continue };
        if !src.contains(old) && !src.contains(new) {
            continue;
        }
        if !uses_in(&path, &src, language, kind, new, ctx, module).is_empty() {
            return None;
        }
        let uses = uses_in(&path, &src, language, kind, old, ctx, module);
        if uses.is_empty() {
            continue;
        }
        // An SCSS member is certain only where its user reaches this file.
        if certain && !scss_reaches(&path, &src, own_path) {
            certain = false;
        }
        if certain && matches!(kind, NameKind::ScssVariable | NameKind::ScssMixin) && defines(&src, kind, old) {
            // Another file defines the same name: not this one's use.
            return None;
        }
        out.files.push(FileEdit::new(path, edits(&uses)));
    }
    out.certain = certain && matches!(kind, NameKind::ScssVariable | NameKind::ScssMixin);
    Some(out)
}

/// Whether an SCSS file defines the variable or mixin itself.
fn defines(src: &str, kind: NameKind, name: &str) -> bool {
    match kind {
        NameKind::ScssVariable => occurrences(src, 0, src.len(), &format!("${name}")).iter().any(|&s| src[s + name.len() + 1..].trim_start().starts_with(':')),
        NameKind::ScssMixin => src.contains(&format!("@mixin {name}")),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_and_camel_convert_case() {
        assert_eq!(kebab("navBar"), "nav-bar");
        assert_eq!(kebab("NavBar"), "nav-bar");
        assert_eq!(kebab("nav_bar"), "nav-bar");
        assert_eq!(kebab("HTMLParser"), "html-parser");
        assert_eq!(kebab("item2Wide"), "item2-wide");
        assert_eq!(camel("nav-bar"), "navBar");
        assert_eq!(camel("nav_bar_item"), "navBarItem");
    }

    #[test]
    fn whole_names_only() {
        assert_eq!(occurrences("a navBar navBar-x xnavBar navBar", 0, 32, "navBar"), vec![2, 26]);
        assert_eq!(occurrences("$a $ab $a-1 #{$a}", 0, 17, "$a"), vec![0, 14]);
    }

    #[test]
    fn css_uses_skip_colors_comments_and_strings() {
        let src = ".navBar, #navBar { color: #navBar; content: \".navBar\"; } /* .navBar */ .a .navBar {}";
        assert_eq!(css_uses(src, false, false, NameKind::Class, "navBar").len(), 2);
        assert_eq!(css_uses(src, false, false, NameKind::Id, "navBar").len(), 1);
    }

    #[test]
    fn markup_uses_are_attribute_tokens_and_blocks() {
        let html = "<p class=\"a navBar\" id=\"navBar\">navBar</p><label for=\"navBar\"></label><a href=\"#navBar\"></a><style>.navBar{}</style>";
        assert_eq!(html_uses(html, NameKind::Class, "navBar").len(), 2);
        assert_eq!(html_uses(html, NameKind::Id, "navBar").len(), 3);
    }
}

#[cfg(test)]
mod module_tests {
    use super::*;

    #[test]
    fn module_reads_are_found() {
        let src = "import styles from \"./button.module.css\";\nexport const B = () => <b className={styles[\"btn-primary\"]} a={styles.btn} />;\n";
        assert_eq!(module_imports("src/button.jsx", src, "src/button.module.css"), vec!["styles".to_owned()]);
        assert_eq!(script_uses(src, Language::Jsx, NameKind::ModuleClass, "btn-primary", &["styles".to_owned()]).len(), 1);
        assert_eq!(script_uses(src, Language::Jsx, NameKind::ModuleClass, "btn", &["styles".to_owned()]).len(), 1);
    }
}

// ---------------------------------------------------------------------
// A JavaScript or TypeScript binding, renamed in its module and in every
// project module that imports it.

/// The project file a relative specifier of `from` names, among the files
/// that exist: `./a` is `a.js`, `a.ts`, … or `a/index.…`; a `.js` path may
/// name its `.ts` source.
pub(crate) fn resolve_js(ctx: &NativeContext<'_>, from: &str, spec: &str) -> Option<String> {
    if !spec.starts_with('.') {
        return None;
    }
    let base = crate::resolve_path(from, spec)?;
    const EXTS: &[&str] = &["js", "jsx", "ts", "tsx", "mjs", "cjs", "mts", "cts"];
    let mut candidates = vec![base.clone()];
    candidates.extend(EXTS.iter().map(|e| format!("{base}.{e}")));
    if let Some(stem) = base.strip_suffix(".js") {
        candidates.extend(["ts", "tsx"].iter().map(|e| format!("{stem}.{e}")));
    }
    candidates.extend(EXTS.iter().map(|e| format!("{base}/index.{e}")));
    candidates.into_iter().find(|c| ctx.file.exists(c) && !ctx.file.is_dir(c))
}

/// `Name`: the first letter upper-cased.
pub(crate) fn upper_first(name: &str) -> String {
    let mut c = name.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

/// `name`: the first letter lower-cased.
pub(crate) fn lower_first(name: &str) -> String {
    let mut c = name.chars();
    c.next().map(|f| f.to_lowercase().chain(c).collect()).unwrap_or_default()
}

/// PascalCase: `My_component` is `MyComponent`, `NAVBAR` is `Navbar`.
pub(crate) fn pascal(name: &str) -> String {
    name.split('_')
        .filter(|p| !p.is_empty())
        .map(|p| if p.chars().all(|c| !c.is_ascii_lowercase()) { upper_first(&p.to_ascii_lowercase()) } else { upper_first(p) })
        .collect()
}

/// The edits renaming symbol `sym` of a module's `model` to `new`: its
/// declarations and references, a shorthand property keeping its key, and
/// the closing JSX tags that name it. `None` when `new` is taken in the file.
fn binding_edits(src: &str, language: Language, model: &codelaw_semantic::SemanticModel, sym: usize, new: &str) -> Option<Vec<TextEdit>> {
    let s = model.symbol(sym);
    let old = s.name.clone();
    if model.symbols.iter().any(|x| x.name == new) || model.references.iter().any(|r| r.name == new) {
        return None;
    }
    let root = codelaw_lang::parse(language, src);
    let mut kind_at: std::collections::HashMap<usize, String> = std::collections::HashMap::new();
    let mut closing: Vec<(usize, usize)> = Vec::new();
    for n in root.root().dfs() {
        let r = n.range();
        if n.kind().starts_with("shorthand_property_identifier") {
            kind_at.insert(r.start, n.kind().to_string());
        }
        if n.kind() == "jsx_closing_element"
            && let Some(name) = n.field("name")
            && name.text() == old
        {
            let nr = name.range();
            closing.push((nr.start, nr.end));
        }
    }
    let mut spans: Vec<(usize, usize)> = s.decls.clone();
    spans.extend(s.refs.iter().map(|&r| (model.references[r].start, model.references[r].end)));
    if !closing.is_empty() && s.refs.iter().any(|&r| src[..model.references[r].start].ends_with('<')) {
        spans.extend(closing);
    }
    spans.sort_unstable();
    spans.dedup();
    Some(
        spans
            .into_iter()
            .map(|(a, b)| match kind_at.get(&a) {
                Some(_) => TextEdit::new(a, b, format!("{old}: {new}")),
                None => TextEdit::new(a, b, new),
            })
            .collect(),
    )
}

/// The first whole-name occurrence of `name` in `src[from..to]`.
pub(crate) fn first_name(src: &str, from: usize, to: usize, name: &str) -> Option<(usize, usize)> {
    occurrences(src, from, to.min(src.len()), name).first().map(|&s| (s, s + name.len()))
}

/// Whether a script names `target` in a `require(…)` or an `import(…)`,
/// which a rename can't follow.
pub(crate) fn loads_dynamically(ctx: &NativeContext<'_>, path: &str, src: &str, target: &str) -> bool {
    let re = regex::Regex::new(r#"(?:require|import)\s*\(\s*['"]([^'"]+)['"]"#).expect("a valid pattern");
    re.captures_iter(src).any(|c| resolve_js(ctx, path, &c[1]).as_deref() == Some(target))
}

/// Rename the top-level binding `old` of the module at `decl_path` (whose
/// text is `decl_src`) to `new`, there and in every project module that
/// imports it: the edits by file. `None` when there is no safe rename: `new`
/// is taken in a file, or a module reaches the binding through a namespace,
/// a re-export, `require` or `import()`. Certain when nothing imports it.
pub(crate) fn rename_export(ctx: &NativeContext<'_>, decl_path: &str, decl_src: &str, old: &str, new: &str) -> Option<(BTreeMapEdits, bool)> {
    let language = Language::from_path(std::path::Path::new(decl_path)).filter(|l| l.is_js_family())?;
    let root = codelaw_lang::parse(language, decl_src);
    let model = codelaw_semantic::build(language, &root, decl_src)?;
    let sym = model.symbols.iter().position(|s| s.name == old && s.scope == 0 && s.kind != codelaw_semantic::SymbolKind::Import)?;
    let mut out: BTreeMapEdits = std::collections::BTreeMap::new();
    out.insert(decl_path.to_owned(), binding_edits(decl_src, language, &model, sym, new)?);
    // The names importers see change only where the binding is exported as itself.
    let exported = model.exports.iter().any(|e| e.local == Some(sym) && e.name == old && e.source.is_none());
    if !exported {
        return Some((out, true));
    }
    let files: Vec<String> = ctx
        .file
        .project_files()?
        .into_iter()
        .filter(|p| p != decl_path && Language::from_path(std::path::Path::new(p)).is_some_and(|l| l.is_js_family()))
        .collect();
    if files.len() > MAX_FILES {
        return None;
    }
    for path in files {
        let Some(src) = (if path == ctx.file.path() { Some(String::from_utf8_lossy(ctx.file.bytes()).trim_start_matches('\u{feff}').to_owned()) } else { ctx.file.read(&path) }) else { continue };
        if !src.contains(old) {
            continue;
        }
        let l = Language::from_path(std::path::Path::new(&path)).expect("filtered");
        if loads_dynamically(ctx, &path, &src, decl_path) {
            return None;
        }
        let r = codelaw_lang::parse(l, &src);
        let Some(m) = codelaw_semantic::build(l, &r, &src) else { continue };
        if m.exports.iter().any(|e| e.source.as_deref().is_some_and(|s| resolve_js(ctx, &path, s).as_deref() == Some(decl_path))) {
            return None;
        }
        let mut edits = Vec::new();
        for imp in m.imports.iter().filter(|i| resolve_js(ctx, &path, &i.source).as_deref() == Some(decl_path)) {
            match &imp.imported {
                codelaw_semantic::Imported::Named(n) if n == old => {
                    let local = imp.local.filter(|&id| m.symbol(id).name == old);
                    match local {
                        Some(id) => edits.extend(binding_edits(&src, l, &m, id, new)?),
                        None => edits.push(first_name(&src, imp.start, imp.end, old).map(|(a, b)| TextEdit::new(a, b, new))?),
                    }
                }
                codelaw_semantic::Imported::Namespace => return None,
                _ => {}
            }
        }
        if !edits.is_empty() {
            edits.sort_by_key(|e| (e.start, e.end));
            edits.dedup();
            out.insert(path, edits);
        }
    }
    Some((out, false))
}

/// Edits by project-relative path.
pub(crate) type BTreeMapEdits = std::collections::BTreeMap<String, Vec<TextEdit>>;

/// The finding's edits for a rename by file: this file's own, the others in
/// [`crate::in_files`]; a suggestion unless `certain`.
pub(crate) fn plan_edits(ctx: &NativeContext<'_>, mut by_file: BTreeMapEdits, certain: bool) -> Vec<TextEdit> {
    let own_path = ctx.file.path().to_owned();
    let host = String::from_utf8_lossy(ctx.file.bytes());
    let in_host = host.trim_start_matches('\u{feff}') == ctx.source;
    let own = if in_host { by_file.remove(&own_path).unwrap_or_default() } else { Vec::new() };
    let files = by_file.into_iter().map(|(p, e)| FileEdit::new(p, e)).collect();
    Plan { own, files, certain }.edits()
}

/// The rename of a binding a rule found at `at` (a declaration or a
/// reference in this file) to `new`: where it is declared here, or the
/// project module it is imported from by name.
pub(crate) fn rename_binding_at(ctx: &NativeContext<'_>, at: usize, new: &str) -> Vec<TextEdit> {
    let Some(model) = ctx.semantic else { return Vec::new() };
    let Some(sym) = model.symbol_at(at) else { return Vec::new() };
    let s = model.symbol(sym);
    let own_path = ctx.file.path().to_owned();
    let host = String::from_utf8_lossy(ctx.file.bytes()).trim_start_matches('\u{feff}').to_owned();
    if host != ctx.source {
        return Vec::new();
    }
    if s.kind != codelaw_semantic::SymbolKind::Import {
        if s.scope != 0 {
            // A local binding: this file only.
            let Some(e) = binding_edits(ctx.source, ctx.language, model, sym, new) else { return Vec::new() };
            return e;
        }
        return rename_export(ctx, &own_path, &host, &s.name, new).map(|(by, certain)| plan_edits(ctx, by, certain)).unwrap_or_default();
    }
    let Some(imp) = model.import_of(sym) else { return Vec::new() };
    let codelaw_semantic::Imported::Named(name) = &imp.imported else { return Vec::new() };
    if *name != s.name {
        return Vec::new();
    }
    let Some(target) = resolve_js(ctx, &own_path, &imp.source) else { return Vec::new() };
    let Some(text) = ctx.file.read(&target) else { return Vec::new() };
    rename_export(ctx, &target, &text, name, new).map(|(by, _)| plan_edits(ctx, by, false)).unwrap_or_default()
}

// ---------------------------------------------------------------------
// An undefined name imported from the project module that exports it
// (SR-MULTI-FILE-FIXES: the fix reads other files, and edits this one).

/// The specifier from `from` to the project module `to`: relative, without
/// its extension or a trailing `/index`.
pub(crate) fn module_spec(from: &str, to: &str) -> String {
    let from_dir: Vec<&str> = from.split('/').collect::<Vec<_>>().split_last().map(|(_, d)| d.to_vec()).unwrap_or_default();
    let to_parts: Vec<&str> = to.split('/').collect();
    let common = from_dir.iter().zip(&to_parts).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = (common..from_dir.len()).map(|_| "..".to_owned()).collect();
    parts.extend(to_parts[common..].iter().map(|s| (*s).to_owned()));
    let mut s = parts.join("/");
    for ext in [".tsx", ".ts", ".jsx", ".mjs", ".cjs", ".js"] {
        if let Some(stem) = s.strip_suffix(ext) {
            s = stem.to_owned();
            break;
        }
    }
    if let Some(dir) = s.strip_suffix("/index") {
        s = dir.to_owned();
    }
    if s == "index" {
        s = ".".to_owned();
    }
    if s.starts_with("..") || s == "." { s } else { format!("./{s}") }
}

/// Where a new import goes: after the last import, else after the leading
/// directives (`"use client"`) and a hashbang, else at the top.
fn import_point(src: &str, language: Language) -> (usize, bool) {
    let root = codelaw_lang::parse(language, src);
    let top: Vec<_> = root.root().children().collect();
    if let Some(last) = top.iter().rfind(|n| n.kind() == "import_statement") {
        return (last.range().end, true);
    }
    let mut at = 0;
    for n in &top {
        let k = n.kind();
        let directive = k == "expression_statement" && n.children().next().is_some_and(|c| c.kind() == "string");
        if k == "hash_bang_line" || directive {
            at = n.range().end;
        } else if k != "comment" {
            break;
        }
    }
    (at, at > 0)
}

/// The fix of an undefined `name`: import it by name from each project
/// module that exports it (closest path first, three at most), as choices
/// labelled by the path; one module is one edit. Nothing when none does.
pub(crate) fn import_choices(ctx: &NativeContext<'_>, name: &str) -> Vec<TextEdit> {
    let own = ctx.file.path().to_owned();
    let host = String::from_utf8_lossy(ctx.file.bytes()).trim_start_matches('\u{feff}').to_owned();
    if host != ctx.source || !name.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$') {
        return Vec::new();
    }
    // A global of a known environment (`describe`, `window`) is the rule's
    // `env` to set, not an import to find.
    let global = ctx.data.iter().filter(|(k, _)| k.starts_with("globals/")).any(|(_, v)| match v.as_ref() {
        serde_json::Value::Object(o) => o.contains_key(name),
        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(name)),
        _ => false,
    });
    if global {
        return Vec::new();
    }
    let Some(files) = ctx.file.project_files() else { return Vec::new() };
    let files: Vec<String> = files.into_iter().filter(|p| *p != own && Language::from_path(std::path::Path::new(p)).is_some_and(|l| l.is_js_family())).collect();
    if files.len() > MAX_FILES {
        return Vec::new();
    }
    let mut found: Vec<String> = Vec::new();
    for path in files {
        let Some(src) = ctx.file.read(&path) else { continue };
        if !src.contains(name) || !src.contains("export") {
            continue;
        }
        let l = Language::from_path(std::path::Path::new(&path)).expect("filtered");
        let root = codelaw_lang::parse(l, &src);
        let Some(m) = codelaw_semantic::build(l, &root, &src) else { continue };
        if m.exports.iter().any(|e| e.name == name && e.source.is_none() && !e.type_only) {
            found.push(module_spec(&own, &path));
        }
    }
    found.sort_by_key(|s| (s.matches("..").count(), s.len(), s.clone()));
    found.truncate(3);
    let (at, after) = import_point(ctx.source, ctx.language);
    let edit = |spec: &str| {
        let stmt = format!("import {{ {name} }} from \"{spec}\";");
        vec![if after { TextEdit::insert(at, format!("\n{stmt}")) } else { TextEdit::insert(at, format!("{stmt}\n")) }]
    };
    let choices: Vec<codelaw_core::FixChoice> = found.iter().enumerate().map(|(i, s)| crate::choice(s.as_str(), i == 0, edit(s))).collect();
    crate::choice_edits(choices)
}

// ---------------------------------------------------------------------
// A Python star import made explicit: the names the module exports that
// this file (or this function) uses, read from the module itself.

/// The project file a Python module names from `from` (`a.b`, `.sub`).
fn py_module_file(ctx: &NativeContext<'_>, from: &str, module: &str) -> Option<String> {
    let level = module.chars().take_while(|c| *c == '.').count();
    let rest = module[level..].replace('.', "/");
    let bases: Vec<String> = if level > 0 {
        let mut dir: Vec<&str> = from.split('/').collect();
        dir.pop();
        for _ in 1..level {
            dir.pop()?;
        }
        vec![dir.join("/")]
    } else {
        vec![String::new(), "src".to_owned()]
    };
    for base in bases {
        let join = |p: &str| if base.is_empty() { p.to_owned() } else if p.is_empty() { base.clone() } else { format!("{base}/{p}") };
        for cand in [join(&format!("{rest}.py")), join(&if rest.is_empty() { "__init__.py".to_owned() } else { format!("{rest}/__init__.py") })] {
            if ctx.file.exists(&cand) && !ctx.file.is_dir(&cand) {
                return Some(cand);
            }
        }
    }
    None
}

/// The names `from <module> import *` binds: `__all__` when the module
/// writes it as a literal list, else every top-level name without a
/// leading underscore.
fn py_star_exports(src: &str) -> Option<std::collections::BTreeSet<String>> {
    let re = regex::Regex::new(r#"(?m)^__all__\s*=\s*[\[(]([^\])]*)[\])]"#).expect("a valid pattern");
    if let Some(c) = re.captures(src) {
        let names = regex::Regex::new(r#"['"]([A-Za-z_][A-Za-z0-9_]*)['"]"#).expect("a valid pattern");
        return Some(names.captures_iter(&c[1]).map(|m| m[1].to_owned()).collect());
    }
    if src.contains("__all__") {
        // Built some other way: not known without running the module.
        return None;
    }
    let root = codelaw_lang::parse(Language::Python, src);
    let m = codelaw_semantic::build(Language::Python, &root, src)?;
    Some(m.symbols.iter().filter(|s| s.scope == 0 && !s.name.starts_with('_')).map(|s| s.name.clone()).collect())
}

/// The fix of a star import `stmt` (an `import_from_statement`): the star
/// replaced with the names it binds that the file uses — inside `within`
/// (a function's bytes) when given. `auto` when it is the file's one star
/// import, else a suggestion; nothing when the module isn't the project's.
pub(crate) fn py_star_fix(ctx: &NativeContext<'_>, stmt: tree_sitter::Node<'_>, within: Option<(usize, usize)>) -> Vec<TextEdit> {
    let src = ctx.source;
    let host = String::from_utf8_lossy(ctx.file.bytes()).trim_start_matches('\u{feff}').to_owned();
    if host != src {
        return Vec::new();
    }
    let (Some(module), Some(star)) = (stmt.child_by_field_name("module_name"), (0..stmt.child_count()).filter_map(|i| stmt.child(i)).find(|c| c.kind() == "wildcard_import")) else { return Vec::new() };
    let Some(path) = py_module_file(ctx, ctx.file.path(), &src[module.byte_range()]) else { return Vec::new() };
    let Some(exports) = ctx.file.read(&path).and_then(|t| py_star_exports(&t)) else { return Vec::new() };
    let root = codelaw_lang::parse(Language::Python, src);
    let Some(model) = codelaw_semantic::build(Language::Python, &root, src) else { return Vec::new() };
    let mut used: Vec<String> = model
        .references
        .iter()
        .filter(|r| r.symbol.is_none() && exports.contains(&r.name) && within.is_none_or(|(a, b)| a <= r.start && r.end <= b))
        .map(|r| r.name.clone())
        .collect();
    used.sort();
    used.dedup();
    if used.is_empty() {
        return Vec::new();
    }
    let line = src[..stmt.start_byte()].rfind('\n').map_or(0, |i| i + 1);
    let indent = &src[line..stmt.start_byte()];
    let one = used.join(", ");
    let names = if stmt.end_byte() - stmt.start_byte() + one.len() + indent.len() > 88 {
        format!("(\n{}\n{indent})", used.iter().map(|n| format!("{indent}    {n},")).collect::<Vec<_>>().join("\n"))
    } else {
        one
    };
    let edits = vec![TextEdit::new(star.start_byte(), star.end_byte(), names)];
    let stars = model.imports.iter().filter(|i| i.imported == codelaw_semantic::Imported::Star).count();
    if stars == 1 { edits } else { crate::as_suggestion(edits) }
}

/// The star import of this file that binds `name`, when exactly one does.
pub(crate) fn py_star_providing<'t>(ctx: &NativeContext<'_>, root: tree_sitter::Node<'t>, name: &str) -> Option<tree_sitter::Node<'t>> {
    let mut stack = vec![root];
    let mut found = Vec::new();
    while let Some(n) = stack.pop() {
        if n.kind() == "wildcard_import"
            && let Some(stmt) = n.parent()
            && let Some(module) = stmt.child_by_field_name("module_name")
            && let Some(path) = py_module_file(ctx, ctx.file.path(), &ctx.source[module.byte_range()])
            && ctx.file.read(&path).and_then(|t| py_star_exports(&t)).is_some_and(|e| e.contains(name))
        {
            found.push(stmt);
        }
        let mut c = n.walk();
        stack.extend(n.children(&mut c));
    }
    (found.len() == 1).then(|| found[0])
}

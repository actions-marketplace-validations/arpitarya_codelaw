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
use std::cell::RefCell;
use std::collections::BTreeMap;

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

/// One file's plans: its path, a hash of its text, and each rename asked.
type Plans = (String, u64, BTreeMap<(NameKind, String, String), Option<Plan>>);

thread_local! {
    /// The last file's plans, so each finding of one name reads the project once.
    static PLANS: RefCell<Option<Plans>> = const { RefCell::new(None) };
}

fn text_key(t: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

/// The rename of `old` to `new`, a name of `kind`, in this file and every
/// file of the project that uses it; `None` when there is no safe one: the
/// project is not known, a file already uses `new`, or there are too many
/// files to read.
pub(crate) fn rename(ctx: &NativeContext<'_>, kind: NameKind, old: &str, new: &str) -> Option<Plan> {
    let own_path = ctx.file.path().to_owned();
    let host_text = String::from_utf8_lossy(ctx.file.bytes());
    let host = host_text.strip_prefix('\u{feff}').unwrap_or(&host_text).to_owned();
    let key = (kind, old.to_owned(), new.to_owned());
    let stamp = text_key(&host) ^ text_key(ctx.source);
    if let Some(p) = PLANS.with(|c| c.borrow().as_ref().filter(|(p, h, _)| *p == own_path && *h == stamp).and_then(|(_, _, m)| m.get(&key).cloned())) {
        return p;
    }
    let plan = plan(ctx, kind, old, new, &own_path, &host);
    PLANS.with(|c| {
        let mut c = c.borrow_mut();
        match c.as_mut() {
            Some((p, h, m)) if *p == own_path && *h == stamp => {
                m.insert(key, plan.clone());
            }
            _ => *c = Some((own_path, stamp, BTreeMap::from([(key, plan.clone())]))),
        }
    });
    plan
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

//! The `js/regexp-*` rules: eslint-plugin-regexp 3.3.2's documented
//! behaviour, reimplemented on codelaw's own regular-expression parser
//! ([`crate::regex_ast`]). A rule sees every regular expression literal and
//! every `RegExp("…")` / `new RegExp("…")` whose arguments are plain string
//! literals. Edits are offered on literals only (a string's escapes make
//! the offsets of its pattern differ from the source), except edits to a
//! plain flags string.
//!
//! Every pattern edit is checked before it is offered: the edited pattern
//! still parses with the same flags, keeps its capturing groups (unless the
//! rule's purpose is to remove them) and, for an edit inside a class,
//! matches the same characters.

use crate::regex_ast::{self as ra, CItem, CKind, CharSet, Class, ClassOp, EscKind, Item, Kind, Legacy, Ref, Regex};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const JS_ONLY: &[Language] = &[Language::Javascript, Language::Jsx];
const FLAG_ORDER: &str = "dgimsuvy";

/// One regular expression in the source.
pub struct Site<'t> {
    _t: std::marker::PhantomData<&'t ()>,
    pub pattern: String,
    pub re: Regex,
    pub literal: bool,
    /// Source offset of the pattern's first byte (a literal's).
    pub base: usize,
    /// Source span of the flags text, when it can be edited.
    pub flags_span: Option<(usize, usize)>,
    /// What a finding covers when the pattern isn't a literal.
    pub whole: (usize, usize),
}

fn unparen(mut n: Node<'_>) -> Node<'_> {
    while n.kind() == "parenthesized_expression" {
        match n.named_child(0) {
            Some(c) => n = c,
            None => break,
        }
    }
    n
}

/// A JavaScript string literal's value, escapes decoded; `None` for a
/// malformed one.
pub fn js_string(n: Node<'_>, src: &str) -> Option<String> {
    if n.kind() != "string" {
        return None;
    }
    let raw = text(n, src);
    if raw.len() < 2 {
        return None;
    }
    let body = &raw[1..raw.len() - 1];
    let mut out = String::new();
    let mut it = body.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let e = it.next()?;
        match e {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            'v' => out.push('\u{b}'),
            'f' => out.push('\u{c}'),
            'b' => out.push('\u{8}'),
            '0' if !it.peek().is_some_and(|d| d.is_ascii_digit()) => out.push('\0'),
            'x' => {
                let h: String = [it.next()?, it.next()?].iter().collect();
                out.push(char::from_u32(u32::from_str_radix(&h, 16).ok()?)?);
            }
            'u' => {
                let v = if it.peek() == Some(&'{') {
                    it.next();
                    let mut h = String::new();
                    loop {
                        let d = it.next()?;
                        if d == '}' {
                            break;
                        }
                        h.push(d);
                    }
                    u32::from_str_radix(&h, 16).ok()?
                } else {
                    let h: String = [it.next()?, it.next()?, it.next()?, it.next()?].iter().collect();
                    u32::from_str_radix(&h, 16).ok()?
                };
                out.push(char::from_u32(v)?);
            }
            '\n' => {}
            '\r' => {
                if it.peek() == Some(&'\n') {
                    it.next();
                }
            }
            c => out.push(c),
        }
    }
    Some(out)
}

impl<'t> Site<'t> {
    pub fn of(n: Node<'t>, src: &'t str) -> Option<Site<'t>> {
        match n.kind() {
            "regex" => {
                let p = n.child_by_field_name("pattern")?;
                let f = n.child_by_field_name("flags");
                let flags = f.map(|f| text(f, src)).unwrap_or("");
                let re = ra::parse(text(p, src), flags).ok()?;
                let flags_span = Some(f.map(|f| (f.start_byte(), f.end_byte())).unwrap_or((n.end_byte(), n.end_byte())));
                Some(Site { _t: std::marker::PhantomData, pattern: text(p, src).to_owned(), re, literal: true, base: p.start_byte(), flags_span, whole: (n.start_byte(), n.end_byte()) })
            }
            "call_expression" | "new_expression" => {
                let callee = n.child_by_field_name("function").or_else(|| n.child_by_field_name("constructor"))?;
                if text(callee, src) != "RegExp" {
                    return None;
                }
                let args = named_children(n.child_by_field_name("arguments")?);
                let p = args.first()?;
                let pattern = js_string(*p, src)?;
                let (flags, flags_span) = match args.get(1) {
                    Some(f) => {
                        let v = js_string(*f, src)?;
                        let raw = text(*f, src);
                        let span = (raw.len() == v.len() + 2).then(|| (f.start_byte() + 1, f.end_byte() - 1));
                        (v, span)
                    }
                    None => (String::new(), None),
                };
                if args.len() > 2 {
                    return None;
                }
                let re = ra::parse(&pattern, &flags).ok()?;
                Some(Site { _t: std::marker::PhantomData, pattern, re, literal: false, base: 0, flags_span, whole: (p.start_byte(), p.end_byte()) })
            }
            _ => None,
        }
    }

    pub fn t(&self, s: usize, e: usize) -> &str {
        &self.pattern[s..e]
    }

    pub fn it(&self, i: &Item) -> &str {
        &self.pattern[i.start..i.end]
    }
}

/// What a rule writes into.
pub struct Out<'a, 't> {
    pub site: &'a Site<'t>,
    out: &'a mut Vec<NativeFinding>,
    /// Whether an edit may change the number of capturing groups.
    pub drops_groups: bool,
}

/// Whether text ending in `before` followed by `after` could read as one
/// token that neither was (`\1` + `0` → `\10`, `\x4` + `1`).
fn joins(before: &str, after: &str) -> bool {
    let Some(first) = after.chars().next() else { return false };
    let b = before.as_bytes();
    let digits = b.iter().rev().take_while(|c| c.is_ascii_digit()).count();
    if digits > 0 && b.len() > digits && b[b.len() - digits - 1] == b'\\' && first.is_ascii_digit() {
        return true;
    }
    if b.iter().rev().take_while(|c| **c == b'\\').count() % 2 == 1 {
        return true;
    }
    let tail: String = before.chars().rev().take(6).collect::<Vec<_>>().into_iter().rev().collect();
    if let Some(p) = tail.rfind('\\') {
        let esc = &tail[p + 1..];
        let want = if esc.starts_with('x') { 3 } else { 5 };
        if (esc.starts_with('x') || esc.starts_with('u')) && esc.len() < want && esc[1..].chars().all(|c| c.is_ascii_hexdigit()) && first.is_ascii_hexdigit() {
            return true;
        }
        if esc == "c" || esc == "k" || esc == "p" || esc == "P" {
            return true;
        }
    }
    false
}

impl<'a, 't> Out<'a, 't> {
    fn span(&self, s: usize, e: usize) -> (usize, usize) {
        if self.site.literal { (self.site.base + s, self.site.base + e) } else { self.site.whole }
    }

    /// The pattern with these edits, when they pass the checks.
    fn apply(&self, edits: &[(usize, usize, String)]) -> Option<String> {
        let p = &self.site.pattern;
        let mut sorted: Vec<_> = edits.to_vec();
        sorted.sort_by_key(|e| e.0);
        let mut out = String::new();
        let mut at = 0;
        for (s, e, t) in &sorted {
            if *s < at || *e > p.len() {
                return None;
            }
            out.push_str(&p[at..*s]);
            let next = if t.is_empty() { &p[*e..] } else { t.as_str() };
            if joins(&out, next) || (!t.is_empty() && joins(&format!("{out}{t}"), &p[*e..])) {
                return None;
            }
            out.push_str(t);
            at = *e;
        }
        out.push_str(&p[at..]);
        if out.is_empty() {
            out = "(?:)".into();
        }
        let re = ra::parse(&out, &self.site.re.flags.raw).ok()?;
        if !self.drops_groups && re.groups != self.site.re.groups {
            return None;
        }
        Some(out)
    }

    pub fn report(&mut self, s: usize, e: usize, msg: impl Into<String>) {
        self.edits(s, e, msg, Vec::new());
    }

    pub fn fix(&mut self, s: usize, e: usize, msg: impl Into<String>, rs: usize, re_: usize, with: impl Into<String>) {
        self.edits(s, e, msg, vec![(rs, re_, with.into())]);
    }

    /// A finding over pattern bytes `s..e` with pattern edits.
    pub fn edits(&mut self, s: usize, e: usize, msg: impl Into<String>, edits: Vec<(usize, usize, String)>) {
        let (start, end) = self.span(s, e);
        let mut out_edits = Vec::new();
        if self.site.literal
            && !edits.is_empty()
            && let Some(new) = self.apply(&edits)
        {
            out_edits.push(TextEdit::new(self.site.base, self.site.base + self.site.pattern.len(), new));
        }
        self.out.push(NativeFinding { start, end, message: msg.into(), edits: out_edits });
    }

    /// A finding on the flags, with the flags rewritten.
    pub fn flags(&mut self, msg: impl Into<String>, new: Option<String>) {
        let (start, end) = match self.site.flags_span {
            Some((s, e)) if s < e => (s, e),
            _ => self.site.whole,
        };
        let mut edits = Vec::new();
        if let (Some(new), Some((s, e))) = (new, self.site.flags_span)
            && ra::parse(&self.site.pattern, &new).is_ok()
        {
            edits.push(TextEdit::new(s, e, new));
        }
        self.out.push(NativeFinding { start, end, message: msg.into(), edits });
    }

    /// Whether `new_class`, the source of a class, matches what `c` does.
    pub fn same_class(&self, c: &Class, new_class: &str) -> bool {
        let f = &self.site.re.flags;
        let Ok(r) = ra::parse(new_class, &f.raw) else { return false };
        let [a] = r.alts.as_slice() else { return false };
        let [it] = a.items.as_slice() else { return false };
        let before = ra::class_set(c, f);
        let after = ra::item_set(it, f);
        let before = before.map(|s| if f.i { s.fold() } else { s });
        before.is_some() && before == after
    }
}

pub fn sorted_flags(f: &str) -> String {
    FLAG_ORDER.chars().filter(|c| f.contains(*c)).chain(f.chars().filter(|c| !FLAG_ORDER.contains(*c))).collect()
}

fn with_flag(f: &str, add: char) -> String {
    if f.contains(add) {
        return f.to_owned();
    }
    let mut s = f.to_owned();
    s.push(add);
    if f == sorted_flags(f) { sorted_flags(&s) } else { s }
}

/// A rule over every regular expression.
pub struct Rx {
    pub id: &'static str,
    pub f: fn(&mut Out<'_, '_>),
}

impl NativeRule for Rx {
    fn id(&self) -> &'static str {
        self.id
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["regex", "call_expression", "new_expression"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let Some(site) = Site::of(n, ctx.source) else { return };
        let mut o = Out { site: &site, out, drops_groups: false };
        (self.f)(&mut o);
    }
}

/// A rule over method calls whose arguments or receiver are regexes.
pub struct Call {
    pub id: &'static str,
    pub f: fn(Node<'_>, &str, &mut Vec<NativeFinding>),
    pub langs: &'static [Language],
    pub kinds: &'static [&'static str],
}

impl NativeRule for Call {
    fn id(&self) -> &'static str {
        self.id
    }
    fn languages(&self) -> &'static [Language] {
        self.langs
    }
    fn kinds(&self) -> &'static [&'static str] {
        self.kinds
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        (self.f)(node.get_inner_node(), ctx.source, out);
    }
}

const fn call(id: &'static str, f: fn(Node<'_>, &str, &mut Vec<NativeFinding>)) -> Call {
    Call { id, f, langs: JS, kinds: &["call_expression"] }
}

// ---------------------------------------------------------------------------
// Helpers over the tree

fn items_in(re: &Regex) -> Vec<Item> {
    let mut v = Vec::new();
    ra::walk(&re.alts, &mut |i| v.push(i.clone()));
    v
}

fn classes_in(re: &Regex) -> Vec<Class> {
    let mut v = Vec::new();
    ra::each_class(&re.alts, &mut |c| v.push(c.clone()));
    v
}

fn disjunctions(re: &Regex) -> Vec<(Vec<ra::Alt>, Option<Item>)> {
    let mut v = Vec::new();
    ra::each_disjunction(&re.alts, None, &mut |d, h| v.push((d.to_vec(), h.cloned())));
    v
}

fn alts_in(re: &Regex) -> Vec<ra::Alt> {
    let mut v = Vec::new();
    ra::each_alt(&re.alts, &mut |a| v.push(a.clone()));
    v
}

/// The set of a single-character item, or of a quantifier's body, with the
/// counts.
fn atom_set(it: &Item, f: &ra::Flags) -> Option<(CharSet, u32, Option<u32>, bool)> {
    match &it.kind {
        Kind::Quant { min, max, greedy, body, .. } => Some((ra::item_set(body, f)?, *min, *max, *greedy)),
        _ => Some((ra::item_set(it, f)?, 1, Some(1), true)),
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Wordy {
    Word,
    NonWord,
    Mixed,
}

fn wordy(s: &CharSet, f: &ra::Flags) -> Wordy {
    let w = if f.i { ra::word().fold() } else { ra::word() };
    if s.is_subset(&w) {
        Wordy::Word
    } else if s.disjoint(&w) {
        Wordy::NonWord
    } else {
        Wordy::Mixed
    }
}

fn cap_free(it: &Item) -> bool {
    !ra::has_capture(it)
}

/// `body{n}` in its shortest form ("" for 0).
fn exact(body: &str, n: u32) -> String {
    match n {
        0 => String::new(),
        1 => body.to_owned(),
        n => format!("{body}{{{n}}}"),
    }
}

fn char_spots(re: &Regex) -> Vec<(u32, usize, usize)> {
    let mut spots = Vec::new();
    for it in items_in(re) {
        if let Kind::Char(c) = it.kind {
            spots.push((c, it.start, it.end));
        }
    }
    for c in classes_in(re) {
        for i in &c.items {
            match &i.kind {
                CKind::Char(v) => spots.push((*v, i.start, i.end)),
                CKind::Range { lo, hi, lo_span, hi_span } => {
                    spots.push((*lo, lo_span.0, lo_span.1));
                    spots.push((*hi, hi_span.0, hi_span.1));
                }
                _ => {}
            }
        }
    }
    spots
}

// ---------------------------------------------------------------------------
// The rules

fn confusing_quantifier(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        if let Kind::Quant { min, max, greedy, body, op } = &it.kind
            && *min > 0
            && *max != Some(0)
            && ra::can_be_empty(body)
            && !ra::zero_width(body)
        {
            let q = o.site.t(op.0, op.1).to_owned();
            o.fix(op.0, op.1, format!("`{q}` reads as at least {min}, but what it repeats can match the empty string, so it can match nothing at all"), op.0, op.1, ra::quant_text(0, *max, *greedy));
        }
    }
}

fn control_escape_for(c: u32) -> Option<&'static str> {
    Some(match c {
        0 => "\\0",
        9 => "\\t",
        10 => "\\n",
        11 => "\\v",
        12 => "\\f",
        13 => "\\r",
        _ => return None,
    })
}

fn control_character_escape(o: &mut Out<'_, '_>) {
    for (c, s, e) in char_spots(&o.site.re) {
        let Some(want) = control_escape_for(c) else { continue };
        let raw = o.site.t(s, e).to_owned();
        if raw == want || (!o.site.literal && !raw.starts_with('\\')) {
            continue;
        }
        if c == 0 && o.site.pattern[e..].starts_with(|d: char| d.is_ascii_digit()) {
            continue;
        }
        o.fix(s, e, format!("Write this control character as `{want}`, not `{}`", raw.escape_debug()), s, e, want);
    }
}

fn grapheme_count(s: &str) -> usize {
    let mut n = 0;
    let mut join = false;
    for c in s.chars() {
        let u = c as u32;
        if u == 0x200D {
            join = true;
            continue;
        }
        if !joined_mark(u) && !join {
            n += 1;
        }
        join = false;
    }
    n
}

fn grapheme_string_literal(o: &mut Out<'_, '_>) {
    for c in classes_in(&o.site.re) {
        for i in &c.items {
            if let CKind::Strings(v) = &i.kind
                && let Some(s) = v.iter().find(|s| grapheme_count(s) > 1)
            {
                o.report(i.start, i.end, format!("`{s}` in this string literal is more than one character; match it with an alternation outside the class"));
            }
        }
    }
}

fn hexadecimal_escape(o: &mut Out<'_, '_>) {
    for (c, s, e) in char_spots(&o.site.re) {
        let raw = o.site.t(s, e).to_owned();
        if c <= 0xFF && raw.starts_with("\\u") {
            o.fix(s, e, format!("Write `{raw}` as the hexadecimal escape `\\x{c:02X}`"), s, e, format!("\\x{c:02X}"));
        }
    }
}

fn match_any(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    let all = CharSet::of(&[(0, f.max())]);
    for it in items_in(&o.site.re) {
        if !matches!(it.kind, Kind::Class(_)) {
            continue;
        }
        let raw = o.site.it(&it).to_owned();
        if raw == "[\\s\\S]" {
            continue;
        }
        let Some(set) = ra::item_set(&it, &f) else { continue };
        if set == all {
            o.fix(it.start, it.end, format!("Write `{raw}` as `[\\s\\S]`, the usual way to match any character"), it.start, it.end, "[\\s\\S]");
        }
    }
}

fn esc_text(e: &ra::SetEsc, negate: bool) -> String {
    let (lo, up) = match &e.kind {
        EscKind::Digit => ("\\d", "\\D"),
        EscKind::Word => ("\\w", "\\W"),
        EscKind::Space => ("\\s", "\\S"),
        EscKind::Prop(p) => return format!("\\{}{{{p}}}", if negate { 'P' } else { 'p' }),
    };
    (if negate { up } else { lo }).to_owned()
}

fn negation(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for c in classes_in(&o.site.re) {
        if !c.negate || c.op != ClassOp::Union {
            continue;
        }
        let [i] = c.items.as_slice() else { continue };
        let CKind::Set(e) = &i.kind else { continue };
        if matches!(e.kind, EscKind::Prop(_)) && f.v && f.i {
            continue;
        }
        let want = esc_text(e, !e.negate);
        let raw = o.site.t(c.start, c.end).to_owned();
        o.fix(c.start, c.end, format!("Write `{raw}` as `{want}`"), c.start, c.end, want);
    }
}

fn no_contradiction_with_assertion(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 1..v.len().saturating_sub(1) {
            let Kind::Boundary { negate } = v[k].kind else { continue };
            let Some(prev) = ra::item_set(&v[k - 1], &f) else { continue };
            let Kind::Quant { min: 0, max, body, op, .. } = &v[k + 1].kind else { continue };
            let Some(bs) = ra::item_set(body, &f) else { continue };
            let (pw, bw) = (wordy(&prev, &f), wordy(&bs, &f));
            if pw == Wordy::Mixed || bw == Wordy::Mixed {
                continue;
            }
            let body_fits = (pw != bw) != negate;
            let q = v[k + 1].clone();
            let qt = o.site.it(&q).to_owned();
            if !body_fits {
                let edits = if cap_free(&q) { vec![(q.start, q.end, String::new())] } else { vec![] };
                o.edits(q.start, q.end, format!("`{qt}` can never match here: the assertion before it rules out its characters"), edits);
            } else if let Some(next) = v.get(k + 2).and_then(|n| ra::item_set(n, &f)) {
                let nw = wordy(&next, &f);
                let skip_fits = nw != Wordy::Mixed && ((pw != nw) != negate);
                if nw != Wordy::Mixed && !skip_fits {
                    let new = if *max == Some(1) { String::new() } else { ra::quant_text(1, *max, true) };
                    o.fix(op.0, op.1, format!("`{qt}` is always entered: skipping it would contradict the assertion before it"), op.0, op.1, new);
                }
            }
        }
    }
}

fn citem_text<'s>(o: &'s Out<'_, '_>, i: &CItem) -> &'s str {
    o.site.t(i.start, i.end)
}

/// Edits removing class items (and putting `put_first`'s text at one), when
/// the class still matches the same characters.
fn remove_items(o: &Out<'_, '_>, c: &Class, drop: &[usize], put: Option<(usize, String)>) -> Option<Vec<(usize, usize, String)>> {
    let mut edits = Vec::new();
    for (k, i) in c.items.iter().enumerate() {
        if let Some((at, t)) = &put
            && *at == k
        {
            edits.push((i.start, i.end, t.clone()));
        } else if drop.contains(&k) {
            edits.push((i.start, i.end, String::new()));
        }
    }
    let mut new = String::new();
    let mut at = c.start;
    for (s, e, t) in &edits {
        new.push_str(o.site.t(at, *s));
        new.push_str(t);
        at = *e;
    }
    new.push_str(o.site.t(at, c.end));
    (o.same_class(c, &new) && new != "[]" && new != "[^]").then_some(edits)
}

fn no_dupe_characters_character_class(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for c in classes_in(&o.site.re) {
        if c.op != ClassOp::Union {
            continue;
        }
        let sets: Vec<Option<CharSet>> = c.items.iter().map(|i| ra::citem_set(i, &f)).collect();
        let mut dropped: Vec<usize> = Vec::new();
        for j in 0..c.items.len() {
            let Some(sj) = &sets[j] else { continue };
            let dup = (0..c.items.len()).find(|&i| i != j && !dropped.contains(&i) && sets[i].as_ref().is_some_and(|si| sj.is_subset(si) && (i < j || sj != si)));
            if let Some(i) = dup {
                dropped.push(j);
                let (a, b) = (citem_text(o, &c.items[j]).to_owned(), citem_text(o, &c.items[i]).to_owned());
                let msg = if a == b { format!("`{a}` is in this class twice") } else { format!("`{a}` is already matched by `{b}` in this class") };
                let edits = remove_items(o, &c, &[j], None).unwrap_or_default();
                let it = &c.items[j];
                o.edits(it.start, it.end, msg, edits);
            }
        }
    }
}

fn no_dupe_disjunctions(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for (alts, _) in disjunctions(&o.site.re) {
        for j in 1..alts.len() {
            let a = &alts[j];
            let ta = o.site.t(a.start, a.end).to_owned();
            if ta.is_empty() {
                continue;
            }
            let single = |x: &ra::Alt| match x.items.as_slice() {
                [i] => ra::item_set(i, &f),
                _ => None,
            };
            let found = (0..j).find_map(|i| {
                let b = &alts[i];
                let tb = o.site.t(b.start, b.end);
                if tb == ta {
                    return Some(format!("`{ta}` repeats an earlier alternative"));
                }
                match (single(a), single(b)) {
                    (Some(sa), Some(sb)) if sa.is_subset(&sb) => Some(format!("`{ta}` can never match: the earlier alternative `{tb}` matches everything it does")),
                    _ => None,
                }
            });
            if let Some(msg) = found {
                let edits = if a.items.iter().all(cap_free) { vec![(a.start - 1, a.end, String::new())] } else { vec![] };
                o.edits(a.start, a.end, msg, edits);
            }
        }
    }
}

fn quantified_after(o: &Out<'_, '_>, end: usize) -> bool {
    matches!(o.site.pattern[end..].chars().next(), Some('*' | '+' | '?' | '{'))
}

fn no_empty_alternative(o: &mut Out<'_, '_>) {
    for (alts, holder) in disjunctions(&o.site.re) {
        if alts.len() < 2 {
            continue;
        }
        for (k, a) in alts.iter().enumerate() {
            if !a.items.is_empty() {
                continue;
            }
            let p = a.start;
            let (s, e) = if p > 0 && o.site.pattern.as_bytes()[p - 1] == b'|' { (p - 1, p) } else { (p, p + 1) };
            let mut edits = Vec::new();
            if k == alts.len() - 1
                && let Some(h) = &holder
                && matches!(h.kind, Kind::Group { .. })
                && !quantified_after(o, h.end)
            {
                edits = vec![(s, e, String::new()), (h.end, h.end, "?".into())];
            }
            o.edits(s, e, "This alternative is empty: make the group optional instead", edits);
        }
    }
}

fn no_empty_capturing_group(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        if let Kind::Group { capture: Some(_), alts, .. } = &it.kind
            && alts.iter().all(|a| a.items.iter().all(ra::zero_width))
        {
            o.report(it.start, it.end, format!("`{}` captures nothing: it can only match the empty string", o.site.it(&it)));
        }
    }
}

fn no_empty_character_class(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for c in classes_in(&o.site.re) {
        if c.items.is_empty() || c.items.iter().any(|i| matches!(i.kind, CKind::Strings(_))) {
            continue; // `js/no-empty-character-class` reports `[]`.
        }
        if ra::class_set(&c, &f).is_some_and(|s| s.is_empty()) {
            o.report(c.start, c.end, format!("`{}` matches no character at all", o.site.t(c.start, c.end)));
        }
    }
}

/// `it`, or the quantifier around it when it has one.
fn with_quant(re: &Regex, it: &Item) -> (usize, usize) {
    let mut span = (it.start, it.end);
    ra::walk(&re.alts, &mut |x| {
        if let Kind::Quant { body, .. } = &x.kind
            && body.start == it.start
            && body.end == it.end
        {
            span = (x.start, x.end);
        }
    });
    span
}

fn no_empty_group(o: &mut Out<'_, '_>) {
    let re = o.site.re.clone();
    for it in items_in(&re) {
        if let Kind::Group { capture, alts, .. } = &it.kind
            && alts.iter().all(|a| a.items.is_empty())
        {
            let (s, e) = with_quant(&re, &it);
            let edits = if capture.is_none() { vec![(s, e, String::new())] } else { vec![] };
            o.edits(it.start, it.end, format!("`{}` is an empty group", o.site.it(&it)), edits);
        }
    }
}

fn no_empty_lookarounds_assertion(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        if let Kind::Look { alts, negate, .. } = &it.kind
            && alts.iter().any(|a| a.items.iter().all(ra::can_be_empty))
        {
            let what = if *negate { "always fails" } else { "always succeeds" };
            o.report(it.start, it.end, format!("`{}` can match the empty string, so it {what}", o.site.it(&it)));
        }
    }
}

fn no_empty_string_literal(o: &mut Out<'_, '_>) {
    for c in classes_in(&o.site.re) {
        for i in &c.items {
            if let CKind::Strings(v) = &i.kind
                && v.iter().all(|s| s.is_empty())
            {
                o.report(i.start, i.end, "This string literal holds only the empty string");
            }
        }
    }
}

fn no_escape_backspace(o: &mut Out<'_, '_>) {
    for c in classes_in(&o.site.re) {
        for i in &c.items {
            if i.kind == CKind::Char(8) && o.site.t(i.start, i.end) == "\\b" {
                o.fix(i.start, i.end, "`[\\b]` is a backspace, easily read as a word boundary: write `\\x08`", i.start, i.end, "\\x08");
            }
        }
    }
}

fn no_extra_lookaround_assertions(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        let Kind::Look { behind, alts, .. } = &it.kind else { continue };
        for a in alts {
            let inner = if *behind { a.items.first() } else { a.items.last() };
            let Some(inner) = inner else { continue };
            let Kind::Look { behind: ib, negate: false, alts: ia } = &inner.kind else { continue };
            if ib != behind {
                continue;
            }
            let body = o.site.t(inner.start + if *behind { 4 } else { 3 }, inner.end - 1).to_owned();
            let new = if ia.len() == 1 { body } else { format!("(?:{body})") };
            o.fix(inner.start, inner.end, format!("`{}` can be written into the lookaround around it", o.site.it(inner)), inner.start, inner.end, new);
        }
    }
}

fn invisible(c: char) -> bool {
    matches!(c as u32, 0x9..=0xD | 0x85 | 0xA0 | 0xAD | 0x115F | 0x1160 | 0x1680 | 0x180E | 0x2000..=0x200F | 0x2028..=0x202F | 0x205F..=0x206F | 0x3000 | 0x3164 | 0xFEFF | 0xFFA0)
}

fn no_invisible_character(o: &mut Out<'_, '_>) {
    if !o.site.literal {
        return;
    }
    let p = o.site.pattern.clone();
    for (k, c) in p.char_indices() {
        if !invisible(c) || (k > 0 && p.as_bytes()[k - 1] == b'\\') {
            continue;
        }
        let u = c as u32;
        let esc = control_escape_for(u).map(str::to_owned).unwrap_or_else(|| if u <= 0xFF { format!("\\x{u:02X}") } else { format!("\\u{u:04X}") });
        o.fix(k, k + c.len_utf8(), format!("An invisible character, U+{u:04X}, is in this pattern: write it as `{esc}`"), k, k + c.len_utf8(), esc);
    }
}

fn lazy_end(o: &mut Out<'_, '_>, alts: &[ra::Alt]) {
    for a in alts {
        let Some(last) = a.items.last() else { continue };
        match &last.kind {
            Kind::Quant { min, max, greedy: false, body, .. } if Some(*min) != *max => {
                let edits = if cap_free(last) { vec![(last.start, last.end, exact(o.site.it(body), *min))] } else { vec![] };
                o.edits(last.start, last.end, format!("`{}` is lazy at the end of the pattern, so it always matches as few as it can", o.site.it(last)), edits);
            }
            Kind::Group { alts, .. } => lazy_end(o, alts),
            _ => {}
        }
    }
}

fn no_lazy_ends(o: &mut Out<'_, '_>) {
    let alts = o.site.re.alts.clone();
    lazy_end(o, &alts);
}

const LEGACY_PROPS: &[&str] = &["$1", "$2", "$3", "$4", "$5", "$6", "$7", "$8", "$9", "input", "$_", "lastMatch", "$&", "lastParen", "$+", "leftContext", "$`", "rightContext", "$'"];

fn no_legacy_features(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let (obj, prop) = match n.kind() {
        "member_expression" => (n.child_by_field_name("object"), n.child_by_field_name("property").map(|p| text(p, src).to_owned())),
        "subscript_expression" => (n.child_by_field_name("object"), n.child_by_field_name("index").and_then(|i| js_string(i, src))),
        _ => return,
    };
    let (Some(obj), Some(prop)) = (obj, prop) else { return };
    if text(obj, src) == "RegExp" && LEGACY_PROPS.contains(&prop.as_str()) {
        out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: format!("`RegExp.{prop}` is a legacy, global record of the last match; read the match result instead"), edits: vec![] });
    }
}

fn no_misleading_capturing_group(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 1..v.len() {
            let Kind::Group { capture: Some(_), alts, .. } = &v[k].kind else { continue };
            let [ga] = alts.as_slice() else { continue };
            let Some(first) = ga.items.first() else { continue };
            if !matches!(first.kind, Kind::Quant { .. }) {
                continue;
            }
            let Some((s, 0, _, _)) = atom_set(first, &f) else { continue };
            let Kind::Quant { max: None, greedy: true, body: pb, op, .. } = &v[k - 1].kind else { continue };
            let Some(ps) = ra::item_set(pb, &f) else { continue };
            if s.is_subset(&ps) {
                o.fix(first.start, first.end, format!("`{}` always captures nothing at its start: the greedy `{}` before the group takes those characters first", o.site.it(first), o.site.it(&v[k - 1])), op.1, op.1, "?");
            }
        }
    }
}

fn joined_mark(c: u32) -> bool {
    matches!(c, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F | 0xFE00..=0xFE0F | 0x200D | 0x1F3FB..=0x1F3FF | 0xE0020..=0xE007F)
}

fn no_misleading_unicode_character(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 0..v.len() {
            let Kind::Quant { body, op, .. } = &v[k].kind else { continue };
            let Kind::Char(c) = body.kind else { continue };
            if o.site.it(body).starts_with('\\') {
                continue;
            }
            let astral = c > 0xFFFF && !f.unicode();
            let mut s = k;
            if joined_mark(c) {
                while s > 0 {
                    let Kind::Char(p) = v[s - 1].kind else { break };
                    s -= 1;
                    let joined_before = s > 0 && matches!(v[s - 1].kind, Kind::Char(0x200D));
                    if !joined_mark(p) && !joined_before {
                        break;
                    }
                }
                if s == k {
                    continue;
                }
            } else if !astral {
                continue;
            }
            let start = v[s].start;
            let chars = o.site.t(start, body.end).to_owned();
            let q = o.site.t(op.0, op.1).to_owned();
            let why = if astral && s == k { "without the `u` flag it repeats the second half of this character only" } else { "it repeats the last code point of this character only" };
            o.fix(start, v[k].end, format!("`{q}` after `{chars}` is misleading: {why}"), start, v[k].end, format!("(?:{chars}){q}"));
        }
    }
}

/// `recv.method(args…)`: its parts.
fn method_call<'t>(n: Node<'t>, src: &str) -> Option<(Node<'t>, String, Vec<Node<'t>>)> {
    if n.kind() != "call_expression" {
        return None;
    }
    let f = n.child_by_field_name("function")?;
    if f.kind() != "member_expression" {
        return None;
    }
    let obj = f.child_by_field_name("object")?;
    let prop = text(f.child_by_field_name("property")?, src).to_owned();
    let args = named_children(n.child_by_field_name("arguments")?);
    Some((obj, prop, args))
}

fn regex_arg<'t>(n: Node<'t>, src: &'t str) -> Option<Site<'t>> {
    let n = unparen(n);
    if matches!(n.kind(), "regex" | "new_expression" | "call_expression") { Site::of(n, src) } else { None }
}

fn no_missing_g_flag(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((_, m, args)) = method_call(n, src) else { return };
    if m != "replaceAll" && m != "matchAll" {
        return;
    }
    let Some(site) = args.first().and_then(|a| regex_arg(*a, src)) else { return };
    if site.re.flags.g {
        return;
    }
    let new = with_flag(&site.re.flags.raw, 'g');
    let edits = match site.flags_span {
        Some((s, e)) => vec![TextEdit::new(s, e, new)],
        None => vec![],
    };
    let a = unparen(args[0]);
    out.push(NativeFinding { start: a.start_byte(), end: a.end_byte(), message: format!("`{m}` throws a TypeError for a regular expression without the `g` flag"), edits });
}

fn no_non_standard_flag(o: &mut Out<'_, '_>) {
    let raw = o.site.re.flags.raw.clone();
    if let Some(c) = raw.chars().find(|c| !FLAG_ORDER.contains(*c)) {
        let new: String = raw.chars().filter(|x| *x != c).collect();
        o.flags(format!("`{c}` is not a standard regular expression flag"), Some(new));
    }
}

fn no_obscure_range(o: &mut Out<'_, '_>) {
    let ok = |a: u32, b: u32| {
        let both = |lo: u32, hi: u32| (lo..=hi).contains(&a) && (lo..=hi).contains(&b);
        both(0x30, 0x39) || both(0x61, 0x7A) || both(0x41, 0x5A)
    };
    for c in classes_in(&o.site.re) {
        for i in &c.items {
            if let CKind::Range { lo, hi, lo_span, hi_span } = &i.kind {
                let esc_lo = o.site.t(lo_span.0, lo_span.1).starts_with('\\');
                let esc_hi = o.site.t(hi_span.0, hi_span.1).starts_with('\\');
                if (esc_lo && esc_hi) || ok(*lo, *hi) {
                    continue;
                }
                let esc = |c: u32| if c <= 0xFF { format!("\\x{c:02X}") } else if c <= 0xFFFF { format!("\\u{c:04X}") } else { format!("\\u{{{c:X}}}") };
                let new = format!("{}-{}", esc(*lo), esc(*hi));
                let edits = if *hi <= 0xFFFF || o.site.re.flags.unicode() { vec![(i.start, i.end, new)] } else { vec![] };
                o.edits(i.start, i.end, format!("The range `{}` is obscure: it isn't plain digits or letters, so what it holds is hard to see", citem_text(o, i)), edits);
            }
        }
    }
}

fn no_octal(o: &mut Out<'_, '_>) {
    let spots: Vec<_> = o.site.re.legacy.iter().filter(|l| l.2 == Legacy::Octal).cloned().collect();
    for (s, e, _) in spots {
        let raw = o.site.t(s, e).to_owned();
        let v = u32::from_str_radix(&raw[1..], 8).unwrap_or(0);
        o.fix(s, e, format!("`{raw}` is an octal escape, easily read as a backreference: write `\\x{v:02X}`"), s, e, format!("\\x{v:02X}"));
    }
}

fn no_optional_assertion(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        if let Kind::Quant { min: 0, body, .. } = &it.kind
            && let Kind::Group { capture: None, alts, .. } = &body.kind
            && alts.iter().all(|a| a.items.iter().all(ra::zero_width))
            && alts.iter().any(|a| !a.items.is_empty())
        {
            o.fix(it.start, it.end, format!("The assertions in `{}` are optional, so they assert nothing", o.site.it(&it)), it.start, it.end, "");
        }
    }
}

/// Each item's optional contexts: a quantifier with minimum 0, or one
/// alternative of several — as spans.
fn contexts(alts: &[ra::Alt], stack: &mut Vec<(usize, usize)>, out: &mut Vec<(usize, Vec<(usize, usize)>)>) {
    for a in alts {
        let pushed = alts.len() > 1;
        if pushed {
            stack.push((a.start, a.end));
        }
        for it in &a.items {
            ctx_item(it, stack, out);
        }
        if pushed {
            stack.pop();
        }
    }
}

fn ctx_item(it: &Item, stack: &mut Vec<(usize, usize)>, out: &mut Vec<(usize, Vec<(usize, usize)>)>) {
    out.push((it.start, stack.clone()));
    match &it.kind {
        Kind::Quant { min, body, .. } => {
            if *min == 0 {
                stack.push((it.start, it.end));
            }
            ctx_item(body, stack, out);
            if *min == 0 {
                stack.pop();
            }
        }
        Kind::Group { alts, .. } | Kind::Look { alts, .. } => contexts(alts, stack, out),
        _ => {}
    }
}

fn no_potentially_useless_backreference(o: &mut Out<'_, '_>) {
    let re = o.site.re.clone();
    let mut ctx = Vec::new();
    contexts(&re.alts, &mut Vec::new(), &mut ctx);
    let items = items_in(&re);
    for b in &items {
        let Kind::Backref(r) = &b.kind else { continue };
        let num = match r {
            Ref::Num(n) => *n,
            Ref::Name(nm) => match re.names.iter().position(|x| x.as_deref() == Some(nm.as_str())) {
                Some(p) => p as u32 + 1,
                None => continue,
            },
        };
        let Some(g) = items.iter().find(|i| matches!(i.kind, Kind::Group { capture: Some(c), .. } if c == num)) else { continue };
        if b.start < g.end {
            continue; // before or inside: `js/no-useless-backreference`.
        }
        let gc = ctx.iter().find(|c| c.0 == g.start && !matches!(items.iter().find(|i| i.start == g.start).map(|i| &i.kind), Some(Kind::Quant { .. }))).map(|c| c.1.clone()).unwrap_or_default();
        let gc = ctx.iter().rev().find(|c| c.0 == g.start).map(|c| c.1.clone()).unwrap_or(gc);
        let bc = ctx.iter().rev().find(|c| c.0 == b.start).map(|c| c.1.clone()).unwrap_or_default();
        let outside: Vec<_> = gc.iter().filter(|x| !bc.contains(x)).collect();
        if outside.is_empty() {
            continue;
        }
        // In a sibling alternative of the group's: always useless, `js/no-useless-backreference`.
        let sibling = outside.iter().any(|x| disjunctions(&re).iter().any(|(alts, _)| alts.iter().any(|a| (a.start, a.end) == **x) && alts.iter().any(|a| a.start <= b.start && b.end <= a.end && (a.start, a.end) != **x)));
        if !sibling {
            o.report(b.start, b.end, format!("`{}` may refer to a group that took no part in the match, and then matches the empty string", o.site.it(b)));
        }
    }
}

fn no_standalone_backslash(o: &mut Out<'_, '_>) {
    let spots: Vec<_> = o.site.re.legacy.iter().filter(|l| l.2 == Legacy::LoneBackslash).cloned().collect();
    for (s, e, _) in spots {
        o.fix(s, e, "This backslash stands for itself, since `\\c` isn't followed by a letter: write `\\\\`", s, e, "\\\\");
    }
}

fn no_super_linear_backtracking(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for it in items_in(&o.site.re) {
        let Kind::Quant { max: None, body, .. } = &it.kind else { continue };
        let Kind::Group { alts, .. } = &body.kind else { continue };
        let whole = o.site.it(&it).to_owned();
        if alts.len() > 1 {
            let sets: Vec<Option<CharSet>> = alts.iter().map(|a| if let [i] = a.items.as_slice() { ra::item_set(i, &f) } else { None }).collect();
            'outer: for j in 1..alts.len() {
                for i in 0..j {
                    let (Some(a), Some(b)) = (&sets[i], &sets[j]) else { continue };
                    if a.disjoint(b) {
                        continue;
                    }
                    let (ai, aj) = (&alts[i], &alts[j]);
                    let msg = format!("`{whole}` can backtrack exponentially: its alternatives `{}` and `{}` match the same characters", o.site.t(ai.start, ai.end), o.site.t(aj.start, aj.end));
                    if b.is_subset(a) && aj.items.iter().all(cap_free) {
                        o.fix(it.start, it.end, msg, aj.start - 1, aj.end, "");
                    } else if a.is_subset(b) && ai.items.iter().all(cap_free) {
                        o.fix(it.start, it.end, msg, ai.start, ai.end + 1, "");
                    } else {
                        o.report(it.start, it.end, msg);
                    }
                    break 'outer;
                }
            }
            continue;
        }
        let [a] = alts.as_slice() else { continue };
        if a.items.len() < 2 {
            continue;
        }
        let unbounded = a.items.iter().any(|x| matches!(x.kind, Kind::Quant { max: None, .. }) && atom_set(x, &f).is_some());
        let rest_optional = a.items.iter().filter(|x| !(matches!(x.kind, Kind::Quant { max: None, .. }) && atom_set(x, &f).is_some())).all(ra::can_be_empty);
        if unbounded && rest_optional {
            o.report(it.start, it.end, format!("`{whole}` can backtrack exponentially: what it repeats can split one run of characters in many ways"));
        }
    }
}

fn no_super_linear_move(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    if f.y {
        return;
    }
    for a in o.site.re.alts.clone() {
        let [first, rest @ ..] = a.items.as_slice() else { continue };
        if rest.is_empty() {
            continue;
        }
        if let Kind::Quant { max: None, min, body, .. } = &first.kind
            && *min >= 1
            && ra::item_set(body, &f).is_some()
        {
            o.report(first.start, first.end, format!("`{}` at the start of an unanchored pattern makes a failed search take quadratic time on a long run of its characters", o.site.it(first)));
        }
    }
}

fn no_trivially_nested_assertion(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        let Kind::Look { negate, alts, .. } = &it.kind else { continue };
        let [a] = alts.as_slice() else { continue };
        let [inner] = a.items.as_slice() else { continue };
        let new = match (&inner.kind, negate) {
            (Kind::Start | Kind::End | Kind::Boundary { .. } | Kind::Look { .. }, false) => o.site.it(inner).to_owned(),
            (Kind::Boundary { negate: b }, true) => (if *b { "\\b" } else { "\\B" }).to_owned(),
            (Kind::Look { behind, negate: n2, .. }, true) => {
                let raw = o.site.it(inner);
                let head = match (behind, n2) {
                    (false, false) => "(?!",
                    (false, true) => "(?=",
                    (true, false) => "(?<!",
                    (true, true) => "(?<=",
                };
                format!("{head}{}", &raw[if *behind { 4 } else { 3 }..])
            }
            _ => continue,
        };
        o.fix(it.start, it.end, format!("`{}` wraps a single assertion: write `{new}`", o.site.it(&it)), it.start, it.end, new);
    }
}

/// `{a,b}` of `{c,d}` as one quantifier, when the counts have no gap.
fn combine(outer: (u32, Option<u32>), inner: (u32, Option<u32>)) -> Option<(u32, Option<u32>)> {
    let (a, b) = outer;
    let (c, d) = inner;
    let min = a.checked_mul(c)?;
    let max = match (b, d) {
        (Some(0), _) | (_, Some(0)) => Some(0),
        (Some(x), Some(y)) => Some(x.checked_mul(y)?),
        _ => None,
    };
    let top = b.unwrap_or(a + 64).min(a + 64);
    for k in a..top {
        if let Some(d) = d
            && (k as u64) * (d as u64) + 1 < (k as u64 + 1) * (c as u64)
        {
            return None;
        }
    }
    Some((min, max))
}

fn no_trivially_nested_quantifier(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        let Kind::Quant { min: a, max: b, greedy: g1, body, .. } = &it.kind else { continue };
        let Kind::Group { capture: None, alts, .. } = &body.kind else { continue };
        if !o.site.it(body).starts_with("(?:") {
            continue;
        }
        let [alt] = alts.as_slice() else { continue };
        let [inner] = alt.items.as_slice() else { continue };
        let Kind::Quant { min: c, max: d, greedy: g2, body: ib, .. } = &inner.kind else { continue };
        if g1 != g2 {
            continue;
        }
        let Some((min, max)) = combine((*a, *b), (*c, *d)) else { continue };
        let new = format!("{}{}", o.site.it(ib), ra::quant_text(min, max, *g1));
        o.fix(it.start, it.end, format!("`{}` nests one quantifier in another: write `{new}`", o.site.it(&it)), it.start, it.end, new);
    }
}

fn group_open_len(raw: &str) -> usize {
    if raw.starts_with("(?<") { raw.find('>').map(|x| x + 1).unwrap_or(1) } else { 1 }
}

/// The regex a call reads only for whether or where it matches.
fn captures_ignored<'t>(n: Node<'t>, src: &str) -> Option<Node<'t>> {
    let (obj, m, args) = method_call(n, src)?;
    let obj = unparen(obj);
    match m.as_str() {
        "test" if obj.kind() == "regex" => Some(obj),
        "search" => args.first().map(|a| unparen(*a)).filter(|a| a.kind() == "regex"),
        "replace" | "replaceAll" => {
            let re = unparen(*args.first()?);
            let r = args.get(1)?;
            (re.kind() == "regex" && r.kind() == "string" && !text(*r, src).contains('$')).then_some(re)
        }
        _ => None,
    }
}

fn no_unused_capturing_group(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some(re_node) = captures_ignored(n, src) else { return };
    let Some(site) = Site::of(re_node, src) else { return };
    if ra::has_backref(&site.re.alts) {
        return;
    }
    let groups: Vec<Item> = items_in(&site.re).into_iter().filter(|i| matches!(i.kind, Kind::Group { capture: Some(_), .. })).collect();
    if groups.is_empty() {
        return;
    }
    let mut o = Out { site: &site, out, drops_groups: true };
    // One edit making every group non-capturing, offered on each finding.
    let all: Vec<(usize, usize, String)> = groups.iter().map(|g| (g.start, g.start + group_open_len(site.it(g)), "(?:".to_owned())).collect();
    for g in &groups {
        let open = group_open_len(site.it(g));
        o.edits(g.start, g.start + open, format!("The group `{}` captures text nothing reads: make it non-capturing", site.it(g)), all.clone());
    }
}

fn no_useless_assertions(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 0..v.len() {
            let it = &v[k];
            let verdict: Option<bool> = match &it.kind {
                Kind::Start if !f.m => v[..k].iter().any(|p| !ra::can_be_empty(p)).then_some(false),
                Kind::End if !f.m => v[k + 1..].iter().any(|p| !ra::can_be_empty(p)).then_some(false),
                Kind::Boundary { negate } if k > 0 && k + 1 < v.len() => {
                    let (Some(p), Some(n)) = (ra::item_set(&v[k - 1], &f), ra::item_set(&v[k + 1], &f)) else { continue };
                    let (pw, nw) = (wordy(&p, &f), wordy(&n, &f));
                    if pw == Wordy::Mixed || nw == Wordy::Mixed {
                        continue;
                    }
                    Some((pw != nw) != *negate)
                }
                _ => None,
            };
            match verdict {
                Some(true) => o.fix(it.start, it.end, format!("`{}` always succeeds here, so it does nothing", o.site.it(it)), it.start, it.end, ""),
                Some(false) => o.report(it.start, it.end, format!("`{}` always fails here, so this alternative can never match", o.site.it(it))),
                None => {}
            }
        }
    }
}

fn no_useless_character_class(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for it in items_in(&o.site.re) {
        let Kind::Class(c) = &it.kind else { continue };
        if c.negate || c.op != ClassOp::Union {
            continue;
        }
        let [i] = c.items.as_slice() else { continue };
        let new = match &i.kind {
            CKind::Char(8) => continue,
            CKind::Char(v) => ra::char_text(*v, &f),
            CKind::Set(_) => citem_text(o, i).to_owned(),
            _ => continue,
        };
        o.fix(it.start, it.end, format!("`{}` is a class of one: write `{new}`", o.site.it(&it)), it.start, it.end, new);
    }
}

/// `$n` and `$<name>` in a replacement's text, with their offsets in it.
fn dollar_refs(raw: &str) -> Vec<(usize, usize, Result<u32, String>)> {
    let b = raw.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'$' {
            i += 1;
            continue;
        }
        match b.get(i + 1) {
            Some(b'$') => i += 2,
            Some(d) if d.is_ascii_digit() => {
                let len = if b.get(i + 2).is_some_and(|x| x.is_ascii_digit()) { 2 } else { 1 };
                let n: u32 = raw[i + 1..i + 1 + len].parse().unwrap_or(0);
                out.push((i, i + 1 + len, Ok(n)));
                i += 1 + len;
            }
            Some(b'<') => match raw[i..].find('>') {
                Some(c) => {
                    out.push((i, i + c + 1, Err(raw[i + 2..i + c].to_owned())));
                    i += c + 1;
                }
                None => i += 2,
            },
            _ => i += 1,
        }
    }
    out
}

fn replace_parts<'t>(n: Node<'t>, src: &'t str) -> Option<(Site<'t>, Node<'t>)> {
    let (_, m, args) = method_call(n, src)?;
    if m != "replace" && m != "replaceAll" {
        return None;
    }
    let site = regex_arg(*args.first()?, src)?;
    let r = *args.get(1)?;
    (r.kind() == "string").then_some((site, r))
}

fn no_useless_dollar_replacements(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((site, r)) = replace_parts(n, src) else { return };
    let raw = text(r, src);
    let named = site.re.names.iter().any(|x| x.is_some());
    for (s, e, what) in dollar_refs(raw) {
        let bad = match &what {
            Ok(0) => continue,
            Ok(k) => {
                let k = if *k >= 10 && *k > site.re.groups { *k / 10 } else { *k };
                k > site.re.groups
            }
            Err(name) => named && !site.re.names.iter().any(|x| x.as_deref() == Some(name.as_str())),
        };
        if bad {
            let at = r.start_byte() + s;
            out.push(NativeFinding { start: at, end: r.start_byte() + e, message: format!("`{}` refers to a group the regular expression doesn't have, so it is inserted as text: write `$${}` to say so", &raw[s..e], &raw[s + 1..e]), edits: vec![TextEdit::new(at, at + 1, "$$")] });
        }
    }
}

const CLASS_NEEDLESS: &str = ".*+?(){}|$";

fn no_useless_escape(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    let mut spots: Vec<(usize, usize)> = o.site.re.legacy.iter().filter(|l| l.2 == Legacy::IdentityEscape).map(|l| (l.0, l.1)).collect();
    if !f.v {
        for c in classes_in(&o.site.re) {
            for i in &c.items {
                let raw = citem_text(o, i);
                if raw.len() == 2 && raw.starts_with('\\') && CLASS_NEEDLESS.contains(&raw[1..]) {
                    spots.push((i.start, i.end));
                }
            }
        }
    }
    spots.sort();
    spots.dedup();
    for (s, e) in spots {
        let raw = o.site.t(s, e).to_owned();
        if raw == "\\/" || raw.len() < 2 || raw[1..].starts_with(['x', 'u']) && raw.len() > 2 {
            continue;
        }
        let ch = raw[1..].to_owned();
        o.fix(s, e, format!("`{raw}` escapes a character that needs no escape here"), s, e, ch);
    }
}

fn no_useless_flag(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    let re = o.site.re.clone();
    let items = items_in(&re);
    let mut drop = String::new();
    if f.i {
        let mut cased = ra::has_backref(&re.alts);
        let plain = ra::Flags::parse(&f.raw.replace('i', ""));
        for it in &items {
            if matches!(it.kind, Kind::Char(_) | Kind::Class(_) | Kind::Set(_)) {
                match ra::item_set(it, &plain) {
                    Some(s) => cased |= s.fold() != s,
                    None => cased = true,
                }
            }
        }
        if !cased {
            drop.push('i');
        }
    }
    if f.m && !items.iter().any(|i| matches!(i.kind, Kind::Start | Kind::End)) {
        drop.push('m');
    }
    if f.s && !items.iter().any(|i| matches!(i.kind, Kind::Dot)) {
        drop.push('s');
    }
    if drop.is_empty() {
        return;
    }
    let new: String = f.raw.chars().filter(|x| !drop.contains(*x)).collect();
    let why: Vec<&str> = drop.chars().map(|c| match c {
        'i' => "`i`: nothing in the pattern has an upper and a lower case",
        'm' => "`m`: the pattern has no `^` or `$`",
        _ => "`s`: the pattern has no `.`",
    }).collect();
    o.flags(format!("A flag changes nothing — {}", why.join("; ")), Some(new));
}

fn no_useless_lazy(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 0..v.len() {
            let Kind::Quant { min, max, greedy: false, body, op } = &v[k].kind else { continue };
            let fixed = Some(*min) == *max;
            let disjoint = !fixed
                && v.get(k + 1).is_some_and(|n| {
                    let (Some(a), Some(b)) = (ra::item_set(body, &f), ra::item_set(n, &f)) else { return false };
                    a.disjoint(&b)
                });
            if fixed || disjoint {
                let why = if fixed { "it repeats a fixed number of times" } else { "what follows can't match what it repeats" };
                o.fix(op.0, op.1, format!("`{}` is lazy for nothing: {why}", o.site.t(op.0, op.1)), op.1 - 1, op.1, "");
            }
        }
    }
}

fn no_useless_non_capturing_group(o: &mut Out<'_, '_>) {
    let re = o.site.re.clone();
    let quantified: Vec<usize> = items_in(&re).iter().filter_map(|it| if let Kind::Quant { body, .. } = &it.kind { Some(body.start) } else { None }).collect();
    for (alts, holder) in disjunctions(&re) {
        let sole = alts.len() == 1;
        for a in &alts {
            for it in &a.items {
                let Kind::Group { capture: None, alts: inner, .. } = &it.kind else { continue };
                let raw = o.site.it(it).to_owned();
                if !raw.starts_with("(?:") || inner.iter().all(|x| x.items.is_empty()) {
                    continue;
                }
                let body = raw[3..raw.len() - 1].to_owned();
                let ok = if quantified.contains(&it.start) {
                    inner.len() == 1 && inner[0].items.len() == 1 && matches!(inner[0].items[0].kind, Kind::Char(_) | Kind::Set(_) | Kind::Class(_) | Kind::Dot | Kind::Group { .. } | Kind::Backref(_))
                } else if inner.len() == 1 {
                    true
                } else {
                    sole && a.items.len() == 1 && holder.as_ref().is_none_or(|h| !matches!(h.kind, Kind::Quant { .. }))
                };
                if ok {
                    o.fix(it.start, it.end, format!("`{raw}` groups nothing that needs a group"), it.start, it.end, body);
                }
            }
        }
    }
}

fn no_useless_quantifier(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        let Kind::Quant { min, max, body, op, .. } = &it.kind else { continue };
        let q = o.site.t(op.0, op.1).to_owned();
        if *min == 1 && *max == Some(1) {
            o.fix(op.0, op.1, format!("`{q}` repeats once: it does nothing"), op.0, op.1, "");
        } else if *min >= 1 && ra::zero_width(body) {
            o.fix(op.0, op.1, format!("`{q}` repeats something that matches only the empty string"), op.0, op.1, "");
        }
    }
}

fn no_useless_range(o: &mut Out<'_, '_>) {
    for c in classes_in(&o.site.re) {
        for (k, i) in c.items.iter().enumerate() {
            let CKind::Range { lo, hi, lo_span, hi_span } = &i.kind else { continue };
            if hi - lo > 1 {
                continue;
            }
            let a = o.site.t(lo_span.0, lo_span.1).to_owned();
            let new = if lo == hi { a } else { format!("{a}{}", o.site.t(hi_span.0, hi_span.1)) };
            let edits = remove_items(o, &c, &[], Some((k, new))).unwrap_or_default();
            o.edits(i.start, i.end, format!("The range `{}` holds one or two characters: list them", citem_text(o, i)), edits);
        }
    }
}

fn no_useless_set_operand(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for c in classes_in(&o.site.re) {
        if c.op == ClassOp::Union || c.items.len() != 2 {
            continue;
        }
        let (Some(a), Some(b)) = (ra::citem_set(&c.items[0], &f), ra::citem_set(&c.items[1], &f)) else { continue };
        let (ta, tb) = (citem_text(o, &c.items[0]).to_owned(), citem_text(o, &c.items[1]).to_owned());
        let keep = match c.op {
            ClassOp::Intersection if b.is_subset(&a) => Some((ta, tb)),
            ClassOp::Intersection if a.is_subset(&b) => Some((tb, ta)),
            ClassOp::Subtraction if a.disjoint(&b) => Some((tb, ta)),
            _ => None,
        };
        let Some((useless, kept)) = keep else { continue };
        let neg = if c.negate { "^" } else { "" };
        o.fix(c.start, c.end, format!("The operand `{useless}` changes nothing in `{}`", o.site.t(c.start, c.end)), c.start, c.end, format!("[{neg}{kept}]"));
    }
}

fn no_useless_string_literal(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for c in classes_in(&o.site.re) {
        for i in &c.items {
            let CKind::Strings(v) = &i.kind else { continue };
            let (single, multi): (Vec<&String>, Vec<&String>) = v.iter().partition(|s| s.chars().count() == 1);
            if single.is_empty() {
                continue;
            }
            let mut new: String = single.iter().map(|s| ra::class_char_text(s.chars().next().unwrap() as u32, &f)).collect();
            if !multi.is_empty() {
                new.push_str(&format!("\\q{{{}}}", multi.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("|")));
            }
            o.fix(i.start, i.end, format!("`{}` holds single characters: put them in the class directly", citem_text(o, i)), i.start, i.end, new);
        }
    }
}

fn no_useless_two_nums_quantifier(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        let Kind::Quant { min, max: Some(m), greedy, op, .. } = &it.kind else { continue };
        let q = o.site.t(op.0, op.1).to_owned();
        if min == m && q.contains(',') {
            let new = format!("{{{min}}}{}", if *greedy { "" } else { "?" });
            o.fix(op.0, op.1, format!("`{q}` gives the same number twice: write `{new}`"), op.0, op.1, new);
        }
    }
}

fn no_zero_quantifier(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        if let Kind::Quant { max: Some(0), .. } = &it.kind {
            let edits = if cap_free(&it) { vec![(it.start, it.end, String::new())] } else { vec![] };
            o.edits(it.start, it.end, format!("`{}` repeats zero times: it matches only the empty string", o.site.it(&it)), edits);
        }
    }
}

fn optimal_lookaround_quantifier(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        let Kind::Look { behind, alts, .. } = &it.kind else { continue };
        for a in alts {
            let q = if *behind { a.items.first() } else { a.items.last() };
            let Some(q) = q else { continue };
            let Kind::Quant { min, max, body, .. } = &q.kind else { continue };
            if Some(*min) == *max || !cap_free(q) {
                continue;
            }
            let new = exact(o.site.it(body), *min);
            let side = if *behind { "start of a lookbehind" } else { "end of a lookahead" };
            let shown = if new.is_empty() { "nothing".to_owned() } else { format!("`{new}`") };
            o.fix(q.start, q.end, format!("`{}` at the {side} only ever needs its minimum: write {shown}", o.site.it(q)), q.start, q.end, new);
        }
    }
}

fn optimal_quantifier_concatenation(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 0..v.len().saturating_sub(1) {
            let (x, y) = (&v[k], &v[k + 1]);
            if !matches!(x.kind, Kind::Quant { .. }) && !matches!(y.kind, Kind::Quant { .. }) {
                continue;
            }
            let (Some((a, a1, b1, g1)), Some((b, a2, b2, g2))) = (atom_set(x, &f), atom_set(y, &f)) else { continue };
            if !g1 || !g2 {
                continue;
            }
            let body = |i: &Item| match &i.kind {
                Kind::Quant { body, .. } => o.site.it(body).to_owned(),
                _ => o.site.it(i).to_owned(),
            };
            let (bx, by) = (body(x), body(y));
            let both = format!("{}{}", o.site.it(x), o.site.it(y));
            if a == b && bx == by && (b1.is_none() || b2.is_none()) {
                let new = format!("{bx}{}", ra::quant_text(a1 + a2, None, true));
                if new != both {
                    o.fix(x.start, y.end, format!("`{both}` is `{new}`"), x.start, y.end, new);
                }
            } else if b1.is_none() && b.is_subset(&a) && a != b && Some(a2) != b2 {
                o.fix(y.start, y.end, format!("In `{both}`, `{}` only ever matches its minimum: `{}` takes the rest first", o.site.it(y), o.site.it(x)), y.start, y.end, exact(&by, a2));
            } else if b2.is_none() && a.is_subset(&b) && a != b && Some(a1) != b1 {
                o.fix(x.start, x.end, format!("In `{both}`, `{}` only ever needs its minimum: `{}` can take the rest", o.site.it(x), o.site.it(y)), x.start, x.end, exact(&bx, a1));
            }
        }
    }
}

fn prefer_character_class(o: &mut Out<'_, '_>) {
    for (alts, holder) in disjunctions(&o.site.re) {
        if alts.len() < 3 {
            continue;
        }
        let mut parts = Vec::new();
        for a in &alts {
            let [i] = a.items.as_slice() else {
                parts.clear();
                break;
            };
            let t = match &i.kind {
                Kind::Char(c) => ra::class_char_text(*c, &o.site.re.flags),
                Kind::Set(_) => o.site.it(i).to_owned(),
                Kind::Class(c) if !c.negate && c.op == ClassOp::Union => o.site.t(c.start + 1, c.end - 1).to_owned(),
                _ => {
                    parts.clear();
                    break;
                }
            };
            parts.push(t);
        }
        if parts.is_empty() {
            continue;
        }
        let class = format!("[{}]", parts.concat());
        let (s, e) = (alts[0].start, alts[alts.len() - 1].end);
        let (rs, re_) = match &holder {
            Some(h) if matches!(h.kind, Kind::Group { capture: None, .. }) && o.site.it(h).starts_with("(?:") => (h.start, h.end),
            _ => (s, e),
        };
        o.fix(s, e, format!("These single-character alternatives are one character class: `{class}`"), rs, re_, class);
    }
}

/// Replace the items of a class that together are exactly `target` with
/// `lower` (`\d`, `\w`).
fn prefer_short(o: &mut Out<'_, '_>, target: CharSet, lower: &str, upper: &str) {
    let f = o.site.re.flags.clone();
    let target = if f.i { target.fold() } else { target };
    for it in items_in(&o.site.re) {
        let Kind::Class(c) = &it.kind else { continue };
        if c.op != ClassOp::Union {
            continue;
        }
        let mut idx = Vec::new();
        let mut union = CharSet::empty();
        for (k, i) in c.items.iter().enumerate() {
            if !matches!(i.kind, CKind::Char(_) | CKind::Range { .. }) {
                continue;
            }
            let Some(s) = ra::citem_set(i, &f) else { continue };
            if s.is_subset(&target) {
                idx.push(k);
                union = union.union(&s);
            }
        }
        if idx.is_empty() || union != target {
            continue;
        }
        let raw = o.site.it(&it).to_owned();
        if idx.len() == c.items.len() {
            let new = if c.negate { upper } else { lower };
            o.fix(it.start, it.end, format!("`{raw}` is `{new}`"), it.start, it.end, new);
        } else {
            let edits = remove_items(o, c, &idx[1..], Some((idx[0], lower.to_owned()))).unwrap_or_default();
            o.edits(it.start, it.end, format!("Part of `{raw}` is `{lower}`"), edits);
        }
    }
}

fn prefer_d(o: &mut Out<'_, '_>) {
    prefer_short(o, ra::digit(), "\\d", "\\D");
}

fn prefer_w(o: &mut Out<'_, '_>) {
    prefer_short(o, ra::word(), "\\w", "\\W");
}

fn prefer_escape_replacement_dollar_char(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((_, r)) = replace_parts(n, src) else { return };
    let raw = text(r, src);
    let b = raw.as_bytes();
    let mut i = 1;
    while i + 1 < b.len() {
        if b[i] == b'$' {
            match b.get(i + 1) {
                Some(b'$') => {
                    i += 2;
                    continue;
                }
                Some(c) if b"&`'<".contains(c) || c.is_ascii_digit() => {}
                _ => {
                    let at = r.start_byte() + i;
                    out.push(NativeFinding { start: at, end: at + 1, message: "A lone `$` in a replacement: write `$$` for a dollar sign".into(), edits: vec![TextEdit::new(at, at + 1, "$$")] });
                }
            }
        }
        i += 1;
    }
}

fn prefer_lookaround(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((site, r)) = replace_parts(n, src) else { return };
    if !site.literal || site.re.groups != 1 || site.re.alts.len() != 1 || ra::has_backref(&site.re.alts) {
        return;
    }
    let raw = text(r, src);
    let q = &raw[..1];
    let body = &raw[1..raw.len() - 1];
    let refs = dollar_refs(body);
    let [(rs, re_, Ok(1))] = refs.as_slice() else { return };
    let items = &site.re.alts[0].items;
    let (Some(first), Some(last)) = (items.first(), items.last()) else { return };
    let group = |it: &Item| matches!(it.kind, Kind::Group { capture: Some(1), .. });
    let p = &site.pattern;
    let (new_pat, new_rep) = if *rs == 0 && group(first) && items.len() > 1 {
        let inner = &p[first.start + group_open_len(site.it(first))..first.end - 1];
        (format!("(?<={inner}){}", &p[first.end..]), format!("{q}{}{q}", &body[*re_..]))
    } else if *re_ == body.len() && group(last) && items.len() > 1 {
        let inner = &p[last.start + group_open_len(site.it(last))..last.end - 1];
        (format!("{}(?={inner})", &p[..last.start]), format!("{q}{}{q}", &body[..*rs]))
    } else {
        return;
    };
    if ra::parse(&new_pat, &site.re.flags.raw).is_err() {
        return;
    }
    out.push(NativeFinding {
        start: site.whole.0,
        end: site.whole.1,
        message: "The captured text is only put back unchanged: match it with a lookaround instead".into(),
        edits: vec![TextEdit::new(site.base, site.base + p.len(), new_pat), TextEdit::new(r.start_byte(), r.end_byte(), new_rep)],
    });
}

fn prefer_named_backreference(o: &mut Out<'_, '_>) {
    let names = o.site.re.names.clone();
    for it in items_in(&o.site.re) {
        if let Kind::Backref(Ref::Num(n)) = &it.kind
            && let Some(Some(name)) = names.get(*n as usize - 1)
        {
            o.fix(it.start, it.end, format!("Refer to the group by its name: `\\k<{name}>`"), it.start, it.end, format!("\\k<{name}>"));
        }
    }
}

fn prefer_named_capture_group(o: &mut Out<'_, '_>) {
    for it in items_in(&o.site.re) {
        if let Kind::Group { capture: Some(_), name: None, .. } = &it.kind {
            o.report(it.start, it.start + 1, format!("`{}` is a capturing group with no name", o.site.it(&it)));
        }
    }
}

fn prefer_named_replacement(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((site, r)) = replace_parts(n, src) else { return };
    let raw = text(r, src);
    for (s, e, what) in dollar_refs(&raw[1..raw.len() - 1]) {
        let Ok(k) = what else { continue };
        if k == 0 {
            continue;
        }
        if let Some(Some(name)) = site.re.names.get(k as usize - 1) {
            let (a, b) = (r.start_byte() + 1 + s, r.start_byte() + 1 + e);
            out.push(NativeFinding { start: a, end: b, message: format!("Refer to the group by its name: `$<{name}>`"), edits: vec![TextEdit::new(a, b, format!("$<{name}>"))] });
        }
    }
}

fn quant_style(o: &mut Out<'_, '_>, from: &str, to: &str) {
    for it in items_in(&o.site.re) {
        let Kind::Quant { op, .. } = &it.kind else { continue };
        let q = o.site.t(op.0, op.1).to_owned();
        let core = q.trim_end_matches('?');
        if core == from {
            let lazy = if q.len() > core.len() { "?" } else { "" };
            o.fix(op.0, op.1, format!("Write `{q}` as `{to}{lazy}`"), op.0, op.0 + core.len(), to);
        }
    }
}

fn prefer_plus_quantifier(o: &mut Out<'_, '_>) {
    quant_style(o, "{1,}", "+");
}
fn prefer_star_quantifier(o: &mut Out<'_, '_>) {
    quant_style(o, "{0,}", "*");
}
fn prefer_question_quantifier(o: &mut Out<'_, '_>) {
    quant_style(o, "{0,1}", "?");
}

fn prefer_predefined_assertion(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    let all = CharSet::of(&[(0, f.max())]);
    let w = if f.i { ra::word().fold() } else { ra::word() };
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 0..v.len() {
            let Kind::Look { behind, negate, alts } = &v[k].kind else { continue };
            let [a] = alts.as_slice() else { continue };
            let [inner] = a.items.as_slice() else { continue };
            let Some(s) = ra::item_set(inner, &f) else { continue };
            let new: Option<&str> = if *negate && s == all && !f.m {
                Some(if *behind { "^" } else { "$" })
            } else if s == w {
                let side = if *behind { v.get(k + 1) } else if k > 0 { v.get(k - 1) } else { None };
                match (side.and_then(|x| ra::item_set(x, &f)).map(|x| wordy(&x, &f)), negate) {
                    (Some(Wordy::Word), true) | (Some(Wordy::NonWord), false) => Some("\\b"),
                    (Some(Wordy::NonWord), true) | (Some(Wordy::Word), false) => Some("\\B"),
                    _ => None,
                }
            } else {
                None
            };
            if let Some(new) = new {
                let it = &v[k];
                o.fix(it.start, it.end, format!("`{}` is the assertion `{new}`", o.site.it(it)), it.start, it.end, new);
            }
        }
    }
}

fn is_atom(it: &Item) -> bool {
    matches!(it.kind, Kind::Char(_) | Kind::Set(_) | Kind::Class(_) | Kind::Dot)
}

fn prefer_quantifier(o: &mut Out<'_, '_>) {
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        let mut k = 0;
        while k < v.len() {
            if !is_atom(&v[k]) {
                k += 1;
                continue;
            }
            let t = o.site.it(&v[k]).to_owned();
            let mut j = k + 1;
            while j < v.len() && is_atom(&v[j]) && o.site.it(&v[j]) == t {
                j += 1;
            }
            let n = j - k;
            let new = format!("{t}{{{n}}}");
            if n >= 2 && new.len() < t.len() * n {
                o.fix(v[k].start, v[j - 1].end, format!("`{}` repeats `{t}`: write `{new}`", o.site.t(v[k].start, v[j - 1].end)), v[k].start, v[j - 1].end, new);
            }
            k = j;
        }
    }
}

fn prefer_range(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    let cat = |v: u32| match v {
        0x30..=0x39 => 1,
        0x61..=0x7A => 2,
        0x41..=0x5A => 3,
        _ => 0,
    };
    for c in classes_in(&o.site.re) {
        let mut chars: Vec<(usize, u32)> = c.items.iter().enumerate().filter_map(|(k, i)| match i.kind {
            CKind::Char(v) if cat(v) != 0 && !citem_text(o, i).starts_with('\\') => Some((k, v)),
            _ => None,
        }).collect();
        chars.sort_by_key(|x| x.1);
        chars.dedup_by_key(|x| x.1);
        let mut s = 0;
        while s < chars.len() {
            let mut e = s + 1;
            while e < chars.len() && chars[e].1 == chars[e - 1].1 + 1 && cat(chars[e].1) == cat(chars[s].1) {
                e += 1;
            }
            if e - s >= 3 {
                let run = &chars[s..e];
                let first = run.iter().map(|x| x.0).min().unwrap();
                let drop: Vec<usize> = run.iter().map(|x| x.0).filter(|k| *k != first).collect();
                let new = format!("{}-{}", ra::class_char_text(run[0].1, &f), ra::class_char_text(run[run.len() - 1].1, &f));
                let edits = remove_items(o, &c, &drop, Some((first, new.clone()))).unwrap_or_default();
                let i = &c.items[first];
                o.edits(i.start, i.end, format!("These consecutive characters are the range `{new}`"), edits);
            }
            s = e;
        }
    }
}

/// Whether only the truth of `n`'s value is used.
fn boolean_context(n: Node<'_>) -> bool {
    let mut cur = n;
    while let Some(p) = cur.parent() {
        match p.kind() {
            "parenthesized_expression" => {
                if p.parent().is_some_and(|g| matches!(g.kind(), "if_statement" | "while_statement" | "do_statement")) {
                    return true;
                }
            }
            "unary_expression" => return p.child(0).is_some_and(|c| c.kind() == "!"),
            "ternary_expression" => return p.child_by_field_name("condition").is_some_and(|c| c.id() == cur.id()),
            "binary_expression" => {
                if !p.child_by_field_name("operator").is_some_and(|o| matches!(o.kind(), "&&" | "||")) {
                    return false;
                }
            }
            _ => return false,
        }
        cur = p;
    }
    false
}

fn prefer_regexp_exec(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((obj, m, args)) = method_call(n, src) else { return };
    if m != "match" || args.len() != 1 || boolean_context(n) {
        return;
    }
    let a = unparen(args[0]);
    if a.kind() != "regex" {
        return;
    }
    let Some(site) = Site::of(a, src) else { return };
    if site.re.flags.g {
        return;
    }
    let new = format!("{}.exec({})", text(a, src), text(obj, src));
    out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: "Call `RegExp#exec` rather than `String#match` for a regular expression without `g`".into(), edits: vec![TextEdit::new(n.start_byte(), n.end_byte(), new)] });
}

fn prefer_regexp_test(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let Some((obj, m, args)) = method_call(n, src) else { return };
    if !boolean_context(n) || args.len() != 1 {
        return;
    }
    let ob = unparen(obj);
    match m.as_str() {
        "exec" if ob.kind() == "regex" => {
            let Some(site) = Site::of(ob, src) else { return };
            if site.re.flags.g || site.re.flags.y {
                return;
            }
            let Some(p) = n.child_by_field_name("function").and_then(|f| f.child_by_field_name("property")) else { return };
            out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: "Only whether it matches is used: call `test`, not `exec`".into(), edits: vec![TextEdit::new(p.start_byte(), p.end_byte(), "test")] });
        }
        "match" => {
            let a = unparen(args[0]);
            if a.kind() != "regex" {
                return;
            }
            let Some(site) = Site::of(a, src) else { return };
            if site.re.flags.g || site.re.flags.y {
                return;
            }
            let new = format!("{}.test({})", text(a, src), text(obj, src));
            out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: "Only whether it matches is used: call `RegExp#test`, not `String#match`".into(), edits: vec![TextEdit::new(n.start_byte(), n.end_byte(), new)] });
        }
        _ => {}
    }
}

fn prefer_result_array_groups(n: Node<'_>, src: &str, out: &mut Vec<NativeFinding>) {
    let (Some(name), Some(value)) = (n.child_by_field_name("name"), n.child_by_field_name("value")) else { return };
    if name.kind() != "identifier" {
        return;
    }
    let Some((obj, m, args)) = method_call(unparen(value), src) else { return };
    let re_node = match m.as_str() {
        "exec" => unparen(obj),
        "match" if args.len() == 1 => unparen(args[0]),
        _ => return,
    };
    if re_node.kind() != "regex" {
        return;
    }
    let Some(site) = Site::of(re_node, src) else { return };
    if site.re.flags.g && m == "match" {
        return;
    }
    let var = text(name, src);
    let mut scope = n;
    while let Some(p) = scope.parent() {
        scope = p;
        if matches!(scope.kind(), "statement_block" | "program") {
            break;
        }
    }
    let mut stack = vec![scope];
    while let Some(x) = stack.pop() {
        if x.kind() == "subscript_expression"
            && let (Some(ob), Some(i)) = (x.child_by_field_name("object"), x.child_by_field_name("index"))
            && ob.kind() == "identifier"
            && text(ob, src) == var
            && i.kind() == "number"
            && let Ok(k) = text(i, src).parse::<usize>()
            && k >= 1
            && let Some(Some(g)) = site.re.names.get(k - 1)
        {
            let opt = text(x, src)[var.len()..].trim_start().starts_with("?.");
            let new = format!("{var}{}groups.{g}", if opt { "?." } else { "." });
            out.push(NativeFinding { start: x.start_byte(), end: x.end_byte(), message: format!("Read the named group: `{new}`"), edits: vec![TextEdit::new(x.start_byte(), x.end_byte(), new)] });
        }
        let mut c = x.walk();
        stack.extend(x.children(&mut c));
    }
}

fn operand_text(o: &Out<'_, '_>, it: &Item) -> Option<String> {
    match &it.kind {
        Kind::Char(c) if char::from_u32(*c).is_some_and(|ch| ch.is_ascii_alphanumeric()) => Some(o.site.it(it).to_owned()),
        Kind::Set(_) | Kind::Class(_) => Some(o.site.it(it).to_owned()),
        _ => None,
    }
}

fn prefer_set_operation(o: &mut Out<'_, '_>) {
    if !o.site.re.flags.v {
        return;
    }
    for alt in alts_in(&o.site.re) {
        let v = &alt.items;
        for k in 0..v.len().saturating_sub(1) {
            let Kind::Look { behind: false, negate: true, alts } = &v[k].kind else { continue };
            let [a] = alts.as_slice() else { continue };
            let [inner] = a.items.as_slice() else { continue };
            let (Some(x), Some(y)) = (operand_text(o, inner), operand_text(o, &v[k + 1])) else { continue };
            let new = format!("[{y}--{x}]");
            o.fix(v[k].start, v[k + 1].end, format!("`{}` is the set operation `{new}`", o.site.t(v[k].start, v[k + 1].end)), v[k].start, v[k + 1].end, new);
        }
    }
}

fn prefer_unicode_codepoint_escapes(o: &mut Out<'_, '_>) {
    if !o.site.re.flags.unicode() {
        return;
    }
    for (c, s, e) in char_spots(&o.site.re) {
        let raw = o.site.t(s, e).to_owned();
        if c > 0xFFFF && raw.starts_with("\\u") && raw.len() == 12 {
            o.fix(s, e, format!("Write the surrogate pair `{raw}` as `\\u{{{c:X}}}`"), s, e, format!("\\u{{{c:X}}}"));
        }
    }
}

/// Whether adding `flag` (`u` or `v`) can't change what the pattern matches.
fn unicode_safe(site: &Site<'_>, flag: char) -> bool {
    let f = &site.re.flags;
    if f.i || !site.pattern.is_ascii() {
        return false;
    }
    if ra::parse(&site.pattern, &with_flag(&f.raw.replace('u', ""), flag)).is_err() {
        return false;
    }
    let negated = items_in(&site.re).iter().any(|it| matches!(&it.kind, Kind::Dot | Kind::Set(ra::SetEsc { negate: true, .. })) || matches!(&it.kind, Kind::Class(c) if c.negate));
    let in_class = classes_in(&site.re).iter().any(|c| c.negate || c.items.iter().any(|i| matches!(&i.kind, CKind::Set(e) if e.negate)));
    !negated && !in_class
}

fn require_unicode_regexp(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    if f.unicode() {
        return;
    }
    let new = unicode_safe(o.site, 'u').then(|| with_flag(&f.raw, 'u'));
    o.flags("Use the `u` flag, so the pattern reads code points and rejects mistaken escapes", new);
}

fn require_unicode_sets_regexp(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    if f.v {
        return;
    }
    let base: String = f.raw.chars().filter(|c| *c != 'u').collect();
    let new = if f.u {
        let cand = with_flag(&base, 'v');
        let negated = classes_in(&o.site.re).iter().any(|c| c.negate);
        (ra::parse(&o.site.pattern, &cand).is_ok() && !(f.i && negated)).then_some(cand)
    } else {
        unicode_safe(o.site, 'v').then(|| with_flag(&base, 'v'))
    };
    o.flags("Use the `v` flag, the stricter successor of `u`", new);
}

fn simplify_set_operations(o: &mut Out<'_, '_>) {
    if !o.site.re.flags.v {
        return;
    }
    for c in classes_in(&o.site.re) {
        let neg = if c.negate { "^" } else { "" };
        let inner_neg = |i: &CItem| match &i.kind {
            CKind::Class(n) if n.negate && n.op == ClassOp::Union => Some(o.site.t(n.start + 2, n.end - 1).to_owned()),
            _ => None,
        };
        let new = match (c.op, c.items.as_slice()) {
            (ClassOp::Intersection, [a, b]) => inner_neg(b).map(|bb| format!("[{neg}{}--[{bb}]]", citem_text(o, a))),
            (ClassOp::Subtraction, [a, b]) => inner_neg(b).map(|bb| format!("[{neg}{}&&[{bb}]]", citem_text(o, a))),
            (ClassOp::Union, [a, b]) if c.negate => match (inner_neg(a), inner_neg(b)) {
                (Some(x), Some(y)) => Some(format!("[[{x}]&&[{y}]]")),
                _ => None,
            },
            _ => None,
        };
        if let Some(new) = new {
            o.fix(c.start, c.end, format!("`{}` is simpler as `{new}`", o.site.t(c.start, c.end)), c.start, c.end, new);
        }
    }
}

fn sort_alternatives(o: &mut Out<'_, '_>) {
    for (alts, _) in disjunctions(&o.site.re) {
        if alts.len() < 2 {
            continue;
        }
        let texts: Vec<String> = alts.iter().map(|a| o.site.t(a.start, a.end).to_owned()).collect();
        if !texts.iter().all(|t| !t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric())) {
            continue;
        }
        if texts.iter().enumerate().any(|(i, a)| texts.iter().enumerate().any(|(j, b)| i != j && b.starts_with(a.as_str()))) {
            continue;
        }
        let mut sorted = texts.clone();
        sorted.sort();
        if sorted != texts {
            let (s, e) = (alts[0].start, alts[alts.len() - 1].end);
            o.fix(s, e, format!("Sort these alternatives: `{}`", sorted.join("|")), s, e, sorted.join("|"));
        }
    }
}

fn sort_flags(o: &mut Out<'_, '_>) {
    let raw = o.site.re.flags.raw.clone();
    let sorted = sorted_flags(&raw);
    if sorted != raw {
        o.flags(format!("Write the flags in order: `{sorted}`"), Some(sorted));
    }
}

fn strict(o: &mut Out<'_, '_>) {
    for (s, e, what) in o.site.re.legacy.clone() {
        let raw = o.site.t(s, e).to_owned();
        match what {
            Legacy::LoneBracket => o.fix(s, e, format!("`{raw}` stands for itself only outside strict syntax: escape it"), s, e, format!("\\{raw}")),
            Legacy::IdentityEscape => o.fix(s, e, format!("`{raw}` isn't a valid escape in strict syntax"), s, e, raw[1..].to_owned()),
            Legacy::Octal => {
                let v = u32::from_str_radix(&raw[1..], 8).unwrap_or(0);
                o.fix(s, e, format!("The octal escape `{raw}` isn't allowed in strict syntax"), s, e, format!("\\x{v:02X}"));
            }
            Legacy::LoneBackslash => o.fix(s, e, "A lone backslash before `c` isn't allowed in strict syntax", s, e, "\\\\"),
            Legacy::QuantifiedLookahead => o.report(s, e, format!("`{raw}` quantifies a lookahead, which strict syntax forbids")),
            Legacy::ClassEscapeRange => {
                if let Some(d) = raw.rfind('-') {
                    o.fix(s, e, format!("`{raw}` is a range with a class escape at one end, which strict syntax forbids"), s + d, s + d + 1, "\\-");
                }
            }
        }
    }
}

fn unicode_escape(o: &mut Out<'_, '_>) {
    if !o.site.re.flags.unicode() {
        return;
    }
    for (c, s, e) in char_spots(&o.site.re) {
        let raw = o.site.t(s, e).to_owned();
        if raw.starts_with("\\u") && raw.len() == 6 && c > 0xFF {
            let new = format!("\\u{{{}}}", &raw[2..]);
            o.fix(s, e, format!("Write `{raw}` as `{new}`"), s, e, new);
        }
    }
}

const SCRIPTS: &[(&str, &str)] = &[
    ("Adlm", "Adlam"), ("Aghb", "Caucasian_Albanian"), ("Arab", "Arabic"), ("Armi", "Imperial_Aramaic"), ("Armn", "Armenian"), ("Avst", "Avestan"), ("Bali", "Balinese"), ("Bamu", "Bamum"),
    ("Bass", "Bassa_Vah"), ("Batk", "Batak"), ("Beng", "Bengali"), ("Bhks", "Bhaiksuki"), ("Bopo", "Bopomofo"), ("Brah", "Brahmi"), ("Brai", "Braille"), ("Bugi", "Buginese"), ("Buhd", "Buhid"),
    ("Cakm", "Chakma"), ("Cans", "Canadian_Aboriginal"), ("Cari", "Carian"), ("Cher", "Cherokee"), ("Chrs", "Chorasmian"), ("Copt", "Coptic"), ("Qaac", "Coptic"), ("Cpmn", "Cypro_Minoan"),
    ("Cprt", "Cypriot"), ("Cyrl", "Cyrillic"), ("Deva", "Devanagari"), ("Diak", "Dives_Akuru"), ("Dogr", "Dogra"), ("Dsrt", "Deseret"), ("Dupl", "Duployan"), ("Egyp", "Egyptian_Hieroglyphs"),
    ("Elba", "Elbasan"), ("Elym", "Elymaic"), ("Ethi", "Ethiopic"), ("Geor", "Georgian"), ("Glag", "Glagolitic"), ("Gong", "Gunjala_Gondi"), ("Gonm", "Masaram_Gondi"), ("Goth", "Gothic"),
    ("Gran", "Grantha"), ("Grek", "Greek"), ("Gujr", "Gujarati"), ("Guru", "Gurmukhi"), ("Hang", "Hangul"), ("Hani", "Han"), ("Hano", "Hanunoo"), ("Hatr", "Hatran"), ("Hebr", "Hebrew"),
    ("Hira", "Hiragana"), ("Hluw", "Anatolian_Hieroglyphs"), ("Hmng", "Pahawh_Hmong"), ("Hmnp", "Nyiakeng_Puachue_Hmong"), ("Hrkt", "Katakana_Or_Hiragana"), ("Hung", "Old_Hungarian"),
    ("Ital", "Old_Italic"), ("Java", "Javanese"), ("Kali", "Kayah_Li"), ("Kana", "Katakana"), ("Khar", "Kharoshthi"), ("Khmr", "Khmer"), ("Khoj", "Khojki"), ("Kits", "Khitan_Small_Script"),
    ("Knda", "Kannada"), ("Kthi", "Kaithi"), ("Lana", "Tai_Tham"), ("Laoo", "Lao"), ("Latn", "Latin"), ("Lepc", "Lepcha"), ("Limb", "Limbu"), ("Lina", "Linear_A"), ("Linb", "Linear_B"),
    ("Lyci", "Lycian"), ("Lydi", "Lydian"), ("Mahj", "Mahajani"), ("Maka", "Makasar"), ("Mand", "Mandaic"), ("Mani", "Manichaean"), ("Marc", "Marchen"), ("Medf", "Medefaidrin"),
    ("Mend", "Mende_Kikakui"), ("Merc", "Meroitic_Cursive"), ("Mero", "Meroitic_Hieroglyphs"), ("Mlym", "Malayalam"), ("Mong", "Mongolian"), ("Mroo", "Mro"), ("Mtei", "Meetei_Mayek"),
    ("Mult", "Multani"), ("Mymr", "Myanmar"), ("Nagm", "Nag_Mundari"), ("Nand", "Nandinagari"), ("Narb", "Old_North_Arabian"), ("Nbat", "Nabataean"), ("Nkoo", "Nko"), ("Nshu", "Nushu"),
    ("Ogam", "Ogham"), ("Olck", "Ol_Chiki"), ("Orkh", "Old_Turkic"), ("Orya", "Oriya"), ("Osge", "Osage"), ("Osma", "Osmanya"), ("Ougr", "Old_Uyghur"), ("Palm", "Palmyrene"),
    ("Pauc", "Pau_Cin_Hau"), ("Perm", "Old_Permic"), ("Phag", "Phags_Pa"), ("Phli", "Inscriptional_Pahlavi"), ("Phlp", "Psalter_Pahlavi"), ("Phnx", "Phoenician"), ("Plrd", "Miao"),
    ("Prti", "Inscriptional_Parthian"), ("Rjng", "Rejang"), ("Rohg", "Hanifi_Rohingya"), ("Runr", "Runic"), ("Samr", "Samaritan"), ("Sarb", "Old_South_Arabian"), ("Saur", "Saurashtra"),
    ("Sgnw", "SignWriting"), ("Shaw", "Shavian"), ("Shrd", "Sharada"), ("Sidd", "Siddham"), ("Sind", "Khudawadi"), ("Sinh", "Sinhala"), ("Sogd", "Sogdian"), ("Sogo", "Old_Sogdian"),
    ("Sora", "Sora_Sompeng"), ("Soyo", "Soyombo"), ("Sund", "Sundanese"), ("Sylo", "Syloti_Nagri"), ("Syrc", "Syriac"), ("Tagb", "Tagbanwa"), ("Takr", "Takri"), ("Tale", "Tai_Le"),
    ("Talu", "New_Tai_Lue"), ("Taml", "Tamil"), ("Tang", "Tangut"), ("Tavt", "Tai_Viet"), ("Telu", "Telugu"), ("Tfng", "Tifinagh"), ("Tglg", "Tagalog"), ("Thaa", "Thaana"), ("Tibt", "Tibetan"),
    ("Tirh", "Tirhuta"), ("Tnsa", "Tangsa"), ("Ugar", "Ugaritic"), ("Vaii", "Vai"), ("Vith", "Vithkuqi"), ("Wara", "Warang_Citi"), ("Wcho", "Wancho"), ("Xpeo", "Old_Persian"),
    ("Xsux", "Cuneiform"), ("Yezi", "Yezidi"), ("Yiii", "Yi"), ("Zanb", "Zanabazar_Square"), ("Zinh", "Inherited"), ("Qaai", "Inherited"), ("Zyyy", "Common"), ("Zzzz", "Unknown"),
];

fn unicode_property(o: &mut Out<'_, '_>) {
    let mut spots = Vec::new();
    for it in items_in(&o.site.re) {
        if let Kind::Set(ra::SetEsc { kind: EscKind::Prop(p), .. }) = &it.kind {
            spots.push((p.clone(), it.start, it.end));
        }
    }
    for c in classes_in(&o.site.re) {
        for i in &c.items {
            if let CKind::Set(ra::SetEsc { kind: EscKind::Prop(p), .. }) = &i.kind {
                spots.push((p.clone(), i.start, i.end));
            }
        }
    }
    for (p, s, e) in spots {
        let new = if let Some(v) = p.strip_prefix("General_Category=").or_else(|| p.strip_prefix("gc=")) {
            Some(v.to_owned())
        } else if let Some((k, v)) = p.split_once('=') {
            if matches!(k, "Script" | "sc" | "Script_Extensions" | "scx") { SCRIPTS.iter().find(|x| x.0 == v).map(|x| format!("{k}={}", x.1)) } else { None }
        } else {
            None
        };
        if let Some(new) = new {
            let raw = o.site.t(s, e).to_owned();
            let full = format!("{}{new}}}", &raw[..3]);
            o.fix(s, e, format!("Write `{raw}` as `{full}`"), s, e, full);
        }
    }
}

fn use_ignore_case(o: &mut Out<'_, '_>) {
    let f = o.site.re.flags.clone();
    if f.i || !o.site.literal || ra::has_backref(&o.site.re.alts) {
        return;
    }
    let is_upper = |s: &CharSet| s.0.iter().all(|&(a, b)| (0x41..=0x5A).contains(&a) && (0x41..=0x5A).contains(&b));
    let mut drop: Vec<(usize, usize)> = Vec::new();
    for it in items_in(&o.site.re) {
        match &it.kind {
            Kind::Char(c) if !ra::case_variants(*c).is_empty() => return,
            Kind::Set(ra::SetEsc { kind: EscKind::Prop(_), .. }) => return,
            Kind::Class(c) => {
                if c.op != ClassOp::Union {
                    return;
                }
                let mut upper = Vec::new();
                let mut rest = CharSet::empty();
                for i in &c.items {
                    let Some(s) = ra::citem_set(i, &f) else { return };
                    if matches!(i.kind, CKind::Char(_) | CKind::Range { .. }) && is_upper(&s) {
                        upper.push((i.start, i.end, s));
                    } else {
                        rest = rest.union(&s);
                    }
                }
                let all = upper.iter().fold(rest.clone(), |a, b| a.union(&b.2));
                if all.fold() != all {
                    return; // a letter in one case only
                }
                if !upper.is_empty() && rest.fold() == all {
                    drop.extend(upper.iter().map(|x| (x.0, x.1)));
                }
            }
            _ => {}
        }
    }
    if drop.is_empty() {
        return;
    }
    let mut new = String::new();
    let mut at = 0;
    for (s, e) in &drop {
        new.push_str(o.site.t(at, *s));
        at = *e;
    }
    new.push_str(&o.site.pattern[at..]);
    let flags = with_flag(&f.raw, 'i');
    let (s, e) = drop[0];
    let mut edits = Vec::new();
    if let Some((fs, fe)) = o.site.flags_span
        && ra::parse(&new, &flags).is_ok()
    {
        edits.push(TextEdit::new(o.site.base, o.site.base + o.site.pattern.len(), new));
        edits.push(TextEdit::new(fs, fe, flags));
    }
    o.out.push(NativeFinding { start: o.site.base + s, end: o.site.base + e, message: "Both cases of every letter are listed: use the `i` flag".into(), edits });
}

macro_rules! rules {
    ($($name:ident = $id:literal => $f:ident;)*) => {
        $(pub static $name: Rx = Rx { id: $id, f: $f };)*
    };
}

rules! {
    CONFUSING_QUANTIFIER = "js/regexp-confusing-quantifier" => confusing_quantifier;
    CONTROL_CHARACTER_ESCAPE = "js/regexp-control-character-escape" => control_character_escape;
    GRAPHEME_STRING_LITERAL = "js/regexp-grapheme-string-literal" => grapheme_string_literal;
    HEXADECIMAL_ESCAPE = "js/regexp-hexadecimal-escape" => hexadecimal_escape;
    MATCH_ANY = "js/regexp-match-any" => match_any;
    NEGATION = "js/regexp-negation" => negation;
    NO_CONTRADICTION_WITH_ASSERTION = "js/regexp-no-contradiction-with-assertion" => no_contradiction_with_assertion;
    NO_DUPE_CHARACTERS_CHARACTER_CLASS = "js/regexp-no-dupe-characters-character-class" => no_dupe_characters_character_class;
    NO_DUPE_DISJUNCTIONS = "js/regexp-no-dupe-disjunctions" => no_dupe_disjunctions;
    NO_EMPTY_ALTERNATIVE = "js/regexp-no-empty-alternative" => no_empty_alternative;
    NO_EMPTY_CAPTURING_GROUP = "js/regexp-no-empty-capturing-group" => no_empty_capturing_group;
    NO_EMPTY_CHARACTER_CLASS = "js/regexp-no-empty-character-class" => no_empty_character_class;
    NO_EMPTY_GROUP = "js/regexp-no-empty-group" => no_empty_group;
    NO_EMPTY_LOOKAROUNDS_ASSERTION = "js/regexp-no-empty-lookarounds-assertion" => no_empty_lookarounds_assertion;
    NO_EMPTY_STRING_LITERAL = "js/regexp-no-empty-string-literal" => no_empty_string_literal;
    NO_ESCAPE_BACKSPACE = "js/regexp-no-escape-backspace" => no_escape_backspace;
    NO_EXTRA_LOOKAROUND_ASSERTIONS = "js/regexp-no-extra-lookaround-assertions" => no_extra_lookaround_assertions;
    NO_INVISIBLE_CHARACTER = "js/regexp-no-invisible-character" => no_invisible_character;
    NO_LAZY_ENDS = "js/regexp-no-lazy-ends" => no_lazy_ends;
    NO_MISLEADING_CAPTURING_GROUP = "js/regexp-no-misleading-capturing-group" => no_misleading_capturing_group;
    NO_MISLEADING_UNICODE_CHARACTER = "js/regexp-no-misleading-unicode-character" => no_misleading_unicode_character;
    NO_NON_STANDARD_FLAG = "js/regexp-no-non-standard-flag" => no_non_standard_flag;
    NO_OBSCURE_RANGE = "js/regexp-no-obscure-range" => no_obscure_range;
    NO_OCTAL = "js/regexp-no-octal" => no_octal;
    NO_OPTIONAL_ASSERTION = "js/regexp-no-optional-assertion" => no_optional_assertion;
    NO_POTENTIALLY_USELESS_BACKREFERENCE = "js/regexp-no-potentially-useless-backreference" => no_potentially_useless_backreference;
    NO_STANDALONE_BACKSLASH = "js/regexp-no-standalone-backslash" => no_standalone_backslash;
    NO_SUPER_LINEAR_BACKTRACKING = "js/regexp-no-super-linear-backtracking" => no_super_linear_backtracking;
    NO_SUPER_LINEAR_MOVE = "js/regexp-no-super-linear-move" => no_super_linear_move;
    NO_TRIVIALLY_NESTED_ASSERTION = "js/regexp-no-trivially-nested-assertion" => no_trivially_nested_assertion;
    NO_TRIVIALLY_NESTED_QUANTIFIER = "js/regexp-no-trivially-nested-quantifier" => no_trivially_nested_quantifier;
    NO_USELESS_ASSERTIONS = "js/regexp-no-useless-assertions" => no_useless_assertions;
    NO_USELESS_CHARACTER_CLASS = "js/regexp-no-useless-character-class" => no_useless_character_class;
    NO_USELESS_ESCAPE = "js/regexp-no-useless-escape" => no_useless_escape;
    NO_USELESS_FLAG = "js/regexp-no-useless-flag" => no_useless_flag;
    NO_USELESS_LAZY = "js/regexp-no-useless-lazy" => no_useless_lazy;
    NO_USELESS_NON_CAPTURING_GROUP = "js/regexp-no-useless-non-capturing-group" => no_useless_non_capturing_group;
    NO_USELESS_QUANTIFIER = "js/regexp-no-useless-quantifier" => no_useless_quantifier;
    NO_USELESS_RANGE = "js/regexp-no-useless-range" => no_useless_range;
    NO_USELESS_SET_OPERAND = "js/regexp-no-useless-set-operand" => no_useless_set_operand;
    NO_USELESS_STRING_LITERAL = "js/regexp-no-useless-string-literal" => no_useless_string_literal;
    NO_USELESS_TWO_NUMS_QUANTIFIER = "js/regexp-no-useless-two-nums-quantifier" => no_useless_two_nums_quantifier;
    NO_ZERO_QUANTIFIER = "js/regexp-no-zero-quantifier" => no_zero_quantifier;
    OPTIMAL_LOOKAROUND_QUANTIFIER = "js/regexp-optimal-lookaround-quantifier" => optimal_lookaround_quantifier;
    OPTIMAL_QUANTIFIER_CONCATENATION = "js/regexp-optimal-quantifier-concatenation" => optimal_quantifier_concatenation;
    PREFER_CHARACTER_CLASS = "js/regexp-prefer-character-class" => prefer_character_class;
    PREFER_D = "js/regexp-prefer-d" => prefer_d;
    PREFER_NAMED_BACKREFERENCE = "js/regexp-prefer-named-backreference" => prefer_named_backreference;
    PREFER_NAMED_CAPTURE_GROUP = "js/regexp-prefer-named-capture-group" => prefer_named_capture_group;
    PREFER_PLUS_QUANTIFIER = "js/regexp-prefer-plus-quantifier" => prefer_plus_quantifier;
    PREFER_PREDEFINED_ASSERTION = "js/regexp-prefer-predefined-assertion" => prefer_predefined_assertion;
    PREFER_QUANTIFIER = "js/regexp-prefer-quantifier" => prefer_quantifier;
    PREFER_QUESTION_QUANTIFIER = "js/regexp-prefer-question-quantifier" => prefer_question_quantifier;
    PREFER_RANGE = "js/regexp-prefer-range" => prefer_range;
    PREFER_SET_OPERATION = "js/regexp-prefer-set-operation" => prefer_set_operation;
    PREFER_STAR_QUANTIFIER = "js/regexp-prefer-star-quantifier" => prefer_star_quantifier;
    PREFER_UNICODE_CODEPOINT_ESCAPES = "js/regexp-prefer-unicode-codepoint-escapes" => prefer_unicode_codepoint_escapes;
    PREFER_W = "js/regexp-prefer-w" => prefer_w;
    REQUIRE_UNICODE_REGEXP = "js/regexp-require-unicode-regexp" => require_unicode_regexp;
    REQUIRE_UNICODE_SETS_REGEXP = "js/regexp-require-unicode-sets-regexp" => require_unicode_sets_regexp;
    SIMPLIFY_SET_OPERATIONS = "js/regexp-simplify-set-operations" => simplify_set_operations;
    SORT_ALTERNATIVES = "js/regexp-sort-alternatives" => sort_alternatives;
    SORT_FLAGS = "js/regexp-sort-flags" => sort_flags;
    STRICT = "js/regexp-strict" => strict;
    UNICODE_ESCAPE = "js/regexp-unicode-escape" => unicode_escape;
    UNICODE_PROPERTY = "js/regexp-unicode-property" => unicode_property;
    USE_IGNORE_CASE = "js/regexp-use-ignore-case" => use_ignore_case;
}

pub static NO_LEGACY_FEATURES: Call = Call { id: "js/regexp-no-legacy-features", f: no_legacy_features, langs: JS, kinds: &["member_expression", "subscript_expression"] };
pub static NO_MISSING_G_FLAG: Call = call("js/regexp-no-missing-g-flag", no_missing_g_flag);
pub static NO_UNUSED_CAPTURING_GROUP: Call = call("js/regexp-no-unused-capturing-group", no_unused_capturing_group);
pub static NO_USELESS_DOLLAR_REPLACEMENTS: Call = call("js/regexp-no-useless-dollar-replacements", no_useless_dollar_replacements);
pub static PREFER_ESCAPE_REPLACEMENT_DOLLAR_CHAR: Call = call("js/regexp-prefer-escape-replacement-dollar-char", prefer_escape_replacement_dollar_char);
pub static PREFER_LOOKAROUND: Call = call("js/regexp-prefer-lookaround", prefer_lookaround);
pub static PREFER_NAMED_REPLACEMENT: Call = call("js/regexp-prefer-named-replacement", prefer_named_replacement);
pub static PREFER_REGEXP_EXEC: Call = call("js/regexp-prefer-regexp-exec", prefer_regexp_exec);
pub static PREFER_REGEXP_TEST: Call = call("js/regexp-prefer-regexp-test", prefer_regexp_test);
pub static PREFER_RESULT_ARRAY_GROUPS: Call = Call { id: "js/regexp-prefer-result-array-groups", f: prefer_result_array_groups, langs: JS_ONLY, kinds: &["variable_declarator"] };

pub static RULES: &[&dyn NativeRule] = &[
    &CONFUSING_QUANTIFIER, &CONTROL_CHARACTER_ESCAPE, &GRAPHEME_STRING_LITERAL, &HEXADECIMAL_ESCAPE, &MATCH_ANY, &NEGATION, &NO_CONTRADICTION_WITH_ASSERTION,
    &NO_DUPE_CHARACTERS_CHARACTER_CLASS, &NO_DUPE_DISJUNCTIONS, &NO_EMPTY_ALTERNATIVE, &NO_EMPTY_CAPTURING_GROUP, &NO_EMPTY_CHARACTER_CLASS, &NO_EMPTY_GROUP,
    &NO_EMPTY_LOOKAROUNDS_ASSERTION, &NO_EMPTY_STRING_LITERAL, &NO_ESCAPE_BACKSPACE, &NO_EXTRA_LOOKAROUND_ASSERTIONS, &NO_INVISIBLE_CHARACTER, &NO_LAZY_ENDS,
    &NO_LEGACY_FEATURES, &NO_MISLEADING_CAPTURING_GROUP, &NO_MISLEADING_UNICODE_CHARACTER, &NO_MISSING_G_FLAG, &NO_NON_STANDARD_FLAG, &NO_OBSCURE_RANGE, &NO_OCTAL,
    &NO_OPTIONAL_ASSERTION, &NO_POTENTIALLY_USELESS_BACKREFERENCE, &NO_STANDALONE_BACKSLASH, &NO_SUPER_LINEAR_BACKTRACKING, &NO_SUPER_LINEAR_MOVE,
    &NO_TRIVIALLY_NESTED_ASSERTION, &NO_TRIVIALLY_NESTED_QUANTIFIER, &NO_UNUSED_CAPTURING_GROUP, &NO_USELESS_ASSERTIONS, &NO_USELESS_CHARACTER_CLASS,
    &NO_USELESS_DOLLAR_REPLACEMENTS, &NO_USELESS_ESCAPE, &NO_USELESS_FLAG, &NO_USELESS_LAZY, &NO_USELESS_NON_CAPTURING_GROUP, &NO_USELESS_QUANTIFIER,
    &NO_USELESS_RANGE, &NO_USELESS_SET_OPERAND, &NO_USELESS_STRING_LITERAL, &NO_USELESS_TWO_NUMS_QUANTIFIER, &NO_ZERO_QUANTIFIER, &OPTIMAL_LOOKAROUND_QUANTIFIER,
    &OPTIMAL_QUANTIFIER_CONCATENATION, &PREFER_CHARACTER_CLASS, &PREFER_D, &PREFER_ESCAPE_REPLACEMENT_DOLLAR_CHAR, &PREFER_LOOKAROUND, &PREFER_NAMED_BACKREFERENCE,
    &PREFER_NAMED_CAPTURE_GROUP, &PREFER_NAMED_REPLACEMENT, &PREFER_PLUS_QUANTIFIER, &PREFER_PREDEFINED_ASSERTION, &PREFER_QUANTIFIER, &PREFER_QUESTION_QUANTIFIER,
    &PREFER_RANGE, &PREFER_REGEXP_EXEC, &PREFER_REGEXP_TEST, &PREFER_RESULT_ARRAY_GROUPS, &PREFER_SET_OPERATION, &PREFER_STAR_QUANTIFIER,
    &PREFER_UNICODE_CODEPOINT_ESCAPES, &PREFER_W, &REQUIRE_UNICODE_REGEXP, &REQUIRE_UNICODE_SETS_REGEXP, &SIMPLIFY_SET_OPERATIONS, &SORT_ALTERNATIVES, &SORT_FLAGS,
    &STRICT, &UNICODE_ESCAPE, &UNICODE_PROPERTY, &USE_IGNORE_CASE,
];

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(rule: &dyn NativeRule, src: &str) -> Vec<String> {
        crate::run(rule, Language::Javascript, src)
            .into_iter()
            .map(|f| {
                let mut s = src.to_owned();
                let mut e = f.edits.clone();
                e.sort_by_key(|x| std::cmp::Reverse(x.start));
                for x in e {
                    s.replace_range(x.start..x.end, &x.text);
                }
                s
            })
            .collect()
    }

    #[test]
    fn a_sample_of_fixes() {
        assert_eq!(fixed(&PREFER_D, "/[0-9]/"), ["/\\d/"]);
        assert_eq!(fixed(&PREFER_W, "/[a-zA-Z0-9_]/"), ["/\\w/"]);
        assert_eq!(fixed(&NEGATION, "/[^\\d]/"), ["/\\D/"]);
        assert_eq!(fixed(&SORT_FLAGS, "/a/mig"), ["/a/gim"]);
        assert_eq!(fixed(&NO_USELESS_NON_CAPTURING_GROUP, "/(?:ab)c/"), ["/abc/"]);
        assert_eq!(fixed(&NO_USELESS_NON_CAPTURING_GROUP, "/\\1(?:2)(a)/"), ["/\\1(?:2)(a)/"]);
        assert_eq!(fixed(&NO_TRIVIALLY_NESTED_QUANTIFIER, "/(?:a+)?/"), ["/a*/"]);
        assert_eq!(fixed(&OPTIMAL_QUANTIFIER_CONCATENATION, "/\\w+\\d*/"), ["/\\w+/"]);
        assert_eq!(fixed(&PREFER_CHARACTER_CLASS, "/(?:a|b|c)/"), ["/[abc]/"]);
        assert_eq!(fixed(&PREFER_RANGE, "/[abcd]/"), ["/[a-d]/"]);
        assert_eq!(fixed(&NO_DUPE_CHARACTERS_CHARACTER_CLASS, "/[aab]/"), ["/[ab]/"]);
        assert_eq!(fixed(&NO_MISSING_G_FLAG, "s.replaceAll(/a/, 'b')"), ["s.replaceAll(/a/g, 'b')"]);
        assert_eq!(fixed(&PREFER_PREDEFINED_ASSERTION, "/a(?!\\w)/"), ["/a\\b/"]);
        assert_eq!(fixed(&USE_IGNORE_CASE, "/[a-zA-Z]/"), ["/[a-z]/i"]);
        assert_eq!(fixed(&NO_UNUSED_CAPTURING_GROUP, "/(a)b/.test(s)"), ["/(?:a)b/.test(s)"]);
        assert_eq!(fixed(&STRICT, "/a]/"), ["/a\\]/"]);
        assert_eq!(fixed(&MATCH_ANY, "/[\\d\\D]/"), ["/[\\s\\S]/"]);
        assert_eq!(fixed(&PREFER_REGEXP_TEST, "if (/a/.exec(s)) {}"), ["if (/a/.test(s)) {}"]);
        assert_eq!(fixed(&PREFER_RESULT_ARRAY_GROUPS, "const m = /(?<y>\\d)/.exec(s); f(m[1]);"), ["const m = /(?<y>\\d)/.exec(s); f(m.groups.y);"]);
        assert_eq!(fixed(&PREFER_LOOKAROUND, "s.replace(/(a)b/g, '$1c')"), ["s.replace(/(?<=a)b/g, 'c')"]);
    }

    #[test]
    fn clean_patterns_pass() {
        let src = r"const a = /^[\w.+-]+@[\w-]+\.[\w.-]+$/u; const b = /(?<year>\d{4})-(?<month>\d{2})/v; const c = /\s+/g;";
        let ignore = ["js/regexp-prefer-named-capture-group", "js/regexp-require-unicode-sets-regexp", "js/regexp-require-unicode-regexp", "js/regexp-no-super-linear-move", "js/regexp-unicode-escape"];
        for r in RULES {
            if ignore.contains(&r.id()) {
                continue;
            }
            assert!(crate::run(*r, Language::Javascript, src).is_empty(), "{} fired", r.id());
        }
    }
}

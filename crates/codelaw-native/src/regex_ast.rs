//! A parser for JavaScript regular expressions, written for codelaw (no
//! dependency, L11): the syntax of ECMAScript 2025 in its three modes —
//! the legacy one with Annex B's leniencies, `u` and `v` (class set
//! expressions, `\q{…}` string literals) — into a tree whose every node
//! keeps its byte span in the pattern, so a rule can report and edit any
//! part. [`CharSet`] gives the characters a single-character element
//! matches, for the rules that compare them.
//!
//! Unicode property escapes (`\p{…}`) are kept by name: the engine carries
//! no Unicode property tables, so a set holding one is unknown ([`None`]).

use std::fmt::Write as _;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub raw: String,
    pub d: bool,
    pub g: bool,
    pub i: bool,
    pub m: bool,
    pub s: bool,
    pub u: bool,
    pub v: bool,
    pub y: bool,
}

impl Flags {
    pub fn parse(raw: &str) -> Flags {
        let has = |c| raw.contains(c);
        Flags { raw: raw.to_owned(), d: has('d'), g: has('g'), i: has('i'), m: has('m'), s: has('s'), u: has('u'), v: has('v'), y: has('y') }
    }
    pub fn unicode(&self) -> bool {
        self.u || self.v
    }
    /// The largest character the pattern can match: a code point with `u`
    /// or `v`, a UTF-16 code unit without.
    pub fn max(&self) -> u32 {
        if self.unicode() { 0x10FFFF } else { 0xFFFF }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EscKind {
    Digit,
    Word,
    Space,
    /// `\p{…}`: what is inside the braces.
    Prop(String),
}

/// `\d \D \w \W \s \S \p{…} \P{…}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetEsc {
    pub kind: EscKind,
    pub negate: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ref {
    Num(u32),
    Name(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassOp {
    Union,
    Intersection,
    Subtraction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub negate: bool,
    pub op: ClassOp,
    pub items: Vec<CItem>,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CKind {
    Char(u32),
    /// A range; the spans of its two ends.
    Range { lo: u32, hi: u32, lo_span: (usize, usize), hi_span: (usize, usize) },
    Set(SetEsc),
    /// `\b` inside a class: the backspace character, written as an escape.
    /// Kept as `Char(8)`; this marks nothing extra.
    Class(Class),
    /// `\q{a|bc}` (the `v` flag): its strings.
    Strings(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CItem {
    pub kind: CKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Char(u32),
    Dot,
    Set(SetEsc),
    Class(Class),
    /// `^`
    Start,
    /// `$`
    End,
    /// `\b` (`negate`: `\B`).
    Boundary { negate: bool },
    Look { behind: bool, negate: bool, alts: Vec<Alt> },
    /// `capture` is the group's number; `None` for `(?:…)` (and the
    /// modifier groups `(?i:…)`).
    Group { capture: Option<u32>, name: Option<String>, alts: Vec<Alt> },
    /// `op` is the span of the quantifier itself (`+?`, `{2,3}`).
    Quant { min: u32, max: Option<u32>, greedy: bool, body: Box<Item>, op: (usize, usize) },
    Backref(Ref),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alt {
    pub items: Vec<Item>,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct Regex {
    pub alts: Vec<Alt>,
    /// The number of capturing groups.
    pub groups: u32,
    /// Each capturing group's name, by number - 1.
    pub names: Vec<Option<String>>,
    pub flags: Flags,
    /// Places only Annex B allows (the legacy mode's leniencies): their
    /// span and what they are. Empty with `u` or `v`.
    pub legacy: Vec<(usize, usize, Legacy)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Legacy {
    /// A `]`, `{` or `}` that stands for itself.
    LoneBracket,
    /// `\a`-style: a backslash before a character that needs none.
    IdentityEscape,
    /// `\07`, `\1` with no group 1.
    Octal,
    /// `\c` not followed by a letter: a backslash that stands for itself.
    LoneBackslash,
    /// A quantifier on a lookahead.
    QuantifiedLookahead,
    /// A range with a class escape at one end, `[\d-z]`.
    ClassEscapeRange,
}

const SYNTAX: &str = "^$\\.*+?()[]{}|/";

struct P<'a> {
    s: &'a str,
    i: usize,
    flags: Flags,
    groups: u32,
    total_groups: u32,
    named: bool,
    names: Vec<Option<String>>,
    legacy: Vec<(usize, usize, Legacy)>,
}

type R<T> = Result<T, String>;

pub fn parse(pattern: &str, flags: &str) -> Result<Regex, String> {
    let flags = Flags::parse(flags);
    let (total, named) = count_groups(pattern);
    let mut p = P { s: pattern, i: 0, flags, groups: 0, total_groups: total, named, names: Vec::new(), legacy: Vec::new() };
    let alts = p.disjunction()?;
    if p.i < p.s.len() {
        return Err(format!("unexpected `{}`", p.peek().unwrap_or(' ')));
    }
    let mut re = Regex { alts, groups: p.groups, names: p.names, flags: p.flags, legacy: p.legacy };
    resolve_check(&mut re)?;
    Ok(re)
}

/// Capturing groups, counted before parsing (a backreference may come
/// before its group), and whether any is named.
fn count_groups(p: &str) -> (u32, bool) {
    let b = p.as_bytes();
    let (mut i, mut n, mut named, mut class) = (0, 0, false, 0);
    while i < b.len() {
        match b[i] {
            b'\\' => i += 1,
            b'[' => class += 1,
            b']' if class > 0 => class -= 1,
            b'(' if class == 0 => {
                if b.get(i + 1) != Some(&b'?') {
                    n += 1;
                } else if b.get(i + 2) == Some(&b'<') && !matches!(b.get(i + 3), Some(b'=') | Some(b'!')) {
                    n += 1;
                    named = true;
                }
            }
            _ => {}
        }
        i += 1;
    }
    (n, named)
}

fn resolve_check(re: &mut Regex) -> R<()> {
    // Every `\k<name>` names a group, when any group is named or in u/v mode.
    let mut missing = None;
    let names = re.names.clone();
    walk(&re.alts, &mut |it| {
        if let Kind::Backref(Ref::Name(n)) = &it.kind
            && !names.iter().any(|x| x.as_deref() == Some(n.as_str()))
        {
            missing.get_or_insert(n.clone());
        }
    });
    if let Some(n) = missing {
        return Err(format!("no group is named `{n}`"));
    }
    Ok(())
}

impl<'a> P<'a> {
    fn peek(&self) -> Option<char> {
        self.s[self.i..].chars().next()
    }
    fn peek_at(&self, k: usize) -> Option<char> {
        self.s[self.i..].chars().nth(k)
    }
    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.i += c.len_utf8();
            true
        } else {
            false
        }
    }
    fn starts(&self, t: &str) -> bool {
        self.s[self.i..].starts_with(t)
    }
    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.i += c.len_utf8();
        Some(c)
    }
    fn unicode(&self) -> bool {
        self.flags.unicode()
    }

    fn disjunction(&mut self) -> R<Vec<Alt>> {
        let mut alts = vec![self.alternative()?];
        while self.eat('|') {
            alts.push(self.alternative()?);
        }
        Ok(alts)
    }

    fn alternative(&mut self) -> R<Alt> {
        let start = self.i;
        let mut items: Vec<Item> = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            let atom = self.term()?;
            items.push(atom);
        }
        Ok(Alt { items, start, end: self.i })
    }

    fn term(&mut self) -> R<Item> {
        let start = self.i;
        let c = self.peek().unwrap_or('\0');
        let kind = match c {
            '^' => {
                self.i += 1;
                Kind::Start
            }
            '$' => {
                self.i += 1;
                Kind::End
            }
            '.' => {
                self.i += 1;
                Kind::Dot
            }
            '(' => self.group()?,
            '[' => Kind::Class(self.class()?),
            '\\' => self.atom_escape()?,
            '*' | '+' | '?' => return Err(format!("nothing to repeat before `{c}`")),
            '{' => {
                if self.unicode() {
                    return Err("a lone `{` is not allowed with the `u` or `v` flag".into());
                }
                if self.quantifier_ahead() {
                    return Err("nothing to repeat before `{`".into());
                }
                self.i += 1;
                self.legacy.push((start, self.i, Legacy::LoneBracket));
                Kind::Char('{' as u32)
            }
            '}' | ']' => {
                if self.unicode() {
                    return Err(format!("a lone `{c}` is not allowed with the `u` or `v` flag"));
                }
                self.i += 1;
                self.legacy.push((start, self.i, Legacy::LoneBracket));
                Kind::Char(c as u32)
            }
            _ => {
                self.i += c.len_utf8();
                // A surrogate pair is one character only in u/v mode; Rust
                // strings hold code points, so nothing to join here.
                Kind::Char(c as u32)
            }
        };
        let atom = Item { kind, start, end: self.i };
        self.quantified(atom)
    }

    fn quantifier_ahead(&self) -> bool {
        let r = &self.s[self.i..];
        let Some(body) = r.strip_prefix('{') else { return false };
        let Some(close) = body.find('}') else { return false };
        let q = &body[..close];
        let mut parts = q.splitn(2, ',');
        let a = parts.next().unwrap_or("");
        let b = parts.next();
        !a.is_empty() && a.bytes().all(|d| d.is_ascii_digit()) && b.is_none_or(|b| b.bytes().all(|d| d.is_ascii_digit()))
    }

    fn quantified(&mut self, atom: Item) -> R<Item> {
        let qstart = self.i;
        let (min, max) = match self.peek() {
            Some('*') => {
                self.i += 1;
                (0, None)
            }
            Some('+') => {
                self.i += 1;
                (1, None)
            }
            Some('?') => {
                self.i += 1;
                (0, Some(1))
            }
            Some('{') if self.quantifier_ahead() => {
                let close = self.s[self.i..].find('}').unwrap() + self.i;
                let q = &self.s[self.i + 1..close];
                let mut parts = q.splitn(2, ',');
                let lo: u32 = parts.next().unwrap().parse().unwrap_or(u32::MAX);
                let hi = match parts.next() {
                    None => Some(lo),
                    Some("") => None,
                    Some(h) => Some(h.parse().unwrap_or(u32::MAX)),
                };
                if hi.is_some_and(|h| h < lo) {
                    return Err(format!("the quantifier `{{{q}}}` has its numbers out of order"));
                }
                self.i = close + 1;
                (lo, hi)
            }
            _ => return Ok(atom),
        };
        let greedy = !self.eat('?');
        match &atom.kind {
            Kind::Start | Kind::End | Kind::Boundary { .. } => return Err("nothing to repeat: an assertion can't be quantified".into()),
            Kind::Look { behind: true, .. } => return Err("nothing to repeat: a lookbehind can't be quantified".into()),
            Kind::Look { behind: false, .. } => {
                if self.unicode() {
                    return Err("nothing to repeat: a lookahead can't be quantified with the `u` or `v` flag".into());
                }
                self.legacy.push((atom.start, self.i, Legacy::QuantifiedLookahead));
            }
            _ => {}
        }
        let start = atom.start;
        let item = Item { kind: Kind::Quant { min, max, greedy, body: Box::new(atom), op: (qstart, self.i) }, start, end: self.i };
        if matches!(self.peek(), Some('*' | '+' | '?')) || (self.peek() == Some('{') && self.quantifier_ahead()) {
            return Err("nothing to repeat: a quantifier follows a quantifier".into());
        }
        Ok(item)
    }

    fn group(&mut self) -> R<Kind> {
        self.i += 1; // (
        
        let kind = if self.starts("?:") {
            self.i += 2;
            let alts = self.disjunction()?;
            Kind::Group { capture: None, name: None, alts }
        } else if self.starts("?=") || self.starts("?!") {
            let negate = self.starts("?!");
            self.i += 2;
            let alts = self.disjunction()?;
            Kind::Look { behind: false, negate, alts }
        } else if self.starts("?<=") || self.starts("?<!") {
            let negate = self.starts("?<!");
            self.i += 3;
            let alts = self.disjunction()?;
            Kind::Look { behind: true, negate, alts }
        } else if self.starts("?<") {
            self.i += 2;
            let close = self.s[self.i..].find('>').ok_or("`(?<` has no closing `>`")? + self.i;
            let name = self.s[self.i..close].to_owned();
            if !valid_name(&name) {
                return Err(format!("`{name}` is not a valid group name"));
            }
            if self.names.iter().any(|n| n.as_deref() == Some(name.as_str())) {
                return Err(format!("the group name `{name}` is used twice"));
            }
            self.i = close + 1;
            self.groups += 1;
            let num = self.groups;
            self.names.push(Some(name.clone()));
            let alts = self.disjunction()?;
            Kind::Group { capture: Some(num), name: Some(name), alts }
        } else if self.starts("?") {
            // Modifiers: `(?i:…)`, `(?-i:…)`, `(?i-m:…)` (ES2025).
            let r = &self.s[self.i + 1..];
            let len = r.find(':').filter(|n| r[..*n].chars().all(|c| "ims-".contains(c)) && *n > 0);
            let Some(len) = len else {
                return Err("an invalid group: `(?` must be followed by `:`, `=`, `!` or `<`".into());
            };
            self.i += 2 + len;
            let alts = self.disjunction()?;
            Kind::Group { capture: None, name: None, alts }
        } else {
            self.groups += 1;
            let num = self.groups;
            self.names.push(None);
            let alts = self.disjunction()?;
            Kind::Group { capture: Some(num), name: None, alts }
        };
        if !self.eat(')') {
            return Err("a group `(` is never closed".into());
        }
        Ok(kind)
    }

    fn atom_escape(&mut self) -> R<Kind> {
        let start = self.i;
        self.i += 1; // backslash
        let Some(c) = self.peek() else { return Err("the pattern ends with a lone `\\`".into()) };
        match c {
            'b' => {
                self.i += 1;
                Ok(Kind::Boundary { negate: false })
            }
            'B' => {
                self.i += 1;
                Ok(Kind::Boundary { negate: true })
            }
            '1'..='9' => {
                let len = self.s[self.i..].bytes().take_while(|d| d.is_ascii_digit()).count();
                let n: u32 = self.s[self.i..self.i + len].parse().unwrap_or(u32::MAX);
                if n <= self.total_groups {
                    self.i += len;
                    return Ok(Kind::Backref(Ref::Num(n)));
                }
                if self.unicode() {
                    return Err(format!("`\\{n}` refers to a group that doesn't exist"));
                }
                // Annex B: an octal escape, or `\8` / `\9` as themselves.
                if c == '8' || c == '9' {
                    self.i += 1;
                    self.legacy.push((start, self.i, Legacy::IdentityEscape));
                    return Ok(Kind::Char(c as u32));
                }
                let v = self.octal();
                self.legacy.push((start, self.i, Legacy::Octal));
                Ok(Kind::Char(v))
            }
            'k' => {
                if self.unicode() || self.named {
                    if !self.starts("k<") {
                        return Err("`\\k` must be followed by `<name>`".into());
                    }
                    let close = self.s[self.i..].find('>').ok_or("`\\k<` has no closing `>`")? + self.i;
                    let name = self.s[self.i + 2..close].to_owned();
                    self.i = close + 1;
                    return Ok(Kind::Backref(Ref::Name(name)));
                }
                self.i += 1;
                self.legacy.push((start, self.i, Legacy::IdentityEscape));
                Ok(Kind::Char('k' as u32))
            }
            _ => {
                if let Some(e) = self.set_escape()? {
                    return Ok(Kind::Set(e));
                }
                let v = self.char_escape(start, false)?;
                Ok(Kind::Char(v))
            }
        }
    }

    /// After the backslash: `\d \w \s \p{…}` and their negations.
    fn set_escape(&mut self) -> R<Option<SetEsc>> {
        let c = self.peek().unwrap_or('\0');
        let (kind, negate) = match c {
            'd' | 'D' => (EscKind::Digit, c == 'D'),
            'w' | 'W' => (EscKind::Word, c == 'W'),
            's' | 'S' => (EscKind::Space, c == 'S'),
            'p' | 'P' if self.unicode() => {
                if self.peek_at(1) != Some('{') {
                    return Err(format!("`\\{c}` must be followed by `{{property}}`"));
                }
                let close = self.s[self.i..].find('}').ok_or("`\\p{` has no closing `}`")? + self.i;
                let name = self.s[self.i + 2..close].to_owned();
                if name.is_empty() || !name.chars().all(|x| x.is_ascii_alphanumeric() || x == '_' || x == '=') {
                    return Err(format!("`{name}` is not a valid property"));
                }
                self.i = close + 1;
                return Ok(Some(SetEsc { kind: EscKind::Prop(name), negate: c == 'P' }));
            }
            _ => return Ok(None),
        };
        self.i += 1;
        Ok(Some(SetEsc { kind, negate }))
    }

    fn octal(&mut self) -> u32 {
        // Up to three octal digits, value ≤ 0o377.
        let mut v = 0u32;
        let mut n = 0;
        while n < 3 {
            let Some(d) = self.peek().and_then(|c| c.to_digit(8)) else { break };
            if v * 8 + d > 0o377 {
                break;
            }
            v = v * 8 + d;
            self.i += 1;
            n += 1;
        }
        v
    }

    fn hex(&mut self, n: usize) -> Option<u32> {
        let t = self.s.get(self.i..self.i + n)?;
        if !t.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        self.i += n;
        u32::from_str_radix(t, 16).ok()
    }

    /// A character escape after the backslash (`self.i` at the character
    /// after it); `start` is the backslash. `in_class` allows `\-` and the
    /// class-only forms.
    fn char_escape(&mut self, start: usize, in_class: bool) -> R<u32> {
        let c = self.next().ok_or("the pattern ends with a lone `\\`")?;
        let v = match c {
            't' => 9,
            'n' => 10,
            'v' => 11,
            'f' => 12,
            'r' => 13,
            '0' if !self.peek().is_some_and(|d| d.is_ascii_digit()) => 0,
            '0'..='7' if !self.unicode() => {
                self.i -= 1;
                let v = self.octal();
                self.legacy.push((start, self.i, Legacy::Octal));
                v
            }
            'c' => match self.peek() {
                Some(l) if l.is_ascii_alphabetic() || (in_class && !self.unicode() && (l.is_ascii_digit() || l == '_')) => {
                    self.i += 1;
                    (l as u32) % 32
                }
                _ => {
                    if self.unicode() {
                        return Err("`\\c` must be followed by a letter".into());
                    }
                    // The backslash stands for itself; `c` is read next.
                    self.i -= 1;
                    self.legacy.push((start, start + 1, Legacy::LoneBackslash));
                    '\\' as u32
                }
            },
            'x' => match self.hex(2) {
                Some(v) => v,
                None if self.unicode() => return Err("`\\x` must be followed by two hex digits".into()),
                None => {
                    self.legacy.push((start, self.i, Legacy::IdentityEscape));
                    'x' as u32
                }
            },
            'u' => {
                if self.unicode() && self.peek() == Some('{') {
                    let close = self.s[self.i..].find('}').ok_or("`\\u{` has no closing `}`")? + self.i;
                    let h = &self.s[self.i + 1..close];
                    let v = u32::from_str_radix(h, 16).ok().filter(|v| *v <= 0x10FFFF && !h.is_empty()).ok_or("`\\u{…}` holds no valid code point")?;
                    self.i = close + 1;
                    v
                } else if let Some(v) = self.hex(4) {
                    // A surrogate pair written as two escapes is one code
                    // point in u/v mode.
                    if self.unicode() && (0xD800..0xDC00).contains(&v) && self.starts("\\u") {
                        let save = self.i;
                        self.i += 2;
                        match self.hex(4) {
                            Some(lo) if (0xDC00..0xE000).contains(&lo) => 0x10000 + ((v - 0xD800) << 10) + (lo - 0xDC00),
                            _ => {
                                self.i = save;
                                v
                            }
                        }
                    } else {
                        v
                    }
                } else if self.unicode() {
                    return Err("`\\u` must be followed by four hex digits or `{…}`".into());
                } else {
                    self.legacy.push((start, self.i, Legacy::IdentityEscape));
                    'u' as u32
                }
            }
            '-' if in_class => '-' as u32,
            c if SYNTAX.contains(c) => c as u32,
            c => {
                if self.unicode() {
                    if in_class && self.flags.v && "&-!#%,:;<=>@`~".contains(c) {
                        return Ok(c as u32);
                    }
                    return Err(format!("`\\{c}` is not a valid escape with the `u` or `v` flag"));
                }
                self.legacy.push((start, self.i, Legacy::IdentityEscape));
                c as u32
            }
        };
        Ok(v)
    }

    fn class(&mut self) -> R<Class> {
        let start = self.i;
        self.i += 1; // [
        let negate = self.eat('^');
        if self.flags.v {
            return self.class_v(start, negate);
        }
        let mut items: Vec<CItem> = Vec::new();
        loop {
            match self.peek() {
                None => return Err("a character class `[` is never closed".into()),
                Some(']') => {
                    self.i += 1;
                    break;
                }
                _ => {}
            }
            let a = self.class_atom()?;
            // A range?
            if self.peek() == Some('-') && self.peek_at(1).is_some_and(|c| c != ']') {
                let dash = self.i;
                self.i += 1;
                let b = self.class_atom()?;
                match (&a.kind, &b.kind) {
                    (CKind::Char(lo), CKind::Char(hi)) => {
                        if lo > hi {
                            return Err("a range in a character class is out of order".into());
                        }
                        items.push(CItem { kind: CKind::Range { lo: *lo, hi: *hi, lo_span: (a.start, a.end), hi_span: (b.start, b.end) }, start: a.start, end: b.end });
                    }
                    _ => {
                        if self.unicode() {
                            return Err("a class escape can't be the end of a range".into());
                        }
                        self.legacy.push((a.start, b.end, Legacy::ClassEscapeRange));
                        items.push(a);
                        items.push(CItem { kind: CKind::Char('-' as u32), start: dash, end: dash + 1 });
                        items.push(b);
                    }
                }
                continue;
            }
            items.push(a);
        }
        Ok(Class { negate, op: ClassOp::Union, items, start, end: self.i })
    }

    fn class_atom(&mut self) -> R<CItem> {
        let start = self.i;
        let c = self.next().ok_or("a character class `[` is never closed")?;
        let kind = if c == '\\' {
            if self.peek() == Some('b') {
                self.i += 1;
                CKind::Char(8)
            } else if let Some(e) = self.set_escape()? {
                CKind::Set(e)
            } else if !self.unicode() && self.peek().is_some_and(|d| d.is_ascii_digit()) {
                let d = self.peek().unwrap();
                if d == '8' || d == '9' {
                    self.i += 1;
                    self.legacy.push((start, self.i, Legacy::IdentityEscape));
                    CKind::Char(d as u32)
                } else if d == '0' && !self.peek_at(1).is_some_and(|x| x.is_ascii_digit()) {
                    self.i += 1;
                    CKind::Char(0)
                } else {
                    let v = self.octal();
                    self.legacy.push((start, self.i, Legacy::Octal));
                    CKind::Char(v)
                }
            } else {
                CKind::Char(self.char_escape(start, true)?)
            }
        } else {
            CKind::Char(c as u32)
        };
        Ok(CItem { kind, start, end: self.i })
    }

    /// A class under the `v` flag: a union, or operands joined by `&&` or
    /// `--`.
    fn class_v(&mut self, start: usize, negate: bool) -> R<Class> {
        let mut items: Vec<CItem> = Vec::new();
        let mut op = ClassOp::Union;
        loop {
            match self.peek() {
                None => return Err("a character class `[` is never closed".into()),
                Some(']') => {
                    self.i += 1;
                    break;
                }
                _ => {}
            }
            if self.starts("&&") || self.starts("--") {
                let this = if self.starts("&&") { ClassOp::Intersection } else { ClassOp::Subtraction };
                if items.is_empty() || (op != ClassOp::Union && op != this) || (op == ClassOp::Union && items.len() != 1) {
                    return Err("a class set operator needs one operand on each side, and one kind of operator per class".into());
                }
                op = this;
                self.i += 2;
                if self.peek() == Some(']') {
                    return Err("a class set operator needs an operand after it".into());
                }
                items.push(self.class_v_operand()?);
                continue;
            }
            if op != ClassOp::Union {
                return Err("a class with a set operator holds operands joined by it only".into());
            }
            let a = self.class_v_operand()?;
            if self.peek() == Some('-') && !self.starts("--") {
                let CKind::Char(lo) = a.kind else { return Err("a range needs a character at each end".into()) };
                self.i += 1;
                let b = self.class_v_operand()?;
                let CKind::Char(hi) = b.kind else { return Err("a range needs a character at each end".into()) };
                if lo > hi {
                    return Err("a range in a character class is out of order".into());
                }
                items.push(CItem { kind: CKind::Range { lo, hi, lo_span: (a.start, a.end), hi_span: (b.start, b.end) }, start: a.start, end: b.end });
                continue;
            }
            items.push(a);
        }
        Ok(Class { negate, op, items, start, end: self.i })
    }

    fn class_v_operand(&mut self) -> R<CItem> {
        let start = self.i;
        let c = self.peek().ok_or("a character class `[` is never closed")?;
        if c == '[' {
            let inner = self.class()?;
            return Ok(CItem { start, end: self.i, kind: CKind::Class(inner) });
        }
        if self.starts("\\q{") {
            self.i += 3;
            let close = self.s[self.i..].find('}').ok_or("`\\q{` has no closing `}`")? + self.i;
            let body = &self.s[self.i..close];
            let strings = split_q(body);
            self.i = close + 1;
            return Ok(CItem { start, end: self.i, kind: CKind::Strings(strings) });
        }
        if c == '\\' {
            self.i += 1;
            if self.peek() == Some('b') {
                self.i += 1;
                return Ok(CItem { start, end: self.i, kind: CKind::Char(8) });
            }
            if let Some(e) = self.set_escape()? {
                return Ok(CItem { start, end: self.i, kind: CKind::Set(e) });
            }
            let v = self.char_escape(start, true)?;
            return Ok(CItem { start, end: self.i, kind: CKind::Char(v) });
        }
        if "()[]{}/-|".contains(c) {
            return Err(format!("`{c}` must be escaped in a class with the `v` flag"));
        }
        const DOUBLE: &str = "&!#$%*+,.:;<=>?@^`~";
        if DOUBLE.contains(c) && self.peek_at(1) == Some(c) {
            return Err(format!("`{c}{c}` is reserved in a class with the `v` flag"));
        }
        self.i += c.len_utf8();
        Ok(CItem { start, end: self.i, kind: CKind::Char(c as u32) })
    }
}

/// `\q{a|bc}`'s strings, escapes decoded loosely.
fn split_q(body: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut chars = body.chars();
    while let Some(c) = chars.next() {
        match c {
            '|' => out.push(String::new()),
            '\\' => {
                if let Some(n) = chars.next() {
                    out.last_mut().unwrap().push(n);
                }
            }
            c => out.last_mut().unwrap().push(c),
        }
    }
    out
}

fn valid_name(n: &str) -> bool {
    let mut c = n.chars();
    c.next().is_some_and(|f| f.is_alphabetic() || f == '_' || f == '$') && c.all(|x| x.is_alphanumeric() || x == '_' || x == '$')
}

/// Calls `f` on every item, outside in.
pub fn walk(alts: &[Alt], f: &mut dyn FnMut(&Item)) {
    for a in alts {
        for it in &a.items {
            walk_item(it, f);
        }
    }
}

pub fn walk_item(it: &Item, f: &mut dyn FnMut(&Item)) {
    f(it);
    match &it.kind {
        Kind::Look { alts, .. } | Kind::Group { alts, .. } => walk(alts, f),
        Kind::Quant { body, .. } => walk_item(body, f),
        _ => {}
    }
}

/// Every alternative list (the pattern's, each group's, each lookaround's),
/// with the item that holds it (`None` for the pattern).
pub fn each_disjunction<'a>(alts: &'a [Alt], holder: Option<&'a Item>, f: &mut dyn FnMut(&'a [Alt], Option<&'a Item>)) {
    f(alts, holder);
    for a in alts {
        for it in &a.items {
            each_disjunction_item(it, f);
        }
    }
}

fn each_disjunction_item<'a>(it: &'a Item, f: &mut dyn FnMut(&'a [Alt], Option<&'a Item>)) {
    match &it.kind {
        Kind::Look { alts, .. } | Kind::Group { alts, .. } => each_disjunction(alts, Some(it), f),
        Kind::Quant { body, .. } => each_disjunction_item(body, f),
        _ => {}
    }
}

/// Every class, nested ones included.
pub fn each_class<'a>(alts: &'a [Alt], f: &mut dyn FnMut(&'a Class)) {
    fn class<'a>(c: &'a Class, f: &mut dyn FnMut(&'a Class)) {
        f(c);
        for it in &c.items {
            if let CKind::Class(n) = &it.kind {
                class(n, f);
            }
        }
    }
    fn item<'a>(it: &'a Item, f: &mut dyn FnMut(&'a Class)) {
        match &it.kind {
            Kind::Class(c) => class(c, f),
            Kind::Look { alts, .. } | Kind::Group { alts, .. } => {
                for a in alts {
                    for i in &a.items {
                        item(i, f);
                    }
                }
            }
            Kind::Quant { body, .. } => item(body, f),
            _ => {}
        }
    }
    for a in alts {
        for i in &a.items {
            item(i, f);
        }
    }
}

/// Every alternative (item sequence), at every depth.
pub fn each_alt<'a>(alts: &'a [Alt], f: &mut dyn FnMut(&'a Alt)) {
    each_disjunction(alts, None, &mut |d, _| {
        for a in d {
            f(a);
        }
    });
}

// ---------------------------------------------------------------------------
// Character sets

/// A set of characters: sorted, non-overlapping, non-adjacent ranges.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CharSet(pub Vec<(u32, u32)>);

impl CharSet {
    pub fn empty() -> Self {
        CharSet(Vec::new())
    }
    pub fn of(ranges: &[(u32, u32)]) -> Self {
        let mut s = CharSet(ranges.to_vec());
        s.normalize();
        s
    }
    pub fn char(c: u32) -> Self {
        CharSet(vec![(c, c)])
    }
    fn normalize(&mut self) {
        self.0.sort();
        let mut out: Vec<(u32, u32)> = Vec::new();
        for &(a, b) in &self.0 {
            if let Some(last) = out.last_mut()
                && a <= last.1.saturating_add(1)
            {
                last.1 = last.1.max(b);
                continue;
            }
            out.push((a, b));
        }
        self.0 = out;
    }
    pub fn union(&self, o: &CharSet) -> CharSet {
        let mut v = self.0.clone();
        v.extend_from_slice(&o.0);
        CharSet::of(&v)
    }
    pub fn negate(&self, max: u32) -> CharSet {
        let mut out = Vec::new();
        let mut next = 0u32;
        for &(a, b) in &self.0 {
            if a > next {
                out.push((next, a - 1));
            }
            next = b.saturating_add(1);
        }
        if next <= max {
            out.push((next, max));
        }
        CharSet(out)
    }
    pub fn intersect(&self, o: &CharSet) -> CharSet {
        let mut out = Vec::new();
        for &(a, b) in &self.0 {
            for &(c, d) in &o.0 {
                let lo = a.max(c);
                let hi = b.min(d);
                if lo <= hi {
                    out.push((lo, hi));
                }
            }
        }
        CharSet::of(&out)
    }
    pub fn subtract(&self, o: &CharSet) -> CharSet {
        self.intersect(&o.negate(0x10FFFF))
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn contains(&self, c: u32) -> bool {
        self.0.iter().any(|&(a, b)| a <= c && c <= b)
    }
    pub fn is_subset(&self, o: &CharSet) -> bool {
        self.subtract(o).is_empty()
    }
    pub fn disjoint(&self, o: &CharSet) -> bool {
        self.intersect(o).is_empty()
    }
    pub fn len(&self) -> u64 {
        self.0.iter().map(|&(a, b)| (b - a) as u64 + 1).sum()
    }
    /// Closed under simple case folding (what the `i` flag matches).
    pub fn fold(&self) -> CharSet {
        let mut v = self.0.clone();
        for &(a, b) in &self.0 {
            if b - a > 0x3000 {
                continue;
            }
            for c in a..=b {
                for o in case_variants(c) {
                    v.push((o, o));
                }
            }
        }
        CharSet::of(&v)
    }
}

/// The other-case forms of a character (simple mappings only).
pub fn case_variants(c: u32) -> Vec<u32> {
    let Some(ch) = char::from_u32(c) else { return Vec::new() };
    let mut out = Vec::new();
    let mut l = ch.to_lowercase();
    if let (Some(x), None) = (l.next(), l.next())
        && x != ch
    {
        out.push(x as u32);
    }
    let mut u = ch.to_uppercase();
    if let (Some(x), None) = (u.next(), u.next())
        && x != ch
    {
        out.push(x as u32);
    }
    out
}

pub fn digit() -> CharSet {
    CharSet::of(&[(0x30, 0x39)])
}
pub fn word() -> CharSet {
    CharSet::of(&[(0x30, 0x39), (0x41, 0x5A), (0x5F, 0x5F), (0x61, 0x7A)])
}
pub fn space() -> CharSet {
    CharSet::of(&[(9, 13), (0x20, 0x20), (0xA0, 0xA0), (0x1680, 0x1680), (0x2000, 0x200A), (0x2028, 0x2029), (0x202F, 0x202F), (0x205F, 0x205F), (0x3000, 0x3000), (0xFEFF, 0xFEFF)])
}
pub fn line_terminators() -> CharSet {
    CharSet::of(&[(10, 10), (13, 13), (0x2028, 0x2029)])
}

/// What a class escape matches, `None` for a property.
pub fn esc_set(e: &SetEsc, flags: &Flags) -> Option<CharSet> {
    let base = match &e.kind {
        EscKind::Digit => digit(),
        EscKind::Word => {
            let w = word();
            if flags.i && flags.unicode() {
                // ſ and K (Kelvin) fold into the word set.
                w.union(&CharSet::of(&[(0x17F, 0x17F), (0x212A, 0x212A)]))
            } else {
                w
            }
        }
        EscKind::Space => space(),
        EscKind::Prop(_) => return None,
    };
    Some(if e.negate { base.negate(flags.max()) } else { base })
}

/// The characters a single-character item matches (folded under `i`);
/// `None` when it isn't one, or holds a property.
pub fn item_set(it: &Item, flags: &Flags) -> Option<CharSet> {
    let s = match &it.kind {
        Kind::Char(c) => CharSet::char(*c),
        Kind::Dot => {
            let all = CharSet::of(&[(0, flags.max())]);
            if flags.s { all } else { all.subtract(&line_terminators()) }
        }
        Kind::Set(e) => esc_set(e, flags)?,
        Kind::Class(c) => class_set(c, flags)?,
        _ => return None,
    };
    Some(if flags.i { s.fold() } else { s })
}

/// What a class matches, `None` when it holds a property or a string.
pub fn class_set(c: &Class, flags: &Flags) -> Option<CharSet> {
    let mut sets = Vec::new();
    for it in &c.items {
        sets.push(citem_set(it, flags)?);
    }
    let s = match c.op {
        ClassOp::Union => sets.iter().fold(CharSet::empty(), |a, b| a.union(b)),
        ClassOp::Intersection => {
            let mut it = sets.into_iter();
            let first = it.next().unwrap_or_default();
            it.fold(first, |a, b| a.intersect(&b))
        }
        ClassOp::Subtraction => {
            let mut it = sets.into_iter();
            let first = it.next().unwrap_or_default();
            it.fold(first, |a, b| a.subtract(&b))
        }
    };
    let s = if flags.i { s.fold() } else { s };
    Some(if c.negate { s.negate(flags.max()) } else { s })
}

pub fn citem_set(it: &CItem, flags: &Flags) -> Option<CharSet> {
    let s = match &it.kind {
        CKind::Char(c) => CharSet::char(*c),
        CKind::Range { lo, hi, .. } => CharSet::of(&[(*lo, *hi)]),
        CKind::Set(e) => esc_set(e, flags)?,
        CKind::Class(c) => class_set(c, flags)?,
        CKind::Strings(v) => {
            if v.iter().all(|s| s.chars().count() == 1) {
                CharSet::of(&v.iter().map(|s| {
                    let c = s.chars().next().unwrap() as u32;
                    (c, c)
                }).collect::<Vec<_>>())
            } else {
                return None;
            }
        }
    };
    Some(if flags.i { s.fold() } else { s })
}

// ---------------------------------------------------------------------------
// Small facts about items

/// Whether the item matches only the empty string (assertions, empty groups).
pub fn zero_width(it: &Item) -> bool {
    match &it.kind {
        Kind::Start | Kind::End | Kind::Boundary { .. } | Kind::Look { .. } => true,
        Kind::Group { alts, .. } => alts.iter().all(|a| a.items.iter().all(zero_width)),
        Kind::Quant { body, max, .. } => *max == Some(0) || zero_width(body),
        Kind::Backref(_) => false,
        _ => false,
    }
}

/// Whether the item can match the empty string.
pub fn can_be_empty(it: &Item) -> bool {
    match &it.kind {
        Kind::Start | Kind::End | Kind::Boundary { .. } | Kind::Look { .. } => true,
        Kind::Group { alts, .. } => alts.iter().any(|a| a.items.iter().all(can_be_empty)),
        Kind::Quant { body, min, .. } => *min == 0 || can_be_empty(body),
        Kind::Backref(_) => true,
        _ => false,
    }
}

/// Whether the item holds a capturing group.
pub fn has_capture(it: &Item) -> bool {
    let mut found = false;
    walk_item(it, &mut |x| {
        if matches!(x.kind, Kind::Group { capture: Some(_), .. }) {
            found = true;
        }
    });
    found
}

pub fn has_backref(alts: &[Alt]) -> bool {
    let mut found = false;
    walk(alts, &mut |x| {
        if matches!(x.kind, Kind::Backref(_)) {
            found = true;
        }
    });
    found
}

/// The quantifier text for `{min,max}`, shortest form.
pub fn quant_text(min: u32, max: Option<u32>, greedy: bool) -> String {
    let mut s = match (min, max) {
        (0, None) => "*".to_owned(),
        (1, None) => "+".to_owned(),
        (0, Some(1)) => "?".to_owned(),
        (n, None) => format!("{{{n},}}"),
        (n, Some(m)) if n == m => format!("{{{n}}}"),
        (n, Some(m)) => format!("{{{n},{m}}}"),
    };
    if !greedy {
        s.push('?');
    }
    s
}

/// How to write a character outside a class, in a pattern with these flags.
pub fn char_text(c: u32, flags: &Flags) -> String {
    let Some(ch) = char::from_u32(c) else { return format!("\\u{c:04X}") };
    match c {
        9 => "\\t".into(),
        10 => "\\n".into(),
        11 => "\\v".into(),
        12 => "\\f".into(),
        13 => "\\r".into(),
        0 => "\\0".into(),
        _ if SYNTAX.contains(ch) => format!("\\{ch}"),
        _ if c < 0x20 || c == 0x7F => format!("\\x{c:02X}"),
        _ if c > 0xFFFF && !flags.unicode() => {
            let v = c - 0x10000;
            format!("\\u{:04X}\\u{:04X}", 0xD800 + (v >> 10), 0xDC00 + (v & 0x3FF))
        }
        _ => ch.to_string(),
    }
}

/// How to write a character inside a class.
pub fn class_char_text(c: u32, flags: &Flags) -> String {
    let Some(ch) = char::from_u32(c) else { return format!("\\u{c:04X}") };
    let special = if flags.v { "\\]-[^(){}/|&!#$%*+,.:;<=>?@`~" } else { "\\]-^[" };
    if special.contains(ch) {
        return format!("\\{ch}");
    }
    let mut s = char_text(c, flags);
    if s.starts_with('\\') && s.len() == 2 && SYNTAX.contains(&s[1..]) {
        s.remove(0);
    }
    s
}

/// A short description of a set for messages.
pub fn describe(s: &CharSet) -> String {
    let mut out = String::new();
    for (k, &(a, b)) in s.0.iter().enumerate() {
        if k > 0 {
            out.push(' ');
        }
        if k == 6 {
            out.push('…');
            break;
        }
        let _ = if a == b { write!(out, "U+{a:04X}") } else { write!(out, "U+{a:04X}–U+{b:04X}") };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(p: &str, f: &str) -> String {
        match parse(p, f) {
            Ok(r) => format!("{:?}", r.alts.iter().map(|a| a.items.len()).collect::<Vec<_>>()),
            Err(e) => format!("error: {e}"),
        }
    }

    #[test]
    fn parses_the_three_modes() {
        assert_eq!(shape("a|bc|", ""), "[1, 2, 0]");
        assert_eq!(shape("(?<y>\\d{4})-\\k<y>", ""), "[3]");
        assert_eq!(shape("a{", ""), "[2]");
        assert!(shape("a{", "u").starts_with("error"));
        assert_eq!(shape("[\\w&&\\d]", "v"), "[1]");
        assert!(shape("[a&&b--c]", "v").starts_with("error"));
        assert_eq!(shape("[\\q{abc|d}a-z]", "v"), "[1]");
        assert_eq!(shape("\\1(a)", ""), "[2]");
        assert_eq!(shape("\\8", ""), "[1]");
        assert!(shape("\\8", "u").starts_with("error"));
        assert!(shape("a**", "").starts_with("error"));
        assert_eq!(shape("(?i:a)b", ""), "[2]");
        assert!(shape("(?<a>x)(?<a>y)", "").starts_with("error"));
        assert_eq!(shape("\\uD83D\\uDE00", "u"), "[1]");
    }

    #[test]
    fn sets() {
        let r = parse("[a-z\\d]", "").unwrap();
        let s = item_set(&r.alts[0].items[0], &r.flags).unwrap();
        assert!(digit().is_subset(&s));
        assert!(!word().is_subset(&s));
        let r = parse("[a-z]", "i").unwrap();
        let s = item_set(&r.alts[0].items[0], &r.flags).unwrap();
        assert!(s.contains('Q' as u32));
        assert_eq!(digit().negate(0xFFFF).negate(0xFFFF), digit());
    }
}

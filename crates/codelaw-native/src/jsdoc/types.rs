//! A parser for the type expressions inside JSDoc `{…}`: TypeScript's type
//! syntax together with JSDoc's and Closure's (`Array.<T>`, `function(string)`,
//! `?T`, `!T`, `T=`, `...T`, `*`, `module:name`, `Foo#bar`). It keeps the
//! source position of every node so a rule can point at, or rewrite, one part
//! of a type, and prints a type in the plugin's default format.
//!
//! Constructs the rules never look inside (conditional types, indexed access,
//! `import()`, `asserts x is T`, accessors) are kept as `Raw` nodes: their text
//! is carried verbatim and only their parts a rule may name are parsed.

use std::fmt::Write as _;

#[derive(Clone, Debug)]
pub struct Ty {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Name(String),
    /// A string literal, quotes included.
    Str(String),
    Num(String),
    Star,
    Unknown,
    Prefix(String, Box<Ty>),
    Suffix(char, Box<Ty>),
    Union(Vec<Ty>),
    Inter(Vec<Ty>),
    Array(Box<Ty>),
    Generic { base: Box<Ty>, args: Vec<Ty>, dot: bool },
    Object(Vec<Member>),
    Tuple(Vec<Ty>),
    Func(Box<Func>),
    Paren(Box<Ty>),
    Template { parts: Vec<String>, interps: Vec<Ty> },
    /// `import('module')`: the specifier, unquoted.
    Import(String),
    /// Verbatim text; the children are the types inside it.
    Raw(Vec<Ty>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Function,
    Arrow,
    New,
    AbstractNew,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub style: Style,
    pub tparams: Vec<TParam>,
    pub params: Vec<Param>,
    pub ret: Option<Ty>,
}

#[derive(Clone, Debug)]
pub struct TParam {
    pub name: String,
    pub constraint: Option<Ty>,
    pub default: Option<Ty>,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub rest: bool,
    pub name: Option<String>,
    pub optional: bool,
    pub ty: Option<Ty>,
}

#[derive(Clone, Debug)]
pub struct Key {
    /// As written, quotes included.
    pub raw: String,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub kind: MKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub enum MKind {
    Field { readonly: bool, key: Key, optional: bool, ty: Option<Ty> },
    Method { key: Key, optional: bool, f: Func },
    /// `(a): R` or `new (a): R`.
    Call { f: Func },
    /// Verbatim: an index signature, a computed key, an accessor.
    Verbatim(Vec<Ty>),
}

// ---------------------------------------------------------------- lexer

#[derive(Clone, Debug, PartialEq)]
enum T {
    Ident(String),
    Str,
    Num,
    Template,
    P(&'static str),
    Eof,
}

#[derive(Clone, Debug)]
struct Tok {
    t: T,
    s: usize,
    e: usize,
}

const PUNCT: &[&str] = &["...", "=>", ".<", "<", ">", "(", ")", "[", "]", "{", "}", ",", ";", ":", "?", "!", "|", "&", "=", "*", ".", "-", "+", "#", "~", "@", "/"];

fn is_id_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

fn lex(src: &str) -> Result<Vec<Tok>, String> {
    let b = src.as_bytes();
    let mut toks = Vec::new();
    let mut i = 0;
    while i < src.len() {
        let c = src[i..].chars().next().unwrap();
        if c == '\n' {
            // A comment's line delimiter: spaces then one `*` (not `*/`).
            i += 1;
            let rest = &src[i..];
            let t = rest.trim_start_matches([' ', '\t', '\r']);
            let ws = rest.len() - t.len();
            if t.starts_with('*') && !t.starts_with("*/") {
                i += ws + 1;
            }
            continue;
        }
        if c.is_whitespace() {
            i += c.len_utf8();
            continue;
        }
        let s = i;
        if is_id_start(c) {
            let mut j = i;
            loop {
                while j < src.len() && is_id_part(src[j..].chars().next().unwrap()) {
                    j += src[j..].chars().next().unwrap().len_utf8();
                }
                // `.name`, `#name`, `~name` continue a namepath; `.<` and `...` do not.
                if j + 1 < src.len() && matches!(b[j], b'.' | b'#' | b'~') && src[j + 1..].chars().next().is_some_and(is_id_start) && !(b[j] == b'.' && b[j + 1] == b'<') {
                    j += 1;
                    continue;
                }
                break;
            }
            let word = &src[i..j];
            // `module:path/to-file`, `external:name`, `event:name`.
            if matches!(word, "module" | "external" | "event") && src[j..].starts_with(':') && !src[j + 1..].starts_with(|c: char| c.is_whitespace()) {
                let mut k = j + 1;
                while k < src.len() {
                    let ch = src[k..].chars().next().unwrap();
                    if ch.is_whitespace() || matches!(ch, ',' | ';' | '(' | ')' | '{' | '}' | '<' | '>' | '|' | '&' | '=' | '[' | ']') {
                        break;
                    }
                    k += ch.len_utf8();
                }
                toks.push(Tok { t: T::Ident(src[i..k].to_owned()), s, e: k });
                i = k;
                continue;
            }
            toks.push(Tok { t: T::Ident(word.to_owned()), s, e: j });
            i = j;
            continue;
        }
        if c.is_ascii_digit() || (c == '.' && src[i + 1..].starts_with(|d: char| d.is_ascii_digit())) {
            let mut j = i;
            while j < src.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'.' || b[j] == b'_' || ((b[j] == b'-' || b[j] == b'+') && matches!(b[j - 1], b'e' | b'E') && !src[i..j].starts_with("0x"))) {
                j += 1;
            }
            toks.push(Tok { t: T::Num, s, e: j });
            i = j;
            continue;
        }
        if c == '"' || c == '\'' {
            let mut j = i + 1;
            loop {
                if j >= src.len() {
                    return Err("unterminated string".into());
                }
                match b[j] {
                    b'\\' => j += 2,
                    q if q == c as u8 => {
                        j += 1;
                        break;
                    }
                    _ => j += 1,
                }
            }
            toks.push(Tok { t: T::Str, s, e: j.min(src.len()) });
            i = j;
            continue;
        }
        if c == '`' {
            let mut j = i + 1;
            let mut depth = 0usize;
            loop {
                if j >= src.len() {
                    return Err("unterminated template literal".into());
                }
                match b[j] {
                    b'\\' => j += 2,
                    b'$' if depth == 0 && b.get(j + 1) == Some(&b'{') => {
                        depth = 1;
                        j += 2;
                    }
                    b'{' if depth > 0 => {
                        depth += 1;
                        j += 1;
                    }
                    b'}' if depth > 0 => {
                        depth -= 1;
                        j += 1;
                    }
                    b'`' if depth == 0 => {
                        j += 1;
                        break;
                    }
                    _ => j += 1,
                }
            }
            toks.push(Tok { t: T::Template, s, e: j.min(src.len()) });
            i = j;
            continue;
        }
        let mut hit = None;
        for p in PUNCT {
            if src[i..].starts_with(p) {
                hit = Some(*p);
                break;
            }
        }
        match hit {
            Some(p) => {
                toks.push(Tok { t: T::P(p), s, e: i + p.len() });
                i += p.len();
            }
            None => return Err(format!("unexpected character `{c}`")),
        }
    }
    toks.push(Tok { t: T::Eof, s: src.len(), e: src.len() });
    Ok(toks)
}

// --------------------------------------------------------------- parser

struct P<'a> {
    src: &'a str,
    base: usize,
    toks: Vec<Tok>,
    i: usize,
    /// Inside the `extends` clause of a conditional type: `?` ends it.
    in_ext: bool,
    depth: usize,
}

type R<T> = Result<T, String>;

/// Parse a whole type; positions are offsets into `src`.
pub fn parse(src: &str) -> R<Ty> {
    parse_at(src, 0)
}

fn parse_at(src: &str, base: usize) -> R<Ty> {
    let toks = lex(src)?;
    let mut p = P { src, base, toks, i: 0, in_ext: false, depth: 0 };
    let t = p.ty()?;
    if p.cur() != &T::Eof {
        return Err(format!("unexpected `{}`", p.cur_text()));
    }
    Ok(t)
}

impl<'a> P<'a> {
    fn cur(&self) -> &T {
        &self.toks[self.i].t
    }
    fn peek(&self, n: usize) -> &T {
        &self.toks[(self.i + n).min(self.toks.len() - 1)].t
    }
    fn s(&self) -> usize {
        self.toks[self.i].s
    }
    fn prev_end(&self) -> usize {
        if self.i == 0 { 0 } else { self.toks[self.i - 1].e }
    }
    fn cur_text(&self) -> &str {
        &self.src[self.toks[self.i].s..self.toks[self.i].e]
    }
    fn is(&self, p: &str) -> bool {
        matches!(self.cur(), T::P(x) if *x == p)
    }
    fn is_at(&self, n: usize, p: &str) -> bool {
        matches!(self.peek(n), T::P(x) if *x == p)
    }
    fn is_word(&self, w: &str) -> bool {
        matches!(self.cur(), T::Ident(x) if x == w)
    }
    fn is_word_at(&self, n: usize, w: &str) -> bool {
        matches!(self.peek(n), T::Ident(x) if x == w)
    }
    fn eat(&mut self, p: &str) -> bool {
        if self.is(p) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, p: &str) -> R<()> {
        if self.eat(p) {
            Ok(())
        } else {
            Err(format!("expected `{p}`, found `{}`", if self.cur() == &T::Eof { "the end".to_owned() } else { self.cur_text().to_owned() }))
        }
    }
    fn mk(&self, kind: Kind, s: usize, e: usize) -> Ty {
        Ty { kind, start: self.base + s, end: self.base + e }
    }
    fn slice(&self, s: usize, e: usize) -> String {
        self.src[s..e].to_owned()
    }

    /// Whether the current token can start a type.
    fn starts_type(&self) -> bool {
        match self.cur() {
            T::Ident(_) | T::Str | T::Num | T::Template => true,
            T::P(p) => matches!(*p, "(" | "[" | "{" | "<" | "?" | "!" | "..." | "*" | "-"),
            T::Eof => false,
        }
    }

    /// Whether the current token ends a type (so a trailing `=`, `?`, `!` is a suffix).
    fn at_boundary(&self) -> bool {
        match self.cur() {
            T::Eof => true,
            T::P(p) => matches!(*p, "," | ")" | "}" | ">" | "]" | "|" | "&" | ";" | "=>" | "=" | ":"),
            _ => false,
        }
    }

    fn ty(&mut self) -> R<Ty> {
        self.depth += 1;
        if self.depth > 200 {
            return Err("type is nested too deeply".into());
        }
        let r = self.conditional();
        self.depth -= 1;
        r
    }

    fn conditional(&mut self) -> R<Ty> {
        let check = self.union()?;
        if self.is_word("extends") && !self.in_ext {
            self.i += 1;
            let was = self.in_ext;
            self.in_ext = true;
            let ext = self.union()?;
            self.in_ext = was;
            self.expect("?")?;
            let a = self.ty()?;
            self.expect(":")?;
            let b = self.ty()?;
            let (s, e) = (check.start - self.base, b.end - self.base);
            return Ok(self.mk(Kind::Raw(vec![check, ext, a, b]), s, e));
        }
        Ok(check)
    }

    fn union(&mut self) -> R<Ty> {
        let start = self.s();
        let leading = self.eat("|");
        let first = self.inter()?;
        if !self.is("|") {
            if leading {
                let e = self.prev_end();
                return Ok(self.mk(Kind::Union(vec![first]), start, e));
            }
            return Ok(first);
        }
        let mut parts = vec![first];
        while self.eat("|") {
            parts.push(self.inter()?);
        }
        let e = self.prev_end();
        Ok(self.mk(Kind::Union(parts), start, e))
    }

    fn inter(&mut self) -> R<Ty> {
        let start = self.s();
        let leading = self.eat("&");
        let first = self.unary()?;
        if !self.is("&") {
            if leading {
                let e = self.prev_end();
                return Ok(self.mk(Kind::Inter(vec![first]), start, e));
            }
            return Ok(first);
        }
        let mut parts = vec![first];
        while self.eat("&") {
            parts.push(self.unary()?);
        }
        let e = self.prev_end();
        Ok(self.mk(Kind::Inter(parts), start, e))
    }

    fn unary(&mut self) -> R<Ty> {
        let start = self.s();
        if let T::P(p) = self.cur().clone() {
            match p {
                "?" => {
                    self.i += 1;
                    if self.starts_type() && !self.at_boundary() {
                        let inner = self.unary()?;
                        let e = self.prev_end();
                        return Ok(self.mk(Kind::Prefix("?".into(), Box::new(inner)), start, e));
                    }
                    return Ok(self.mk(Kind::Unknown, start, start + 1));
                }
                "!" | "..." => {
                    self.i += 1;
                    if self.starts_type() {
                        let inner = self.unary()?;
                        let e = self.prev_end();
                        return Ok(self.mk(Kind::Prefix(p.into(), Box::new(inner)), start, e));
                    }
                    // A bare `...` (the rest of a parameter list in Closure types).
                    return Ok(self.mk(Kind::Name(p.into()), start, start + p.len()));
                }
                _ => {}
            }
        }
        if let T::Ident(w) = self.cur().clone() {
            let next_is_type_start = !matches!(self.peek(1), T::Eof) && !matches!(self.peek(1), T::P(x) if matches!(*x, "," | ")" | "}" | ">" | "]" | "|" | "&" | ";" | "=>" | "=" | ":" | "?" | "." | ".<"));
            match w.as_str() {
                "keyof" | "typeof" | "readonly" | "unique" | "infer" if next_is_type_start => {
                    self.i += 1;
                    let inner = self.unary()?;
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Prefix(w, Box::new(inner)), start, e));
                }
                "asserts" if next_is_type_start => {
                    // `asserts x` or `asserts x is T`
                    self.i += 1;
                    let mut kids = Vec::new();
                    let nm = self.s();
                    self.i += 1;
                    let ne = self.prev_end();
                    kids.push(self.mk(Kind::Name(self.slice(nm, ne)), nm, ne));
                    if self.is_word("is") {
                        self.i += 1;
                        kids.push(self.ty()?);
                    }
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Raw(kids), start, e));
                }
                _ => {}
            }
        }
        self.postfix()
    }

    fn postfix(&mut self) -> R<Ty> {
        let start = self.s();
        let mut t = self.primary()?;
        loop {
            if self.is("[") {
                if self.is_at(1, "]") {
                    self.i += 2;
                    let e = self.prev_end();
                    t = self.mk(Kind::Array(Box::new(t)), start, e);
                } else {
                    self.i += 1;
                    let idx = self.ty()?;
                    self.expect("]")?;
                    let e = self.prev_end();
                    t = self.mk(Kind::Raw(vec![t, idx]), start, e);
                }
            } else if self.is("<") && matches!(t.kind, Kind::Name(_) | Kind::Raw(_) | Kind::Import(_)) {
                let args = self.type_args()?;
                let e = self.prev_end();
                t = self.mk(Kind::Generic { base: Box::new(t), args, dot: false }, start, e);
            } else if self.is(".<") {
                self.i += 1;
                let args = self.type_args_rest()?;
                let e = self.prev_end();
                t = self.mk(Kind::Generic { base: Box::new(t), args, dot: true }, start, e);
            } else if self.is(".") && matches!(self.peek(1), T::Ident(_)) {
                self.i += 2;
                let e = self.prev_end();
                t = self.mk(Kind::Raw(vec![t]), start, e);
            } else if self.is("=") && !self.in_ext && matches!(self.peek(1), T::Eof | T::P("," | ")" | "}" | ">" | "]" | "|" | ";")) {
                self.i += 1;
                let e = self.prev_end();
                t = self.mk(Kind::Suffix('=', Box::new(t)), start, e);
            } else if (self.is("?") && !self.in_ext || self.is("!")) && matches!(self.peek(1), T::Eof | T::P("," | ")" | "}" | ">" | "]" | "|" | "&" | ";" | "=" | "=>" | ":")) {
                let c = if self.is("?") { '?' } else { '!' };
                self.i += 1;
                let e = self.prev_end();
                t = self.mk(Kind::Suffix(c, Box::new(t)), start, e);
            } else if self.is_word("is") && matches!(t.kind, Kind::Name(_)) && !self.in_ext && self.starts_type_at(1) {
                self.i += 1;
                let rhs = self.union()?;
                let e = self.prev_end();
                t = self.mk(Kind::Raw(vec![t, rhs]), start, e);
            } else {
                break;
            }
        }
        Ok(t)
    }

    fn starts_type_at(&self, n: usize) -> bool {
        match self.peek(n) {
            T::Ident(_) | T::Str | T::Num | T::Template => true,
            T::P(p) => matches!(*p, "(" | "[" | "{" | "<" | "?" | "!" | "..." | "*" | "-"),
            T::Eof => false,
        }
    }

    fn type_args(&mut self) -> R<Vec<Ty>> {
        self.expect("<")?;
        self.type_args_rest()
    }

    fn type_args_rest(&mut self) -> R<Vec<Ty>> {
        // After the `<` (or `.<`).
        let mut args = Vec::new();
        if self.is(">") {
            return Err("expected a type argument".into());
        }
        loop {
            args.push(self.ty()?);
            if !self.eat(",") {
                break;
            }
            if self.is(">") {
                break;
            }
        }
        self.expect(">")?;
        Ok(args)
    }

    fn primary(&mut self) -> R<Ty> {
        let start = self.s();
        let tok = self.toks[self.i].clone();
        match tok.t {
            T::Ident(ref w) => {
                if w == "function" && (self.is_at(1, "(") || self.is_at(1, "<")) {
                    self.i += 1;
                    let f = self.signature(Style::Function)?;
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Func(Box::new(f)), start, e));
                }
                if w == "new" && (self.is_at(1, "(") || self.is_at(1, "<")) {
                    self.i += 1;
                    let f = self.arrow(Style::New)?;
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Func(Box::new(f)), start, e));
                }
                if w == "abstract" && self.is_word_at(1, "new") {
                    self.i += 2;
                    let f = self.arrow(Style::AbstractNew)?;
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Func(Box::new(f)), start, e));
                }
                if w == "import" && self.is_at(1, "(") {
                    self.i += 2;
                    let (ss, se) = (self.s(), self.toks[self.i].e);
                    if !matches!(self.cur(), T::Str) {
                        return Err("expected a module name string in `import(…)`".into());
                    }
                    let spec = self.slice(ss + 1, se - 1);
                    self.i += 1;
                    if self.eat(",") && !self.is(")") {
                        // import attributes
                        self.skip_balanced_until_close_paren()?;
                    }
                    self.expect(")")?;
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Import(spec), start, e));
                }
                self.i += 1;
                Ok(self.mk(Kind::Name(w.clone()), start, tok.e))
            }
            T::Str => {
                self.i += 1;
                Ok(self.mk(Kind::Str(self.slice(tok.s, tok.e)), start, tok.e))
            }
            T::Num => {
                self.i += 1;
                Ok(self.mk(Kind::Num(self.slice(tok.s, tok.e)), start, tok.e))
            }
            T::Template => {
                self.i += 1;
                self.template(tok.s, tok.e)
            }
            T::P("*") => {
                self.i += 1;
                Ok(self.mk(Kind::Star, start, tok.e))
            }
            T::P("-") if matches!(self.peek(1), T::Num) => {
                self.i += 2;
                let e = self.prev_end();
                Ok(self.mk(Kind::Num(self.slice(start, e)), start, e))
            }
            T::P("(") => {
                // An arrow function when the matching `)` is followed by `=>`.
                if self.paren_is_arrow() {
                    let f = self.arrow(Style::Arrow)?;
                    let e = self.prev_end();
                    return Ok(self.mk(Kind::Func(Box::new(f)), start, e));
                }
                self.i += 1;
                let was = self.in_ext;
                self.in_ext = false;
                let inner = self.ty()?;
                self.in_ext = was;
                self.expect(")")?;
                let e = self.prev_end();
                Ok(self.mk(Kind::Paren(Box::new(inner)), start, e))
            }
            T::P("<") => {
                let f = self.arrow(Style::Arrow)?;
                let e = self.prev_end();
                Ok(self.mk(Kind::Func(Box::new(f)), start, e))
            }
            T::P("{") => self.object(),
            T::P("[") => self.tuple(),
            T::Eof => Err("unexpected end of type".into()),
            _ => Err(format!("unexpected `{}`", self.cur_text())),
        }
    }

    fn skip_balanced_until_close_paren(&mut self) -> R<()> {
        let mut depth = 0i32;
        loop {
            match self.cur() {
                T::Eof => return Err("expected `)`".into()),
                T::P("(") | T::P("{") | T::P("[") => depth += 1,
                T::P("}") | T::P("]") => depth -= 1,
                T::P(")") => {
                    if depth == 0 {
                        return Ok(());
                    }
                    depth -= 1;
                }
                _ => {}
            }
            self.i += 1;
        }
    }

    fn paren_is_arrow(&self) -> bool {
        let mut depth = 0i32;
        let mut j = self.i;
        while j < self.toks.len() {
            match &self.toks[j].t {
                T::P("(") | T::P("[") | T::P("{") => depth += 1,
                T::P(")") | T::P("]") | T::P("}") => {
                    depth -= 1;
                    if depth == 0 {
                        return matches!(self.toks.get(j + 1).map(|t| &t.t), Some(T::P("=>")));
                    }
                }
                T::Eof => return false,
                _ => {}
            }
            j += 1;
        }
        false
    }

    fn template(&mut self, s: usize, e: usize) -> R<Ty> {
        let raw = &self.src[s..e];
        let inner = &raw[1..raw.len() - 1];
        let mut parts = vec![String::new()];
        let mut interps = Vec::new();
        let b = inner.as_bytes();
        let mut i = 0;
        while i < inner.len() {
            if b[i] == b'\\' {
                parts.last_mut().unwrap().push_str(&inner[i..(i + 2).min(inner.len())]);
                i += 2;
                continue;
            }
            if b[i] == b'$' && b.get(i + 1) == Some(&b'{') {
                let mut depth = 1;
                let mut j = i + 2;
                while j < inner.len() && depth > 0 {
                    match b[j] {
                        b'{' => depth += 1,
                        b'}' => depth -= 1,
                        _ => {}
                    }
                    j += 1;
                }
                if depth > 0 {
                    return Err("unterminated template expression".into());
                }
                let body_start = i + 2;
                let body_end = j - 1;
                let piece = &inner[body_start..body_end];
                let off = self.base + s + 1 + body_start;
                interps.push(parse_at(piece, off)?);
                parts.push(String::new());
                i = j;
                continue;
            }
            let ch = inner[i..].chars().next().unwrap();
            parts.last_mut().unwrap().push(ch);
            i += ch.len_utf8();
        }
        Ok(self.mk(Kind::Template { parts, interps }, s, e))
    }

    fn tuple(&mut self) -> R<Ty> {
        let start = self.s();
        self.expect("[")?;
        let mut elems = Vec::new();
        while !self.is("]") {
            // `name: T`, `name?: T`, `...name: T`
            let es = self.s();
            let named = {
                let mut n = 0;
                if self.is_at(0, "...") {
                    n = 1;
                }
                matches!(self.peek(n), T::Ident(_)) && (self.is_at(n + 1, ":") || (self.is_at(n + 1, "?") && self.is_at(n + 2, ":")))
            };
            if named {
                let mut kids = Vec::new();
                self.eat("...");
                self.i += 1;
                self.eat("?");
                self.expect(":")?;
                kids.push(self.ty()?);
                let e = self.prev_end();
                elems.push(self.mk(Kind::Raw(kids), es, e));
            } else {
                elems.push(self.ty()?);
            }
            if !self.eat(",") {
                break;
            }
        }
        self.expect("]")?;
        let e = self.prev_end();
        Ok(self.mk(Kind::Tuple(elems), start, e))
    }

    /// `<T, U extends X = Y>`
    fn type_params(&mut self) -> R<Vec<TParam>> {
        let mut out = Vec::new();
        self.expect("<")?;
        while !self.is(">") {
            let T::Ident(name) = self.cur().clone() else { return Err("expected a type parameter name".into()) };
            self.i += 1;
            let mut tp = TParam { name, constraint: None, default: None };
            if self.is_word("extends") {
                self.i += 1;
                tp.constraint = Some(self.ty()?);
            }
            if self.eat("=") {
                tp.default = Some(self.ty()?);
            }
            out.push(tp);
            if !self.eat(",") {
                break;
            }
        }
        self.expect(">")?;
        Ok(out)
    }

    fn params(&mut self) -> R<Vec<Param>> {
        self.expect("(")?;
        let mut out = Vec::new();
        while !self.is(")") {
            let rest = self.eat("...");
            let mut p = Param { rest, name: None, optional: false, ty: None };
            let named = match self.cur() {
                T::Ident(_) => self.is_at(1, ":") || (self.is_at(1, "?") && self.is_at(2, ":")),
                _ => false,
            };
            if named {
                let T::Ident(n) = self.cur().clone() else { unreachable!() };
                self.i += 1;
                p.name = Some(n);
                p.optional = self.eat("?");
                self.expect(":")?;
                p.ty = Some(self.ty()?);
            } else if (self.is("{") || self.is("[")) && self.pattern_then_colon() {
                // A destructuring pattern as the name.
                let (a, b) = self.skip_pattern()?;
                p.name = Some(self.slice(a, b));
                p.optional = self.eat("?");
                self.expect(":")?;
                p.ty = Some(self.ty()?);
            } else if matches!(self.cur(), T::Ident(_)) && (self.is_at(1, ",") || self.is_at(1, ")")) && rest {
                let T::Ident(n) = self.cur().clone() else { unreachable!() };
                self.i += 1;
                p.name = Some(n);
            } else {
                p.ty = Some(self.ty()?);
            }
            out.push(p);
            if !self.eat(",") {
                break;
            }
        }
        self.expect(")")?;
        Ok(out)
    }

    fn pattern_then_colon(&self) -> bool {
        let mut depth = 0i32;
        let mut j = self.i;
        while j < self.toks.len() {
            match &self.toks[j].t {
                T::P("{") | T::P("[") | T::P("(") => depth += 1,
                T::P("}") | T::P("]") | T::P(")") => {
                    depth -= 1;
                    if depth == 0 {
                        return matches!(self.toks.get(j + 1).map(|t| &t.t), Some(T::P(":")) | Some(T::P("?")));
                    }
                }
                T::Eof => return false,
                _ => {}
            }
            j += 1;
        }
        false
    }

    fn skip_pattern(&mut self) -> R<(usize, usize)> {
        let s = self.s();
        let mut depth = 0i32;
        loop {
            match self.cur() {
                T::P("{") | T::P("[") | T::P("(") => depth += 1,
                T::P("}") | T::P("]") | T::P(")") => {
                    depth -= 1;
                    if depth == 0 {
                        self.i += 1;
                        break;
                    }
                }
                T::Eof => return Err("unterminated pattern".into()),
                _ => {}
            }
            self.i += 1;
        }
        Ok((s, self.prev_end()))
    }

    /// `function(…): R` (after the `function` keyword).
    fn signature(&mut self, style: Style) -> R<Func> {
        let tparams = if self.is("<") { self.type_params()? } else { Vec::new() };
        let params = self.params()?;
        let ret = if self.eat(":") { Some(self.ty()?) } else { None };
        Ok(Func { style, tparams, params, ret })
    }

    /// `<T>(a) => R`, with the `new` already read for the constructor styles.
    fn arrow(&mut self, style: Style) -> R<Func> {
        let tparams = if self.is("<") { self.type_params()? } else { Vec::new() };
        let params = self.params()?;
        self.expect("=>")?;
        let ret = Some(self.ty()?);
        Ok(Func { style, tparams, params, ret })
    }

    fn object(&mut self) -> R<Ty> {
        let start = self.s();
        self.expect("{")?;
        let mut members = Vec::new();
        loop {
            while self.eat(",") || self.eat(";") {}
            if self.is("}") {
                break;
            }
            members.push(self.member()?);
            if !(self.is(",") || self.is(";") || self.is("}")) {
                // A line break may separate fields.
                let prev_end = self.prev_end();
                let here = self.s();
                if !self.src[prev_end..here].contains('\n') {
                    return Err(format!("expected `,` or `}}`, found `{}`", self.cur_text()));
                }
            }
        }
        self.expect("}")?;
        let e = self.prev_end();
        Ok(self.mk(Kind::Object(members), start, e))
    }

    fn member(&mut self) -> R<Member> {
        let start = self.s();
        let mk = |p: &P, kind: MKind, e: usize| Member { kind, start: p.base + start, end: p.base + e };
        // Call and construct signatures.
        if self.is("(") || self.is("<") {
            let f = self.signature(Style::Function)?;
            let e = self.prev_end();
            return Ok(mk(self, MKind::Call { f }, e));
        }
        if self.is_word("new") && (self.is_at(1, "(") || self.is_at(1, "<")) {
            self.i += 1;
            let f = self.signature(Style::New)?;
            let e = self.prev_end();
            return Ok(mk(self, MKind::Call { f }, e));
        }
        let mut readonly = false;
        if (self.is_word("readonly") || self.is("+") || self.is("-")) && !self.is_at(1, ":") && !self.is_at(1, "?") && !self.is_at(1, "(") && !self.is_at(1, ",") && !self.is_at(1, "}") && !self.is_at(1, ";") {
            if self.is("+") || self.is("-") {
                self.i += 1;
            }
            if self.is_word("readonly") {
                self.i += 1;
                readonly = true;
            }
        }
        // get/set accessors.
        if (self.is_word("get") || self.is_word("set")) && matches!(self.peek(1), T::Ident(_) | T::Str) && self.is_at(2, "(") {
            self.i += 1;
            let key_s = self.s();
            self.i += 1;
            let _ = key_s;
            let f = self.signature(Style::Function)?;
            let e = self.prev_end();
            let kids = f.ret.into_iter().collect();
            return Ok(mk(self, MKind::Verbatim(kids), e));
        }
        // Index signatures and computed keys (and mapped types).
        if self.is("[") {
            let mut kids = Vec::new();
            self.i += 1;
            let mut depth = 1;
            while depth > 0 {
                match self.cur() {
                    T::Eof => return Err("expected `]`".into()),
                    T::P("[") => depth += 1,
                    T::P("]") => depth -= 1,
                    _ => {}
                }
                self.i += 1;
            }
            // `+?` / `-?` / `?`
            if self.is("+") || self.is("-") {
                self.i += 1;
            }
            self.eat("?");
            if self.is("(") || self.is("<") {
                let f = self.signature(Style::Function)?;
                kids.extend(f.ret);
            } else if self.eat(":") {
                kids.push(self.ty()?);
            }
            let e = self.prev_end();
            return Ok(mk(self, MKind::Verbatim(kids), e));
        }
        let ks = self.s();
        let key_e = match self.cur() {
            T::Ident(_) | T::Str | T::Num => self.toks[self.i].e,
            _ => return Err(format!("unexpected `{}` in an object type", self.cur_text())),
        };
        let key = Key { raw: self.slice(ks, key_e) };
        self.i += 1;
        let optional = self.eat("?");
        if self.is("(") || self.is("<") {
            let f = self.signature(Style::Function)?;
            let e = self.prev_end();
            return Ok(mk(self, MKind::Method { key, optional, f }, e));
        }
        let ty = if self.eat(":") { Some(self.ty()?) } else { None };
        let e = self.prev_end();
        Ok(mk(self, MKind::Field { readonly, key, optional, ty }, e))
    }
}

// -------------------------------------------------------------- walking

/// Where a node sits, for the rules that rewrite it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parent {
    Root,
    Func,
    GenericArg,
    ArrayElem,
    FieldValue,
    Paren,
    Tuple,
    Other,
}

/// Visit every type node, outermost first.
pub fn walk<'t>(t: &'t Ty, f: &mut dyn FnMut(&'t Ty, Parent)) {
    walk_in(t, Parent::Root, f)
}

fn walk_func<'t>(fun: &'t Func, f: &mut dyn FnMut(&'t Ty, Parent)) {
    for tp in &fun.tparams {
        if let Some(c) = &tp.constraint {
            walk_in(c, Parent::Other, f);
        }
        if let Some(d) = &tp.default {
            walk_in(d, Parent::Other, f);
        }
    }
    for p in &fun.params {
        if let Some(t) = &p.ty {
            walk_in(t, Parent::Func, f);
        }
    }
    if let Some(r) = &fun.ret {
        walk_in(r, Parent::Func, f);
    }
}

fn walk_in<'t>(t: &'t Ty, parent: Parent, f: &mut dyn FnMut(&'t Ty, Parent)) {
    f(t, parent);
    match &t.kind {
        Kind::Prefix(_, x) | Kind::Suffix(_, x) => walk_in(x, Parent::Other, f),
        Kind::Union(v) | Kind::Inter(v) => v.iter().for_each(|x| walk_in(x, Parent::Other, f)),
        Kind::Array(x) => walk_in(x, Parent::ArrayElem, f),
        Kind::Generic { base, args, .. } => {
            walk_in(base, Parent::Other, f);
            args.iter().for_each(|x| walk_in(x, Parent::GenericArg, f));
        }
        Kind::Object(ms) => {
            for m in ms {
                match &m.kind {
                    MKind::Field { ty, .. } => {
                        if let Some(x) = ty {
                            walk_in(x, Parent::FieldValue, f);
                        }
                    }
                    MKind::Method { f: fun, .. } | MKind::Call { f: fun } => walk_func(fun, f),
                    MKind::Verbatim(v) => v.iter().for_each(|x| walk_in(x, Parent::Other, f)),
                }
            }
        }
        Kind::Tuple(v) => v.iter().for_each(|x| walk_in(x, Parent::Tuple, f)),
        Kind::Func(fun) => walk_func(fun, f),
        Kind::Paren(x) => walk_in(x, Parent::Paren, f),
        Kind::Template { interps, .. } => interps.iter().for_each(|x| walk_in(x, Parent::Other, f)),
        Kind::Raw(v) => v.iter().for_each(|x| walk_in(x, Parent::Other, f)),
        Kind::Name(_) | Kind::Str(_) | Kind::Num(_) | Kind::Star | Kind::Unknown | Kind::Import(_) => {}
    }
}

// -------------------------------------------------------------- printer

fn is_plain_key(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(is_id_start) && cs.all(is_id_part)
}

pub struct Printer<'a> {
    /// The type's text.
    pub src: &'a str,
    /// Print children as written (for finding which node's own format is off).
    pub shallow: bool,
}

impl Printer<'_> {
    fn orig(&self, t: &Ty) -> &str {
        &self.src[t.start..t.end]
    }

    fn child(&self, t: &Ty) -> String {
        if self.shallow { self.orig(t).to_owned() } else { self.print(t) }
    }

    fn quote(&self, raw: &str) -> String {
        // Double quotes, unless the text holds one or an escape.
        if raw.starts_with('\'') {
            let inner = &raw[1..raw.len() - 1];
            if !inner.contains('"') && !inner.contains('\\') {
                return format!("\"{inner}\"");
            }
        }
        raw.to_owned()
    }

    fn key(&self, k: &Key) -> String {
        if k.raw.starts_with(['"', '\'']) {
            let inner = &k.raw[1..k.raw.len() - 1];
            if is_plain_key(inner) {
                return inner.to_owned();
            }
            return self.quote(&k.raw);
        }
        k.raw.clone()
    }

    fn needs_parens_in_array(t: &Ty) -> bool {
        matches!(&t.kind, Kind::Union(_) | Kind::Inter(_) | Kind::Func(_) | Kind::Prefix(..) | Kind::Suffix(..) | Kind::Unknown)
    }

    fn params(&self, ps: &[Param]) -> String {
        ps.iter().map(|p| self.param(p)).collect::<Vec<_>>().join(", ")
    }

    fn param(&self, p: &Param) -> String {
        let mut s = String::new();
        if p.rest {
            s.push_str("...");
        }
        match (&p.name, &p.ty) {
            (Some(n), Some(t)) => {
                let _ = write!(s, "{n}{}: {}", if p.optional { "?" } else { "" }, self.child(t));
            }
            (Some(n), None) => {
                let _ = write!(s, "{n}{}", if p.optional { "?" } else { "" });
            }
            (None, Some(t)) => s.push_str(&self.child(t)),
            (None, None) => {}
        }
        s
    }

    fn tparams(&self, tps: &[TParam]) -> String {
        if tps.is_empty() {
            return String::new();
        }
        let items: Vec<String> = tps
            .iter()
            .map(|tp| {
                let mut s = tp.name.clone();
                if let Some(c) = &tp.constraint {
                    let _ = write!(s, " extends {}", self.child(c));
                }
                if let Some(d) = &tp.default {
                    let _ = write!(s, " = {}", self.child(d));
                }
                s
            })
            .collect();
        format!("<{}>", items.join(", "))
    }

    /// `<T>(a: A): R` — a method or call signature.
    pub fn sig(&self, f: &Func) -> String {
        let mut s = format!("{}({})", self.tparams(&f.tparams), self.params(&f.params));
        if let Some(r) = &f.ret {
            let _ = write!(s, ": {}", self.child(r));
        }
        s
    }

    /// `<T>(a: A) => R`
    pub fn arrow_text(&self, f: &Func) -> String {
        let head = match f.style {
            Style::New => "new ",
            Style::AbstractNew => "abstract new ",
            _ => "",
        };
        let mut s = format!("{head}{}({})", self.tparams(&f.tparams), self.params(&f.params));
        if let Some(r) = &f.ret {
            let _ = write!(s, " => {}", self.child(r));
        }
        s
    }

    pub fn print(&self, t: &Ty) -> String {
        match &t.kind {
            Kind::Name(n) => n.clone(),
            Kind::Str(raw) => self.quote(raw),
            Kind::Num(n) => n.clone(),
            Kind::Star => "*".into(),
            Kind::Unknown => "?".into(),
            Kind::Prefix(op, x) => {
                let gap = if op.chars().all(|c| c.is_alphabetic()) { " " } else { "" };
                format!("{op}{gap}{}", self.child(x))
            }
            Kind::Suffix(c, x) => format!("{}{c}", self.child(x)),
            Kind::Union(v) => v.iter().map(|x| self.child(x)).collect::<Vec<_>>().join(" | "),
            Kind::Inter(v) => v.iter().map(|x| self.child(x)).collect::<Vec<_>>().join(" & "),
            Kind::Array(x) => {
                let inner = self.child(x);
                if Self::needs_parens_in_array(x) { format!("({inner})[]") } else { format!("{inner}[]") }
            }
            Kind::Generic { base, args, dot } => {
                // `Array<T>` is written `T[]`.
                if let (Kind::Name(n), 1) = (&base.kind, args.len())
                    && n == "Array"
                {
                    let inner = self.child(&args[0]);
                    return if Self::needs_parens_in_array(&args[0]) { format!("({inner})[]") } else { format!("{inner}[]") };
                }
                let _ = dot;
                format!("{}<{}>", self.child(base), args.iter().map(|x| self.child(x)).collect::<Vec<_>>().join(", "))
            }
            Kind::Object(ms) => {
                let items: Vec<String> = ms.iter().map(|m| self.member(m)).collect();
                format!("{{{}}}", items.join(", "))
            }
            Kind::Tuple(v) => format!("[{}]", v.iter().map(|x| self.child(x)).collect::<Vec<_>>().join(", ")),
            Kind::Func(f) => match f.style {
                Style::Function => format!("function{}", self.sig(f)),
                _ => self.arrow_text(f),
            },
            Kind::Paren(x) => format!("({})", self.child(x)),
            Kind::Template { .. } | Kind::Import(_) | Kind::Raw(_) => self.orig(t).to_owned(),
        }
    }

    fn member(&self, m: &Member) -> String {
        match &m.kind {
            MKind::Field { readonly, key, optional, ty } => {
                let mut s = String::new();
                if *readonly {
                    s.push_str("readonly ");
                }
                s.push_str(&self.key(key));
                if *optional {
                    s.push('?');
                }
                if let Some(t) = ty {
                    let _ = write!(s, ": {}", self.child(t));
                }
                s
            }
            MKind::Method { key, optional, f } => format!("{}{}{}", self.key(key), if *optional { "?" } else { "" }, self.sig(f)),
            MKind::Call { f } => {
                if f.style == Style::New { format!("new {}", self.sig(f)) } else { self.sig(f) }
            }
            MKind::Verbatim(_) => self.src[m.start..m.end].to_owned(),
        }
    }
}

/// The message for the first node whose own format differs from the default.
pub fn first_format_issue(t: &Ty, src: &str) -> Option<String> {
    let pr = Printer { src, shallow: true };
    let mut found: Option<String> = None;
    walk(t, &mut |n, _| {
        if found.is_some() {
            return;
        }
        let own = pr.print(n);
        if own == src[n.start..n.end] {
            return;
        }
        found = Some(match &n.kind {
            Kind::Union(_) => "Inconsistent \" \" union spacing usage".to_owned(),
            Kind::Generic { base, dot, .. } if matches!(&base.kind, Kind::Name(x) if x == "Array") => {
                if *dot { "Dot usage should be false".to_owned() } else { "Array bracket style should be square".to_owned() }
            }
            Kind::Generic { dot: true, .. } => "Dot usage should be false".to_owned(),
            Kind::Generic { .. } | Kind::Tuple(_) => "Element spacing should be \" \"".to_owned(),
            Kind::Str(_) => "Inconsistent double string quotes usage".to_owned(),
            Kind::Object(ms) => {
                let text = &src[n.start..n.end];
                if text.contains(';') || ms.iter().any(|_| false) {
                    "Inconsistent comma separator usage".to_owned()
                } else {
                    "There was an error with type formatting".to_owned()
                }
            }
            Kind::Func(f) => {
                if f.style == Style::Arrow || f.style == Style::New || f.style == Style::AbstractNew {
                    "Post-return-marker spacing should be \" \"".to_owned()
                } else {
                    "Parameter spacing should be \" \"".to_owned()
                }
            }
            _ => "There was an error with type formatting".to_owned(),
        });
    });
    found
}

//! SPDX licence expressions (W-129; SR-RULE-PACKS decision 15): `MIT`,
//! `(MIT OR Apache-2.0)`, `GPL-2.0-only WITH Classpath-exception-2.0`,
//! `Apache-2.0 AND BSD-3-Clause`. Codelaw's own parser — no new dependency
//! (L11). `AND` binds tighter than `OR`; parentheses group; `WITH` keeps
//! its exception with its id; ids compare without regard to case.
//!
//! A licence that is not an expression — free text such as `BSD` or `see
//! LICENSE` — is never guessed at: it is `unknown`. The one exception is a
//! closed table of exact texts that name one SPDX id without doubt (PyPI's
//! trove classifiers, `Apache 2.0`), [`ALIASES`].

/// A parsed expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    /// A licence id (`MIT`, `GPL-2.0+`, `LicenseRef-x`), with its `WITH`
    /// exception when it has one.
    Id { id: String, with: Option<String> },
    And(Vec<Expr>),
    Or(Vec<Expr>),
}

/// How `[deps.licence]` judges one licence, worst last.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verdict {
    Allowed,
    Unknown,
    Denied,
}

/// Exact texts that name one SPDX id without doubt — the trove
/// classifiers `deps refresh` keeps for a PyPI package with no
/// `license_expression` (their last segment), and the few spellings
/// registries hold most. Compared without regard to case. A text not here
/// and not an expression is `unknown`.
pub const ALIASES: &[(&str, &str)] = &[
    ("MIT License", "MIT"),
    ("The MIT License", "MIT"),
    ("ISC License (ISCL)", "ISC"),
    ("Apache 2.0", "Apache-2.0"),
    ("Apache 2", "Apache-2.0"),
    ("Apache License 2.0", "Apache-2.0"),
    ("Apache License, Version 2.0", "Apache-2.0"),
    ("Apache Software License 2.0", "Apache-2.0"),
    ("BSD 3-Clause", "BSD-3-Clause"),
    ("BSD-3", "BSD-3-Clause"),
    ("BSD 2-Clause", "BSD-2-Clause"),
    ("The Unlicense (Unlicense)", "Unlicense"),
    ("CC0 1.0 Universal (CC0 1.0) Public Domain Dedication", "CC0-1.0"),
    ("Python Software Foundation License", "PSF-2.0"),
    ("Mozilla Public License 2.0 (MPL 2.0)", "MPL-2.0"),
    ("Zero-Clause BSD (0BSD)", "0BSD"),
    ("Blue Oak Model License (BlueOak-1.0.0)", "BlueOak-1.0.0"),
    ("GNU General Public License v2 (GPLv2)", "GPL-2.0-only"),
    ("GNU General Public License v2 or later (GPLv2+)", "GPL-2.0-or-later"),
    ("GNU General Public License v3 (GPLv3)", "GPL-3.0-only"),
    ("GNU General Public License v3 or later (GPLv3+)", "GPL-3.0-or-later"),
    ("GNU Lesser General Public License v2 (LGPLv2)", "LGPL-2.0-only"),
    ("GNU Lesser General Public License v2 or later (LGPLv2+)", "LGPL-2.0-or-later"),
    ("GNU Lesser General Public License v3 (LGPLv3)", "LGPL-3.0-only"),
    ("GNU Lesser General Public License v3 or later (LGPLv3+)", "LGPL-3.0-or-later"),
    ("GNU Affero General Public License v3", "AGPL-3.0-only"),
    ("GNU Affero General Public License v3 or later (AGPLv3+)", "AGPL-3.0-or-later"),
    ("Server Side Public License (SSPL)", "SSPL-1.0"),
    ("European Union Public Licence 1.2 (EUPL 1.2)", "EUPL-1.2"),
];

/// The licences allowed when `[deps.licence] allow` is not set: the
/// permissive ones.
pub const DEFAULT_ALLOW: &[&str] = &["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "0BSD", "Unlicense", "CC0-1.0", "PSF-2.0", "Python-2.0", "BlueOak-1.0.0"];

/// The licences denied when `[deps.licence] deny` is not set: copyleft and
/// source-available.
pub const DEFAULT_DENY: &[&str] = &["GPL-*", "AGPL-*", "LGPL-*", "SSPL-*", "BUSL-*", "EUPL-*"];

fn tokens(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in text.chars() {
        match c {
            '(' | ')' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                out.push(c.to_string());
            }
            c if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// A licence id or exception: letters, digits, `-`, `.`, a final `+`.
fn is_id(t: &str) -> bool {
    let body = t.strip_suffix('+').unwrap_or(t);
    !body.is_empty() && body.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.')) && !matches!(t.to_ascii_uppercase().as_str(), "AND" | "OR" | "WITH")
}

struct Parser {
    toks: Vec<String>,
    at: usize,
}

impl Parser {
    fn peek_op(&self, op: &str) -> bool {
        self.toks.get(self.at).is_some_and(|t| t.eq_ignore_ascii_case(op))
    }
    fn or(&mut self) -> Option<Expr> {
        let mut parts = vec![self.and()?];
        while self.peek_op("OR") {
            self.at += 1;
            parts.push(self.and()?);
        }
        Some(if parts.len() == 1 { parts.pop()? } else { Expr::Or(parts) })
    }
    fn and(&mut self) -> Option<Expr> {
        let mut parts = vec![self.atom()?];
        while self.peek_op("AND") {
            self.at += 1;
            parts.push(self.atom()?);
        }
        Some(if parts.len() == 1 { parts.pop()? } else { Expr::And(parts) })
    }
    fn atom(&mut self) -> Option<Expr> {
        let t = self.toks.get(self.at)?.clone();
        self.at += 1;
        if t == "(" {
            let e = self.or()?;
            (self.toks.get(self.at)? == ")").then_some(())?;
            self.at += 1;
            return Some(e);
        }
        if !is_id(&t) {
            return None;
        }
        let mut with = None;
        if self.peek_op("WITH") {
            self.at += 1;
            let w = self.toks.get(self.at)?.clone();
            if !is_id(&w) {
                return None;
            }
            self.at += 1;
            with = Some(w);
        }
        Some(Expr::Id { id: t, with })
    }
}

/// Parse a licence as the snapshot keeps it; `None` for free text. An
/// [`ALIASES`] text is its id; the classifiers `deps refresh` joins with
/// ` OR ` are read one by one.
pub fn parse(text: &str) -> Option<Expr> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    if let Some((_, id)) = ALIASES.iter().find(|(a, _)| a.eq_ignore_ascii_case(t)) {
        return Some(Expr::Id { id: (*id).to_owned(), with: None });
    }
    let mut p = Parser { toks: tokens(t), at: 0 };
    if let Some(e) = p.or().filter(|_| p.at == p.toks.len()) {
        return Some(e);
    }
    // `MIT License OR BSD License`: classifiers, each an alias.
    if t.contains(" OR ") {
        let parts = t.split(" OR ").map(parse).collect::<Option<Vec<_>>>()?;
        return Some(Expr::Or(parts));
    }
    None
}

/// Whether `glob` (`*` any run of characters) matches `text`, ignoring case.
pub fn glob(glob: &str, text: &str) -> bool {
    let g = glob.to_ascii_lowercase();
    let t = text.to_ascii_lowercase();
    let parts: Vec<&str> = g.split('*').collect();
    if parts.len() == 1 {
        return g == t;
    }
    let mut rest = t.as_str();
    for (i, p) in parts.iter().enumerate() {
        if i == 0 {
            match rest.strip_prefix(p) {
                Some(r) => rest = r,
                None => return false,
            }
        } else if i == parts.len() - 1 {
            return rest.ends_with(p);
        } else {
            match rest.find(p) {
                Some(at) => rest = &rest[at + p.len()..],
                None => return false,
            }
        }
    }
    true
}

/// The lists `[deps.licence]` judges by: the configured ones first, so a
/// configured `allow` lifts a default denial and a configured `deny` a
/// default allowance; then the defaults.
#[derive(Clone, Debug, Default)]
pub struct Lists {
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

impl Lists {
    /// One id's verdict. A `WITH` id is matched as `id WITH exception`
    /// first, then as its id.
    pub fn id(&self, id: &str, with: Option<&str>) -> Verdict {
        let names: Vec<String> = match with {
            Some(w) => vec![format!("{id} WITH {w}"), id.to_owned()],
            None => vec![id.to_owned()],
        };
        let hit = |list: &[String]| names.iter().any(|n| list.iter().any(|g| glob(g, n)));
        let hit_static = |list: &[&str]| names.iter().any(|n| list.iter().any(|g| glob(g, n)));
        if hit(&self.deny) {
            Verdict::Denied
        } else if hit(&self.allow) {
            Verdict::Allowed
        } else if hit_static(DEFAULT_DENY) {
            Verdict::Denied
        } else if hit_static(DEFAULT_ALLOW) {
            Verdict::Allowed
        } else {
            Verdict::Unknown
        }
    }

    /// An expression's verdict: `OR` takes its best alternative (one
    /// allowed is enough), `AND` its worst (all must be).
    pub fn judge(&self, e: &Expr) -> Verdict {
        match e {
            Expr::Id { id, with } => self.id(id, with.as_deref()),
            Expr::Or(parts) => parts.iter().map(|p| self.judge(p)).min().unwrap_or(Verdict::Unknown),
            Expr::And(parts) => parts.iter().map(|p| self.judge(p)).max().unwrap_or(Verdict::Unknown),
        }
    }

    /// A licence text's verdict: free text is `Unknown`.
    pub fn text(&self, text: &str) -> Verdict {
        parse(text).map(|e| self.judge(&e)).unwrap_or(Verdict::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expressions_parse() {
        assert_eq!(parse("MIT"), Some(Expr::Id { id: "MIT".into(), with: None }));
        let e = parse("(MIT OR Apache-2.0) AND BSD-3-Clause").unwrap();
        assert!(matches!(e, Expr::And(ref p) if p.len() == 2 && matches!(p[0], Expr::Or(_))));
        assert!(matches!(parse("MIT or GPL-3.0-only"), Some(Expr::Or(_))));
        assert_eq!(parse("GPL-2.0-only WITH Classpath-exception-2.0"), Some(Expr::Id { id: "GPL-2.0-only".into(), with: Some("Classpath-exception-2.0".into()) }));
        assert_eq!(parse("MIT License"), Some(Expr::Id { id: "MIT".into(), with: None }));
        assert!(matches!(parse("MIT License OR BSD 3-Clause"), Some(Expr::Or(_))));
        for free in ["BSD", "see LICENSE file", "Apache Software License", "(MIT", "MIT AND", ""] {
            assert_eq!(Lists::default().text(free), Verdict::Unknown, "{free}");
        }
        assert!(parse("see LICENSE file").is_none() && parse("(MIT").is_none());
    }

    #[test]
    fn verdicts() {
        let d = Lists::default();
        assert_eq!(d.text("MIT"), Verdict::Allowed);
        assert_eq!(d.text("mit"), Verdict::Allowed);
        assert_eq!(d.text("GPL-3.0-only"), Verdict::Denied);
        assert_eq!(d.text("MIT OR GPL-3.0-only"), Verdict::Allowed);
        assert_eq!(d.text("MIT AND GPL-3.0-only"), Verdict::Denied);
        assert_eq!(d.text("MIT AND LicenseRef-Custom"), Verdict::Unknown);
        assert_eq!(d.text("MPL-2.0"), Verdict::Unknown);
        assert_eq!(d.text("UNLICENSED"), Verdict::Unknown);
        assert_eq!(d.text("GPL-2.0+"), Verdict::Denied);
        let mine = Lists { allow: vec!["LGPL-*".into(), "GPL-2.0-only WITH Classpath-exception-2.0".into()], deny: vec!["Unlicense".into()] };
        assert_eq!(mine.text("LGPL-3.0-only"), Verdict::Allowed);
        assert_eq!(mine.text("GPL-2.0-only WITH Classpath-exception-2.0"), Verdict::Allowed);
        assert_eq!(mine.text("GPL-2.0-only"), Verdict::Denied);
        assert_eq!(mine.text("Unlicense"), Verdict::Denied);
        assert!(glob("GPL-*", "gpl-3.0") && glob("*-or-later", "GPL-3.0-or-later") && !glob("GPL-*", "LGPL-3.0"));
    }
}

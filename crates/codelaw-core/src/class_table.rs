//! A provider's class table (SR-ARCHITECTURE decision 28a): what a class in
//! a class list is to the CSS framework the project uses — its place in the
//! framework's order, the properties it sets, its variants and its canonical
//! spelling — and the sort that orders a class list by it (SR-FORMATTER
//! decision 9).
//!
//! The table is built by a provider package from the framework installed in
//! the project; nothing here knows a class name. The engine implements
//! [`ClassTable`]; the formatter and the native rules only ask it.

use std::fmt;

/// What the table knows of one class.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassInfo {
    /// Its place in the framework's order; comparable only with the other
    /// answers of the same [`ClassTable::lookup`].
    pub order: u128,
    /// The CSS properties it sets, sorted.
    pub properties: Vec<String>,
    /// Its variants as written (`md:hover`), empty for none.
    pub variants: String,
    /// Whether it carries the important modifier.
    pub important: bool,
    /// The framework's canonical spelling, when it is not this one.
    pub canonical: Option<String>,
}

/// A provider's table, asked class by class.
pub trait ClassTable: Send + Sync {
    /// What the table was built from — the provider, the framework's version
    /// and a digest of the files it read. Every cache key a sorted or checked
    /// file enters holds it; two tables with one identity answer alike.
    fn identity(&self) -> &str;

    /// One answer per class, in order, all from one state of the table:
    /// `None` for a class the framework does not know. `Err` when the
    /// provider cannot answer.
    fn lookup(&self, classes: &[&str]) -> Result<Vec<Option<ClassInfo>>, String>;
}

impl fmt::Debug for dyn ClassTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClassTable({})", self.identity())
    }
}

/// How one class list is sorted: prettier-plugin-tailwindcss's options for
/// a string, a template part or an attribute value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SortOptions {
    /// The first word is glued to what comes before (a template part after
    /// `${…}` with no space): it stays where it is.
    pub ignore_first: bool,
    /// The last word is glued to what follows: it stays where it is.
    pub ignore_last: bool,
    /// Whitespace runs become one space, and leading/trailing whitespace is
    /// dropped at the ends that are not joined to something else.
    pub collapse: bool,
    pub collapse_start: bool,
    pub collapse_end: bool,
    /// A known class written twice keeps its first place only.
    pub remove_duplicates: bool,
}

impl Default for SortOptions {
    fn default() -> Self {
        SortOptions { ignore_first: false, ignore_last: false, collapse: true, collapse_start: true, collapse_end: true, remove_duplicates: true }
    }
}

fn is_ws(c: char) -> bool {
    matches!(c, '\t' | '\r' | '\u{c}' | '\n' | ' ')
}

/// Split into words and the whitespace runs between them, as
/// `str.split(/([\t\r\f\n ]+)/)` does: `words.len() == spaces.len() + 1`.
fn split(text: &str) -> (Vec<&str>, Vec<&str>) {
    let mut words = Vec::new();
    let mut spaces = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if is_ws(c) {
            let mut end = i + c.len_utf8();
            while let Some(&(j, d)) = chars.peek() {
                if !is_ws(d) {
                    break;
                }
                end = j + d.len_utf8();
                chars.next();
            }
            words.push(&text[start..i]);
            spaces.push(&text[i..end]);
            start = end;
        }
    }
    words.push(&text[start..]);
    (words, spaces)
}

/// Sort one class list by the table, as prettier-plugin-tailwindcss sorts
/// it: classes the framework does not know first, in their written order;
/// then the known ones by the framework's order (ties keep their written
/// order); `...` and `…` last. `Ok(None)` when nothing would change.
pub fn sort_classes(text: &str, table: &dyn ClassTable, o: SortOptions) -> Result<Option<String>, String> {
    if text.is_empty() || text.contains("{{") {
        return Ok(None);
    }
    if o.collapse && text.chars().all(is_ws) {
        return Ok(if text == " " { None } else { Some(" ".into()) });
    }
    let (mut classes, whitespace) = split(text);
    let mut whitespace: Vec<String> = whitespace.into_iter().map(|w| if o.collapse { " ".to_owned() } else { w.to_owned() }).collect();
    if classes.last() == Some(&"") {
        classes.pop();
    }
    let mut prefix = String::new();
    if o.ignore_first && !classes.is_empty() {
        prefix.push_str(classes.remove(0));
        if !whitespace.is_empty() {
            prefix.push_str(&whitespace.remove(0));
        }
    }
    let mut suffix = String::new();
    if o.ignore_last {
        let w = whitespace.pop().unwrap_or_default();
        let c = classes.pop().unwrap_or_default();
        suffix = format!("{w}{c}");
    }
    let infos = table.lookup(&classes)?;
    let mut ordered: Vec<(&str, Option<u128>)> = classes.iter().zip(&infos).map(|(c, i)| (*c, i.as_ref().map(|i| i.order))).collect();
    // Stable: equal keys keep their written order.
    ordered.sort_by_key(|(c, order)| {
        let ellipsis = *c == "..." || *c == "…";
        (ellipsis, order.is_some(), order.unwrap_or(0))
    });
    let mut removed = std::collections::BTreeSet::new();
    if o.remove_duplicates {
        let mut seen = std::collections::HashSet::new();
        let mut kept = Vec::with_capacity(ordered.len());
        for (index, (c, order)) in ordered.into_iter().enumerate() {
            if seen.contains(c) {
                removed.insert(index);
                continue;
            }
            if order.is_some() {
                seen.insert(c);
            }
            kept.push((c, order));
        }
        ordered = kept;
    }
    let whitespace: Vec<String> = whitespace.into_iter().enumerate().filter(|(i, _)| !removed.contains(&(i + 1))).map(|(_, w)| w).collect();
    let mut result = String::new();
    for (i, (c, _)) in ordered.iter().enumerate() {
        result.push_str(c);
        if let Some(w) = whitespace.get(i) {
            result.push_str(w);
        }
    }
    if o.collapse {
        prefix = trim_end_to_space(&prefix);
        suffix = trim_start_to_space(&suffix);
        let start_trimmed = result.trim_start_matches(char::is_whitespace);
        result = if start_trimmed.len() != result.len() { format!("{}{start_trimmed}", if o.collapse_start { "" } else { " " }) } else { result };
        let end_trimmed = result.trim_end_matches(char::is_whitespace);
        result = if end_trimmed.len() != result.len() { format!("{end_trimmed}{}", if o.collapse_end { "" } else { " " }) } else { result };
    }
    let out = format!("{prefix}{result}{suffix}");
    Ok(if out == text { None } else { Some(out) })
}

/// `s.replace(/\s+$/g, " ")`.
fn trim_end_to_space(s: &str) -> String {
    let t = s.trim_end_matches(char::is_whitespace);
    if t.len() == s.len() { s.to_owned() } else { format!("{t} ") }
}

/// `s.replace(/^\s+/g, " ")`.
fn trim_start_to_space(s: &str) -> String {
    let t = s.trim_start_matches(char::is_whitespace);
    if t.len() == s.len() { s.to_owned() } else { format!(" {t}") }
}

/// A fixed table, for tests: each known class's order is its place in the
/// list it was built from.
#[derive(Debug, Default)]
pub struct FixedTable {
    pub identity: String,
    pub classes: Vec<(String, ClassInfo)>,
}

impl FixedTable {
    /// Classes in the framework's order, each setting the given properties.
    pub fn ordered(classes: &[(&str, &[&str])]) -> FixedTable {
        let classes = classes
            .iter()
            .enumerate()
            .map(|(i, (c, props))| {
                let (variants, base) = match c.rfind(':') {
                    Some(at) => (c[..at].to_owned(), &c[at + 1..]),
                    None => (String::new(), *c),
                };
                let important = base.starts_with('!') || base.ends_with('!');
                (
                    (*c).to_owned(),
                    ClassInfo { order: i as u128, properties: props.iter().map(|p| (*p).to_owned()).collect(), variants, important, canonical: None },
                )
            })
            .collect();
        FixedTable { identity: "fixed".into(), classes }
    }
}

impl ClassTable for FixedTable {
    fn identity(&self) -> &str {
        &self.identity
    }

    fn lookup(&self, classes: &[&str]) -> Result<Vec<Option<ClassInfo>>, String> {
        Ok(classes.iter().map(|c| self.classes.iter().find(|(n, _)| n == c).map(|(_, i)| i.clone())).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> FixedTable {
        FixedTable::ordered(&[("block", &["display"]), ("flex", &["display"]), ("p-4", &["padding"]), ("px-2", &["padding-inline"]), ("md:p-2", &["padding"])])
    }

    fn sort(text: &str, o: SortOptions) -> String {
        sort_classes(text, &table(), o).unwrap().unwrap_or_else(|| text.to_owned())
    }

    #[test]
    fn unknown_classes_first_known_by_order_ellipsis_last() {
        assert_eq!(sort("md:p-2 ... foo p-4 flex bar", SortOptions::default()), "foo bar flex p-4 md:p-2 ...");
    }

    #[test]
    fn whitespace_collapses_and_known_duplicates_go() {
        assert_eq!(sort("  p-4\n\tflex   p-4 foo foo ", SortOptions::default()), "foo foo flex p-4");
        assert_eq!(sort_classes("   ", &table(), SortOptions::default()).unwrap().as_deref(), Some(" "));
        assert_eq!(sort_classes("flex p-4", &table(), SortOptions::default()).unwrap(), None);
    }

    #[test]
    fn template_parts_keep_the_words_glued_to_an_expression() {
        let o = SortOptions { ignore_first: true, ignore_last: true, collapse_start: false, collapse_end: false, ..SortOptions::default() };
        // `${a}-x p-4 flex b-${c}`: `-x` and `b-` stay put.
        assert_eq!(sort("-x p-4 flex b-", o), "-x flex p-4 b-");
    }

    #[test]
    fn uncollapsed_whitespace_is_kept_in_place() {
        let o = SortOptions { collapse: false, remove_duplicates: false, ..SortOptions::default() };
        assert_eq!(sort("p-4\n  flex", o), "flex\n  p-4");
    }

    #[test]
    fn templates_are_left_alone() {
        assert_eq!(sort_classes("p-4 {{ x }} flex", &table(), SortOptions::default()).unwrap(), None);
    }
}

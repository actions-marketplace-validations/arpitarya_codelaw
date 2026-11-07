//! The text filters a capture may carry in a message or fix template
//! (SR-RULE-FORMAT decision 3b): `$NAME|trim`, `$NAME|map(==:===)`, …
//!
//! A closed set, each a pure function of the capture's text. A filter that
//! can't produce a value refuses (`Err`): in a fix, the fix is then not
//! offered rather than half-written (W-162; SR-LAW-4 decision 2).

/// Every filter, by name. A `|` followed by anything else is plain text.
const FILTERS: &[&str] = &["trim", "lower-first", "upper-first", "lower", "upper", "map", "number", "escape", "unescape-octal", "count"];

/// The next filter at the start of `rest`: its name, its argument (the
/// text inside `(…)`, if any) and the bytes it spans. `None` when `rest`
/// does not start with a filter.
pub(crate) fn next_filter(rest: &str) -> Option<(&str, Option<&str>, usize)> {
    let after_bar = rest.strip_prefix('|')?;
    let len = after_bar.bytes().take_while(|b| b.is_ascii_lowercase() || *b == b'-').count();
    let name = &after_bar[..len];
    // The longest known name that the text starts with and that is not
    // followed by more of a word: `|trimmed` is text, not `|trim`.
    let name = FILTERS.iter().copied().filter(|f| name == *f).max_by_key(|f| f.len())?;
    let tail = &after_bar[name.len()..];
    if tail.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return None;
    }
    if let Some(args) = tail.strip_prefix('(') {
        let close = closing_paren(args)?;
        return Some((name, Some(&args[..close]), 1 + name.len() + 1 + close + 1));
    }
    Some((name, None, 1 + name.len()))
}

/// The index of the `)` that closes an argument list, skipping `\`-escaped
/// characters.
fn closing_paren(args: &str) -> Option<usize> {
    let mut escaped = false;
    for (i, c) in args.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            ')' => return Some(i),
            _ => {}
        }
    }
    None
}

/// Split on an unescaped separator, unescaping `\x` to `x` in each part.
fn split_unescaped(s: &str, sep: char, limit: usize) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(n) = chars.next() {
                // Keep the escape for the next split level.
                parts.last_mut().expect("one part").extend(['\\', n]);
            }
        } else if c == sep && parts.len() < limit {
            parts.push(String::new());
        } else {
            parts.last_mut().expect("one part").push(c);
        }
    }
    parts
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(n) = chars.next() {
                out.push(n);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Apply one filter to a capture's text.
pub fn apply_filter(name: &str, arg: Option<&str>, v: &str) -> Result<String, String> {
    let need = |what: &str| arg.ok_or_else(|| format!("`|{name}` takes ({what})"));
    match name {
        "trim" => Ok(v.trim().to_owned()),
        "lower-first" => {
            let mut cs = v.chars();
            Ok(match cs.next() {
                Some(c) => c.to_lowercase().chain(cs).collect(),
                None => String::new(),
            })
        }
        "upper-first" => {
            let mut cs = v.chars();
            Ok(match cs.next() {
                Some(c) => c.to_uppercase().chain(cs).collect(),
                None => String::new(),
            })
        }
        "lower" => Ok(v.to_lowercase()),
        "upper" => Ok(v.to_uppercase()),
        "map" => map(need("key:value, …")?, v),
        "number" => number(need("strip-zero-fraction | group-digits | decimal")?.trim(), v),
        "escape" => escape(need("template | js-string | css-ident")?.trim(), v),
        "unescape-octal" => unescape_octal(v),
        "count" => Ok(v.matches(unescape(need("text")?).as_str()).count().to_string()),
        other => Err(format!("unknown filter `|{other}`")),
    }
}

/// `|map(k:v, …)`: the value for the entry whose key is the whole text.
fn map(arg: &str, v: &str) -> Result<String, String> {
    for entry in split_unescaped(arg, ',', usize::MAX) {
        let kv = split_unescaped(entry.trim_start(), ':', 2);
        let [k, to] = kv.as_slice() else { return Err(format!("`|map` entry `{entry}` is not `key:value`")) };
        if unescape(k) == v {
            return Ok(unescape(to));
        }
    }
    Err(format!("`|map` has no entry for `{v}`"))
}

/// `|number(…)` on a JavaScript or Python number literal.
fn number(mode: &str, v: &str) -> Result<String, String> {
    let not = || format!("`|number({mode})`: `{v}` is not a literal it rewrites");
    match mode {
        "strip-zero-fraction" => {
            let (int, frac) = v.split_once('.').ok_or_else(not)?;
            if !int.bytes().all(|b| b.is_ascii_digit() || b == b'_') || !frac.bytes().all(|b| b.is_ascii_digit() || b == b'_') {
                return Err(not());
            }
            let frac = frac.trim_end_matches(['0', '_']);
            let int = if int.is_empty() { "0" } else { int };
            Ok(if frac.is_empty() { int.to_owned() } else { format!("{int}.{frac}") })
        }
        "group-digits" => {
            let digits: String = v.chars().filter(|c| *c != '_').collect();
            if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                return Err(not());
            }
            let n = digits.len();
            let mut out = String::with_capacity(n + n / 3);
            for (i, c) in digits.chars().enumerate() {
                if i > 0 && (n - i).is_multiple_of(3) {
                    out.push('_');
                }
                out.push(c);
            }
            Ok(out)
        }
        "decimal" => decimal(v).ok_or_else(not),
        _ => Err(format!("`|number` takes strip-zero-fraction, group-digits or decimal, not `{mode}`")),
    }
}

/// The plain decimal form of a number literal, as JavaScript would print
/// it as a property key. `None` when the value is not exactly an integer
/// below 2^53 or a plain decimal fraction.
fn decimal(v: &str) -> Option<String> {
    let s: String = v.chars().filter(|c| *c != '_').collect();
    let radix = |p: &str, r: u32| u128::from_str_radix(p, r).ok().filter(|n| *n < (1u128 << 53)).map(|n| n.to_string());
    let lower = s.to_ascii_lowercase();
    if let Some(h) = lower.strip_prefix("0x") {
        return radix(h, 16);
    }
    if let Some(o) = lower.strip_prefix("0o") {
        return radix(o, 8);
    }
    if let Some(b) = lower.strip_prefix("0b") {
        return radix(b, 2);
    }
    if lower.len() > 1 && lower.starts_with('0') && lower.bytes().all(|b| b.is_ascii_digit()) {
        // A legacy octal literal, unless a digit says it is decimal.
        return if lower.bytes().all(|b| b < b'8') { radix(&lower[1..], 8) } else { radix(lower.trim_start_matches('0'), 10).or(Some("0".into())) };
    }
    if let Some((mant, exp)) = lower.split_once('e') {
        let exp: i32 = exp.parse().ok()?;
        let (int, frac) = mant.split_once('.').unwrap_or((mant, ""));
        let digits = format!("{int}{frac}");
        let shift = exp - frac.len() as i32;
        if shift < 0 || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let n: u128 = digits.parse::<u128>().ok()?.checked_mul(10u128.checked_pow(shift as u32)?)?;
        return (n < (1u128 << 53)).then(|| n.to_string());
    }
    let (int, frac) = lower.split_once('.').unwrap_or((&lower, ""));
    if !int.bytes().all(|b| b.is_ascii_digit()) || !frac.bytes().all(|b| b.is_ascii_digit()) || (int.is_empty() && frac.is_empty()) {
        return None;
    }
    let int = int.trim_start_matches('0');
    let int = if int.is_empty() { "0" } else { int };
    let frac = frac.trim_end_matches('0');
    Some(if frac.is_empty() { int.to_owned() } else { format!("{int}.{frac}") })
}

/// `|escape(…)`: the text, as written in source, made safe for another
/// context. Escape sequences already in the text are kept as they are.
fn escape(mode: &str, v: &str) -> Result<String, String> {
    match mode {
        // A string literal's contents, into a template literal's.
        "template" => {
            let mut out = String::with_capacity(v.len());
            let mut chars = v.chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => {
                        out.push(c);
                        if let Some(n) = chars.next() {
                            out.push(n);
                        }
                    }
                    '`' => out.push_str("\\`"),
                    '$' if chars.peek() == Some(&'{') => out.push_str("\\$"),
                    _ => out.push(c),
                }
            }
            Ok(out)
        }
        // Plain text, into a JavaScript string literal of either quote.
        "js-string" => {
            let mut out = String::with_capacity(v.len());
            for c in v.chars() {
                match c {
                    '\\' | '\'' | '"' => {
                        out.push('\\');
                        out.push(c);
                    }
                    '\n' => out.push_str("\\n"),
                    _ => out.push(c),
                }
            }
            Ok(out)
        }
        // An id or class name, into a CSS selector. A name holding an
        // escape sequence of its source string is refused: its value is not
        // its text.
        "css-ident" => {
            if v.is_empty() || v.contains('\\') {
                return Err(format!("`|escape(css-ident)`: `{v}` is not a plain name"));
            }
            let mut out = String::with_capacity(v.len());
            for (i, c) in v.chars().enumerate() {
                if i == 0 && c.is_ascii_digit() {
                    out.push_str(&format!("\\{:x} ", c as u32));
                } else if c.is_ascii_alphanumeric() || c == '-' || c == '_' || !c.is_ascii() {
                    out.push(c);
                } else {
                    out.push('\\');
                    out.push(c);
                }
            }
            Ok(out)
        }
        _ => Err(format!("`|escape` takes template, js-string or css-ident, not `{mode}`")),
    }
}

/// `|unescape-octal`: every octal escape in a string literal's source
/// (`\251`, `\05`) as the `\u00XX` escape of the same character. `\0` not
/// followed by a digit is the NUL escape, not octal, and stays.
fn unescape_octal(v: &str) -> Result<String, String> {
    let b = v.as_bytes();
    let mut out = String::with_capacity(v.len());
    let mut i = 0;
    let mut changed = false;
    while i < b.len() {
        if b[i] == b'\\' && i + 1 < b.len() {
            let start = i + 1;
            let first = b[start];
            if (b'0'..=b'7').contains(&first) {
                let max = if first <= b'3' { 3 } else { 2 };
                let len = b[start..].iter().take(max).take_while(|d| (b'0'..=b'7').contains(*d)).count();
                if !(first == b'0' && len == 1 && !b.get(start + 1).is_some_and(u8::is_ascii_digit)) {
                    let value = u32::from_str_radix(&v[start..start + len], 8).expect("octal digits");
                    out.push_str(&format!("\\u{value:04X}"));
                    i = start + len;
                    changed = true;
                    continue;
                }
            }
            out.push_str(&v[i..i + 1]);
            let ch = v[start..].chars().next().expect("in bounds");
            out.push(ch);
            i = start + ch.len_utf8();
            continue;
        }
        let ch = v[i..].chars().next().expect("in bounds");
        out.push(ch);
        i += ch.len_utf8();
    }
    if changed { Ok(out) } else { Err(format!("`|unescape-octal`: `{v}` has no octal escape")) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(name: &str, arg: Option<&str>, v: &str) -> Result<String, String> {
        apply_filter(name, arg, v)
    }

    #[test]
    fn filters_are_recognised_only_as_whole_names() {
        assert_eq!(next_filter("|trim rest"), Some(("trim", None, 5)));
        assert_eq!(next_filter("|map(a:b) x"), Some(("map", Some("a:b"), 9)));
        assert_eq!(next_filter("|map(a\\):b)"), Some(("map", Some("a\\):b"), 11)));
        assert_eq!(next_filter("|trimmed"), None);
        assert_eq!(next_filter("| x"), None);
        assert_eq!(next_filter("|map(open"), None);
    }

    #[test]
    fn map_takes_whole_values_and_refuses_the_rest() {
        assert_eq!(f("map", Some("==:===, !=:!=="), "!="), Ok("!==".into()));
        assert_eq!(f("map", Some("==:===, !=:!=="), "=="), Ok("===".into()));
        assert!(f("map", Some("==:==="), "<").is_err());
        assert_eq!(f("map", Some("a\\,b:c, x:"), "a,b"), Ok("c".into()));
        assert_eq!(f("map", Some("a\\,b:c, x:"), "x"), Ok("".into()));
    }

    #[test]
    fn number_rewrites_literals() {
        assert_eq!(f("number", Some("strip-zero-fraction"), "10.0"), Ok("10".into()));
        assert_eq!(f("number", Some("strip-zero-fraction"), "10."), Ok("10".into()));
        assert_eq!(f("number", Some("strip-zero-fraction"), "1.50"), Ok("1.5".into()));
        assert!(f("number", Some("strip-zero-fraction"), "0x1").is_err());
        assert_eq!(f("number", Some("group-digits"), "1000000"), Ok("1_000_000".into()));
        assert_eq!(f("number", Some("group-digits"), "10000"), Ok("10_000".into()));
        assert_eq!(f("number", Some("decimal"), "0x10"), Ok("16".into()));
        assert_eq!(f("number", Some("decimal"), "0b101"), Ok("5".into()));
        assert_eq!(f("number", Some("decimal"), "1e3"), Ok("1000".into()));
        assert_eq!(f("number", Some("decimal"), "1.5e1"), Ok("15".into()));
        assert_eq!(f("number", Some("decimal"), ".5"), Ok("0.5".into()));
        assert_eq!(f("number", Some("decimal"), "5."), Ok("5".into()));
        assert_eq!(f("number", Some("decimal"), "1_000"), Ok("1000".into()));
        assert_eq!(f("number", Some("decimal"), "010"), Ok("8".into()));
        assert_eq!(f("number", Some("decimal"), "09"), Ok("9".into()));
        assert!(f("number", Some("decimal"), "1e-3").is_err());
        assert!(f("number", Some("decimal"), "1e400").is_err());
    }

    #[test]
    fn escape_keeps_existing_escapes() {
        assert_eq!(f("escape", Some("template"), "a`b${c}\\`"), Ok("a\\`b\\${c}\\`".into()));
        assert_eq!(f("escape", Some("css-ident"), "app"), Ok("app".into()));
        assert_eq!(f("escape", Some("css-ident"), "a.b"), Ok("a\\.b".into()));
        assert_eq!(f("escape", Some("css-ident"), "1a"), Ok("\\31 a".into()));
        assert!(f("escape", Some("css-ident"), "a\\nb").is_err());
        assert_eq!(f("escape", Some("js-string"), "a\\.b'"), Ok("a\\\\.b\\'".into()));
    }

    #[test]
    fn unescape_octal_writes_unicode_escapes() {
        assert_eq!(f("unescape-octal", None, "'Copyright \\251'"), Ok("'Copyright \\u00A9'".into()));
        assert_eq!(f("unescape-octal", None, "'\\0\\05\\\\1'"), Ok("'\\0\\u0005\\\\1'".into()));
        assert!(f("unescape-octal", None, "'\\0'").is_err());
    }

    #[test]
    fn count_counts_occurrences() {
        assert_eq!(f("count", Some(","), "[, , x]"), Ok("2".into()));
    }
}

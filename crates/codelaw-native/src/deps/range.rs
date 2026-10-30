//! Whole npm ranges, as `engines.node` and a monorepo's workspaces write
//! them (W-129): `>=18 <21`, `^18 || ^20`, `18.x`, `1.2 - 1.4`, `*`. Each
//! `||` alternative is a comparator set; a version is allowed when every
//! comparator of one set holds it. Prereleases are compared plainly (the
//! rules that read this compare runtime pins and range floors, never
//! prerelease targets). No new dependency (L11): semver's own `VersionReq`
//! has no `||`, hyphen or x-range.

use semver::Version;

/// One comparator: an operator and a version.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Cmp {
    op: &'static str,
    v: Version,
}

impl Cmp {
    fn holds(&self, v: &Version) -> bool {
        match self.op {
            ">=" => v >= &self.v,
            ">" => v > &self.v,
            "<=" => v <= &self.v,
            "<" => v < &self.v,
            _ => v == &self.v,
        }
    }
}

/// A parsed npm range: `||` alternatives of comparator sets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NpmRange {
    sets: Vec<Vec<Cmp>>,
}

/// A partial version: one to three numbers, or a wildcard part (`x`, `*`)
/// that ends it.
fn partial(text: &str) -> Option<Vec<u64>> {
    let t = text.trim().trim_start_matches(['v', '=']).trim();
    if t.is_empty() || matches!(t, "*" | "x" | "X") {
        return Some(Vec::new());
    }
    let core = t.split(['-', '+']).next().unwrap_or(t);
    let mut out = Vec::new();
    for p in core.split('.') {
        if matches!(p, "x" | "X" | "*") {
            break;
        }
        out.push(p.parse::<u64>().ok()?);
    }
    (out.len() <= 3).then_some(out)
}

fn at(parts: &[u64]) -> Version {
    Version::new(parts.first().copied().unwrap_or(0), parts.get(1).copied().unwrap_or(0), parts.get(2).copied().unwrap_or(0))
}

/// The exclusive bound one past `parts` at its last written component.
fn bump(parts: &[u64], i: usize) -> Version {
    let mut p = [parts.first().copied().unwrap_or(0), parts.get(1).copied().unwrap_or(0), parts.get(2).copied().unwrap_or(0)];
    p[i] += 1;
    for x in p.iter_mut().skip(i + 1) {
        *x = 0;
    }
    Version::new(p[0], p[1], p[2])
}

/// The comparators one token stands for (`^1.2`, `~1`, `>=2`, `1.x`).
fn token(op: &str, ver: &str, out: &mut Vec<Cmp>) -> Option<()> {
    let p = partial(ver)?;
    let lo = at(&p);
    match op {
        "^" => {
            if p.is_empty() {
                return Some(());
            }
            let first = p.iter().position(|x| *x != 0).unwrap_or(p.len() - 1);
            out.push(Cmp { op: ">=", v: lo });
            out.push(Cmp { op: "<", v: bump(&p, first.min(2)) });
        }
        "~" => {
            if p.is_empty() {
                return Some(());
            }
            out.push(Cmp { op: ">=", v: lo });
            out.push(Cmp { op: "<", v: bump(&p, if p.len() == 1 { 0 } else { 1 }) });
        }
        "" | "=" => match p.len() {
            0 => {}
            3 => out.push(Cmp { op: "=", v: lo }),
            n => {
                out.push(Cmp { op: ">=", v: lo });
                out.push(Cmp { op: "<", v: bump(&p, n - 1) });
            }
        },
        ">=" => out.push(Cmp { op: ">=", v: lo }),
        ">" => match p.len() {
            0 => out.push(Cmp { op: "<", v: Version::new(0, 0, 0) }),
            3 => out.push(Cmp { op: ">", v: lo }),
            n => out.push(Cmp { op: ">=", v: bump(&p, n - 1) }),
        },
        "<" => out.push(Cmp { op: "<", v: lo }),
        "<=" => match p.len() {
            0 | 3 => out.push(Cmp { op: "<=", v: lo }),
            n => out.push(Cmp { op: "<", v: bump(&p, n - 1) }),
        },
        _ => return None,
    }
    Some(())
}

const OPS: [&str; 7] = [">=", "<=", "^", "~", ">", "<", "="];

fn set(text: &str) -> Option<Vec<Cmp>> {
    let t = text.trim();
    let mut out = Vec::new();
    // A hyphen range: `1.2 - 1.4` (spaces around the hyphen).
    if let Some((a, b)) = t.split_once(" - ") {
        let lo = partial(a)?;
        let hi = partial(b)?;
        out.push(Cmp { op: ">=", v: at(&lo) });
        match hi.len() {
            0 => {}
            3 => out.push(Cmp { op: "<=", v: at(&hi) }),
            n => out.push(Cmp { op: "<", v: bump(&hi, n - 1) }),
        }
        return Some(out);
    }
    let words: Vec<&str> = t.split_whitespace().collect();
    let mut i = 0;
    while i < words.len() {
        let w = words[i];
        let op = OPS.iter().find(|o| w.starts_with(**o)).copied().unwrap_or("");
        let mut ver = &w[op.len()..];
        // `>= 1.2`: the operator alone, its version the next word.
        if ver.is_empty() && !op.is_empty() {
            i += 1;
            ver = words.get(i)?;
        }
        token(op, ver, &mut out)?;
        i += 1;
    }
    Some(out)
}

impl NpmRange {
    /// `None` for what is not a version range: a tag (`latest`), a
    /// protocol (`workspace:`, `file:`, `npm:`, a git URL).
    pub fn parse(text: &str) -> Option<NpmRange> {
        let t = text.trim();
        if t.contains(':') || t.contains('/') {
            return None;
        }
        let sets = t.split("||").map(set).collect::<Option<Vec<_>>>()?;
        Some(NpmRange { sets })
    }

    pub fn allows(&self, v: &Version) -> bool {
        let plain = Version::new(v.major, v.minor, v.patch);
        self.sets.iter().any(|s| s.iter().all(|c| c.holds(&plain)))
    }

    /// The lowest version the range allows from below: the smallest lower
    /// bound of its sets (`0.0.0` for a set with none).
    pub fn floor(&self) -> Version {
        self.sets
            .iter()
            .map(|s| {
                s.iter()
                    .filter_map(|c| match c.op {
                        ">=" | "=" => Some(c.v.clone()),
                        ">" => Some(Version::new(c.v.major, c.v.minor, c.v.patch + 1)),
                        _ => None,
                    })
                    .max()
                    .unwrap_or(Version::new(0, 0, 0))
            })
            .min()
            .unwrap_or(Version::new(0, 0, 0))
    }
}

/// Whether a partial pin (`20`, `20.11`, `20.11.1`) can be inside `range`:
/// its lowest and its highest version are both tried, so `.nvmrc`'s `20`
/// satisfies `>=20.11`.
pub fn pin_allowed(range: &NpmRange, pin: &[u64]) -> bool {
    if range.allows(&at(pin)) {
        return true;
    }
    if pin.len() < 3 {
        let mut hi = [pin.first().copied().unwrap_or(0), pin.get(1).copied().unwrap_or(u32::MAX as u64), u32::MAX as u64];
        for (i, p) in pin.iter().enumerate() {
            hi[i] = *p;
        }
        return range.allows(&Version::new(hi[0], hi[1], hi[2]));
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn ranges_as_npm_reads_them() {
        let r = |s: &str| NpmRange::parse(s).unwrap_or_else(|| panic!("{s} parses"));
        assert!(r(">=18 <21").allows(&v("20.11.1")) && !r(">=18 <21").allows(&v("21.0.0")));
        assert!(r("^18 || ^20").allows(&v("20.1.0")) && !r("^18 || ^20").allows(&v("19.0.0")));
        assert!(r("18.x").allows(&v("18.9.9")) && !r("18.x").allows(&v("19.0.0")));
        assert!(r("1.2 - 1.4").allows(&v("1.4.9")) && !r("1.2 - 1.4").allows(&v("1.5.0")));
        assert!(r(">= 20.11.0").allows(&v("22.0.0")) && !r(">= 20.11.0").allows(&v("20.10.0")));
        assert!(r("*").allows(&v("0.0.1")) && r("").allows(&v("9.9.9")));
        assert!(r("^0.2.3").allows(&v("0.2.9")) && !r("^0.2.3").allows(&v("0.3.0")));
        assert_eq!(r("^18.2 || >=20").floor(), v("18.2.0"));
        assert_eq!(r(">1.2.3").floor(), v("1.2.4"));
        for bad in ["latest", "workspace:*", "file:../a", "npm:x@1", "git+https://x/y.git"] {
            assert!(NpmRange::parse(bad).is_none(), "{bad}");
        }
        assert!(pin_allowed(&r(">=20.11"), &[20]) && !pin_allowed(&r(">=21"), &[20]) && pin_allowed(&r("^20.11.0"), &[20, 11, 1]));
    }
}

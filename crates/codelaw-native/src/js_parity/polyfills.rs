//! W-175: eslint-plugin-unicorn v79.0.0 `no-unnecessary-polyfills`, written
//! from the rule's behaviour. A bare import of a polyfill — a core-js entry,
//! or a package named for the feature it adds (`array.prototype.flat`,
//! `es6-promise`, `polyfill-array-includes`) — is reported when every engine
//! the project supports already has the feature built in.
//!
//! The tables are the data package `@codelaw/data-polyfills` (L6 decision 15;
//! L11 D-53), asked for through `FileContext::data_table`: nothing is read
//! unless this rule is on and a file imports a package. The project's
//! targets are the rule's `targets` option, else the nearest `browserslist`
//! (`package.json` or `.browserslistrc`) whose queries spell versions out
//! (`chrome >= 100`), else `engines` of the nearest `package.json`.

use super::{JS, finding};
use crate::js_ast::arguments;
use crate::util::{named_children, text, unquote};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use tree_sitter::Node;

/// Polyfill packages that name a feature without saying it.
const ADDITIONAL: &[(&str, &str)] = &[("p-finally", "es.promise.finally"), ("setprototypeof", "es.object.set-prototype-of"), ("code-point-at", "es.string.code-point-at")];
/// Legacy aliases of one built-in: reported when that feature alone is available.
const DIRECT: &[&str] = &["es6-symbol", "promise-polyfill", "es6-promise", "weakmap-polyfill"];
const DELIMS: &[&str] = &["", ".", "-", ".prototype.", "/"];
const PREFIXES: &[&str] = &["", "polyfill-", "mdn-polyfills/"];
const SUFFIXES: &[&str] = &["", "-polyfill"];

fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut up = false;
    for c in s.chars() {
        if c == '-' || c == '_' {
            up = true;
        } else if up {
            out.push(c.to_ascii_uppercase());
            up = false;
        } else {
            out.push(c);
        }
    }
    out
}

fn variants(s: &str) -> Vec<String> {
    let mut v = vec![s.to_ascii_lowercase(), camel(s).to_ascii_lowercase()];
    v.dedup();
    v
}

/// The table, indexed for the names a polyfill goes by.
struct Index {
    value: Arc<Value>,
    /// The feature ids, in table order.
    features: Vec<String>,
    /// feature index -> (engine -> first version)
    since: Vec<BTreeMap<String, Vec<u64>>>,
    /// `constructor<d>method`, with no ecma part
    plain: HashMap<String, Vec<usize>>,
    /// `method` or `constructor`, for the prefix and suffix forms
    short: HashMap<String, Vec<usize>>,
    /// what follows the ecma part: `constructor<d>method`, or `constructor`
    after_ecma: HashMap<String, Vec<usize>>,
    additional: HashMap<String, usize>,
}

fn version(s: &str) -> Option<Vec<u64>> {
    s.split('.').map(|p| p.parse().ok()).collect()
}

fn at_least(a: &[u64], b: &[u64]) -> bool {
    for i in 0..a.len().max(b.len()) {
        let (x, y) = (a.get(i).copied().unwrap_or(0), b.get(i).copied().unwrap_or(0));
        if x != y {
            return x > y;
        }
    }
    true
}

fn index_of(table: &Arc<Value>) -> Option<Arc<Index>> {
    static CACHE: Mutex<Option<(usize, Arc<Index>)>> = Mutex::new(None);
    let key = Arc::as_ptr(table) as usize;
    let mut guard = CACHE.lock().ok()?;
    if let Some((k, i)) = guard.as_ref()
        && *k == key
    {
        return Some(i.clone());
    }
    let features: Vec<String> = table.get("features")?.as_array()?.iter().filter_map(|f| f.as_str().map(str::to_owned)).collect();
    let since: Vec<BTreeMap<String, Vec<u64>>> = table
        .get("since")?
        .as_array()?
        .iter()
        .map(|row| row.as_object().map(|o| o.iter().filter_map(|(e, v)| Some((e.clone(), version(v.as_str()?)?))).collect()).unwrap_or_default())
        .collect();
    if features.len() != since.len() {
        return None;
    }
    let mut ix = Index { value: table.clone(), features, since, plain: HashMap::new(), short: HashMap::new(), after_ecma: HashMap::new(), additional: HashMap::new() };
    for (i, f) in ix.features.clone().iter().enumerate() {
        let mut parts = f.splitn(4, '.');
        let (_, Some(ctor)) = (parts.next(), parts.next()) else { continue };
        let method = parts.next().unwrap_or("");
        for c in variants(ctor) {
            if method.is_empty() {
                ix.short.entry(c.clone()).or_default().push(i);
                ix.after_ecma.entry(c).or_default().push(i);
                continue;
            }
            for m in variants(method) {
                for d in DELIMS {
                    ix.plain.entry(format!("{c}{d}{m}")).or_default().push(i);
                    ix.after_ecma.entry(format!("{c}{d}{m}")).or_default().push(i);
                }
            }
            // `ecma d constructor` is a form of its own, whatever the method.
            ix.after_ecma.entry(c).or_default().push(i);
        }
        if !method.is_empty() {
            for m in variants(method) {
                ix.short.entry(m).or_default().push(i);
            }
        }
    }
    for (name, feature) in ADDITIONAL {
        if let Some(i) = ix.features.iter().position(|f| f == feature) {
            ix.additional.insert((*name).to_owned(), i);
        }
    }
    for v in ix.plain.values_mut().chain(ix.short.values_mut()).chain(ix.after_ecma.values_mut()) {
        v.sort_unstable();
        v.dedup();
    }
    let ix = Arc::new(ix);
    *guard = Some((key, ix.clone()));
    Some(ix)
}

impl Index {
    fn namespace(&self, i: usize) -> &str {
        self.features[i].split('.').next().unwrap_or("")
    }

    /// The feature a module is named for: the broadest, then the first.
    fn feature_of(&self, module: &str) -> Option<usize> {
        let m = module.to_ascii_lowercase();
        let mut found: Vec<usize> = Vec::new();
        if let Some(&i) = self.additional.get(&m) {
            found.push(i);
        }
        for p in PREFIXES {
            for s in SUFFIXES {
                let Some(core) = m.strip_prefix(p).and_then(|r| r.strip_suffix(s)).filter(|c| !c.is_empty()) else { continue };
                if let Some(v) = self.plain.get(core) {
                    found.extend(v);
                }
                if p.is_empty() != s.is_empty()
                    && let Some(v) = self.short.get(core)
                {
                    found.extend(v);
                }
                for end in 1..core.len().min(12) {
                    if !core.is_char_boundary(end) {
                        continue;
                    }
                    let (ecma, rest) = core.split_at(end);
                    for d in DELIMS {
                        let Some(after) = rest.strip_prefix(d) else { continue };
                        for &i in self.after_ecma.get(after).into_iter().flatten() {
                            let ns = self.namespace(i);
                            let ok = if ns == "es" { ecma.strip_prefix("es").is_some_and(|r| r.chars().all(|c| c.is_ascii_digit())) } else { ecma == ns };
                            if ok {
                                found.push(i);
                            }
                        }
                    }
                }
            }
        }
        found.sort_unstable();
        found.dedup();
        found.into_iter().min_by_key(|&i| (self.features[i].split('.').count(), i))
    }

    fn entry(&self, name: &str) -> Option<Vec<usize>> {
        let list = self.value.get("entries")?.get(name)?.as_array()?;
        Some(list.iter().filter_map(|x| x.as_u64().map(|x| x as usize)).collect())
    }

    /// Whether `feature` is missing from some target: the polyfill is needed.
    fn unavailable(&self, i: usize, targets: &[(String, Vec<u64>)]) -> bool {
        let row = &self.since[i];
        targets.iter().any(|(engine, v)| row.get(engine).is_none_or(|first| !at_least(v, first)))
    }

    /// Every feature of an entry is built in everywhere (an `esnext.` twin of
    /// an `es.` feature counts as missing when the `es.` one is).
    fn all_available(&self, features: &[usize], targets: &[(String, Vec<u64>)]) -> bool {
        features.iter().all(|&i| {
            if self.unavailable(i, targets) {
                return false;
            }
            let name = &self.features[i];
            match name.strip_prefix("esnext.") {
                Some(rest) => self.features.iter().position(|f| *f == format!("es.{rest}")).is_none_or(|j| !self.unavailable(j, targets)),
                None => true,
            }
        })
    }
}

// ---- targets ----------------------------------------------------------------------------------

fn engine_key(name: &str) -> Option<&'static str> {
    Some(match name {
        "chrome" | "and_chr" => "chrome",
        "edge" => "edge",
        "firefox" | "and_ff" => "firefox",
        "safari" => "safari",
        "ios_saf" | "ios" => "ios",
        "samsung" => "samsung",
        "opera" => "opera",
        "node" => "node",
        "deno" => "deno",
        "bun" => "bun",
        "electron" => "electron",
        "android" => "android",
        _ => return None,
    })
}

/// `chrome >= 100`, `node 18`, `safari>=15.4`: one engine and its oldest version.
fn query(q: &str) -> Option<(String, Vec<u64>)> {
    let q = q.trim().to_ascii_lowercase();
    let q = q.replace(">=", " ");
    let mut parts = q.split_whitespace();
    let (name, ver) = (parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    Some((engine_key(name)?.to_owned(), version(ver)?))
}

/// The lower bound of a semver range (`>=18`, `^18.2.0`, `~14`, `18`).
fn range(r: &str) -> Option<Vec<u64>> {
    let r = r.trim().trim_start_matches(['>', '=', '^', '~', 'v']).trim();
    version(r.split([' ', '|']).next()?.trim_end_matches(".x").trim_end_matches(".*"))
}

fn merge(out: &mut Vec<(String, Vec<u64>)>, engine: String, v: Vec<u64>) {
    match out.iter_mut().find(|(e, _)| *e == engine) {
        Some((_, old)) => {
            if !at_least(&v, old) {
                *old = v;
            }
        }
        None => out.push((engine, v)),
    }
}

fn targets(ctx: &NativeContext<'_>) -> Option<Vec<(String, Vec<u64>)>> {
    let mut out = Vec::new();
    if let Some(Value::Object(o)) = ctx.options.get("targets")
        && !o.is_empty()
    {
        for (e, v) in o {
            merge(&mut out, engine_key(e)?.to_owned(), range(v.as_str()?)?);
        }
        return Some(out);
    }
    // browserslist's own search: `.browserslistrc`, or `package.json`'s field, nearest first.
    let mut dir = ctx.file.path().rfind('/').map_or("", |i| &ctx.file.path()[..i]).to_owned();
    let mut engines: Option<Value> = None;
    loop {
        let at = |f: &str| if dir.is_empty() { f.to_owned() } else { format!("{dir}/{f}") };
        if let Some(rc) = ctx.file.read(&at(".browserslistrc")) {
            for line in rc.lines().map(|l| l.split('#').next().unwrap_or("").trim()).filter(|l| !l.is_empty()) {
                for q in line.split(',') {
                    let (e, v) = query(q)?;
                    merge(&mut out, e, v);
                }
            }
            return (!out.is_empty()).then_some(out);
        }
        if let Some(pkg) = ctx.file.read(&at("package.json")).and_then(|t| serde_json::from_str::<Value>(&t).ok()) {
            let list: Option<Vec<String>> = match pkg.get("browserslist") {
                Some(Value::Array(a)) => Some(a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect()),
                Some(Value::String(s)) => Some(s.split(',').map(str::to_owned).collect()),
                Some(Value::Object(o)) => o.get("production").and_then(Value::as_array).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect()),
                _ => None,
            };
            if let Some(list) = list {
                for q in list {
                    let (e, v) = query(&q)?;
                    merge(&mut out, e, v);
                }
                return (!out.is_empty()).then_some(out);
            }
            if engines.is_none() {
                engines = pkg.get("engines").cloned();
            }
        }
        if dir.is_empty() {
            break;
        }
        dir = dir.rfind('/').map_or("", |i| &dir[..i]).to_owned();
    }
    if let Some(Value::Object(o)) = engines {
        for (e, v) in &o {
            if let Some(k) = engine_key(e) {
                merge(&mut out, k.to_owned(), range(v.as_str()?)?);
            }
        }
    }
    (!out.is_empty()).then_some(out)
}

// ---- the rule ---------------------------------------------------------------------------------

/// The imports of a file: the string node, its text, and the statement a
/// bare one (`import "x"`, `require("x");`) can be deleted as.
fn imports<'t>(root: Node<'t>, src: &str) -> Vec<(Node<'t>, String, Option<Node<'t>>)> {
    let mut all = Vec::new();
    crate::util::descendants(root, &|_| false, &mut all);
    let mut out = Vec::new();
    for x in all {
        let found = match x.kind() {
            "import_statement" => x.child_by_field_name("source").map(|s| (s, named_children(x).iter().all(|c| c.kind() == "string" || c.kind() == "comment").then_some(x))),
            "call_expression" => {
                let f = x.child_by_field_name("function");
                let args = arguments(x);
                match (f.map(|f| (f.kind(), text(f, src))), &args[..]) {
                    (Some(("import", _)), [a, ..]) => Some((*a, None)),
                    (Some(("identifier", "require")), [a]) => Some((*a, x.parent().filter(|p| p.kind() == "expression_statement"))),
                    _ => None,
                }
            }
            _ => None,
        };
        if let Some((s, stmt)) = found.filter(|(s, _)| s.kind() == "string") {
            out.push((s, unquote(text(s, src)).to_owned(), stmt));
        }
    }
    out
}

pub struct NoUnnecessaryPolyfills;

impl NativeRule for NoUnnecessaryPolyfills {
    fn id(&self) -> &'static str {
        "js/no-unnecessary-polyfills"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn needs_semantic(&self) -> bool {
        false
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        // Without the data package there is nothing to judge by: the tree is not even walked.
        let Some(table) = ctx.file.data_table("polyfills") else { return };
        let wanted: Vec<_> = imports(n, ctx.source).into_iter().filter(|(_, m, _)| !m.is_empty() && !m.starts_with(['.', '/'])).collect();
        if wanted.is_empty() {
            return;
        }
        let Some(ix) = index_of(&table) else { return };
        let Some(targets) = targets(ctx) else { return };
        for (s, module, stmt) in wanted {
            let entry = module.replace("core-js-pure", "core-js");
            let entry = entry.strip_suffix(".js").unwrap_or(&entry);
            let message = if let Some(features) = ix.entry(entry) {
                if !ix.all_available(&features, &targets) {
                    continue;
                }
                if features.len() > 1 {
                    format!("Every feature `{module}` polyfills is built in to the project's targets: use the built-ins instead")
                } else {
                    format!("`{module}` polyfills a feature every target has built in: use it directly")
                }
            } else {
                let Some(i) = ix.feature_of(&module) else { continue };
                let direct = DIRECT.contains(&module.to_ascii_lowercase().as_str());
                let reported = if direct {
                    !ix.unavailable(i, &targets)
                } else {
                    let mut parts = ix.features[i].splitn(4, '.');
                    let (_, ns, method) = (parts.next(), parts.next().unwrap_or(""), parts.next().unwrap_or(""));
                    let full = format!("core-js/full/{ns}{}{method}", if method.is_empty() { "" } else { "/" });
                    ix.entry(&full).is_some_and(|f| ix.all_available(&f, &targets))
                };
                if !reported {
                    continue;
                }
                format!("`{module}` polyfills `{}`, which every target has built in: use the built-in instead", ix.features[i])
            };
            // A bare import has nothing bound: removing it is the whole fix.
            let edits = stmt.map(|st| delete_statement(ctx.source, st)).into_iter().collect();
            out.push(finding(s, message, edits));
        }
    }
}

fn delete_statement(src: &str, st: Node<'_>) -> TextEdit {
    let (a, mut b) = (st.start_byte(), st.end_byte());
    if src[b..].starts_with("\r\n") {
        b += 2;
    } else if src[b..].starts_with('\n') {
        b += 1;
    }
    TextEdit::delete(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Arc<Value> {
        Arc::new(serde_json::json!({
            "schema": 1, "version": "x", "engines": ["chrome", "node"],
            "features": ["es.array.flat", "es.array.flat-map", "es.promise", "es.promise.finally", "es.symbol", "esnext.array.at", "web.url-search-params"],
            "since": [{"chrome": "69", "node": "11.0"}, {"chrome": "69", "node": "11.0"}, {"chrome": "32", "node": "0.12"}, {"chrome": "63", "node": "10.0"}, {"chrome": "49", "node": "6.0"}, {}, {"chrome": "49", "node": "10.0"}],
            "entries": {"core-js/es/array/flat": [0], "core-js/es/promise": [2, 3]}
        }))
    }

    #[test]
    fn queries_and_ranges_read_the_oldest_version() {
        assert_eq!(query("chrome >= 100"), Some(("chrome".into(), vec![100])));
        assert_eq!(query("ios_saf 15.4"), Some(("ios".into(), vec![15, 4])));
        assert_eq!(query("defaults"), None);
        assert_eq!(query("last 2 chrome versions"), None);
        assert_eq!(range(">=18.2.0"), Some(vec![18, 2, 0]));
        assert_eq!(range("^20"), Some(vec![20]));
        let mut t = Vec::new();
        merge(&mut t, "chrome".into(), vec![100]);
        merge(&mut t, "chrome".into(), vec![90]);
        assert_eq!(t, vec![("chrome".to_owned(), vec![90])]);
    }

    #[test]
    fn a_module_is_named_for_the_feature_it_adds() {
        let ix = index_of(&table()).unwrap();
        let name = |m: &str| ix.feature_of(m).map(|i| ix.features[i].clone());
        assert_eq!(name("array.prototype.flat").as_deref(), Some("es.array.flat"));
        assert_eq!(name("es6-promise").as_deref(), Some("es.promise"));
        assert_eq!(name("polyfill-array-flat-map").as_deref(), Some("es.array.flat-map"));
        assert_eq!(name("flat-map-polyfill").as_deref(), Some("es.array.flat-map"));
        assert_eq!(name("es7.array.flatMap").as_deref(), Some("es.array.flat-map"));
        assert_eq!(name("p-finally").as_deref(), Some("es.promise.finally"));
        // the broad constructor feature wins over a narrower one
        assert_eq!(name("polyfill-promise").as_deref(), Some("es.promise"));
        // a bare constructor name is the package of that name, not a polyfill
        assert_eq!(name("promise"), None);
        assert_eq!(name("lodash"), None);
        assert_eq!(name("esnext.array.at").as_deref(), Some("esnext.array.at"));
    }

    #[test]
    fn a_feature_is_needed_when_a_target_lacks_it() {
        let ix = index_of(&table()).unwrap();
        let new = vec![("chrome".to_owned(), vec![100]), ("node".to_owned(), vec![20])];
        let old = vec![("chrome".to_owned(), vec![60]), ("node".to_owned(), vec![20])];
        assert!(ix.all_available(&[0], &new));
        assert!(!ix.all_available(&[0], &old));
        // a target engine the feature has no row for lacks it
        assert!(!ix.all_available(&[0], &[("safari".to_owned(), vec![17])]));
        // `esnext.` twins count as missing when nothing has them
        assert!(!ix.all_available(&[5], &new));
    }
}

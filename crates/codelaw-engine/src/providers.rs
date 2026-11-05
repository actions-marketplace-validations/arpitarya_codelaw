//! Providers (SR-RULE-PACKS decision 19, SR-ARCHITECTURE decision 28a): an
//! npm package that builds a pack's data table from a package installed in
//! the project — `@codelaw/provider-tailwind` asks the project's own
//! `tailwindcss`. The engine finds the provider in the project's
//! `node_modules`, runs it with Node (one JSON request on stdin, one answer
//! on stdout), and keeps the table in the cache folder, keyed by what the
//! provider read, so a warm run starts no Node at all.
//!
//! The table answers class by class. A class not in it yet is asked for
//! together with every class it already knows, so all its answers come from
//! one run and their order is one order.

use crate::{EngineError, Project};
use codelaw_config::{Config, OnMissing};
use codelaw_core::class_table::{ClassInfo, ClassTable};
use codelaw_rules::RuleSet;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// The protocol both sides speak.
pub const PROTOCOL: u32 = 1;

/// How long one provider run may take.
const TIMEOUT: Duration = Duration::from_secs(300);

/// The most files the first run reads for the classes it seeds the table with.
const SEED_FILES: usize = 20_000;

/// The providers of a run, by npm name.
#[derive(Default)]
pub(crate) struct Providers {
    pub(crate) tables: BTreeMap<String, Arc<ProviderTable>>,
}

impl Providers {
    /// The table of the provider whose short name is `short`.
    pub(crate) fn table(&self, short: &str) -> Option<Arc<ProviderTable>> {
        self.tables.values().find(|t| t.short == short).cloned()
    }

    /// Every table's identity, for the cache key of a checked file.
    pub(crate) fn key(&self) -> String {
        self.tables.values().map(|t| t.identity.as_str()).collect::<Vec<_>>().join(";")
    }
}

/// One provider's table.
pub struct ProviderTable {
    pub package: String,
    pub short: String,
    identity: String,
    run: Runner,
    /// Every class asked for: the answer, `None` for a class the framework
    /// does not know.
    classes: Mutex<HashMap<String, Option<ClassInfo>>>,
    cache_file: Option<PathBuf>,
    meta: Value,
}

/// What a provider run needs.
#[derive(Clone)]
struct Runner {
    node: PathBuf,
    entry: PathBuf,
    root: PathBuf,
    options: Value,
    rules: Vec<String>,
}

impl Runner {
    fn ask(&self, classes: &[String]) -> Result<Value, String> {
        let mut cmd = Command::new(&self.node);
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }
        let mut child = cmd
            .arg(&self.entry)
            .current_dir(&self.root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("cannot start Node: {e}"))?;
        let request = json!({ "protocol": PROTOCOL, "root": self.root, "options": self.options, "rules": self.rules, "classes": classes });
        let mut stdin = child.stdin.take().expect("piped");
        let body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
        let writer = std::thread::spawn(move || {
            let _ = stdin.write_all(&body);
        });
        let mut stdout = child.stdout.take().expect("piped");
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut out = String::new();
            let _ = stdout.read_to_string(&mut out);
            let _ = tx.send(out);
        });
        let out = match rx.recv_timeout(TIMEOUT) {
            Ok(o) => o,
            Err(_) => {
                codelaw_core::process::kill_group(&mut child);
                return Err(format!("the provider did not answer within {TIMEOUT:?}"));
            }
        };
        let _ = writer.join();
        let _ = child.wait();
        let v: Value = serde_json::from_str(out.trim()).map_err(|e| format!("the provider answered something that is not JSON: {e}"))?;
        if v["protocol"].as_u64() != Some(u64::from(PROTOCOL)) {
            return Err(format!("the provider speaks protocol {}, codelaw {PROTOCOL}", v["protocol"]));
        }
        if let Some(e) = v["error"].as_str() {
            return Err(e.to_owned());
        }
        if let Some(why) = v["unavailable"].as_str() {
            return Err(why.to_owned());
        }
        Ok(v)
    }
}

fn info(v: &Value) -> Option<ClassInfo> {
    let o = v.as_object()?;
    Some(ClassInfo {
        order: o.get("order")?.as_str()?.parse().ok()?,
        properties: o.get("properties").and_then(Value::as_array).map(|a| a.iter().filter_map(|p| p.as_str().map(str::to_owned)).collect()).unwrap_or_default(),
        variants: o.get("variants").and_then(Value::as_str).unwrap_or_default().to_owned(),
        important: o.get("important").and_then(Value::as_bool).unwrap_or(false),
        canonical: o.get("canonical").and_then(Value::as_str).map(str::to_owned),
    })
}

fn info_json(i: &ClassInfo) -> Value {
    json!({ "order": i.order.to_string(), "properties": i.properties, "variants": i.variants, "important": i.important, "canonical": i.canonical })
}

/// A file's content hash, for the table's identity.
fn file_hash(path: &Path) -> Option<String> {
    std::fs::read(path).ok().map(|b| codelaw_cache::hash_bytes(&b).to_hex())
}

/// What the table was built from: the provider and its version, the
/// framework's version, the request's options and rules, and every file the
/// provider read with its hash.
fn identity(package: &str, meta: &Value) -> String {
    let digest = codelaw_cache::hash_bytes(serde_json::to_string(meta).unwrap_or_default().as_bytes()).to_hex();
    format!("{package}@{}+{}:{}", meta["version"].as_str().unwrap_or("?"), meta["framework"].as_str().unwrap_or("?"), &digest[..16])
}

impl ProviderTable {
    /// Replace the table with one run over `classes`.
    fn refill(&self, map: &mut HashMap<String, Option<ClassInfo>>, wanted: &[String]) -> Result<(), String> {
        let mut ask: BTreeSet<String> = map.iter().filter(|(_, i)| i.is_some()).map(|(c, _)| c.clone()).collect();
        ask.extend(wanted.iter().cloned());
        let ask: Vec<String> = ask.into_iter().collect();
        let v = self.run.ask(&ask)?;
        let answers = v["classes"].as_object().ok_or("the provider's answer has no `classes`")?;
        for c in &ask {
            map.insert(c.clone(), answers.get(c).and_then(info));
        }
        self.save(map);
        Ok(())
    }

    fn save(&self, map: &HashMap<String, Option<ClassInfo>>) {
        let Some(file) = &self.cache_file else { return };
        let classes: serde_json::Map<String, Value> = map.iter().map(|(c, i)| (c.clone(), i.as_ref().map(info_json).unwrap_or(Value::Null))).collect();
        let body = json!({ "meta": self.meta, "identity": self.identity, "classes": classes });
        if let Some(dir) = file.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let tmp = file.with_extension("json.tmp");
        if std::fs::write(&tmp, serde_json::to_vec(&body).unwrap_or_default()).is_ok() {
            let _ = std::fs::rename(&tmp, file);
        }
    }
}

impl ClassTable for ProviderTable {
    fn identity(&self) -> &str {
        &self.identity
    }

    fn lookup(&self, classes: &[&str]) -> Result<Vec<Option<ClassInfo>>, String> {
        let mut map = self.classes.lock().unwrap_or_else(|e| e.into_inner());
        let missing: Vec<String> = classes.iter().filter(|c| !map.contains_key(**c)).map(|c| (*c).to_owned()).collect();
        if !missing.is_empty() {
            self.refill(&mut map, &missing)?;
        }
        Ok(classes.iter().map(|c| map.get(*c).cloned().flatten()).collect())
    }
}

/// The folder of an installed npm package: `node_modules/<name>` in `dir` or
/// a folder above it.
fn find_package(dir: &Path, name: &str) -> Option<PathBuf> {
    // A provider's source folder linked in by codelaw's own fixtures has no
    // package.json; its entry point is enough.
    dir.ancestors().map(|d| d.join("node_modules").join(name)).find(|p| p.join("package.json").is_file() || p.join("index.mjs").is_file())
}

fn package_json(dir: &Path) -> Value {
    std::fs::read_to_string(dir.join("package.json")).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(Value::Null)
}

/// The provider's own settings and `on_missing` from `[providers]`.
fn settings(config: &Config, short: &str) -> (OnMissing, Value) {
    let raw = config.files.first().map(|f| &f.raw.providers);
    let all = raw.and_then(|p| p.on_missing).unwrap_or(OnMissing::Skip);
    match (short, raw) {
        ("tailwind", Some(p)) => {
            let t = p.tailwind.clone().unwrap_or_default();
            let stylesheet = t.stylesheet.map(|s| config.config_dir().join(s));
            (t.on_missing.unwrap_or(all), json!({ "stylesheet": stylesheet }))
        }
        _ => (all, json!({})),
    }
}

/// Every class the project's class lists name: what a table's first run is
/// asked for.
fn seed(root: &Path, lists: &codelaw_lang::class_lists::ClassListOptions) -> Vec<String> {
    use codelaw_core::Language;
    let mut out = BTreeSet::new();
    let walk = ignore::WalkBuilder::new(root).hidden(true).build();
    let mut files = 0;
    for e in walk.flatten() {
        let path = e.path();
        let language = match path.extension().and_then(|x| x.to_str()) {
            Some("js" | "mjs" | "cjs" | "jsx") => Language::Jsx,
            Some("ts" | "mts" | "cts") => Language::Typescript,
            Some("tsx") => Language::Tsx,
            Some("html" | "htm") => Language::Html,
            _ => continue,
        };
        if path.components().any(|c| c.as_os_str() == "node_modules") {
            continue;
        }
        files += 1;
        if files > SEED_FILES {
            break;
        }
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let root = codelaw_lang::parse(language, &text);
        for l in codelaw_lang::class_lists::class_lists(language, &root, lists) {
            out.extend(text[l.start..l.end].split_whitespace().filter(|w| w.len() < 200).map(str::to_owned));
        }
    }
    out.into_iter().collect()
}

/// The providers this project's packs name (or, for codelaw's own tests,
/// every first-party one when `CODELAW_PROVIDER_ROOT` names a folder that
/// holds them), each with its table — or, under `on_missing = "skip"`, a
/// note saying why there is none.
pub(crate) fn load(config: &Config, rules: &RuleSet, options: &crate::EngineOptions, lists: &codelaw_lang::class_lists::ClassListOptions, cache_dir: Option<PathBuf>, warnings: &mut Vec<String>) -> Result<Providers, EngineError> {
    let mut wanted: BTreeMap<String, String> = BTreeMap::new(); // package -> the pack that names it
    for (label, dir) in &rules.packs {
        if let Some(m) = codelaw_rules::read_manifest(dir) {
            for p in m.providers {
                wanted.entry(p).or_insert_with(|| label.clone());
            }
        }
    }
    let forced_root = options.provider_root_env.as_ref().map(PathBuf::from).filter(|p| p.is_dir());
    if forced_root.is_some() {
        for (package, short) in codelaw_rules::FIRST_PARTY_PROVIDERS {
            let ns = format!("{short}/");
            if rules.rules.iter().any(|r| r.rule.id.starts_with(&ns)) {
                wanted.entry((*package).to_owned()).or_insert_with(|| format!("the {short} rules"));
            }
        }
    }
    let mut out = Providers::default();
    for (package, pack) in wanted {
        let short = codelaw_rules::FIRST_PARTY_PROVIDERS.iter().find(|(p, _)| *p == package).map(|(_, s)| (*s).to_owned()).unwrap_or_else(|| package.rsplit('/').next().unwrap_or(&package).trim_start_matches("provider-").to_owned());
        let (on_missing, provider_options) = settings(config, &short);
        let root = forced_root.clone().unwrap_or_else(|| config.root_dir.clone());
        match open(&package, &short, &root, provider_options, rules, options, lists, cache_dir.as_deref()) {
            Ok(t) => {
                out.tables.insert(package, Arc::new(t));
            }
            Err(why) => {
                let note = format!("{pack} needs {package}, which gave no table: {why}; the rules and the class sorting that read it were skipped (`[providers] on_missing`)");
                match on_missing {
                    OnMissing::Skip => warnings.push(note),
                    OnMissing::Error => {
                        return Err(EngineError::Usage(format!("{pack} needs {package}, which gave no table: {why} (`[providers] on_missing = \"error\"`)")));
                    }
                }
            }
        }
    }
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
fn open(package: &str, short: &str, root: &Path, provider_options: Value, rules: &RuleSet, options: &crate::EngineOptions, lists: &codelaw_lang::class_lists::ClassListOptions, cache_dir: Option<&Path>) -> Result<ProviderTable, String> {
    let dir = find_package(root, package).ok_or_else(|| format!("it is not installed (npm i -D {package})"))?;
    let pj = package_json(&dir);
    let version = pj["version"].as_str().unwrap_or("0.0.0").to_owned();
    let entry = dir.join(pj["main"].as_str().unwrap_or("index.mjs"));
    let node = codelaw_adapters::locate_program("node", root, options.path_env.as_deref()).ok_or("Node is not on the PATH")?;
    let ns = format!("{short}/");
    let rule_ids: Vec<String> = rules.rules.iter().filter(|r| r.rule.id.starts_with(&ns) && r.default.as_ref().is_some_and(|d| d.severity.is_some())).map(|r| r.rule.id.clone()).collect();
    let run = Runner { node, entry, root: root.to_path_buf(), options: provider_options.clone(), rules: rule_ids.clone() };
    let cache_file = cache_dir.map(|d| d.join("providers").join(format!("{short}.json")));
    // A table on disk holds while the provider, the request and every file
    // the provider read are what they were.
    if let Some(file) = &cache_file
        && let Some(saved) = std::fs::read(file).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok())
    {
        let meta = &saved["meta"];
        let inputs_hold = meta["inputs"].as_object().is_some_and(|m| m.iter().all(|(p, h)| file_hash(Path::new(p)).as_deref() == h.as_str()));
        if inputs_hold && meta["version"] == json!(version) && meta["options"] == provider_options && meta["rules"] == json!(rule_ids) {
            let classes = saved["classes"].as_object().map(|m| m.iter().map(|(c, i)| (c.clone(), info(i))).collect()).unwrap_or_default();
            return Ok(ProviderTable {
                package: package.to_owned(),
                short: short.to_owned(),
                identity: identity(package, meta),
                run,
                classes: Mutex::new(classes),
                cache_file: cache_file.clone(),
                meta: meta.clone(),
            });
        }
    }
    let seeded = seed(root, lists);
    let v = run.ask(&seeded)?;
    let inputs: serde_json::Map<String, Value> = v["inputs"].as_array().into_iter().flatten().filter_map(Value::as_str).map(|p| (p.to_owned(), json!(file_hash(Path::new(p))))).collect();
    let meta = json!({ "version": version, "framework": v["tailwind"].as_str().or(v["framework"].as_str()), "options": provider_options, "rules": rule_ids, "inputs": inputs });
    let answers = v["classes"].as_object().cloned().unwrap_or_default();
    let classes: HashMap<String, Option<ClassInfo>> = seeded.iter().map(|c| (c.clone(), answers.get(c).and_then(info))).collect();
    let table = ProviderTable { package: package.to_owned(), short: short.to_owned(), identity: identity(package, &meta), run, classes: Mutex::new(HashMap::new()), cache_file, meta };
    table.save(&classes);
    *table.classes.lock().unwrap_or_else(|e| e.into_inner()) = classes;
    Ok(table)
}

impl Project {
    /// Why a rule that reads a provider's table cannot run here, if it
    /// cannot: the rule tester's `not checked` (SR-RULE-PACKS decision 19).
    pub fn provider_unavailable(&self, native: &str) -> Option<String> {
        let short = codelaw_native::lookup(native)?.provider()?;
        self.providers.table(short).is_none().then(|| format!("{native} reads the `{short}` provider's class table, and none was built"))
    }

    /// The class table of the provider `short`, when there is one.
    pub fn class_table(&self, short: &str) -> Option<Arc<ProviderTable>> {
        self.providers.table(short)
    }
}

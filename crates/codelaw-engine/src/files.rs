//! What a native rule sees of a file beyond its text (SR-ARCHITECTURE
//! decision 15b): the file's path, its bytes as they are on disk, and a
//! read-only view of the files around it. An embedded region sees its host
//! file.
//!
//! Every question a rule asks about another file is recorded with a hash of
//! its answer. A cached result is reused only while every answer still
//! holds (SR-CACHE §1), so a link checked against a missing file is checked
//! again once the file exists.

use codelaw_cache::{Key, hash_bytes};
use codelaw_core::Language;
use codelaw_native::FileContext;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Where the files around a file come from.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Tree<'a> {
    /// The files on disk under the project root.
    Disk,
    /// An in-memory set: `check_files` and the rule tester. Nothing on disk
    /// is read.
    /// `Memory`'s second part names the files that have the executable bit
    /// (a snippet header `# run.py +x`, W-184); every other file has none.
    Memory(&'a [(String, Language, String)], &'a std::collections::BTreeSet<String>),
    /// The files on disk, with these texts in place of theirs: a
    /// `fix --dry-run`'s multi-file fixes, which write nothing
    /// (SR-MULTI-FILE-FIXES decision 2).
    Overlay(&'a std::collections::BTreeMap<String, String>),
}

/// The set of executable files of a tree that has none.
pub(crate) static NO_MODES: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

/// One question a rule asked.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Question {
    Exists(String),
    IsDir(String),
    Read(String),
    List(String),
    /// The project's whole file list.
    Files,
    /// The as-of date (L5 decision 2).
    AsOf,
    /// Whether the file at this path has the executable bit (W-184).
    Executable(String),
}

/// A question and the hash of its answer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Asked {
    pub question: Question,
    pub answer: [u8; 32],
}

/// The as-of date, asked for only when a rule needs it.
pub(crate) type AsOfFn<'a> = &'a (dyn Fn() -> Option<String> + Sync);

/// A file as a native rule sees it.
pub(crate) struct ProjectFile<'a> {
    path: &'a str,
    bytes: Vec<u8>,
    root: &'a Path,
    tree: Tree<'a>,
    as_of: Option<AsOfFn<'a>>,
    asked: Mutex<Vec<Asked>>,
    providers: Option<&'a crate::providers::Providers>,
    /// The project, for the data packages its rules may read.
    project: Option<&'a crate::Project>,
    /// The project's file list, walked once per file however often a rule
    /// asks (a multi-file fix asks once per finding).
    files: std::sync::OnceLock<Vec<String>>,
    /// The other files read, each once however often a rule asks.
    texts: Mutex<std::collections::HashMap<String, Option<String>>>,
}

impl<'a> ProjectFile<'a> {
    /// `text` is the file's text without a byte-order mark; `bom` says
    /// whether the file has one.
    pub fn new(path: &'a str, text: &str, bom: bool, root: &'a Path, tree: Tree<'a>) -> Self {
        let mut bytes = Vec::with_capacity(text.len() + 3);
        if bom {
            bytes.extend_from_slice("\u{feff}".as_bytes());
        }
        bytes.extend_from_slice(text.as_bytes());
        ProjectFile { path, bytes, root, tree, as_of: None, asked: Mutex::new(Vec::new()), providers: None, project: None, files: std::sync::OnceLock::new(), texts: Mutex::new(std::collections::HashMap::new()) }
    }

    /// Where the as-of date comes from.
    pub fn with_as_of(mut self, as_of: AsOfFn<'a>) -> Self {
        self.as_of = Some(as_of);
        self
    }

    /// The providers' tables its rules may read.
    pub fn with_providers(mut self, providers: &'a crate::providers::Providers) -> Self {
        self.providers = Some(providers);
        self
    }

    /// The data packages its rules may read (`FileContext::data_table`).
    pub fn with_project(mut self, project: &'a crate::Project) -> Self {
        self.project = Some(project);
        self
    }

    /// Every question asked so far, in order, without repeats.
    pub fn asked(&self) -> Vec<Asked> {
        self.asked.lock().map(|a| a.clone()).unwrap_or_default()
    }

    fn record(&self, question: Question, answer: Key) {
        if let Ok(mut a) = self.asked.lock()
            && !a.iter().any(|x| x.question == question)
        {
            a.push(Asked { question, answer: answer.0 });
        }
    }
}

/// A project-relative path a rule may name: `/`-separated, with no empty,
/// `.` or `..` segment and nothing that could leave the root. `""` is the
/// root itself.
fn safe(path: &str) -> bool {
    path.is_empty() || (!path.contains('\\') && !path.contains(':') && path.split('/').all(|s| !s.is_empty() && s != "." && s != ".."))
}

fn hash_bool(b: bool) -> Key {
    hash_bytes(&[b as u8])
}

fn hash_exec(e: Option<bool>) -> Key {
    hash_bytes(&[match e {
        None => 2,
        Some(b) => b as u8,
    }])
}

fn hash_text(t: &Option<String>) -> Key {
    match t {
        None => hash_bytes(b"\0none"),
        Some(s) => hash_bytes(format!("\x01{s}").as_bytes()),
    }
}

fn hash_list(l: &Option<Vec<String>>) -> Key {
    match l {
        None => hash_bytes(b"\0none"),
        Some(v) => hash_bytes(format!("\x01{}", v.join("\0")).as_bytes()),
    }
}

/// The answer to a question, from `tree`.
fn answer(root: &Path, tree: Tree<'_>, q: &Question, as_of: &dyn Fn() -> Option<String>) -> Key {
    match q {
        Question::AsOf => hash_text(&as_of()),
        Question::Executable(p) => hash_exec(executable(root, tree, p)),
        Question::Exists(p) => hash_bool(exists(root, tree, p)),
        Question::IsDir(p) => hash_bool(is_dir(root, tree, p)),
        Question::Read(p) => hash_text(&read(root, tree, p)),
        Question::List(p) => hash_list(&list(root, tree, p)),
        Question::Files => hash_list(&Some(project_files(root, tree))),
    }
}

/// Whether every recorded answer still holds against the disk and the
/// as-of date (a cached result's check before it is reused).
pub(crate) fn still_holds(root: &Path, asked: &[Asked], as_of: &dyn Fn() -> Option<String>) -> bool {
    asked.iter().all(|a| answer(root, Tree::Disk, &a.question, as_of).0 == a.answer)
}

fn exists(root: &Path, tree: Tree<'_>, path: &str) -> bool {
    if !safe(path) {
        return false;
    }
    match tree {
        Tree::Disk | Tree::Overlay(_) => root.join(path).exists(),
        Tree::Memory(files, _) => path.is_empty() || files.iter().any(|(p, _, _)| p == path || p.strip_prefix(path).is_some_and(|r| r.starts_with('/'))),
    }
}

fn is_dir(root: &Path, tree: Tree<'_>, path: &str) -> bool {
    if !safe(path) {
        return false;
    }
    match tree {
        Tree::Disk | Tree::Overlay(_) => root.join(path).is_dir(),
        Tree::Memory(files, _) => path.is_empty() || files.iter().any(|(p, _, _)| p.strip_prefix(path).is_some_and(|r| r.starts_with('/'))),
    }
}

/// Other files' texts read by native rules, kept while their size and
/// modification time hold: a multi-file fix reads every file that may use a
/// name, from each file with a finding, and a cached result checks each
/// answer again (SR-CACHE). Bounded: past [`READ_CACHE_BYTES`] it starts over.
type ReadEntries = std::collections::HashMap<std::path::PathBuf, (u64, std::time::SystemTime, Option<String>)>;
static READ_CACHE: std::sync::OnceLock<Mutex<(usize, ReadEntries)>> = std::sync::OnceLock::new();
const READ_CACHE_BYTES: usize = 8 << 20;

fn read_disk(p: &Path) -> Option<String> {
    let meta = std::fs::metadata(p).ok().filter(|m| m.is_file())?;
    let stamp = (meta.len(), meta.modified().ok()?);
    let cache = READ_CACHE.get_or_init(|| Mutex::new((0, std::collections::HashMap::new())));
    if let Ok(c) = cache.lock()
        && let Some((len, at, text)) = c.1.get(p)
        && (*len, *at) == stamp
    {
        return text.clone();
    }
    let text = std::fs::read(p).ok().and_then(|b| String::from_utf8(b).ok()).map(|t| t.strip_prefix('\u{feff}').map(str::to_owned).unwrap_or(t));
    if let Ok(mut c) = cache.lock() {
        let size = text.as_ref().map_or(0, String::len);
        if c.0 + size > READ_CACHE_BYTES {
            *c = (0, std::collections::HashMap::new());
        }
        if size <= READ_CACHE_BYTES / 4 {
            c.0 += size;
            c.1.insert(p.to_path_buf(), (stamp.0, stamp.1, text.clone()));
        }
    }
    text
}

fn read(root: &Path, tree: Tree<'_>, path: &str) -> Option<String> {
    if !safe(path) || path.is_empty() {
        return None;
    }
    match tree {
        Tree::Overlay(texts) if texts.contains_key(path) => texts.get(path).cloned(),
        Tree::Disk | Tree::Overlay(_) => read_disk(&root.join(path)),
        Tree::Memory(files, _) => files.iter().find(|(p, _, _)| p == path).map(|(_, _, t)| t.clone()),
    }
}

fn project_files(root: &Path, tree: Tree<'_>) -> Vec<String> {
    match tree {
        Tree::Disk | Tree::Overlay(_) => crate::discover::walk_files(root),
        Tree::Memory(files, _) => {
            let mut v: Vec<String> = files.iter().map(|(p, _, _)| p.clone()).collect();
            v.sort();
            v
        }
    }
}

/// Whether the file at `path` has the executable bit. `None` where there is
/// no such bit to read: off Unix (no mode bits, so no rule that depends on
/// one reports there: the same answer on every machine, L5), a path that
/// is not a file, or a name no rule may use.
fn executable(root: &Path, tree: Tree<'_>, path: &str) -> Option<bool> {
    if !safe(path) || path.is_empty() {
        return None;
    }
    match tree {
        Tree::Memory(files, modes) => files.iter().any(|(p, _, _)| p == path).then(|| modes.contains(path)),
        Tree::Disk | Tree::Overlay(_) => disk_executable(&root.join(path)),
    }
}

#[cfg(unix)]
fn disk_executable(p: &Path) -> Option<bool> {
    use std::os::unix::fs::PermissionsExt;
    let meta = std::fs::metadata(p).ok().filter(|m| m.is_file())?;
    Some(meta.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn disk_executable(_: &Path) -> Option<bool> {
    None
}

fn list(root: &Path, tree: Tree<'_>, path: &str) -> Option<Vec<String>> {
    if !safe(path) || !is_dir(root, tree, path) {
        return None;
    }
    let mut names: Vec<String> = match tree {
        Tree::Disk | Tree::Overlay(_) => std::fs::read_dir(root.join(path)).ok()?.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).collect(),
        Tree::Memory(files, _) => {
            let prefix = if path.is_empty() { String::new() } else { format!("{path}/") };
            files.iter().filter_map(|(p, _, _)| p.strip_prefix(prefix.as_str())).map(|r| r.split('/').next().unwrap_or(r).to_owned()).collect()
        }
    };
    names.sort();
    names.dedup();
    Some(names)
}

impl FileContext for ProjectFile<'_> {
    fn path(&self) -> &str {
        self.path
    }
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    fn exists(&self, path: &str) -> bool {
        let a = exists(self.root, self.tree, path);
        self.record(Question::Exists(path.to_owned()), hash_bool(a));
        a
    }
    fn is_dir(&self, path: &str) -> bool {
        let a = is_dir(self.root, self.tree, path);
        self.record(Question::IsDir(path.to_owned()), hash_bool(a));
        a
    }
    fn read(&self, path: &str) -> Option<String> {
        let known = self.texts.lock().ok().and_then(|t| t.get(path).cloned());
        let a = match known {
            Some(a) => a,
            None => {
                let a = read(self.root, self.tree, path);
                if let Ok(mut t) = self.texts.lock() {
                    t.insert(path.to_owned(), a.clone());
                }
                a
            }
        };
        self.record(Question::Read(path.to_owned()), hash_text(&a));
        a
    }
    fn list(&self, path: &str) -> Option<Vec<String>> {
        let a = list(self.root, self.tree, path);
        self.record(Question::List(path.to_owned()), hash_list(&a));
        a
    }
    fn project_files(&self) -> Option<Vec<String>> {
        let files = self.files.get_or_init(|| project_files(self.root, self.tree)).clone();
        self.record(Question::Files, hash_list(&Some(files.clone())));
        Some(files)
    }
    fn is_executable(&self) -> Option<bool> {
        let a = executable(self.root, self.tree, self.path);
        self.record(Question::Executable(self.path.to_owned()), hash_exec(a));
        a
    }
    fn as_of(&self) -> Option<String> {
        let a = self.as_of.and_then(|f| f());
        self.record(Question::AsOf, hash_text(&a));
        a
    }
    fn data_table(&self, short: &str) -> Option<std::sync::Arc<serde_json::Value>> {
        let table = match short {
            "polyfills" => self.project?.polyfill_table().map(|t| (t.version.clone(), t.value.clone())),
            "confusables" => self.project?.confusables_table().map(|t| (t.version.clone(), t.value.clone())),
            "python-deprecations" => self.project?.python_deprecation_table().map(|t| (t.version.clone(), t.value.clone())),
            _ => None,
        };
        // The answer is the table's version: a refreshed table re-judges.
        self.record(Question::Read(format!("data-table:{short}")), hash_text(&table.as_ref().map(|t| t.0.clone())));
        table.map(|t| t.1)
    }
    fn python_target(&self, rule: &str) -> Option<Vec<u64>> {
        self.project?.python_target(&self.root.join(self.path), Some(rule)).value
    }
    fn class_table(&self, short: &str) -> Option<std::sync::Arc<dyn codelaw_core::class_table::ClassTable>> {
        self.providers?.table(short).map(|t| t as std::sync::Arc<dyn codelaw_core::class_table::ClassTable>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_tree_answers_and_records() {
        let files = vec![
            ("docs/a.md".to_owned(), Language::Markdown, "# A\n".to_owned()),
            ("docs/sub/b.md".to_owned(), Language::Markdown, "# B\n".to_owned()),
            ("README.md".to_owned(), Language::Markdown, "# R\n".to_owned()),
        ];
        let f = ProjectFile::new("docs/a.md", "# A\n", true, Path::new("/nowhere"), Tree::Memory(&files, &NO_MODES));
        assert_eq!(f.bytes(), "\u{feff}# A\n".as_bytes());
        assert!(f.exists("docs/sub/b.md"));
        assert!(f.exists("docs/sub"));
        assert!(f.is_dir("docs/sub"));
        assert!(!f.is_dir("docs/a.md"));
        assert!(!f.exists("docs/c.md"));
        assert!(!f.exists("../etc/passwd"));
        assert!(!f.exists("docs/./a.md"));
        assert_eq!(f.read("README.md").as_deref(), Some("# R\n"));
        assert_eq!(f.list("docs"), Some(vec!["a.md".to_owned(), "sub".to_owned()]));
        assert_eq!(f.list(""), Some(vec!["README.md".to_owned(), "docs".to_owned()]));
        assert_eq!(f.list("README.md"), None);
        f.exists("docs/sub/b.md");
        assert_eq!(f.asked().len(), 11, "a repeated question is recorded once");
    }

    #[test]
    fn disk_answers_are_checked_again() {
        let dir = std::env::temp_dir().join(format!("codelaw-files-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let f = ProjectFile::new("a.md", "", false, &dir, Tree::Disk);
        assert!(!f.exists("b.md"));
        let asked = f.asked();
        assert!(still_holds(&dir, &asked, &|| None));
        std::fs::write(dir.join("b.md"), "x").unwrap();
        assert!(!still_holds(&dir, &asked, &|| None), "the file now exists, so the cached answer is stale");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_as_of_date_is_an_answer_too() {
        let date = || Some("2026-10-25".to_owned());
        let f = ProjectFile::new("package.json", "{}", false, Path::new("/nowhere"), Tree::Disk).with_as_of(&date);
        assert_eq!(f.as_of().as_deref(), Some("2026-10-25"));
        let asked = f.asked();
        assert_eq!(asked.len(), 1);
        assert!(still_holds(Path::new("/nowhere"), &asked, &date));
        assert!(!still_holds(Path::new("/nowhere"), &asked, &|| Some("2026-10-26".to_owned())), "a new as-of date is a new answer");
        let plain = ProjectFile::new("a.md", "", false, Path::new("/nowhere"), Tree::Disk);
        assert_eq!(plain.as_of(), None);
    }

    #[cfg(unix)]
    #[test]
    fn the_executable_bit_is_an_answer_too() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("codelaw-files-exec-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("run.py"), "print(1)\n").unwrap();
        std::fs::set_permissions(dir.join("run.py"), std::fs::Permissions::from_mode(0o644)).unwrap();
        let f = ProjectFile::new("run.py", "print(1)\n", false, &dir, Tree::Disk);
        assert_eq!(f.is_executable(), Some(false));
        let asked = f.asked();
        assert!(still_holds(&dir, &asked, &|| None));
        std::fs::set_permissions(dir.join("run.py"), std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(!still_holds(&dir, &asked, &|| None), "a chmod is a new answer");
        assert_eq!(ProjectFile::new("run.py", "", false, &dir, Tree::Disk).is_executable(), Some(true));
        assert_eq!(ProjectFile::new("gone.py", "", false, &dir, Tree::Disk).is_executable(), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_memory_tree_gives_the_bit_to_the_files_named() {
        let files = vec![("a.py".to_owned(), Language::Python, String::new()), ("b.py".to_owned(), Language::Python, String::new())];
        let modes: std::collections::BTreeSet<String> = ["a.py".to_owned()].into();
        let root = Path::new("/nowhere");
        assert_eq!(ProjectFile::new("a.py", "", false, root, Tree::Memory(&files, &modes)).is_executable(), Some(true));
        assert_eq!(ProjectFile::new("b.py", "", false, root, Tree::Memory(&files, &modes)).is_executable(), Some(false));
    }
}

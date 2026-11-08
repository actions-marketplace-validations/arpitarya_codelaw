//! Multi-file fixes (SR-MULTI-FILE-FIXES): a fix that edits files other
//! than its finding's, applied after every file's own fixes and never in
//! the loop of one file, one fix at a time, through the five safeguards:
//!
//! 1. every edited file still parses, the finding is gone and no rule
//!    fires in an edited file that did not fire before (L4 decisions 9, 10);
//! 2. the project graph, rebuilt over the edited files, gains no reference
//!    that resolves to nothing (`codelaw_graph::dangling`);
//! 3. every file is written, or none (`codelaw_fix::write_all_or_nothing`);
//! 4. `auto` only when every reference is certain — the rule's own rung,
//!    and `multi_file`, decide it when the finding is made (`lint.rs`);
//! 5. never on save or from the agent hook: [`crate::FixOptions::on_edit`]
//!    skips this pass, and the per-file loop never offers such a fix.
//!
//! A refused fix leaves every file as it was; its finding is reported as
//! `manual`, its note naming the safeguard.

use crate::discover::FileEntry;
use crate::files::Tree;
use crate::lint::{RunMode, lint_text};
use crate::project::Source;
use crate::{Project, fix::firing};
use codelaw_core::{Diagnostic, FixClass, Language, TextEdit};
use std::collections::{BTreeMap, BTreeSet};

/// The most multi-file fixes one run applies.
pub(crate) const MAX_FIXES: usize = 200;

/// Where the files of a multi-file pass come from.
pub(crate) enum Base<'a> {
    /// The disk under the project root; with `write`, each fix is written.
    Disk { write: bool },
    /// The rule tester's virtual repo: nothing on disk is read or written.
    Memory(&'a [(String, Language, String)]),
}

/// One run of the multi-file pass.
pub(crate) struct MultiFix<'a> {
    project: &'a Project,
    base: Base<'a>,
    unsafe_fixes: bool,
    choice: Option<&'a str>,
    /// The current text of every file this pass read or changed.
    pub texts: BTreeMap<String, String>,
    /// The texts the pass started from, of the files it changed.
    pub originals: BTreeMap<String, String>,
    /// The bytes each disk file held when read, and whether its text has a
    /// byte-order mark there.
    bytes: BTreeMap<String, (Vec<u8>, bool)>,
    /// Fixes applied, per file and rule.
    pub applied: BTreeMap<String, BTreeMap<String, usize>>,
    /// Fixes refused: (rule, file, message) → why.
    pub refused: BTreeMap<(String, String, String), String>,
    /// The graph's sources, as the pass started.
    sources: Vec<Source>,
    /// The dangling references of the current texts, and the classes
    /// their stylesheets define.
    dangling: Option<(BTreeSet<String>, BTreeSet<String>)>,
}

/// Whether `d` offers a multi-file fix this run applies.
pub(crate) fn wanted(d: &Diagnostic, unsafe_fixes: bool, choice: Option<&str>) -> bool {
    d.is_multi_file() && (d.fix == FixClass::Auto || (unsafe_fixes && d.fix == FixClass::Suggest)) && d.applicable_fix(choice).is_some()
}

impl<'a> MultiFix<'a> {
    pub fn new(project: &'a Project, base: Base<'a>, sources: Vec<Source>, unsafe_fixes: bool, choice: Option<&'a str>) -> Self {
        MultiFix {
            project,
            base,
            unsafe_fixes,
            choice,
            texts: BTreeMap::new(),
            originals: BTreeMap::new(),
            bytes: BTreeMap::new(),
            applied: BTreeMap::new(),
            refused: BTreeMap::new(),
            sources,
            dangling: None,
        }
    }

    /// Start from these texts instead of the base's (a dry run's fixed files).
    pub fn preload(&mut self, rel: &str, text: &str) {
        self.texts.insert(rel.to_owned(), text.to_owned());
    }

    fn language(&self, rel: &str) -> Option<Language> {
        match &self.base {
            Base::Memory(files) => files.iter().find(|(p, _, _)| p == rel).map(|(_, l, _)| *l),
            Base::Disk { .. } => Language::from_path(std::path::Path::new(rel)),
        }
    }

    fn entry(&self, rel: &str) -> Option<FileEntry> {
        Some(FileEntry { abs: self.project.config.root_dir.join(rel), rel: rel.to_owned(), language: self.language(rel)? })
    }

    /// The current text of `rel`, or `None` when there is no such file.
    fn text(&mut self, rel: &str) -> Option<String> {
        if let Some(t) = self.texts.get(rel) {
            return Some(t.clone());
        }
        let text = match &self.base {
            Base::Memory(files) => files.iter().find(|(p, _, _)| p == rel).map(|(_, _, t)| t.clone())?,
            Base::Disk { .. } => {
                if rel.split('/').any(|s| s.is_empty() || s == "." || s == "..") {
                    return None;
                }
                let bytes = std::fs::read(self.project.config.root_dir.join(rel)).ok()?;
                let text = String::from_utf8(bytes.clone()).ok()?;
                let (text, bom) = match text.strip_prefix('\u{feff}') {
                    Some(t) => (t.to_owned(), true),
                    None => (text, false),
                };
                self.bytes.insert(rel.to_owned(), (bytes, bom));
                text
            }
        };
        self.texts.insert(rel.to_owned(), text.clone());
        Some(text)
    }

    fn bom(&self, rel: &str) -> bool {
        self.bytes.get(rel).is_some_and(|(_, b)| *b)
    }

    /// Lint `rel`'s `text` with the files around it as `texts` has them.
    fn lint(&self, rel: &str, text: &str, texts: &BTreeMap<String, String>) -> Vec<Diagnostic> {
        let Some(entry) = self.entry(rel) else { return Vec::new() };
        let bom = self.bom(rel);
        match &self.base {
            Base::Memory(files) => {
                let set: Vec<(String, Language, String)> = files.iter().map(|(p, l, t)| (p.clone(), *l, texts.get(p).cloned().unwrap_or_else(|| t.clone()))).collect();
                let mode = RunMode { external_fixers: false, refused: None, bom, tree: Tree::Memory(&set), choice: self.choice };
                lint_text(self.project, &entry, text, &mode).diagnostics
            }
            Base::Disk { .. } => {
                let mode = RunMode { external_fixers: false, refused: None, bom, tree: Tree::Overlay(texts), choice: self.choice };
                lint_text(self.project, &entry, text, &mode).diagnostics
            }
        }
    }

    /// The project rules' diagnostics over the files as `texts` has them.
    fn graph_diags(&self, texts: &BTreeMap<String, String>) -> Vec<Diagnostic> {
        if self.project.graph_rules().is_empty() {
            return Vec::new();
        }
        let mut sources = self.sources.clone();
        for s in &mut sources {
            if let Some(t) = texts.get(&s.rel) {
                s.text = t.clone();
            }
        }
        self.project.run_graph_rules(&sources, self.project.graph_config().unwrap_or_default())
    }

    /// Every diagnostic of `rel` with the files as `texts` has them: its
    /// file rules', and the project rules' in it out of `graph`.
    fn all(&self, rel: &str, text: &str, texts: &BTreeMap<String, String>, graph: &[Diagnostic]) -> Vec<Diagnostic> {
        let mut d = self.lint(rel, text, texts);
        d.extend(graph.iter().filter(|g| g.file == rel).cloned());
        d
    }

    /// The current diagnostics of `rel`.
    pub fn diagnostics(&mut self, rel: &str) -> Vec<Diagnostic> {
        let Some(text) = self.text(rel) else { return Vec::new() };
        let graph = self.graph_diags(&self.texts);
        self.all(rel, &text, &self.texts, &graph)
    }

    fn dangling_of(&self, texts: &BTreeMap<String, String>) -> (BTreeSet<String>, BTreeSet<String>) {
        let mut sources = self.sources.clone();
        for s in &mut sources {
            if let Some(t) = texts.get(&s.rel) {
                s.text = t.clone();
            }
        }
        let graph = self.project.build_graph(&sources, self.project.graph_config().unwrap_or_default());
        (codelaw_graph::dangling(&graph), codelaw_graph::defined_classes(&graph))
    }

    /// Apply every multi-file fix on offer in `files`, one at a time, until
    /// none is left or [`MAX_FIXES`] are applied.
    pub fn run(&mut self, files: &BTreeSet<String>) {
        let mut queue: BTreeSet<String> = files.clone();
        let mut done = 0;
        let mut steps = 0;
        while let Some(rel) = queue.pop_first() {
            steps += 1;
            if done >= MAX_FIXES || steps > MAX_FIXES * 4 {
                break;
            }
            let diags = self.diagnostics(&rel);
            let key = |d: &Diagnostic| (d.rule.clone(), d.file.clone(), d.message.clone());
            let Some(d) = diags.into_iter().find(|d| d.file == rel && wanted(d, self.unsafe_fixes, self.choice) && !self.refused.contains_key(&key(d))) else { continue };
            match self.apply(&d) {
                Ok(()) => {
                    done += 1;
                    // What the fix changed may offer, or end, fixes anywhere.
                    queue.extend(files.iter().cloned());
                }
                Err(why) => {
                    self.refused.insert(key(&d), why);
                    queue.insert(rel);
                }
            }
        }
    }

    /// Apply one finding's multi-file fix through the safeguards.
    fn apply(&mut self, d: &Diagnostic) -> Result<(), String> {
        let (own, other) = d.applicable_fix(self.choice).ok_or("no fix")?;
        let mut per: BTreeMap<String, (Vec<TextEdit>, bool)> = BTreeMap::new();
        per.entry(d.file.clone()).or_default().0.extend(own.iter().cloned());
        for f in other {
            let e = per.entry(f.path.clone()).or_default();
            e.0.extend(f.edits.iter().cloned());
            e.1 |= f.strip_bom;
        }
        per.retain(|_, (edits, strip)| !edits.is_empty() || *strip);
        // Safeguard 1: each file takes its edits and still parses.
        let mut next = self.texts.clone();
        let mut strip: BTreeSet<String> = BTreeSet::new();
        for (path, (edits, strip_bom)) in &per {
            let old = self.text(path).ok_or_else(|| format!("`{path}` is not a file of the project"))?;
            next.insert(path.clone(), old.clone());
            let new = codelaw_fix::apply_edits(&old, edits).map_err(|e| format!("`{path}`: {e}"))?;
            if let Some(l) = self.language(path) {
                let entry = FileEntry { abs: self.project.config.root_dir.join(path), rel: path.clone(), language: l };
                if crate::fix::syntax_errors(self.project, &entry, &new) > crate::fix::syntax_errors(self.project, &entry, &old) {
                    return Err(format!("it would add a syntax error to `{path}`"));
                }
            }
            if *strip_bom {
                strip.insert(path.clone());
            }
            next.insert(path.clone(), new);
        }
        let changed: Vec<&String> = per.keys().filter(|p| next.get(*p) != self.texts.get(*p) || strip.contains(*p)).collect();
        if changed.is_empty() {
            return Err("it changes nothing".into());
        }
        // The finding is gone, and no rule fires in an edited file that did
        // not fire there before (L4 decisions 9 and 10).
        let (graph_before, graph_after) = (self.graph_diags(&self.texts), self.graph_diags(&next));
        for path in &changed {
            if self.language(path).is_none() {
                continue;
            }
            let before = firing(&self.all(path, &self.texts[*path], &self.texts, &graph_before));
            let after_diags = self.all(path, &next[*path], &next, &graph_after);
            if let Some(r) = firing(&after_diags).difference(&before).next() {
                return Err(format!("it makes `{r}` fire in `{path}`"));
            }
            if **path == d.file && !strip.contains(*path) && after_diags.iter().any(|x| x.rule == d.rule && x.message == d.message) {
                return Err("the rule still fires after it".into());
            }
        }
        // Safeguard 2: no reference left resolving to nothing.
        if self.dangling.is_none() {
            self.dangling = Some(self.dangling_of(&self.texts));
        }
        let after = self.dangling_of(&next);
        let (before, classes) = self.dangling.as_ref().expect("just set");
        let left: Vec<String> = codelaw_graph::introduced(before, &after.0, classes).iter().map(|k| k.split('\0').skip(1).collect::<Vec<_>>().join(" ")).collect();
        if !left.is_empty() {
            return Err(format!("it leaves {} dangling: {}", if left.len() == 1 { "a reference" } else { "references" }, left.join("; ")));
        }
        // Safeguard 3: every file, or none.
        if let Base::Disk { write: true } = self.base {
            let writes: Vec<codelaw_fix::FileWrite> = changed
                .iter()
                .map(|p| {
                    let (read, bom) = self.bytes.get(*p).cloned().unwrap_or_default();
                    let mut bytes = Vec::new();
                    if bom && !strip.contains(*p) {
                        bytes.extend_from_slice("\u{feff}".as_bytes());
                    }
                    bytes.extend_from_slice(next[*p].as_bytes());
                    codelaw_fix::FileWrite { path: self.project.config.root_dir.join(p), read, bytes }
                })
                .collect();
            codelaw_fix::write_all_or_nothing(&writes).map_err(|e| e.note())?;
            for w in writes {
                let rel = crate::discover::rel_path(&w.path, self.project.root());
                let bom = w.bytes.starts_with("\u{feff}".as_bytes());
                self.bytes.insert(rel, (w.bytes, bom));
            }
        } else {
            for p in &strip {
                if let Some((_, b)) = self.bytes.get_mut(p) {
                    *b = false;
                }
            }
        }
        let changed: Vec<String> = changed.into_iter().cloned().collect();
        for p in &changed {
            if !self.originals.contains_key(p) {
                let was = self.texts.get(p).cloned().unwrap_or_default();
                self.originals.insert(p.clone(), was);
            }
            *self.applied.entry(p.clone()).or_default().entry(d.rule.clone()).or_default() += 1;
        }
        self.texts = next;
        self.dangling = Some(after);
        Ok(())
    }

    /// `d` as reported after this pass: a refused multi-file fix is
    /// `manual`, its note naming why (SR-MULTI-FILE-FIXES decision 2).
    pub fn mark(&self, d: &mut Diagnostic) {
        let Some(why) = self.refused.get(&(d.rule.clone(), d.file.clone(), d.message.clone())) else { return };
        if !d.is_multi_file() {
            return;
        }
        d.fix = FixClass::Manual;
        d.edits.clear();
        d.choices.clear();
        d.files.clear();
        d.note = Some(format!("fix-refused: {why}"));
        if let Some(c) = self.project.compiled.iter().find(|c| c.id() == d.rule) {
            d.fix_exception = Some(crate::lint::manual_exception(&c.loaded.rule));
            if d.steps.is_empty() {
                d.steps = c.loaded.rule.steps.clone();
            }
        }
        if d.steps.is_empty() {
            d.steps = vec![format!("Make the change by hand; `codelaw explain {}` shows what the rule requires.", d.rule)];
        }
    }

    /// Whether the pass wrote or would write a file's byte-order mark.
    pub fn keeps_bom(&self, rel: &str) -> bool {
        self.bom(rel)
    }
}

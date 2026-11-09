use crate::text::Position;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warn,
    Info,
}

impl Severity {
    pub fn parse(s: &str) -> Option<Severity> {
        Some(match s {
            "error" => Severity::Error,
            "warn" => Severity::Warn,
            "info" => Severity::Info,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warn => "warn",
            Severity::Info => "info",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The three fix classes of L4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FixClass {
    Auto,
    Suggest,
    Manual,
}

impl FixClass {
    pub fn parse(s: &str) -> Option<FixClass> {
        Some(match s {
            "auto" => FixClass::Auto,
            "suggest" => FixClass::Suggest,
            "manual" => FixClass::Manual,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            FixClass::Auto => "auto",
            FixClass::Suggest => "suggest",
            FixClass::Manual => "manual",
        }
    }
}

impl fmt::Display for FixClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Why a `manual` rule has no edit (SR-LAW-4 decision 5): the four named
/// exceptions, and nothing else.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FixException {
    /// The fix needs a name, value or text the code does not contain.
    Value,
    /// The fix moves code across files or changes architecture.
    Restructure,
    /// The finding is missing human-authored content.
    Authored,
    /// Any automatic or suggested edit would create false safety.
    Security,
}

impl FixException {
    pub const ALL: [FixException; 4] = [FixException::Value, FixException::Restructure, FixException::Authored, FixException::Security];

    pub fn parse(s: &str) -> Option<FixException> {
        Some(match s {
            "value" => FixException::Value,
            "restructure" => FixException::Restructure,
            "authored" => FixException::Authored,
            "security" => FixException::Security,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            FixException::Value => "value",
            FixException::Restructure => "restructure",
            FixException::Authored => "authored",
            FixException::Security => "security",
        }
    }
}

impl fmt::Display for FixException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// One labelled alternative of a `suggest` finding (SR-LAW-4 decision 4):
/// the right edit is one of a few the code determines, and the agent or
/// person picks. `fix --unsafe` applies only a `default` one.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct FixChoice {
    pub label: String,
    #[serde(default)]
    pub default: bool,
    pub edits: Vec<TextEdit>,
    /// The choice's edits in other files (SR-MULTI-FILE-FIXES decision 1).
    #[serde(default)]
    pub files: Vec<FileEdit>,
}

impl FixChoice {
    pub fn new(label: impl Into<String>, default: bool, edits: Vec<TextEdit>) -> Self {
        FixChoice { label: label.into(), default, edits, files: Vec::new() }
    }
}

/// `files` is omitted from JSON when empty, and always there for the
/// cache's positional codec (as `Diagnostic`'s optional fields).
impl Serialize for FixChoice {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let readable = s.is_human_readable();
        let with_files = !readable || !self.files.is_empty();
        let mut st = s.serialize_struct("FixChoice", 3 + usize::from(with_files))?;
        st.serialize_field("label", &self.label)?;
        st.serialize_field("default", &self.default)?;
        st.serialize_field("edits", &self.edits)?;
        if with_files {
            st.serialize_field("files", &self.files)?;
        }
        st.end()
    }
}

/// A fix's edits in one file other than its finding's (SR-MULTI-FILE-FIXES
/// decision 1): the file's project-relative path, its edits over the file's
/// text (without a byte-order mark), and the file-level operation
/// `strip_bom`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FileEdit {
    pub path: String,
    pub edits: Vec<TextEdit>,
    /// Remove the file's UTF-8 byte-order mark: the mark sits in bytes the
    /// text never sees.
    #[serde(default)]
    pub strip_bom: bool,
}

impl FileEdit {
    pub fn new(path: impl Into<String>, edits: Vec<TextEdit>) -> Self {
        FileEdit { path: path.into(), edits, strip_bom: false }
    }
}

/// An LSP-style text edit over byte offsets.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TextEdit {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

impl TextEdit {
    pub fn new(start: usize, end: usize, text: impl Into<String>) -> Self {
        TextEdit { start, end, text: text.into() }
    }

    pub fn delete(start: usize, end: usize) -> Self {
        TextEdit { start, end, text: String::new() }
    }

    pub fn insert(at: usize, text: impl Into<String>) -> Self {
        TextEdit { start: at, end: at, text: text.into() }
    }
}

/// One finding. Every diagnostic carries its fix (L4 decision 8): the
/// finding's rung, the edits for `auto` and `suggest` (or the labelled
/// `choices`), the steps and the exception for `manual`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Diagnostic {
    pub rule: String,
    pub severity: Severity,
    pub file: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start: Position,
    pub end: Position,
    pub message: String,
    /// This finding's rung: at most the rule's, lower when its own edit
    /// can't be proved (L4 decision 2).
    pub fix: FixClass,
    /// The edit of an `auto` or single-diff `suggest` finding; empty when
    /// the finding carries `choices`.
    pub edits: Vec<TextEdit>,
    pub steps: Vec<String>,
    /// Where the rule came from: `local`, `engine`, or `<pack>@<version>`.
    pub source: String,
    /// Why a fix was not offered, e.g. `fix-conflict` or `fix-verify-failed`.
    pub note: Option<String>,
    /// Structured facts about the finding, for `--format json` and SARIF: a
    /// size rule's measured value and its limit (`{ "lines": 412, "max": 300 }`).
    #[serde(default, deserialize_with = "de_data")]
    pub data: Option<serde_json::Value>,
    /// A `suggest` finding's labelled alternatives (L4 decision 4).
    #[serde(default)]
    pub choices: Vec<FixChoice>,
    /// Why a `manual` finding has no edit (L4 decision 5).
    #[serde(default)]
    pub fix_exception: Option<FixException>,
    /// A multi-file fix's edits in files other than `file`
    /// (SR-MULTI-FILE-FIXES decision 1); `edits` holds this file's.
    #[serde(default)]
    pub files: Vec<FileEdit>,
}

/// `data` is omitted from JSON when absent, but the cache's codec is
/// positional (a skipped field shifts every later one) and cannot carry a
/// `serde_json::Value` (it is self-describing): so a non-human-readable
/// serializer always gets the field, as the value's JSON text.
impl Serialize for Diagnostic {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let readable = s.is_human_readable();
        let n = 13
            + usize::from(!readable || self.data.is_some())
            + usize::from(!readable || !self.choices.is_empty())
            + usize::from(!readable || self.fix_exception.is_some())
            + usize::from(!readable || !self.files.is_empty());
        let mut st = s.serialize_struct("Diagnostic", n)?;
        st.serialize_field("rule", &self.rule)?;
        st.serialize_field("severity", &self.severity)?;
        st.serialize_field("file", &self.file)?;
        st.serialize_field("start_byte", &self.start_byte)?;
        st.serialize_field("end_byte", &self.end_byte)?;
        st.serialize_field("start", &self.start)?;
        st.serialize_field("end", &self.end)?;
        st.serialize_field("message", &self.message)?;
        st.serialize_field("fix", &self.fix)?;
        st.serialize_field("edits", &self.edits)?;
        st.serialize_field("steps", &self.steps)?;
        st.serialize_field("source", &self.source)?;
        st.serialize_field("note", &self.note)?;
        if readable {
            if self.data.is_some() {
                st.serialize_field("data", &self.data)?;
            }
        } else {
            st.serialize_field("data", &self.data.as_ref().map(|v| v.to_string()))?;
        }
        // Like `data`: omitted from JSON when empty, always there for the
        // cache's positional codec.
        if !readable || !self.choices.is_empty() {
            st.serialize_field("choices", &self.choices)?;
        }
        if !readable || self.fix_exception.is_some() {
            st.serialize_field("fix_exception", &self.fix_exception)?;
        }
        if !readable || !self.files.is_empty() {
            st.serialize_field("files", &self.files)?;
        }
        st.end()
    }
}

fn de_data<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<serde_json::Value>, D::Error> {
    if d.is_human_readable() {
        Option::<serde_json::Value>::deserialize(d)
    } else {
        Option::<String>::deserialize(d)?.map(|t| serde_json::from_str(&t).map_err(serde::de::Error::custom)).transpose()
    }
}

impl Diagnostic {
    /// The edits `codelaw fix` applies for this finding: its own, else the
    /// choice labelled `choice`, else its `default` choice (L4 decision 4).
    pub fn applicable_edits(&self, choice: Option<&str>) -> Option<&[TextEdit]> {
        if !self.edits.is_empty() {
            return Some(&self.edits);
        }
        let pick = match choice {
            Some(l) => self.choices.iter().find(|c| c.label == l).or_else(|| self.choices.iter().find(|c| c.default)),
            None => self.choices.iter().find(|c| c.default),
        };
        pick.map(|c| c.edits.as_slice()).filter(|e| !e.is_empty())
    }

    /// The fix `codelaw fix` applies, as [`Diagnostic::applicable_edits`]
    /// picks it, with its edits in other files: `(this file's edits, the
    /// other files')`. `None` when there is nothing to apply.
    pub fn applicable_fix(&self, choice: Option<&str>) -> Option<(&[TextEdit], &[FileEdit])> {
        if !self.edits.is_empty() || !self.files.is_empty() {
            return Some((&self.edits, &self.files));
        }
        let pick = match choice {
            Some(l) => self.choices.iter().find(|c| c.label == l).or_else(|| self.choices.iter().find(|c| c.default)),
            None => self.choices.iter().find(|c| c.default),
        };
        pick.filter(|c| !c.edits.is_empty() || !c.files.is_empty()).map(|c| (c.edits.as_slice(), c.files.as_slice()))
    }

    /// Whether this finding's fix edits a file other than its own, or does
    /// a file-level operation: such a fix is applied only by `codelaw fix`,
    /// never on save or by the agent hook (SR-MULTI-FILE-FIXES decision 2.5).
    pub fn is_multi_file(&self) -> bool {
        !self.files.is_empty() || self.choices.iter().any(|c| !c.files.is_empty())
    }

    /// The number of files this finding's fix edits (its default choice's,
    /// when it has choices), its own file included.
    pub fn files_edited(&self) -> usize {
        let (own, other) = match self.applicable_fix(None) {
            Some(f) => f,
            None => return 0,
        };
        let mut paths: Vec<&str> = other.iter().map(|f| f.path.as_str()).filter(|p| *p != self.file).collect();
        paths.sort_unstable();
        paths.dedup();
        paths.len() + usize::from(!own.is_empty() || other.iter().any(|f| f.path == self.file))
    }

    /// The L5 decision 2 order: path, line, column, rule.
    pub fn sort_key(&self) -> (&str, Position, &str, usize, &str) {
        (&self.file, self.start, &self.rule, self.end_byte, &self.message)
    }
}

/// A file that was not checked, and why (SR-ARCHITECTURE decision 26).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Skipped {
    pub file: String,
    pub reason: String,
}

/// Whether a finding's `file` is a logical location, not a path: a commit
/// (`commit:<id>`), a branch (`branch:<name>`) or the pull request (`pr`),
/// for the `git/*` rules (SR-ARCHITECTURE decision 14, W-146).
pub fn is_logical_location(file: &str) -> bool {
    file == "pr" || file.starts_with("commit:") || file.starts_with("branch:")
}

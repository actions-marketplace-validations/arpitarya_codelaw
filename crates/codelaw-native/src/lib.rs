//! Native rules: checks a pattern can't express, implemented in Rust and
//! registered by id (SR-ARCHITECTURE decision 15b). The rule's Markdown
//! file is still the rule — its docs, fix class, options and examples —
//! and names one of these by `native: <id>`.
//!
//! Every native rule declares the node kinds it inspects, so the engine
//! hands it only those (decision 15); it is deterministic (L5) and the
//! engine isolates it on panic (decision 25).

mod accessors;
mod classes;
mod conventions;
mod css_atrules;
mod css_declarations;
mod css_media;
mod css_misc;
mod css_modules;
mod css_selectors;
mod css_syntax;
mod css_tree;
mod css_values;
mod scss_rules;
mod data_json;
mod data_md;
mod data_toml;
mod data_yaml;
pub mod deps;
mod codeowners;
mod duplicates;
mod expressions;
#[cfg(feature = "fault-injection")]
mod fault;
mod html_a11y;
mod html_content;
mod html_document;
mod html_dom;
mod html_rules;
mod html_security;
mod js_ast;
mod js_console;
mod size;
mod literals;
mod markers;
mod multi_file;
mod test_markers;
mod no_undef;
mod no_unreachable;
mod node;
mod python_bugbear;
mod python_conventions;
mod python_format;
mod python_isort;
mod python_pycodestyle;
mod python_pyflakes;
mod python_util;
mod react;
mod react_hooks;
mod tanstack_query;
mod redux_toolkit;
mod size_lines;
pub mod spell;
mod regexp;
pub mod regex_ast;
mod regexp_rules;
mod react_ts_extra;
mod renames;
mod rewrites;
mod css_lists;
mod css_style;
mod scss_extra;
mod html_extra;
mod md_extra;
mod oxlint_extra;
mod js_parity;
mod jsdoc;
mod py_parity;
mod py_ruff;
mod py_ruff_doc;
mod py_ruff_fmt;
mod py_ruff_generic;
mod py_ruff_tc;
mod py_classes;
mod py_pylint_imports;
mod py_flow;
mod py_scope;
mod py_values;
mod stencil;
mod testing_parity;
mod query_parity;
mod py_framework;
mod tailwind;
mod typescript;
mod unused_vars;
mod useless_assignment;
mod useless_escape;
mod use_instead;
mod util;

use codelaw_core::{FileEdit, FixChoice, FixClass, Language, Severity, TextEdit};
use codelaw_lang::SyntaxNode;
use codelaw_semantic::SemanticModel;
use std::collections::BTreeMap;
use std::sync::Arc;

/// What a native rule sees of one file.
pub struct NativeContext<'a> {
    pub source: &'a str,
    pub language: Language,
    pub semantic: Option<&'a SemanticModel>,
    /// The rule's options, resolved for this file.
    pub options: &'a BTreeMap<String, serde_json::Value>,
    /// The rule's pack's data tables (SR-ARCHITECTURE decision 15c).
    pub data: &'a BTreeMap<String, Arc<serde_json::Value>>,
    /// The file itself: its path, its raw bytes and a read-only view of the
    /// files around it (decision 15b). For an embedded region, the host file.
    pub file: &'a dyn FileContext,
}

/// The file a native rule checks, beyond its text (SR-ARCHITECTURE decision
/// 15b): where it is, its bytes as they are on disk, and a read-only view of
/// the other files of the project.
///
/// Paths are relative to the project root, `/`-separated, with no `.` or
/// `..` segment ([`resolve_path`] makes one from a link). Nothing outside the
/// project root can be named, and nothing can be written. The engine records
/// every question a rule asks, so a cached result is reused only while every
/// answer still holds.
pub trait FileContext: Sync {
    /// The file's path.
    fn path(&self) -> &str;
    /// The file's bytes, a UTF-8 byte-order mark included: the text rules
    /// see has none.
    fn bytes(&self) -> &[u8];
    /// Whether a file or folder exists at `path`.
    fn exists(&self, path: &str) -> bool;
    /// Whether `path` is a folder.
    fn is_dir(&self, path: &str) -> bool;
    /// The text of the file at `path`, when it exists and is UTF-8.
    fn read(&self, path: &str) -> Option<String>;
    /// The names in the folder at `path` (`""` is the root), sorted.
    fn list(&self, path: &str) -> Option<Vec<String>>;
    /// Every file of the project, whatever its type, that the ignore files
    /// and the default excludes leave in: project-relative, sorted. `None`
    /// when the context does not know them (a file checked alone). The
    /// engine records the question, so a cached result is reused only
    /// while the list is the same.
    fn project_files(&self) -> Option<Vec<String>> {
        None
    }
    /// The as-of date, `YYYY-MM-DD`, when there is one (L5 decision 2):
    /// the only date a rule may judge by. The engine records the question,
    /// so a cached result is reused only on the same date.
    fn as_of(&self) -> Option<String> {
        None
    }
    /// The class table of the provider `short` (`tailwind`), when the
    /// project has one (SR-RULE-PACKS decision 19). Its identity is in the
    /// file's cache key, so the question is not recorded.
    fn class_table(&self, _short: &str) -> Option<std::sync::Arc<dyn codelaw_core::class_table::ClassTable>> {
        None
    }
    /// The table of the data package `short` (`polyfills`: `@codelaw/data-polyfills`,
    /// L6 decision 15), parsed once per run and read only when a rule asks.
    /// `None` when it is not installed. The engine records its version as
    /// the answer, so a refreshed table re-judges the files.
    fn data_table(&self, _short: &str) -> Option<std::sync::Arc<serde_json::Value>> {
        None
    }
}

/// A file with no neighbours: what a rule sees when only its text is known.
#[derive(Clone, Debug, Default)]
pub struct DetachedFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

impl DetachedFile {
    pub fn new(path: impl Into<String>, source: &str) -> Self {
        DetachedFile { path: path.into(), bytes: source.as_bytes().to_vec() }
    }
}

impl FileContext for DetachedFile {
    fn path(&self) -> &str {
        &self.path
    }
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    fn exists(&self, path: &str) -> bool {
        path.is_empty() || path == self.path
    }
    fn is_dir(&self, path: &str) -> bool {
        path.is_empty()
    }
    fn read(&self, path: &str) -> Option<String> {
        (path == self.path).then(|| String::from_utf8_lossy(&self.bytes).into_owned())
    }
    fn list(&self, path: &str) -> Option<Vec<String>> {
        path.is_empty().then(|| vec![self.path.clone()])
    }
}

/// Resolve `link` against the folder of `from` (both project-relative): the
/// project-relative path it names, or `None` when it leaves the project
/// root. A `link` starting with `/` is relative to the root.
pub fn resolve_path(from: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    if !link.starts_with('/') {
        parts.extend(from.split('/').filter(|p| !p.is_empty()));
        parts.pop();
    }
    for seg in link.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

/// One finding of a native rule. `edits` are offered when the rule file's
/// fix class is `auto` or `suggest`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeFinding {
    pub start: usize,
    pub end: usize,
    pub message: String,
    pub edits: Vec<TextEdit>,
}

pub trait NativeRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn languages(&self) -> &'static [Language];
    /// The node kinds `check` is called on.
    fn kinds(&self) -> &'static [&'static str];
    fn needs_semantic(&self) -> bool {
        false
    }
    /// The provider whose class table the rule reads (`tailwind`): without
    /// the table it finds nothing, and the rule tester says its examples
    /// were not checked (SR-RULE-PACKS decision 19).
    fn provider(&self) -> Option<&'static str> {
        None
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>);
    /// A config section whose keys the rule reads as its options (the
    /// rule's own options win): `[deps]` for the dependency rules
    /// (SR-RULE-PACKS decision 15).
    fn config_section(&self) -> Option<&'static str> {
        None
    }
    /// The severity of one finding, for a rule that grades its findings;
    /// `None` keeps the rule's.
    /// Structured data about one finding, for `--format json` and SARIF
    /// (the measured value and the limit, for a size rule); `None` carries
    /// none.
    fn data_of(&self, _finding: &NativeFinding, _options: &BTreeMap<String, serde_json::Value>) -> Option<serde_json::Value> {
        None
    }
    fn severity_of(&self, _finding: &NativeFinding, _options: &BTreeMap<String, serde_json::Value>) -> Option<Severity> {
        None
    }
    /// Where else one finding is shown: a line of a plain-text config file
    /// the rule read (`.nvmrc`, `CODEOWNERS`, …: [`codelaw_core::TEXT_CONFIG_FILES`],
    /// SR-LAW-3 decision 10). The finding stays on this file, where its fix
    /// is; each place returned here gets a copy of it (SR-ARCHITECTURE
    /// decision 15d). A path outside the list fails the rule on this file.
    /// Called once per finding, after `check`, with the same context.
    fn elsewhere(&self, _finding: &NativeFinding, _ctx: &NativeContext<'_>) -> Vec<Elsewhere> {
        Vec::new()
    }
    /// The highest rung one finding's edit reaches (SR-LAW-4 decision 2):
    /// `Some(Suggest)` for a finding of an `auto` rule outside the subset its
    /// `fix_reason:` states; `None` keeps the rule's. The finding never
    /// rises above the rule's own class. Called once per finding, after
    /// `check`, with the same context.
    /// By default, `Suggest` when `check` marked the finding's edits with
    /// [`as_suggestion`].
    fn rung_of(&self, finding: &NativeFinding, _ctx: &NativeContext<'_>) -> Option<FixClass> {
        finding.edits.iter().any(|e| e.start == SUGGEST_AT && e.end == SUGGEST_AT).then_some(FixClass::Suggest)
    }
    /// A `suggest` finding's labelled alternatives (SR-LAW-4 decision 4),
    /// when the right edit is one of a few the code determines; the
    /// finding's own `edits` are then ignored. Called once per finding,
    /// after `check`, with the same context.
    /// By default, the choices `check` wrote into the finding's `edits`
    /// with [`choice_edits`].
    fn choices_of(&self, finding: &NativeFinding, _ctx: &NativeContext<'_>) -> Vec<FixChoice> {
        choices_in(&finding.edits)
    }
}

/// The marker that makes a finding of an `auto` rule a suggestion
/// ([`as_suggestion`]): an edit no file can hold.
const SUGGEST_AT: usize = usize::MAX - 1;

/// A finding's edits marked as a suggestion: the default
/// [`NativeRule::rung_of`] reports it as `suggest` (L4 decision 2), and
/// [`plain_edits`] drops the marker before the edits are applied.
pub fn as_suggestion(mut edits: Vec<TextEdit>) -> Vec<TextEdit> {
    if !edits.is_empty() && !edits.iter().any(|e| e.start == CHOICE_AT) {
        edits.push(TextEdit { start: SUGGEST_AT, end: SUGGEST_AT, text: String::new() });
    }
    edits
}

/// A finding's edits in its own file, without the markers `check` wrote
/// into them: the edits [`in_files`] put in other files are left out.
pub fn plain_edits(edits: Vec<TextEdit>) -> Vec<TextEdit> {
    edits.into_iter().take_while(|e| !is_file_marker(e)).filter(|e| e.start != SUGGEST_AT && e.start != CHOICE_AT).collect()
}

/// The marker opening one other file's edits ([`in_files`]): its text is
/// the file's path.
const FILE_AT: usize = usize::MAX - 2;
/// The marker of a file whose byte-order mark the fix removes: its text is
/// the file's path.
const BOM_AT: usize = usize::MAX - 3;

fn is_file_marker(e: &TextEdit) -> bool {
    (e.start == FILE_AT || e.start == BOM_AT) && e.end == e.start
}

/// A multi-file fix written into a finding's edits (SR-MULTI-FILE-FIXES
/// decision 1), so `check` can compute it where it has the tree: this
/// file's edits, then each other file's, opened by a marker carrying its
/// path. A `FileEdit` for this file's own path is in the host file's
/// offsets (an embedded region's rule reaching outside its region). The
/// engine reads them back with [`files_in`]; [`plain_edits`] keeps only
/// the first part. Inside a choice, as one of [`choice_edits`]'s edit sets.
pub fn in_files(own: Vec<TextEdit>, files: Vec<FileEdit>) -> Vec<TextEdit> {
    let mut out = own;
    for f in files {
        if f.edits.is_empty() && !f.strip_bom {
            continue;
        }
        let at = if f.strip_bom { BOM_AT } else { FILE_AT };
        out.push(TextEdit { start: at, end: at, text: f.path });
        out.extend(f.edits);
    }
    out
}

/// The other files' edits [`in_files`] wrote into a finding's `edits`.
pub fn files_in(edits: &[TextEdit]) -> Vec<FileEdit> {
    let mut out: Vec<FileEdit> = Vec::new();
    // The file later edits go to, once a marker opened one.
    let mut at: Option<usize> = None;
    for e in edits {
        if e.start == SUGGEST_AT || e.start == CHOICE_AT {
            continue;
        }
        if is_file_marker(e) {
            let i = match out.iter().position(|f| f.path == e.text) {
                Some(i) => i,
                None => {
                    out.push(FileEdit::new(e.text.clone(), Vec::new()));
                    out.len() - 1
                }
            };
            out[i].strip_bom |= e.start == BOM_AT;
            at = Some(i);
        } else if let Some(i) = at {
            out[i].edits.push(e.clone());
        }
    }
    for f in &mut out {
        f.edits.sort_by_key(|e| (e.start, e.end));
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

/// The marker opening one choice in a finding's `edits` ([`choice_edits`]):
/// an edit no file can hold.
const CHOICE_AT: usize = usize::MAX;

/// A finding's labelled choices written into its `edits`, so `check` can
/// compute them where it has the tree: each choice is a marker edit
/// carrying its label, then its own edits. The default
/// [`NativeRule::choices_of`] reads them back, and the engine ignores a
/// finding's `edits` once it has choices. A choice with no edits is
/// dropped; with fewer than two left, the one left is the finding's edit.
pub fn choice_edits(choices: Vec<FixChoice>) -> Vec<TextEdit> {
    let choices: Vec<FixChoice> = choices.into_iter().filter(|c| !c.edits.is_empty() || !c.files.is_empty()).collect();
    if choices.len() < 2 {
        return choices.into_iter().next().map(|c| in_files(c.edits, c.files)).unwrap_or_default();
    }
    let mut out = Vec::new();
    for c in choices {
        out.push(TextEdit { start: CHOICE_AT, end: CHOICE_AT, text: format!("{}{}", if c.default { '1' } else { '0' }, c.label) });
        out.extend(in_files(c.edits, c.files));
    }
    out
}

/// The choices [`choice_edits`] wrote into `edits`; none when it wrote
/// plain edits.
pub fn choices_in(edits: &[TextEdit]) -> Vec<FixChoice> {
    // Each choice's edit set, as `in_files` wrote it.
    let mut sets: Vec<(FixChoice, Vec<TextEdit>)> = Vec::new();
    for e in edits {
        if e.start == SUGGEST_AT {
            continue;
        }
        if e.start == CHOICE_AT && e.end == CHOICE_AT {
            let (flag, label) = e.text.split_at(1);
            sets.push((FixChoice::new(label, flag == "1", Vec::new()), Vec::new()));
        } else if let Some((_, set)) = sets.last_mut() {
            set.push(e.clone());
        } else {
            return Vec::new();
        }
    }
    sets.into_iter()
        .map(|(mut c, set)| {
            c.files = files_in(&set);
            c.edits = plain_edits(set);
            c
        })
        .collect()
}

/// One labelled choice.
pub fn choice(label: impl Into<String>, default: bool, edits: Vec<TextEdit>) -> FixChoice {
    FixChoice::new(label, default, edits)
}

/// One labelled choice whose edits reach other files
/// (SR-MULTI-FILE-FIXES decision 1).
pub fn choice_in_files(label: impl Into<String>, default: bool, edits: Vec<TextEdit>, files: Vec<FileEdit>) -> FixChoice {
    FixChoice { label: label.into(), default, edits, files }
}

/// A second place for a finding, in a plain-text config file
/// ([`NativeRule::elsewhere`]): the file's project-relative path and a byte
/// range of its text as [`FileContext::read`] gives it. `edits` are in that
/// file; the editor offers them and `--format json` carries them, but
/// `codelaw fix` never writes a text config file (SR-LAW-3 decision 10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Elsewhere {
    pub path: String,
    pub start: usize,
    pub end: usize,
    pub edits: Vec<TextEdit>,
}

impl Elsewhere {
    pub fn new(path: impl Into<String>, start: usize, end: usize) -> Self {
        Elsewhere { path: path.into(), start, end, edits: Vec::new() }
    }

    /// Line `line` (1-based) of `text`, without its line ending; `None`
    /// past the last line.
    pub fn line(path: impl Into<String>, text: &str, line: usize) -> Option<Self> {
        let mut start = 0;
        for (i, l) in text.split_inclusive('\n').enumerate() {
            if i + 1 == line {
                let body = l.trim_end_matches(['\n', '\r']);
                return Some(Elsewhere::new(path, start, start + body.len()));
            }
            start += l.len();
        }
        None
    }
}

const JS_ONLY: &[Language] = &[Language::Javascript, Language::Jsx];
const TS_ONLY: &[Language] = &[Language::Typescript, Language::Tsx];

static JS_MAX_LINES: size::Size = size::Size::js("js/max-lines", size::Measure::FileLines);
static JS_MAX_LINES_PER_FUNCTION: size::Size = size::Size::js("js/max-lines-per-function", size::Measure::FunctionLines);
static JS_MAX_PARAMS: size::Size = size::Size::js("js/max-params", size::Measure::Params);
static PY_MAX_LINES: size::Size = size::Size::py("py/max-lines", size::Measure::FileLines);
static PY_MAX_LINES_PER_FUNCTION: size::Size = size::Size::py("py/max-lines-per-function", size::Measure::FunctionLines);
static JS_COMPLEXITY: size::Size = size::Size::js("js/complexity", size::Measure::Complexity);
static JS_MAX_DEPTH: size::Size = size::Size::js("js/max-depth", size::Measure::Depth);
static PY_COMPLEXITY: size::Size = size::Size::py("py/complexity", size::Measure::Complexity);
static PY_MAX_DEPTH: size::Size = size::Size::py("py/max-depth", size::Measure::Depth);
static REACT_COMPONENT_MAX_LINES: size::Size = size::Size::react("react/component-max-lines", size::Measure::ComponentLines);
static REACT_MAX_PROPS: size::Size = size::Size::react("react/max-props", size::Measure::Props);
static REACT_MAX_JSX_DEPTH: size::Size = size::Size::react("react/max-jsx-depth", size::Measure::JsxDepth);
static PY_MAX_PARAMS: size::Size = size::Size::py("py/max-params", size::Measure::Params);
static JS_NO_UNUSED_VARS: unused_vars::NoUnusedVars = unused_vars::NoUnusedVars { id: "js/no-unused-vars", languages: JS_ONLY };
static TS_NO_UNUSED_VARS: unused_vars::NoUnusedVars = unused_vars::NoUnusedVars { id: "ts/no-unused-vars", languages: TS_ONLY };
static TS_NO_UNUSED_EXPRESSIONS: typescript::NoUnusedExpressions = typescript::NoUnusedExpressions { id: "ts/no-unused-expressions", languages: TS_ONLY };

static JS_USE_INSTEAD: use_instead::UseInstead = use_instead::UseInstead { id: "js/use-instead", python: false };
static PY_USE_INSTEAD: use_instead::UseInstead = use_instead::UseInstead { id: "py/use-instead", python: true };

static CSS_MAX_LINES: size_lines::MaxLines = size_lines::MaxLines::css("css/max-lines");
static HTML_MAX_LINES: size_lines::MaxLines = size_lines::MaxLines::html("html/max-lines");

static TODO_JS: markers::TodoMarker = markers::TodoMarker { id: "js/todo-marker", languages: &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx], source: markers::Source::Tree };
static TODO_PY: markers::TodoMarker = markers::TodoMarker { id: "py/todo-marker", languages: &[Language::Python], source: markers::Source::Tree };
static TODO_CSS: markers::TodoMarker = markers::TodoMarker { id: "css/todo-marker", languages: &[Language::Css], source: markers::Source::Css };
static TODO_SCSS: markers::TodoMarker = markers::TodoMarker { id: "scss/todo-marker", languages: &[Language::Scss], source: markers::Source::Scss };
static TODO_HTML: markers::TodoMarker = markers::TodoMarker { id: "html/todo-marker", languages: &[Language::Html], source: markers::Source::Tree };
static TODO_YAML: markers::TodoMarker = markers::TodoMarker { id: "yaml/todo-marker", languages: &[Language::Yaml], source: markers::Source::Tree };
static TODO_TOML: markers::TodoMarker = markers::TodoMarker { id: "toml/todo-marker", languages: &[Language::Toml], source: markers::Source::Tree };
static TODO_MD: markers::TodoMarker = markers::TodoMarker { id: "md/todo-marker", languages: &[Language::Markdown], source: markers::Source::Markdown };

/// Every native rule; ids are unique, order doesn't matter.
static REGISTRY: &[&dyn NativeRule] = &[
    &renames::NewCap,
    &renames::JsxPascalCase,
    &renames::HookPrefix,
    &renames::NoDefaultExport,
    &renames::NoDefaultImport,
    &renames::JsxNoUndef,
    &TODO_JS,
    &TODO_PY,
    &TODO_CSS,
    &TODO_SCSS,
    &TODO_HTML,
    &TODO_YAML,
    &TODO_TOML,
    &TODO_MD,
    &test_markers::NoFocusedTests,
    &test_markers::JsSkippedTest,
    &test_markers::PySkippedTest,
    &css_modules::ClassNaming,
    &css_modules::ComposesValid,
    &css_modules::NoGlobal,
    &scss_rules::AtExtendNoMissingPlaceholder,
    &scss_rules::AtIfNoNull,
    &scss_rules::AtMixinPattern,
    &scss_rules::AtRuleNoUnknown,
    &scss_rules::DollarVariablePattern,
    &scss_rules::DoubleSlashCommentEmpty,
    &scss_rules::FunctionNoUnknown,
    &scss_rules::LoadNoPartialLeadingUnderscore,
    &scss_rules::MaxNestingDepth,
    &scss_rules::NoDuplicateDollarVariables,
    &scss_rules::NoDuplicateMixins,
    &scss_rules::NoGlobalFunctionNames,
    &scss_rules::NoImport,
    &scss_rules::NoSlashDivision,
    &scss_rules::PercentPlaceholderPattern,
    &CSS_MAX_LINES,
    &HTML_MAX_LINES,
    &css_values::AlphaValueNotation,
    &css_declarations::AnnotationNoUnknown,
    &css_atrules::AtRuleDescriptorNoUnknown,
    &css_atrules::AtRuleDescriptorValueNoUnknown,
    &css_atrules::AtRuleNoDeprecated,
    &css_atrules::AtRuleNoUnknown,
    &css_atrules::AtRuleNoVendorPrefix,
    &css_atrules::AtRulePreludeNoInvalid,
    &css_selectors::BlockNoRedundantNestedStyleRules,
    &css_values::ColorFunctionAliasNotation,
    &css_values::ColorFunctionNotation,
    &css_values::ColorHexLength,
    &css_misc::CommentNoEmpty,
    &css_atrules::ContainerNamePattern,
    &css_atrules::CustomMediaPattern,
    &css_declarations::CustomPropertyNoMissingVarFunction,
    &css_declarations::CustomPropertyPattern,
    &css_declarations::DeclarationBlockNoDuplicateCustomProperties,
    &css_declarations::DeclarationBlockNoDuplicateProperties,
    &css_declarations::DeclarationBlockNoRedundantLonghandProperties,
    &css_declarations::DeclarationBlockNoShorthandPropertyOverrides,
    &css_declarations::DeclarationPropertyValueKeywordNoDeprecated,
    &css_declarations::DeclarationPropertyValueNoUnknown,
    &css_declarations::FontFamilyNoDuplicateNames,
    &css_declarations::FontFamilyNoMissingGenericFamilyKeyword,
    &css_values::FunctionCalcNoUnspacedOperator,
    &css_values::HueDegreeNotation,
    &css_atrules::ImportNotation,
    &css_atrules::KeyframeBlockNoDuplicateSelectors,
    &css_atrules::KeyframeDeclarationNoImportant,
    &css_atrules::KeyframeSelectorNotation,
    &css_atrules::KeyframesNamePattern,
    &css_atrules::LayerNamePattern,
    &css_values::LengthZeroNoUnit,
    &css_values::LightnessNotation,
    &css_media::MediaFeatureNameNoUnknown,
    &css_media::MediaFeatureNameNoVendorPrefix,
    &css_media::MediaFeatureNameValueNoUnknown,
    &css_media::MediaFeatureRangeNotation,
    &css_media::MediaQueryNoInvalid,
    &css_media::MediaTypeNoDeprecated,
    &css_declarations::NamedGridAreasNoInvalid,
    &css_selectors::NestingSelectorNoMissingScopingRoot,
    &css_selectors::NoDescendingSpecificity,
    &css_atrules::NoDuplicateAtImportRules,
    &css_selectors::NoDuplicateSelector,
    &css_misc::NoEmptyBlock,
    &css_misc::NoEmptySource,
    &css_declarations::NoImportant,
    &css_misc::NoInvalidDoubleSlashComments,
    &css_atrules::NoInvalidPositionAtImportRule,
    &css_declarations::NoInvalidPositionDeclaration,
    &css_misc::NoIrregularWhitespace,
    &css_declarations::NoUnknownProperty,
    &css_values::NumberMaxPrecision,
    &css_declarations::PropertyNoDeprecated,
    &css_declarations::PropertyNoVendorPrefix,
    &css_selectors::SelectorAnbNoUnmatchable,
    &css_selectors::SelectorClassPattern,
    &css_selectors::SelectorIdPattern,
    &css_selectors::SelectorNoVendorPrefix,
    &css_selectors::SelectorNotNotation,
    &css_selectors::SelectorPseudoClassNoUnknown,
    &css_selectors::SelectorPseudoElementColonNotation,
    &css_selectors::SelectorPseudoElementNoUnknown,
    &css_selectors::SelectorTypeNoUnknown,
    &css_declarations::ShorthandPropertyNoRedundantValues,
    &css_selectors::StringNoNewline,
    &css_atrules::SyntaxStringNoInvalid,
    &css_declarations::ValueNoVendorPrefix,
    &html_a11y::AreaAlt,
    &html_a11y::AriaHiddenBody,
    &html_a11y::AriaLabelMisuse,
    &html_content::AttributeAllowedValues,
    &html_content::AttributeMisuse,
    &html_a11y::AutocompletePassword,
    &html_rules::ButtonType,
    &html_content::CloseAttr,
    &html_content::CloseOrder,
    &html_content::Deprecated,
    &html_rules::DoctypeHtml,
    &html_content::ElementName,
    &html_content::ElementPermittedContent,
    &html_content::ElementPermittedOccurrences,
    &html_content::ElementPermittedOrder,
    &html_content::ElementPermittedParent,
    &html_content::ElementRequiredAncestor,
    &html_content::ElementRequiredAttributes,
    &html_content::ElementRequiredContent,
    &html_a11y::EmptyHeading,
    &html_a11y::EmptyTitle,
    &html_a11y::FieldsetLegend,
    &html_rules::FormDupName,
    &html_a11y::FormSubmit,
    &html_document::HeadingLevel,
    &html_a11y::HiddenFocusable,
    &html_a11y::ImgAlt,
    &html_a11y::ImgTitle,
    &html_content::InputAttributes,
    &html_a11y::InputImageAlt,
    &html_document::InputMissingLabel,
    &html_a11y::LinkText,
    &html_rules::LongTitle,
    &html_rules::MapDupName,
    &html_rules::MapIdName,
    &html_a11y::MetaRefresh,
    &html_document::MissingDoctype,
    &html_a11y::MultipleLabeledControls,
    &html_a11y::NoAbstractRole,
    &html_a11y::NoAutoplay,
    &html_rules::NoConditionalComment,
    &html_content::NoDeprecatedAttr,
    &html_rules::NoDupAttr,
    &html_rules::NoDupClass,
    &html_rules::NoDuplicateId,
    &html_content::NoImplicitClose,
    &html_rules::NoImplicitInputType,
    &html_rules::NoInlineScript,
    &html_rules::NoInlineStyle,
    &html_document::NoMissingReferences,
    &html_document::NoUtf8Bom,
    &html_rules::NoMultipleMain,
    &html_rules::NoRawCharacters,
    &html_a11y::NoRedundantAriaLabel,
    &html_a11y::NoRedundantFor,
    &html_a11y::NoRedundantRole,
    &html_rules::PreferButton,
    &html_a11y::PreferNativeElement,
    &html_rules::PreferTbody,
    &html_document::RequireSri,
    &html_content::ScriptElement,
    &html_rules::ScriptType,
    &html_rules::TargetBlankRel,
    &html_rules::TelNonBreaking,
    &html_a11y::TextContent,
    &html_a11y::ThScope,
    &html_a11y::UniqueLandmark,
    &html_rules::UnrecognizedCharRef,
    &html_a11y::ValidAutocomplete,
    &html_rules::ValidFor,
    &html_rules::ValidId,
    &html_content::VoidContent,
    &html_security::AngularjsUrlExpression,
    &html_security::CspMetaUnsafe,
    &html_security::FormCsrfToken,
    &html_security::IframeAllowPowerful,
    &html_security::IframeSandbox,
    &html_security::NoCredentialsInUrl,
    &html_security::NoExternalBase,
    &html_security::NoInlineEventHandler,
    &html_security::NoJavascriptUrl,
    &html_security::NoMixedContent,
    &html_security::NoPluginEmbed,
    &html_security::NoSecretInHiddenInput,
    &html_security::NoSensitiveComments,
    &html_security::NoSrcdocScript,
    &html_security::NoTemplateHtmlSink,
    &html_security::NoUnsafeTemplateInterpolation,
    &html_security::PasswordFormPost,
    &html_security::ReferrerPolicy,
    &html_security::SriCrossorigin,
    &classes::ConstructorSuper,
    &accessors::GetterReturn,
    &conventions::ImportGroups,
    &expressions::NoConstantBinaryExpression,
    &rewrites::PreferTemplate,
    &rewrites::GhaPinSha,
    &duplicates::NoDupeArgs,
    &duplicates::NoDupeClassMembers,
    &duplicates::NoDupeElseIf,
    &duplicates::NoDupeKeys,
    &duplicates::NoDuplicateCase,
    &literals::NoEmptyCharacterClass,
    &literals::NoFallthrough,
    &regexp::NoInvalidRegexp,
    &literals::NoIrregularWhitespace,
    &literals::NoLossOfPrecision,
    &regexp::NoMisleadingCharacterClass,
    &literals::NoRegexSpaces,
    &accessors::NoSetterReturn,
    &literals::NoSparseArrays,
    &classes::NoThisBeforeSuper,
    &js_console::NoConsole,
    &JS_USE_INSTEAD,
    &PY_USE_INSTEAD,
    &JS_MAX_LINES,
    &JS_MAX_LINES_PER_FUNCTION,
    &JS_MAX_PARAMS,
    &JS_COMPLEXITY,
    &JS_MAX_DEPTH,
    &REACT_COMPONENT_MAX_LINES,
    &REACT_MAX_PROPS,
    &REACT_MAX_JSX_DEPTH,
    &PY_MAX_LINES,
    &PY_MAX_LINES_PER_FUNCTION,
    &PY_MAX_PARAMS,
    &PY_COMPLEXITY,
    &PY_MAX_DEPTH,
    &no_undef::NoUndef,
    &literals::NoUnexpectedMultiline,
    &no_unreachable::NoUnreachable,
    &expressions::NoUnsafeOptionalChaining,
    &classes::NoUnusedPrivateClassMembers,
    &JS_NO_UNUSED_VARS,
    &useless_assignment::NoUselessAssignment,
    &regexp::NoUselessBackreference,
    &useless_escape::NoUselessEscape,
    &typescript::NoVar,
    &typescript::PreferConst,
    &data_json::NoDuplicateKeys,
    &data_json::PackageEngines,
    &data_json::PackageScriptsSorted,
    &data_json::TsconfigStrict,
    &data_md::CommandsShowOutput,
    &data_md::DescriptiveLinkText,
    &data_md::FirstLineHeading,
    &data_md::HeadingIncrement,
    &data_md::LinkFragments,
    &data_md::LinkImageReferenceDefinitions,
    &data_md::NoBareUrls,
    &data_md::NoDuplicateHeading,
    &data_md::NoEmphasisAsHeading,
    &data_md::NoInlineHtml,
    &data_md::NoReversedLinks,
    &data_md::ReferenceLinksImages,
    &data_md::RelativeLinks,
    &data_md::SingleH1,
    &data_md::TableColumnCount,
    &node::Hashbang,
    &node::NoDeprecatedApi,
    &python_bugbear::AbstractBaseClassWithoutAbstractMethod,
    &python_isort::AllSorted,
    &python_pycodestyle::AmbiguousClassName,
    &python_pycodestyle::AmbiguousFunctionName,
    &python_pycodestyle::AmbiguousVariableName,
    &python_bugbear::AssertFalse,
    &python_bugbear::AssertRaisesException,
    &python_pyflakes::AssertTuple,
    &python_bugbear::AssignmentToOsEnviron,
    &python_bugbear::BatchedWithoutExplicitStrict,
    &python_pyflakes::BreakOutsideLoop,
    &python_bugbear::CachedInstanceMethod,
    &python_pyflakes::ContinueOutsideLoop,
    &python_pyflakes::DefaultExceptNotLast,
    &python_pyflakes::MultiValueRepeatedKeyLiteral,
    &python_pyflakes::MultiValueRepeatedKeyVariable,
    &python_bugbear::DuplicateHandlerException,
    &python_bugbear::DuplicateValue,
    &python_bugbear::DuplicateTryBlockException,
    &python_bugbear::EmptyMethodWithoutAbstractDecorator,
    &python_bugbear::ExceptWithEmptyTuple,
    &python_bugbear::ExceptWithNonExceptionClasses,
    &python_bugbear::FStringDocstring,
    &python_pyflakes::FStringMissingPlaceholders,
    &python_format::StringDotFormatInvalidFormat,
    &python_format::StringDotFormatMissingArguments,
    &python_format::StringDotFormatMixingAutomatic,
    &python_format::StringDotFormatExtraNamedArguments,
    &python_format::StringDotFormatExtraPositionalArguments,
    &python_pyflakes::ForwardAnnotationSyntaxError,
    &python_bugbear::FunctionCallInDefaultArgument,
    &python_bugbear::FunctionUsesLoopVariable,
    &python_bugbear::GetAttrWithConstant,
    &python_pyflakes::IfTuple,
    &python_pyflakes::ImportShadowedByLoopVar,
    &python_pyflakes::InvalidPrintSyntax,
    &python_pyflakes::IsLiteral,
    &python_bugbear::JumpStatementInFinally,
    &python_pycodestyle::LambdaAssignment,
    &python_pyflakes::LateFutureImport,
    &python_bugbear::LoopVariableOverridesIterator,
    &python_bugbear::MapWithoutExplicitStrict,
    &python_pycodestyle::ModuleImportNotAtTopOfFile,
    &python_pycodestyle::MultipleImportsOnOneLine,
    &python_pyflakes::MultipleStarredExpressions,
    &python_bugbear::MutableContextvarDefault,
    &python_pyflakes::NestedStarImport,
    &python_pycodestyle::BareExcept,
    &python_bugbear::NoExplicitStacklevel,
    &python_bugbear::MutableArgumentDefault,
    &python_conventions::NoPrint,
    &python_pyflakes::UndefinedLocalWithImportStar,
    &python_pycodestyle::NoneComparison,
    &python_pycodestyle::NotInTest,
    &python_pycodestyle::NotIsTest,
    &python_format::PercentFormatPositionalCountMismatch,
    &python_format::PercentFormatExpectedMapping,
    &python_format::PercentFormatExpectedSequence,
    &python_format::PercentFormatInvalidFormat,
    &python_format::PercentFormatMissingArgument,
    &python_format::PercentFormatMixedPositionalAndNamed,
    &python_format::PercentFormatStarRequiresSequence,
    &python_format::PercentFormatUnsupportedFormatCharacter,
    &python_format::PercentFormatExtraNamedArguments,
    &python_bugbear::RaiseLiteral,
    &python_pyflakes::RaiseNotImplemented,
    &python_bugbear::RaiseWithoutFromInsideExcept,
    &python_bugbear::ReSubPositionalArgs,
    &python_pyflakes::RedefinedWhileUnused,
    &python_bugbear::RedundantTupleInExceptionHandler,
    &python_pyflakes::ReturnOutsideFunction,
    &python_bugbear::ReuseOfGroupbyGenerator,
    &python_bugbear::SetAttrWithConstant,
    &python_bugbear::StarArgUnpackingAfterKeywordArg,
    &python_pyflakes::StarImportUsage,
    &python_bugbear::StaticKeyDictComprehension,
    &python_bugbear::StripWithMultiCharacters,
    &python_pyflakes::ExpressionsInStarAssignment,
    &python_pycodestyle::TrueFalseComparison,
    &python_pycodestyle::TypeComparison,
    &python_bugbear::UnaryPrefixIncrementDecrement,
    &python_pyflakes::UndefinedExport,
    &python_pyflakes::FutureFeatureNotDefined,
    &python_pyflakes::UndefinedLocal,
    &python_pyflakes::UndefinedName,
    &python_bugbear::UnintentionalTypeAnnotation,
    &python_bugbear::UnreliableCallableCheck,
    &python_isort::UnsortedImports,
    &python_pyflakes::UnusedAnnotation,
    &python_pyflakes::UnusedImport,
    &python_bugbear::UnusedLoopControlVariable,
    &python_pyflakes::UnusedVariable,
    &python_bugbear::UselessComparison,
    &python_bugbear::UselessContextlibSuppress,
    &python_bugbear::UselessExpression,
    &python_pyflakes::YieldOutsideFunction,
    &python_bugbear::ZipWithoutExplicitStrict,
    &react::ExhaustiveDeps,
    &react::NoContext,
    &react_hooks::ErrorBoundaries,
    &react_hooks::Globals,
    &react_hooks::NoNestedComponents,
    &react_hooks::NoRefInRender,
    &react_hooks::NoSetStateInEffect,
    &react_hooks::NoSetStateInRender,
    &react_hooks::NoUnnecessaryUsePrefix,
    &react_hooks::PreferLazyStateInit,
    &react_hooks::Purity,
    &react_hooks::UseMemo,
    &react::JsxNoDuplicateProps,
    &react::NoUnknownProperty,
    &conventions::PropsTypeName,
    &react::RequireRenderReturn,
    &react::RulesOfHooks,
    &tanstack_query::ArrayQueryKey,
    &tanstack_query::ExhaustiveDeps,
    &tanstack_query::InfiniteQueryPropertyOrder,
    &tanstack_query::MutationPropertyOrder,
    &tanstack_query::NoMutatingSetQueryData,
    &tanstack_query::NoRemovedV4Options,
    &tanstack_query::NoRenamedV4Options,
    &tanstack_query::NoRestDestructuring,
    &tanstack_query::NoUnstableDeps,
    &tanstack_query::NoVoidQueryFn,
    &tanstack_query::NoUnsupportedSuspenseOptions,
    &tanstack_query::ObjectSyntax,
    &tanstack_query::StableQueryClient,
    &redux_toolkit::NoBlindPayloadReturn,
    &redux_toolkit::NoMutatingReducer,
    &redux_toolkit::NoNonSerializableState,
    &redux_toolkit::NoObjectSelector,
    &redux_toolkit::NoSequentialDispatch,
    &redux_toolkit::NoSideEffectsInReducer,
    &redux_toolkit::ObjectShorthandMapDispatch,
    &redux_toolkit::PreferCreateSlice,
    &redux_toolkit::SliceKeyNames,
    &tailwind::ImportantModifierSuffix,
    &tailwind::NegativeArbitraryValues,
    &tailwind::NoArbitraryValue,
    &tailwind::NoRestrictedClasses,
    &tailwind::NoUnknownClasses,
    &tailwind::NoConflictingClasses,
    &tailwind::CanonicalClassname,
    &tailwind::NoConcatenatedClasses,
    &stencil::BanDefaultTrue,
    &stencil::BanExportedConstEnums,
    &stencil::BanPrefix,
    &stencil::BanSideEffects,
    &stencil::DecoratorsContext,
    &stencil::EnforceSlotJsdoc,
    &stencil::HostDataDeprecated,
    &stencil::MethodsMustBePublic,
    &stencil::NoUnusedWatch,
    &stencil::OwnMethodsMustBePrivate,
    &stencil::OwnPropsMustBePrivate,
    &stencil::PreferVdomListener,
    &stencil::PropReadonly,
    &stencil::PropsMustBePublic,
    &stencil::RequiredJsdoc,
    &stencil::ReservedMemberNames,
    &stencil::StrictMutable,
    &stencil::TagPrefix,
    &data_toml::DepsSorted,
    &codeowners::CodeownersCoverage,
    &deps::OutdatedDependency,
    &deps::StaleDepsSnapshot,
    &deps::Vulnerable,
    &deps::IncompleteScan,
    &deps::Licence,
    &deps::LockfileSync,
    &deps::VersionDrift,
    &deps::RuntimeAgreement,
    &data_toml::NoDuplicateKeys,
    &data_toml::PyprojectRequiresPython,
    &typescript::NoDuplicateEnumValues,
    &typescript::NoUnsafeDeclarationMerging,
    &TS_NO_UNUSED_EXPRESSIONS,
    &TS_NO_UNUSED_VARS,
    &typescript::NoWrapperObjectTypes,
    &typescript::PreferAsConst,
    &conventions::TypeOnlyImports,
    &data_yaml::DocumentStart,
    &data_yaml::NoAmbiguousBooleans,
    &data_yaml::NoAmbiguousNumbers,
    &data_yaml::NoDuplicateKeys,
    &data_yaml::NoUndeclaredAlias,
    &spell::KnownTypo,
    &spell::KnownTypoIdentifier,
    &spell::KnownTypoKey,
    &spell::LocaleVariant,
    &spell::SuspiciousWord,
    &spell::UnknownWord,
];

/// Every registered native rule.
pub fn registry() -> &'static [&'static dyn NativeRule] {
    &ALL
}

/// `REGISTRY` and the rule families kept in their own lists.
static ALL: std::sync::LazyLock<Vec<&'static dyn NativeRule>> = std::sync::LazyLock::new(|| REGISTRY.iter().chain(regexp_rules::RULES).chain(react_ts_extra::RULES).chain(css_lists::RULES).chain(css_style::RULES).chain(scss_extra::RULES).chain(html_extra::RULES).chain(md_extra::RULES).chain(oxlint_extra::RULES).chain(js_parity::RULES).chain(jsdoc::RULES).chain(py_parity::RULES).chain(py_ruff_doc::RULES).chain(py_ruff::RULES).chain(py_ruff_fmt::RULES).chain(py_ruff_generic::RULES).chain(py_ruff_tc::RULES).chain(py_scope::RULES).chain(py_flow::RULES).chain(py_classes::RULES).chain(py_values::RULES).chain(py_pylint_imports::RULES).chain(testing_parity::RULES).chain(query_parity::RULES).chain(py_framework::RULES).copied().collect());

pub fn lookup(id: &str) -> Option<&'static dyn NativeRule> {
    #[cfg(feature = "fault-injection")]
    if let Some(r) = fault::lookup(id) {
        return Some(r);
    }
    ALL.iter().copied().find(|r| r.id() == id)
}

/// A file and a fixed set of neighbours, for the tests of rules that read
/// them.
#[cfg(test)]
pub(crate) struct MemoryFiles {
    path: String,
    bytes: Vec<u8>,
    files: Vec<(String, String)>,
}

#[cfg(test)]
impl MemoryFiles {
    pub fn new(path: &str, source: &str, files: &[(&str, &str)]) -> Self {
        MemoryFiles { path: path.into(), bytes: source.as_bytes().to_vec(), files: files.iter().map(|(p, t)| (p.to_string(), t.to_string())).collect() }
    }
}

#[cfg(test)]
impl FileContext for MemoryFiles {
    fn path(&self) -> &str {
        &self.path
    }
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    fn exists(&self, path: &str) -> bool {
        path.is_empty() || path == self.path || self.is_dir(path) || self.files.iter().any(|(p, _)| p == path)
    }
    fn is_dir(&self, path: &str) -> bool {
        let dir = format!("{path}/");
        path.is_empty() || self.path.starts_with(&dir) || self.files.iter().any(|(p, _)| p.starts_with(&dir))
    }
    fn read(&self, path: &str) -> Option<String> {
        self.files.iter().find(|(p, _)| p == path).map(|(_, t)| t.clone())
    }
    fn list(&self, _: &str) -> Option<Vec<String>> {
        None
    }
}

#[cfg(test)]
pub(crate) fn run(rule: &dyn NativeRule, language: Language, src: &str) -> Vec<NativeFinding> {
    let root = codelaw_lang::parse(language, src);
    let model = codelaw_semantic::build(language, &root, src);
    let options = BTreeMap::new();
    let data = BTreeMap::new();
    let file = DetachedFile::new(format!("example.{}", language.extension()), src);
    let ctx = NativeContext { source: src, language, semantic: model.as_ref(), options: &options, data: &data, file: &file };
    let mut out = Vec::new();
    for node in root.root().dfs() {
        if rule.kinds().contains(&node.kind().as_ref()) {
            rule.check(node, &ctx, &mut out);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn ids_are_unique() {
        let ids: Vec<&str> = super::registry().iter().map(|r| r.id()).collect();
        let unique: std::collections::BTreeSet<&str> = ids.iter().copied().collect();
        assert_eq!(unique.len(), ids.len(), "a native id is registered twice");
    }

    #[test]
    fn resolve_path() {
        use super::resolve_path as r;
        assert_eq!(r("docs/a.md", "b.md").as_deref(), Some("docs/b.md"));
        assert_eq!(r("docs/a.md", "./x/../c.md").as_deref(), Some("docs/c.md"));
        assert_eq!(r("docs/a.md", "../README.md").as_deref(), Some("README.md"));
        assert_eq!(r("docs/a.md", "/src/x.rs").as_deref(), Some("src/x.rs"));
        assert_eq!(r("a.md", "../outside.md"), None);
        assert_eq!(r("a.md", "sub/").as_deref(), Some("sub"));
    }
}

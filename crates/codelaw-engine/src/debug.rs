//! The queries behind `config --print` and `match --explain` (SR-CLI-MCP
//! decision 1, SR-RULE-AUTHORING decision 3; SR-CONFIG decision 7). The
//! rule listings are `query.rs`'s; `fix --trace` is the fix pipeline's own
//! loop (`fix.rs`).

use crate::compile::Matcher;
use crate::{EngineError, Project};
use codelaw_config::{TraceEnv, Traced};
use codelaw_core::{Language, LineIndex, Position};
use codelaw_lang::Grammar;
use codelaw_match::{Explainer, NodeExplanation};
use serde_json::{Value, json};
use std::path::Path;

/// `match <id> <file> [--explain]`.
#[derive(Clone, Debug)]
pub struct MatchReport {
    pub rule: String,
    pub file: String,
    pub language: Language,
    /// Each candidate node, with its positions.
    pub nodes: Vec<(Position, Position, NodeExplanation)>,
}

impl Project {
    /// `config --print [--for <file>]`: the config's values (SR-CONFIG
    /// decision 7), then every rule's severity, each with its source.
    pub fn config_trace(&self, file: Option<&Path>, env: &TraceEnv) -> Vec<Traced> {
        let abs = file.map(|f| self.locate_path(f).0);
        let mut out = self.config.trace(abs.as_deref(), env);
        let layers = self.config.layers_for(abs.as_deref().unwrap_or(self.root()));
        // The resolved targets and their sources (SR-CONFIG decision 11f):
        // for a folder, as a file in it sees them.
        let probe = match abs.as_deref() {
            Some(a) if a.is_dir() => a.join("_"),
            Some(a) => a.to_owned(),
            None => self.root().join("_"),
        };
        let py = self.python_target(&probe, None);
        out.push(Traced { key: "targets.python".into(), value: py.value.map_or(Value::Null, |v| json!(crate::targets::dotted(&v))), source: py.source });
        let br = self.browsers_target(&probe, None);
        let browsers = br.value.map_or(Value::Null, |b| json!(b.iter().map(|(n, v)| format!("{n} >= {}", crate::targets::dotted(v))).collect::<Vec<_>>()));
        out.push(Traced { key: "targets.browsers".into(), value: browsers, source: br.source });
        for c in self.compiled.iter() {
            let r = &c.loaded;
            let id = &r.rule.id;
            let effective = self.rules.effective(r, &layers);
            let lowered = effective.as_ref().and_then(|e| e.fix);
            let sources = self.config.rule_setting_sources(abs.as_deref(), id);
            let source = if r.preview_blocked {
                "preview rule: runs only with `preview = true`".to_owned()
            } else if let Some((s, _)) = sources.last() {
                s.clone()
            } else if let Some(by) = &r.enabled_by {
                format!("preset {by}")
            } else {
                "default: not in any extended preset".to_owned()
            };
            let severity = crate::severity_name(&effective);
            let value = match effective.filter(|e| !e.options.is_empty()) {
                Some(e) => json!({ "severity": severity, "options": e.options }),
                None => Value::String(severity.clone()),
            };
            out.push(Traced { key: format!("rules.{}", serde_json::to_string(id).expect("a string")), value, source });
            // A lowered fix and the entry that lowered it (SR-LAW-4 decision 16).
            if let Some(f) = lowered {
                let at = sources.iter().rev().find(|(_, s)| s.fix().is_some()).map(|(s, _)| s.clone()).unwrap_or_default();
                out.push(Traced { key: format!("rules.{}.fix", serde_json::to_string(id).expect("a string")), value: json!(f.name()), source: at });
            }
            // Whether its `auto` fix runs on save and from the agent hook,
            // when the rule or the config says other than the default
            // (SR-CONFIG decision 11e).
            let (on_edit, at) = self.on_edit(r, &layers);
            if !on_edit || !at.starts_with("the rule's default") {
                out.push(Traced { key: format!("rules.{}.on_edit", serde_json::to_string(id).expect("a string")), value: json!(on_edit), source: at });
            }
            // Why the rule is off, from the entry that turned it off
            // (SR-CONFIG decision 11c).
            if severity == "off"
                && let Some((at, s)) = sources.iter().rev().find(|(_, s)| s.severity().is_some())
                && let Some(reason) = s.reason()
            {
                out.push(Traced { key: format!("rules.{}.reason", serde_json::to_string(id).expect("a string")), value: json!(reason), source: at.clone() });
            }
        }
        out
    }

    /// Which clause of a `match:` rule held or failed on each candidate
    /// node of `file`, and what was captured (`match --explain`).
    pub fn explain_match(&self, id: &str, file: &Path) -> Result<MatchReport, EngineError> {
        let Some(c) = self.compiled.iter().find(|c| c.id() == id) else {
            let near = codelaw_config::did_you_mean(id, self.compiled.iter().map(|c| c.id()))
                .map(|n| format!("; did you mean `{n}`?"))
                .unwrap_or_default();
            return Err(EngineError::Usage(format!("unknown rule `{id}`{near}")));
        };
        let (abs, rel, language) = self.locate(file)?;
        if !matches!(c.matcher, Matcher::Match(_)) {
            return Err(EngineError::Usage(format!("`{id}` is not a `match:` rule; `match --explain` explains `match:` rules")));
        }
        if !c.applies_to_language(language) {
            let langs: Vec<&str> = c.loaded.rule.languages.iter().map(|l| l.name()).collect();
            return Err(EngineError::Usage(format!("`{id}` does not apply to {} ({rel}); it is for {}", language.name(), langs.join(", "))));
        }
        let grammar = Grammar::for_language(language);
        let explainer = Explainer::new(&c.loaded.rule, grammar, &c.loaded.data).map_err(|e| EngineError::Usage(format!("`{id}` on {}: {e}", language.name())))?;
        let text = std::fs::read_to_string(&abs).map_err(|e| EngineError::Usage(format!("{rel}: {e}")))?;
        let text = text.strip_prefix('\u{feff}').map(str::to_owned).unwrap_or(text);
        let root = codelaw_lang::parse(language, &text);
        let semantic = if c.needs_semantic { codelaw_semantic::build(language, &root, &text) } else { None };
        let index = LineIndex::new(&text);
        let nodes = explainer.explain(&root, semantic.as_ref()).into_iter().map(|n| (index.position(n.start), index.position(n.end), n)).collect();
        Ok(MatchReport { rule: id.to_owned(), file: rel, language, nodes })
    }
}

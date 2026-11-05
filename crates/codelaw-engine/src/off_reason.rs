//! `codelaw/off-without-reason` (SR-CONFIG decision 11c, W-158): with
//! `[rules] require_reason` on, a rule entry in the project's own config
//! that sets `severity = "off"` with no `reason` is a finding on that line
//! of the config file — held to the bar `.codelawignore` and the inline
//! comment already are.

use crate::Project;
use codelaw_core::Diagnostic;

pub(crate) const OFF_WITHOUT_REASON: &str = "codelaw/off-without-reason";

impl Project {
    /// One finding per `"off"` entry that does not say why.
    pub(crate) fn off_reason_findings(&self) -> Vec<Diagnostic> {
        self.config
            .offs_without_reason()
            .into_iter()
            .filter_map(|o| {
                let rel = o.file.strip_prefix(self.root()).map(codelaw_core::normalize_path).unwrap_or_else(|_| codelaw_core::normalize_path(&o.file));
                let message = format!(
                    "`{}` is turned off in {} with no `reason`, and `[rules] require_reason` is on: write `\"{}\" = {{ severity = \"off\", reason = \"<why>\" }}`",
                    o.rule, o.place, o.rule
                );
                self.at_line(&rel, o.line.unwrap_or(1), OFF_WITHOUT_REASON, message)
            })
            .collect()
    }
}

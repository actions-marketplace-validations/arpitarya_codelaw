//! The fix ladder, counted (SR-LAW-4 decision 7): each pack's rules per
//! rung and per exception. No count limits them (decision 6, retired
//! 2026-11-05).

use crate::{BlockKind, RuleFile};
use codelaw_core::{FixClass, FixException};
use serde::Serialize;
use std::collections::BTreeMap;

/// One pack's rules by rung (SR-LAW-4 decision 7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct FixStats {
    pub pack: String,
    pub rules: usize,
    pub auto: usize,
    pub suggest: usize,
    /// `suggest` rules whose `fix:` offers labelled choices (a native rule's
    /// choices are known only when it runs, so they count as `suggest`).
    pub choices: usize,
    pub manual: usize,
    /// `manual` rules by `fix_exception:`.
    pub exceptions: BTreeMap<String, usize>,
    /// `manual` rules still waiting to move up the ladder (`fix_pending:`).
    pub pending: usize,
}

impl FixStats {
    pub fn of<'a>(pack: &str, rules: impl IntoIterator<Item = &'a RuleFile>) -> FixStats {
        let mut s = FixStats { pack: pack.to_owned(), ..Default::default() };
        for x in FixException::ALL {
            s.exceptions.insert(x.name().to_owned(), 0);
        }
        for r in rules {
            s.rules += 1;
            match r.fix {
                FixClass::Auto => s.auto += 1,
                FixClass::Suggest => {
                    s.suggest += 1;
                    if !matches!(r.block.kind, BlockKind::Native(_)) && !r.block.choice_labels().is_empty() {
                        s.choices += 1;
                    }
                }
                FixClass::Manual => {
                    s.manual += 1;
                    match r.fix_exception {
                        Some(x) => *s.exceptions.entry(x.name().to_owned()).or_default() += 1,
                        None => s.pending += 1,
                    }
                }
            }
        }
        s
    }
}

/// The fix-stats table as text, one row per pack and a total.
pub fn fix_stats_text(stats: &[FixStats]) -> String {
    let mut out = String::from("pack                 rules  auto  suggest (choices)  manual  value  restructure  authored  security  pending\n");
    let mut total = FixStats { pack: "total".into(), ..Default::default() };
    let row = |s: &FixStats| {
        let x = |k: &str| s.exceptions.get(k).copied().unwrap_or(0);
        format!(
            "{:<20} {:>5}  {:>4}  {:>7} {:>9}  {:>6}  {:>5}  {:>11}  {:>8}  {:>8}  {:>7}\n",
            s.pack,
            s.rules,
            s.auto,
            s.suggest,
            format!("({})", s.choices),
            s.manual,
            x("value"),
            x("restructure"),
            x("authored"),
            x("security"),
            s.pending,
        )
    };
    for s in stats {
        out.push_str(&row(s));
        total.rules += s.rules;
        total.auto += s.auto;
        total.suggest += s.suggest;
        total.choices += s.choices;
        total.manual += s.manual;
        total.pending += s.pending;
        for (k, v) in &s.exceptions {
            *total.exceptions.entry(k.clone()).or_default() += v;
        }
    }
    out.push_str(&row(&total));
    out
}

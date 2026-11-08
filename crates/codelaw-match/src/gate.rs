//! Which nodes a rule need not be asked about (W-172).
//!
//! A rule anchored on `identifier` and `inside: { kind: formal_parameters }`
//! is asked about every identifier of every file, and each time ast-grep
//! walks down from the tree's root to find the identifier's parent — the
//! larger share of a check's time. The answer is known without that walk
//! when the walk that visits the nodes knows each node's parent.
//!
//! [`ParentGate::of`] reads a `match:` block for what it needs of a node's
//! *parent*: an `inside:` that stops at its neighbour (ast-grep's default)
//! holds only if the parent matches the clause. From the clauses it keeps
//! the parent kinds a node of each kind may have; a node whose parent has
//! another kind cannot match, so the rule is not asked. The gate is a
//! necessary condition and nothing more: a rule it cannot read gets an empty
//! gate and is asked about every node of its kinds as before.

use codelaw_lang::{AgLanguage, Grammar};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// For the kinds a rule is anchored on: the parent kinds that node may have.
#[derive(Clone, Debug, Default)]
pub struct ParentGate {
    by_kind: Vec<(u16, Box<[u16]>)>,
}

impl ParentGate {
    /// Whether the gate says anything: false for a rule asked about every node.
    pub fn is_empty(&self) -> bool {
        self.by_kind.is_empty()
    }

    /// Whether a node of `kind` whose parent has kind `parent` (`u16::MAX`
    /// for the root) may match.
    #[inline]
    pub fn allows(&self, kind: u16, parent: u16) -> bool {
        match self.by_kind.iter().find(|(k, _)| *k == kind) {
            None => true,
            Some((_, parents)) => parents.contains(&parent),
        }
    }

    /// The gate of a `match:` block and its utils for one grammar.
    pub fn of(matcher: &Value, utils: Option<&Value>, grammar: Grammar) -> ParentGate {
        let utils = utils.and_then(Value::as_object);
        let Some(alts) = alts(matcher, utils, 0) else { return ParentGate::default() };
        // Every alternative must name the kinds it holds for; one that holds
        // for any kind says nothing about which parents a node may have.
        let mut kinds: BTreeSet<&str> = BTreeSet::new();
        for a in &alts {
            match &a.kinds {
                Some(k) => kinds.extend(k.iter().map(String::as_str)),
                None => return ParentGate::default(),
            }
        }
        let mut by_kind = Vec::new();
        for kind in kinds {
            let id = grammar.kind_to_id(kind);
            if id == 0 {
                continue;
            }
            let mut parents: BTreeSet<u16> = BTreeSet::new();
            let mut open = false;
            for a in alts.iter().filter(|a| a.kinds.as_ref().is_some_and(|k| k.contains(kind))) {
                match &a.parents {
                    None => open = true,
                    Some(p) => parents.extend(p.iter().map(|n| grammar.kind_to_id(n)).filter(|i| *i != 0)),
                }
            }
            if !open {
                by_kind.push((id, parents.into_iter().collect()));
            }
        }
        ParentGate { by_kind }
    }
}

/// One way a node can satisfy a rule: the kinds it may have and the kinds its
/// parent may have, `None` where the rule says nothing.
#[derive(Clone)]
struct Alt {
    kinds: Option<BTreeSet<String>>,
    parents: Option<BTreeSet<String>>,
}

/// More alternatives than this and the rule is not read.
const MAX_ALTS: usize = 64;

fn meet(a: &Option<BTreeSet<String>>, b: &Option<BTreeSet<String>>) -> Option<BTreeSet<String>> {
    match (a, b) {
        (None, x) | (x, None) => x.clone(),
        (Some(a), Some(b)) => Some(a.intersection(b).cloned().collect()),
    }
}

/// The alternatives of a rule: it holds for a node when one of them does.
/// `None` when the rule is too large to read.
fn alts(v: &Value, utils: Option<&Map<String, Value>>, depth: u32) -> Option<Vec<Alt>> {
    let anything = || Alt { kinds: None, parents: None };
    let Value::Object(o) = v else { return Some(vec![anything()]) };
    if depth > 6 {
        return Some(vec![anything()]);
    }
    let mut acc = vec![anything()];
    for (key, x) in o {
        let term: Vec<Alt> = match key.as_str() {
            "kind" => match x.as_str() {
                Some(k) if k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') => vec![Alt { kinds: Some(BTreeSet::from([k.to_owned()])), parents: None }],
                _ => vec![anything()],
            },
            "any" => match x.as_array() {
                Some(a) if !a.is_empty() => {
                    let mut all = Vec::new();
                    for b in a {
                        all.extend(alts(b, utils, depth + 1)?);
                    }
                    all
                }
                _ => vec![anything()],
            },
            "all" => {
                let mut all = vec![anything()];
                for b in x.as_array().into_iter().flatten() {
                    all = product(&all, &alts(b, utils, depth + 1)?)?;
                }
                all
            }
            "matches" => match (x.as_str(), utils) {
                (Some(id), Some(u)) if u.contains_key(id) => alts(&u[id], utils, depth + 1)?,
                _ => vec![anything()],
            },
            // The parent only: ast-grep's default `stopBy` is the neighbour.
            "inside" if x.get("stopBy").is_none_or(|s| s.as_str() == Some("neighbor")) => {
                let mut parents: BTreeSet<String> = BTreeSet::new();
                let mut any = false;
                for a in alts(x, utils, depth + 1)? {
                    match a.kinds {
                        Some(k) => parents.extend(k),
                        None => any = true,
                    }
                }
                vec![Alt { kinds: None, parents: (!any).then_some(parents) }]
            }
            _ => vec![anything()],
        };
        acc = product(&acc, &term)?;
    }
    Some(acc)
}

fn product(a: &[Alt], b: &[Alt]) -> Option<Vec<Alt>> {
    if a.len() * b.len() > MAX_ALTS {
        return None;
    }
    let mut out = Vec::with_capacity(a.len() * b.len());
    for x in a {
        for y in b {
            let kinds = meet(&x.kinds, &y.kinds);
            let parents = meet(&x.parents, &y.parents);
            // An alternative no node can satisfy is dropped.
            if kinds.as_ref().is_some_and(BTreeSet::is_empty) || parents.as_ref().is_some_and(BTreeSet::is_empty) {
                continue;
            }
            out.push(Alt { kinds, parents });
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn gate(m: Value, utils: Value) -> ParentGate {
        ParentGate::of(&m, Some(&utils), Grammar::TypeScript)
    }

    fn id(k: &str) -> u16 {
        Grammar::TypeScript.kind_to_id(k)
    }

    #[test]
    fn a_neighbour_inside_names_the_parent_kinds() {
        let g = gate(json!({ "kind": "identifier", "inside": { "kind": "formal_parameters" } }), json!({}));
        assert!(g.allows(id("identifier"), id("formal_parameters")));
        assert!(!g.allows(id("identifier"), id("call_expression")));
        assert!(!g.allows(id("identifier"), u16::MAX));
        // Another kind is not the gate's to say.
        assert!(g.allows(id("string"), id("call_expression")));
    }

    #[test]
    fn branches_join_and_a_utility_is_read() {
        let utils = json!({ "executor": { "any": [{ "kind": "arrow_function" }, { "kind": "function_expression" }] } });
        let g = gate(
            json!({ "any": [
                { "kind": "identifier", "inside": { "matches": "executor", "field": "parameter" } },
                { "kind": "identifier", "nthChild": 1, "inside": { "kind": "formal_parameters", "inside": { "matches": "executor" } } },
            ] }),
            utils,
        );
        for parent in ["arrow_function", "function_expression", "formal_parameters"] {
            assert!(g.allows(id("identifier"), id(parent)), "{parent}");
        }
        assert!(!g.allows(id("identifier"), id("member_expression")));
    }

    #[test]
    fn a_branch_with_no_parent_clause_opens_its_kind() {
        let g = gate(json!({ "any": [{ "kind": "identifier", "inside": { "kind": "import_clause" } }, { "kind": "import_specifier" }] }), json!({}));
        assert!(g.allows(id("identifier"), id("import_clause")));
        assert!(!g.allows(id("identifier"), id("call_expression")));
        assert!(g.allows(id("import_specifier"), id("call_expression")));
        let open = gate(json!({ "any": [{ "kind": "identifier", "inside": { "kind": "import_clause" } }, { "kind": "identifier" }] }), json!({}));
        assert!(open.is_empty());
    }

    #[test]
    fn a_deeper_stop_is_not_read() {
        let g = gate(json!({ "kind": "identifier", "inside": { "kind": "class_body", "stopBy": "end" } }), json!({}));
        assert!(g.is_empty());
    }

    #[test]
    fn a_rule_with_no_kind_says_nothing() {
        let g = gate(json!({ "pattern": "foo($A)", "inside": { "kind": "class_body" } }), json!({}));
        assert!(g.is_empty());
    }
}

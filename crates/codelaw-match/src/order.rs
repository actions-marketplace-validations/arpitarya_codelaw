//! The order a rule object's keys are tried in (W-172).
//!
//! ast-grep tries an object's keys in a fixed order: the atomic ones
//! (`pattern`, `kind`, `regex`, `nthChild`), then the composite ones (`all`,
//! `any`, `not`, `matches`), then the relational ones (`inside`, `has`,
//! `precedes`, `follows`). Every key must hold, so the order changes the cost,
//! never the answer — but it can change the cost a great deal: `nthChild` and
//! `inside` ask for a node's parent, which tree-sitter finds by walking down
//! from the root, so a rule that tries `matches: two` (two parent walks) before
//! `has: { field: function, regex: … }` (one node's own children) pays for the
//! walk on every node the cheap key would have turned down.
//!
//! [`cheap_first`] rewrites an object whose keys would be tried in a worse
//! order into `all:` [one key each, cheapest first]; `all` keeps the order it
//! is given. An object it cannot judge is left as written.

use serde_json::{Map, Value};

/// The order ast-grep tries the keys in.
const NATURAL: [&str; 13] = ["pattern", "kind", "regex", "nthChild", "range", "all", "any", "not", "matches", "inside", "has", "precedes", "follows"];

/// Keys of a relational rule's object that are not part of the rule itself.
const WRAPPER: [&str; 2] = ["field", "stopBy"];

/// What a utility rule costs to try, by id.
type Utils<'a> = Option<&'a Map<String, Value>>;

/// Reorder every rule object in `v` (a rule, or a map of rules by name).
pub fn cheap_first(v: &mut Value, utils: Utils<'_>) {
    reorder(v, utils, false);
}

fn reorder(v: &mut Value, utils: Utils<'_>, relational: bool) {
    match v {
        Value::Array(a) => a.iter_mut().for_each(|x| reorder(x, utils, false)),
        Value::Object(o) => {
            if !is_rule(o, relational) {
                // A map of rules (`utils:`, `constraints:`), or something this
                // pass does not know: look inside for rules.
                for x in o.values_mut() {
                    reorder(x, utils, false);
                }
                return;
            }
            for (k, x) in o.iter_mut() {
                match k.as_str() {
                    "inside" | "has" | "precedes" | "follows" => reorder(x, utils, true),
                    "all" | "any" | "not" => reorder(x, utils, false),
                    _ => {}
                }
            }
            sort_keys(o, utils);
        }
        _ => {}
    }
}

/// Whether `o` is a rule object: every key is one ast-grep reads in a rule.
fn is_rule(o: &Map<String, Value>, relational: bool) -> bool {
    !o.is_empty() && o.keys().all(|k| NATURAL.contains(&k.as_str()) || (relational && WRAPPER.contains(&k.as_str())))
}

fn sort_keys(o: &mut Map<String, Value>, utils: Utils<'_>) {
    let mut keys: Vec<(usize, u32, &String)> = Vec::new();
    for k in o.keys().filter(|k| NATURAL.contains(&k.as_str())) {
        keys.push((NATURAL.iter().position(|n| n == k).unwrap_or(0), cost(k, &o[k], utils, 0), k));
    }
    if keys.len() < 2 {
        return;
    }
    // Two keys that capture a name: the first to bind it wins, so their order stays.
    if keys.iter().filter(|(_, _, k)| mentions_capture(&o[*k])).count() > 1 {
        return;
    }
    // Keys that tie keep ast-grep's own order.
    let mut by_natural = keys.clone();
    by_natural.sort_by_key(|(n, _, _)| *n);
    let current: Vec<&String> = by_natural.iter().map(|(_, _, k)| *k).collect();
    keys.sort_by_key(|(n, c, _)| (*c, *n));
    let better: Vec<&String> = keys.iter().map(|(_, _, k)| *k).collect();
    if better == current {
        return;
    }
    let order: Vec<String> = better.into_iter().cloned().collect();
    let mut rules = Vec::with_capacity(order.len());
    for k in &order {
        if let Some(x) = o.remove(k) {
            let mut one = Map::new();
            one.insert(k.clone(), x);
            rules.push(Value::Object(one));
        }
    }
    // What is left is the relational rule's own keys (`field`, `stopBy`).
    o.insert("all".into(), Value::Array(rules));
}

fn mentions_capture(v: &Value) -> bool {
    match v {
        Value::String(s) => s.contains('$'),
        Value::Array(a) => a.iter().any(mentions_capture),
        Value::Object(o) => o.values().any(mentions_capture),
        _ => false,
    }
}

/// A rough cost of trying one key on one node: a node's own kind or text is
/// 1; a walk over its children is more; anything that asks for a parent is
/// more again.
fn cost(key: &str, v: &Value, utils: Utils<'_>, depth: u32) -> u32 {
    if depth > 6 {
        return 100;
    }
    match key {
        "pattern" | "kind" | "regex" | "range" => 1,
        // The parent, then a count of its children.
        "nthChild" => 50,
        "not" => rule_cost(v, utils, depth + 1),
        "all" | "any" => v.as_array().map_or(100, |a| a.iter().map(|x| rule_cost(x, utils, depth + 1)).sum()),
        "matches" => match (v, utils) {
            (Value::String(id), Some(u)) => u.get(id).map_or(100, |x| rule_cost(x, utils, depth + 1)),
            _ => 100,
        },
        "has" => {
            let deep = v.get("stopBy").is_some_and(|s| s.as_str() != Some("neighbor"));
            2 + rule_cost(v, utils, depth + 1) * if deep { 8 } else { 1 }
        }
        // The parent, or a sibling, or something above the node; what the
        // clause asks of the node it finds is asked only when that node is
        // the right kind, so only the clause's own kind and text count.
        "inside" | "precedes" | "follows" => 20 + shallow_cost(v),
        _ => 100,
    }
}

fn shallow_cost(v: &Value) -> u32 {
    match v {
        Value::Object(o) => o.iter().filter(|(k, _)| matches!(k.as_str(), "pattern" | "kind" | "regex" | "range")).count() as u32,
        _ => 1,
    }
}

fn rule_cost(v: &Value, utils: Utils<'_>, depth: u32) -> u32 {
    match v {
        Value::Object(o) => o.iter().filter(|(k, _)| NATURAL.contains(&k.as_str())).map(|(k, x)| cost(k, x, utils, depth)).sum::<u32>().max(1),
        _ => 100,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn run(mut v: Value) -> Value {
        cheap_first(&mut v, None);
        v
    }

    #[test]
    fn a_cheap_relational_key_goes_before_a_costly_one() {
        let v = run(json!({
            "kind": "call_expression",
            "has": { "field": "function", "regex": "^Promise$" },
            "inside": { "kind": "arguments" }
        }));
        // `inside` asks for the parent; `has` and `kind` do not.
        assert_eq!(v, json!({ "all": [{ "kind": "call_expression" }, { "has": { "field": "function", "regex": "^Promise$" } }, { "inside": { "kind": "arguments" } }] }));
    }

    #[test]
    fn nth_child_is_tried_after_the_text_checks() {
        let v = run(json!({ "kind": "identifier", "nthChild": 1, "not": { "regex": "^_" } }));
        assert_eq!(v, json!({ "all": [{ "kind": "identifier" }, { "not": { "regex": "^_" } }, { "nthChild": 1 }] }));
    }

    #[test]
    fn an_object_already_in_order_is_left_alone() {
        let v = json!({ "kind": "identifier", "regex": "^a", "inside": { "kind": "arguments" } });
        assert_eq!(run(v.clone()), v);
    }

    #[test]
    fn a_wrapper_keeps_its_field_and_gets_the_order() {
        let v = run(json!({ "inside": { "field": "body", "nthChild": 2, "kind": "block", "not": { "regex": "x" } } }));
        assert_eq!(v, json!({ "inside": { "field": "body", "all": [{ "kind": "block" }, { "not": { "regex": "x" } }, { "nthChild": 2 }] } }));
    }

    #[test]
    fn two_captures_keep_their_order() {
        let v = json!({ "pattern": "$A", "has": { "pattern": "$B" }, "nthChild": 1, "kind": "identifier" });
        assert_eq!(run(v.clone()), v);
    }

    #[test]
    fn a_utility_is_costed_by_what_it_holds() {
        let utils = json!({ "deep": { "inside": { "kind": "x" } }, "flat": { "kind": "y" } });
        let mut v = json!({ "matches": "deep", "has": { "kind": "z" } });
        cheap_first(&mut v, utils.as_object());
        assert_eq!(v, json!({ "all": [{ "has": { "kind": "z" } }, { "matches": "deep" }] }));
    }

    #[test]
    fn a_map_of_rules_is_walked() {
        let v = run(json!({ "executor": { "nthChild": 1, "kind": "identifier", "not": { "regex": "x" } } }));
        assert_eq!(v, json!({ "executor": { "all": [{ "kind": "identifier" }, { "not": { "regex": "x" } }, { "nthChild": 1 }] } }));
    }
}

use super::RULES;
use crate::{is_idle, option_is_set, run_with};
use codelaw_core::Language;
use serde_json::json;
use std::collections::BTreeMap;

/// Source that gives every rule something to look at.
const BUSY: &str = r#"#!/usr/bin/env node
// FIXME: later
var data = load();
function get_user(temp) {
  for (const key in temp) {
    use(key, data);
  }
  return Object.assign({}, temp);
}
const add = (a, b) => a + b;
items.map(function (x) {
  return x;
});
import React from "react";
import _ from "lodash";
const fs = require("fs");
const text = "Loading...";
leaked = 1;
/**
 * Returns Array<string> of `names`.
 * @param {string} user_name
 */
export function find(user_name) {}
"#;

#[test]
fn every_rule_names_its_options() {
    for r in RULES {
        assert!(!r.needs_options().is_empty(), "{} names no option it needs", r.id());
    }
}

#[test]
fn a_rule_is_idle_until_an_option_has_a_value() {
    let none = BTreeMap::new();
    for r in RULES {
        assert!(is_idle(*r, &none), "{} runs with no options", r.id());
        let key = r.needs_options()[0];
        let empties = [json!(null), json!(false), json!(""), json!([]), json!({}), json!(0)];
        for e in empties {
            let mut o = BTreeMap::new();
            o.insert(key.to_owned(), e);
            assert!(is_idle(*r, &o), "{} runs with an empty `{key}`", r.id());
        }
        for v in [json!(true), json!("x"), json!(["x"]), json!({"a": 1}), json!(1)] {
            let mut o = BTreeMap::new();
            o.insert(key.to_owned(), v);
            assert!(!is_idle(*r, &o), "{} is idle with `{key}` set", r.id());
        }
    }
    assert!(option_is_set(&BTreeMap::from([("a".to_owned(), json!("x"))]), "a"));
    assert!(!option_is_set(&BTreeMap::new(), "a"));
}

#[test]
fn run_anyway_a_rule_with_no_options_finds_nothing() {
    for r in RULES {
        for language in [Language::Javascript, Language::Typescript] {
            let found = run_with(*r, language, BUSY, BTreeMap::new());
            assert!(found.is_empty(), "{} reported without an option: {:?}", r.id(), found.first().map(|f| &f.message));
        }
    }
}

#[test]
fn the_ids_are_registered() {
    for r in RULES {
        assert!(crate::lookup(r.id()).is_some(), "{} is not in the registry", r.id());
    }
}

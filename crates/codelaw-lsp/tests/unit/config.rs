//! W-157: codelaw's own config, completed and checked as it is typed
//! (SR-EDITOR decision 9). The cursor's place in a half-typed file, each
//! completion source, hovers, and the loader's errors at their line with
//! their "did you mean" quick fix.

use super::config::{At, Catalogue, Kind, PresetInfo, RuleInfo, complete, fix, home, hover, locate, place, problem};
use super::*;
use codelaw_config::Home;
use serde_json::json;
use std::fs;

fn at(text: &str) -> (String, usize) {
    let i = text.find('|').expect("a cursor");
    (text.replacen('|', "", 1), i)
}

fn place_of(text: &str, home: Home) -> Option<config::Place> {
    let (t, i) = at(text);
    place(&t, i, home)
}

fn path(p: &config::Place) -> Vec<&str> {
    p.path.iter().map(String::as_str).collect()
}

#[test]
fn the_cursor_s_place_in_toml_survives_half_typed_text() {
    let p = place_of("[rules]\n\"js/ma|", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p), p.quoted), (At::Key, vec!["rules"], true));
    let p = place_of("[rules]\n\"js/max-lines\" = |", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p)), (At::Value, vec!["rules", "js/max-lines"]));
    let p = place_of("[rules]\n\"js/max-lines\" = { severity = \"warn\", options = { m|", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p)), (At::Key, vec!["rules", "js/max-lines", "options"]));
    let p = place_of("extends = [\"codelaw:js/recommended\", \"codelaw:|\"]", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p), p.in_array), (At::Value, vec!["extends"], true));
    let p = place_of("[scope.tests]\nrules = { \"js/max-lines\" = |", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p)), (At::Value, vec!["scope", "tests", "rules", "js/max-lines"]));
    let p = place_of("[deps]\nmin_rel|", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p), p.token.1 - p.token.0), (At::Key, vec!["deps"], 7));
    let p = place_of("[sc|", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p)), (At::Header, vec![]));
    let p = place_of("[scope.|", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p)), (At::Header, vec!["scope"]));
    // A dotted key and a comment.
    let p = place_of("deps.min|", Home::CodelawToml).unwrap();
    assert_eq!((p.at, path(&p)), (At::Key, vec!["deps"]));
    assert_eq!(place_of("# a note |", Home::CodelawToml), None);
}

#[test]
fn pyproject_and_package_json_offer_only_inside_codelaw_s_own_table() {
    let p = place_of("[project]\nname = \"x\"\n\n[tool.codelaw]\nlan|", Home::Pyproject).unwrap();
    assert_eq!((p.at, path(&p)), (At::Key, vec![]));
    let p = place_of("[tool.codelaw.rules]\n\"py/x\" = |", Home::Pyproject).unwrap();
    assert_eq!((p.at, path(&p)), (At::Value, vec!["rules", "py/x"]));
    assert_eq!(place_of("[project]\nna|", Home::Pyproject), None);
    let p = place_of("{ \"name\": \"x\", \"codelaw\": { \"rules\": { \"|", Home::PackageJson).unwrap();
    assert_eq!((p.at, path(&p), p.quoted), (At::Key, vec!["rules"], true));
    let p = place_of("{ \"codelaw\": { \"extends\": [\"codelaw:js/recommended\", \"|", Home::PackageJson).unwrap();
    assert_eq!((p.at, path(&p), p.in_array), (At::Value, vec!["extends"], true));
    let p = place_of("{ \"codelaw\": { \"rules\": { \"js/a\": { \"severity\": \"|", Home::PackageJson).unwrap();
    assert_eq!((p.at, path(&p)), (At::Value, vec!["rules", "js/a", "severity"]));
    assert_eq!(place_of("{ \"name\": \"|", Home::PackageJson), None);
    assert_eq!(home(Path::new("/x/codelaw.toml")), Some(Home::CodelawToml));
    assert_eq!(home(Path::new("/x/Cargo.toml")), None);
}

fn rule(fix: FixClass) -> RuleInfo {
    let options = serde_json::from_value(json!({
        "max": { "type": "integer", "default": 300, "description": "the most lines" },
        "skip_blank_lines": { "type": "boolean", "default": true }
    }))
    .unwrap();
    RuleInfo { title: "A title".into(), fix, severity: "warn".into(), pack: "@codelaw/rules-js@1.0.0".into(), page: Some("https://example.com/r".into()), summary: "Says why.".into(), options }
}

fn cat() -> Catalogue {
    let mut c = Catalogue { extends: vec!["codelaw:js/recommended".into(), "codelaw:py/recommended".into()], ..Default::default() };
    c.rules.insert("js/max-lines".into(), rule(FixClass::Suggest));
    c.rules.insert("js/no-var".into(), rule(FixClass::Auto));
    c.rules.insert("js/todo".into(), rule(FixClass::Manual));
    c.presets.push(PresetInfo { spec: "codelaw:js/recommended".into(), pack: "@codelaw/rules-js@1.0.0".into(), description: Some("the defaults".into()), scopes: vec!["tests".into()] });
    c.missing.push(("py".into(), "install it with `pip install codelaw-rules-python`".into()));
    c.scopes.push(("tests".into(), "codelaw:js/recommended".into()));
    c
}

fn labels(text: &str, home: Home) -> Vec<(String, String, Kind)> {
    let (t, i) = at(text);
    complete(&t, i, home, &cat()).map(|(_, items)| items.into_iter().map(|i| (i.label, i.insert, i.kind)).collect()).unwrap_or_default()
}

fn has(list: &[(String, String, Kind)], label: &str) -> bool {
    list.iter().any(|(l, ..)| l == label)
}

#[test]
fn completions_come_from_the_schema_and_the_packs() {
    // Keys of the root table, from the schema.
    let root = labels("ext|", Home::CodelawToml);
    assert!(has(&root, "extends") && has(&root, "rules") && has(&root, "deps"), "{root:?}");
    // A header offers only tables.
    let headers = labels("[|", Home::CodelawToml);
    assert!(has(&headers, "deps") && has(&headers, "rules") && !has(&headers, "extends"), "{headers:?}");
    // Rule ids in [rules], quoted when typed bare; a missing pack is named.
    let rules = labels("[rules]\njs/m|", Home::CodelawToml);
    assert!(rules.iter().any(|(l, ins, k)| l == "js/max-lines" && ins == "\"js/max-lines\"" && *k == Kind::Rule), "{rules:?}");
    assert!(rules.iter().any(|(l, _, k)| l == "py/…" && *k == Kind::Note), "the uninstalled pack the config extends: {rules:?}");
    let quoted = labels("[rules]\n\"js/m|\"", Home::CodelawToml);
    assert!(quoted.iter().any(|(l, ins, _)| l == "js/max-lines" && ins == "js/max-lines"), "{quoted:?}");
    // Scope and profile rules are rule tables too.
    assert!(has(&labels("[scope.tests.rules]\n|", Home::CodelawToml), "js/no-var"));
    assert!(has(&labels("[profile.ci.rules]\n|", Home::CodelawToml), "js/no-var"));
    // A rule's value: severities; in its table: its keys and its options.
    let sev = labels("[rules]\n\"js/no-var\" = |", Home::CodelawToml);
    assert_eq!(sev.iter().map(|(l, ..)| l.as_str()).collect::<Vec<_>>(), ["error", "warn", "info", "off"]);
    assert!(sev.iter().all(|(l, ins, _)| ins == &format!("\"{l}\"")));
    let keys = labels("[rules]\n\"js/no-var\" = { |", Home::CodelawToml);
    assert!(has(&keys, "severity") && has(&keys, "options"), "{keys:?}");
    let opts = labels("[rules]\n\"js/max-lines\" = { options = { |", Home::CodelawToml);
    assert!(has(&opts, "max") && has(&opts, "skip_blank_lines"), "{opts:?}");
    let b = labels("[rules]\n\"js/max-lines\" = { options = { skip_blank_lines = |", Home::CodelawToml);
    assert!(has(&b, "true") && has(&b, "false"), "{b:?}");
    let n = labels("[rules]\n\"js/max-lines\" = { options = { max = |", Home::CodelawToml);
    assert!(has(&n, "300"), "the option's default: {n:?}");
    // Presets in `extends`, and the uninstalled packs, named as such.
    let ext = labels("extends = [\"|\"]", Home::CodelawToml);
    assert!(has(&ext, "codelaw:js/recommended") && has(&ext, "codelaw:py/…"), "{ext:?}");
    // Enums from the schema; scope names from the presets; profiles.
    assert!(has(&labels("languages = [\"|\"]", Home::CodelawToml), "typescript"));
    assert!(has(&labels("[scope.|", Home::CodelawToml), "tests"));
    assert!(has(&labels("[profile.|", Home::CodelawToml), "ci"));
    // package.json: keys come quoted.
    let j = labels("{ \"codelaw\": { |", Home::PackageJson);
    assert!(j.iter().any(|(l, ins, _)| l == "extends" && ins == "\"extends\""), "{j:?}");
}

#[test]
fn fix_offers_only_the_rule_s_own_rung_and_below() {
    // SR-LAW-4 decision 16: a project may lower a rule's fix, never raise it.
    let vals = |id: &str| -> Vec<String> { labels(&format!("[rules]\n\"{id}\" = {{ fix = |"), Home::CodelawToml).into_iter().map(|(l, ..)| l).collect() };
    assert_eq!(vals("js/no-var"), ["auto", "suggest", "manual"]);
    assert_eq!(vals("js/max-lines"), ["suggest", "manual"]);
    assert_eq!(vals("js/todo"), ["manual"]);
}

#[test]
fn hover_names_a_rule_a_preset_or_a_key() {
    let c = cat();
    let text = "extends = [\"codelaw:js/recommended\"]\n[rules]\n\"js/max-lines\" = \"warn\"\n[deps]\nmin_release_age = 7\n";
    let (rule, _) = hover(text, text.find("max-lines").unwrap(), Home::CodelawToml, &c).unwrap();
    assert!(rule.contains("A title") && rule.contains("fix `suggest`") && rule.contains("@codelaw/rules-js@1.0.0") && rule.contains("(https://example.com/r)"), "{rule}");
    let (preset, _) = hover(text, text.find("recommended").unwrap(), Home::CodelawToml, &c).unwrap();
    assert!(preset.contains("the defaults"), "{preset}");
    let (key, (s, e)) = hover(text, text.find("release").unwrap(), Home::CodelawToml, &c).unwrap();
    assert_eq!(&text[s..e], "min_release_age");
    assert!(key.contains("deps.min_release_age") && key.contains("SR-RULE-PACKS"), "{key}");
}

#[test]
fn a_loader_error_lands_on_what_it_names() {
    let text = "extends = [\"codelaw:js/recomended\"]\n\n[rules]\n\"js/max-line\" = \"warn\"\n";
    let p = problem(text, "unknown rule `js/max-line`; did you mean `js/max-lines`?", None, true);
    assert_eq!(&text[p.start..p.end], "js/max-line");
    assert_eq!(fix(text, &p), Some((p.start, p.end, "js/max-lines".into())));
    // A preset: `codelaw:` is kept once.
    let p = problem(text, "@codelaw/rules-js@1.0.0 has no preset `js/recomended`; did you mean `codelaw:js/recommended`?", None, true);
    let (s, e, near) = fix(text, &p).unwrap();
    assert_eq!((&text[s..e], near.as_str()), ("codelaw:js/recomended", "codelaw:js/recommended"));
    // A `line:column:` the TOML parser gives.
    let text = "[rules]\n\"js/a\" = \"warn\"\nlien_width = 3\n";
    assert_eq!(locate(text, "3:1: unknown key `lien_width`", None), (text.find("lien").unwrap(), text.find(" = 3").unwrap()));
    // Nothing named here: the first line.
    assert_eq!(locate(text, "the cache folder cannot be made", None), (0, 7));
}

fn repo(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("codelaw-lsp-config-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("node_modules/@codelaw")).unwrap();
    let packs = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().join("packs/js");
    copy(&packs, &root.join("node_modules/@codelaw/rules-js"));
    canonical(&root)
}

fn copy(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap().flatten() {
        if e.path().is_dir() {
            copy(&e.path(), &to.join(e.file_name()));
        } else {
            fs::copy(e.path(), to.join(e.file_name())).unwrap();
        }
    }
}

#[test]
fn the_loader_s_errors_show_as_typed_and_the_catalogue_is_the_pack_search_s() {
    let root = repo("live");
    let file = root.join("codelaw.toml");
    fs::write(&file, "extends = [\"codelaw:js/recommended\"]\n").unwrap();
    // Unsaved text: an unknown rule, with its "did you mean".
    let typed = "extends = [\"codelaw:js/recommended\"]\n\n[rules]\n\"js/max-line\" = \"warn\"\n";
    let problems = config_problems(&file, typed);
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert_eq!(&typed[problems[0].start..problems[0].end], "js/max-line");
    assert_eq!(problems[0].suggestion.as_deref(), Some("js/max-lines"));
    // An option the rule does not have.
    let typed = "extends = [\"codelaw:js/recommended\"]\n[rules]\n\"js/max-lines\" = { options = { maxx = 3 } }\n";
    let problems = config_problems(&file, typed);
    assert!(problems.iter().any(|p| &typed[p.start..p.end] == "maxx" && p.suggestion.as_deref() == Some("max")), "{problems:?}");
    // A pack that is not installed, on its `extends` entry.
    let typed = "extends = [\"codelaw:js/recommended\", \"codelaw:py/recommended\"]\n";
    let problems = config_problems(&file, typed);
    assert!(problems.iter().any(|p| &typed[p.start..p.end] == "codelaw:py/recommended" && p.message.contains("codelaw-rules-python")), "{problems:?}");
    // A key the schema does not have.
    let typed = "extends = [\"codelaw:js/recommended\"]\npreveiw = true\n";
    let problems = config_problems(&file, typed);
    assert!(problems.iter().any(|p| &typed[p.start..p.end] == "preveiw" && p.suggestion.as_deref() == Some("preview")), "{problems:?}");
    // What loads is clean; a `pyproject.toml` that does not configure codelaw is not looked at.
    assert_eq!(config_problems(&file, "extends = [\"codelaw:js/recommended\"]\n"), vec![]);
    assert_eq!(config_problems(&root.join("pyproject.toml"), "[project]\nname = \"x\"\n"), vec![]);

    // The catalogue: the installed pack's rules and presets, the rest missing.
    let config = codelaw_config::find_config(&root).unwrap();
    let search = codelaw_engine::pack_search(&config, &EngineOptions::default());
    let cat = config::catalogue(&config, &search, None);
    assert!(cat.rules.contains_key("js/max-lines") && cat.rules["js/max-lines"].pack.starts_with("@codelaw/rules-js@"));
    assert!(cat.rules.contains_key("codelaw/unused-suppression"), "the engine's own rules");
    assert!(cat.presets.iter().any(|p| p.spec == "codelaw:js/recommended"));
    assert!(cat.missing.iter().any(|(ns, _)| ns == "py") && !cat.missing.iter().any(|(ns, _)| ns == "js"));
    assert!(cat.scopes.iter().any(|(s, _)| s == "tests"), "js/recommended ships `tests`: {:?}", cat.scopes);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn the_server_publishes_completes_and_fixes_a_config_as_typed() {
    let root = repo("server");
    let file = root.join("codelaw.toml");
    fs::write(&file, "extends = [\"codelaw:js/recommended\"]\n").unwrap();
    let u = uri::from_path(&file);
    let (conn, client) = Connection::memory();
    let mut server = Server::new(&conn, vec![root.clone()]);
    let text = "extends = [\"codelaw:js/recommended\"]\n\n[rules]\n\"js/max-line\" = \"warn\"\n";
    let params = json!({ "textDocument": { "uri": u, "languageId": "toml", "version": 1, "text": text } });
    server.notification(Notification::new(DidOpenTextDocument::METHOD.into(), params)).unwrap();
    let published: Vec<serde_json::Value> = client
        .receiver
        .try_iter()
        .filter_map(|m| match m {
            Message::Notification(n) if n.method == PublishDiagnostics::METHOD => Some(n.params),
            _ => None,
        })
        .collect();
    let d = published.last().expect("diagnostics")["diagnostics"].as_array().unwrap().clone();
    let err = d.iter().find(|d| d["source"] == "codelaw config").unwrap_or_else(|| panic!("{d:?}"));
    assert_eq!(err["range"]["start"], json!({ "line": 3, "character": 1 }));
    assert!(err["message"].as_str().unwrap().contains("unknown rule `js/max-line`"));
    // Its quick fix.
    let p: CodeActionParams = serde_json::from_value(json!({
        "textDocument": { "uri": u }, "range": err["range"], "context": { "diagnostics": [] }
    }))
    .unwrap();
    let actions = serde_json::to_value(server.code_actions(p)).unwrap();
    assert_eq!(actions[0]["title"], "Change to `js/max-lines`");
    assert_eq!(actions[0]["edit"]["changes"][&u][0]["newText"], "js/max-lines");
    // Completion of a rule id, from the installed pack.
    let p: CompletionParams = serde_json::from_value(json!({ "textDocument": { "uri": u }, "position": { "line": 3, "character": 4 } })).unwrap();
    let Some(CompletionResponse::List(list)) = server.completion(p) else { panic!("a list") };
    let item = list.items.iter().find(|i| i.label == "js/max-lines").expect("js/max-lines");
    assert!(item.detail.as_deref().unwrap().contains("@codelaw/rules-js@"), "{item:?}");
    // Hover on a rule id.
    let p: HoverParams = serde_json::from_value(json!({ "textDocument": { "uri": u }, "position": { "line": 0, "character": 2 } })).unwrap();
    let h = serde_json::to_value(server.config_hover(p)).unwrap();
    assert!(h["contents"]["value"].as_str().unwrap().contains("extends"), "{h}");
    let _ = fs::remove_dir_all(&root);
}

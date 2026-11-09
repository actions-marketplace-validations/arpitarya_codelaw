//! The declarative edit primitives of a rule's `fix:` (DESIGN §3.3,
//! SR-RULE-FORMAT decision 3). Each turns a match into LSP-style text
//! edits; only the edited ranges change (L4 decision 6).

use codelaw_core::{FixChoice, Language, TextEdit};
use codelaw_lang::Root;
use codelaw_match::{Match, try_interpolate};
use codelaw_semantic::SemanticModel;
use serde_json::Value;

/// What a primitive acts on: the matched node, or one capture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Match,
    Capture(String),
}

/// Where `move-to` and `collect-into-block` put text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    EndOfFile,
    StartOfFile,
    AfterImports,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Primitive {
    Replace { target: Target, with: String },
    Remove { target: Target },
    /// With `line`, the text is a line of its own: above the line the
    /// target starts on (`insert-before`) or below the line it ends on
    /// (`insert-after`), at the indentation of the line the target starts
    /// on (W-162).
    InsertBefore { target: Target, text: String, line: bool },
    InsertAfter { target: Target, text: String, line: bool },
    StripExportKeyword { target: Target },
    /// Gathers the names every match declares into one block. `$$$NAMES`
    /// in `block` is the value names; `$$$TYPE_NAMES` in `type_block` the
    /// type-only names (interfaces and type aliases).
    CollectIntoBlock { target: Target, block: String, type_block: Option<String>, position: Place },
    MoveTo { target: Target, position: Place },
    /// Rename a symbol and every reference to it in this file. Refused for
    /// an exported symbol: importers elsewhere would break.
    RenameSymbolLocal { target: Target, to: String },
    /// `rename-symbol: { target, to, scope: binding }`: the binding the
    /// target names, its declarations and the references that resolve to
    /// it, and nothing else. Refused for an exported symbol, when `to` is
    /// already a name in the file, and when a declaration or reference is
    /// a shorthand property (`{ a }`), whose key would change (W-162).
    RenameSymbol { target: Target, to: String },
    /// A regular-expression edit inside the target's text: every match of
    /// `pattern` becomes `replace` (`$1`, `${name}` name its groups; a
    /// capture's `$NAME` is interpolated first). Consecutive `edit-text`
    /// steps on one target run in order, each on the text the one before
    /// left, and give one edit: the bytes that changed (SR-RULE-FORMAT
    /// decision 3c; W-162).
    EditText { target: Target, pattern: String, replace: String },
    /// Replace `within` (the block itself by default) and the blank space
    /// before it with the block's statements, each on its own line,
    /// re-indented to the indentation of the line `within` starts on;
    /// comments are kept. Refused when a string in the block spans lines,
    /// whose text re-indenting would change (W-162).
    UnwrapBlock { target: Target, within: Target },
    /// Move the target onto a line of its own above the line `before`
    /// starts on, at that line's indentation, or below the line `after`
    /// ends on, at the indentation of the line `after` starts on; the
    /// target and the blank space before it leave their place. A target
    /// alone on its lines takes them, and the blank lines above it, with it,
    /// and those blank lines are written between it and its new neighbour
    /// (W-162).
    Move { target: Target, to: MoveTo },
    /// Write the items of the list `target` (an argument list, a `{ … }`
    /// of import specifiers, an array) into the list `into`, after its last
    /// item; with `unique`, an item whose text `into` already holds is left
    /// out. The target stays where it is: a `remove` step takes it away
    /// (W-162).
    MergeInto { target: Target, into: Target, unique: bool },
    /// Make the file import `name` from `module` (W-162): nothing when it
    /// already does; else the name joins an import of the module the file
    /// has, or a new import goes after the last one. `name: default` with
    /// `as: X` is a default import; in Python, no `name` is `import module`.
    /// Refused when the name the import binds is already a name in the file.
    /// A `target` only gates the step: it is skipped when the match did
    /// not bind it.
    EnsureImport { target: Option<Target>, module: String, name: Option<String>, alias: Option<String> },
}

/// Where `move` puts its target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveTo {
    Before(Target),
    After(Target),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixSpec {
    Steps(Vec<Primitive>),
    /// `{ external: "<command>" }` (SR-ARCHITECTURE decision 18).
    External(String),
    /// `{ choices: [{ label, default?, fix }] }`: a `suggest` fix whose
    /// right edit is one of a few the code determines (SR-LAW-4 decision 4).
    Choices(Vec<ChoiceSpec>),
}

/// One labelled alternative of a `choices:` fix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoiceSpec {
    pub label: String,
    pub default: bool,
    pub steps: Vec<Primitive>,
}

impl FixSpec {
    /// Whether a step reads the semantic model (a rename), so the engine
    /// builds it for the rule even when its match does not.
    pub fn needs_semantic(&self) -> bool {
        let renames = |s: &[Primitive]| s.iter().any(|p| matches!(p, Primitive::RenameSymbolLocal { .. } | Primitive::RenameSymbol { .. }));
        match self {
            FixSpec::Steps(s) => renames(s),
            FixSpec::Choices(cs) => cs.iter().any(|c| renames(&c.steps)),
            FixSpec::External(_) => false,
        }
    }

    /// A fix with `collect-into-block` is computed once over every match in
    /// the file; each diagnostic then carries the whole edit set.
    pub fn is_group(&self) -> bool {
        matches!(self, FixSpec::Steps(s) if s.iter().any(|p| matches!(p, Primitive::CollectIntoBlock { .. })))
    }
}

const PRIMITIVES: &[&str] =
    &["replace", "remove", "insert-before", "insert-after", "strip-export-keyword", "collect-into-block", "move-to", "rename-symbol-local", "rename-symbol", "edit-text", "unwrap-block", "move", "merge-into", "ensure-import"];

fn target(v: &Value) -> Result<Target, String> {
    match v.as_str() {
        Some("match") | Some("$$MATCH") => Ok(Target::Match),
        Some(s) if s.starts_with('$') && s.len() > 1 => Ok(Target::Capture(s.trim_start_matches('$').to_owned())),
        _ => Err(format!("a fix target is `match` or a capture like `$DECL`, not `{v}`")),
    }
}

fn place(v: &Value) -> Result<Place, String> {
    match v.as_str() {
        Some("end-of-file") => Ok(Place::EndOfFile),
        Some("start-of-file") => Ok(Place::StartOfFile),
        Some("after-imports") => Ok(Place::AfterImports),
        _ => Err(format!("a position is `end-of-file`, `start-of-file` or `after-imports`, not `{v}`")),
    }
}

fn field<'a>(o: &'a serde_json::Map<String, Value>, key: &str, primitive: &str) -> Result<&'a Value, String> {
    o.get(key).ok_or_else(|| format!("`{primitive}` needs `{key}`"))
}

fn string(v: &Value, what: &str) -> Result<String, String> {
    v.as_str().map(str::to_owned).ok_or_else(|| format!("`{what}` is a string"))
}

fn only_keys(o: &serde_json::Map<String, Value>, keys: &[&str], primitive: &str) -> Result<(), String> {
    for k in o.keys() {
        if !keys.contains(&k.as_str()) {
            return Err(format!("`{primitive}` takes {}, not `{k}`", keys.iter().map(|k| format!("`{k}`")).collect::<Vec<_>>().join(", ")));
        }
    }
    Ok(())
}

fn primitive(item: &Value) -> Result<Primitive, String> {
    let (name, arg) = match item {
        Value::String(s) => (s.as_str(), &Value::Null),
        Value::Object(o) if o.len() == 1 => {
            let (k, v) = o.iter().next().expect("one key");
            (k.as_str(), v)
        }
        _ => return Err("each fix step is one primitive: `- remove: $X`".into()),
    };
    let target_or_match = |v: &Value| if v.is_null() { Ok(Target::Match) } else { target(v) };
    Ok(match name {
        "replace" => match arg {
            Value::String(s) => Primitive::Replace { target: Target::Match, with: s.clone() },
            Value::Object(o) => {
                only_keys(o, &["target", "with"], name)?;
                Primitive::Replace { target: target(field(o, "target", name)?)?, with: string(field(o, "with", name)?, "with")? }
            }
            _ => return Err("`replace` takes a template, or `{ target, with }`".into()),
        },
        "remove" => Primitive::Remove { target: target_or_match(arg)? },
        "strip-export-keyword" => Primitive::StripExportKeyword { target: target_or_match(arg)? },
        "insert-before" | "insert-after" => {
            let (t, text, line) = match arg {
                Value::String(s) => (Target::Match, s.clone(), false),
                Value::Object(o) => {
                    only_keys(o, &["target", "text", "line"], name)?;
                    let line = match o.get("line") {
                        None => false,
                        Some(Value::Bool(b)) => *b,
                        Some(_) => return Err(format!("`{name}`: `line` is `true` or `false`")),
                    };
                    (o.get("target").map(target).transpose()?.unwrap_or(Target::Match), string(field(o, "text", name)?, "text")?, line)
                }
                _ => return Err(format!("`{name}` takes text, or `{{ target?, text, line? }}`")),
            };
            if name == "insert-before" { Primitive::InsertBefore { target: t, text, line } } else { Primitive::InsertAfter { target: t, text, line } }
        }
        "collect-into-block" => {
            let Value::Object(o) = arg else { return Err("`collect-into-block` takes `{ block, type_block, position }`".into()) };
            only_keys(o, &["target", "block", "type_block", "position"], name)?;
            Primitive::CollectIntoBlock {
                target: o.get("target").map(target).transpose()?.unwrap_or(Target::Match),
                block: string(field(o, "block", name)?, "block")?,
                type_block: o.get("type_block").map(|v| string(v, "type_block")).transpose()?,
                position: o.get("position").map(place).transpose()?.unwrap_or(Place::EndOfFile),
            }
        }
        "move-to" => match arg {
            Value::String(_) => Primitive::MoveTo { target: Target::Match, position: place(arg)? },
            Value::Object(o) => {
                only_keys(o, &["target", "position"], name)?;
                Primitive::MoveTo { target: target(field(o, "target", name)?)?, position: place(field(o, "position", name)?)? }
            }
            _ => return Err("`move-to` takes a position, or `{ target, position }`".into()),
        },
        "rename-symbol-local" => {
            let Value::Object(o) = arg else { return Err("`rename-symbol-local` takes `{ target, to }`".into()) };
            only_keys(o, &["target", "to"], name)?;
            Primitive::RenameSymbolLocal { target: target(field(o, "target", name)?)?, to: string(field(o, "to", name)?, "to")? }
        }
        "rename-symbol" => {
            let Value::Object(o) = arg else { return Err("`rename-symbol` takes `{ target, to, scope? }`".into()) };
            only_keys(o, &["target", "to", "scope"], name)?;
            if let Some(s) = o.get("scope")
                && s.as_str() != Some("binding")
            {
                return Err("`rename-symbol`: `scope` is `binding`".into());
            }
            Primitive::RenameSymbol { target: o.get("target").map(target).transpose()?.unwrap_or(Target::Match), to: string(field(o, "to", name)?, "to")? }
        }
        "unwrap-block" => match arg {
            Value::String(_) => Primitive::UnwrapBlock { target: target(arg)?, within: target(arg)? },
            Value::Object(o) => {
                only_keys(o, &["target", "within"], name)?;
                let t = target(field(o, "target", name)?)?;
                Primitive::UnwrapBlock { within: o.get("within").map(target).transpose()?.unwrap_or_else(|| t.clone()), target: t }
            }
            _ => return Err("`unwrap-block` takes a capture, or `{ target, within? }`".into()),
        },
        "move" => {
            let Value::Object(o) = arg else { return Err("`move` takes `{ target?, before | after }`".into()) };
            only_keys(o, &["target", "before", "after"], name)?;
            let to = match (o.get("before"), o.get("after")) {
                (Some(b), None) => MoveTo::Before(target(b)?),
                (None, Some(a)) => MoveTo::After(target(a)?),
                _ => return Err("`move` takes one of `before` and `after`".into()),
            };
            Primitive::Move { target: o.get("target").map(target).transpose()?.unwrap_or(Target::Match), to }
        }
        "ensure-import" => {
            let Value::Object(o) = arg else { return Err("`ensure-import` takes `{ module, name?, as? }`".into()) };
            only_keys(o, &["target", "module", "name", "as"], name)?;
            let get = |k: &str| o.get(k).map(|v| string(v, k)).transpose();
            let (module, n, alias) = (string(field(o, "module", name)?, "module")?, get("name")?, get("as")?);
            if n.as_deref() == Some("default") && alias.is_none() {
                return Err("`ensure-import`: a `default` import needs `as`".into());
            }
            Primitive::EnsureImport { target: o.get("target").map(target).transpose()?, module, name: n, alias }
        }
        "merge-into" => {
            let Value::Object(o) = arg else { return Err("`merge-into` takes `{ target?, into, unique? }`".into()) };
            only_keys(o, &["target", "into", "unique"], name)?;
            let unique = match o.get("unique") {
                None => false,
                Some(Value::Bool(b)) => *b,
                Some(_) => return Err("`merge-into`: `unique` is `true` or `false`".into()),
            };
            Primitive::MergeInto { target: o.get("target").map(target).transpose()?.unwrap_or(Target::Match), into: target(field(o, "into", name)?)?, unique }
        }
        "edit-text" => {
            let Value::Object(o) = arg else { return Err("`edit-text` takes `{ target?, pattern, replace }`".into()) };
            only_keys(o, &["target", "pattern", "replace"], name)?;
            let pattern = string(field(o, "pattern", name)?, "pattern")?;
            regex::Regex::new(&pattern).map_err(|e| format!("`edit-text`: `pattern` is not a regular expression: {e}"))?;
            Primitive::EditText {
                target: o.get("target").map(target).transpose()?.unwrap_or(Target::Match),
                pattern,
                replace: string(field(o, "replace", name)?, "replace")?,
            }
        }
        other => {
            // The first four characters, not bytes: a name is whatever a
            // (possibly third-party) rule file wrote, `fixé` included.
            let prefix: String = other.chars().take(4).collect();
            let near = PRIMITIVES
                .iter()
                .find(|p| p.starts_with(prefix.as_str()))
                .map(|p| format!("; did you mean `{p}`?"))
                .unwrap_or_default();
            return Err(format!("unknown fix primitive `{other}`{near} (one of {})", PRIMITIVES.join(", ")));
        }
    })
}

/// The steps of one fix value: a template, one primitive or a list.
fn steps_of(v: &Value) -> Result<Vec<Primitive>, String> {
    match parse_fix(v)? {
        FixSpec::Steps(s) => Ok(s),
        _ => Err("a choice's `fix:` is a template or a list of primitives".into()),
    }
}

/// Parse `{ choices: [...] }` (SR-LAW-4 decision 4).
fn parse_choices(v: &Value) -> Result<FixSpec, String> {
    let Some(list) = v.as_array().filter(|l| l.len() >= 2) else {
        return Err("`choices:` is a list of two or more `{ label, fix }`; one edit is a plain `fix:`".into());
    };
    let mut out = Vec::new();
    for c in list {
        let Value::Object(o) = c else { return Err("each choice is `{ label, default?, fix }`".into()) };
        only_keys(o, &["label", "default", "fix"], "choices")?;
        let label = string(field(o, "label", "choices")?, "label")?;
        if label.trim().is_empty() || label.contains(')') {
            return Err(format!("a choice's label is a few words, without `)`, not `{label}`"));
        }
        let default = match o.get("default") {
            None => false,
            Some(Value::Bool(b)) => *b,
            Some(_) => return Err("a choice's `default` is `true` or `false`".into()),
        };
        let steps = steps_of(field(o, "fix", "choices")?)?;
        if steps.iter().any(|p| matches!(p, Primitive::CollectIntoBlock { .. })) {
            return Err("a choice can't `collect-into-block`".into());
        }
        out.push(ChoiceSpec { label, default, steps });
    }
    if out.iter().filter(|c| c.default).count() > 1 {
        return Err("at most one choice is `default: true`".into());
    }
    Ok(FixSpec::Choices(out))
}

/// Parse a rule's `fix:` value.
pub fn parse_fix(v: &Value) -> Result<FixSpec, String> {
    match v {
        Value::Object(o) if o.contains_key("choices") => {
            only_keys(o, &["choices"], "choices")?;
            parse_choices(&o["choices"])
        }
        Value::String(s) => Ok(FixSpec::Steps(vec![Primitive::Replace { target: Target::Match, with: s.clone() }])),
        Value::Object(o) if o.contains_key("external") => {
            only_keys(o, &["external"], "external")?;
            Ok(FixSpec::External(string(&o["external"], "external")?))
        }
        Value::Array(items) => {
            let steps = items.iter().map(primitive).collect::<Result<Vec<_>, _>>()?;
            let groups = steps.iter().filter(|p| matches!(p, Primitive::CollectIntoBlock { .. })).count();
            if groups > 1 {
                return Err("a fix has at most one `collect-into-block`".into());
            }
            Ok(FixSpec::Steps(steps))
        }
        Value::Object(_) => Ok(FixSpec::Steps(vec![primitive(v)?])),
        _ => Err("`fix:` is a template, a list of primitives, or `{ external: \"<command>\" }`".into()),
    }
}

/// What a primitive can see of the file.
pub struct FixContext<'a> {
    pub source: &'a str,
    pub root: &'a Root,
    pub language: Language,
    pub semantic: Option<&'a SemanticModel>,
}

fn range(t: &Target, m: &Match) -> Result<(usize, usize), String> {
    match t {
        Target::Match => Ok((m.start, m.end)),
        Target::Capture(name) => m
            .single
            .get(name)
            .map(|c| (c.start, c.end))
            .or_else(|| m.multi_span(name))
            .ok_or_else(|| format!("the fix names `${name}`, which this match did not capture")),
    }
}

/// The outermost node spanning exactly `start..end`.
fn node_at(root: &Root, start: usize, end: usize) -> Option<tree_sitter::Node<'_>> {
    let mut n = root.root().get_inner_node().descendant_for_byte_range(start, end)?;
    while let Some(p) = n.parent() {
        if p.start_byte() == start && p.end_byte() == end {
            n = p;
        } else {
            break;
        }
    }
    (n.start_byte() == start && n.end_byte() == end).then_some(n)
}

/// The innermost node spanning exactly `start..end`: the list itself, not
/// a clause that holds only it (`import_clause` around `named_imports`).
fn list_at(root: &Root, start: usize, end: usize) -> Option<tree_sitter::Node<'_>> {
    let mut n = node_at(root, start, end)?;
    loop {
        let mut c = n.walk();
        let kids: Vec<_> = n.named_children(&mut c).collect();
        match kids.as_slice() {
            [only] if only.start_byte() == start && only.end_byte() == end => n = *only,
            _ => return Some(n),
        }
    }
}

/// Widen a removal so the file stays tidy: a separating comma goes with
/// the item, and a node alone on its lines takes the lines with it.
fn removal(src: &str, node: Option<tree_sitter::Node<'_>>, mut start: usize, mut end: usize) -> (usize, usize) {
    if let Some(n) = node {
        if let Some(next) = n.next_sibling().filter(|s| s.kind() == ",") {
            end = next.end_byte();
            end += src[end..].bytes().take_while(|b| *b == b' ' || *b == b'\t').count();
        } else if let Some(prev) = n.prev_sibling().filter(|s| s.kind() == ",") {
            start = prev.start_byte();
        }
    }
    let line_start = src[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    if src[line_start..start].trim().is_empty() {
        let rest = &src[end..];
        let line_end = rest.find('\n').map(|i| i + 1).unwrap_or(rest.len());
        if rest[..line_end].trim().is_empty() {
            return (line_start, end + line_end);
        }
    }
    (start, end)
}

fn place_offset(ctx: &FixContext<'_>, place: Place) -> usize {
    match place {
        Place::StartOfFile => 0,
        Place::EndOfFile => ctx.source.len(),
        Place::AfterImports => {
            let root = ctx.root.root().get_inner_node();
            let mut c = root.walk();
            let last = root
                .named_children(&mut c)
                .filter(|n| matches!(n.kind(), "import_statement" | "import_from_statement" | "future_import_statement"))
                .last();
            match last {
                Some(n) => {
                    let end = n.end_byte();
                    ctx.source[end..].find('\n').map(|i| end + i + 1).unwrap_or(ctx.source.len())
                }
                None => 0,
            }
        }
    }
}

/// Text inserted at a place, on its own line.
fn line_at(ctx: &FixContext<'_>, offset: usize, text: &str) -> String {
    let needs_newline_before = offset > 0 && !ctx.source[..offset].ends_with('\n');
    format!("{}{}\n", if needs_newline_before { "\n" } else { "" }, text.trim_end_matches('\n'))
}

/// The export statement for a target that is it, or is its declaration.
fn export_statement<'t>(node: tree_sitter::Node<'t>) -> Option<tree_sitter::Node<'t>> {
    if node.kind() == "export_statement" { Some(node) } else { node.parent().filter(|p| p.kind() == "export_statement") }
}

fn declaration_of<'t>(node: tree_sitter::Node<'t>) -> tree_sitter::Node<'t> {
    export_statement(node).and_then(|s| s.child_by_field_name("declaration")).unwrap_or(node)
}

/// Whether a step acts on a capture the matched branch did not bind: a rule
/// whose `any:` binds `$A` in one branch and `$B` in another gives each
/// branch its own steps (SR-RULE-FORMAT decision 3c), so such a step is
/// skipped, as a `calls:` entry is.
fn unbound(p: &Primitive, m: &Match) -> bool {
    let t = match p {
        Primitive::Replace { target, .. }
        | Primitive::Remove { target }
        | Primitive::InsertBefore { target, .. }
        | Primitive::InsertAfter { target, .. }
        | Primitive::EditText { target, .. }
        | Primitive::RenameSymbol { target, .. }
        | Primitive::EnsureImport { target: Some(target), .. }
        | Primitive::UnwrapBlock { target, .. } => target,
        Primitive::Move { target, to: MoveTo::Before(other) | MoveTo::After(other) } => {
            let gone = |t: &Target| matches!(t, Target::Capture(name) if !m.single.contains_key(name) && m.multi_span(name).is_none());
            return gone(target) || gone(other);
        }
        Primitive::MergeInto { target, into, .. } => {
            let gone = |t: &Target| matches!(t, Target::Capture(name) if !m.single.contains_key(name) && m.multi_span(name).is_none());
            return gone(target) || gone(into);
        }
        _ => return false,
    };
    matches!(t, Target::Capture(name) if !m.single.contains_key(name) && m.multi_span(name).is_none())
}

fn step_edits(p: &Primitive, m: &Match, ctx: &FixContext<'_>, out: &mut Vec<TextEdit>) -> Result<(), String> {
    if unbound(p, m) {
        return Ok(());
    }
    match p {
        Primitive::Replace { target, with } => {
            let (s, e) = range(target, m)?;
            out.push(TextEdit::new(s, e, tpl(with, m, ctx)?));
        }
        Primitive::Remove { target } => {
            let (s, e) = range(target, m)?;
            let (s, e) = removal(ctx.source, node_at(ctx.root, s, e), s, e);
            out.push(TextEdit::delete(s, e));
        }
        Primitive::InsertBefore { target, text, line } => {
            let (s, _) = range(target, m)?;
            let text = tpl(text, m, ctx)?;
            if *line {
                let src = ctx.source;
                let at = src[..s].rfind('\n').map(|i| i + 1).unwrap_or(0);
                out.push(TextEdit::insert(at, own_lines(&text, &line_indent(src, s))));
            } else {
                out.push(TextEdit::insert(s, text));
            }
        }
        Primitive::InsertAfter { target, text, line } => {
            let (s, e) = range(target, m)?;
            let text = tpl(text, m, ctx)?;
            if *line {
                let src = ctx.source;
                let (at, lead) = match src[e..].find('\n') {
                    Some(i) => (e + i + 1, ""),
                    None => (src.len(), "\n"),
                };
                out.push(TextEdit::insert(at, format!("{lead}{}", own_lines(&text, &line_indent(src, s)))));
            } else {
                out.push(TextEdit::insert(e, text));
            }
        }
        Primitive::StripExportKeyword { target } => {
            let (s, e) = range(target, m)?;
            let node = node_at(ctx.root, s, e).ok_or("`strip-export-keyword`: no node at the target")?;
            let stmt = export_statement(node).ok_or("`strip-export-keyword` needs an export statement or its declaration")?;
            let decl = stmt.child_by_field_name("declaration").ok_or("`strip-export-keyword`: the export has no declaration")?;
            let mut c = stmt.walk();
            let keyword = stmt.children(&mut c).find(|k| k.kind() == "export").ok_or("no `export` keyword")?;
            out.push(TextEdit::delete(keyword.start_byte(), decl.start_byte()));
        }
        Primitive::MoveTo { target, position } => {
            let (s, e) = range(target, m)?;
            let text = ctx.source[s..e].to_owned();
            let (rs, re) = removal(ctx.source, node_at(ctx.root, s, e), s, e);
            let at = place_offset(ctx, *position);
            if at > rs && at < re {
                return Err("`move-to`: the target already contains the destination".into());
            }
            out.push(TextEdit::delete(rs, re));
            out.push(TextEdit::insert(at, line_at(ctx, at, &text)));
        }
        Primitive::RenameSymbolLocal { target, to } => {
            let (s, _) = range(target, m)?;
            let model = ctx.semantic.ok_or("`rename-symbol-local` needs a language with a semantic model")?;
            let id = model.symbol_at(s).ok_or("`rename-symbol-local`: the target names no symbol")?;
            let symbol = model.symbol(id);
            if symbol.exported {
                return Err(format!("`{}` is exported; renaming it would break importers", symbol.name));
            }
            let new = tpl(to, m, ctx)?;
            let mut ranges: Vec<(usize, usize)> = symbol.decls.clone();
            ranges.extend(symbol.refs.iter().map(|&r| (model.references[r].start, model.references[r].end)));
            ranges.sort();
            ranges.dedup();
            out.extend(ranges.into_iter().map(|(s, e)| TextEdit::new(s, e, new.clone())));
        }
        Primitive::RenameSymbol { target, to } => {
            let (s, _) = range(target, m)?;
            let model = ctx.semantic.ok_or("`rename-symbol` needs a language with a semantic model")?;
            let id = model.symbol_at(s).ok_or("`rename-symbol`: the target names no symbol")?;
            let symbol = model.symbol(id);
            if symbol.exported {
                return Err(format!("`{}` is exported; renaming it would break importers", symbol.name));
            }
            let new = tpl(to, m, ctx)?;
            if new != symbol.name && names_in(ctx).contains(&new) {
                return Err(format!("`rename-symbol`: `{new}` is already a name in the file"));
            }
            let mut ranges: Vec<(usize, usize)> = symbol.decls.clone();
            ranges.extend(symbol.refs.iter().map(|&r| (model.references[r].start, model.references[r].end)));
            ranges.sort();
            ranges.dedup();
            for &(s, e) in &ranges {
                let kind = ctx.root.root().get_inner_node().descendant_for_byte_range(s, e).map(|n| n.kind()).unwrap_or("");
                if kind.starts_with("shorthand_property") {
                    return Err(format!("`rename-symbol`: `{}` is a shorthand property, whose key would change", symbol.name));
                }
            }
            out.extend(ranges.into_iter().map(|(s, e)| TextEdit::new(s, e, new.clone())));
        }
        Primitive::EnsureImport { module, name, alias, .. } => ensure_import(ctx, module, name.as_deref(), alias.as_deref(), out)?,
        Primitive::CollectIntoBlock { .. } => {}
        Primitive::EditText { .. } => unreachable!("edit-text steps run in `text_edits`"),
        Primitive::UnwrapBlock { target, within } => {
            let (bs, be) = range(target, m)?;
            let (ws, we) = range(within, m)?;
            let block = node_at(ctx.root, bs, be).ok_or("`unwrap-block`: no node at the target")?;
            if spans_lines_in_string(block, ctx.source) {
                return Err("`unwrap-block`: a string in the block spans lines, and re-indenting would change it".into());
            }
            let src = ctx.source;
            let (open, close) = (src[bs..be].find('{'), src[bs..be].rfind('}'));
            let (inner_s, inner_e) = match (open, close) {
                // A Python block has no braces: its lines are its body.
                // (from the line after `within`'s first, comments there kept).
                _ if block.kind() == "block" => (src[ws..].find('\n').map(|i| ws + i + 1).filter(|h| *h <= bs).unwrap_or(bs), be),
                (Some(o), Some(c)) if o < c => (bs + o + 1, bs + c),
                _ => return Err("`unwrap-block`: the target is not a braced block".into()),
            };
            let indent = line_indent(src, ws);
            let body = reindent(&src[inner_s..inner_e], &indent);
            // The blank space before `within` goes with it.
            let start = ws - src[..ws].bytes().rev().take_while(|b| *b == b' ' || *b == b'\t').count();
            let text = match () {
                _ if body.is_empty() => String::new(),
                // `within` starts its line: the body takes its place there.
                _ if start == 0 || src[..start].ends_with('\n') => body.trim_start().to_owned(),
                _ => format!("\n{body}"),
            };
            let start = if text.starts_with('\n') || text.is_empty() { start } else { ws };
            out.push(TextEdit::new(start, we, text));
        }
        Primitive::MergeInto { target, into, unique } => {
            let (ts, te) = range(target, m)?;
            let (is, ie) = range(into, m)?;
            if ts < ie && is < te {
                return Err("`merge-into`: the target and `into` overlap".into());
            }
            let from = list_at(ctx.root, ts, te).ok_or("`merge-into`: no node at the target")?;
            let to = list_at(ctx.root, is, ie).ok_or("`merge-into`: no node at `into`")?;
            let items = |n: tree_sitter::Node<'_>| -> Vec<(usize, usize)> {
                let mut c = n.walk();
                n.named_children(&mut c).filter(|k| k.kind() != "comment").map(|k| (k.start_byte(), k.end_byte())).collect()
            };
            let src = ctx.source;
            let held: Vec<&str> = items(to).iter().map(|&(s, e)| &src[s..e]).collect();
            let mut new: Vec<&str> = Vec::new();
            for (s, e) in items(from) {
                let text = &src[s..e];
                if *unique && (held.contains(&text) || new.contains(&text)) {
                    continue;
                }
                new.push(text);
            }
            if new.is_empty() {
                return Ok(());
            }
            match items(to).last() {
                Some(&(_, last)) => out.push(TextEdit::insert(last, format!(", {}", new.join(", ")))),
                None => {
                    let open = to.child(0).filter(|c| !c.is_named()).ok_or("`merge-into`: `into` is not a delimited list")?;
                    out.push(TextEdit::insert(open.end_byte(), new.join(", ")));
                }
            }
        }
        Primitive::Move { target, to } => {
            let (s, mut e) = range(target, m)?;
            let src = ctx.source;
            // A `;` that ends the target, when the grammar keeps it apart
            // (a TypeScript member signature), goes with it.
            if let Some(semi) = node_at(ctx.root, s, e).and_then(|n| n.next_sibling()).filter(|n| n.kind() == ";" && n.start_byte() == e) {
                e = semi.end_byte();
            }
            let text = src[s..e].to_owned();
            // What leaves: the target and the blank space before it, or,
            // for a target alone on its lines, those lines and the blank
            // lines above them.
            let line_start = src[..s].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let rest_end = src[e..].find('\n').map(|i| e + i + 1).unwrap_or(src.len());
            let alone = src[line_start..s].trim().is_empty() && src[e..rest_end].trim().is_empty();
            let (gone_s, gone_e) = if alone {
                let mut from = line_start;
                while from > 0 {
                    let prev = src[..from - 1].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    if !src[prev..from - 1].trim().is_empty() {
                        break;
                    }
                    from = prev;
                }
                (from, rest_end)
            } else {
                (s - src[..s].bytes().rev().take_while(|b| *b == b' ' || *b == b'\t').count(), e)
            };
            let (at, indent) = match to {
                MoveTo::Before(b) => {
                    let (bs, _) = range(b, m)?;
                    (src[..bs].rfind('\n').map(|i| i + 1).unwrap_or(0), line_indent(src, bs))
                }
                MoveTo::After(a) => {
                    let (as_, ae) = range(a, m)?;
                    (src[ae..].find('\n').map(|i| ae + i + 1).unwrap_or(src.len()), line_indent(src, as_))
                }
            };
            if at > gone_s && at < gone_e {
                return Err("`move`: the target holds the place it moves to".into());
            }
            if matches!(to, MoveTo::Before(_)) && !alone && at >= s {
                return Err("`move`: the target is not after the line it moves above".into());
            }
            let lead = if at == src.len() && !src.ends_with('\n') { "\n" } else { "" };
            // The blank lines that left with a target alone on its lines
            // keep it apart from its new neighbour: below it when it moves
            // up, above it when it moves down.
            let blanks = if alone { "\n".repeat(src[gone_s..line_start].matches('\n').count()) } else { String::new() };
            let moved = match to {
                MoveTo::Before(_) => format!("{lead}{indent}{text}\n{blanks}"),
                MoveTo::After(_) => format!("{lead}{blanks}{indent}{text}\n"),
            };
            out.push(TextEdit::insert(at, moved));
            out.push(TextEdit::delete(gone_s, gone_e));
        }
    }
    Ok(())
}

/// Every identifier-like word the file spells, with the language's globals
/// and keywords: what `$FRESH` and `rename-symbol` must not collide with.
fn names_in(ctx: &FixContext<'_>) -> std::collections::BTreeSet<String> {
    let mut names = std::collections::BTreeSet::new();
    let root = ctx.root.root().get_inner_node();
    let mut c = root.walk();
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.child_count() == 0 {
            let t = &ctx.source[n.byte_range()];
            if t.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$') {
                names.insert(t.to_owned());
            }
        }
        stack.extend(n.children(&mut c));
    }
    let globals: &[&str] = if ctx.language == Language::Python { codelaw_semantic::PYTHON_BUILTINS } else { codelaw_semantic::ES_GLOBALS };
    names.extend(globals.iter().map(|g| (*g).to_owned()));
    names.extend(["arguments", "await", "yield", "let", "static", "enum"].map(str::to_owned));
    if ctx.language == Language::Python {
        names.extend(["self", "cls"].map(str::to_owned));
    }
    names
}

/// A fix template with each `$FRESH(<base>)` replaced by the first of
/// `base`, `base2`, … that is no name in the file (`base` is a template
/// itself), then interpolated (W-162).
fn tpl(template: &str, m: &Match, ctx: &FixContext<'_>) -> Result<String, String> {
    let mut text = template.to_owned();
    let mut names: Option<std::collections::BTreeSet<String>> = None;
    while let Some(at) = text.find("$FRESH(") {
        let close = text[at..].find(')').map(|i| at + i).ok_or("`$FRESH(` has no `)`")?;
        let base = try_interpolate(&text[at + 7..close], m, ctx.source)?;
        if base.is_empty() || !base.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$') {
            return Err(format!("`$FRESH({base})`: the base is not a name"));
        }
        let names = names.get_or_insert_with(|| names_in(ctx));
        let fresh = std::iter::once(base.clone()).chain((2..).map(|i| format!("{base}{i}"))).find(|n| !names.contains(n)).expect("a free name");
        text.replace_range(at..=close, &fresh);
    }
    try_interpolate(&text, m, ctx.source)
}

/// An `edit-text` pattern with each `$NAME` of a capture the match bound
/// replaced by that capture's text, escaped: the pattern can name what the
/// rule captured (W-162).
fn escaped_captures(pattern: &str, m: &Match, src: &str) -> String {
    let mut out = String::new();
    let mut rest = pattern;
    while let Some(i) = rest.find('$') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        let len = after.bytes().take_while(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || *b == b'_').count();
        match m.single.get(&after[..len]).filter(|_| len > 0 && after.as_bytes()[0].is_ascii_uppercase()) {
            Some(c) => {
                out.push_str(&regex::escape(&src[c.start..c.end]));
                rest = &after[len..];
            }
            None => {
                out.push('$');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn kids(n: tree_sitter::Node<'_>) -> Vec<tree_sitter::Node<'_>> {
    let mut c = n.walk();
    n.named_children(&mut c).collect()
}

/// The edit for `ensure-import` (W-162; SR-RULE-FORMAT decision 3c).
fn ensure_import(ctx: &FixContext<'_>, module: &str, name: Option<&str>, alias: Option<&str>, out: &mut Vec<TextEdit>) -> Result<(), String> {
    let src = ctx.source;
    let root = ctx.root.root().get_inner_node();
    let mut c = root.walk();
    let top: Vec<tree_sitter::Node<'_>> = root.named_children(&mut c).collect();
    let txt = |n: tree_sitter::Node<'_>| &src[n.byte_range()];
    let binds = alias.or(name).map(str::to_owned).unwrap_or_else(|| module.split('.').next().unwrap_or(module).to_owned());
    let taken = || Err(format!("`ensure-import`: `{binds}` is already a name in the file"));
    let python = ctx.language == Language::Python;
    let mut last_import = None;
    for &s in &top {
        let is_import = matches!(s.kind(), "import_statement" | "import_from_statement" | "future_import_statement");
        if is_import {
            last_import = Some(s);
        }
        if python {
            match (s.kind(), name) {
                ("import_statement", None) if kids(s).iter().any(|k| k.kind() == "dotted_name" && txt(*k) == module) => return Ok(()),
                ("import_from_statement", Some(n)) if s.child_by_field_name("module_name").is_some_and(|m| txt(m) == module) => {
                    let names: Vec<_> = kids(s).into_iter().skip(1).collect();
                    if names.iter().any(|k| k.kind() == "wildcard_import" || (k.kind() == "dotted_name" && txt(*k) == n)) {
                        return Ok(());
                    }
                    if names_in(ctx).contains(n) {
                        return taken();
                    }
                    let last = names.last().ok_or("`ensure-import`: an import with no names")?;
                    out.push(TextEdit::insert(last.end_byte(), format!(", {n}")));
                    return Ok(());
                }
                _ => {}
            }
        } else if s.kind() == "import_statement"
            && s.child_by_field_name("source").is_some_and(|m| txt(m).trim_matches(['"', '\'']) == module)
            && !txt(s).starts_with("import type")
        {
            let clause = kids(s).into_iter().find(|k| k.kind() == "import_clause");
            let parts = clause.map(kids).unwrap_or_default();
            match name {
                Some("default") => {
                    if let Some(d) = parts.iter().find(|k| k.kind() == "identifier") {
                        return if txt(*d) == binds { Ok(()) } else { taken() };
                    }
                }
                Some(n) => {
                    if let Some(named) = parts.iter().find(|k| k.kind() == "named_imports") {
                        let specs = kids(*named);
                        if specs.iter().any(|k| k.child_by_field_name("name").is_some_and(|x| txt(x) == n) && k.child_by_field_name("alias").is_none()) {
                            return Ok(());
                        }
                        if names_in(ctx).contains(n) {
                            return taken();
                        }
                        match specs.last() {
                            Some(l) => out.push(TextEdit::insert(l.end_byte(), format!(", {n}"))),
                            None => out.push(TextEdit::insert(named.start_byte() + 1, format!(" {n} "))),
                        }
                        return Ok(());
                    }
                }
                None => return Err("`ensure-import` needs `name` in JavaScript".into()),
            }
        }
    }
    if names_in(ctx).contains(&binds) {
        return taken();
    }
    let line = match (python, name) {
        (true, None) => format!("import {module}"),
        (true, Some(n)) => format!("from {module} import {n}"),
        (false, Some("default")) => format!("import {binds} from \"{module}\";"),
        (false, Some(n)) => format!("import {{ {n} }} from \"{module}\";"),
        (false, None) => unreachable!("checked above"),
    };
    // After the last import; else after a docstring or the directives.
    let after = last_import.or_else(|| {
        top.iter()
            .take_while(|s| s.kind() == "expression_statement" && s.named_child(0).is_some_and(|e| e.kind() == "string"))
            .last()
            .copied()
    });
    let at = match after {
        Some(n) => src[n.end_byte()..].find('\n').map(|i| n.end_byte() + i + 1).unwrap_or(src.len()),
        None => 0,
    };
    let lead = if at == src.len() && !src.is_empty() && !src.ends_with('\n') { "\n" } else { "" };
    let gap = if after.is_none() && !src.is_empty() { "\n" } else { "" };
    out.push(TextEdit::insert(at, format!("{lead}{line}\n{gap}")));
    Ok(())
}

/// Text written as lines of its own, each at `indent` plus the indentation
/// it has in the text, each ending in a newline.
fn own_lines(text: &str, indent: &str) -> String {
    let text = text.trim_end_matches('\n');
    let mut out = String::new();
    for (i, l) in text.split('\n').enumerate() {
        if i == 0 || !l.trim().is_empty() {
            out.push_str(indent);
        }
        out.push_str(l);
        out.push('\n');
    }
    out
}

/// The whitespace a line starts with, for the line `offset` is on.
fn line_indent(src: &str, offset: usize) -> String {
    let start = src[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
    src[start..].chars().take_while(|c| *c == ' ' || *c == '\t').collect()
}

/// A block's inner text as lines at `indent`: blank lines at either end
/// dropped, the common indentation of the rest replaced by `indent`.
fn reindent(inner: &str, indent: &str) -> String {
    let lines: Vec<&str> = inner.split('\n').collect();
    let first = lines.iter().position(|l| !l.trim().is_empty());
    let last = lines.iter().rposition(|l| !l.trim().is_empty());
    let (Some(first), Some(last)) = (first, last) else { return String::new() };
    let lines = &lines[first..=last];
    let common = lines.iter().filter(|l| !l.trim().is_empty()).map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
    lines
        .iter()
        .map(|l| if l.trim().is_empty() { String::new() } else { format!("{indent}{}", l[common.min(l.len() - l.trim_start().len())..].trim_end()) })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether a string or template in the node spans lines.
fn spans_lines_in_string(node: tree_sitter::Node<'_>, src: &str) -> bool {
    let mut c = node.walk();
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        if (n.kind().contains("string") || n.kind() == "template_literal") && src[n.byte_range()].contains('\n') {
            return true;
        }
        stack.extend(n.children(&mut c));
    }
    false
}

/// The text of every target the `edit-text` steps act on, after them, in
/// order: one minimal edit per target (the common prefix and suffix left
/// out). A target whose text no step changed gives no edit.
fn text_edits(steps: &[&Primitive], m: &Match, ctx: &FixContext<'_>, out: &mut Vec<TextEdit>) -> Result<(), String> {
    let mut texts: Vec<((usize, usize), String)> = Vec::new();
    for p in steps {
        let Primitive::EditText { target, pattern, replace } = p else { continue };
        if unbound(p, m) {
            continue;
        }
        let range = range(target, m)?;
        let at = match texts.iter().position(|(r, _)| *r == range) {
            Some(i) => i,
            None => {
                texts.push((range, ctx.source[range.0..range.1].to_owned()));
                texts.len() - 1
            }
        };
        let re = regex::Regex::new(&escaped_captures(pattern, m, ctx.source)).map_err(|e| format!("`edit-text`: {e}"))?;
        let with = tpl(replace, m, ctx)?;
        texts[at].1 = re.replace_all(&texts[at].1, with.as_str()).into_owned();
    }
    for ((s, e), new) in texts {
        let old = &ctx.source[s..e];
        if *old == new {
            continue;
        }
        let pre = old.bytes().zip(new.bytes()).take_while(|(a, b)| a == b).count();
        let pre = (0..=pre).rev().find(|i| old.is_char_boundary(*i) && new.is_char_boundary(*i)).unwrap_or(0);
        let max_suf = (old.len() - pre).min(new.len() - pre);
        let suf = old.bytes().rev().zip(new.bytes().rev()).take(max_suf).take_while(|(a, b)| a == b).count();
        let suf = (0..=suf).rev().find(|i| old.is_char_boundary(old.len() - i) && new.is_char_boundary(new.len() - i)).unwrap_or(0);
        out.push(TextEdit::new(s + pre, e - suf, new[pre..new.len() - suf].to_owned()));
    }
    Ok(())
}

/// Every step of a fix: the `edit-text` steps together, then the others.
fn all_step_edits(steps: &[Primitive], m: &Match, ctx: &FixContext<'_>, out: &mut Vec<TextEdit>) -> Result<(), String> {
    let (text, rest): (Vec<&Primitive>, Vec<&Primitive>) = steps.iter().partition(|p| matches!(p, Primitive::EditText { .. }));
    for p in rest {
        step_edits(p, m, ctx, out)?;
    }
    text_edits(&text, m, ctx, out)
}

/// Names a declaration introduces, split into values and type-only names.
fn declared(node: tree_sitter::Node<'_>, src: &str) -> (Vec<String>, Vec<String>) {
    let decl = declaration_of(node);
    let names: Vec<String> = codelaw_semantic::declared_names(decl).into_iter().map(|n| src[n.byte_range()].to_owned()).collect();
    if matches!(decl.kind(), "interface_declaration" | "type_alias_declaration") { (Vec::new(), names) } else { (names, Vec::new()) }
}

fn sort_edits(mut edits: Vec<TextEdit>) -> Vec<TextEdit> {
    edits.sort_by_key(|e| (e.start, e.end));
    edits.dedup();
    edits
}

/// The edits for one match.
pub fn edits_for(spec: &FixSpec, m: &Match, ctx: &FixContext<'_>) -> Result<Vec<TextEdit>, String> {
    let FixSpec::Steps(steps) = spec else { return Err("an external fixer runs through `run_external`".into()) };
    let mut out = Vec::new();
    all_step_edits(steps, m, ctx, &mut out)?;
    if out.is_empty() && steps.iter().any(|p| unbound(p, m)) {
        return Err("no step of the fix acts on what this match captured".into());
    }
    if out.is_empty() && steps.iter().any(|p| matches!(p, Primitive::EditText { .. })) {
        return Err("`edit-text`: no pattern of the fix occurs in its target".into());
    }
    Ok(sort_edits(out))
}

/// Each labelled choice's edits for one match (SR-LAW-4 decision 4). A
/// choice whose edits can't be built, or that has none (every step acts on
/// a capture this match did not bind), is left out; none left is an error.
pub fn choice_edits(spec: &FixSpec, m: &Match, ctx: &FixContext<'_>) -> Result<Vec<FixChoice>, String> {
    let FixSpec::Choices(choices) = spec else { return Err("not a `choices:` fix".into()) };
    let mut out = Vec::new();
    let mut first_error = None;
    for c in choices {
        let mut edits = Vec::new();
        match all_step_edits(&c.steps, m, ctx, &mut edits) {
            Ok(()) if edits.is_empty() => {
                first_error.get_or_insert_with(|| format!("the choice `{}` changes nothing for this match", c.label));
            }
            Ok(()) => out.push(FixChoice { label: c.label.clone(), default: c.default, edits: sort_edits(edits), files: Vec::new() }),
            Err(e) => {
                first_error.get_or_insert(e);
            }
        }
    }
    match (out.is_empty(), first_error) {
        (true, Some(e)) => Err(e),
        _ => Ok(out),
    }
}

/// The edits for every match of a group fix together.
pub fn group_edits(spec: &FixSpec, matches: &[Match], ctx: &FixContext<'_>) -> Result<Vec<TextEdit>, String> {
    let FixSpec::Steps(steps) = spec else { return Err("not a group fix".into()) };
    let mut out = Vec::new();
    let mut values = Vec::new();
    let mut types = Vec::new();
    let mut collect = None;
    for m in matches {
        for p in steps {
            match p {
                Primitive::CollectIntoBlock { target, block, type_block, position } => {
                    let (s, e) = range(target, m)?;
                    let node = node_at(ctx.root, s, e).ok_or("`collect-into-block`: no node at the target")?;
                    let (v, t) = declared(node, ctx.source);
                    values.extend(v);
                    types.extend(t);
                    collect = Some((block, type_block, *position));
                }
                Primitive::EditText { .. } => {}
                other => step_edits(other, m, ctx, &mut out)?,
            }
        }
        let text: Vec<&Primitive> = steps.iter().filter(|p| matches!(p, Primitive::EditText { .. })).collect();
        text_edits(&text, m, ctx, &mut out)?;
    }
    if let Some((block, type_block, position)) = collect {
        let at = place_offset(ctx, position);
        let mut lines = Vec::new();
        if !values.is_empty() {
            lines.push(block.replace("$$$NAMES", &values.join(", ")));
        }
        if !types.is_empty() {
            match type_block {
                Some(tb) => lines.push(tb.replace("$$$TYPE_NAMES", &types.join(", "))),
                None => return Err("`collect-into-block` found type-only names but has no `type_block`".into()),
            }
        }
        if !lines.is_empty() {
            out.push(TextEdit::insert(at, line_at(ctx, at, &lines.join("\n"))));
        }
    }
    Ok(sort_edits(out))
}

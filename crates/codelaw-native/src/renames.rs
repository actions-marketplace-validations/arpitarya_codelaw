//! The JavaScript rules whose fix renames a binding, or turns a default
//! export into a named one, in its module and in every project module that
//! imports it (SR-MULTI-FILE-FIXES): `js/new-cap`, `react/jsx-pascal-case`,
//! `react/hook-prefix`, `js/no-default-export` and `js/no-default-import`.
//! The renames are `multi_file`'s; the engine's safeguards check each across
//! the files it edits.

use crate::multi_file::{BTreeMapEdits, pascal, plan_edits, rename_binding_at, resolve_js, upper_first};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const JSX_ONLY: &[Language] = &[Language::Javascript, Language::Jsx, Language::Tsx];

fn finding(start: usize, end: usize, message: String, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start, end, message, edits }
}

/// `js/new-cap`: `new foo()` — the constructor renamed `Foo` where it is
/// declared and everywhere it is used.
pub struct NewCap;

impl NativeRule for NewCap {
    fn id(&self) -> &'static str {
        "js/new-cap"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["new_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(c) = node.field("constructor") else { return };
        let lower = |t: &str| t.starts_with(|ch: char| ch.is_ascii_lowercase() || ch == '_' || ch == '$');
        let name = match c.kind().as_ref() {
            "identifier" => c.text().to_string(),
            "member_expression" => match c.field("property") {
                Some(p) => p.text().to_string(),
                None => return,
            },
            _ => return,
        };
        if !lower(&name) {
            return;
        }
        let r = node.range();
        let edits = if c.kind() == "identifier" && name.starts_with(|ch: char| ch.is_ascii_lowercase()) {
            rename_binding_at(ctx, c.range().start, &upper_first(&name))
        } else {
            Vec::new()
        };
        out.push(finding(r.start, r.end, format!("`new {}`: a constructor's name starts with a capital letter", c.text()), edits));
    }
}

/// `react/jsx-pascal-case`: `<My_component />` — the component renamed
/// `MyComponent` where it is declared and everywhere it is used.
pub struct JsxPascalCase;

impl NativeRule for JsxPascalCase {
    fn id(&self) -> &'static str {
        "react/jsx-pascal-case"
    }
    fn languages(&self) -> &'static [Language] {
        JSX_ONLY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_opening_element", "jsx_self_closing_element"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(n) = node.field("name").filter(|n| n.kind() == "identifier") else { return };
        let name = n.text().to_string();
        let re = regex::Regex::new("^[A-Z]([A-Za-z0-9]*_[A-Za-z0-9_]*|[A-Z0-9]{2,})$").expect("a valid pattern");
        if !re.is_match(&name) {
            return;
        }
        let new = pascal(&name);
        let edits = if new != name && new.chars().next().is_some_and(|c| c.is_ascii_uppercase()) { rename_binding_at(ctx, n.range().start, &new) } else { Vec::new() };
        let r = node.range();
        out.push(finding(r.start, r.end, format!("`{name}` isn't PascalCase"), edits));
    }
}

/// `react/hook-prefix`: a function that calls hooks is a hook — renamed
/// `useX` where it is declared and everywhere it is called.
pub struct HookPrefix;

fn calls_hook(n: tree_sitter::Node<'_>, src: &str) -> bool {
    let mut c = n.walk();
    for k in n.children(&mut c) {
        if matches!(k.kind(), "function_declaration" | "arrow_function" | "function_expression") {
            continue;
        }
        if k.kind() == "call_expression"
            && let Some(f) = k.child_by_field_name("function")
        {
            let t = &src[f.byte_range()];
            let t = t.strip_prefix("React.").unwrap_or(t);
            if t.strip_prefix("use").is_some_and(|r| r.starts_with(|c: char| c.is_ascii_uppercase())) {
                return true;
            }
        }
        if calls_hook(k, src) {
            return true;
        }
    }
    false
}

impl NativeRule for HookPrefix {
    fn id(&self) -> &'static str {
        "react/hook-prefix"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["function_declaration"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let f = node.get_inner_node();
        let (Some(name), Some(body)) = (f.child_by_field_name("name"), f.child_by_field_name("body")) else { return };
        let n = &ctx.source[name.byte_range()];
        let is_hook = n.strip_prefix("use").is_some_and(|r| r.starts_with(|c: char| c.is_ascii_uppercase()));
        if !n.starts_with(|c: char| c.is_ascii_lowercase()) || is_hook || !calls_hook(body, ctx.source) {
            return;
        }
        let edits = rename_binding_at(ctx, name.start_byte(), &format!("use{}", upper_first(n)));
        out.push(finding(f.start_byte(), f.end_byte(), format!("`{n}` calls hooks, so it is a hook: name it `use…`"), edits));
    }
}

/// The name a default export gives its value, and the edit that makes it
/// a named export in its own module: `export default function load` loses
/// its `default`; `export default load;` becomes `export { load };`, or goes
/// when `load` is exported by name already.
fn named_export(node: tree_sitter::Node<'_>, src: &str, model: &codelaw_semantic::SemanticModel) -> Option<(String, TextEdit)> {
    let mut c = node.walk();
    let kids: Vec<tree_sitter::Node<'_>> = node.children(&mut c).collect();
    let d = kids.iter().position(|k| k.kind() == "default")?;
    let next = kids.get(d + 1)?;
    match next.kind() {
        "function_declaration" | "generator_function_declaration" | "class_declaration" | "function_expression" | "class" => {
            let name = next.child_by_field_name("name")?;
            let n = src[name.byte_range()].to_owned();
            Some((n, TextEdit::delete(kids[d].start_byte(), next.start_byte())))
        }
        "identifier" => {
            let n = src[next.byte_range()].to_owned();
            // A top-level binding of the module, not a global.
            model.symbols.iter().find(|s| s.name == n && s.scope == 0)?;
            let by_name = model.exports.iter().any(|e| e.name == n && e.source.is_none());
            let (s, e) = (node.start_byte(), node.end_byte());
            if by_name {
                let end = if src[e..].starts_with('\n') { e + 1 } else { e };
                Some((n, TextEdit::delete(s, end)))
            } else {
                let semi = if src[s..e].trim_end().ends_with(';') { ";" } else { "" };
                Some((n.clone(), TextEdit::new(s, e, format!("export {{ {n} }}{semi}"))))
            }
        }
        _ => None,
    }
}

/// The edit making a default import of `name` a named one, when the
/// import's clause is `x` or `x, { … }`.
fn named_import(src: &str, language: Language, at: usize, name: &str) -> Option<TextEdit> {
    let root = codelaw_lang::parse(language, src);
    let stmt = root.root().dfs().find(|n| n.kind() == "import_statement" && n.range().start <= at && at < n.range().end)?;
    let clause = stmt.children().find(|c| c.kind() == "import_clause")?;
    let kids: Vec<_> = clause.children().collect();
    let local = kids.first().filter(|k| k.kind() == "identifier")?;
    let x = local.text().to_string();
    let spec = if x == name { name.to_owned() } else { format!("{name} as {x}") };
    match kids.len() {
        1 => Some(TextEdit::new(local.range().start, local.range().end, format!("{{ {spec} }}"))),
        _ => {
            let named = kids.iter().find(|k| k.kind() == "named_imports")?;
            let open = named.range().start;
            let inner = named.text();
            let empty = inner.trim_start_matches('{').trim_end_matches('}').trim().is_empty();
            let text = if empty { format!("{{ {spec} ") } else { format!("{{ {spec}, ") };
            Some(TextEdit::new(local.range().start, open + 1 + (inner[1..].len() - inner[1..].trim_start().len()), text))
        }
    }
}

/// `js/no-default-export`: the default export made a named one, and every
/// project module's default import of it a named import.
pub struct NoDefaultExport;

impl NativeRule for NoDefaultExport {
    fn id(&self) -> &'static str {
        "js/no-default-export"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["export_statement"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let n = node.get_inner_node();
        let mut c = n.walk();
        if !n.children(&mut c).any(|k| k.kind() == "default") {
            return;
        }
        let edits = default_to_named(n, ctx).unwrap_or_default();
        out.push(finding(n.start_byte(), n.end_byte(), "Export this by name instead of as the default".into(), edits));
    }
}

fn default_to_named(n: tree_sitter::Node<'_>, ctx: &NativeContext<'_>) -> Option<Vec<TextEdit>> {
    let model = ctx.semantic?;
    let host = String::from_utf8_lossy(ctx.file.bytes()).trim_start_matches('\u{feff}').to_owned();
    if host != ctx.source {
        return None;
    }
    let (name, own) = named_export(n, ctx.source, model)?;
    let own_path = ctx.file.path().to_owned();
    let mut by: BTreeMapEdits = BTreeMapEdits::new();
    by.insert(own_path.clone(), vec![own]);
    for path in ctx.file.project_files()?.into_iter().filter(|p| *p != own_path && Language::from_path(std::path::Path::new(p)).is_some_and(|l| l.is_js_family())) {
        let Some(src) = ctx.file.read(&path) else { continue };
        let l = Language::from_path(std::path::Path::new(&path)).expect("filtered");
        let r = codelaw_lang::parse(l, &src);
        let Some(m) = codelaw_semantic::build(l, &r, &src) else { continue };
        let ours = |s: &str| resolve_js(ctx, &path, s).as_deref() == Some(own_path.as_str());
        if m.exports.iter().any(|e| e.source.as_deref().is_some_and(ours)) || crate::multi_file::loads_dynamically(ctx, &path, &src, &own_path) {
            return None;
        }
        let mut edits = Vec::new();
        for imp in m.imports.iter().filter(|i| ours(&i.source)) {
            match imp.imported {
                codelaw_semantic::Imported::Default => edits.push(named_import(&src, l, imp.start, &name)?),
                codelaw_semantic::Imported::Namespace => return None,
                _ => {}
            }
        }
        if !edits.is_empty() {
            by.insert(path, edits);
        }
    }
    Some(plan_edits(ctx, by, false))
}

/// `js/no-default-import`: `import x from "./m"` made `import { name as x }`
/// where the module's default export is a binding it also exports by name.
pub struct NoDefaultImport;

impl NativeRule for NoDefaultImport {
    fn id(&self) -> &'static str {
        "js/no-default-import"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["import_statement"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(clause) = node.children().find(|c| c.kind() == "import_clause") else { return };
        if !clause.children().any(|k| k.kind() == "identifier") {
            return;
        }
        let r = node.range();
        let edits = named_of_default(node.clone(), ctx).unwrap_or_default();
        out.push(finding(r.start, r.end, "Import a named export instead of the default".into(), edits));
    }
}

fn named_of_default(node: SyntaxNode<'_>, ctx: &NativeContext<'_>) -> Option<Vec<TextEdit>> {
    let source = node.field("source")?;
    let spec = source.text();
    let spec = spec.trim_matches(['"', '\'']);
    let target = resolve_js(ctx, ctx.file.path(), spec)?;
    let text = ctx.file.read(&target)?;
    let l = Language::from_path(std::path::Path::new(&target)).filter(|l| l.is_js_family())?;
    let root = codelaw_lang::parse(l, &text);
    let m = codelaw_semantic::build(l, &root, &text)?;
    let default = m.exports.iter().find(|e| e.name == "default" && e.source.is_none())?;
    let local = default.local?;
    let name = m.symbol(local).name.clone();
    m.exports.iter().find(|e| e.name == name && e.local == Some(local))?;
    let e = named_import(ctx.source, ctx.language, node.range().start, &name)?;
    Some(crate::as_suggestion(vec![e]))
}

/// `react/jsx-no-undef`: `<Header />` where nothing declares `Header` —
/// imported from the project module that exports it, where one does.
pub struct JsxNoUndef;

impl NativeRule for JsxNoUndef {
    fn id(&self) -> &'static str {
        "react/jsx-no-undef"
    }
    fn languages(&self) -> &'static [Language] {
        JSX_ONLY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_opening_element", "jsx_self_closing_element"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let Some(n) = node.field("name").filter(|n| n.kind() == "identifier") else { return };
        let name = n.text().to_string();
        if !name.starts_with(|c: char| c.is_ascii_uppercase()) {
            return;
        }
        let r = n.range();
        let Some(reference) = model.reference_at(r.start) else { return };
        if !model.is_undeclared(reference, &|_| false) {
            return;
        }
        let edits = crate::multi_file::import_choices(ctx, &name);
        out.push(finding(r.start, r.end, format!("`{name}` is not defined; import or declare the component"), edits));
    }
}

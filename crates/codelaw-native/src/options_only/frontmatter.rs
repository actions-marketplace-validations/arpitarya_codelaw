//! `md/require-frontmatter-fields` (W-178): a Markdown file's front matter
//! carries the fields a project lists. Silent until a project sets `fields`.

use super::{finding, opt_strings};
use crate::util::text;
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use std::collections::BTreeMap;

const MD: &[Language] = &[Language::Markdown];

/// The top-level fields of a front matter block and whether each has a value.
/// YAML (`---`): a field is a line that starts in column 0 with a key and a
/// colon; its value is the rest of that line, or the indented lines and list
/// items under it. TOML (`+++`): a `key = value` line before the first table.
fn fields_of(block: &str, toml: bool) -> BTreeMap<String, bool> {
    let mut out: BTreeMap<String, bool> = BTreeMap::new();
    let mut lines: Vec<&str> = block.lines().collect();
    if lines.len() < 2 {
        return out;
    }
    lines.remove(0);
    if lines.last().is_some_and(|l| matches!(l.trim_end(), "---" | "+++" | "...")) {
        lines.pop();
    }
    let mut current: Option<String> = None;
    for l in lines {
        if l.trim().is_empty() || l.trim_start().starts_with('#') {
            continue;
        }
        if toml && l.starts_with('[') {
            break;
        }
        let indented = l.starts_with([' ', '\t']) || (!toml && l.starts_with('-'));
        if indented {
            if let Some(k) = &current {
                out.insert(k.clone(), true);
            }
            continue;
        }
        let sep = if toml { '=' } else { ':' };
        let Some(i) = l.find(sep) else { continue };
        let key = l[..i].trim().trim_matches(['"', '\'']).to_owned();
        let value = l[i + 1..].trim();
        let value = if toml || !value.starts_with('#') { value } else { "" };
        let empty = value.is_empty() || matches!(value, "\"\"" | "''" | "[]" | "{}" | "null" | "~");
        if !key.is_empty() {
            out.insert(key.clone(), !empty);
            current = Some(key);
        }
    }
    out
}

pub struct RequireFrontmatterFields;

impl NativeRule for RequireFrontmatterFields {
    fn id(&self) -> &'static str {
        "md/require-frontmatter-fields"
    }
    fn languages(&self) -> &'static [Language] {
        MD
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["document"]
    }
    fn needs_options(&self) -> &'static [&'static str] {
        &["fields"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let fields = opt_strings(ctx, "fields");
        if fields.is_empty() || src.trim().is_empty() {
            return;
        }
        let doc = node.get_inner_node();
        let block = crate::util::named_children(doc).into_iter().find(|c| matches!(c.kind(), "minus_metadata" | "plus_metadata") && src[..c.start_byte()].trim_start_matches('\u{feff}').is_empty());
        let list = |names: &[&String]| names.iter().map(|f| format!("`{f}`")).collect::<Vec<_>>().join(", ");
        match block {
            None => {
                let end = src.find('\n').unwrap_or(src.len());
                out.push(finding(0, end, format!("The file has no front matter; the project requires {}", list(&fields.iter().collect::<Vec<_>>())), Vec::new()));
            }
            Some(b) => {
                let have = fields_of(text(b, src), b.kind() == "plus_metadata");
                let missing: Vec<&String> = fields.iter().filter(|f| !have.get(f.as_str()).copied().unwrap_or(false)).collect();
                if !missing.is_empty() {
                    let line_end = src[b.start_byte()..].find('\n').map_or(src.len(), |i| b.start_byte() + i);
                    out.push(finding(b.start_byte(), line_end, format!("The front matter is missing {}", list(&missing)), Vec::new()));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_fields_and_their_values() {
        let f = fields_of("---\ntitle: A\ndescription:\ntags:\n  - x\nempty: \"\"\n# c: 1\n---", false);
        assert_eq!(f.get("title"), Some(&true));
        assert_eq!(f.get("description"), Some(&false));
        assert_eq!(f.get("tags"), Some(&true));
        assert_eq!(f.get("empty"), Some(&false));
        assert!(!f.contains_key("c"));
    }

    #[test]
    fn toml_fields_stop_at_the_first_table() {
        let f = fields_of("+++\ntitle = \"A\"\n[extra]\ndescription = \"B\"\n+++", true);
        assert_eq!(f.get("title"), Some(&true));
        assert!(!f.contains_key("description"));
    }
}

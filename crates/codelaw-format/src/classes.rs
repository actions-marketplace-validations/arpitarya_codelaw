//! Class sorting (SR-FORMATTER decision 9): every class list of a file put
//! in the order of the provider's table, before the engine lays the file
//! out. Only the text inside each list changes.

use crate::FormatError;
use crate::options::ClassSort;
use codelaw_core::Language;
use codelaw_core::class_table::sort_classes;

/// The file with its class lists sorted; `None` when none changes. A file
/// that does not parse is left to the engine, which reports it.
pub fn sort_class_lists(language: Language, text: &str, sort: &ClassSort) -> Result<Option<String>, FormatError> {
    if !matches!(language, Language::Html | Language::Javascript | Language::Jsx | Language::Typescript | Language::Tsx) {
        return Ok(None);
    }
    let root = codelaw_lang::parse(language, text);
    // A list inside a part codelaw's parser could not read is left alone;
    // the rest of the file is sorted. (tree-sitter rejects some JSX that the
    // engines accept, such as `&` in an attribute string; the engine that
    // formats the file is the judge of whether it parses.)
    let broken: Vec<(usize, usize)> = if codelaw_lang::syntax_error_count(&root) > 0 {
        root.root().dfs().filter(|n| n.is_error() || n.get_inner_node().is_missing()).map(|n| (n.range().start, n.range().end)).collect()
    } else {
        Vec::new()
    };
    let lists: Vec<_> = codelaw_lang::class_lists::class_lists(language, &root, &sort.lists)
        .into_iter()
        .filter(|l| !broken.iter().any(|&(a, b)| l.start < b && a < l.end))
        .collect();
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    let mut changed = false;
    for l in lists {
        if l.start < at {
            continue;
        }
        let sorted = sort_classes(&text[l.start..l.end], sort.table.as_ref(), l.sort).map_err(|e| FormatError::Engine(format!("the class table: {e}")))?;
        if let Some(s) = sorted {
            out.push_str(&text[at..l.start]);
            out.push_str(&s);
            at = l.end;
            changed = true;
        }
    }
    if !changed {
        return Ok(None);
    }
    out.push_str(&text[at..]);
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codelaw_core::class_table::FixedTable;
    use codelaw_lang::class_lists::ClassListOptions;
    use std::sync::Arc;

    fn sort() -> ClassSort {
        let table = FixedTable::ordered(&[("block", &["display"]), ("flex", &["display"]), ("p-4", &["padding"]), ("md:p-2", &["padding"])]);
        ClassSort { table: Arc::new(table), lists: ClassListOptions::default() }
    }

    #[test]
    fn jsx_html_and_class_functions_are_sorted() {
        let src = "const a = <div className=\"md:p-2  p-4 foo flex\" title=\"p-4 flex\" />;\nconst b = cn(\"p-4 flex\", ok && \"md:p-2 block\");\n";
        let out = sort_class_lists(Language::Jsx, src, &sort()).unwrap().unwrap();
        assert_eq!(out, "const a = <div className=\"foo flex p-4 md:p-2\" title=\"p-4 flex\" />;\nconst b = cn(\"flex p-4\", ok && \"block md:p-2\");\n");
        let html = "<p class=\"p-4 flex\">x</p>\n";
        assert_eq!(sort_class_lists(Language::Html, html, &sort()).unwrap().unwrap(), "<p class=\"flex p-4\">x</p>\n");
        assert_eq!(sort_class_lists(Language::Css, ".a{}", &sort()).unwrap(), None);
    }

    #[test]
    fn a_sorted_file_formats_to_a_fixpoint() {
        let o = crate::FormatOptions { class_sort: Some(sort()), ..Default::default() };
        let out = crate::format(Language::Tsx, "a.tsx", "export const a = <div className=\"p-4 flex\" />;\n", &o).unwrap();
        assert_eq!(out, "export const a = <div className=\"flex p-4\" />;\n");
    }
}

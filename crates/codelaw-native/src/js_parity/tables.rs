//! W-139 strand W-139-js: eslint-plugin-sonarjs 4.2.2's
//! `table-header-reference` (behaviour only: SonarJS is source-available,
//! nothing here is copied), written again. A JSX `<table>` whose cells
//! carry `headers` is laid out as a grid — `colSpan` and `rowSpan` taken
//! into account — and each id a cell names must be a `<th>` in one of its
//! own columns or rows.

use super::finding;
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

const JSX: &[Language] = &[Language::Jsx, Language::Tsx];

#[derive(Clone)]
struct Cell<'t> {
    header: bool,
    headers: Option<Vec<String>>,
    id: Option<String>,
    node: Node<'t>,
    internal: usize,
    row_span: usize,
}

fn tag_name<'s>(el: Node<'_>, src: &'s str) -> Option<&'s str> {
    let open = if el.kind() == "jsx_self_closing_element" { el } else { el.child_by_field_name("open_tag")? };
    open.child_by_field_name("name").map(|n| text(n, src))
}

fn open_of<'t>(el: Node<'t>) -> Option<Node<'t>> {
    if el.kind() == "jsx_self_closing_element" { Some(el) } else { el.child_by_field_name("open_tag") }
}

/// The literal value of an attribute, `None` when it is not a literal.
fn attr<'t>(el: Node<'t>, name: &str, src: &str) -> Option<Option<String>> {
    let open = open_of(el)?;
    for a in named_children(open) {
        if a.kind() != "jsx_attribute" {
            continue;
        }
        let Some(k) = a.named_child(0) else { continue };
        if !text(k, src).eq_ignore_ascii_case(name) {
            continue;
        }
        let Some(v) = a.named_child(1) else { return Some(None) };
        return Some(match v.kind() {
            "string" => Some(crate::util::string_value(v, src)),
            "jsx_expression" => v.named_child(0).and_then(|e| match e.kind() {
                "string" => Some(crate::util::string_value(e, src)),
                "number" => Some(text(e, src).to_owned()),
                "template_string" if !named_children(e).iter().any(|c| c.kind() == "template_substitution") => Some(text(e, src).trim_matches('`').to_owned()),
                _ => None,
            }),
            _ => None,
        });
    }
    None
}

fn span(el: Node<'_>, name: &str, src: &str, max: usize) -> Option<usize> {
    match attr(el, name, src) {
        None => Some(1),
        Some(None) => None,
        Some(Some(v)) => {
            let digits: String = v.trim().chars().take_while(char::is_ascii_digit).collect();
            let n: i64 = digits.parse().ok()?;
            if name == "rowspan" && n == 0 {
                return Some(0);
            }
            Some((n.max(1) as usize).min(max))
        }
    }
}

fn is_html(el: Node<'_>, src: &str) -> bool {
    tag_name(el, src).is_some_and(|n| n.chars().next().is_some_and(|c| c.is_ascii_lowercase()) && !n.contains('.'))
}

fn body_children<'t>(el: Node<'t>) -> Vec<Node<'t>> {
    named_children(el).into_iter().filter(|c| !matches!(c.kind(), "jsx_opening_element" | "jsx_closing_element")).collect()
}

fn extract_rows<'t>(tree: Node<'t>, src: &str, count: &mut usize) -> Option<Vec<Vec<Cell<'t>>>> {
    let mut rows = Vec::new();
    let mut unknown = false;
    for child in body_children(tree) {
        match child.kind() {
            "jsx_element" | "jsx_self_closing_element" => {
                let Some(name) = tag_name(child, src) else {
                    // A fragment.
                    match extract_rows(child, src, count) {
                        Some(r) => rows.extend(r),
                        None => unknown = true,
                    }
                    continue;
                };
                match name.to_lowercase().as_str() {
                    "tr" => match extract_row(child, src, count) {
                        Some(r) => rows.push(r),
                        None => unknown = true,
                    },
                    "table" => {}
                    "thead" | "tbody" | "tfoot" => match extract_rows(child, src, count) {
                        Some(r) => rows.extend(r),
                        None => unknown = true,
                    },
                    _ if !is_html(child, src) => unknown = true,
                    _ => {}
                }
            }
            "jsx_expression" if child.named_child(0).is_some_and(|e| e.kind() != "comment") => unknown = true,
            _ => {}
        }
    }
    (!unknown).then_some(rows)
}

fn extract_row<'t>(tr: Node<'t>, src: &str, count: &mut usize) -> Option<Vec<Cell<'t>>> {
    let mut row = Vec::new();
    for child in body_children(tr) {
        if child.kind() == "jsx_text" || child.kind() == "jsx_expression" && child.named_child(0).is_none_or(|e| e.kind() == "comment") {
            continue;
        }
        if !matches!(child.kind(), "jsx_element" | "jsx_self_closing_element") || !matches!(tag_name(child, src), Some("td" | "th")) {
            return None;
        }
        let col = span(child, "colspan", src, 1000)?;
        let rowspan = span(child, "rowspan", src, 65534)?;
        let headers = match attr(child, "headers", src) {
            Some(Some(v)) if !v.trim().is_empty() => Some(v.split_whitespace().map(str::to_owned).collect()),
            _ => None,
        };
        let id = attr(child, "id", src).flatten();
        for _ in 0..col {
            row.push(Cell { header: tag_name(child, src) == Some("th"), headers: headers.clone(), id: id.clone(), node: child, internal: *count, row_span: rowspan });
        }
        *count += 1;
    }
    Some(row)
}

fn grid<'t>(table: Node<'t>, src: &str) -> Option<Vec<Vec<Cell<'t>>>> {
    let mut count = 0;
    let rows = extract_rows(table, src, &mut count)?;
    if rows.is_empty() {
        return Some(Vec::new());
    }
    let ncols = rows[0].len();
    let mut columns: Vec<Option<Cell<'t>>> = vec![None; ncols];
    let mut row = 0;
    let mut result = Vec::new();
    while row < rows.len() {
        let mut out_row = Vec::new();
        let mut index = 0;
        let mut used = false;
        let mut only_max = true;
        for slot in columns.iter_mut() {
            if slot.is_none() {
                if index == rows[row].len() {
                    continue;
                }
                *slot = Some(rows[row][index].clone());
                index += 1;
                used = true;
            }
            let Some(cur) = slot.as_mut() else { continue };
            out_row.push(cur.clone());
            if cur.row_span > 0 {
                only_max = false;
                cur.row_span -= 1;
                if cur.row_span == 0 {
                    *slot = None;
                }
            }
        }
        if only_max {
            break;
        }
        result.push(out_row);
        if used {
            row += 1;
        }
    }
    Some(result)
}

pub struct TableHeaderReference;
impl NativeRule for TableHeaderReference {
    fn id(&self) -> &'static str {
        "js/table-header-reference"
    }
    fn languages(&self) -> &'static [Language] {
        JSX
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["jsx_element"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let el = node.get_inner_node();
        let src = ctx.source;
        if tag_name(el, src) != Some("table") {
            return;
        }
        let Some(g) = grid(el, src) else { return };
        if g.is_empty() {
            return;
        }
        let row_headers: Vec<BTreeSet<&str>> = g.iter().map(|r| r.iter().filter(|c| c.header).filter_map(|c| c.id.as_deref()).collect()).collect();
        let ncols = g[0].len();
        let col_headers: Vec<BTreeSet<&str>> = (0..ncols).map(|i| g.iter().filter_map(|r| r.get(i)).filter(|c| c.header).filter_map(|c| c.id.as_deref()).collect()).collect();
        let all: BTreeSet<&str> = row_headers.iter().chain(col_headers.iter()).flat_map(|s| s.iter().copied()).collect();
        // Each block's extent, by its internal number.
        let mut blocks: BTreeMap<usize, (usize, usize, usize, usize, &Cell<'_>)> = BTreeMap::new();
        for (r, row) in g.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if cell.headers.is_none() {
                    continue;
                }
                blocks.entry(cell.internal).and_modify(|b| {
                    b.1 = r;
                    b.3 = c;
                }).or_insert((r, r, c, c, cell));
            }
        }
        for (min_row, max_row, min_col, max_col, cell) in blocks.values() {
            let mut actual: BTreeSet<&str> = BTreeSet::new();
            for s in col_headers.iter().skip(*min_col).take(max_col - min_col + 1) {
                actual.extend(s.iter().copied());
            }
            for s in row_headers.iter().skip(*min_row).take(max_row - min_row + 1) {
                actual.extend(s.iter().copied());
            }
            for h in cell.headers.iter().flatten() {
                if actual.contains(h.as_str()) {
                    continue;
                }
                let msg = if all.contains(h.as_str()) { format!("The id \"{h}\" in `headers` names the header of another column or row") } else { format!("The id \"{h}\" in `headers` does not name any `<th>` of this table") };
                out.push(finding(cell.node, msg, vec![]));
            }
        }
    }
}

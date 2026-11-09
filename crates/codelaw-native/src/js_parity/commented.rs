//! W-139 strand W-139-js: eslint-plugin-sonarjs 4.2.2's `no-commented-code`
//! (behaviour only: SonarJS is source-available, nothing here is copied),
//! written again. A comment — or a run of adjacent line comments — whose
//! text parses as JavaScript and looks like code is commented-out code.

use super::span;
use crate::util::{named_children, text, whole_lines};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

struct Group {
    start: usize,
    end: usize,
    text: String,
}

fn comments<'t>(root: Node<'t>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(x) = stack.pop() {
        if x.kind() == "comment" {
            out.push(x);
            continue;
        }
        let mut c = named_children(x);
        c.reverse();
        stack.extend(c);
    }
    out.sort_by_key(|c| c.start_byte());
    out
}

fn groups(root: Node<'_>, src: &str) -> Vec<Group> {
    let mut out: Vec<Group> = Vec::new();
    let mut run: Option<(Group, usize)> = None; // the group and the row of its last comment
    for c in comments(root) {
        let t = text(c, src);
        if let Some(rest) = t.strip_prefix("//") {
            let row = c.start_position().row;
            let alone = src[..c.start_byte()].rsplit('\n').next().is_some_and(|l| l.trim().is_empty());
            match run.as_mut() {
                Some((g, last)) if *last + 1 == row && alone => {
                    g.end = c.end_byte();
                    g.text.push('\n');
                    g.text.push_str(rest);
                    *last = row;
                }
                _ => {
                    if let Some((g, _)) = run.take() {
                        out.push(g);
                    }
                    run = Some((Group { start: c.start_byte(), end: c.end_byte(), text: rest.to_owned() }, row));
                }
            }
        } else {
            if let Some((g, _)) = run.take() {
                out.push(g);
            }
            let inner = t.strip_prefix("/*").and_then(|x| x.strip_suffix("*/")).unwrap_or(t);
            out.push(Group { start: c.start_byte(), end: c.end_byte(), text: inner.to_owned() });
        }
    }
    if let Some((g, _)) = run.take() {
        out.push(g);
    }
    out
}

fn doc_example(s: &str) -> bool {
    let first = s.lines().next().unwrap_or("");
    let stripped = first.trim_start_matches("//").trim().to_lowercase();
    stripped.starts_with("e.g.") || stripped.starts_with("e.g:") || stripped.starts_with("for example") || stripped.starts_with("example:") || stripped.starts_with("examples:")
}

/// A cheap check that something in the text could be code: a character a
/// statement needs.
fn prefilter(s: &str) -> bool {
    s.contains([';', '{', '}', '(', ')', '=', '<', '>']) || s.contains("import ") && (s.contains('"') || s.contains('\'')) || s.contains("export ")
}

fn looks_like_code(s: &str) -> bool {
    s.lines().any(|l| {
        let t = l.trim().trim_start_matches('*').trim();
        if t.is_empty() {
            return false;
        }
        t.ends_with([';', '{', '}'])
            || ["!=", "==", "<=", ">=", "&&", "||", "++", "--", "=>", "+=", "-=", "?.", "??"].iter().any(|op| t.contains(op))
            || (t.contains('(') && t.ends_with(')') && !t.contains(' ') || t.contains('(') && t.ends_with(");"))
            || ["if (", "for (", "while (", "function ", "return ", "const ", "let ", "var ", "import ", "export ", "class ", "await ", "throw "].iter().any(|k| t.starts_with(k) && (t.contains('=') || t.contains('(') || t.contains(';')))
    })
}

fn inject_braces(s: &str) -> String {
    let (mut balance, mut min) = (0i32, 0i32);
    for c in s.chars() {
        match c {
            '{' => balance += 1,
            '}' => {
                balance -= 1;
                min = min.min(balance);
            }
            _ => {}
        }
    }
    let open = -min;
    let close = balance - min;
    if open > 0 || close > 0 {
        format!("{}{s}{}", "{".repeat(open as usize), "}".repeat(close as usize))
    } else {
        s.to_owned()
    }
}

fn excluded(body: &[Node<'_>], s: &str, src: &str) -> bool {
    if body.len() != 1 {
        return false;
    }
    let st = body[0];
    let _ = src;
    match st.kind() {
        "break_statement" | "labeled_statement" | "continue_statement" => true,
        "return_statement" | "throw_statement" => named_children(st).iter().all(|c| c.kind() == "identifier" || c.kind() == "comment") ,
        "expression_statement" => {
            let Some(e) = st.named_child(0) else { return true };
            if matches!(e.kind(), "identifier" | "sequence_expression" | "string" | "number") {
                return true;
            }
            if e.kind() == "unary_expression" && matches!(text(e.child(0).unwrap_or(e), s), "+" | "-") {
                return true;
            }
            !text(st, s).trim_end().ends_with(';')
        }
        _ => false,
    }
}

fn contains_code(value: &str, language: Language) -> bool {
    if !prefilter(value) || !looks_like_code(value) {
        return false;
    }
    let root = codelaw_lang::parse(language, value);
    if codelaw_lang::syntax_error_count(&root) == 0 {
        let top = root.root().get_inner_node();
        let body: Vec<Node<'_>> = named_children(top).into_iter().filter(|c| !matches!(c.kind(), "comment" | "hash_bang_line")).collect();
        return !body.is_empty() && !excluded(&body, value, value);
    }
    // Attributes of a JSX element, written alone.
    let wrapped = format!("<_S125Wrapper {value} />");
    let root = codelaw_lang::parse(Language::Jsx, &wrapped);
    if codelaw_lang::syntax_error_count(&root) != 0 {
        return false;
    }
    let top = root.root().get_inner_node();
    let body: Vec<Node<'_>> = named_children(top).into_iter().filter(|c| c.kind() != "comment").collect();
    body.len() == 1 && text(body[0], &wrapped).starts_with("<_S125Wrapper") && text(body[0], &wrapped).contains('=')
}

pub struct NoCommentedCode;
impl NativeRule for NoCommentedCode {
    fn id(&self) -> &'static str {
        "js/no-commented-code"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["program"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let src = ctx.source;
        let lang = if matches!(ctx.language, Language::Javascript | Language::Jsx) { Language::Javascript } else { ctx.language };
        for g in groups(node.get_inner_node(), src) {
            let trimmed = g.text.trim();
            if trimmed == "}" || doc_example(trimmed) || !contains_code(&inject_braces(trimmed), lang) {
                continue;
            }
            let alone = src[..g.start].rsplit('\n').next().is_some_and(|l| l.trim().is_empty()) && src[g.end..].split('\n').next().is_some_and(|l| l.trim().is_empty());
            let edit = if alone {
                let (s, e) = whole_lines(src, g.start, g.end);
                TextEdit::delete(s, e)
            } else {
                TextEdit::delete(g.start, g.end)
            };
            out.push(span(g.start, g.end, "This comment holds code that is switched off: delete it (version control keeps it) or bring it back", vec![edit]));
        }
    }
}

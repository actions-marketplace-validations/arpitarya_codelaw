//! `js/no-unreachable`: statements after a `return`, `throw`, `break` or
//! `continue` in the same block never run. A pattern can match the exit,
//! but not "every statement after it, unless hoisted".

use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use crate::util::{exits, exits_with, whole_lines};

pub struct NoUnreachable;

/// Hoisted declarations still take effect, so they are not unreachable.
fn hoisted(node: &tree_sitter::Node<'_>) -> bool {
    match node.kind() {
        "function_declaration" | "generator_function_declaration" | "empty_statement" | "comment" => true,
        "variable_declaration" => {
            let mut c = node.walk();
            node.named_children(&mut c).all(|d| d.kind() != "variable_declarator" || d.child_by_field_name("value").is_none())
        }
        _ => false,
    }
}

impl NativeRule for NoUnreachable {
    fn id(&self) -> &'static str {
        "js/no-unreachable"
    }

    fn languages(&self) -> &'static [Language] {
        &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx]
    }

    fn kinds(&self) -> &'static [&'static str] {
        &["program", "statement_block", "switch_case", "switch_default"]
    }

    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        scan(node, ctx, false, out);
    }
}

/// A `process.exit(…)` call statement, read as a `throw` (eslint-plugin-n's
/// `process-exit-as-throw`).
fn process_exit(n: &tree_sitter::Node<'_>, src: &str) -> bool {
    if n.kind() != "expression_statement" {
        return false;
    }
    let Some(call) = n.named_child(0).filter(|c| c.kind() == "call_expression") else { return false };
    let Some(f) = call.child_by_field_name("function").filter(|f| f.kind() == "member_expression") else { return false };
    let (Some(o), Some(p)) = (f.child_by_field_name("object"), f.child_by_field_name("property")) else { return false };
    let optional = crate::util::children(f).iter().any(|c| c.kind() == "optional_chain" || c.kind() == "?.") || crate::util::children(call).iter().any(|c| c.kind() == "optional_chain" || c.kind() == "?.");
    o.kind() == "identifier" && &src[o.byte_range()] == "process" && &src[p.byte_range()] == "exit" && !optional
}

/// Whether the file makes its own `process`, which is then not the global.
fn declares_process(src: &str) -> bool {
    static OWN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"\b(?:const|let|var|function|class)\s+process\b|\bimport\s+(?:\*\s+as\s+)?process\b|\bimport\s*\{[^}]*\bprocess\b[^}]*\}|[{,]\s*process\s*[,}]|(?:^|[^.\w])process\s*=[^=]").unwrap()
    });
    OWN.is_match(src)
}

/// The statements after one that leaves the block. With `as_throw`, only
/// the runs that follow a `process.exit()` and nothing else that already
/// leaves (`js/no-unreachable` owns those).
fn scan(node: SyntaxNode<'_>, ctx: &NativeContext<'_>, as_throw: bool, out: &mut Vec<NativeFinding>) {
    {
        let src = ctx.source;
        if as_throw && declares_process(src) {
            return;
        }
        let ts = node.get_inner_node();
        let mut c = ts.walk();
        let statements: Vec<_> = ts.named_children(&mut c).filter(|n| !n.is_extra()).collect();
        let body_start = if matches!(ts.kind(), "switch_case" | "switch_default") {
            // Skip the `case <value>` expression: only statements count.
            ts.child_by_field_name("value").map(|v| v.end_byte()).unwrap_or(0)
        } else {
            0
        };
        let mut exited = false;
        let mut run: Option<(usize, usize)> = None;
        let flush = |run: &mut Option<(usize, usize)>, out: &mut Vec<NativeFinding>| {
            if let Some((start, end)) = run.take() {
                let (a, b) = whole_lines(src, start, end);
                out.push(NativeFinding {
                    start,
                    end,
                    message: if as_throw {
                        "Unreachable code: it comes after `process.exit()`, which ends the process like a `throw`".into()
                    } else {
                        "Unreachable code: it comes after a `return`, `throw`, `break` or `continue`".into()
                    },
                    edits: vec![TextEdit::delete(a, b)],
                });
            }
        };
        for s in statements {
            if s.start_byte() < body_start {
                continue;
            }
            if exited {
                if hoisted(&s) {
                    flush(&mut run, out);
                } else {
                    run = Some(match run {
                        Some((start, _)) => (start, s.end_byte()),
                        None => (s.start_byte(), s.end_byte()),
                    });
                }
            } else if as_throw {
                if exits(&s) {
                    // Already unreachable past here, as `js/no-unreachable` reports.
                    break;
                }
                if exits_with(&s, &|n| process_exit(n, src)) {
                    exited = true;
                }
            } else if exits(&s) {
                exited = true;
            }
        }
        flush(&mut run, out);
    }
}

/// `node/process-exit-as-throw`: eslint-plugin-n reports nothing itself, it
/// makes the code-path rules read `process.exit()` as a `throw`; the one
/// consequence that stands alone in a file — what follows it never runs — is
/// reported here.
pub struct ProcessExitAsThrow;

impl NativeRule for ProcessExitAsThrow {
    fn id(&self) -> &'static str {
        "node/process-exit-as-throw"
    }

    fn languages(&self) -> &'static [Language] {
        &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx]
    }

    fn kinds(&self) -> &'static [&'static str] {
        &["program", "statement_block", "switch_case", "switch_default"]
    }

    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        scan(node, ctx, true, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(src: &str) -> Vec<String> {
        crate::run(&NoUnreachable, Language::Typescript, src).into_iter().map(|f| src[f.start..f.end].to_owned()).collect()
    }

    #[test]
    fn after_exits() {
        assert_eq!(found("function f() { return 1; a(); b(); }"), ["a(); b();"]);
        assert_eq!(found("function f() { throw e; function g() {} var v; c(); }"), ["c();"]);
        assert_eq!(found("for (;;) { if (x) { break; } else { continue; } y(); }"), ["y();"]);
        assert_eq!(found("switch (x) { case 1: return 1; z(); default: w(); }"), ["z();"]);
        assert_eq!(found("function f() { try { return 1; } finally { cleanup(); } after(); }"), ["after();"]);
    }

    fn after_exit_call(src: &str) -> Vec<String> {
        crate::run(&ProcessExitAsThrow, Language::Typescript, src).into_iter().map(|f| src[f.start..f.end].to_owned()).collect()
    }

    #[test]
    fn process_exit_is_a_throw() {
        assert_eq!(after_exit_call("function f() { process.exit(1); a(); b(); }"), ["a(); b();"]);
        assert_eq!(after_exit_call("function f() { if (x) { process.exit(1); } else { process.exit(2); } a(); }"), ["a();"]);
        assert_eq!(after_exit_call("function f() { try { process.exit(1); } finally { c(); } d(); }"), ["d();"]);
        assert!(after_exit_call("function f() { if (x) process.exit(1); a(); }").is_empty());
        assert!(after_exit_call("function f() { return 1; a(); }").is_empty(), "js/no-unreachable owns those");
        assert!(after_exit_call("const process = mine();\nfunction f() { process.exit(1); a(); }").is_empty(), "a local process is not the global");
        assert!(after_exit_call("function f() { process.exit?.(1); a(); }").is_empty());
        assert!(found("function f() { process.exit(1); a(); }").is_empty(), "js/no-unreachable keeps ESLint's reading");
    }

    #[test]
    fn reachable_code_is_left_alone() {
        assert!(found("function f() { if (x) return 1; return 2; }").is_empty());
        assert!(found("function f() { try { return 1; } catch (e) { log(e); } after(); }").is_empty());
        assert!(found("function f() { return; var hoisted; }").is_empty());
    }
}

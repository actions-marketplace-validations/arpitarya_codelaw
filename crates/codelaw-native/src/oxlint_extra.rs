//! Oxlint-only rules (W-139, W-148's 18 rows: the React ones and
//! `branches-sharing-code`) and Biome's `noConstantMathMinMaxClamp`, each
//! its documented behaviour, reimplemented.

use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const JSX: &[Language] = &[Language::Javascript, Language::Jsx, Language::Tsx];

fn finding(n: Node<'_>, message: impl Into<String>, edits: Vec<TextEdit>) -> NativeFinding {
    NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.into(), edits }
}

macro_rules! rule {
    ($name:ident, $id:literal, $langs:expr, $kinds:expr, |$n:ident, $src:ident, $out:ident| $body:block) => {
        pub struct $name;
        impl NativeRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn languages(&self) -> &'static [Language] {
                $langs
            }
            fn kinds(&self) -> &'static [&'static str] {
                $kinds
            }
            fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, $out: &mut Vec<NativeFinding>) {
                let $n = node.get_inner_node();
                let $src = ctx.source;
                $body
            }
        }
    };
}

fn callee<'s>(call: Node<'_>, src: &'s str) -> Option<&'s str> {
    call.child_by_field_name("function").map(|f| text(f, src))
}

rule!(NoCloneElement, "react/no-clone-element", JSX, &["call_expression"], |n, src, out| {
    if matches!(callee(n, src), Some("React.cloneElement" | "cloneElement")) {
        out.push(finding(n, "Don't clone elements to change their props: pass the props, or a render function, instead", vec![]));
    }
});

rule!(NoReactChildren, "react/no-react-children", JSX, &["member_expression"], |n, src, out| {
    let t = text(n, src);
    if (t.starts_with("React.Children.") || t.starts_with("Children.")) && n.parent().is_some_and(|p| p.kind() == "call_expression") {
        out.push(finding(n, "`Children` utilities read a component's children as data, which breaks with fragments and wrappers: pass the data as props", vec![]));
    }
});

fn is_component_name(s: &str) -> bool {
    s.starts_with(|c: char| c.is_ascii_uppercase())
}

/// What a top-level export declares: (name, is a function or class).
fn export_names<'t>(stmt: Node<'t>, src: &str) -> Vec<(String, bool, Node<'t>)> {
    let mut out = Vec::new();
    let Some(d) = stmt.child_by_field_name("declaration") else { return out };
    match d.kind() {
        "function_declaration" | "class_declaration" => {
            if let Some(nm) = d.child_by_field_name("name") {
                out.push((text(nm, src).to_owned(), true, d));
            }
        }
        "lexical_declaration" | "variable_declaration" => {
            for v in named_children(d).into_iter().filter(|c| c.kind() == "variable_declarator") {
                if let Some(nm) = v.child_by_field_name("name").filter(|x| x.kind() == "identifier") {
                    let f = v.child_by_field_name("value").is_some_and(|x| matches!(x.kind(), "arrow_function" | "function_expression" | "function" | "call_expression"));
                    out.push((text(nm, src).to_owned(), f, v));
                }
            }
        }
        _ => {}
    }
    out
}

rule!(OnlyExportComponents, "react/only-export-components", &[Language::Jsx, Language::Tsx], &["program"], |n, src, out| {
    let mut comps = 0;
    let mut others = Vec::new();
    for s in named_children(n) {
        if s.kind() != "export_statement" || text(s, src).starts_with("export type") || text(s, src).starts_with("export interface") {
            continue;
        }
        for (name, f, node) in export_names(s, src) {
            if is_component_name(&name) && f {
                comps += 1;
            } else if !(name.chars().all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())) {
                others.push((name, node));
            }
        }
    }
    if comps == 0 {
        return;
    }
    for (name, node) in others {
        out.push(finding(node, format!("`{name}` is exported beside components: Fast Refresh then reloads the whole page; move it to its own module"), vec![]));
    }
});

rule!(PreferFunctionComponent, "react/prefer-function-component", JS, &["class_declaration", "class"], |n, src, out| {
    if crate::react::is_component_class(n, src) {
        let body = n.child_by_field_name("body").map(|b| text(b, src)).unwrap_or("");
        if body.contains("componentDidCatch") || body.contains("getDerivedStateFromError") {
            return; // error boundaries have no function form
        }
        let at = n.child_by_field_name("name").unwrap_or(n);
        out.push(finding(at, "Write this component as a function: class components are React's legacy form", vec![]));
    }
});

/// A JSX attribute on a component whose value is an expression of `kind`.
fn component_prop_values<'t>(attr: Node<'t>, src: &str) -> Option<Node<'t>> {
    let el = attr.parent()?;
    let name = el.child_by_field_name("name").map(|x| text(x, src))?;
    if !is_component_name(name.rsplit('.').next().unwrap_or(name)) {
        return None;
    }
    let v = named_children(attr).into_iter().find(|c| c.kind() == "jsx_expression")?;
    v.named_child(0)
}

rule!(JsxNoJsxAsProp, "react/jsx-no-jsx-as-prop", JSX, &["jsx_attribute"], |n, src, out| {
    if let Some(v) = component_prop_values(n, src)
        && matches!(v.kind(), "jsx_element" | "jsx_self_closing_element" | "jsx_fragment")
    {
        out.push(finding(v, "A JSX element built in a prop is new on every render, so the child always re-renders: hoist it or memoise it", vec![]));
    }
});

rule!(JsxNoNewArrayAsProp, "react/jsx-no-new-array-as-prop", JSX, &["jsx_attribute"], |n, src, out| {
    if let Some(v) = component_prop_values(n, src)
        && v.kind() == "array"
    {
        out.push(finding(v, "An array literal in a prop is new on every render, so the child always re-renders: hoist it or memoise it", vec![]));
    }
});

rule!(JsxNoNewObjectAsProp, "react/jsx-no-new-object-as-prop", JSX, &["jsx_attribute"], |n, src, out| {
    if let Some(v) = component_prop_values(n, src)
        && v.kind() == "object"
    {
        out.push(finding(v, "An object literal in a prop is new on every render, so the child always re-renders: hoist it or memoise it", vec![]));
    }
});

fn block_stmts(n: Node<'_>) -> Option<Vec<Node<'_>>> {
    (n.kind() == "statement_block").then(|| named_children(n).into_iter().filter(|c| c.kind() != "comment").collect())
}

rule!(BranchesSharingCode, "js/branches-sharing-code", JS, &["if_statement"], |n, src, out| {
    let (Some(cons), Some(alt)) = (n.child_by_field_name("consequence"), n.child_by_field_name("alternative")) else { return };
    let Some(alt_body) = alt.named_child(0).filter(|b| b.kind() == "statement_block") else { return };
    let (Some(a), Some(b)) = (block_stmts(cons), block_stmts(alt_body)) else { return };
    if a.len() < 2 || b.len() < 2 {
        return;
    }
    let same = |x: Node<'_>, y: Node<'_>| text(x, src) == text(y, src);
    let indent = |x: Node<'_>| {
        let ls = src[..x.start_byte()].rfind('\n').map(|k| k + 1).unwrap_or(0);
        src[ls..x.start_byte()].to_owned()
    };
    let line_of = |x: Node<'_>| {
        let ls = src[..x.start_byte()].rfind('\n').map(|k| k + 1).unwrap_or(x.start_byte());
        let le = src[x.end_byte()..].find('\n').map(|k| x.end_byte() + k + 1).unwrap_or(x.end_byte());
        (ls, le)
    };
    if same(a[0], b[0]) {
        let (s1, e1) = line_of(a[0]);
        let (s2, e2) = line_of(b[0]);
        let ind = indent(n);
        let edits = vec![TextEdit::insert(n.start_byte(), format!("{}\n{ind}", text(a[0], src))), TextEdit::delete(s1, e1), TextEdit::delete(s2, e2)];
        out.push(finding(b[0], format!("Both branches start with `{}`: move it before the `if`", text(a[0], src)), edits));
    } else if same(a[a.len() - 1], b[b.len() - 1]) {
        let (x, y) = (a[a.len() - 1], b[b.len() - 1]);
        if matches!(x.kind(), "return_statement" | "throw_statement" | "break_statement" | "continue_statement") {
            return;
        }
        let (s1, e1) = line_of(x);
        let (s2, e2) = line_of(y);
        let ind = indent(n);
        let edits = vec![TextEdit::delete(s1, e1), TextEdit::delete(s2, e2), TextEdit::insert(n.end_byte(), format!("\n{ind}{}", text(x, src)))];
        out.push(finding(y, format!("Both branches end with `{}`: move it after the `if`", text(x, src)), edits));
    }
});

fn math_call<'t>(n: Node<'t>, src: &str) -> Option<(&'static str, Vec<Node<'t>>)> {
    let f = match callee(n, src)? {
        "Math.min" => "min",
        "Math.max" => "max",
        _ => return None,
    };
    Some((f, named_children(n.child_by_field_name("arguments")?)))
}

fn number(n: Node<'_>, src: &str) -> Option<f64> {
    match n.kind() {
        "number" => text(n, src).replace('_', "").parse().ok(),
        "unary_expression" if text(n, src).starts_with('-') => n.child_by_field_name("argument").and_then(|a| number(a, src)).map(|v| -v),
        _ => None,
    }
}

rule!(NoConstantMathMinMaxClamp, "js/no-constant-math-min-max-clamp", JS, &["call_expression"], |n, src, out| {
    let Some((outer, args)) = math_call(n, src) else { return };
    let [x, y] = args.as_slice() else { return };
    let (lit, inner) = if number(*x, src).is_some() { (*x, *y) } else { (*y, *x) };
    let Some(a) = number(lit, src) else { return };
    let Some((inner_f, inner_args)) = math_call(inner, src) else { return };
    if inner_f == outer {
        return;
    }
    let [p, q] = inner_args.as_slice() else { return };
    let ilit = if number(*p, src).is_some() { *p } else { *q };
    let Some(b) = number(ilit, src) else { return };
    let constant = if outer == "min" { a < b } else { a > b };
    if constant {
        let edits = vec![TextEdit::new(lit.start_byte(), lit.end_byte(), text(ilit, src)), TextEdit::new(ilit.start_byte(), ilit.end_byte(), text(lit, src))];
        out.push(finding(n, format!("This clamp always gives {}: the bounds are swapped", text(lit, src)), edits));
    }
});

pub static RULES: &[&dyn NativeRule] = &[
    &NoCloneElement, &NoReactChildren, &OnlyExportComponents, &PreferFunctionComponent, &JsxNoJsxAsProp, &JsxNoNewArrayAsProp, &JsxNoNewObjectAsProp, &BranchesSharingCode,
    &NoConstantMathMinMaxClamp,
];

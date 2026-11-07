//! Class rules a pattern can't express: whether a derived constructor
//! calls `super()` (`js/constructor-super`), whether `this` comes before it
//! (`js/no-this-before-super`), and private members nothing reads
//! (`js/no-unused-private-class-members`).
//!
//! The `super()` checks follow straight-line order through the
//! constructor's top-level statements; a `super()` reached only on some
//! branches counts as called.

use crate::util::{children, descendants, is_function, named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use std::collections::BTreeMap;
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];
const CLASSES: &[&str] = &["class_declaration", "class", "abstract_class_declaration"];

/// Whether the class extends something other than `null`.
fn derived(class: Node<'_>, src: &str) -> bool {
    children(class).into_iter().any(|c| {
        c.kind() == "class_heritage" && {
            let t = text(c, src);
            let rest = t.trim_start_matches("extends").trim();
            !rest.starts_with("null") && t.contains("extends")
        }
    })
}

fn constructor<'t>(class: Node<'t>, src: &str) -> Option<Node<'t>> {
    let body = class.child_by_field_name("body")?;
    named_children(body).into_iter().find(|m| {
        m.kind() == "method_definition" && m.child_by_field_name("name").is_some_and(|n| text(n, src) == "constructor") && m.child_by_field_name("body").is_some()
    })
}

/// Nodes inside `n`, not entering nested non-arrow functions or classes
/// (an arrow function shares `this` and `super`).
fn same_this<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    descendants(n, &|c| is_function(c.kind()), &mut out);
    out
}

fn is_super_call(n: &Node<'_>) -> bool {
    n.kind() == "call_expression" && n.child_by_field_name("function").is_some_and(|f| f.kind() == "super")
}

pub struct ConstructorSuper;

impl NativeRule for ConstructorSuper {
    fn id(&self) -> &'static str {
        "js/constructor-super"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        CLASSES
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let class = node.get_inner_node();
        let Some(ctor) = constructor(class, ctx.source) else { return };
        let body = ctor.child_by_field_name("body").expect("checked");
        let calls: Vec<Node> = same_this(body).into_iter().filter(is_super_call).collect();
        let name = ctor.child_by_field_name("name").expect("checked");
        if derived(class, ctx.source) && calls.is_empty() {
            // `super(...arguments)` is what an implicit constructor calls.
            let at = body.start_byte() + 1;
            let pad = format!("{}  ", crate::python_util::indent_at(ctx.source, ctor.start_byte()));
            let edits = crate::choice_edits(vec![
                crate::choice("pass the arguments", true, vec![TextEdit::insert(at, format!("\n{pad}super(...arguments);"))]),
                crate::choice("no arguments", false, vec![TextEdit::insert(at, format!("\n{pad}super();"))]),
            ]);
            out.push(NativeFinding { start: name.start_byte(), end: name.end_byte(), message: "A derived class's constructor must call `super()`".into(), edits });
        }
        if !derived(class, ctx.source) {
            for c in calls {
                out.push(NativeFinding { start: c.start_byte(), end: c.end_byte(), message: "`super()` in a class that extends nothing".into(), edits: Vec::new() });
            }
        }
    }
}

/// The edits that move the constructor's `super(…);` statement above the
/// first statement using `this`, when both are statements of its body and
/// the call's arguments read neither `this` nor a name declared between.
fn super_first(body: Node<'_>, inside: &[Node<'_>], src: &str) -> Option<Vec<TextEdit>> {
    let call = inside.iter().find(|n| is_super_call(n))?;
    let stmt = call.parent().filter(|p| p.kind() == "expression_statement" && p.parent().is_some_and(|b| b.id() == body.id()))?;
    let first_this = inside.iter().find(|n| matches!(n.kind(), "this" | "super") && n.start_byte() < call.start_byte() && !(n.kind() == "super" && n.parent().is_some_and(|p| p.id() == call.id())))?;
    let mut top = *first_this;
    while top.parent()?.id() != body.id() {
        top = top.parent()?;
    }
    let args = text(call.child_by_field_name("arguments")?, src);
    if args.contains("this") {
        return None;
    }
    let between = &src[top.start_byte()..stmt.start_byte()];
    let declares = ["let ", "const ", "var ", "function ", "class "];
    if declares.iter().any(|d| between.contains(d)) {
        return None;
    }
    let pad = crate::python_util::indent_at(src, top.start_byte());
    let (a, b) = crate::util::whole_lines(src, stmt.start_byte(), stmt.end_byte());
    Some(vec![TextEdit::insert(top.start_byte(), format!("{}\n{pad}", text(stmt, src))), TextEdit::delete(a, b)])
}

pub struct NoThisBeforeSuper;

impl NativeRule for NoThisBeforeSuper {
    fn id(&self) -> &'static str {
        "js/no-this-before-super"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        CLASSES
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let class = node.get_inner_node();
        if !derived(class, ctx.source) {
            return;
        }
        let Some(ctor) = constructor(class, ctx.source) else { return };
        let body = ctor.child_by_field_name("body").expect("checked");
        // An arrow function shares `this` but runs later, so its body is not
        // in the constructor's order (ESLint follows the same paths).
        let mut inside = Vec::new();
        descendants(body, &|c| is_function(c.kind()) || c.kind() == "arrow_function", &mut inside);
        // The first `super(...)` call ends where `this` becomes legal; `this`
        // inside its own arguments is still too early.
        let first_super = inside.iter().filter(|n| is_super_call(n)).map(|n| n.end_byte()).min();
        let mut moved = super_first(body, &inside, ctx.source);
        for n in inside {
            let too_early = first_super.is_none_or(|s| n.start_byte() < s);
            let is_this = n.kind() == "this";
            let is_super_member = n.kind() == "super" && n.parent().is_some_and(|p| p.kind() == "member_expression");
            if too_early && (is_this || is_super_member) {
                let what = if is_this { "`this`" } else { "`super`" };
                out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: format!("{what} is used before `super()` is called"), edits: moved.take().unwrap_or_default() });
            }
        }
    }
}

/// The edit removing a class member whose declaration has no effect: a
/// method, or a field with no value or a pure one, with no decorator. A
/// member alone on its line takes the line.
fn member_removal(m: Node<'_>, src: &str) -> Option<TextEdit> {
    let field = matches!(m.kind(), "field_definition" | "public_field_definition");
    if !field && m.kind() != "method_definition" {
        return None;
    }
    if named_children(m).iter().any(|c| c.kind() == "decorator") || (field && m.child_by_field_name("value").is_some_and(|v| !crate::unused_vars::pure(v))) {
        return None;
    }
    let (mut s, mut e) = (m.start_byte(), m.end_byte());
    if let Some(semi) = m.next_sibling().filter(|n| n.kind() == ";" && n.start_byte() == e) {
        e = semi.end_byte();
    }
    let line_start = src[..s].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = src[e..].find('\n').map(|i| e + i + 1).unwrap_or(src.len());
    if src[line_start..s].trim().is_empty() && src[e..line_end].trim().is_empty() {
        (s, e) = (line_start, line_end);
    } else {
        e += src[e..].bytes().take_while(|b| *b == b' ').count();
    }
    Some(TextEdit::delete(s, e))
}

pub struct NoUnusedPrivateClassMembers;

impl NativeRule for NoUnusedPrivateClassMembers {
    fn id(&self) -> &'static str {
        "js/no-unused-private-class-members"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["class_body"]
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let body = node.get_inner_node();
        if crate::util::in_ambient(body) {
            return;
        }
        let mut declared: BTreeMap<String, Node> = BTreeMap::new();
        for m in named_children(body) {
            let key = match m.kind() {
                "field_definition" | "public_field_definition" => m.child_by_field_name("property").or_else(|| m.child_by_field_name("name")),
                "method_definition" => m.child_by_field_name("name"),
                _ => None,
            };
            if let Some(k) = key.filter(|k| k.kind() == "private_property_identifier") {
                declared.entry(text(k, ctx.source).to_owned()).or_insert(k);
            }
        }
        if declared.is_empty() {
            return;
        }
        let mut all = Vec::new();
        descendants(body, &|_| false, &mut all);
        let mut read: BTreeMap<String, ()> = BTreeMap::new();
        let mut used: BTreeMap<String, ()> = BTreeMap::new();
        for n in all.into_iter().filter(|n| n.kind() == "private_property_identifier") {
            if declared.values().any(|d| d.id() == n.id()) {
                continue;
            }
            used.insert(text(n, ctx.source).to_owned(), ());
            // `this.#x = v` writes; everything else reads.
            let write_only = n.parent().is_some_and(|member| {
                member.kind() == "member_expression"
                    && member.parent().is_some_and(|a| a.kind() == "assignment_expression" && a.child_by_field_name("left").is_some_and(|l| l.id() == member.id()))
            });
            if !write_only {
                read.insert(text(n, ctx.source).to_owned(), ());
            }
        }
        for (name, k) in declared {
            if !read.contains_key(&name) {
                // Nothing names it at all: the member goes, when declaring it
                // does nothing either (W-162, purity). One that is written
                // keeps its writes, whose values may matter: by hand.
                let edits = if used.contains_key(&name) { Vec::new() } else { k.parent().and_then(|m| member_removal(m, ctx.source)).into_iter().collect() };
                out.push(NativeFinding { start: k.start_byte(), end: k.end_byte(), message: format!("Private member `{name}` is never read"), edits });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(rule: &dyn NativeRule, src: &str) -> Vec<String> {
        crate::run(rule, Language::Javascript, src).into_iter().map(|f| format!("{} @{}", f.message, &src[f.start..f.end])).collect()
    }

    #[test]
    fn constructor_super() {
        assert_eq!(found(&ConstructorSuper, "class A extends B { constructor() { this.x = 1; } }"), ["A derived class's constructor must call `super()` @constructor"]);
        assert_eq!(found(&ConstructorSuper, "class A { constructor() { super(); } }"), ["`super()` in a class that extends nothing @super()"]);
        assert!(found(&ConstructorSuper, "class A extends B { constructor() { if (x) super(1); else super(2); } }").is_empty());
        assert!(found(&ConstructorSuper, "class A extends null { constructor() {} }").is_empty());
    }

    #[test]
    fn this_before_super() {
        assert_eq!(found(&NoThisBeforeSuper, "class A extends B { constructor() { this.a = 1; super(this.b); this.c = 2; } }"), [
            "`this` is used before `super()` is called @this",
            "`this` is used before `super()` is called @this"
        ]);
        assert!(found(&NoThisBeforeSuper, "class A extends B { constructor() { super(); const f = function () { return this; }; } }").is_empty());
        assert!(found(&NoThisBeforeSuper, "class A extends B { constructor() { super({ onDone: () => this.done() }); } }").is_empty());
    }

    #[test]
    fn private_members() {
        assert_eq!(
            found(&NoUnusedPrivateClassMembers, "class A { #a = 1; #b = 2; #c() {} m() { this.#b = 3; return this.#a + this.#c(); } }"),
            ["Private member `#b` is never read @#b"]
        );
        let fixed = |src: &str| crate::run(&NoUnusedPrivateClassMembers, Language::Javascript, src).into_iter().map(|f| f.edits.len()).collect::<Vec<_>>();
        assert_eq!(fixed("class A {\n  #a = [];\n  #b() {}\n  #c = load();\n  #d = 1;\n  m() { this.#d = 2; }\n}\n"), [1, 1, 0, 0]);
    }
}

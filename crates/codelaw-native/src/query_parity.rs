//! `tanstack-query/prefer-query-options` (W-139, `@tanstack/eslint-plugin-query`
//! 5.105.0): query options and query keys come from a `queryOptions()` /
//! `infiniteQueryOptions()` result, not an object or an array written at
//! the call.

use crate::js_ast::{arguments, as_function, body, declarator_of, import_of, member, member_value, root_of, unparen};
use crate::util::{named_children, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use codelaw_semantic::{Imported, SemanticModel};
use tree_sitter::Node;

const JS: &[Language] = &[Language::Javascript, Language::Jsx, Language::Typescript, Language::Tsx];

pub const RULES: &[&dyn NativeRule] = &[&PreferQueryOptions];

const QUERY_HOOKS: &[&str] = &["useQuery", "useInfiniteQuery", "useSuspenseQuery", "useSuspenseInfiniteQuery", "usePrefetchQuery", "usePrefetchInfiniteQuery"];
const QUERIES_HOOKS: &[&str] = &["useQueries", "useSuspenseQueries"];
const FILTER_HOOKS: &[&str] = &["useIsFetching"];
const CLIENT_OPTION_METHODS: &[&str] = &["fetchQuery", "prefetchQuery", "fetchInfiniteQuery", "prefetchInfiniteQuery", "ensureQueryData", "ensureInfiniteQueryData"];
const CLIENT_KEY_METHODS: &[&str] = &["getQueryData", "setQueryData", "getQueryState", "setQueryDefaults", "getQueryDefaults"];
const CLIENT_FILTER_METHODS: &[&str] = &["invalidateQueries", "cancelQueries", "refetchQueries", "removeQueries", "resetQueries", "isFetching", "getQueriesData", "setQueriesData"];
const BUILDERS: &[&str] = &["queryOptions", "infiniteQueryOptions"];

const OPTIONS_MESSAGE: &str = "Prefer using queryOptions() or infiniteQueryOptions() to co-locate queryKey and queryFn.";
const KEY_MESSAGE: &str = "Prefer referencing a queryKey from a queryOptions() result instead of typing it manually.";

fn is_tanstack(source: &str) -> bool {
    source.starts_with("@tanstack/") && source.ends_with("-query")
}

/// The name a TanStack Query package exports that an identifier imports.
fn imported_name(model: &SemanticModel, ident: Node<'_>) -> Option<String> {
    match &import_of(model, ident, &is_tanstack)?.imported {
        Imported::Named(n) => Some(n.clone()),
        _ => None,
    }
}

/// The object literal a value is, looking through type assertions.
fn object_of(n: Node<'_>) -> Option<Node<'_>> {
    let n = unparen(n);
    (n.kind() == "object").then_some(n)
}

fn has_inline_options(obj: Node<'_>, src: &str) -> bool {
    member(obj, "queryKey", src).is_some() || member(obj, "queryFn", src).is_some()
}

fn is_inline_array(n: Node<'_>) -> bool {
    unparen(n).kind() == "array"
}

fn has_inline_filter_key(obj: Node<'_>, src: &str) -> bool {
    member(obj, "queryKey", src).and_then(member_value).is_some_and(is_inline_array)
}

/// The object literals a node can evaluate to, through function bodies,
/// `return`s and conditional, logical and sequence expressions.
fn returned_objects(n: Node<'_>) -> Vec<Node<'_>> {
    let n = unparen(n);
    match n.kind() {
        "object" => vec![n],
        "arrow_function" | "function_expression" | "function" => body(n).map(returned_objects).unwrap_or_default(),
        "statement_block" => named_children(n).into_iter().filter(|s| s.kind() == "return_statement").filter_map(|r| r.named_child(0)).flat_map(returned_objects).collect(),
        "ternary_expression" => ["consequence", "alternative"].iter().filter_map(|f| n.child_by_field_name(f)).flat_map(returned_objects).collect(),
        "binary_expression" if matches!(n.child_by_field_name("operator").map(|o| o.kind()), Some("&&" | "||" | "??")) => ["left", "right"].iter().filter_map(|f| n.child_by_field_name(f)).flat_map(returned_objects).collect(),
        "sequence_expression" => named_children(n).into_iter().flat_map(returned_objects).collect(),
        _ => Vec::new(),
    }
}

/// The query objects of a `queries` value: the objects of an array literal,
/// or those a `.map()` callback returns.
fn query_objects<'t>(n: Node<'t>, src: &str) -> Vec<Node<'t>> {
    let n = unparen(n);
    match n.kind() {
        "array" => named_children(n).into_iter().filter(|e| e.kind() == "object").collect(),
        "call_expression" => {
            let is_map = n.child_by_field_name("function").is_some_and(|f| f.kind() == "member_expression" && f.child_by_field_name("property").is_some_and(|p| text(p, src) == "map"));
            if !is_map {
                return Vec::new();
            }
            arguments(n).first().and_then(|a| as_function(*a)).map(returned_objects).unwrap_or_default()
        }
        _ => Vec::new(),
    }
}

/// Whether a node is a QueryClient: from `useQueryClient()` or `new QueryClient()`,
/// followed through `const` bindings and type assertions.
fn is_query_client(model: &SemanticModel, n: Node<'_>) -> bool {
    let root = root_of(n);
    let mut cur = unparen(n);
    let mut seen: Vec<usize> = Vec::new();
    loop {
        if seen.contains(&cur.id()) {
            return false;
        }
        seen.push(cur.id());
        if cur.kind() != "identifier" {
            break;
        }
        let next = declarator_of(model, root, cur).and_then(|d| d.child_by_field_name("value"));
        match next {
            Some(v) => cur = unparen(v),
            None => return false,
        }
    }
    match cur.kind() {
        "call_expression" => cur.child_by_field_name("function").filter(|f| f.kind() == "identifier").and_then(|f| imported_name(model, f)).is_some_and(|n| n == "useQueryClient"),
        "new_expression" => cur.child_by_field_name("constructor").filter(|f| f.kind() == "identifier").and_then(|f| imported_name(model, f)).is_some_and(|n| n == "QueryClient"),
        _ => false,
    }
}

pub struct PreferQueryOptions;

impl NativeRule for PreferQueryOptions {
    fn id(&self) -> &'static str {
        "tanstack-query/prefer-query-options"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["call_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let call = node.get_inner_node();
        let src = ctx.source;
        let Some(&options) = arguments(call).first() else { return };
        let Some(callee) = call.child_by_field_name("function") else { return };
        let report = |out: &mut Vec<NativeFinding>, n: Node<'_>, message: &str| out.push(NativeFinding { start: n.start_byte(), end: n.end_byte(), message: message.to_owned(), edits: vec![] });
        let options_object = |out: &mut Vec<NativeFinding>, n: Node<'_>| {
            if let Some(o) = object_of(n).filter(|o| has_inline_options(*o, src)) {
                report(out, o, OPTIONS_MESSAGE);
            }
        };
        let filter_key = |out: &mut Vec<NativeFinding>, n: Node<'_>| {
            if let Some(o) = object_of(n).filter(|o| has_inline_filter_key(*o, src)) {
                report(out, o, KEY_MESSAGE);
            }
        };
        if callee.kind() == "identifier" {
            let Some(name) = imported_name(model, callee) else { return };
            let name = name.as_str();
            if BUILDERS.contains(&name) {
                return;
            }
            if QUERY_HOOKS.contains(&name) {
                options_object(out, options);
            } else if QUERIES_HOOKS.contains(&name) {
                if let Some(obj) = object_of(options)
                    && let Some(queries) = member(obj, "queries", src).and_then(member_value)
                {
                    for q in query_objects(queries, src) {
                        options_object(out, q);
                    }
                }
            } else if FILTER_HOOKS.contains(&name) {
                filter_key(out, options);
            }
            return;
        }
        if callee.kind() != "member_expression" {
            return;
        }
        let (Some(object), Some(property)) = (callee.child_by_field_name("object"), callee.child_by_field_name("property")) else { return };
        if property.kind() != "property_identifier" || !is_query_client(model, object) {
            return;
        }
        let method = text(property, src);
        if CLIENT_OPTION_METHODS.contains(&method) {
            options_object(out, options);
        } else if CLIENT_KEY_METHODS.contains(&method) {
            if is_inline_array(options) {
                report(out, unparen(options), KEY_MESSAGE);
            }
        } else if CLIENT_FILTER_METHODS.contains(&method) {
            filter_key(out, options);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;
    use codelaw_core::Language::Tsx;

    fn messages(src: &str) -> Vec<String> {
        run(&PreferQueryOptions, Tsx, src).into_iter().map(|f| f.message).collect()
    }

    #[test]
    fn inline_options_in_hooks() {
        let src = "import { useQuery, queryOptions } from '@tanstack/react-query';\nuseQuery({ queryKey: ['a'], queryFn });\nuseQuery(queryOptions({ queryKey: ['a'], queryFn }));\nuseQuery({ ...opts, enabled });\n";
        assert_eq!(messages(src), vec![OPTIONS_MESSAGE.to_owned()]);
    }

    #[test]
    fn queries_map_and_array() {
        let src = "import { useQueries } from '@tanstack/react-query';\nuseQueries({ queries: ids.map((id) => ({ queryKey: ['a', id], queryFn })) });\nuseQueries({ queries: [{ queryKey: ['b'] }, opts] });\n";
        assert_eq!(messages(src).len(), 2);
    }

    #[test]
    fn query_client_methods() {
        let src = "import { useQueryClient, QueryClient } from '@tanstack/react-query';\nconst qc = useQueryClient();\nconst c2 = new QueryClient();\nqc.fetchQuery({ queryKey: ['a'], queryFn });\nqc.getQueryData(['a']);\nc2.invalidateQueries({ queryKey: ['a'] });\nqc.invalidateQueries({ queryKey: todoKeys.all });\nother.getQueryData(['a']);\n";
        assert_eq!(messages(src).len(), 3);
    }

    #[test]
    fn not_a_tanstack_import() {
        assert!(messages("import { useQuery } from './mine';\nuseQuery({ queryKey: ['a'] });\n").is_empty());
    }
}

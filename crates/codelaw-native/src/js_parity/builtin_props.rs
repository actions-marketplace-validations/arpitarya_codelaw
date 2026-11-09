//! W-139 strand W-139-js: eslint-plugin-unicorn v79.0.0's
//! `no-nonstandard-builtin-properties`, read from the rule's source and
//! tests and written again. A property read, called or built on a
//! built-in namespace or constructor (`Object`, `Math`, `Number`, `JSON`,
//! `Reflect`, `Promise`, `Array`, `String`, `Symbol`, `Date`, `BigInt`,
//! `Map`, `Iterator`), on its `prototype`, or on a string or array
//! literal, is checked against the names the language defines. Upstream's
//! list also covers the typed arrays, `Intl`, `Temporal`, `URL` and more;
//! only the namespaces whose whole surface is listed here are checked.

use super::values::is_global;
use super::{JS, finding};
use crate::util::{named_children, nearest, text};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use tree_sitter::Node;

/// What a function value has beyond its own statics.
const FUNCTION_MEMBERS: &[&str] = &["length", "name", "prototype", "apply", "bind", "call", "toString", "arguments", "caller"];
const OBJECT_PROTO: &[&str] = &["constructor", "hasOwnProperty", "isPrototypeOf", "propertyIsEnumerable", "toLocaleString", "toString", "valueOf", "__proto__", "__defineGetter__", "__defineSetter__", "__lookupGetter__", "__lookupSetter__"];

struct Surface {
    /// Statics, callable.
    methods: &'static [&'static str],
    /// Statics that are values, not functions.
    constants: &'static [&'static str],
    /// Whether the namespace is itself a function (a constructor).
    function: bool,
    /// `X.prototype`'s members, when listed.
    proto: Option<&'static [&'static str]>,
}

const ARRAY_PROTO: &[&str] = &[
    "length", "at", "concat", "copyWithin", "entries", "every", "fill", "filter", "find", "findIndex", "findLast", "findLastIndex", "flat", "flatMap", "forEach", "includes", "indexOf", "join", "keys", "lastIndexOf", "map", "pop", "push", "reduce", "reduceRight", "reverse", "shift", "slice", "some", "sort", "splice", "toLocaleString", "toReversed", "toSorted", "toSpliced", "toString", "unshift", "values", "with",
];
const STRING_PROTO: &[&str] = &[
    "length", "at", "charAt", "charCodeAt", "codePointAt", "concat", "endsWith", "includes", "indexOf", "isWellFormed", "lastIndexOf", "localeCompare", "match", "matchAll", "normalize", "padEnd", "padStart", "repeat", "replace", "replaceAll", "search", "slice", "split", "startsWith", "substring", "substr", "toLocaleLowerCase", "toLocaleUpperCase", "toLowerCase", "toString", "toUpperCase", "toWellFormed", "trim", "trimEnd", "trimLeft", "trimRight", "trimStart", "valueOf", "anchor", "big", "blink", "bold", "fixed", "fontcolor", "fontsize", "italics", "link", "small", "strike", "sub", "sup",
];

fn surface(name: &str) -> Option<Surface> {
    Some(match name {
        "Object" => Surface { methods: &["assign", "create", "defineProperties", "defineProperty", "entries", "freeze", "fromEntries", "getOwnPropertyDescriptor", "getOwnPropertyDescriptors", "getOwnPropertyNames", "getOwnPropertySymbols", "getPrototypeOf", "groupBy", "hasOwn", "is", "isExtensible", "isFrozen", "isSealed", "keys", "preventExtensions", "seal", "setPrototypeOf", "values"], constants: &[], function: true, proto: Some(OBJECT_PROTO) },
        "Math" => Surface { methods: &["abs", "acos", "acosh", "asin", "asinh", "atan", "atan2", "atanh", "cbrt", "ceil", "clz32", "cos", "cosh", "exp", "expm1", "floor", "f16round", "fround", "hypot", "imul", "log", "log10", "log1p", "log2", "max", "min", "pow", "random", "round", "sign", "sin", "sinh", "sqrt", "sumPrecise", "tan", "tanh", "trunc"], constants: &["E", "LN10", "LN2", "LOG10E", "LOG2E", "PI", "SQRT1_2", "SQRT2"], function: false, proto: None },
        "Number" => Surface { methods: &["isFinite", "isInteger", "isNaN", "isSafeInteger", "parseFloat", "parseInt"], constants: &["EPSILON", "MAX_SAFE_INTEGER", "MAX_VALUE", "MIN_SAFE_INTEGER", "MIN_VALUE", "NaN", "NEGATIVE_INFINITY", "POSITIVE_INFINITY"], function: true, proto: None },
        "JSON" => Surface { methods: &["isRawJSON", "parse", "rawJSON", "stringify"], constants: &[], function: false, proto: None },
        "Reflect" => Surface { methods: &["apply", "construct", "defineProperty", "deleteProperty", "get", "getOwnPropertyDescriptor", "getPrototypeOf", "has", "isExtensible", "ownKeys", "preventExtensions", "set", "setPrototypeOf"], constants: &[], function: false, proto: None },
        "Promise" => Surface { methods: &["all", "allSettled", "any", "race", "reject", "resolve", "try", "withResolvers"], constants: &[], function: true, proto: Some(&["catch", "finally", "then", "constructor"]) },
        "Array" => Surface { methods: &["from", "fromAsync", "isArray", "of"], constants: &[], function: true, proto: Some(ARRAY_PROTO) },
        "String" => Surface { methods: &["fromCharCode", "fromCodePoint", "raw"], constants: &[], function: true, proto: Some(STRING_PROTO) },
        "Symbol" => Surface { methods: &["for", "keyFor"], constants: &["asyncDispose", "asyncIterator", "dispose", "hasInstance", "isConcatSpreadable", "iterator", "match", "matchAll", "replace", "search", "species", "split", "toPrimitive", "toStringTag", "unscopables"], function: true, proto: None },
        "Date" => Surface { methods: &["now", "parse", "UTC"], constants: &[], function: true, proto: None },
        "BigInt" => Surface { methods: &["asIntN", "asUintN"], constants: &[], function: true, proto: None },
        "Map" => Surface { methods: &["groupBy"], constants: &[], function: true, proto: Some(&["clear", "delete", "entries", "forEach", "get", "getOrInsert", "getOrInsertComputed", "has", "keys", "set", "size", "values", "constructor"]) },
        "Iterator" => Surface { methods: &["concat", "from", "zip", "zipKeyed"], constants: &[], function: true, proto: None },
        _ => return None,
    })
}

/// The name after `.`, or a plain string key.
fn prop_name<'t, 's>(m: Node<'t>, src: &'s str) -> Option<(Node<'t>, &'s str)> {
    let pr = m.child_by_field_name("property")?;
    (pr.kind() == "property_identifier").then(|| (pr, text(pr, src)))
}

fn global_name<'s>(model: &codelaw_semantic::SemanticModel, n: Node<'_>, src: &'s str) -> Option<&'s str> {
    // `globalThis.Object` and `window.Object` are the global.
    let n = crate::js_ast::unparen(n);
    if n.kind() == "identifier" {
        let t = text(n, src);
        return is_global(model, n, t, src).then_some(t);
    }
    if n.kind() == "member_expression"
        && let Some(o) = n.child_by_field_name("object")
        && o.kind() == "identifier"
        && matches!(text(o, src), "globalThis" | "window" | "self" | "global")
        && is_global(model, o, text(o, src), src)
        && let Some((_, name)) = prop_name(n, src)
    {
        return Some(name);
    }
    None
}

enum Hit {
    Unknown,
    Called,
    Built,
}

pub struct NoNonstandardBuiltinProperties;
impl NativeRule for NoNonstandardBuiltinProperties {
    fn id(&self) -> &'static str {
        "js/no-nonstandard-builtin-properties"
    }
    fn languages(&self) -> &'static [Language] {
        JS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["member_expression"]
    }
    fn needs_semantic(&self) -> bool {
        true
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let Some(model) = ctx.semantic else { return };
        let n = node.get_inner_node();
        let src = ctx.source;
        let Some((prop, name)) = prop_name(n, src) else { return };
        let Some(object) = n.child_by_field_name("object") else { return };
        let obj = crate::js_ast::unparen(object);
        // The members the object is known to have, and whether it is a value (not callable).
        let known: Vec<&str>;
        let mut constants: &[&str] = &[];
        let mut is_namespace = false;
        if let Some(g) = global_name(model, obj, src)
            && let Some(s) = surface(g)
        {
            let mut k: Vec<&str> = s.methods.to_vec();
            k.extend_from_slice(s.constants);
            if s.function {
                k.extend_from_slice(FUNCTION_MEMBERS);
                k.extend_from_slice(OBJECT_PROTO);
            } else {
                k.extend_from_slice(OBJECT_PROTO);
                k.push("prototype");
            }
            constants = s.constants;
            known = k;
            is_namespace = true;
        } else if obj.kind() == "member_expression"
            && prop_name(obj, src).is_some_and(|(_, p)| p == "prototype")
            && let Some(inner) = obj.child_by_field_name("object")
            && let Some(g) = global_name(model, inner, src)
            && let Some(s) = surface(g)
            && let Some(proto) = s.proto
        {
            let mut k = proto.to_vec();
            k.extend_from_slice(OBJECT_PROTO);
            known = k;
        } else if obj.kind() == "string" || obj.kind() == "template_string" && !named_children(obj).iter().any(|c| c.kind() == "template_substitution") {
            let mut k = STRING_PROTO.to_vec();
            k.extend_from_slice(OBJECT_PROTO);
            known = k;
        } else if obj.kind() == "array" {
            let mut k = ARRAY_PROTO.to_vec();
            k.extend_from_slice(OBJECT_PROTO);
            known = k;
        } else {
            return;
        }
        let parent = n.parent();
        let called = parent.is_some_and(|p| p.kind() == "call_expression" && p.child_by_field_name("function").is_some_and(|f| f.id() == n.id()));
        let tagged = parent.is_some_and(|p| p.kind() == "call_expression" && p.child_by_field_name("arguments").is_some_and(|a| a.kind() == "template_string") && p.child_by_field_name("function").is_some_and(|f| f.id() == n.id()));
        let built = parent.is_some_and(|p| p.kind() == "new_expression" && p.child_by_field_name("constructor").is_some_and(|f| f.id() == n.id()));
        let hit = if !known.contains(&name) {
            Hit::Unknown
        } else if is_namespace && (called || tagged) && (constants.contains(&name) || matches!(name, "length" | "name" | "prototype")) {
            Hit::Called
        } else if is_namespace && built && !matches!(name, "prototype") && !known.is_empty() && is_not_constructor(name) {
            Hit::Built
        } else {
            return;
        };
        match hit {
            Hit::Unknown => {
                let mut edits = Vec::new();
                let near = nearest(name, known.iter().copied().filter(|k| !OBJECT_PROTO.contains(k) && !FUNCTION_MEMBERS.contains(k)), 2);
                if let Some(first) = near.first()
                    && near.get(1).is_none_or(|second| crate::util::edit_distance(&name.to_lowercase(), &first.to_lowercase()) < crate::util::edit_distance(&name.to_lowercase(), &second.to_lowercase()))
                {
                    edits.push(TextEdit::new(prop.start_byte(), prop.end_byte(), (*first).to_owned()));
                }
                out.push(finding(prop, format!("`{name}` is not a standard property here: a typo, or something added to a built-in at run time"), edits));
            }
            Hit::Called => out.push(finding(prop, format!("`{name}` is a value, not a function: it cannot be called"), vec![])),
            Hit::Built => out.push(finding(prop, format!("`{name}` is not a constructor: it cannot be used with `new`"), vec![])),
        }
    }
}

/// A static that is a function but not a constructor, or a plain value.
fn is_not_constructor(name: &str) -> bool {
    !matches!(name, "constructor" | "bind" | "call" | "apply" | "prototype")
}

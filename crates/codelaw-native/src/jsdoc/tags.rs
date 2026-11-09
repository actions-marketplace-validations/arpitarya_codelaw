//! The tag tables: the tag names the plugin knows in its default (TypeScript)
//! mode, their aliases, and what each tag may or must carry — a name, a type.
//! Written from the plugin's documented tag lists; the structure rows are the
//! plugin's defaults for that mode.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Unset,
    None,
    NamepathDefining,
    NamepathReferencing,
    NameDefining,
    Url,
    Text,
}

/// (tag, namepath role, name required, type allowed, type required, type or name required)
type Row = (&'static str, Role, Option<bool>, Option<bool>, Option<bool>, Option<bool>);

const STRUCTURE: &[Row] = &[
    ("alias", Role::NamepathDefining, None, None, None, Some(true)),
    ("arg", Role::NamepathDefining, Some(true), Some(true), None, None),
    ("argument", Role::NamepathDefining, Some(true), Some(true), None, None),
    ("augments", Role::NamepathReferencing, None, Some(true), None, Some(true)),
    ("borrows", Role::NamepathReferencing, None, None, None, Some(true)),
    ("callback", Role::NamepathDefining, Some(true), None, None, None),
    ("class", Role::NamepathDefining, None, Some(true), None, None),
    ("const", Role::NamepathDefining, None, Some(true), None, None),
    ("constant", Role::NamepathDefining, None, Some(true), None, None),
    ("constructor", Role::NamepathDefining, None, Some(true), None, None),
    ("constructs", Role::NamepathDefining, Some(false), Some(false), None, None),
    ("define", Role::Unset, None, None, Some(false), None),
    ("emits", Role::NamepathReferencing, Some(true), Some(false), None, None),
    ("enum", Role::NameDefining, None, Some(true), None, None),
    ("event", Role::NamepathDefining, Some(true), None, None, None),
    ("exception", Role::Unset, None, Some(true), None, None),
    ("export", Role::Unset, None, Some(false), None, None),
    ("exports", Role::NamepathDefining, Some(false), Some(false), None, None),
    ("extends", Role::NamepathReferencing, Some(false), Some(true), None, Some(true)),
    ("external", Role::NamepathDefining, Some(true), Some(false), None, None),
    ("fires", Role::NamepathReferencing, Some(true), Some(false), None, None),
    ("func", Role::NamepathDefining, None, None, None, None),
    ("function", Role::NamepathDefining, Some(false), Some(false), None, None),
    ("host", Role::NamepathDefining, Some(true), Some(false), None, None),
    ("implements", Role::Unset, None, None, Some(true), None),
    ("interface", Role::NamepathDefining, None, Some(false), None, None),
    ("internal", Role::None, None, None, None, None),
    ("lends", Role::NamepathReferencing, None, None, None, Some(true)),
    ("link", Role::Url, None, None, None, None),
    ("linkcode", Role::Url, None, None, None, None),
    ("linkplain", Role::Url, None, None, None, None),
    ("listens", Role::NamepathReferencing, Some(true), Some(false), None, None),
    ("member", Role::NamepathDefining, None, Some(true), None, None),
    ("memberof!", Role::NamepathReferencing, None, None, None, Some(true)),
    ("memberof", Role::NamepathReferencing, None, None, None, Some(true)),
    ("method", Role::NamepathDefining, None, None, None, None),
    ("mixes", Role::NamepathReferencing, None, None, None, Some(true)),
    ("mixin", Role::NamepathDefining, Some(false), Some(false), None, None),
    ("modifies", Role::Unset, None, Some(true), None, None),
    ("module", Role::Text, None, Some(true), None, None),
    ("name", Role::NamepathDefining, Some(true), None, None, Some(true)),
    ("namespace", Role::NamepathDefining, None, Some(true), None, None),
    ("package", Role::Unset, None, Some(false), None, None),
    ("param", Role::NamepathDefining, Some(true), Some(true), None, None),
    ("private", Role::Unset, None, Some(false), None, None),
    ("prop", Role::NamepathDefining, Some(true), Some(true), None, None),
    ("property", Role::NamepathDefining, Some(true), Some(true), None, None),
    ("protected", Role::Unset, None, Some(false), None, None),
    ("public", Role::Unset, None, Some(false), None, None),
    ("requires", Role::NamepathReferencing, Some(true), Some(false), None, None),
    ("return", Role::Unset, None, Some(true), None, None),
    ("returns", Role::Unset, None, Some(true), None, None),
    ("satisfies", Role::Unset, None, None, Some(true), None),
    ("see", Role::Text, None, None, None, None),
    ("static", Role::Unset, None, Some(false), None, None),
    ("template", Role::NamepathReferencing, Some(true), Some(true), None, None),
    ("this", Role::None, None, None, Some(true), Some(false)),
    ("throws", Role::Unset, None, Some(true), None, None),
    ("tutorial", Role::Unset, Some(true), Some(false), None, None),
    ("type", Role::Unset, None, None, Some(true), None),
    ("typedef", Role::NameDefining, Some(false), Some(true), None, Some(false)),
    ("var", Role::NamepathDefining, None, Some(true), None, None),
    ("yield", Role::Unset, None, Some(true), None, None),
    ("yields", Role::Unset, None, Some(true), None, None),
];

fn row(tag: &str) -> Option<&'static Row> {
    STRUCTURE.iter().find(|r| r.0 == tag)
}

pub fn role(tag: &str) -> Role {
    row(tag).map_or(Role::Unset, |r| r.1)
}

/// Whether the tag may carry a name (a tag the table does not know may).
pub fn might_have_name(tag: &str) -> bool {
    role(tag) != Role::None
}

pub fn might_have_type(tag: &str) -> bool {
    if must_have_type(tag) {
        return true;
    }
    row(tag).and_then(|r| r.3).unwrap_or(true)
}

pub fn must_have_name(tag: &str) -> bool {
    row(tag).and_then(|r| r.2) == Some(true)
}

pub fn must_have_type(tag: &str) -> bool {
    row(tag).and_then(|r| r.4) == Some(true)
}

pub fn type_or_name_required(tag: &str) -> bool {
    row(tag).and_then(|r| r.5) == Some(true)
}

pub fn might_have_name_or_namepath(tag: &str) -> bool {
    matches!(role(tag), Role::NamepathDefining | Role::NamepathReferencing | Role::NameDefining)
}

pub fn might_have_namepath(tag: &str) -> bool {
    matches!(role(tag), Role::NamepathDefining | Role::NamepathReferencing)
}

pub fn might_have_plain_name(tag: &str) -> bool {
    role(tag) == Role::NameDefining
}

/// Tag names and the aliases they stand for, in TypeScript mode: (alias, preferred).
pub const ALIASES: &[(&str, &str)] = &[
    ("virtual", "abstract"),
    ("extends", "augments"),
    ("constructor", "class"),
    ("const", "constant"),
    ("defaultvalue", "default"),
    ("desc", "description"),
    ("host", "external"),
    ("fileoverview", "file"),
    ("overview", "file"),
    ("emits", "fires"),
    ("func", "function"),
    ("method", "function"),
    ("var", "member"),
    ("arg", "param"),
    ("argument", "param"),
    ("prop", "property"),
    ("return", "returns"),
    ("exception", "throws"),
    ("yield", "yields"),
    ("typeParam", "template"),
];

/// The tags with no alias, valid in TypeScript mode (JSDoc's, TSDoc's, TypeDoc's, TypeScript's own).
const PLAIN: &[&str] = &[
    "modifies", "abstract", "access", "alias", "async", "augments", "author", "borrows", "callback", "class", "classdesc", "constant", "constructs", "copyright", "default", "deprecated", "description", "enum", "event", "example", "exports", "external",
    "file", "fires", "function", "generator", "global", "hideconstructor", "ignore", "implements", "inheritdoc", "inheritDoc", "inner", "instance", "interface", "kind", "lends", "license", "listens", "member", "memberof", "memberof!", "mixes", "mixin",
    "module", "name", "namespace", "override", "package", "param", "private", "property", "protected", "public", "readonly", "requires", "returns", "see", "since", "static", "summary", "this", "throws", "todo", "tutorial", "type", "typedef", "variation",
    "version", "yields", "alpha", "beta", "decorator", "eventProperty", "experimental", "packageDocumentation", "privateRemarks", "remarks", "sealed", "category", "categoryDescription", "disableGroups", "document", "expand", "expandType", "group",
    "groupDescription", "hidden", "hideCategories", "hideGroups", "inline", "inlineType", "mergeModuleWith", "preventExpand", "preventInline", "primaryExport", "showCategories", "showGroups", "sortStrategy", "useDeclaredType", "import", "internal",
    "jsx", "overload", "satisfies", "template",
];

pub fn is_known_tag(tag: &str) -> bool {
    PLAIN.contains(&tag) || ALIASES.iter().any(|(a, _)| *a == tag)
}

/// The preferred name of a tag: the canonical one when the name is an alias.
pub fn preferred(tag: &str) -> &str {
    ALIASES.iter().find(|(a, _)| *a == tag).map_or(tag, |(_, p)| p)
}

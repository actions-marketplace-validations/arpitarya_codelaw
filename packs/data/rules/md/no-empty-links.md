---
id: md/no-empty-links
title: "Links have a destination"
languages: [markdown]
fix: suggest
fix_reason: "unwrapping keeps the link's text but drops the link; the reviewer confirms that is the intent, or writes the destination instead"
stability: preview
ported_from: [markdownlint/MD042]
since: 0.1.0
---

A link has a destination: not `()`, `(<>)` or a bare `(#)`.

**Why:** An empty link looks clickable but goes nowhere, usually because a URL was never filled in.

```codelaw
match:
  kind: inline_link
  any:
    - pattern: "[$T]()"
    - pattern: "[$T](#)"
    - pattern: "[$T](<>)"
    - pattern: "[$T](<#>)"
    - not: { has: { kind: link_destination } }
    - has: { kind: link_destination, regex: '^(#|<>|<#>)$' }
message: "This link goes nowhere; give it a destination"
fix: "$T"
```

## How to fix

1. Find where the link should point.
2. Write that destination, or remove the link and keep its text.

## Examples

### Markdown

#### Don't

````md
See [the changelog]().
````

#### Fixed

````md
See the changelog.
````

#### Don't

````md
See [the changelog](#).
````

#### Fixed

````md
See the changelog.
````

#### Do

````md
See [the changelog](CHANGELOG.md).
````

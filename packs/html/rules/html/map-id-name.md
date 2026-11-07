---
id: html/map-id-name
title: "A map's id and name match"
languages: [html]
fix: suggest
fix_reason: "the choices are making `name` follow `id` or `id` follow `name`; a `usemap` that names the other one stops matching, which the reviewer checks"
stability: preview
ported_from: [html-validate/map-id-name]
since: 0.1.0
---

When a `<map>` has both `id` and `name`, they are the same.

**Why:** HTML requires them to match; tools that look maps up by `id` and browsers that use `name` would find different maps.

```codelaw
native: html/map-id-name
```

## How to fix

1. Make `id` and `name` equal, and update references to the one you changed.

## Examples

### HTML

#### Don't

```html
<map id="nav" name="menu"></map>
```

#### Fixed (name follows id)

```html
<map id="nav" name="nav"></map>
```

#### Fixed (id follows name)

```html
<map id="menu" name="menu"></map>
```

#### Do

```html
<map id="nav" name="nav"></map>
```

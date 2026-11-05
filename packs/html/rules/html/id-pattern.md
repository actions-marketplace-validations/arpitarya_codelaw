---
id: html/id-pattern
title: "Ids follow the project's pattern"
languages: [html]
fix: manual
fix_reason: "renaming the id must also change the links, labels, styles and scripts that use it"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*(-[a-z0-9]+)*$", description: "a regular expression ids must match" }
ported_from: [html-validate/id-pattern]
since: 0.5.0
---

An id matches `pattern`, kebab-case by default.

**Why:** Consistent ids are predictable.

```codelaw
native: html/id-pattern
```

## How to fix

1. Rename the id and its references.

## Examples

### HTML

#### Don't

```html
<div id="mainNav"></div>
```

#### Do

```html
<div id="main-nav"></div>
```

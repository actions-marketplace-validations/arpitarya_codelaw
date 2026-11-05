---
id: html/attr-pattern
title: "Attribute names follow the project's pattern"
languages: [html]
fix: manual
fix_reason: "renaming the attribute must also change what reads it"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", default: "^[a-z0-9-:]+$", description: "a regular expression attribute names must match" }
ported_from: [html-validate/attr-pattern]
since: 0.5.0
---

An attribute's name matches `pattern` (lowercase letters, digits, `-` and `:` by default).

**Why:** Consistent attribute names are easy to search for.

```codelaw
native: html/attr-pattern
```

## How to fix

1. Rename the attribute.

## Examples

### HTML

#### Don't

```html
<div data_value="1"></div>
```

#### Do

```html
<div data-value="1"></div>
```

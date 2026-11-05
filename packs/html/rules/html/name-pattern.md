---
id: html/name-pattern
title: "Form field names follow the project's pattern"
languages: [html]
fix: manual
fix_reason: "renaming a form field changes what the server receives"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*([-_][a-z0-9]+)*(\\[\\])?$", description: "a regular expression form field names must match" }
ported_from: [html-validate/name-pattern]
since: 0.5.0
---

The `name` of a form control matches `pattern` (lowercase, words joined by `-` or `_`, an optional `[]`).

**Why:** The server reads these names; one style avoids mismatches.

```codelaw
native: html/name-pattern
```

## How to fix

1. Rename the field, here and where the form is read.

## Examples

### HTML

#### Don't

```html
<input name="firstName">
```

#### Do

```html
<input name="first-name">
```

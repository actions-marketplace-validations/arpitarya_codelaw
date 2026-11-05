---
id: scss/function-disallowed-list
title: "No Sass function the project disallows"
languages: [scss]
fix: manual
fix_reason: "what the project uses instead is its own convention"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the function names disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint-scss/function-disallowed-list]
since: 0.5.0
---

A function named in `list` isn't called. With no option set, the rule does nothing.

**Why:** A project steers its authors to its own helpers.

```codelaw
native: scss/function-disallowed-list
```

## How to fix

1. Use the allowed function.

## Examples

### SCSS

#### Don't

```scss options={"list":["percentage"]}
a {
  width: percentage(0.5);
}
```

#### Do

```scss options={"list":["percentage"]}
a {
  width: 50%;
}
```

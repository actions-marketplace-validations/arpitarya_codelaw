---
id: css/rule-nesting-at-rule-required-list
title: "Nested rules sit inside the at-rules the project requires"
languages: [css, scss]
fix: manual
fix_reason: "which at-rule a nested rule belongs in is the author's call"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the at-rules a nested rule must sit in (`/re/` for a regular expression)" }
ported_from: [stylelint/rule-nesting-at-rule-required-list]
since: 0.5.0
---

A nested style rule is inside one of the at-rules named in `list` (such as `@media` or `@container`). With no option set, the rule does nothing.

**Why:** A project allows nesting only for conditions.

```codelaw
native: css/rule-nesting-at-rule-required-list
```

## How to fix

1. Move the rule into such an at-rule, or un-nest it.

## Examples

### CSS

#### Don't

```css options={"list":["media"]}
a {
  b {
    color: red;
  }
}
```

#### Do

```css options={"list":["media"]}
a b {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["media"]}
a {
  b {
    color: red;
  }
}
```

#### Do

```scss options={"list":["media"]}
a b {
  color: red;
}
```

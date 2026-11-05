---
id: css/comment-pattern
title: "Comments follow the project's pattern"
languages: [css, scss]
fix: manual
fix_reason: "the comment's wording is the author's to write"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", description: "a regular expression comment text must match" }
ported_from: [stylelint/comment-pattern]
since: 0.5.0
---

A comment's text matches the regular expression in `pattern`. With no option set, the rule does nothing.

**Why:** A project keeps markers (`TODO(name):`) greppable.

```codelaw
native: css/comment-pattern
```

## How to fix

1. Rewrite the comment to match.

## Examples

### CSS

#### Don't

```css options={"pattern":"^[A-Z]"}
/* lower case */
a {
  color: red;
}
```

#### Do

```css options={"pattern":"^[A-Z]"}
/* Upper case */
a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"pattern":"^[A-Z]"}
/* lower case */
a {
  color: red;
}
```

#### Do

```scss options={"pattern":"^[A-Z]"}
/* Upper case */
a {
  color: red;
}
```

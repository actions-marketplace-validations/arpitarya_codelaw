---
id: scss/double-slash-comment-inline
title: "`//` comments on their own line"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/double-slash-comment-inline]
since: 0.5.0
---

A `//` comment isn't written after code on the same line; it goes on the line above.

**Why:** A trailing comment is easily lost when the line changes.

```codelaw
native: scss/double-slash-comment-inline
```

## Examples

### SCSS

#### Don't

```scss
a {
  color: red; // brand
}
```

#### Do

```scss
a {
  // brand
  color: red;
}
```

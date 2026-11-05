---
id: scss/comment-no-loud
title: "Silent comments, not loud ones"
languages: [scss]
fix: suggest
fix_reason: "a `//` comment is not copied into the CSS; a comment meant for the output must stay `/* */`"
stability: preview
ported_from: [stylelint-scss/comment-no-loud]
since: 0.5.0
---

A comment in SCSS is written `//`, not `/* */` (which Sass copies into the CSS); `/*! … */` is kept.

**Why:** Notes for authors don't belong in the shipped stylesheet.

```codelaw
native: scss/comment-no-loud
```

## How to fix

1. Write the comment with `//`.

## Examples

### SCSS

#### Don't

```scss
/* note */
a {
  color: red;
}
```

#### Do

```scss
// note
a {
  color: red;
}
```

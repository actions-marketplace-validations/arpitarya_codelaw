---
id: scss/dimension-no-non-numeric-values
title: "No dimension built from interpolated text"
languages: [scss]
fix: suggest
fix_reason: "multiplying needs the variable to hold a number; one that holds text would now fail to compile"
stability: preview
ported_from: [stylelint-scss/dimension-no-non-numeric-values]
since: 0.5.0
---

`#{$x}px` is written `$x * 1px`.

**Why:** Interpolation builds a string that looks like a number.

```codelaw
native: scss/dimension-no-non-numeric-values
```

## How to fix

1. Multiply by one unit instead.

## Examples

### SCSS

#### Don't

```scss
a {
  width: #{$w}px;
}
```

#### Do

```scss
a {
  width: $w * 1px;
}
```

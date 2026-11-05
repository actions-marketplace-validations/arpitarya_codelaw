---
id: scss/function-calculation-no-interpolation
title: "No interpolation in CSS math functions"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/function-calculation-no-interpolation]
since: 0.5.0
---

Inside `calc()`, `min()`, `max()` and `clamp()`, a variable is written plainly: `calc($a + 1px)`, not `calc(#{$a} + 1px)`.

**Why:** Sass reads variables there directly.

```codelaw
native: scss/function-calculation-no-interpolation
```

## Examples

### SCSS

#### Don't

```scss
a {
  width: calc(#{$w} + 1px);
}
```

#### Do

```scss
a {
  width: calc($w + 1px);
}
```

---
id: scss/function-color-relative
title: "No legacy relative colour functions"
languages: [scss]
fix: suggest
fix_reason: "`color.adjust()` gives the same colour, but in a form the author should check"
stability: preview
ported_from: [stylelint-scss/function-color-relative]
since: 0.5.0
---

`lighten()`, `darken()`, `saturate()`, `transparentize()` and the like are written with `color.adjust()`.

**Why:** The legacy functions are deprecated in Dart Sass.

```codelaw
native: scss/function-color-relative
```

## How to fix

1. Use `color.adjust()` with the channel.

## Examples

### SCSS

#### Don't

```scss
@use "sass:color";
a {
  color: darken($c, 10%);
}
```

#### Do

```scss
@use "sass:color";
a {
  color: color.adjust($c, $lightness: -10%);
}
```

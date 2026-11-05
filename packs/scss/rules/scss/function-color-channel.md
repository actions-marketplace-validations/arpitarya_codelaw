---
id: scss/function-color-channel
title: "Colour channels are read with `color.channel()`"
languages: [scss]
fix: suggest
fix_reason: "`color.channel()` returns the channel in the space named, which can differ in unit from the legacy function"
stability: preview
ported_from: [stylelint-scss/function-color-channel]
since: 0.5.0
---

`red($c)`, `hue($c)` and the other channel functions are written `color.channel($c, "red", $space: rgb)`.

**Why:** The legacy functions are deprecated in Dart Sass.

```codelaw
native: scss/function-color-channel
```

## How to fix

1. Call `color.channel()` with the channel and its space.

## Examples

### SCSS

#### Don't

```scss
@use "sass:color";
a {
  width: red($c);
}
```

#### Do

```scss
@use "sass:color";
a {
  width: color.channel($c, "red", $space: rgb);
}
```

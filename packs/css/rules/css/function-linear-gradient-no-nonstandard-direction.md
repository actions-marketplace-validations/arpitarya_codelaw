---
id: css/function-linear-gradient-no-nonstandard-direction
title: "Gradient directions in the standard syntax"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/function-linear-gradient-no-nonstandard-direction]
since: 0.5.0
---

A `linear-gradient()` direction is written `to <side>`: the legacy `top` (the side it starts from) is `to bottom`.

**Why:** Browsers read the legacy form only with a prefix; unprefixed it drops the declaration.

```codelaw
native: css/function-linear-gradient-no-nonstandard-direction
```

## Examples

### CSS

#### Don't

```css
a {
  background: linear-gradient(top, red, blue);
}
```

#### Do

```css
a {
  background: linear-gradient(to bottom, red, blue);
}
```

### SCSS

#### Don't

```scss
a {
  background: linear-gradient(top, red, blue);
}
```

#### Do

```scss
a {
  background: linear-gradient(to bottom, red, blue);
}
```

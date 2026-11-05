---
id: css/color-no-hex
title: "No hex colours"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/color-no-hex]
since: 0.5.0
---

A colour is written with `rgb()`, not as a hex value.

**Why:** A project that writes colours as functions reads them one way.

```codelaw
native: css/color-no-hex
```

## Examples

### CSS

#### Don't

```css
a {
  color: #fff;
}
```

#### Do

```css
a {
  color: rgb(255 255 255);
}
```

### SCSS

#### Don't

```scss
a {
  color: #fff;
}
```

#### Do

```scss
a {
  color: rgb(255 255 255);
}
```

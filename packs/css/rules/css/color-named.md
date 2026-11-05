---
id: css/color-named
title: "No named colours"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/color-named]
since: 0.5.0
---

A colour is written as a hex value, not by name: `red` is `#ff0000`.

**Why:** Names hide the exact value, and some are surprising (`darkgray` is lighter than `gray`).

```codelaw
native: css/color-named
```

## Examples

### CSS

#### Don't

```css
a {
  color: red;
}
```

#### Do

```css
a {
  color: #ff0000;
}
```

### SCSS

#### Don't

```scss
a {
  color: red;
}
```

#### Do

```scss
a {
  color: #ff0000;
}
```

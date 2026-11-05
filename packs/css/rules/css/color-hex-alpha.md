---
id: css/color-hex-alpha
title: "Hex colours with or without an alpha channel, as set"
languages: [css, scss]
fix: auto
stability: preview
options:
  mode: { type: "string", default: "never", description: "`never` or `always`" }
ported_from: [stylelint/color-hex-alpha]
since: 0.5.0
---

With `mode: never` (the default), a hex colour has no alpha channel (`#0008`): the transparency is written with `rgb()`; with `always`, every hex colour has one.

**Why:** The transparency of `#ff000080` is hard to read.

```codelaw
native: css/color-hex-alpha
```

## Examples

### CSS

#### Don't

```css
a {
  color: #ff000080;
}
```

#### Do

```css
a {
  color: rgb(255 0 0 / 0.502);
}
```

### SCSS

#### Don't

```scss
a {
  color: #ff000080;
}
```

#### Do

```scss
a {
  color: rgb(255 0 0 / 0.502);
}
```

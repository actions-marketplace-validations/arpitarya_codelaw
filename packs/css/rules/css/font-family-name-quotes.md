---
id: css/font-family-name-quotes
title: "Font family names are quoted where they should be"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/font-family-name-quotes]
since: 0.5.0
---

A family name of several words is quoted, and a generic family keyword (`serif`, `sans-serif`) is not.

**Why:** A quoted keyword names a font called “serif”, not the generic family.

```codelaw
native: css/font-family-name-quotes
```

## Examples

### CSS

#### Don't

```css
a {
  font-family: Times New Roman, "serif";
}
```

#### Do

```css
a {
  font-family: "Times New Roman", serif;
}
```

### SCSS

#### Don't

```scss
a {
  font-family: Times New Roman, "serif";
}
```

#### Do

```scss
a {
  font-family: "Times New Roman", serif;
}
```

---
id: css/value-keyword-case
title: "Keywords in lower case"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/value-keyword-case]
since: 0.5.0
---

A keyword in a value (`block`, `none`, a colour name) is written in lower case.

**Why:** CSS keywords are case-insensitive; one case reads alike.

```codelaw
native: css/value-keyword-case
```

## Examples

### CSS

#### Don't

```css
a {
  display: BLOCK;
}
```

#### Do

```css
a {
  display: block;
}
```

### SCSS

#### Don't

```scss
a {
  display: BLOCK;
}
```

#### Do

```scss
a {
  display: block;
}
```

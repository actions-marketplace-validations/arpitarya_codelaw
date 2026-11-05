---
id: css/selector-type-case
title: "Type selectors in lower case"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/selector-type-case]
since: 0.5.0
---

A type selector is written in lower case; SVG's camel-case element names keep theirs.

**Why:** HTML element names are case-insensitive; one case reads alike.

```codelaw
native: css/selector-type-case
```

## Examples

### CSS

#### Don't

```css
A {
  color: red;
}
```

#### Do

```css
a {
  color: red;
}
```

### SCSS

#### Don't

```scss
A {
  color: red;
}
```

#### Do

```scss
a {
  color: red;
}
```

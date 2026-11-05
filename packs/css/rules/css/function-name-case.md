---
id: css/function-name-case
title: "Function names in lower case"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/function-name-case]
since: 0.5.0
---

A function name is written in lower case (`rgb()`, not `RGB()`); the camel-case transform functions keep their case.

**Why:** One case is easier to search for.

```codelaw
native: css/function-name-case
```

## Examples

### CSS

#### Don't

```css
a {
  color: RGB(0 0 0);
}
```

#### Do

```css
a {
  color: rgb(0 0 0);
}
```

### SCSS

#### Don't

```scss
a {
  color: RGB(0 0 0);
}
```

#### Do

```scss
a {
  color: rgb(0 0 0);
}
```

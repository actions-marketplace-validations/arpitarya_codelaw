---
id: scss/selector-no-redundant-nesting-selector
title: "No redundant `&`"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/selector-no-redundant-nesting-selector]
since: 0.5.0
---

A nested selector doesn't start with `& ` (a descendant of the parent is what nesting means): `& .b` is `.b`.

**Why:** The `&` adds nothing there.

```codelaw
native: scss/selector-no-redundant-nesting-selector
```

## Examples

### SCSS

#### Don't

```scss
a {
  & .b {
    color: red;
  }
}
```

#### Do

```scss
a {
  .b {
    color: red;
  }
}
```

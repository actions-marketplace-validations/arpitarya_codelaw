---
id: scss/media-feature-value-dollar-variable
title: "Media query values are variables"
languages: [scss]
fix: manual
fix_reason: "the variable's name is the author's to choose"
fix_exception: value
stability: preview
ported_from: [stylelint-scss/media-feature-value-dollar-variable]
since: 0.5.0
---

A media feature's value is a `$variable`, not a literal.

**Why:** Breakpoints live in one place.

```codelaw
native: scss/media-feature-value-dollar-variable
```

## How to fix

1. Declare a variable and use it.

## Examples

### SCSS

#### Don't

```scss
@media (min-width: 768px) {
  a {
    color: red;
  }
}
```

#### Do

```scss
@media (min-width: $md) {
  a {
    color: red;
  }
}
```

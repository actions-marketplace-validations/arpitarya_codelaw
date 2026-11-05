---
id: scss/selector-nest-combinators
title: "Combinators are written by nesting"
languages: [scss]
fix: suggest
fix_reason: "nesting rewrites the rule's layout; the edit is offered for review"
stability: preview
ported_from: [stylelint-scss/selector-nest-combinators]
since: 0.5.0
---

A selector of two compounds is written nested: `.a > .b {}` is `.a { > .b {} }`.

**Why:** The nesting shows the relation.

```codelaw
native: scss/selector-nest-combinators
```

## How to fix

1. Nest the second part.

## Examples

### SCSS

#### Don't

```scss
.a .b {
  color: red;
}
```

#### Do

```scss
.a {
  .b {
    color: red;
  }
}
```

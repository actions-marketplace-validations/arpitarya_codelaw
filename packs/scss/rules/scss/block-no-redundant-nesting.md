---
id: scss/block-no-redundant-nesting
title: "No block that only holds one nested block"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/block-no-redundant-nesting]
since: 0.5.0
---

`.a { .b { … } }` is `.a .b { … }`.

**Why:** The nesting adds a level and nothing else.

```codelaw
native: scss/block-no-redundant-nesting
```

## Examples

### SCSS

#### Don't

```scss
.a {
  .b {
    color: red;
  }
}
```

#### Do

```scss
.a .b {
    color: red;
  }
```

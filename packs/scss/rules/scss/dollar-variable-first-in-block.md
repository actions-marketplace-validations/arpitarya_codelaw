---
id: scss/dollar-variable-first-in-block
title: "Variables are declared first in their block"
languages: [scss]
fix: manual
fix_reason: "moving the declaration can change what it reads"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/dollar-variable-first-in-block]
since: 0.5.0
---

A block's `$variable` declarations come before its other statements.

**Why:** The inputs of a block are read first.

```codelaw
native: scss/dollar-variable-first-in-block
```

## How to fix

1. Move the declaration to the top of the block.

## Examples

### SCSS

#### Don't

```scss
a {
  color: red;
  $w: 1px;
}
```

#### Do

```scss
a {
  $w: 1px;
  color: red;
}
```

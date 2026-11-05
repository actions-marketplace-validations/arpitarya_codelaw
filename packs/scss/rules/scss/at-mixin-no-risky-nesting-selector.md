---
id: scss/at-mixin-no-risky-nesting-selector
title: "No `&` after another selector inside a mixin"
languages: [scss]
fix: manual
fix_reason: "what the mixin's rule should match depends on where it is included"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/at-mixin-no-risky-nesting-selector]
since: 0.5.0
---

Inside a `@mixin`, a nested rule's selector doesn't put `&` after another selector (`.dark &`).

**Why:** What it matches then depends on every place the mixin is included.

```codelaw
native: scss/at-mixin-no-risky-nesting-selector
```

## How to fix

1. Pass the context in as an argument, or move the rule out of the mixin.

## Examples

### SCSS

#### Don't

```scss
@mixin m {
  .dark & {
    color: red;
  }
}
```

#### Do

```scss
@mixin m {
  &.dark {
    color: red;
  }
}
```

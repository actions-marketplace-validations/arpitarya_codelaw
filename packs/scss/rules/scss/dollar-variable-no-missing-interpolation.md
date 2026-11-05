---
id: scss/dollar-variable-no-missing-interpolation
title: "Variables in at-rule names are interpolated"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/dollar-variable-no-missing-interpolation]
since: 0.5.0
---

A `$variable` in a `@keyframes`, `@counter-style` or `@supports` prelude is interpolated, `#{$name}`.

**Why:** Without it, Sass writes the variable's name, not its value.

```codelaw
native: scss/dollar-variable-no-missing-interpolation
```

## Examples

### SCSS

#### Don't

```scss
@keyframes $name {
  to {
    opacity: 0;
  }
}
```

#### Do

```scss
@keyframes #{$name} {
  to {
    opacity: 0;
  }
}
```

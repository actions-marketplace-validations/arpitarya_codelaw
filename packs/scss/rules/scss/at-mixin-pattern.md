---
id: scss/at-mixin-pattern
title: "Mixin names are kebab-case"
languages: [scss]
fix: auto
fix_reason: "auto when every file that includes the mixin reaches it through `@use`, `@forward` or `@import`: the rename then changes every `@include`, as one fix; a user that reaches it another way makes it a diff to accept, and another file defining the same name offers none (SR-MULTI-FILE-FIXES)"
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*(-[a-z0-9]+)*$", description: "a regular expression a mixin name must match" }
ported_from: [stylelint-scss/at-mixin-pattern]
since: 0.3.0
---

Mixin names match a pattern, kebab-case by default.

**Why:** A consistent scheme makes names predictable across a codebase.

```codelaw
native: scss/at-mixin-pattern
```

## How to fix

1. `codelaw fix` renames the mixin to kebab-case here and in every `@include` of it.
2. Where the rename is a suggestion, check the diff and accept it with `codelaw fix --unsafe`.
3. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it by hand.

## Examples

### SCSS

#### Don't

```scss
@mixin ButtonBase {
  color: red;
}
```

#### Do

```scss
@mixin button-base {
  color: red;
}
```

#### Don't

```scss
// src/_mixins.scss
@mixin ButtonBase {
  color: red;
}
// src/app.scss
@use "mixins";
.a {
  @include mixins.ButtonBase;
}
```

#### Do

```scss
// src/_mixins.scss
@mixin button-base {
  color: red;
}
// src/app.scss
@use "mixins";
.a {
  @include mixins.button-base;
}
```

## Differences from the source

Only the definition is checked.

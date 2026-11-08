---
id: scss/percent-placeholder-pattern
title: "`%placeholder` names are kebab-case"
languages: [scss]
fix: suggest
fix_reason: "renamed to kebab-case here and in every `@extend` of the project, as one fix; a placeholder is global, so an `@extend` elsewhere may mean another one, and the rename is a diff to accept (SR-MULTI-FILE-FIXES)"
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*(-[a-z0-9]+)*$", description: "a regular expression a placeholder name, without its `%`, must match" }
ported_from: [stylelint-scss/percent-placeholder-pattern]
since: 0.3.0
---

Placeholder selector names match a pattern, kebab-case by default.

**Why:** A consistent scheme makes names predictable across a codebase.

```codelaw
native: scss/percent-placeholder-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the placeholder and every `@extend` of it (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it by hand.

## Examples

### SCSS

#### Don't

```scss
%ButtonBase {
  color: red;
}
```

#### Do

```scss
%button-base {
  color: red;
}
```

#### Don't

```scss
// src/_base.scss
%ButtonBase {
  color: red;
}
// src/button.scss
@use "base";
.button {
  @extend %ButtonBase;
}
```

#### Do

```scss
// src/_base.scss
%button-base {
  color: red;
}
// src/button.scss
@use "base";
.button {
  @extend %button-base;
}
```

## Differences from the source

Only definitions are checked.

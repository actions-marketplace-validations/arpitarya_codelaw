---
id: scss/dollar-variable-pattern
title: "`$variables` are kebab-case"
languages: [scss]
fix: auto
fix_reason: "auto when every file that uses the variable reaches it through `@use`, `@forward` or `@import`: the rename then changes every use, as one fix; a user that reaches it another way makes the rename a diff to accept, and another file declaring the same name offers none (SR-MULTI-FILE-FIXES)"
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*(-[a-z0-9]+)*$", description: "a regular expression the name, without its `$`, must match" }
ported_from: [stylelint-scss/dollar-variable-pattern]
since: 0.3.0
---

`$variable` names match a pattern, kebab-case by default.

**Why:** A consistent scheme makes names predictable across a codebase.

```codelaw
native: scss/dollar-variable-pattern
```

## How to fix

1. `codelaw fix` renames the variable to kebab-case here and in every file that uses it.
2. Where the rename is a suggestion (a user reaches the file some other way), check the diff and accept it with `codelaw fix --unsafe`.
3. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it and every use by hand.

## Examples

### SCSS

#### Don't

```scss
$FontSize: 16px;
```

#### Do

```scss
$font-size: 16px;
```

#### Don't

```scss
// src/_tokens.scss
$FontSize: 16px;
// src/app.scss
@use "tokens";
.a {
  font-size: tokens.$FontSize;
}
```

#### Do

```scss
// src/_tokens.scss
$font-size: 16px;
// src/app.scss
@use "tokens";
.a {
  font-size: tokens.$font-size;
}
```

## Differences from the source

The pattern applies to declarations, not to the names in an `@each` or a mixin's parameter list.

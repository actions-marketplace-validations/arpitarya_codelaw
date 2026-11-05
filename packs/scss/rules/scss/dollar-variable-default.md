---
id: scss/dollar-variable-default
title: "Top-level variables are `!default`"
languages: [scss]
fix: suggest
fix_reason: "`!default` lets a module that uses this one set the value first, which changes who decides it"
stability: preview
ported_from: [stylelint-scss/dollar-variable-default]
since: 0.5.0
---

A top-level `$variable` is declared with `!default`.

**Why:** A module's settings are then configurable with `@use … with (…)`.

```codelaw
native: scss/dollar-variable-default
```

## How to fix

1. Add `!default`.

## Examples

### SCSS

#### Don't

```scss
$size: 1px;
```

#### Do

```scss
$size: 1px !default;
```

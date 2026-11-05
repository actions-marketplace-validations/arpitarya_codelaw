---
id: scss/no-duplicate-load-rules
title: "No module loaded twice"
languages: [scss]
fix: suggest
fix_reason: "removing the second load is safe, unless its `with` or `as` differs"
stability: preview
ported_from: [stylelint-scss/no-duplicate-load-rules]
since: 0.5.0
---

A file doesn't `@use`, `@forward` or `@import` the same URL twice.

**Why:** The second load is a mistake: Sass refuses a repeated `@use`.

```codelaw
native: scss/no-duplicate-load-rules
```

## How to fix

1. Remove the second load.

## Examples

### SCSS

#### Don't

```scss
@use "a";
@use "a";
```

#### Do

```scss
@use "a";
```

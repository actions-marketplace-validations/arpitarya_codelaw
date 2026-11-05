---
id: scss/at-use-no-redundant-alias
title: "No `@use` alias that repeats the module's name"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/at-use-no-redundant-alias]
since: 0.5.0
---

`@use "colors" as colors;` is `@use "colors";`.

**Why:** The module is already named that.

```codelaw
native: scss/at-use-no-redundant-alias
```

## Examples

### SCSS

#### Don't

```scss
@use "colors" as colors;
```

#### Do

```scss
@use "colors";
```

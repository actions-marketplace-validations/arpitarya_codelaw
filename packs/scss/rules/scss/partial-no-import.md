---
id: scss/partial-no-import
title: "No `@import` in a partial"
languages: [scss]
fix: manual
fix_reason: "where the import belongs is the author's call"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/partial-no-import]
since: 0.5.0
---

A partial (`_name.scss`) doesn't `@import`: the file that loads it does.

**Why:** Imports in partials repeat and reorder styles.

```codelaw
native: scss/partial-no-import
```

## How to fix

1. Move the import to the entry file.

## Examples

### SCSS

#### Don't

```scss path=src/_a.scss
@import "a";
```

#### Do

```scss path=src/_a.scss
@use "a";
```

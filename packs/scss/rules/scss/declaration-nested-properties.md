---
id: scss/declaration-nested-properties
title: "No nested properties"
languages: [scss]
fix: manual
fix_reason: "expanding nested properties rewrites a block into declarations, which the author should review"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/declaration-nested-properties]
since: 0.5.0
---

Properties aren't nested under a shared prefix (`font: { family: x; }`): each is written out (`font-family: x`).

**Why:** A reader searches for `font-family`, not `family`.

```codelaw
native: scss/declaration-nested-properties
```

## How to fix

1. Write each nested property as a full declaration.

## Examples

### SCSS

#### Don't

```scss
a {
  font: {
    family: serif;
  }
}
```

#### Do

```scss
a {
  font-family: serif;
}
```

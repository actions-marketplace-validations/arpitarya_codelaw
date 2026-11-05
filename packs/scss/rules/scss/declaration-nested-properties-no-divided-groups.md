---
id: scss/declaration-nested-properties-no-divided-groups
title: "Nested properties of one prefix are in one group"
languages: [scss]
fix: manual
fix_reason: "merging the groups moves declarations, which can change which one wins"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/declaration-nested-properties-no-divided-groups]
since: 0.5.0
---

A block doesn't hold two nested-property groups with the same prefix.

**Why:** The second group is easy to miss.

```codelaw
native: scss/declaration-nested-properties-no-divided-groups
```

## How to fix

1. Merge the two groups.

## Examples

### SCSS

#### Don't

```scss
a {
  font: {
    size: 1px;
  }
  color: red;
  font: {
    weight: bold;
  }
}
```

#### Do

```scss
a {
  font: {
    size: 1px;
    weight: bold;
  }
  color: red;
}
```

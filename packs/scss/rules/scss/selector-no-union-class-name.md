---
id: scss/selector-no-union-class-name
title: "No class names built with `&`"
languages: [scss]
fix: manual
fix_reason: "the full class name is the author's to write, and its uses in markup to update"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/selector-no-union-class-name]
since: 0.5.0
---

A nested selector doesn't build a class name from its parent's (`&-item`).

**Why:** A reader can't search the stylesheet for `.card-item`.

```codelaw
native: scss/selector-no-union-class-name
```

## How to fix

1. Write the class name out.

## Examples

### SCSS

#### Don't

```scss
.card {
  &-item {
    color: red;
  }
}
```

#### Do

```scss
.card-item {
  color: red;
}
```

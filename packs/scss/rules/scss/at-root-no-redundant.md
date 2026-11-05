---
id: scss/at-root-no-redundant
title: "No redundant `@at-root`"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/at-root-no-redundant]
since: 0.5.0
---

`@at-root` isn't used at the top level, or before a selector that holds `&` (which already resolves to the root).

**Why:** It does nothing there.

```codelaw
native: scss/at-root-no-redundant
```

## Examples

### SCSS

#### Don't

```scss
a {
  @at-root &-b {
    color: red;
  }
}
```

#### Do

```scss
a {
  &-b {
    color: red;
  }
}
```

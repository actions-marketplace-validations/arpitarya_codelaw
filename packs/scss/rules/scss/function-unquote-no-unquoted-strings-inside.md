---
id: scss/function-unquote-no-unquoted-strings-inside
title: "No `unquote()` of an unquoted string"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/function-unquote-no-unquoted-strings-inside]
since: 0.5.0
---

`unquote(a)` is `a`.

**Why:** It does nothing.

```codelaw
native: scss/function-unquote-no-unquoted-strings-inside
```

## Examples

### SCSS

#### Don't

```scss
a {
  font-family: unquote(serif);
}
```

#### Do

```scss
a {
  font-family: serif;
}
```

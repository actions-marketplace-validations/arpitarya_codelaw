---
id: scss/at-function-named-arguments
title: "Function arguments are passed by name"
languages: [scss]
fix: auto
fix_reason: "auto when the function is defined in the same file, whose parameter names the edit writes; otherwise reported without an edit"
stability: preview
ported_from: [stylelint-scss/at-function-named-arguments]
since: 0.5.0
---

A call to a `@function` passes its arguments by name: `size($base: 2)`.

**Why:** A named argument says what it is, and survives a reordered signature.

```codelaw
native: scss/at-function-named-arguments
```

## Examples

### SCSS

#### Don't

```scss
@function size($base) {
  @return $base * 2;
}
a {
  width: size(2px);
}
```

#### Do

```scss
@function size($base) {
  @return $base * 2;
}
a {
  width: size($base: 2px);
}
```

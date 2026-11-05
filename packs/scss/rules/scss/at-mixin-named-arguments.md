---
id: scss/at-mixin-named-arguments
title: "Mixin arguments are passed by name"
languages: [scss]
fix: auto
fix_reason: "auto when the mixin is defined in the same file, whose parameter names the edit writes; otherwise reported without an edit"
stability: preview
ported_from: [stylelint-scss/at-mixin-named-arguments]
since: 0.5.0
---

An `@include` passes its arguments by name: `@include pad($size: 2px)`.

**Why:** A named argument says what it is.

```codelaw
native: scss/at-mixin-named-arguments
```

## Examples

### SCSS

#### Don't

```scss
@mixin pad($size) {
  padding: $size;
}
a {
  @include pad(2px);
}
```

#### Do

```scss
@mixin pad($size) {
  padding: $size;
}
a {
  @include pad($size: 2px);
}
```

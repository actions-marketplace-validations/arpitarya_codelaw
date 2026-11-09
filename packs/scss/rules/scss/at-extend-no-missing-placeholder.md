---
id: scss/at-extend-no-missing-placeholder
title: "`@extend` targets a placeholder"
languages: [scss]
fix: manual
fix_reason: "restructure: turning the class into a placeholder removes the class from the compiled CSS, which the markup that uses it needs; moving the shared declarations into a `%placeholder` is a change to the stylesheet's design (W-162)"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/at-extend-no-missing-placeholder]
since: 0.3.0
---

`@extend` names a `%placeholder`, not a class or an element.

**Why:** Extending a class drags every selector that mentions the class into the output, including ones far from the code at hand.

```codelaw
native: scss/at-extend-no-missing-placeholder
```

## How to fix

1. Move the shared declarations into a `%placeholder`.
2. `@extend` the placeholder from each place that needs it.

## Examples

### SCSS

#### Don't

```scss
.a {
  @extend .b;
}
```

#### Do

```scss
%b {
  color: red;
}

.a {
  @extend %b;
}
```

## Differences from the source

A target with interpolation is not judged.

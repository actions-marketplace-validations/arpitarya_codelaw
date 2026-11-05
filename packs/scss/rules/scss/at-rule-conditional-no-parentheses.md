---
id: scss/at-rule-conditional-no-parentheses
title: "No parentheses around `@if` and `@while` conditions"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/at-rule-conditional-no-parentheses]
since: 0.5.0
---

`@if ($a) {` is written `@if $a {`.

**Why:** Sass needs none; they read as CSS.

```codelaw
native: scss/at-rule-conditional-no-parentheses
```

## Examples

### SCSS

#### Don't

```scss
a {
  @if ($wide) {
    width: 100%;
  }
}
```

#### Do

```scss
a {
  @if $wide {
    width: 100%;
  }
}
```

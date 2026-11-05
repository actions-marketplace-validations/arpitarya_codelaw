---
id: scss/at-mixin-argumentless-call-parentheses
title: "No parentheses on a mixin call without arguments"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/at-mixin-argumentless-call-parentheses]
since: 0.5.0
---

`@include clearfix();` is written `@include clearfix;`.

**Why:** The empty parentheses add nothing.

```codelaw
native: scss/at-mixin-argumentless-call-parentheses
```

## Examples

### SCSS

#### Don't

```scss
a {
  @include clearfix();
}
```

#### Do

```scss
a {
  @include clearfix;
}
```

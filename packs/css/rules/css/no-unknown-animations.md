---
id: css/no-unknown-animations
title: "No animation that isn't defined"
languages: [css, scss]
fix: manual
fix_reason: "which animation was meant, or where it is defined, is the author's to say"
fix_exception: value
stability: preview
ported_from: [stylelint/no-unknown-animations]
since: 0.5.0
---

An `animation-name` (or an `animation` shorthand's name) is defined by a `@keyframes` in the same stylesheet.

**Why:** A misspelt name runs no animation, silently.

```codelaw
native: css/no-unknown-animations
```

## How to fix

1. Define the `@keyframes`, or fix the name.

## Examples

### CSS

#### Don't

```css
a {
  animation-name: fade;
}
```

#### Do

```css
@keyframes fade {
  to {
    opacity: 0;
  }
}

a {
  animation-name: fade;
}
```

### SCSS

#### Don't

```scss
a {
  animation-name: fade;
}
```

#### Do

```scss
@keyframes fade {
  to {
    opacity: 0;
  }
}

a {
  animation-name: fade;
}
```

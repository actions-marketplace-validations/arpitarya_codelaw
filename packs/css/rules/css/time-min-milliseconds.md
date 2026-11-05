---
id: css/time-min-milliseconds
title: "No transition or animation too short to see"
languages: [css, scss]
fix: manual
fix_reason: "the duration is the author's to choose"
fix_exception: value
stability: preview
options:
  min: { type: "number", default: 100, description: "the shortest duration allowed, in milliseconds" }
ported_from: [stylelint/time-min-milliseconds]
since: 0.5.0
---

A transition or animation duration is at least `min` milliseconds (100 by default), or 0.

**Why:** A shorter one is an unnoticed flicker, usually a unit mistake (`ms` for `s`).

```codelaw
native: css/time-min-milliseconds
```

## How to fix

1. Lengthen the duration, or remove the transition.

## Examples

### CSS

#### Don't

```css
a {
  transition-duration: 5ms;
}
```

#### Do

```css
a {
  transition-duration: 150ms;
}
```

### SCSS

#### Don't

```scss
a {
  transition-duration: 5ms;
}
```

#### Do

```scss
a {
  transition-duration: 150ms;
}
```

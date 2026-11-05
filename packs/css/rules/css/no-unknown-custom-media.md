---
id: css/no-unknown-custom-media
title: "No custom media that isn't defined"
languages: [css, scss]
fix: manual
fix_reason: "which custom media was meant is the author's to say"
fix_exception: value
stability: preview
ported_from: [stylelint/no-unknown-custom-media]
since: 0.5.0
---

A `(--name)` in a media query is defined by `@custom-media --name` in the same stylesheet.

**Why:** An undefined custom media matches nothing.

```codelaw
native: css/no-unknown-custom-media
```

## How to fix

1. Define it with `@custom-media`, or fix the name.

## Examples

### CSS

#### Don't

```css
@media (--wide) {
  a {
    color: red;
  }
}
```

#### Do

```css
@custom-media --wide (min-width: 60em);

@media (--wide) {
  a {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss
@media (--wide) {
  a {
    color: red;
  }
}
```

#### Do

```scss
@custom-media --wide (min-width: 60em);

@media (--wide) {
  a {
    color: red;
  }
}
```

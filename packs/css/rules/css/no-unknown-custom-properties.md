---
id: css/no-unknown-custom-properties
title: "No custom property that isn't defined"
languages: [css, scss]
fix: manual
fix_reason: "the property's value, or a fallback, is the author's to write"
fix_exception: value
stability: preview
ported_from: [stylelint/no-unknown-custom-properties]
since: 0.5.0
---

A `var(--name)` without a fallback names a custom property declared (or registered with `@property`) in the same stylesheet.

**Why:** An undefined variable makes the declaration invalid at computed time.

```codelaw
native: css/no-unknown-custom-properties
```

## How to fix

1. Declare the property, or give `var()` a fallback.

## Examples

### CSS

#### Don't

```css
a {
  color: var(--brand);
}
```

#### Do

```css
a {
  --brand: red;
  color: var(--brand);
}
```

### SCSS

#### Don't

```scss
a {
  color: var(--brand);
}
```

#### Do

```scss
a {
  --brand: red;
  color: var(--brand);
}
```

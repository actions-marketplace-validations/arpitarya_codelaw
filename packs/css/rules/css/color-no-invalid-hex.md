---
id: css/color-no-invalid-hex
title: "No invalid hex colour"
languages: [css, scss]
fix: manual
fix_reason: "the colour that was meant is the author's to write"
fix_exception: value
stability: preview
ported_from: [stylelint/color-no-invalid-hex]
since: 0.5.0
---

A hex colour has 3, 4, 6 or 8 hexadecimal digits.

**Why:** An invalid colour drops the whole declaration.

```codelaw
native: css/color-no-invalid-hex
```

## How to fix

1. Write the colour that was meant.

## Examples

### CSS

#### Don't

```css
a {
  color: #ff00g0;
}
```

#### Do

```css
a {
  color: #ff00f0;
}
```

### SCSS

#### Don't

```scss
a {
  color: #ff00g0;
}
```

#### Do

```scss
a {
  color: #ff00f0;
}
```

---
id: css/property-layout-mappings
title: "Flow-relative properties, not physical ones"
languages: [css, scss]
fix: suggest
fix_reason: "a flow-relative property follows the writing mode, so in a right-to-left or vertical layout it lands on another side"
stability: preview
ported_from: [stylelint/property-layout-mappings]
since: 0.5.0
---

A property with a flow-relative equivalent uses it: `margin-left` is `margin-inline-start`, `width` is `inline-size`.

**Why:** The layout then follows the writing direction.

```codelaw
native: css/property-layout-mappings
```

## How to fix

1. Use the flow-relative property.

## Examples

### CSS

#### Don't

```css
a {
  margin-left: 0;
}
```

#### Do

```css
a {
  margin-inline-start: 0;
}
```

### SCSS

#### Don't

```scss
a {
  margin-left: 0;
}
```

#### Do

```scss
a {
  margin-inline-start: 0;
}
```

---
id: css/unit-layout-mappings
title: "Flow-relative units, not physical ones"
languages: [css, scss]
fix: suggest
fix_reason: "a flow-relative unit follows the writing mode, so in a vertical layout it measures the other axis"
stability: preview
ported_from: [stylelint/unit-layout-mappings]
since: 0.5.0
---

A viewport or container unit with a flow-relative equivalent uses it: `vw` is `vi`, `vh` is `vb`.

**Why:** The size then follows the writing direction.

```codelaw
native: css/unit-layout-mappings
```

## How to fix

1. Use the flow-relative unit.

## Examples

### CSS

#### Don't

```css
a {
  width: 50vw;
}
```

#### Do

```css
a {
  width: 50vi;
}
```

### SCSS

#### Don't

```scss
a {
  width: 50vw;
}
```

#### Do

```scss
a {
  width: 50vi;
}
```

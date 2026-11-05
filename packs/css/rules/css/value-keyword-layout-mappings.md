---
id: css/value-keyword-layout-mappings
title: "Flow-relative keywords, not physical ones"
languages: [css, scss]
fix: suggest
fix_reason: "a flow-relative keyword follows the writing mode, so in a right-to-left layout it lands on the other side"
stability: preview
ported_from: [stylelint/value-keyword-layout-mappings]
since: 0.5.0
---

A value with a flow-relative keyword uses it: `float: left` is `inline-start`, `text-align: right` is `end`.

**Why:** The layout then follows the writing direction.

```codelaw
native: css/value-keyword-layout-mappings
```

## How to fix

1. Use the flow-relative keyword.

## Examples

### CSS

#### Don't

```css
a {
  text-align: left;
}
```

#### Do

```css
a {
  text-align: start;
}
```

### SCSS

#### Don't

```scss
a {
  text-align: left;
}
```

#### Do

```scss
a {
  text-align: start;
}
```

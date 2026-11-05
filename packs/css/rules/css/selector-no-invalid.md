---
id: css/selector-no-invalid
title: "No invalid selector"
languages: [css, scss]
fix: manual
fix_reason: "the selector that was meant is the author's to write"
fix_exception: value
stability: preview
ported_from: [stylelint/selector-no-invalid]
since: 0.5.0
---

A selector parses: no unbalanced brackets, no empty selector in a list, no trailing combinator, no doubled `.` or `#`.

**Why:** An invalid selector drops the whole rule.

```codelaw
native: css/selector-no-invalid
```

## How to fix

1. Fix the selector.

## Examples

### CSS

#### Don't

```css
a, , b {
  color: red;
}
```

#### Do

```css
a, b {
  color: red;
}
```

### SCSS

#### Don't

```scss
a, , b {
  color: red;
}
```

#### Do

```scss
a, b {
  color: red;
}
```

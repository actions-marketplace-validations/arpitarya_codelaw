---
id: css/unit-no-unknown
title: "No unknown units"
languages: [css, scss]
fix: manual
fix_reason: "which unit was meant is the author's to say"
fix_exception: value
stability: preview
options:
  ignore: { type: "string[]", default: [], description: "units to accept" }
ported_from: [stylelint/unit-no-unknown]
since: 0.5.0
---

A number's unit is a CSS unit: `px`, `rem`, `dvh` — not a typo. The `ignore` option names others.

**Why:** An unknown unit drops the declaration.

```codelaw
native: css/unit-no-unknown
```

## How to fix

1. Fix the unit.

## Examples

### CSS

#### Don't

```css
a {
  width: 10pix;
}
```

#### Do

```css
a {
  width: 10px;
}
```

### SCSS

#### Don't

```scss
a {
  width: 10pix;
}
```

#### Do

```scss
a {
  width: 10px;
}
```

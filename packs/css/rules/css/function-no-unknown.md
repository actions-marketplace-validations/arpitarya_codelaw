---
id: css/function-no-unknown
title: "No unknown functions"
languages: [css, scss]
fix: manual
fix_reason: "which function was meant is the author's to say"
fix_exception: value
stability: preview
options:
  ignore: { type: "string[]", default: [], description: "function names to accept" }
ported_from: [stylelint/function-no-unknown]
since: 0.5.0
---

A function call in a value names a CSS function: `clamp()`, `color-mix()` — not a typo or a preprocessor function in plain CSS. The `ignore` option names others.

**Why:** An unknown function drops the declaration.

```codelaw
native: css/function-no-unknown
```

## How to fix

1. Fix the name, or list it in `ignore`.

## Examples

### CSS

#### Don't

```css
a {
  width: calc2(1px + 2px);
}
```

#### Do

```css
a {
  width: calc(1px + 2px);
}
```

### SCSS

#### Don't

```scss
a {
  width: calc2(1px + 2px);
}
```

#### Do

```scss
a {
  width: calc(1px + 2px);
}
```

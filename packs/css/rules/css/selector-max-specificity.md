---
id: css/selector-max-specificity
title: "No selector more specific than allowed"
languages: [css, scss]
fix: manual
fix_reason: "lowering a selector's specificity changes which rule wins, so it is the author's call"
fix_exception: restructure
stability: preview
options:
  max: { type: "string", default: "0,4,0", description: "the highest specificity allowed, as `ids,classes,types`" }
ported_from: [stylelint/selector-max-specificity]
since: 0.5.0
---

A selector's specificity is at most `max` (`0,4,0` by default).

**Why:** A very specific selector can only be overridden by a more specific one.

```codelaw
native: css/selector-max-specificity
```

## How to fix

1. Select the element with a class of its own.

## Examples

### CSS

#### Don't

```css options={"max":"0,1,0"}
.a .b {
  color: red;
}
```

#### Do

```css options={"max":"0,1,0"}
.a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"max":"0,1,0"}
.a .b {
  color: red;
}
```

#### Do

```scss options={"max":"0,1,0"}
.a {
  color: red;
}
```

---
id: css/selector-max-type
title: "No selector with too many type selectors"
languages: [css, scss]
fix: manual
fix_reason: "which type selectors to drop changes what the selector matches, so it is the author's call"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 2, description: "the most allowed in one selector" }
ported_from: [stylelint/selector-max-type]
since: 0.5.0
---

A selector holds at most `max` type selectors (2 by default).

**Why:** A selector that specific is hard to override and ties the styles to the markup.

```codelaw
native: css/selector-max-type
```

## How to fix

1. Give the element a class of its own, and select it.

## Examples

### CSS

#### Don't

```css options={"max":1}
div a {
  color: red;
}
```

#### Do

```css options={"max":1}
div .a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"max":1}
div a {
  color: red;
}
```

#### Do

```scss options={"max":1}
div .a {
  color: red;
}
```

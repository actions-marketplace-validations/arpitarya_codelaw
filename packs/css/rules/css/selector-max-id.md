---
id: css/selector-max-id
title: "No selector with too many id selectors"
languages: [css, scss]
fix: manual
fix_reason: "which id selectors to drop changes what the selector matches, so it is the author's call"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 1, description: "the most allowed in one selector" }
ported_from: [stylelint/selector-max-id]
since: 0.5.0
---

A selector holds at most `max` id selectors (1 by default).

**Why:** A selector that specific is hard to override and ties the styles to the markup.

```codelaw
native: css/selector-max-id
```

## How to fix

1. Give the element a class of its own, and select it.

## Examples

### CSS

#### Don't

```css options={"max":0}
#a {
  color: red;
}
```

#### Do

```css options={"max":0}
.a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"max":0}
#a {
  color: red;
}
```

#### Do

```scss options={"max":0}
.a {
  color: red;
}
```

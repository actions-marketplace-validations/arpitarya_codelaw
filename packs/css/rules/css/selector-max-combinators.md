---
id: css/selector-max-combinators
title: "No selector with too many combinators"
languages: [css, scss]
fix: manual
fix_reason: "which combinators to drop changes what the selector matches, so it is the author's call"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 3, description: "the most allowed in one selector" }
ported_from: [stylelint/selector-max-combinators]
since: 0.5.0
---

A selector holds at most `max` combinators (3 by default).

**Why:** A selector that specific is hard to override and ties the styles to the markup.

```codelaw
native: css/selector-max-combinators
```

## How to fix

1. Give the element a class of its own, and select it.

## Examples

### CSS

#### Don't

```css options={"max":1}
a b c {
  color: red;
}
```

#### Do

```css options={"max":1}
a b {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"max":1}
a b c {
  color: red;
}
```

#### Do

```scss options={"max":1}
a b {
  color: red;
}
```

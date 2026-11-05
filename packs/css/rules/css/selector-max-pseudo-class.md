---
id: css/selector-max-pseudo-class
title: "No selector with too many pseudo-classes"
languages: [css, scss]
fix: manual
fix_reason: "which pseudo-classes to drop changes what the selector matches, so it is the author's call"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 3, description: "the most allowed in one selector" }
ported_from: [stylelint/selector-max-pseudo-class]
since: 0.5.0
---

A selector holds at most `max` pseudo-classes (3 by default).

**Why:** A selector that specific is hard to override and ties the styles to the markup.

```codelaw
native: css/selector-max-pseudo-class
```

## How to fix

1. Give the element a class of its own, and select it.

## Examples

### CSS

#### Don't

```css options={"max":1}
a:hover:focus {
  color: red;
}
```

#### Do

```css options={"max":1}
a:hover {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"max":1}
a:hover:focus {
  color: red;
}
```

#### Do

```scss options={"max":1}
a:hover {
  color: red;
}
```

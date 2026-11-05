---
id: css/at-rule-property-required-list
title: "At-rules hold the descriptors the project requires"
languages: [css, scss]
fix: manual
fix_reason: "the descriptor's value is the author's to write"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "at-rule name → the descriptors it must hold" }
ported_from: [stylelint/at-rule-property-required-list]
since: 0.5.0
---

An at-rule named in `list` holds each descriptor listed for it: `{ "font-face": ["font-display"] }`. With no option set, the rule does nothing.

**Why:** A project makes sure a `@font-face` says how it loads.

```codelaw
native: css/at-rule-property-required-list
```

## How to fix

1. Add the descriptor.

## Examples

### CSS

#### Don't

```css options={"list":{"font-face":["font-display"]}}
@font-face {
  font-family: "A";
}
```

#### Do

```css options={"list":{"font-face":["font-display"]}}
@font-face {
  font-family: "A";
  font-display: swap;
}
```

### SCSS

#### Don't

```scss options={"list":{"font-face":["font-display"]}}
@font-face {
  font-family: "A";
}
```

#### Do

```scss options={"list":{"font-face":["font-display"]}}
@font-face {
  font-family: "A";
  font-display: swap;
}
```

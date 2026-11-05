---
id: css/selector-no-unmatchable
title: "No selector that can never match"
languages: [css, scss]
fix: manual
fix_reason: "what the selector should match is the author's to say"
fix_exception: value
stability: preview
ported_from: [stylelint/selector-no-unmatchable]
since: 0.5.0
---

A compound selector doesn't name two ids, two types, or anything after a pseudo-element.

**Why:** It matches nothing, so its rule never applies.

```codelaw
native: css/selector-no-unmatchable
```

## How to fix

1. Fix the selector.

## Examples

### CSS

#### Don't

```css
#a#b {
  color: red;
}
```

#### Do

```css
#a {
  color: red;
}
```

### SCSS

#### Don't

```scss
#a#b {
  color: red;
}
```

#### Do

```scss
#a {
  color: red;
}
```

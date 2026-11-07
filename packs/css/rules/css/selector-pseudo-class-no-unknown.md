---
id: css/selector-pseudo-class-no-unknown
title: "No unknown pseudo-classes"
languages: [css, scss]
fix: suggest
fix_reason: "the known names within two edits are choices, nearest first; a name with none near is manual, and a pseudo-element written with one colon gets its second"
fix_exception: value
stability: stable
options:
  ignorePseudoClasses: { type: "string[]", default: [], description: "pseudo-classes to accept; `/regex/` for a pattern" }
ported_from: [stylelint/selector-pseudo-class-no-unknown]
since: 0.1.0
---

Every pseudo-class is one the specifications define, and pseudo-elements are written with `::`. Vendor-prefixed pseudo-classes are skipped; in `@page`, only page pseudo-classes are allowed.

**Why:** A browser drops a whole rule whose selector has a pseudo-class it does not know.

```codelaw
native: css/selector-pseudo-class-no-unknown
```

## How to fix

1. Correct the pseudo-class name.

## Examples

### CSS

#### Don't

```css
a:hovr {
  color: red;
}
```

#### Do

```css
a:hover {
  color: red;
}
```
### SCSS

#### Don't

```scss
a:hovr {
  color: red;
}
```

#### Do

```scss
a:hover {
  color: red;
}
```

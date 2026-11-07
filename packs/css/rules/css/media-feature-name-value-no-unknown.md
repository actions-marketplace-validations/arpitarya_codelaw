---
id: css/media-feature-name-value-no-unknown
title: "Media features have valid values"
languages: [css, scss]
fix: suggest
fix_reason: "the feature's keywords within two edits are choices, nearest first; a value with none near is manual"
fix_exception: value
stability: stable
ported_from: [stylelint/media-feature-name-value-no-unknown]
since: 0.1.0
---

A media feature with a fixed set of values (`orientation`, `hover`, `prefers-color-scheme`, …) uses one of them, and a feature with a typed value (`width`, `resolution`, `color`) uses that type.

**Why:** A media query with an invalid value never matches, so the block never applies.

```codelaw
native: css/media-feature-name-value-no-unknown
```

## How to fix

1. Use a value the media feature accepts.

## Examples

### CSS

#### Don't

```css
@media (orientation: portait) {
  a {
    color: red;
  }
}
```

#### Do

```css
@media (orientation: portrait) {
  a {
    color: red;
  }
}
```
### SCSS

#### Don't

```scss
@media (orientation: portait) {
  a {
    color: red;
  }
}
```

#### Do

```scss
@media (orientation: portrait) {
  a {
    color: red;
  }
}
```

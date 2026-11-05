---
id: css/selector-no-deprecated
title: "No deprecated selectors"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/selector-no-deprecated]
since: 0.5.0
---

A selector uses `:is()`, not its deprecated forms `:matches()`, `:-webkit-any()` or `:-moz-any()`.

**Why:** The old forms are kept only for compatibility.

```codelaw
native: css/selector-no-deprecated
```

## Examples

### CSS

#### Don't

```css
:matches(a, b) {
  color: red;
}
```

#### Do

```css
:is(a, b) {
  color: red;
}
```

### SCSS

#### Don't

```scss
:matches(a, b) {
  color: red;
}
```

#### Do

```scss
:is(a, b) {
  color: red;
}
```

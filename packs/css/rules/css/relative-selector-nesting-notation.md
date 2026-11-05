---
id: css/relative-selector-nesting-notation
title: "Nested relative selectors start with `&`"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/relative-selector-nesting-notation]
since: 0.5.0
---

A nested selector that starts with a combinator writes the nesting selector: `> a` is `& > a`.

**Why:** The `&` shows what the combinator is relative to.

```codelaw
native: css/relative-selector-nesting-notation
```

## Examples

### CSS

#### Don't

```css
a {
  > b {
    color: red;
  }
}
```

#### Do

```css
a {
  & > b {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss
a {
  > b {
    color: red;
  }
}
```

#### Do

```scss
a {
  & > b {
    color: red;
  }
}
```

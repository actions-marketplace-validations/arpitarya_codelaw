---
id: css/display-notation
title: "`display` in its short form"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/display-notation]
since: 0.5.0
---

A `display` value is written in its one-keyword form: `block flow` is `block`, `inline flex` is `inline-flex`.

**Why:** The short forms are the ones every reader knows.

```codelaw
native: css/display-notation
```

## Examples

### CSS

#### Don't

```css
a {
  display: inline flex;
}
```

#### Do

```css
a {
  display: inline-flex;
}
```

### SCSS

#### Don't

```scss
a {
  display: inline flex;
}
```

#### Do

```scss
a {
  display: inline-flex;
}
```

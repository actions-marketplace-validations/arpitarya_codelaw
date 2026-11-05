---
id: css/comment-whitespace-inside
title: "A space inside comment delimiters"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/comment-whitespace-inside]
since: 0.5.0
---

A comment has a space after `/*` and before `*/`.

**Why:** The text reads apart from its delimiters.

```codelaw
native: css/comment-whitespace-inside
```

## Examples

### CSS

#### Don't

```css
/*note*/
a {
  color: red;
}
```

#### Do

```css
/* note */
a {
  color: red;
}
```

### SCSS

#### Don't

```scss
/*note*/
a {
  color: red;
}
```

#### Do

```scss
/* note */
a {
  color: red;
}
```

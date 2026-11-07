---
id: css/media-query-no-invalid
title: "Media queries follow the grammar"
languages: [css, scss]
fix: suggest
fix_reason: "three conditions joined by both `and` and `or` get the two groupings as choices, `(a and b) or c` and `a and (b or c)`; the query matches nothing today and each grouping changes which media apply, so neither is the default. Any other invalid query is manual, as the intended query is the author's call"
fix_exception: value
stability: stable
options:
  ignoreFunctions: { type: "string[]", default: [], description: "functions to accept in queries, such as `theme`" }
ported_from: [stylelint/media-query-no-invalid]
since: 0.1.0
---

Every query in `@media` and `@custom-media` is grammatical under Media Queries level 5: `not` is followed by a space, comparisons are `<`, `<=`, `>`, `>=` or `=`, and `and` and `or` are not mixed without parentheses.

**Why:** A browser treats an invalid query as `not all`, so the block never applies.

```codelaw
native: css/media-query-no-invalid
```

## How to fix

1. Rewrite the query; add parentheses to group `and` and `or`, and a space after `not`.

## Examples

### CSS

#### Don't

```css
@media (color) and (hover) or (width >= 600px) {
  a {
    color: red;
  }
}
```

#### Fixed (group left)

```css
@media ((color) and (hover)) or (width >= 600px) {
  a {
    color: red;
  }
}
```

#### Fixed (group right)

```css
@media (color) and ((hover) or (width >= 600px)) {
  a {
    color: red;
  }
}
```

#### Do

```css
@media (color) and ((hover) or (width >= 600px)) {
  a {
    color: red;
  }
}
```
### SCSS

#### Don't

```scss
@media (color) and (hover) or (width >= 600px) {
  a {
    color: red;
  }
}
```

#### Fixed (group left)

```scss
@media ((color) and (hover)) or (width >= 600px) {
  a {
    color: red;
  }
}
```

#### Fixed (group right)

```scss
@media (color) and ((hover) or (width >= 600px)) {
  a {
    color: red;
  }
}
```

#### Do

```scss
@media (color) and ((hover) or (width >= 600px)) {
  a {
    color: red;
  }
}
```

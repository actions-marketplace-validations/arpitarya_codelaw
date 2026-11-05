---
id: css/font-weight-notation
title: "Font weights are numeric"
languages: [css, scss]
fix: auto
stability: preview
options:
  mode: { type: "string", default: "numeric", description: "`numeric` or `named-where-possible`" }
ported_from: [stylelint/font-weight-notation]
since: 0.5.0
---

With `mode: numeric` (the default) `font-weight: bold` is `700` and `normal` is `400`; with `named-where-possible` the reverse.

**Why:** One notation is easier to compare.

```codelaw
native: css/font-weight-notation
```

## Examples

### CSS

#### Don't

```css
a {
  font-weight: bold;
}
```

#### Do

```css
a {
  font-weight: 700;
}
```

### SCSS

#### Don't

```scss
a {
  font-weight: bold;
}
```

#### Do

```scss
a {
  font-weight: 700;
}
```

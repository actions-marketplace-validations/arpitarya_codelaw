---
id: css/function-url-quotes
title: "URLs are quoted"
languages: [css, scss]
fix: auto
stability: preview
options:
  mode: { type: "string", default: "always", description: "`always` or `never`" }
ported_from: [stylelint/function-url-quotes]
since: 0.5.0
---

With `mode: always` (the default), the address in `url()` is quoted; with `never`, it is not where it needn't be.

**Why:** A quoted address can hold any character.

```codelaw
native: css/function-url-quotes
```

## Examples

### CSS

#### Don't

```css
a {
  background: url(a.png);
}
```

#### Do

```css
a {
  background: url("a.png");
}
```

### SCSS

#### Don't

```scss
a {
  background: url(a.png);
}
```

#### Do

```scss
a {
  background: url("a.png");
}
```

---
id: css/function-url-no-scheme-relative
title: "No scheme-relative URLs"
languages: [css, scss]
fix: manual
fix_reason: "whether the address is `https:` or `http:` is the author's to say"
fix_exception: value
stability: preview
ported_from: [stylelint/function-url-no-scheme-relative]
since: 0.5.0
---

A `url()` names its scheme: not `//cdn.example.com/a.png`.

**Why:** A scheme-relative address loads over HTTP from an HTTP page.

```codelaw
native: css/function-url-no-scheme-relative
```

## How to fix

1. Write `https:` in front of the address.

## Examples

### CSS

#### Don't

```css
a {
  background: url("//cdn.example.com/a.png");
}
```

#### Do

```css
a {
  background: url("https://cdn.example.com/a.png");
}
```

### SCSS

#### Don't

```scss
a {
  background: url("//cdn.example.com/a.png");
}
```

#### Do

```scss
a {
  background: url("https://cdn.example.com/a.png");
}
```

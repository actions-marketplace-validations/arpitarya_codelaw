---
id: scss/function-quote-no-quoted-strings-inside
title: "No `quote()` of a quoted string"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/function-quote-no-quoted-strings-inside]
since: 0.5.0
---

`quote("a")` is `"a"`.

**Why:** It does nothing.

```codelaw
native: scss/function-quote-no-quoted-strings-inside
```

## Examples

### SCSS

#### Don't

```scss
a {
  content: quote("a");
}
```

#### Do

```scss
a {
  content: "a";
}
```

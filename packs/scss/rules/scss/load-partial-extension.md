---
id: scss/load-partial-extension
title: "No file extension when loading a partial"
languages: [scss]
fix: auto
stability: preview
ported_from: [stylelint-scss/load-partial-extension]
since: 0.5.0
---

`@use "a.scss"` is `@use "a"`.

**Why:** Sass finds the file; the extension ties the code to one syntax.

```codelaw
native: scss/load-partial-extension
```

## Examples

### SCSS

#### Don't

```scss
@use "a.scss";
```

#### Do

```scss
@use "a";
```

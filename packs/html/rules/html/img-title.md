---
id: html/img-title
title: "Decorative images have no title"
languages: [html]
fix: suggest
fix_reason: "the choices are removing the `title`, keeping the image decorative, or moving its text into `alt`, making it described; only the author knows which"
stability: preview
ported_from: [html-validate/wcag/h67]
tags: [a11y]
since: 0.1.0
---

An `<img>` with empty `alt` has no `title`.

**Why:** `alt=""` marks the image as decorative, while a `title` suggests it carries information; screen readers handle the mix inconsistently.

```codelaw
native: html/img-title
```

## How to fix

1. If the image carries information, move the text from `title` to `alt`.
2. If it is decorative, remove the `title`.

## Examples

### HTML

#### Don't

```html
<img src="logo.png" alt="" title="Acme">
```

#### Fixed (remove the title)

```html
<img src="logo.png" alt="">
```

#### Fixed (title as alt)

```html
<img src="logo.png" alt="Acme">
```

#### Do

```html
<img src="logo.png" alt="Acme">
```

---
id: html/no-style-tag
title: "No `<style>` element"
languages: [html]
fix: manual
fix_reason: "moving styles into a stylesheet changes the files the page loads"
fix_exception: restructure
stability: preview
ported_from: [html-validate/no-style-tag]
since: 0.5.0
---

Styles live in a linked stylesheet, not a `<style>` element.

**Why:** Inline styles can't be cached or shared, and a strict Content-Security-Policy blocks them.

```codelaw
native: html/no-style-tag
```

## How to fix

1. Move the rules into a stylesheet and link it.

## Examples

### HTML

#### Don't

```html
<style>
  p { color: red; }
</style>
```

#### Do

```html
<link rel="stylesheet" href="site.css">
```

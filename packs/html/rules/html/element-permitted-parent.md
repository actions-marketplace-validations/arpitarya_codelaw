---
id: html/element-permitted-parent
title: "Elements have a parent HTML allows"
languages: [html]
fix: suggest
fix_reason: "the choices are changing the parent to the element it needs (when the children are all it holds) or wrapping the run of them in one; either changes the markup's structure, which the reviewer confirms"
fix_exception: restructure
stability: preview
ported_from: [html-validate/element-permitted-parent]
since: 0.1.0
---

Elements that need a particular parent have it: `<li>` in `<ul>`, `<ol>` or `<menu>`; `<title>` and `<base>` in `<head>`; `<head>` and `<body>` in `<html>`.

**Why:** Elsewhere they lose their meaning: an `<li>` outside a list is not a list item to assistive technology.

```codelaw
native: html/element-permitted-parent
```

## How to fix

1. Wrap the element in the parent it needs.
2. Or replace it with an element that fits where it is.

## Examples

### HTML

#### Don't

```html
<div>
  <li>Apples</li>
</div>
```

#### Fixed (change the parent)

```html
<ul>
  <li>Apples</li>
</ul>
```

#### Fixed (wrap)

```html
<div>
  <ul><li>Apples</li></ul>
</div>
```

#### Do

```html
<ul>
  <li>Apples</li>
</ul>
```

---
id: html/empty-title
title: "<title> has text"
languages: [html]
fix: suggest
fix_reason: "the empty title takes the text of the document's first `<h1>`; the reviewer confirms it names the page. A document with no plain `<h1>` is manual"
fix_exception: authored
stability: preview
ported_from: [html-validate/empty-title]
tags: [a11y]
since: 0.1.0
---

The `<title>` element has text.

**Why:** The title names the page in tabs, history, bookmarks, search results and screen readers.

```codelaw
native: html/empty-title
```

## How to fix

1. Write a short title that says what the page is.

## Examples

### HTML

#### Don't

```html
<title></title>
<h1>Release notes</h1>
```

#### Fixed

```html
<title>Release notes</title>
<h1>Release notes</h1>
```

#### Do

```html
<title>Pricing — Acme</title>
```

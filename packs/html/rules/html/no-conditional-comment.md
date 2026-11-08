---
id: html/no-conditional-comment
title: "No Internet Explorer conditional comments"
languages: [html]
fix: auto
fix_reason: "no browser codelaw targets reads conditional comments: a comment hidden from them goes, and the markers around content every browser shows go, the content kept, so the page renders the same"
stability: preview
ported_from: [html-validate/no-conditional-comment]
since: 0.1.0
---

The document has no conditional comments (`<!--[if IE]>…<![endif]-->`).

**Why:** Only Internet Explorer 9 and older read them; every current browser treats them as comments, so the markup inside is dead.

```codelaw
native: html/no-conditional-comment
```

## How to fix

1. Delete the conditional comment, or move markup that is still needed out of it.

## Examples

### HTML

#### Don't

```html
<!--[if lt IE 9]><script src="html5shiv.js"></script><![endif]-->
<p>Hello</p>
```

#### Do

```html
<p>Hello</p>
```

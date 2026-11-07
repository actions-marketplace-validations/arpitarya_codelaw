---
id: html/meta-refresh
title: "Meta refresh redirects at once"
languages: [html]
fix: suggest
fix_reason: "the choices are making the refresh instant, when it names a URL, or removing it; either changes when, or whether, the page moves on"
stability: preview
options:
  allowLongDelay: { type: "boolean", default: false, description: "allow delays over 20 hours (72,000 seconds)" }
ported_from: [html-validate/meta-refresh]
tags: [a11y]
since: 0.1.0
---

A `<meta http-equiv="refresh">` redirects immediately (`0;url=…`); it doesn't wait, and doesn't reload the page.

**Why:** A timed refresh moves on before slower readers and screen-reader users have finished; a reload without a URL repeats forever.

```codelaw
native: html/meta-refresh
```

## How to fix

1. Make the refresh instant with `0;url=…`.
2. Or replace it with a link the user follows when ready.

## Examples

### HTML

#### Don't

```html
<meta http-equiv="refresh" content="5;url=/next">
<title>Moved</title>
```

#### Fixed (instant)

```html
<meta http-equiv="refresh" content="0;url=/next">
<title>Moved</title>
```

#### Fixed (remove)

```html
<title>Moved</title>
```

#### Do

```html
<meta http-equiv="refresh" content="0;url=/next">
```

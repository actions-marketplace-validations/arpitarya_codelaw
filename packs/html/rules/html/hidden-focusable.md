---
id: html/hidden-focusable
title: "Focusable elements are not aria-hidden"
languages: [html]
fix: suggest
fix_reason: "the choices are `tabindex=\"-1\"` (out of the tab order), removing `aria-hidden` (shown to screen readers) or `inert` (neither); each changes who can reach it"
stability: preview
ported_from: [html-validate/hidden-focusable]
tags: [a11y]
since: 0.1.0
---

An element keyboard focus can reach — a link, a form control, anything with `tabindex="0"` — is not `aria-hidden`, itself or through an ancestor.

**Why:** Keyboard users land on it while screen readers say nothing, so blind keyboard users reach a silent stop.

```codelaw
native: html/hidden-focusable
```

## How to fix

1. Remove `aria-hidden` if the element should be announced.
2. Otherwise take it out of the tab order with `tabindex="-1"`, or hide it from everyone with `hidden` or `inert`.

## Examples

### HTML

#### Don't

```html
<a href="/home" aria-hidden="true">Home</a>
```

#### Fixed (tabindex -1)

```html
<a tabindex="-1" href="/home" aria-hidden="true">Home</a>
```

#### Fixed (remove aria-hidden)

```html
<a href="/home">Home</a>
```

#### Fixed (inert)

```html
<a inert href="/home" aria-hidden="true">Home</a>
```

#### Do

```html
<a href="/home" aria-hidden="true" tabindex="-1">Home</a>
```

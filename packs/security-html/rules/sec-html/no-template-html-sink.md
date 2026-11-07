---
id: sec-html/no-template-html-sink
title: "No template binding that writes HTML"
languages: [html]
fix: suggest
fix_reason: "the HTML binding becomes its text binding (`[innerHTML]` to `[textContent]`, `v-html` to `v-text`, `x-html` to `x-text`), so the value shows as text; markup it was meant to carry then shows escaped. `[outerHTML]` has no text twin and is manual"
fix_exception: security
stability: preview
security: { cwe: [79], owasp: ["A05:2025"], severity: high }
since: 0.2.0
---

A template does not bind a value as HTML: Angular `[innerHTML]` and `[outerHTML]`, Vue `v-html`, Alpine `x-html`.

**Why:** The value is written into the page as markup, so any part of it a user controls is cross-site scripting.

```codelaw
native: sec-html/no-template-html-sink
```

## How to fix

1. Bind text instead (`{{ value }}`, `v-text`, `x-text`, `[textContent]`).
2. If it must be HTML, sanitise it with a maintained library such as DOMPurify and bind the sanitised result.

## Examples

### HTML

#### Don't

```html
<div [innerHTML]="comment"></div>
```

#### Fixed

```html
<div [textContent]="comment"></div>
```

#### Do

```html
<div>{{ comment }}</div>
```

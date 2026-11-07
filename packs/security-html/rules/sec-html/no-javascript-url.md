---
id: sec-html/no-javascript-url
title: "No javascript: URLs"
languages: [html]
fix: suggest
fix_reason: "an `<a href=\"javascript:void(0)\">` runs nothing and becomes `<button type=\"button\">`, its other attributes kept; it then looks like a button until styled. A `javascript:` URL that runs script is manual"
fix_exception: security
stability: preview
security: { cwe: [79], owasp: ["A05:2025"], severity: high }
since: 0.2.0
---

An `href`, `src`, `action`, `formaction`, `xlink:href` or `<object data>` is never a `javascript:` URL.

**Why:** A `javascript:` URL runs script from an attribute, so it is cross-site scripting waiting for one interpolated value, and a strict Content Security Policy blocks it.

```codelaw
native: sec-html/no-javascript-url
```

## How to fix

1. Replace a `javascript:` link with a `<button type="button">` and attach the listener with `addEventListener`.
2. Point `href`, `src`, `action` or `formaction` at a real URL.

## Examples

### HTML

#### Don't

```html
<a href="javascript:void(0)">Menu</a>
```

#### Fixed

```html
<button type="button">Menu</button>
```

#### Do

```html
<button type="button" id="menu">Menu</button>
```

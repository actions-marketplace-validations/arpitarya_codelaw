---
id: sec-html/no-srcdoc-script
title: "An iframe's srcdoc holds no script"
languages: [html]
fix: suggest
fix_reason: "the choices are an empty `sandbox` (the default), or `sandbox=\"allow-scripts\"` when the framed document must run; either gives it an opaque origin, and `allow-same-origin` is never offered"
fix_exception: security
stability: preview
security: { cwe: [79], owasp: ["A05:2025"], severity: high }
since: 0.2.0
---

An `<iframe srcdoc>` does not contain a `<script>`, an event handler or a `javascript:` URL.

**Why:** `srcdoc` content runs with the origin of the page that holds it unless the iframe is sandboxed, so script in it is script in the page.

```codelaw
native: sec-html/no-srcdoc-script
```

## How to fix

1. Move the document to its own file and load it with `src`.
2. Add `sandbox` without `allow-same-origin`.

## Examples

### HTML

#### Don't

```html
<iframe srcdoc="&lt;script&gt;track()&lt;/script&gt;"></iframe>
```

#### Fixed (sandbox)

```html
<iframe sandbox srcdoc="&lt;script&gt;track()&lt;/script&gt;"></iframe>
```

#### Fixed (sandbox, scripts allowed)

```html
<iframe sandbox="allow-scripts" srcdoc="&lt;script&gt;track()&lt;/script&gt;"></iframe>
```

#### Do

```html
<iframe sandbox src="/track.html"></iframe>
```

---
id: html/require-csp-nonce
title: "Inline scripts and styles carry a nonce"
languages: [html]
fix: manual
fix_reason: "the nonce comes from the server for each response"
fix_exception: value
stability: preview
ported_from: [html-validate/require-csp-nonce]
since: 0.5.0
---

An inline `<script>` or a `<style>` has a `nonce` attribute.

**Why:** A Content-Security-Policy with a nonce blocks those without one.

```codelaw
native: html/require-csp-nonce
```

## How to fix

1. Have the server put the response's nonce on the element.

## Examples

### HTML

#### Don't

```html
<script>run();</script>
```

#### Do

```html
<script nonce="{{ nonce }}">run();</script>
```

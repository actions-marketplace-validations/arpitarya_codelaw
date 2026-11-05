---
id: html/no-unknown-elements
title: "No element HTML doesn't define"
languages: [html]
fix: manual
fix_reason: "which element was meant is the author's to say"
fix_exception: value
stability: preview
ported_from: [html-validate/no-unknown-elements]
since: 0.5.0
---

An element is an HTML element, or a custom element (its name holds a `-`).

**Why:** Browsers treat an unknown element as a plain inline box.

```codelaw
native: html/no-unknown-elements
```

## How to fix

1. Fix the name, or give a custom element a dashed name.

## Examples

### HTML

#### Don't

```html
<sectoin>x</sectoin>
```

#### Do

```html
<section>x</section>
```

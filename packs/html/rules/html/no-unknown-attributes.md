---
id: html/no-unknown-attributes
title: "No attribute the element doesn't have"
languages: [html]
fix: manual
fix_reason: "which attribute was meant is the author's to say"
fix_exception: value
stability: preview
ported_from: [html-validate/no-unknown-attributes]
since: 0.5.0
---

An attribute is one the element (or every element) defines; `data-*`, `aria-*` and event handlers are allowed, and custom elements are not judged.

**Why:** A misspelt attribute does nothing.

```codelaw
native: html/no-unknown-attributes
```

## How to fix

1. Fix the name, or use a `data-` attribute.

## Examples

### HTML

#### Don't

```html
<img scr="a.png" alt="">
```

#### Do

```html
<img src="a.png" alt="">
```

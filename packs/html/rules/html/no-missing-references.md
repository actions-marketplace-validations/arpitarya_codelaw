---
id: html/no-missing-references
title: "Id references point at an element"
languages: [html]
fix: suggest
fix_reason: "the choices are the existing ids within two edits, nearest first, or removing the reference; which element was meant is the reviewer's call"
fix_exception: value
stability: preview
ported_from: [html-validate/no-missing-references]
since: 0.1.0
---

Attributes that refer to ids — `for`, `form`, `list`, `headers`, `aria-labelledby`, `aria-describedby`, `aria-controls` and the others — name ids that exist in the document.

**Why:** A reference to a missing id silently does nothing: the label, description or relation is lost.

```codelaw
native: html/no-missing-references
```

## How to fix

1. Correct the reference, or add the element it should point at.

## Examples

### HTML

#### Don't

```html
<input type="text" aria-describedby="hint">
```

#### Fixed

```html
<input type="text">
```

#### Do

```html
<input type="text" aria-describedby="hint">
<p id="hint">Use your work address.</p>
```

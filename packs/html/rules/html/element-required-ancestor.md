---
id: html/element-required-ancestor
title: "Elements are inside the ancestor they need"
languages: [html]
fix: suggest
fix_reason: "the choices are changing the parent to the ancestor it needs (when the children are all it holds) or wrapping the run of them in one; either changes the markup's structure, which the reviewer confirms"
fix_exception: restructure
stability: preview
ported_from: [html-validate/element-required-ancestor]
since: 0.1.0
---

Elements that only work inside a given ancestor are there: `<area>` in a `<map>`, `<dt>` and `<dd>` in a `<dl>`.

**Why:** Outside it they have no meaning and are rendered as plain text, or not at all.

```codelaw
native: html/element-required-ancestor
```

## How to fix

1. Wrap the elements in the ancestor the message names.
2. Or replace them with elements that make sense where they are.

## Examples

### HTML

#### Don't

```html
<div>
  <dt>Term</dt>
  <dd>Definition</dd>
</div>
```

#### Fixed (change the parent)

```html
<dl>
  <dt>Term</dt>
  <dd>Definition</dd>
</dl>
```

#### Fixed (wrap)

```html
<div>
  <dl><dt>Term</dt>
  <dd>Definition</dd></dl>
</div>
```

#### Do

```html
<dl>
  <dt>Term</dt>
  <dd>Definition</dd>
</dl>
```

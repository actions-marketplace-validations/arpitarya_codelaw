---
id: html/svg-focusable
title: "`<svg>` says it isn't focusable"
languages: [html]
fix: auto
stability: preview
ported_from: [html-validate/svg-focusable]
since: 0.5.0
---

An inline `<svg>` has `focusable="false"`.

**Why:** Older browsers put an `<svg>` in the tab order, so keyboard users stop on a picture.

```codelaw
native: html/svg-focusable
```

## Examples

### HTML

#### Don't

```html
<svg viewBox="0 0 1 1"></svg>
```

#### Do

```html
<svg focusable="false" viewBox="0 0 1 1"></svg>
```

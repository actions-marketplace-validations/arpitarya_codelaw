---
id: html/no-deprecated-attr
title: "No obsolete attributes"
languages: [html]
fix: auto
fix_reason: "auto for an attribute no browser acts on (`version`, `language`, `profile`, …), which goes; a presentational one becomes the inline style it stood for, as a suggestion; any other is manual"
fix_exception: restructure
stability: preview
ported_from: [html-validate/no-deprecated-attr]
since: 0.1.0
---

Elements don't use attributes HTML has made obsolete, such as `align`, `bgcolor`, `border` on `<img>`, or `name` on `<a>`.

**Why:** Browsers support them unevenly; CSS and current attributes do the same job reliably.

```codelaw
native: html/no-deprecated-attr
```

## How to fix

1. Replace presentational attributes with a class and CSS.
2. Replace other obsolete attributes with their current equivalent (for example `id` for `<a name>`).

## Examples

### HTML

#### Don't

```html
<img src="a.png" alt="Chart" align="left">
```

#### Fixed

```html
<img src="a.png" alt="Chart" style="float: left">
```

#### Do

```html
<img src="a.png" alt="Chart" class="float-left">
```

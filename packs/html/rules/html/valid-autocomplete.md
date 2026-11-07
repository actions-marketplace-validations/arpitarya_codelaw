---
id: html/valid-autocomplete
title: "autocomplete values are valid"
languages: [html]
fix: suggest
fix_reason: "the field names within two edits of an unknown token are choices, nearest first; a field that doesn't fit its input type, or a token with none near, is manual"
fix_exception: value
stability: preview
ported_from: [html-validate/valid-autocomplete]
tags: [a11y]
since: 0.1.0
---

`autocomplete` holds a valid token list: optional `section-*`, then `shipping`/`billing`, then `home`/`work`/… for contact fields, then one field name that fits the input type, then optional `webauthn`. `<form>` takes only `on` or `off`.

**Why:** Browsers ignore an invalid value, so autofill and password managers can't help the user.

```codelaw
native: html/valid-autocomplete
```

## How to fix

1. Pick the field name that describes the data (the WHATWG autofill list).
2. Order the tokens as described above, or use `off`.

## Examples

### HTML

#### Don't

```html
<input type="email" name="email" autocomplete="emial">
```

#### Fixed

```html
<input type="email" name="email" autocomplete="email">
```

#### Do

```html
<input type="email" name="email" autocomplete="email">
<input type="number" name="n" autocomplete="off">
```

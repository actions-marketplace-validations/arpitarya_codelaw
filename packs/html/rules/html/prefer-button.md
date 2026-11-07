---
id: html/prefer-button
title: "Buttons are <button> elements"
languages: [html]
fix: suggest
fix_reason: "`<input type=\"submit\" value=\"V\">` becomes `<button type=\"submit\" value=\"V\">V</button>`, every attribute kept; styles that target `input` stop matching. An image input, or one with no value, is manual"
fix_exception: restructure
stability: preview
ported_from: [html-validate/prefer-button]
since: 0.1.0
---

Buttons are written as `<button>`, not `<input type="button">`, `submit`, `reset` or `image`.

**Why:** `<button>` can hold markup and is styled and labelled more consistently.

```codelaw
native: html/prefer-button
```

## How to fix

1. Replace the `<input>` with a `<button>` of the same type, moving `value` into the content.
2. Update the CSS selectors and scripts that targeted the input.

## Examples

### HTML

#### Don't

```html
<input type="submit" value="Send">
```

#### Fixed

```html
<button type="submit" value="Send">Send</button>
```

#### Do

```html
<button type="submit">Send</button>
```

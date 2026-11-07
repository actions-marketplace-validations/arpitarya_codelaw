---
id: html/attribute-misuse
title: "Attributes are used only where they take effect"
languages: [html]
fix: suggest
fix_reason: "the choices are removing the attribute, which does nothing here, or setting what it needs to work (`type=\"submit\"`); only the author knows which was meant"
stability: preview
ported_from: [html-validate/attribute-misuse]
since: 0.1.0
---

An attribute that only works in a given context is used there: `formaction` on a submit button, `target` on a link with `href`, `coords` with a `shape` that needs them, and similar.

**Why:** Outside that context the browser ignores the attribute, so the author's intent is lost.

```codelaw
native: html/attribute-misuse
```

## How to fix

1. Read the message: it names the attribute and what it needs.
2. Add the missing context (for example `href` or `type="submit"`), or remove the attribute.

## Examples

### HTML

#### Don't

```html
<button type="button" formaction="/save">Save</button>
```

#### Fixed (remove)

```html
<button type="button">Save</button>
```

#### Fixed (change the context)

```html
<button type="submit" formaction="/save">Save</button>
```

#### Do

```html
<button type="submit" formaction="/save">Save</button>
```

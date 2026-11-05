---
id: html/allowed-links
title: "Only the kinds of link the project allows"
languages: [html]
fix: manual
fix_reason: "which address to link to is the author's to write"
fix_exception: value
stability: preview
options:
  allow_external: { type: "boolean", default: true, description: "links to other origins" }
  allow_relative: { type: "boolean", default: true, description: "`a/b` links" }
  allow_absolute: { type: "boolean", default: true, description: "`/a/b` links" }
ported_from: [html-validate/allowed-links]
since: 0.5.0
---

With `allow_external`, `allow_relative` or `allow_absolute` set to `false`, links and sources of that kind are reported. All three are `true` by default, so the rule does nothing until one is turned off.

**Why:** A site served from several paths needs relative links, or none.

```codelaw
native: html/allowed-links
```

## How to fix

1. Rewrite the address in an allowed form.

## Examples

### HTML

#### Don't

```html options={"allow_external":false}
<a href="https://example.com/">x</a>
```

#### Do

```html options={"allow_external":false}
<a href="/x">x</a>
```

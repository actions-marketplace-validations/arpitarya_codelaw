---
id: md/link-image-style
title: "Links and images in the styles the project allows"
languages: [markdown]
fix: manual
fix_reason: "rewriting a link in another style needs a label or a URL the author chooses"
fix_exception: value
stability: preview
options:
  autolink: { type: "boolean", default: true, description: "`<https://…>`" }
  inline: { type: "boolean", default: true, description: "`[text](url)`" }
  full: { type: "boolean", default: true, description: "`[text][label]`" }
  collapsed: { type: "boolean", default: true, description: "`[label][]`" }
  shortcut: { type: "boolean", default: true, description: "`[label]`" }
  url_inline: { type: "boolean", default: true, description: "`[url](url)`" }
ported_from: [markdownlint/MD054]
since: 0.5.0
---

With `autolink`, `inline`, `full`, `collapsed`, `shortcut` or `url_inline` set to `false`, links and images of that style are reported. All are allowed by default.

**Why:** One style of link reads alike across documents.

```codelaw
native: md/link-image-style
```

## How to fix

1. Rewrite the link in an allowed style.

## Examples

### Markdown

#### Don't

```md options={"autolink":false}
See <https://example.com>.
```

#### Do

```md options={"autolink":false}
See [the site](https://example.com).
```

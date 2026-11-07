---
id: md/no-empty-link-text
title: "A Markdown link has text"
languages: [markdown]
fix: manual
fix_reason: "the link's text says where it goes, which only the author can write"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-unicorn/no-empty-link-text]
since: 0.5.0
---

An inline link whose label is empty or only spaces — `[](https://example.com)`, `[   ](…)` — gives a screen reader nothing to announce and a reader nothing to click. Upstream also treats an image with empty alternative text as an empty label; that is `md/no-alt-text`'s, and not reported here. `md/no-empty-links` checks the destination.

**Why:** A link with no text is invisible or meaningless to anyone not reading the URL.

```codelaw
match:
  kind: inline_link
  any:
    - not: { has: { kind: link_text } }
    - has: { kind: link_text, regex: '^\s*$' }
message: "Give this link text that says where it goes"
```

## How to fix

1. Write the label between the brackets: `[the release notes](https://example.com)`.

## Examples

### Markdown

#### Don't

```md
See [](https://example.com/notes).
```

#### Do

```md
See [the release notes](https://example.com/notes).
```

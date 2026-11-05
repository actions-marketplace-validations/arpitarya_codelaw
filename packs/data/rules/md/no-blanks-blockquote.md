---
id: md/no-blanks-blockquote
title: "No blank line inside a blockquote"
languages: [markdown]
fix: suggest
fix_reason: "joining the two quotes makes them one; if they were meant apart, put text between them instead"
stability: preview
ported_from: [markdownlint/MD028]
since: 0.5.0
---

Two blockquotes aren't separated only by a blank line.

**Why:** Renderers disagree whether that is one quote or two.

```codelaw
native: md/no-blanks-blockquote
```

## How to fix

1. Mark the blank line with `>`, or put text between the quotes.

## Examples

### Markdown

#### Don't

```md
> one

> two
```

#### Do

```md
> one
>
> two
```

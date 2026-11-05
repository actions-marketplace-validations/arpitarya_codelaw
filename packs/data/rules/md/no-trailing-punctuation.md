---
id: md/no-trailing-punctuation
title: "No punctuation at the end of a heading"
languages: [markdown]
fix: auto
stability: preview
options:
  punctuation: { type: "string", default: ".,;:!。，；：！", description: "the characters a heading may not end with" }
ported_from: [markdownlint/MD026]
since: 0.5.0
---

A heading doesn't end with `.`, `,`, `;`, `:` or `!` (the `punctuation` option sets them); a question mark is allowed.

**Why:** A heading is a title, not a sentence.

```codelaw
native: md/no-trailing-punctuation
```

## Examples

### Markdown

#### Don't

```md
# Setup:

Text.
```

#### Do

```md
# Setup

Text.
```

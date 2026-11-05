---
id: md/required-headings
title: "Documents follow the required outline"
languages: [markdown]
fix: manual
fix_reason: "the missing section is the author's to write"
fix_exception: authored
stability: preview
options:
  headings: { type: "string[]", default: [], description: "the required headings, in order, with their `#` level; `*` and `+` match runs" }
ported_from: [markdownlint/MD043]
since: 0.5.0
---

The document's headings follow `headings`, a list such as `["# Title", "## Usage", "*"]` (`*` any headings, `+` at least one). With no option set, the rule does nothing.

**Why:** A set of documents keeps one structure.

```codelaw
native: md/required-headings
```

## How to fix

1. Add or rename the headings to match the outline.

## Examples

### Markdown

#### Don't

```md options={"headings":["#\u0020Title","##\u0020Usage"]}
# Title

## Install
```

#### Do

```md options={"headings":["#\u0020Title","##\u0020Usage"]}
# Title

## Usage
```

---
id: md/proper-names
title: "Proper names are spelled as the project spells them"
languages: [markdown]
fix: auto
stability: preview
options:
  names: { type: "string[]", default: [], description: "the names, as they are spelled" }
  code_blocks: { type: "boolean", default: true, description: "check fenced and indented code blocks too" }
ported_from: [markdownlint/MD044]
since: 0.5.0
---

A name in `names` (`JavaScript`, `GitHub`) is written with its capitalisation, outside code and links (`code_blocks: false` also skips code blocks). With no option set, the rule does nothing.

**Why:** Consistent names read as care.

```codelaw
native: md/proper-names
```

## Examples

### Markdown

#### Don't

```md options={"names":["JavaScript"]}
Written in javascript.
```

#### Do

```md options={"names":["JavaScript"]}
Written in JavaScript.
```

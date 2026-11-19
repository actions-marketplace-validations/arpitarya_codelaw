---
id: md/require-frontmatter-fields
title: "A Markdown file's front matter carries the fields the project lists"
languages: [markdown]
fix: manual
fix_reason: "the value of a field is the author's"
fix_exception: value
stability: preview
options:
  fields: { type: "string[]", default: [], description: "the front matter fields every file needs, such as `title` and `description`" }
ported_from: [eslint-plugin-unicorn/require-frontmatter-fields]
since: 0.5.2
---

A Markdown file starts with front matter that has every field in `fields`: `fields = ["title", "description"]` reports a file with no front matter, and one whose front matter has no `description`. A field counts when it is a key at the top of the block and has a value; `description:` with nothing after it, `""`, `[]`, `{}` and `null` are missing. YAML (`---`) and TOML (`+++`) front matter are both read; TOML only before its first table. The finding is one per file, on the front matter's first line (on the file's first line when there is none), and lists every field that is missing. An empty file is not reported. Behaviour of eslint-plugin-unicorn 79.0.0's `require-frontmatter-fields`, read and reimplemented.

With no `fields` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on under `[rules]` or in a `[scope.<name>]` for the folder whose pages need the fields (`files = ["docs/**/*.md"]`).

In `codelaw.toml`, under `[rules]`: `"md/require-frontmatter-fields" = { severity = "warn", options = { fields = ["title", "description"] } }`.

**Why:** A documentation site or a search index reads its page titles and summaries from front matter; a page without them shows up blank.

```codelaw
native: md/require-frontmatter-fields
```

## How to fix

1. Add the missing fields to the file's front matter, or the front matter itself, with the values they need.
2. If the file is not one of the pages the fields are for, scope the rule away from it.

## Examples

### Markdown

#### Don't

```md options={"fields":["title","description"]}
---
title: Setup
---

# Setup
```

#### Do

```md options={"fields":["title","description"]}
---
title: Setup
description: Install and configure the tool
---

# Setup
```

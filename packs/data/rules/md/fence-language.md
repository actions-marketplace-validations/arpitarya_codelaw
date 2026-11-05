---
id: md/fence-language
title: "Fenced code blocks name their language"
languages: [markdown]
fix: suggest
fix_reason: "the reviewer picks the language the block holds; `text` is always accepted, `sh` and `json` are offered for the common cases"
stability: preview
ported_from: [markdownlint/MD040]
since: 0.1.0
---

A fenced code block names its language after the opening fence: ` ```ts `, ` ```sh `, or ` ```text ` for plain output.

**Why:** The language drives syntax highlighting, and lets tools — codelaw's `lint_code_blocks` among them — check the code inside.

```codelaw
match:
  kind: fenced_code_block_delimiter
  inside:
    kind: fenced_code_block
    not:
      has: { kind: info_string }
  not:
    follows: { kind: fenced_code_block_delimiter, stopBy: end }
message: "Name the code's language after the opening fence, or `text` for plain output"
fix:
  choices:
    - label: plain text
      default: true
      fix:
        - insert-after: "text"
    - label: shell
      fix:
        - insert-after: "sh"
    - label: json
      fix:
        - insert-after: "json"
```

## How to fix

1. Work out what the block holds.
2. Add its language after the opening fence, or `text` when it is plain output.

## Examples

### Markdown

#### Don't

````md
```
npm install codelaw
```
````

#### Fixed (plain text)

````md
```text
npm install codelaw
```
````

#### Fixed (shell)

````md
```sh
npm install codelaw
```
````

#### Fixed (json)

````md
```json
npm install codelaw
```
````

#### Do

````md
```sh
npm install codelaw
```
````

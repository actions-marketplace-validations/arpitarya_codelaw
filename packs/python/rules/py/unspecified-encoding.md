---
id: py/unspecified-encoding
title: "open() in text mode names its encoding"
languages: [python]
fix: suggest
fix_reason: "UTF-8 replaces the locale's encoding, which differs on some systems"
stability: preview
ported_from: [ruff/PLW1514]
since: 0.5.0
---

`open()` in text mode passes `encoding=`.

**Why:** Without it the encoding is the locale's, so the same file reads differently on another machine.

```codelaw
match:
  pattern: open($$$A)
  has:
    field: arguments
    not: {has: {any: [{kind: keyword_argument, has: {field: name, regex: '^(encoding|mode)$'}}, {kind: dictionary_splat}, {kind: list_splat}, {kind: string, nthChild: 2, regex: 'b'}]}}
    has: {nthChild: 1}
message: "Name the encoding: `encoding=\"utf-8\"`"
fix: 'open($$$A, encoding="utf-8")'
```

## Examples

### Python

#### Don't

```python
f = open("notes.txt")
```

#### Do

```python
f = open("notes.txt", encoding="utf-8")
```

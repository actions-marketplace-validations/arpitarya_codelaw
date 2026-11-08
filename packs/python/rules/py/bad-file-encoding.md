---
id: py/bad-file-encoding
title: "A source file is UTF-8"
languages: [python]
fix: manual
fix_reason: "the file is re-encoded as UTF-8, which every byte outside ASCII changes"
fix_exception: restructure
stability: preview
ported_from: [pylint/bad-file-encoding]
since: 0.5.0
---

A file does not declare a source encoding other than UTF-8 in a `# -*- coding: … -*-` comment.

**Why:** UTF-8 is Python's default; another encoding makes the file hard to read with every other tool.

```codelaw
match:
  kind: comment
  regex: '^#.*coding[:=]\s*([-\w.]+)'
  all:
    - not: {regex: '(?i)coding[:=]\s*(utf-?8|utf_8)\b'}
    - not: {follows: {stopBy: end, not: {kind: comment}}}
message: "Write the file in UTF-8"
```

## How to fix

1. Re-save the file as UTF-8 and remove the coding line.

## Examples

### Python

#### Don't

```python
# -*- coding: latin-1 -*-
x = 1
```

#### Do

```python
# -*- coding: utf-8 -*-
x = 1
```

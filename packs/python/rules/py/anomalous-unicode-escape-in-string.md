---
id: py/anomalous-unicode-escape-in-string
title: "A bytes literal has no `\\u`, `\\U` or `\\N` escape"
languages: [python]
fix: auto
fix_exception: value
stability: preview
ported_from: [pylint/anomalous-unicode-escape-in-string]
since: 0.5.0
---

In a bytes literal, `\u`, `\U` and `\N{…}` are not escapes: `b"\u0041"` is six bytes, a backslash and `u0041`, not `b"A"`. When the literal has no other escape, the fix makes it raw (`rb"\u0041"`), which keeps its bytes exactly and makes plain that the backslash is literal. With other escapes beside it the finding is `manual`: whether the author wanted the backslash, or a `str`, is theirs to say.

**Why:** The literal looks like it holds a character and holds the escape's spelling instead; Python warns about it and will refuse it.

```codelaw
match:
  kind: string
  regex: '^[bB]["'']'
  has: {kind: string_content, regex: '(^|[^\\])(\\\\)*\\[uUN]'}
  any:
    - all:
        - not: {has: {kind: escape_sequence, regex: '^\\[^uUN]'}}
        - pattern: $RAW
    - has: {kind: escape_sequence, regex: '^\\[^uUN]'}
message: "`\\u`, `\\U` and `\\N` are not escapes in bytes"
fix:
  - replace: { target: $RAW, with: "r$RAW" }
```

## How to fix

1. If the bytes are what you meant, make the literal raw or double the backslash.
2. If you meant the character, write a `str` and encode it, or write the byte itself (`b"\x41"`).

## Examples

### Python

#### Don't

```python
marker = b"\u00e9"
```

#### Do

```python
marker = rb"\u00e9"
```

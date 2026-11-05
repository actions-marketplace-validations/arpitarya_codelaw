---
id: py/unicode-kind-prefix
title: "No u prefix on strings"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP025, pylint/redundant-u-string-prefix]
since: 0.5.0
---

String literals do not carry the `u` prefix (the fix covers double-quoted strings).

**Why:** Every Python 3 string is Unicode.

```codelaw
match:
  kind: string_start
  regex: '^[uU]"$'
message: "Remove the `u` prefix"
fix: '"'
```

## Examples

### Python

#### Don't

```python
name = u"bob"
```

#### Do

```python
name = "bob"
```

---
id: py/readlines-in-for
title: "Iterate the file, not readlines()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB129]
since: 0.5.0
---

`for line in f.readlines():` is written `for line in f:`.

**Why:** The file yields its lines without reading them all into a list first.

```codelaw
match:
  kind: call
  pattern: $F.readlines()
  inside: {kind: for_statement, field: right}
message: "Iterate `$F` directly"
fix: "$F"
```

## Examples

### Python

#### Don't

```python
for line in f.readlines():
    print(line)
```

#### Do

```python
for line in f:
    print(line)
```

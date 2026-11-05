---
id: py/repeated-keyword-argument
title: "No keyword passed twice"
languages: [python]
fix: manual
fix_reason: "which value was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE1132]
since: 0.5.0
---

A call does not pass the same keyword argument twice.

**Why:** That is a SyntaxError (or a TypeError through `**`).

```codelaw
match:
  kind: keyword_argument
  has: {field: name, pattern: $N}
  follows: {stopBy: end, kind: keyword_argument, has: {field: name, pattern: $N}}
message: "`$N` is passed twice"
```

## How to fix

1. Keep one of the two.

## Examples

### Python

#### Don't

```python
connect(host="a", port=1, host="b")
```

#### Do

```python
connect(host="b", port=1)
```

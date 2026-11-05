---
id: py/redundant-open-modes
title: "No default mode in open()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP015]
since: 0.5.0
---

`open(path, "r")` is written `open(path)`.

**Why:** `"r"` (text read) is the default.

```codelaw
match:
  pattern: open($P, $M)
constraints:
  M: {regex: '^["''](r|rt|tr)["'']$'}
  P: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
message: "`$M` is the default mode"
fix: "open($P)"
```

## Examples

### Python

#### Don't

```python
f = open(path, "r")
```

#### Do

```python
f = open(path)
```

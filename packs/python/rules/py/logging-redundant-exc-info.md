---
id: py/logging-redundant-exc-info
title: "No exc_info=True on logger.exception"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/G202]
since: 0.5.0
---

`logger.exception()` is not given `exc_info=True`.

**Why:** `exception` already attaches the traceback.

```codelaw
match:
  kind: keyword_argument
  regex: '^exc_info\s*=\s*True$'
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '\.exception$'}}}
message: "`exception` already includes the traceback"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
logger.exception("failed", exc_info=True)
```

#### Do

```python
logger.exception("failed")
```

---
id: py/logging-warn
title: "logging.warning, not warn"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/G010]
since: 0.5.0
---

`logger.warn()` is written `logger.warning()`.

**Why:** `warn` is a deprecated alias, removed in Python 3.13.

```codelaw
match:
  kind: attribute
  pattern: $L.warn
  inside: {kind: call, field: function}
constraints:
  L: {regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)$'}
message: "`warn` is deprecated; call `warning`"
fix: "$L.warning"
```

## Examples

### Python

#### Don't

```python
logger.warn("disk low")
```

#### Do

```python
logger.warning("disk low")
```

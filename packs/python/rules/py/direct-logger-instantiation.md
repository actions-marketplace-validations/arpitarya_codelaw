---
id: py/direct-logger-instantiation
title: "logging.getLogger, not logging.Logger()"
languages: [python]
fix: suggest
fix_reason: "the logger joins the logging hierarchy, so configured handlers now apply"
stability: preview
ported_from: [ruff/LOG001]
since: 0.5.0
---

Loggers are created with `logging.getLogger(name)`, not `logging.Logger(name)`.

**Why:** A directly built logger is outside the hierarchy and ignores configuration.

```codelaw
match:
  pattern: logging.Logger($$$A)
message: "Use `logging.getLogger`"
fix: "logging.getLogger($$$A)"
```

## Examples

### Python

#### Don't

```python
log = logging.Logger(__name__)
```

#### Do

```python
log = logging.getLogger(__name__)
```

---
id: py/root-logger-call
title: "No logging through the root logger"
languages: [python]
fix: manual
fix_reason: "a module logger, `logging.getLogger(__name__)`, is added and every call goes through it"
fix_exception: restructure
stability: preview
ported_from: [ruff/LOG015]
since: 0.5.0
---

A module logs through its own logger, not with `logging.info(...)`, `logging.warning(...)` and the other module-level functions.

**Why:** The root logger cannot be configured per module, and its calls hide which module logged.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^logging\.(debug|info|warning|warn|error|critical|exception|log)$'}
message: "Log through `logging.getLogger(__name__)`"
```

## How to fix

1. Add `logger = logging.getLogger(__name__)` at module level and call `logger.info(...)` and the rest on it.

## Examples

### Python

#### Don't

```python
import logging
logging.info("started")
```

#### Do

```python
import logging
logger = logging.getLogger(__name__)
logger.info("started")
```

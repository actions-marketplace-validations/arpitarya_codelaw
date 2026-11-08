---
id: py/logging-too-many-args
title: "A logging call passes no more arguments than its message has placeholders"
languages: [python]
fix: manual
fix_reason: "whether a placeholder is missing or an argument is extra is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE1205, pylint/logging-too-many-args]
since: 0.5.0
---

A `logging` call (`logging.info`, `logger.warning`, …, on `logging` or a receiver whose name contains `log`) whose message literal has fewer `%` placeholders than the positional arguments after it is reported. A message with a `%(name)s` key or a `*` width, and a call with `*args` or `**kwargs`, are not read.

**Why:** logging formats the message only when it is emitted, and then reports `not all arguments converted` instead of the message.

```codelaw
native: py/logging-too-many-args
```

## How to fix

1. Add the missing placeholder to the message, or remove the extra argument.

## Examples

### Python

#### Don't

```python
logger.info("saved %s", path, size)
```

#### Do

```python
logger.info("saved %s (%d bytes)", path, size)
```

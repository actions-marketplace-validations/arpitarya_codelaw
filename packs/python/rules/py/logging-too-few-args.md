---
id: py/logging-too-few-args
title: "A logging call passes an argument for each placeholder"
languages: [python]
fix: manual
fix_reason: "the value a missing placeholder should show is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE1206, pylint/logging-too-few-args]
since: 0.5.0
---

A `logging` call whose message literal has more `%` placeholders than the positional arguments after it is reported, when it passes at least one and no keyword argument. A message with no arguments is logged as written, so it is not read.

**Why:** logging formats the message only when it is emitted, and then reports `not enough arguments for format string` instead of the message.

```codelaw
native: py/logging-too-few-args
```

## How to fix

1. Pass the missing argument, or remove its placeholder from the message.

## Examples

### Python

#### Don't

```python
logger.info("saved %s in %s", path)
```

#### Do

```python
logger.info("saved %s in %s", path, folder)
```

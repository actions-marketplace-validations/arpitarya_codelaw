---
id: py/logging-string-format
title: "No str.format in a logging call"
languages: [python]
fix: manual
fix_reason: "turning `{}` fields into `%s` with arguments rewrites the format string"
fix_exception: restructure
stability: preview
ported_from: [ruff/G001, pylint/logging-format-interpolation]
since: 0.5.0
---

A logging call's message is not built with `"...".format(...)`.

**Why:** Logging formats lazily, only when the message is emitted; `.format` formats every time.

```codelaw
match:
  kind: call
  has: {field: function, kind: attribute, regex: '\.format$', has: {field: object, kind: string}}
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)\.(debug|info|warning|warn|error|exception|critical|log)$'}}}
message: "Pass the values as logging arguments instead of `.format()`"
```

## How to fix

1. Write the message with `%s` placeholders and pass the values as arguments: `logger.info("user %s", user)`.

## Examples

### Python

#### Don't

```python
logger.info("user {}".format(user))
```

#### Do

```python
logger.info("user %s", user)
```

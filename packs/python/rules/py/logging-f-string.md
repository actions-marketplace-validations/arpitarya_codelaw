---
id: py/logging-f-string
title: "No f-string in a logging call"
languages: [python]
fix: manual
fix_reason: "turning `{}` fields into `%s` with arguments rewrites the string"
fix_exception: restructure
stability: preview
ported_from: [ruff/G004]
since: 0.5.0
---

A logging call's message is not an f-string.

**Why:** Logging formats lazily, only when the message is emitted; an f-string formats every time, and loses the message template log tools group by.

```codelaw
match:
  kind: string
  regex: '^[rR]?[fF]'
  has: {kind: interpolation}
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)\.(debug|info|warning|warn|error|exception|critical)$'}}}
message: "Use `%s` placeholders and pass the values as arguments"
```

## How to fix

1. Replace each `{value}` with `%s` and pass the values as arguments after the message.

## Examples

### Python

#### Don't

```python
logger.info(f"user {name}")
```

#### Do

```python
logger.info("user %s", name)
```

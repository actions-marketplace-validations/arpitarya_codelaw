---
id: py/logging-string-concat
title: "No + in a logging message"
languages: [python]
fix: manual
fix_reason: "turning the concatenation into a format string with arguments rewrites it"
fix_exception: restructure
stability: preview
ported_from: [ruff/G003]
since: 0.5.0
---

A logging call's message is not built with `+`.

**Why:** Logging formats lazily; concatenation runs every time.

```codelaw
match:
  kind: binary_operator
  all:
  - has: {field: operator, regex: ^\+$}
  - has: {kind: string}
  - inside:
      kind: argument_list
      inside:
        kind: call
        has: {field: function, regex: ^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)\.(debug|info|warning|warn|error|exception|critical)$}
message: Use `%s` and pass the value as an argument
```

## How to fix

1. Write the message with `%s` placeholders and pass the values as arguments.

## Examples

### Python

#### Don't

```python
logger.info("user " + name)
```

#### Do

```python
logger.info("user %s", name)
```

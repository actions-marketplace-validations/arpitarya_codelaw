---
id: py/logging-format-truncated
title: "A logging format does not end in a lone %"
languages: [python]
fix: manual
fix_reason: "whether a placeholder or a literal `%` was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/logging-format-truncated]
since: 0.5.0
---

The format string of a logging call does not end with a `%` that starts no placeholder.

**Why:** `logging` reports a formatting error instead of the message.

```codelaw
match:
  kind: string
  nthChild: 1
  regex: '[^%]%["'']$|^["'']%["'']$'
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(logging|log|logger|_log|_logger|LOG|LOGGER)\.(debug|info|warning|warn|error|critical|exception)$'}}}
message: "The format ends in a lone `%`"
```

## How to fix

1. Write `%%` for a literal percent sign, or finish the placeholder.

## Examples

### Python

#### Don't

```python
log.info("disk at 90%")
```

#### Do

```python
log.info("disk at 90%%")
```

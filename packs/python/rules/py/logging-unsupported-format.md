---
id: py/logging-unsupported-format
title: "A logging format uses only known conversions"
languages: [python]
fix: manual
fix_reason: "which conversion was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/logging-unsupported-format]
since: 0.5.0
---

A `%` placeholder in a logging call's format string uses a conversion `%` formatting knows (`s r a d i o u x X e E f F g G c %`).

**Why:** `logging` reports a formatting error instead of the message.

```codelaw
match:
  kind: string
  nthChild: 1
  regex: '%[-#0 +]*(\*|\d+)?(\.(\*|\d+))?[hlL]?[^-#0 +*.\dhlLsradiouxXeEfFgGc%(]'
  not: {regex: '%%'}
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(logging|log|logger|_log|_logger|LOG|LOGGER)\.(debug|info|warning|warn|error|critical|exception)$'}}}
message: "Unsupported `%` conversion in the format"
```

## How to fix

1. Use a conversion `%` formatting supports, such as `%s` or `%d`.

## Examples

### Python

#### Don't

```python
log.info("took %y seconds", t)
```

#### Do

```python
log.info("took %s seconds", t)
```

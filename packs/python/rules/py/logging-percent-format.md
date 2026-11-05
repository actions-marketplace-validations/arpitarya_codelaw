---
id: py/logging-percent-format
title: "No % formatting in a logging call"
languages: [python]
fix: suggest
fix_reason: "the values become logging arguments; review a format that relies on a mapping"
stability: preview
ported_from: [ruff/G002, pylint/logging-not-lazy]
since: 0.5.0
---

A logging call's message is not built with `%`; the values are passed as arguments.

**Why:** Logging formats lazily; `%` formats every time, even when the message is dropped.

```codelaw
match:
  kind: binary_operator
  pattern: $S % $V
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)\.(debug|info|warning|warn|error|exception|critical)$'}}}
constraints:
  S: {kind: string}
  V: {not: {any: [{kind: tuple}, {kind: dictionary}, {kind: parenthesized_expression}]}}
message: "Pass `$V` as a logging argument"
fix: "$S, $V"
```

## Examples

### Python

#### Don't

```python
logger.info("user %s" % user)
```

#### Do

```python
logger.info("user %s", user)
```

---
id: py/logging-eager-conversion
title: "No str() on a logging argument"
languages: [python]
fix: suggest
fix_reason: "`%s` already calls `str`; it differs only if the format uses another conversion for that argument"
stability: preview
ported_from: [ruff/RUF065]
since: 0.5.0
---

An argument after a logging call's format string is not wrapped in `str()` when the format uses only `%s`.

**Why:** Logging formats lazily with `%s`; the call converts eagerly even when the message is not emitted.

```codelaw
match:
  pattern: str($X)
  inside:
    kind: argument_list
    has: {kind: string, nthChild: 1, has: {kind: string_content, regex: '%s'}, not: {has: {kind: string_content, regex: '%[^s%]'}}}
    inside: {kind: call, has: {field: function, regex: '^(logging|logger|log|LOGGER|LOG|_logger|_log|self\.logger|self\.log)\.(debug|info|warning|warn|error|exception|critical)$'}}
constraints:
  X: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
message: "`%s` converts lazily; pass `$X` without `str()`"
fix: "$X"
```

## Examples

### Python

#### Don't

```python
logger.info("user %s", str(user))
```

#### Do

```python
logger.info("user %s", user)
```

---
id: py/mixed-case-variable-in-class-scope
title: "Class attributes are not mixedCase"
languages: [python]
fix: manual
fix_reason: "the attribute is read through instances and the class, here and in other files"
fix_exception: restructure
stability: preview
ported_from: [ruff/N815]
since: 0.5.0
---

A name assigned in a class body is not mixedCase: `maxRetries = 3` is reported; `max_retries` and `MAX_RETRIES` are not.

**Why:** PEP 8 names attributes in lowercase with underscores; mixedCase is the style of neither a variable nor a constant.

```codelaw
match:
  kind: identifier
  pattern: $N
  regex: '^_*[a-z][a-z0-9_]*[A-Z]'
  any:
    - inside: {kind: assignment, field: left}
    - inside: {kind: augmented_assignment, field: left}
  inside:
    stopBy: end
    kind: block
    inside: {kind: class_definition}
  not: {inside: {kind: function_definition, stopBy: {kind: class_definition}}}
message: "Name the attribute in lowercase with underscores"
```

## How to fix

1. Rename the attribute in snake_case and update every place that reads it, in this file and others.

## Examples

### Python

#### Don't

```python
class Client:
    maxRetries = 3
```

#### Do

```python
class Client:
    max_retries = 3
```

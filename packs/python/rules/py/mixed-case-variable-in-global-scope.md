---
id: py/mixed-case-variable-in-global-scope
title: "Module variables are not mixedCase"
languages: [python]
fix: manual
fix_reason: "a module variable can be imported elsewhere, so the rename is a change across files"
fix_exception: restructure
stability: preview
ported_from: [ruff/N816, pylint/invalid-name]
since: 0.5.0
---

A name assigned at module level is not mixedCase: `defaultTimeout = 5` is reported; `default_timeout` and `DEFAULT_TIMEOUT` are not.

**Why:** PEP 8 names module variables in lowercase and constants in capitals; mixedCase is neither.

```codelaw
match:
  kind: identifier
  regex: '^_*[a-z][a-z0-9_]*[A-Z]'
  any:
    - inside: {kind: assignment, field: left, inside: {kind: expression_statement, inside: {kind: module}}}
    - inside: {kind: augmented_assignment, field: left, inside: {kind: expression_statement, inside: {kind: module}}}
message: "Name the module variable in lowercase with underscores, or in capitals for a constant"
```

## How to fix

1. Rename the variable in snake_case (or in capitals for a constant) and update every module that imports it.

## Examples

### Python

#### Don't

```python
defaultTimeout = 5
```

#### Do

```python
DEFAULT_TIMEOUT = 5
```

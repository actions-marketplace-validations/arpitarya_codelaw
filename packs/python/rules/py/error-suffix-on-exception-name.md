---
id: py/error-suffix-on-exception-name
title: "Exception classes end in Error"
languages: [python]
fix: manual
fix_reason: "the class is raised and caught by name, here and in other files"
fix_exception: restructure
stability: preview
ported_from: [ruff/N818]
since: 0.5.0
---

A class that derives from a class whose name ends in `Exception` — `Exception`, `BaseException` or one of the project's — is named with an `Error` suffix.

**Why:** PEP 8 asks that an exception that is an error carry the suffix, so `except` clauses read for what they catch.

```codelaw
match:
  kind: class_definition
  all:
    - has: {field: name, not: {regex: 'Error$'}}
    - has:
        field: superclasses
        has: {any: [{kind: identifier}, {kind: attribute}], regex: 'Exception$'}
message: "Name the exception `…Error`"
```

## How to fix

1. Rename the class with an `Error` suffix (`Timeout` → `TimeoutError`) and update every `raise` and `except` that names it.

## Examples

### Python

#### Don't

```python
class Timeout(Exception):
    pass
```

#### Do

```python
class RequestTimeoutError(Exception):
    pass
```

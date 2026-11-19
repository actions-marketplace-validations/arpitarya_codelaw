---
id: py/using-exception-groups-in-unsupported-version
title: "No `except*` below Python 3.11"
languages: [python]
fix: manual
fix_reason: "an exception group has no older spelling; catching the members one by one is a redesign the author makes"
fix_exception: restructure
finding_target: { python: { below: "3.11" } }
stability: preview
ported_from: [pylint/using-exception-groups-in-unsupported-version]
since: 0.5.0
---

An `except*` handler is reported when the project's target is below Python 3.11, the first version that has exception groups. With no target the rule does nothing.

**Why:** The file is a `SyntaxError` on the older versions the project says it supports.

```codelaw
match:
  kind: except_clause
  regex: '^except\s*\*'
message: "`except*` needs Python 3.11; the target is older"
```

## How to fix

1. Raise the project's target to 3.11, or catch the members of the group in a plain `except`.

## Examples

### Python

#### Don't

```python python=3.10
try:
    run()
except* ValueError:
    pass
```

#### Do

```python python=3.10
try:
    run()
except ValueError:
    pass
```

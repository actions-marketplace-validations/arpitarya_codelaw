---
id: py/camelcase-imported-as-lowercase
title: "A CamelCase name is not imported under a lowercase alias"
languages: [python]
fix: manual
fix_reason: "which alias to give the import is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/N813]
since: 0.5.0
---

An imported CamelCase name (a class) keeps a CamelCase alias: `from models import UserRecord as record` is reported.

**Why:** A lowercase alias makes a class read as a function or a variable.

```codelaw
match:
  kind: aliased_import
  all:
    - has: {field: name, regex: '(^|\.)_*[A-Z][A-Za-z0-9]*[a-z][A-Za-z0-9]*$', not: {regex: '[A-Za-z0-9]_[A-Za-z0-9]*$'}}
    - has: {field: alias, pattern: $A, not: {regex: '[A-Z]'}}
message: "`$A` is a class's alias; write it in CamelCase"
```

## How to fix

1. Drop the alias or write it in CamelCase, and update its uses.

## Examples

### Python

#### Don't

```python
from models import UserRecord as record
```

#### Do

```python
from models import UserRecord as Record
```

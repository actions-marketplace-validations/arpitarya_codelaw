---
id: py/camelcase-imported-as-constant
title: "A CamelCase name is not imported under a CONSTANT alias"
languages: [python]
fix: manual
fix_reason: "which alias to give the import is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/N814]
since: 0.5.0
---

An imported CamelCase name (a class) does not get an alias in capitals: `from models import UserRecord as USER_RECORD` is reported. An alias that is the name's initials is `py/camelcase-imported-as-acronym`'s.

**Why:** An alias in capitals makes a class read as a constant.

```codelaw
match:
  kind: aliased_import
  all:
    - has: {field: name, pattern: $N, regex: '(^|\.)_*[A-Z][A-Za-z0-9]*[a-z][A-Za-z0-9]*$', not: {regex: '[A-Za-z0-9]_[A-Za-z0-9]*$'}}
    - has: {field: alias, pattern: $A, regex: '^_*[A-Z][A-Z0-9_]*$'}
message: "`$A` is a class's alias; write it in CamelCase"
```

## How to fix

1. Drop the alias or write it in CamelCase, and update its uses.

## Examples

### Python

#### Don't

```python
from models import UserRecord as USER_RECORD
```

#### Do

```python
from models import UserRecord as Record
```

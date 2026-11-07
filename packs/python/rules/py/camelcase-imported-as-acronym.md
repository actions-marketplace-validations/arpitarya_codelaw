---
id: py/camelcase-imported-as-acronym
title: "A CamelCase name is not imported as its initials"
languages: [python]
fix: manual
fix_reason: "which alias to give the import is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/N817]
since: 0.5.0
---

An imported CamelCase name is not aliased to its capital letters: `from xml.etree import ElementTree as ET` is reported.

**Why:** An acronym alias reads as a constant and says nothing about what it names.

```codelaw
match:
  kind: aliased_import
  all:
    - has: {field: name, regex: '(^|\.)([A-Z])[a-z0-9]+([A-Z])[a-z0-9]+$'}
    - has: {field: alias, pattern: $A, regex: '^[A-Z]{2}$'}
message: "`$A` is an acronym alias; name the import in CamelCase"
```

## How to fix

1. Drop the alias or give it a CamelCase name, and update its uses.

## Examples

### Python

#### Don't

```python
from xml.etree import ElementTree as ET
```

#### Do

```python
from xml.etree import ElementTree
```

---
id: py/unnecessary-future-import
title: "No __future__ imports Python 3 already has"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP010]
since: 0.5.0
---

`from __future__ import` of `print_function`, `division`, `absolute_import`, `unicode_literals`, `generators`, `nested_scopes`, `with_statement` or `generator_stop` is removed.

**Why:** These are always on in Python 3.

```codelaw
match:
  kind: future_import_statement
  regex: '^from\s+__future__\s+import\s+\(?\s*(print_function|division|absolute_import|unicode_literals|generators|nested_scopes|with_statement|generator_stop)(\s*,\s*(print_function|division|absolute_import|unicode_literals|generators|nested_scopes|with_statement|generator_stop))*\s*\)?$'
message: "This feature is always on in Python 3"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
from __future__ import print_function
print(1)
```

#### Do

```python
print(1)
```

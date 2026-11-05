---
id: py/unnecessary-builtin-import
title: "No importing builtins"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP029]
since: 0.5.0
---

`from builtins import ...` and `from six.moves import ...` of names Python 3 has built in are removed.

**Why:** The names are already available.

```codelaw
match:
  kind: import_from_statement
  regex: '^from\s+(builtins|six\.moves|io)\s+import\s+(open|str|bytes|range|map|filter|zip|input|object|int|chr|dict|list|round|super|pow|next|ascii|oct|hex)(\s*,\s*(open|str|bytes|range|map|filter|zip|input|object|int|chr|dict|list|round|super|pow|next|ascii|oct|hex))*$'
  not: {regex: '^from\s+io\s+import\s+(?:[a-z, ]*[^o, ][a-z]*)'}
message: "These are builtins; remove the import"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
from builtins import str, range
x = str(1)
```

#### Do

```python
x = str(1)
```

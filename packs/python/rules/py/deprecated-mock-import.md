---
id: py/deprecated-mock-import
title: "unittest.mock, not mock"
languages: [python]
fix: suggest
fix_reason: "the standard-library module may differ from the installed backport's version"
stability: preview
ported_from: [ruff/UP026]
since: 0.5.0
---

`import mock` is written `from unittest import mock`.

**Why:** `mock` has been in the standard library since Python 3.3.

```codelaw
match:
  kind: import_statement
  regex: '^import\s+mock$'
message: "Use `from unittest import mock`"
fix: "from unittest import mock"
```

## Examples

### Python

#### Don't

```python
import mock
```

#### Do

```python
from unittest import mock
```

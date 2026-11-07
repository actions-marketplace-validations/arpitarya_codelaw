---
id: py/manual-from-import
title: "from os import path, not import os.path as path"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLR0402, pylint/consider-using-from-import]
since: 0.5.0
---

`import a.b as b` is written `from a import b`.

**Why:** The `from` form says the same with no alias.

```codelaw
match:
  kind: import_statement
  pattern: import $A.$B as $B
message: "Write `from $A import $B`"
fix: "from $A import $B"
```

## Examples

### Python

#### Don't

```python
import os.path as path
```

#### Do

```python
from os import path
```

---
id: py/reimported
title: "No module imported twice"
languages: [python]
fix: auto
stability: preview
ported_from: [pylint/reimported]
since: 0.5.0
---

An `import` statement identical to an earlier one in the same block is removed.

**Why:** The second import does nothing.

```codelaw
match:
  any: [{kind: import_statement}, {kind: import_from_statement}]
  pattern: $I
  follows: {stopBy: end, any: [{kind: import_statement}, {kind: import_from_statement}], pattern: $I}
message: "This is already imported above"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
import os
import sys
import os
```

#### Do

```python
import os
import sys
```

---
id: py/unconventional-import-alias
title: "numpy is imported as np"
languages: [python]
fix: suggest
fix_reason: "the alias and every use of it in the file are renamed"
stability: preview
ported_from: [ruff/ICN001]
since: 0.5.0
---

`import numpy as x` uses the conventional alias `np`. (Ruff checks a table of conventions — `pandas as pd`, `matplotlib.pyplot as plt`, …; numpy is covered.)

**Why:** Readers recognise the conventional alias at a glance.

```codelaw
match:
  kind: identifier
  not: {regex: '^np$'}
  pattern: $A
  inside: {kind: aliased_import, field: alias, has: {field: name, regex: '^numpy$'}}
message: "Import numpy as `np`"
fix:
  - rename-symbol-local: {target: $A, to: np}
```

## Examples

### Python

#### Don't

```python
import numpy as numeric
x = numeric.zeros(3)
```

#### Do

```python
import numpy as np
x = np.zeros(3)
```

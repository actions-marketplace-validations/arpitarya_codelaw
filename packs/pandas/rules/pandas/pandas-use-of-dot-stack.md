---
id: pandas/pandas-use-of-dot-stack
title: "melt, not stack"
languages: [python]
fix: manual
fix_reason: "`melt` takes different arguments, chosen by the author"
stability: preview
ported_from: [ruff/PD013]
since: 0.5.0
---

`.melt()` is used instead of `.stack()`.

**Why:** `melt` names its id and value columns explicitly; `stack` moves data into the index.

```codelaw
match:
  pattern: $X.stack($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `.melt()` instead of `.stack()`"
```

## How to fix

1. Rewrite with `.melt(id_vars=…, value_vars=…)`.

## Examples

### Python

#### Don't

```python
import pandas as pd
long = df.stack()
```

#### Do

```python
import pandas as pd
long = df.melt(id_vars=['d'])
```

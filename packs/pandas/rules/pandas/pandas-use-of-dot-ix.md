---
id: pandas/pandas-use-of-dot-ix
title: "No .ix"
languages: [python]
fix: suggest
fix_reason: "`.loc` (labels) or `.iloc` (positions) depends on what the index holds: both are offered"
stability: preview
ported_from: [ruff/PD007]
since: 0.5.0
---

`.ix` is not used; it was removed from pandas.

**Why:** `.ix` guessed between labels and positions and was removed in pandas 1.0.

```codelaw
match:
  pattern: $O.ix
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "`.ix` was removed; use `.loc` or `.iloc`"
fix:
  choices:
    - label: by position
      fix: "$O.iloc"
    - label: by label
      fix: "$O.loc"
```

## Examples

### Python

#### Don't

```python
import pandas as pd
row = df.ix[0]
```

#### Fixed (by position)

```python
import pandas as pd
row = df.iloc[0]
```

#### Fixed (by label)

```python
import pandas as pd
row = df.loc[0]
```

#### Do

```python
import pandas as pd
row = df.iloc[0]
```

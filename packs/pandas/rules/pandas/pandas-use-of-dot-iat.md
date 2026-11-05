---
id: pandas/pandas-use-of-dot-iat
title: ".iloc, not .iat"
languages: [python]
fix: suggest
fix_reason: "`.iloc` returns more than one value where `.iat` would fail; confirm the lookup"
stability: preview
ported_from: [ruff/PD009]
since: 0.5.0
---

Values are read with `.iloc`, not `.iat`.

**Why:** `.iloc` does everything `.iat` does; one accessor reads more consistently.

```codelaw
match:
  kind: attribute
  all:
  - pattern: $O.iat
  - inside: {kind: subscript, field: value}
  - inside:
      kind: module
      stopBy: end
      has:
        any:
        - {kind: import_statement}
        - {kind: import_from_statement}
        regex: ^(import|from)\s+pandas\b
message: Use `.iloc` instead of `.iat`
fix: "$O.iloc"
```

## Examples

### Python

#### Don't

```python
import pandas as pd
v = df.iat[0, 1]
```

#### Do

```python
import pandas as pd
v = df.iloc[0, 1]
```

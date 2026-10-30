---
id: pandas/pandas-use-of-dot-at
title: ".loc, not .at"
languages: [python]
fix: manual
fix_reason: "`.loc` returns more than one value where `.at` would fail; confirm the lookup"
stability: preview
ported_from: [ruff/PD008]
since: 0.5.0
---

Values are read with `.loc`, not `.at`.

**Why:** `.loc` does everything `.at` does and more; one accessor reads more consistently.

```codelaw
match:
  kind: attribute
  all:
  - has: {field: attribute, regex: ^at$}
  - inside: {kind: subscript, field: value}
  - inside:
      kind: module
      stopBy: end
      has:
        any:
        - {kind: import_statement}
        - {kind: import_from_statement}
        regex: ^(import|from)\s+pandas\b
message: Use `.loc` instead of `.at`
```

## How to fix

1. Replace `.at[...]` with `.loc[...]`.

## Examples

### Python

#### Don't

```python
import pandas as pd
v = df.at[0, 'a']
```

#### Do

```python
import pandas as pd
v = df.loc[0, 'a']
```

---
id: pandas/pandas-use-of-inplace-argument
title: "No inplace=True"
languages: [python]
fix: suggest
fix_reason: "the statement becomes an assignment of the result, so other references to the object no longer see the change; the reviewer confirms (offered for a call on a plain name used as a statement)"
stability: preview
ported_from: [ruff/PD002]
since: 0.5.0
---

A pandas method is not called with `inplace=True`; its result is assigned instead.

**Why:** `inplace=True` rarely saves memory, returns `None` (a common source of bugs when chained) and is being deprecated across pandas.

```codelaw
match:
  kind: keyword_argument
  regex: '^inplace\s*=\s*True$'
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
  any:
    - all:
        - pattern: $K
        - inside:
            kind: argument_list
            stopBy: neighbor
            inside:
              kind: call
              stopBy: neighbor
              all:
                - pattern: $C
                - has: { field: function, kind: attribute, has: { field: object, kind: identifier, pattern: $D } }
              inside: { kind: expression_statement, stopBy: neighbor }
    - kind: keyword_argument
message: "Assign the result instead of passing `inplace=True`"
fix:
  - remove: $K
  - insert-before: { target: $C, text: "$D = " }
```

## How to fix

1. Remove `inplace=True` and assign the result: `df = df.dropna()`.

## Examples

### Python

#### Don't

```python
import pandas as pd
df.dropna(inplace=True)
```

#### Do

```python
import pandas as pd
df = df.dropna()
```

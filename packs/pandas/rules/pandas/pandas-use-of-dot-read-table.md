---
id: pandas/pandas-use-of-dot-read-table
title: "read_csv, not read_table"
languages: [python]
fix: manual
fix_reason: "the separator argument has to be added"
fix_pending: "auto-subset"
stability: preview
ported_from: [ruff/PD012]
since: 0.5.0
---

`pd.read_csv` is used instead of `pd.read_table`.

**Why:** `read_table` is `read_csv` with a tab separator; writing `sep='\t'` says so.

```codelaw
match:
  pattern: pd.read_table($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `pd.read_csv(…, sep='\\t')` instead of `pd.read_table`"
```

## How to fix

1. Call `pd.read_csv(path, sep='\t')`.

## Examples

### Python

#### Don't

```python
import pandas as pd
df = pd.read_table('a.tsv')
```

#### Do

```python
import pandas as pd
df = pd.read_csv('a.tsv', sep='\t')
```

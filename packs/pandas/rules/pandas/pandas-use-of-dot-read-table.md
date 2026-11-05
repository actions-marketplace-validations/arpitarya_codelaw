---
id: pandas/pandas-use-of-dot-read-table
title: "read_csv, not read_table"
languages: [python]
fix: auto
fix_reason: "auto when the call passes no separator of its own (no `sep`, `delimiter`, `delim_whitespace`, second positional or splat); a call that does is reported as a suggestion"
stability: preview
ported_from: [ruff/PD012]
since: 0.5.0
---

`pd.read_csv` is used instead of `pd.read_table`.

**Why:** `read_table` is `read_csv` with a tab separator; writing `sep='\t'` says so.

```codelaw
match:
  pattern: pd.read_table($$$A)
  any:
    - all:
        - pattern: $W
        - not:
            has:
              kind: argument_list
              has:
                any:
                  - kind: dictionary_splat
                  - kind: list_splat
                  - { kind: keyword_argument, regex: '^(sep|delimiter|delim_whitespace)\s*=' }
                  - { nthChild: 2, not: { kind: keyword_argument } }
    - pattern: pd.read_table($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `pd.read_csv(…, sep='\\t')` instead of `pd.read_table`"
auto_when:
  not:
    has:
      kind: argument_list
      has:
        any:
          - kind: dictionary_splat
          - kind: list_splat
          - { kind: keyword_argument, regex: '^(sep|delimiter|delim_whitespace)\s*=' }
          - { nthChild: 2, not: { kind: keyword_argument } }
fix:
  - replace: { target: $W, with: 'pd.read_csv($$$A, sep=''\t'')' }
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

---
id: py/manual-list-copy
title: "A loop that appends every item is extend"
languages: [python]
fix: suggest
fix_reason: "the loop becomes one `extend` call; the loop variable is no longer bound after it"
stability: preview
ported_from: [ruff/PERF402]
since: 0.5.0
---

A `for` loop whose body is only `result.append(x)`, `x` being the loop variable, is written `result.extend(items)`.

**Why:** `extend` (or `list(items)`) copies in one call and says so.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: left, kind: identifier, pattern: $X}
    - has: {field: right, pattern: $Y}
    - not: {has: {kind: else_clause}}
    - not: {regex: '^async'}
    - has:
        field: body
        all:
          - has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $R.append($X)}}
          - not: {has: {nthChild: 2}}
message: "Write `$R.extend($Y)`"
fix: "$R.extend($Y)"
```

## Examples

### Python

#### Don't

```python
for u in users:
    out.append(u)
```

#### Do

```python
out.extend(users)
```

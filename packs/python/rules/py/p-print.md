---
id: py/p-print
title: "No pprint left in code"
languages: [python]
fix: suggest
fix_reason: "the call is removed; review that the output was not wanted"
stability: preview
ported_from: [ruff/T203]
since: 0.5.0
---

`pprint()` and `pprint.pprint()` debugging calls are removed.

**Why:** Debug output left in code clutters stdout.

```codelaw
match:
  kind: expression_statement
  has: {kind: call, nthChild: 1, has: {field: function, regex: '^(pprint\.)?pprint$'}}
  not: {inside: {kind: block, not: {has: {nthChild: 2}}}}
message: "Remove the `pprint` call"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f(x):
    pprint(x)
    return x
```

#### Do

```python
def f(x):
    return x
```

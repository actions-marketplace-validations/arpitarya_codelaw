---
id: py/comparison-of-constant
title: "No comparison of two constants"
languages: [python]
fix: manual
fix_reason: "what the comparison was meant to test is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLR0133, pylint/comparison-of-constants]
since: 0.5.0
---

A comparison does not have a literal on both sides: `1 == 1`, `"a" < "b"`.

**Why:** Its result is fixed when the code is written.

```codelaw
utils:
  lit: {any: [{kind: integer}, {kind: float}, {kind: string, not: {has: {kind: interpolation}}}, {kind: 'true'}, {kind: 'false'}, {kind: none}]}
match:
  kind: comparison_operator
  all:
    - has: {nthChild: 1, matches: lit}
    - has: {nthChild: 2, matches: lit}
    - not: {has: {nthChild: 3}}
message: "Both sides are constants"
```

## How to fix

1. Replace one side with the value that was meant, or the comparison with its result.

## Examples

### Python

#### Don't

```python
if 1 == 1:
    pass
```

#### Do

```python
if retries == 1:
    pass
```

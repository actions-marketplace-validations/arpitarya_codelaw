---
id: py/in-empty-collection
title: "No membership test on an empty literal"
languages: [python]
fix: manual
fix_reason: "the test is always False (or always True for `not in`); what was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/RUF060]
since: 0.5.0
---

`x in []`, `x in ()`, `x in {}` and `x in ""` are reported.

**Why:** The result is constant; the collection was probably meant to hold something.

```codelaw
match:
  kind: comparison_operator
  regex: '\s(not\s+)?in\s+(\[\s*\]|\(\s*\)|\{\s*\}|""|''''|set\(\s*\)|list\(\s*\)|dict\(\s*\)|tuple\(\s*\))$'
message: "Membership in an empty collection is constant"
```

## How to fix

1. Fill in the collection, or remove the test.

## Examples

### Python

#### Don't

```python
if mode in []:
    run()
```

#### Do

```python
if mode in ["fast", "slow"]:
    run()
```

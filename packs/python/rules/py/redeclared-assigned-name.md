---
id: py/redeclared-assigned-name
title: "No name twice in one unpacking"
languages: [python]
fix: manual
fix_reason: "which name each value was meant for is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW0128]
since: 0.5.0
---

A name appears once on the left of an unpacking assignment.

**Why:** The second binding overwrites the first; one value is lost.

```codelaw
match:
  kind: identifier
  pattern: $N
  not: {regex: '^_$'}
  follows: {stopBy: end, kind: identifier, pattern: $N}
  inside: {any: [{kind: pattern_list}, {kind: tuple_pattern}, {kind: list_pattern}], inside: {kind: assignment, field: left}}
message: "`$N` is bound twice here"
```

## How to fix

1. Rename one of the targets, or use `_` for the value you discard.

## Examples

### Python

#### Don't

```python
a, a = pair
```

#### Do

```python
a, b = pair
```

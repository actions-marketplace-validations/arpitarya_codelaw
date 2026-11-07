---
id: py/dict-init-mutate
title: "A dict is written as a literal, not filled after"
languages: [python]
fix: manual
fix_reason: "the assignments that follow move into the literal"
fix_exception: restructure
stability: preview
ported_from: [pylint/dict-init-mutate]
since: 0.5.0
---

`d = {}` followed at once by `d["key"] = value` is written as one literal (Pylint's opt-in `dict-init-mutate` extension).

**Why:** The literal shows the dict's contents in one place.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment, pattern: '$D = {}'}
  precedes: {kind: expression_statement, has: {kind: assignment, pattern: '$D[$K] = $V'}}
message: "Write the keys in the literal"
```

## How to fix

1. Move each `d[key] = value` that follows into the literal.

## Examples

### Python

#### Don't

```python
opts = {}
opts["debug"] = True
```

#### Do

```python
opts = {"debug": True}
```

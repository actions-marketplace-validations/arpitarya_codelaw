---
id: py/unreachable
title: "No code after return or raise"
languages: [python]
fix: suggest
fix_reason: "the dead statement is removed; review whether it was meant to run"
stability: preview
ported_from: [pylint/unreachable]
since: 0.5.0
---

A statement that follows `return`, `raise`, `continue` or `break` in the same block is removed.

**Why:** It never runs.

```codelaw
match:
  not: {kind: comment}
  regex: '\S'
  follows: {stopBy: end, any: [{kind: return_statement}, {kind: raise_statement}, {kind: continue_statement}, {kind: break_statement}]}
  inside: {kind: block}
message: "This code never runs"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f():
    return 1
    print("never")
```

#### Do

```python
def f():
    return 1
```

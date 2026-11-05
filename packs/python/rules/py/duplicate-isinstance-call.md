---
id: py/duplicate-isinstance-call
title: "One isinstance call with a tuple"
languages: [python]
fix: auto
fix_reason: "auto when the tested value is a plain name, so evaluating it once changes nothing"
stability: preview
ported_from: [ruff/SIM101, pylint/consider-merging-isinstance]
since: 0.5.0
---

`isinstance(x, A) or isinstance(x, B)` is written `isinstance(x, (A, B))`.

**Why:** One call says the same thing and reads as one test.

```codelaw
match:
  pattern: isinstance($X, $A) or isinstance($X, $B)
constraints:
  X: {kind: identifier}
message: "Merge into `isinstance($X, ($A, $B))`"
fix: "isinstance($X, ($A, $B))"
```

## Examples

### Python

#### Don't

```python
if isinstance(v, int) or isinstance(v, float):
    pass
```

#### Do

```python
if isinstance(v, (int, float)):
    pass
```

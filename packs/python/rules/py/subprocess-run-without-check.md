---
id: py/subprocess-run-without-check
title: "subprocess.run says check="
languages: [python]
fix: suggest
fix_reason: "`check=False` keeps today's behaviour; say `True` instead if failures should raise"
stability: preview
ported_from: [ruff/PLW1510, pylint/subprocess-run-check]
since: 0.5.0
---

`subprocess.run` is called with an explicit `check=`.

**Why:** Without it a failing command returns quietly; stating `check` makes the choice visible.

```codelaw
match:
  pattern: subprocess.run($$$A)
  has:
    field: arguments
    all:
    - not:
        has:
          kind: keyword_argument
          has: {field: name, regex: ^check$}
    - not:
        has:
          any:
          - {kind: dictionary_splat}
          - {kind: list_splat}
    - has: {nthChild: 1}
message: Say `check=True` or `check=False`
fix: subprocess.run($$$A, check=False)
```

## Examples

### Python

#### Don't

```python
subprocess.run(["ls"])
```

#### Do

```python
subprocess.run(["ls"], check=False)
```

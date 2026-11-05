---
id: py/assert-with-print-message
title: "No print() as an assert message"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF030]
since: 0.5.0
---

An `assert`'s message is the value to show, not `print(...)`.

**Why:** `print` runs, then the message is `None`; the failure shows nothing useful.

```codelaw
match:
  kind: assert_statement
  pattern: assert $C, print($M)
constraints:
  M: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
message: "`print()` returns None; pass the message itself"
fix: "assert $C, $M"
```

## Examples

### Python

#### Don't

```python
assert ok, print("not ok")
```

#### Do

```python
assert ok, "not ok"
```

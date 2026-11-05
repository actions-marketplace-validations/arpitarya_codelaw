---
id: py/misplaced-format-function
title: "format() on the string, not on print()"
languages: [python]
fix: suggest
fix_reason: "the formatted string is printed, which was meant; the call no longer raises"
stability: preview
ported_from: [pylint/misplaced-format-function]
since: 0.5.0
---

`print("{}").format(x)` is written `print("{}".format(x))`.

**Why:** `print` returns `None`, so `.format` on it raises AttributeError.

```codelaw
match:
  pattern: print($S).format($$$A)
constraints:
  S: {kind: string}
message: "Call `format` on the string"
fix: "print($S.format($$$A))"
```

## Examples

### Python

#### Don't

```python
print("total: {}").format(n)
```

#### Do

```python
print("total: {}".format(n))
```

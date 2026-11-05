---
id: py/bad-thread-instantiation
title: "Thread(target=f), not Thread(f)"
languages: [python]
fix: suggest
fix_reason: "the function becomes the `target`, which is what was meant; it was the `group` argument"
stability: preview
ported_from: [pylint/bad-thread-instantiation]
since: 0.5.0
---

`threading.Thread(f)` passes the function as `target=`.

**Why:** The first positional argument is `group`, which must be `None`; the call raises.

```codelaw
match:
  pattern: threading.Thread($F)
constraints:
  F: {not: {any: [{kind: keyword_argument}, {kind: list_splat}, {kind: dictionary_splat}, {kind: none}]}}
message: "Pass the function as `target=$F`"
fix: "threading.Thread(target=$F)"
```

## Examples

### Python

#### Don't

```python
t = threading.Thread(work)
```

#### Do

```python
t = threading.Thread(target=work)
```

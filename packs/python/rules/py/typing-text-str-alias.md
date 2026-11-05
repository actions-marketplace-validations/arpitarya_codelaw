---
id: py/typing-text-str-alias
title: "str, not typing.Text"
languages: [python]
fix: suggest
fix_reason: "the `typing` import may become unused"
stability: preview
ported_from: [ruff/UP019]
since: 0.5.0
---

`typing.Text` is written `str`.

**Why:** `Text` was an alias kept for Python 2 and is deprecated.

```codelaw
match:
  kind: identifier
  regex: '^Text$'
  inside: {kind: type, stopBy: end}
message: "Write `str`"
fix: "str"
```

## Examples

### Python

#### Don't

```python
def f(x: Text) -> None:
    pass
```

#### Do

```python
def f(x: str) -> None:
    pass
```

---
id: py/explicit-f-string-repr-conversion
title: "`!r`, not repr() inside an f-string"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF010]
since: 0.5.0
---

Inside an f-string, `repr(x)` and `ascii(x)` are written as the conversion flags `{x!r}` and `{x!a}` (the `repr` and `ascii` half of Ruff's RUF010; `py/explicit-f-string-type-conversion` is the `str` half).

**Why:** The flag says the same thing without a call.

```codelaw
match:
  kind: interpolation
  not: {has: {any: [{kind: type_conversion}, {kind: format_specifier}]}}
  has:
    field: expression
    pattern: repr($X)
constraints:
  X: {not: {any: [{kind: list_splat}, {kind: keyword_argument}]}}
message: "Write `{$X!r}` instead of calling `repr()` in the f-string"
fix: "{$X!r}"
```

## Examples

### Python

#### Don't

```python
msg = f"got {repr(value)}"
```

#### Do

```python
msg = f"got {value!r}"
```

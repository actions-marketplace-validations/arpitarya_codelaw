---
id: py/explicit-f-string-type-conversion
title: "A conversion flag, not str() inside an f-string"
languages: [python]
fix: auto
fix_reason: "auto for `str()`; `repr()` and `ascii()` are reported with the flag to write"
stability: preview
ported_from: [ruff/RUF010]
since: 0.5.0
---

Inside an f-string, `str(x)`, `repr(x)` and `ascii(x)` are written as the conversion flags `{x!s}`, `{x!r}` and `{x!a}`. The automatic fix covers `str()`; for `repr()` and `ascii()` the message names the flag.

**Why:** The flag says the same thing without a call, and the f-string reads as one expression.

```codelaw
match:
  kind: interpolation
  not: {has: {any: [{kind: type_conversion}, {kind: format_specifier}]}}
  has:
    field: expression
    pattern: str($X)
constraints:
  X: {not: {any: [{kind: string}, {kind: concatenated_string}, {kind: list_splat}, {kind: keyword_argument}]}}
message: "Write `{$X!s}` instead of calling `str()` in the f-string"
fix: "{$X!s}"
```

## Examples

### Python

#### Don't

```python
msg = f"value: {str(count)}"
```

#### Do

```python
msg = f"value: {count!s}"
```

---
id: py/percent-format-unsupported-character
title: "A `%` format uses known conversions"
languages: [python]
fix: suggest
fix_reason: "the string raises ValueError today: the choices are escaping the `%` as `%%` and replacing the conversion with `s`; neither is the default. A string with escapes is manual"
fix_exception: value
stability: preview
ported_from: [ruff/F509, pylint/bad-format-character]
since: 0.1.0
---

Each `%` placeholder ends with a conversion Python knows (`s`, `r`, `a`, `d`, `f`, `x`, …).

**Why:** An unknown conversion raises `ValueError: unsupported format character`.

```codelaw
native: py/percent-format-unsupported-character
```

## How to fix

1. Use the conversion you meant, or escape a literal percent as `%%`.

## Examples

### Python

#### Don't

```py
message = "%y" % (1,)
```

#### Fixed (escape)

```py
message = "%%y" % (1,)
```

#### Fixed (s)

```py
message = "%s" % (1,)
```

#### Do

```py
message = "%s" % (1,)
```

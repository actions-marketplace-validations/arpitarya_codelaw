---
id: py/format-invalid
title: "A `.format` string is well formed"
languages: [python]
fix: suggest
fix_reason: "the string raises ValueError today: a stray `}` is doubled; a `{` never closed is escaped as `{{` or closed as an empty field `{}`, the two choices, neither the default. A string with escapes is manual"
fix_exception: value
stability: preview
ported_from: [ruff/F521, pylint/bad-format-string]
since: 0.1.0
---

A string literal passed through `.format()` has balanced braces; literal braces are written `{{` and `}}`.

**Why:** A stray brace raises `ValueError` when the string is formatted.

```codelaw
native: py/format-invalid
```

## How to fix

1. Close the field, or double a literal brace (`{{`, `}}`).

## Examples

### Python

#### Don't

```py
message = "{".format(1)
```

#### Fixed (escape)

```py
message = "{{".format(1)
```

#### Fixed (field)

```py
message = "{}".format(1)
```

#### Do

```py
message = "{}".format(1)
```

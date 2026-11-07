---
id: py/percent-format-invalid
title: "A `%` format string is complete"
languages: [python]
fix: suggest
fix_reason: "the string raises ValueError today: the choices are escaping the `%` as `%%` (it was text) and, for a key left open, closing it as `%(key)s`; neither is the default. A string with escapes or an f-prefix is manual"
fix_exception: value
stability: preview
ported_from: [ruff/F501, pylint/truncated-format-string]
since: 0.1.0
---

A string formatted with `%` has no unfinished placeholder such as `%(name` or a trailing `%`.

**Why:** Python raises `ValueError` when the string is formatted.

```codelaw
native: py/percent-format-invalid
```

## How to fix

1. Finish the placeholder (`%(name)s`) or escape a literal percent as `%%`.

## Examples

### Python

#### Don't

```py
message = "%(name" % {"name": "Ada"}
```

#### Fixed (escape)

```py
message = "%%(name" % {"name": "Ada"}
```

#### Fixed (close)

```py
message = "%(name)s" % {"name": "Ada"}
```

#### Do

```py
message = "%(name)s" % {"name": "Ada"}
```

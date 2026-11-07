---
id: py/percent-format-expected-mapping
title: "Named `%` placeholders get a mapping"
languages: [python]
fix: suggest
fix_reason: "a tuple with one item per name becomes a dict in the placeholders' order, which assumes the two orders match; any other right side is manual, as the mapping is a value the code doesn't hold"
fix_exception: value
stability: preview
ported_from: [ruff/F502, pylint/format-needs-mapping]
since: 0.1.0
---

A `%` format string with named placeholders (`%(name)s`) is applied to a mapping, not a tuple.

**Why:** Python raises `TypeError: format requires a mapping`.

```codelaw
native: py/percent-format-expected-mapping
```

## How to fix

1. Pass a dict whose keys are the placeholder names.

## Examples

### Python

#### Don't

```py
message = "%(name)s" % ("Ada",)
```

#### Do

```py
message = "%(name)s" % {"name": "Ada"}
```

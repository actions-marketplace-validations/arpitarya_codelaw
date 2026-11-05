---
id: py/bad-string-format-character
title: "% formats use valid conversion characters"
languages: [python]
fix: manual
fix_reason: "which conversion was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE1300]
since: 0.5.0
---

A `%`-format string uses only valid conversions (`%s`, `%d`, `%r`, `%f`, …).

**Why:** An unknown conversion raises ValueError when the string is formatted.

```codelaw
match:
  kind: binary_operator
  all:
  - has: {field: operator, regex: ^%$}
  - has:
      field: left
      kind: string
      not: {regex: '^[rRbB]?[fF]'}
      has: {kind: string_content, regex: '%(\([^)]*\))?[-+ #0]*(\d+|\*)?(\.(\d+|\*))?[hlL]?[^diouxXeEfFgGcrsa%(\-+ #0-9.*hlL]'}
message: This `%` conversion character is not valid
```

## How to fix

1. Use a valid conversion such as `%s`, `%d` or `%f`; write `%%` for a literal percent.

## Examples

### Python

#### Don't

```python
msg = "%y items" % n
```

#### Do

```python
msg = "%d items" % n
```

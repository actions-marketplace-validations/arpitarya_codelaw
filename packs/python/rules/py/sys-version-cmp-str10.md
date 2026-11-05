---
id: py/sys-version-cmp-str10
title: "No comparing sys.version with a one-digit string"
languages: [python]
fix: manual
fix_reason: "the string becomes a tuple, which changes the comparison"
fix_exception: restructure
stability: preview
ported_from: [ruff/YTT302]
since: 0.5.0
---

`sys.version` is not compared with a one-character string such as `"3"`.

**Why:** Strings compare by character, so the test breaks once the major version has two digits.

```codelaw
match:
  kind: comparison_operator
  all:
  - has: {regex: ^sys\.version$}
  - has: {kind: string, regex: '^["'']\d["'']$'}
  - has: {field: operators, regex: '^[<>]=?$'}
message: Compare `sys.version_info` with a tuple
```

## How to fix

1. Compare `sys.version_info` with a tuple: `sys.version_info >= (3,)`.

## Examples

### Python

#### Don't

```python
if sys.version >= "3":
    pass
```

#### Do

```python
if sys.version_info >= (3,):
    pass
```

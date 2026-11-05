---
id: py/sys-version-cmp-str3
title: "No comparing sys.version with a string"
languages: [python]
fix: manual
fix_reason: "the string becomes a tuple, which changes the comparison"
fix_exception: restructure
stability: preview
ported_from: [ruff/YTT103]
since: 0.5.0
---

`sys.version` is not compared with a version string such as `"3.6"`.

**Why:** Strings compare by character: `"3.10" < "3.6"`.

```codelaw
match:
  kind: comparison_operator
  all:
  - has: {regex: ^sys\.version$}
  - has: {kind: string, regex: '^["'']\d\.'}
  - has: {field: operators, regex: '^[<>]=?$'}
message: String comparison of versions breaks on 3.10; compare `sys.version_info`
```

## How to fix

1. Compare `sys.version_info` with a tuple: `sys.version_info >= (3, 6)`.

## Examples

### Python

#### Don't

```python
if sys.version >= "3.6":
    pass
```

#### Do

```python
if sys.version_info >= (3, 6):
    pass
```

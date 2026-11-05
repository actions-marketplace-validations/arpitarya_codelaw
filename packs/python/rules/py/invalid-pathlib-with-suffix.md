---
id: py/invalid-pathlib-with-suffix
title: "with_suffix takes a leading dot"
languages: [python]
fix: suggest
fix_reason: "the suffix gains its dot, which is what `with_suffix` requires"
stability: preview
ported_from: [ruff/PTH210]
since: 0.5.0
---

`p.with_suffix("txt")` is written `p.with_suffix(".txt")`.

**Why:** Without the dot `with_suffix` raises ValueError.

```codelaw
match:
  kind: string
  has: {kind: string_content, regex: '^[^.]', pattern: $C}
  inside: {kind: argument_list, regex: '^\(\s*["''][^"'']*["'']\s*\)$', inside: {kind: call, has: {field: function, regex: '\.with_suffix$'}}}
message: 'The suffix needs a leading dot'
fix: '".$C"'
```

## Examples

### Python

#### Don't

```python
out = path.with_suffix("txt")
```

#### Do

```python
out = path.with_suffix(".txt")
```

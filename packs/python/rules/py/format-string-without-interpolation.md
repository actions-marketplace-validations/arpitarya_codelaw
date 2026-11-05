---
id: py/format-string-without-interpolation
title: "No .format() on a string with no fields"
languages: [python]
fix: auto
fix_reason: "auto when `.format()` gets no arguments, so the result is the string itself"
stability: preview
ported_from: [pylint/format-string-without-interpolation]
since: 0.5.0
---

`"text".format()` on a string with no `{}` fields is written `"text"`.

**Why:** The call returns the string unchanged.

```codelaw
match: {pattern: $S.format()}
constraints:
  S:
    kind: string
    all:
    - not: {regex: '[{}]'}
    - not: {regex: '^[rRbBuU]*[fF]'}
message: This string has no fields; drop `.format()`
fix: $S
```

## Examples

### Python

#### Don't

```python
msg = "done".format()
```

#### Do

```python
msg = "done"
```

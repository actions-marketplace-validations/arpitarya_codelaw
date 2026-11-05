---
id: py/use-maxsplit-arg
title: "split(sep, maxsplit=1) when only the first part is used"
languages: [python]
fix: suggest
fix_reason: "the result's other parts are no longer split; only the first is read here"
stability: preview
ported_from: [ruff/PLC0207, pylint/use-maxsplit-arg]
since: 0.5.0
---

`s.split(sep)[0]` is written `s.split(sep, maxsplit=1)[0]`.

**Why:** Splitting the whole string to read its first part wastes work.

```codelaw
match:
  kind: subscript
  pattern: $S.split($P)[0]
  not: {inside: {kind: assignment, field: left}}
constraints:
  P: {kind: string}
message: "Split once: `$S.split($P, maxsplit=1)[0]`"
fix: "$S.split($P, maxsplit=1)[0]"
```

## Examples

### Python

#### Don't

```python
key = line.split("=")[0]
```

#### Do

```python
key = line.split("=", maxsplit=1)[0]
```

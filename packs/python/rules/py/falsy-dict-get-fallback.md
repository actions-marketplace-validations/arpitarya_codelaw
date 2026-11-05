---
id: py/falsy-dict-get-fallback
title: "No falsy fallback to dict.get in a test"
languages: [python]
fix: auto
fix_reason: "auto in a condition, where `None` and the falsy fallback test the same"
stability: preview
ported_from: [ruff/RUF056]
since: 0.5.0
---

In an `if`/`while` condition, `d.get(k, False)` (or `None`, `0`, `""`, `[]`, `{}`) is written `d.get(k)`.

**Why:** `get` already returns `None`, which is falsy; the fallback changes nothing in a truth test.

```codelaw
match:
  pattern: $D.get($K, $F)
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}, {kind: not_operator}]}
constraints:
  F: {regex: '^(False|None|0|""|''''|\[\]|\{\}|\(\))$'}
message: "The fallback is falsy like `None`; write `$D.get($K)`"
fix: "$D.get($K)"
```

## Examples

### Python

#### Don't

```python
if opts.get("debug", False):
    trace()
```

#### Do

```python
if opts.get("debug"):
    trace()
```

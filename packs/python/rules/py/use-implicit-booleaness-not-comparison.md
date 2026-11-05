---
id: py/use-implicit-booleaness-not-comparison
title: "if not x:, not if x == []"
languages: [python]
fix: suggest
fix_reason: "`not x` is true for any empty or falsy value, not only an empty list"
stability: preview
ported_from: [pylint/use-implicit-booleaness-not-comparison]
since: 0.5.0
---

A value is not compared with an empty literal `[]`, `{}` or `()` in a condition; `not x` tests it.

**Why:** Empty containers are already falsy.

```codelaw
match:
  kind: comparison_operator
  pattern: $X == $E
  inside: {any: [{kind: if_statement}, {kind: elif_clause}, {kind: while_statement}]}
constraints:
  E: {regex: '^(\[\s*\]|\{\s*\}|\(\s*\))$'}
  X: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `not $X`"
fix: "not $X"
```

## Examples

### Python

#### Don't

```python
if items == []:
    stop()
```

#### Do

```python
if not items:
    stop()
```

---
id: py/non-pep695-type-alias
title: "type X = ..., not X: TypeAlias = ..."
languages: [python]
fix: suggest
fix_reason: "the `type` statement needs Python 3.12 and evaluates lazily"
stability: preview
ported_from: [ruff/UP040]
since: 0.5.0
---

`X: TypeAlias = int | str` is written `type X = int | str` (Python 3.12+).

**Why:** The `type` statement is the language's own alias syntax.

```codelaw
match:
  kind: assignment
  has: {field: type, kind: type, regex: '^(typing\.)?TypeAlias$'}
  pattern: "$N: $T = $V"
  inside: {kind: expression_statement, inside: {kind: module}}
message: "Write `type $N = $V`"
fix: "type $N = $V"
```

## Examples

### Python

#### Don't

```python
Number: TypeAlias = int | float
```

#### Do

```python
type Number = int | float
```

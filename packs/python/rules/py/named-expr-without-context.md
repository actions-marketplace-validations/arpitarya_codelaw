---
id: py/named-expr-without-context
title: "No walrus as a statement"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLW0131]
since: 0.5.0
---

A bare `(x := value)` statement is written `x = value`.

**Why:** The walrus exists to bind inside an expression; as a statement it is just an assignment.

```codelaw
match:
  kind: expression_statement
  has: {kind: parenthesized_expression, has: {kind: named_expression, pattern: $N := $V}}
message: "Write `$N = $V`"
fix: "$N = $V"
```

## Examples

### Python

#### Don't

```python
(count := 0)
```

#### Do

```python
count = 0
```

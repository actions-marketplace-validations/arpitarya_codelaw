---
id: py/unnecessary-generator-set
title: "A set comprehension, not set(generator)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C401]
since: 0.5.0
---

`set(x for x in y)` is written `{x for x in y}` (one `for`, no `if`).

**Why:** The comprehension builds the set directly.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^set$}
  - has:
      field: arguments
      kind: generator_expression
      all:
      - has: {field: body, pattern: $E}
      - has: {kind: for_in_clause, pattern: $F}
      - not:
          has:
            any:
            - {kind: if_clause}
            - {kind: for_in_clause, nthChild: 3}
message: Write a set comprehension
fix: '{$E $F}'
```

## Examples

### Python

#### Don't

```python
seen = set(n % 7 for n in nums)
```

#### Do

```python
seen = {n % 7 for n in nums}
```

---
id: py/unnecessary-generator-list
title: "A list comprehension, not list(generator)"
languages: [python]
fix: auto
fix_reason: "auto for one `for` and no `if`; other generators are reported without a fix"
stability: preview
ported_from: [ruff/C400]
since: 0.5.0
---

`list(x for x in y)` is written `[x for x in y]`.

**Why:** The comprehension builds the list directly, without a generator and a call.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^list$}
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
message: Write a list comprehension
fix: '[$E $F]'
```

## Examples

### Python

#### Don't

```python
squares = list(n * n for n in nums)
```

#### Do

```python
squares = [n * n for n in nums]
```

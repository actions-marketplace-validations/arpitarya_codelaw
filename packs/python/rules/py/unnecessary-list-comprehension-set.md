---
id: py/unnecessary-list-comprehension-set
title: "A set comprehension, not set([comprehension])"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C403, pylint/consider-using-set-comprehension]
since: 0.5.0
---

`set([x for x in y])` is written `{x for x in y}` (one `for`, no `if`).

**Why:** The list is built only to be thrown away.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^set$}
  - has:
      field: arguments
      regex: ^\(\[
      not:
        has: {nthChild: 2}
      has:
        kind: list_comprehension
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
seen = set([n % 7 for n in nums])
```

#### Do

```python
seen = {n % 7 for n in nums}
```

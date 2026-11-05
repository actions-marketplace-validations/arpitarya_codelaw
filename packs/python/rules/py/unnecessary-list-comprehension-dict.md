---
id: py/unnecessary-list-comprehension-dict
title: "A dict comprehension, not dict([pairs])"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C404, pylint/consider-using-dict-comprehension]
since: 0.5.0
---

`dict([(k, v) for ...])` is written `{k: v for ...}` (one `for`, no `if`).

**Why:** The list of pairs is built only to be thrown away.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^dict$}
  - has:
      field: arguments
      regex: ^\(\[
      not:
        has: {nthChild: 2}
      has:
        kind: list_comprehension
        all:
        - has: {field: body, kind: tuple, pattern: '($KEY, $VAL)'}
        - has: {kind: for_in_clause, pattern: $F}
        - not:
            has:
              any:
              - {kind: if_clause}
              - {kind: for_in_clause, nthChild: 3}
message: Write a dict comprehension
fix: '{$KEY: $VAL $F}'
```

## Examples

### Python

#### Don't

```python
index = dict([(n, i) for i, n in pairs])
```

#### Do

```python
index = {n: i for i, n in pairs}
```

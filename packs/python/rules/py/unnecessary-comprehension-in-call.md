---
id: py/unnecessary-comprehension-in-call
title: "A generator, not a list, inside any() and all()"
languages: [python]
fix: suggest
fix_reason: "the generator stops at the first decisive item, so side effects of later items no longer run"
stability: preview
ported_from: [ruff/C419, pylint/use-a-generator, pylint/consider-using-generator]
since: 0.5.0
---

`any([x for x in y])` is written `any(x for x in y)` (also `all`, `sum`, `min`, `max`; one `for`, no `if`).

**Why:** The list is built in full only to be consumed once; `any` and `all` can stop early.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^(any|all|sum|min|max)$, pattern: $FN}
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
message: Pass a generator to `$FN`
fix: $FN($E $F)
```

## Examples

### Python

#### Don't

```python
ok = any([x > 0 for x in nums])
```

#### Do

```python
ok = any(x > 0 for x in nums)
```

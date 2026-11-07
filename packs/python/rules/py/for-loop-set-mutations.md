---
id: py/for-loop-set-mutations
title: "A loop that only adds to a set is update"
languages: [python]
fix: suggest
fix_reason: "the loop becomes one `update` or `difference_update` call; the loop variable is no longer bound after it"
stability: preview
ported_from: [ruff/FURB142]
since: 0.5.0
---

A `for` loop whose body is only `s.add(expr)` or `s.discard(expr)`, where `s` was assigned a set (`set()`, `{...}`, a set comprehension) earlier in the same block, is written `s.update(...)` or `s.difference_update(...)`.

**Why:** One call says the loop's whole job.

```codelaw
utils:
  loop:
    kind: for_statement
    all:
      - has: {field: left, pattern: $X}
      - has: {field: right, pattern: $Y}
      - not: {has: {kind: else_clause}}
      - not: {regex: '^async'}
match:
  kind: for_statement
  all:
    - matches: loop
    - follows:
        stopBy: end
        kind: expression_statement
        has: {kind: assignment, all: [{has: {field: left, pattern: $S}}, {has: {field: right, any: [{kind: set}, {kind: set_comprehension}, {pattern: set($$$)}]}}]}
    - any:
        - all: [{pattern: $ADD1}, {has: {field: body, all: [{has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $S.add($X)}}}, {not: {has: {nthChild: 2}}}]}}]
        - all: [{pattern: $ADDN}, {has: {field: body, all: [{has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $S.add($E)}}}, {not: {has: {nthChild: 2}}}, {not: {has: {kind: expression_statement, has: {kind: call, pattern: $S.add($X)}}}}]}}]
        - all: [{pattern: $DEL1}, {has: {field: body, all: [{has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $S.discard($X)}}}, {not: {has: {nthChild: 2}}}]}}]
message: "Update the set in one call"
fix:
  - replace: {target: $ADD1, with: "$S.update($Y)"}
  - replace: {target: $ADDN, with: "$S.update($E for $X in $Y)"}
  - replace: {target: $DEL1, with: "$S.difference_update($Y)"}
```

## Examples

### Python

#### Don't

```python
seen = set()
for u in users:
    seen.add(u.id)
```

#### Do

```python
seen = set()
seen.update(u.id for u in users)
```

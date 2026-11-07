---
id: py/manual-dict-comprehension
title: "A loop that only sets keys is a dict comprehension"
languages: [python]
fix: suggest
fix_reason: "the loop becomes `update` with a comprehension; the loop variable is no longer bound after it"
stability: preview
ported_from: [ruff/PERF403]
since: 0.5.0
---

A `for` loop whose body is only `d[key] = value`, or one `if` holding only that, is written `d.update({key: value for x in items})`.

**Why:** The comprehension says what the loop builds, and runs faster.

```codelaw
utils:
  setitem:
    kind: block
    all:
      - has: {nthChild: 1, kind: expression_statement, has: {kind: assignment, pattern: '$D[$K] = $V'}}
      - not: {has: {nthChild: 2}}
  loop:
    kind: for_statement
    all:
      - has: {field: left, pattern: $X}
      - has: {field: right, pattern: $Y}
      - not: {has: {kind: else_clause}}
      - not: {regex: '^async'}
match:
  any:
    - kind: for_statement
      pattern: $PLAIN
      matches: loop
      has: {field: body, matches: setitem}
    - kind: for_statement
      pattern: $FILTERED
      matches: loop
      has:
        field: body
        all:
          - has: {nthChild: 1, kind: if_statement, all: [{has: {field: condition, pattern: $C}}, {has: {field: consequence, matches: setitem}}, {not: {has: {any: [{kind: elif_clause}, {kind: else_clause}]}}}]}
          - not: {has: {nthChild: 2}}
message: "Write `$D.update({...})` with a comprehension"
fix:
  - replace: {target: $PLAIN, with: "$D.update({$K: $V for $X in $Y})"}
  - replace: {target: $FILTERED, with: "$D.update({$K: $V for $X in $Y if $C})"}
```

## Examples

### Python

#### Don't

```python
for u in users:
    by_id[u.id] = u
```

#### Do

```python
by_id.update({u.id: u for u in users})
```

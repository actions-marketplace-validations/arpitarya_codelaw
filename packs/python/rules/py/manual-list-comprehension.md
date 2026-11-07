---
id: py/manual-list-comprehension
title: "A loop that only appends is a comprehension"
languages: [python]
fix: suggest
fix_reason: "the loop becomes `extend` over a generator; the loop variable is no longer bound after it"
stability: preview
ported_from: [ruff/PERF401]
since: 0.5.0
---

A `for` loop whose body is only `result.append(expr)`, or one `if` holding only that, is written `result.extend(expr for x in items)` (with the `if` as the generator's condition). A loop that appends the loop variable itself is `py/manual-list-copy`'s.

**Why:** The comprehension says what the loop builds, and runs faster.

```codelaw
utils:
  append:
    kind: block
    all:
      - has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $R.append($E)}}
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
      has: {field: body, all: [{matches: append}, {not: {has: {kind: expression_statement, has: {kind: call, pattern: $R.append($X)}}}}]}
    - kind: for_statement
      pattern: $FILTERED
      matches: loop
      has:
        field: body
        all:
          - has: {nthChild: 1, kind: if_statement, all: [{has: {field: condition, pattern: $C}}, {has: {field: consequence, matches: append}}, {not: {has: {any: [{kind: elif_clause}, {kind: else_clause}]}}}]}
          - not: {has: {nthChild: 2}}
message: "Write `$R.extend(...)` with a generator"
fix:
  - replace: {target: $PLAIN, with: "$R.extend($E for $X in $Y)"}
  - replace: {target: $FILTERED, with: "$R.extend($E for $X in $Y if $C)"}
```

## Examples

### Python

#### Don't

```python
def names(users):
    out = []
    for u in users:
        out.append(u.name)
    return out
```

#### Do

```python
def names(users):
    out = []
    out.extend(u.name for u in users)
    return out
```

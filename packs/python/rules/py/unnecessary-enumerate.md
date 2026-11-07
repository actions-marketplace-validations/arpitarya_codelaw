---
id: py/unnecessary-enumerate
title: "No enumerate whose index or value is unused"
languages: [python]
fix: suggest
fix_reason: "suggest when the index is `_`, iterating the sequence itself; when the value is `_` the finding is manual, since `range(len(...))` needs a sized sequence"
fix_exception: value
stability: preview
ported_from: [ruff/FURB148]
since: 0.5.0
---

`for _, v in enumerate(items):` is written `for v in items:`; `for i, _ in enumerate(items):` iterates `range(len(items))`. A call to `enumerate` with a `start` is left alone.

**Why:** `enumerate` makes a pair the loop throws half of away.

```codelaw
match:
  kind: for_statement
  any:
    - all:
        - has: {field: left, pattern: $LEFT, any: [{kind: pattern_list}, {kind: tuple_pattern}], all: [{has: {nthChild: 1, regex: '^_$'}}, {has: {nthChild: 2, kind: identifier, pattern: $V}}, {not: {has: {nthChild: 3}}}]}
        - has: {field: right, kind: call, pattern: $RIGHT, all: [{has: {field: function, regex: '^enumerate$'}}, {has: {field: arguments, all: [{has: {nthChild: 1, pattern: $Y}}, {not: {has: {nthChild: 2}}}]}}]}
    - all:
        - has: {field: left, any: [{kind: pattern_list}, {kind: tuple_pattern}], all: [{has: {nthChild: 2, regex: '^_$'}}, {not: {has: {nthChild: 3}}}]}
        - has: {field: right, kind: call, all: [{has: {field: function, regex: '^enumerate$'}}, {has: {field: arguments, all: [{has: {nthChild: 1}}, {not: {has: {nthChild: 2}}}]}}]}
message: "Drop the half of `enumerate` that is not used"
fix:
  - replace: {target: $LEFT, with: $V}
  - replace: {target: $RIGHT, with: $Y}
```

## Examples

### Python

#### Don't

```python
for _, user in enumerate(users):
    notify(user)
```

#### Do

```python
for user in users:
    notify(user)
```

---
id: py/for-loop-writes
title: "A loop that only writes to a file is writelines"
languages: [python]
fix: suggest
fix_reason: "the loop becomes one `writelines` call; the loop variable is no longer bound after it"
stability: preview
ported_from: [ruff/FURB122]
since: 0.5.0
---

Inside `with open(...) as f:`, a `for` loop whose body is only `f.write(expr)` is written `f.writelines(expr for x in items)`, or `f.writelines(items)` when it writes the item itself.

**Why:** `writelines` says the loop's whole job in one call.

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
    - inside:
        kind: with_statement
        stopBy: {any: [{kind: function_definition}, {kind: class_definition}]}
        has: {kind: with_clause, has: {kind: with_item, has: {kind: as_pattern, all: [{has: {kind: call, has: {field: function, regex: '^open$'}}}, {has: {kind: as_pattern_target, has: {kind: identifier, pattern: $F}}}]}}}
    - any:
        - all: [{pattern: $SAME}, {has: {field: body, all: [{has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $F.write($X)}}}, {not: {has: {nthChild: 2}}}]}}]
        - all: [{pattern: $OTHER}, {has: {field: body, all: [{has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $F.write($E)}}}, {not: {has: {nthChild: 2}}}, {not: {has: {kind: expression_statement, has: {kind: call, pattern: $F.write($X)}}}}]}}]
message: "Write `$F.writelines(...)`"
fix:
  - replace: {target: $SAME, with: "$F.writelines($Y)"}
  - replace: {target: $OTHER, with: "$F.writelines($E for $X in $Y)"}
```

## Examples

### Python

#### Don't

```python
with open(path, "w") as f:
    for line in lines:
        f.write(line + "\n")
```

#### Do

```python
with open(path, "w") as f:
    f.writelines(line + "\n" for line in lines)
```

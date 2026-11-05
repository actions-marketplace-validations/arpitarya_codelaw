---
id: py/collapsible-if
title: "One if, not an if nested in an if"
languages: [python]
fix: manual
fix_reason: "joining the conditions re-indents the inner body, which a template edit cannot do"
fix_exception: restructure
stability: preview
ported_from: [ruff/SIM102]
since: 0.5.0
---

An `if` whose whole body is another `if` (neither with `else`) is written as one `if a and b:`.

**Why:** One condition reads as one test, with one level less of indentation.

```codelaw
match:
  kind: if_statement
  not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}
  has:
    field: consequence
    regex: '^if\s'
    not: {has: {nthChild: 2}}
    has: {kind: if_statement, not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}}
message: "Merge the nested `if` into one condition with `and`"
```

## How to fix

1. Join the two conditions with `and` (parenthesize each if it contains `or`) and dedent the inner body one level.

## Examples

### Python

#### Don't

```python
if ready:
    if count > 0:
        go()
```

#### Do

```python
if ready and count > 0:
    go()
```

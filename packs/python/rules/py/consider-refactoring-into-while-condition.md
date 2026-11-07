---
id: py/consider-refactoring-into-while-condition
title: "A `while True` that starts by breaking puts the test in its condition"
languages: [python]
fix: manual
fix_reason: "moving the test into the condition means negating it and re-reading the loop's other exits"
fix_exception: restructure
stability: preview
ported_from: [pylint/consider-refactoring-into-while-condition]
since: 0.5.0
---

A `while` whose condition is a truthy constant (`True`, a non-zero number, a non-empty string) and whose body starts with `if <test>: break` can say `while not <test>:` instead. This is one of Pylint's optional extensions; it is in no preset.

**Why:** The loop's exit condition is easier to find in the loop's header than in its first statement.

```codelaw
match:
  kind: while_statement
  all:
    - has:
        field: condition
        any:
          - kind: "true"
          - {kind: integer, not: {regex: '^0+$'}}
          - {kind: string, not: {regex: '^[a-zA-Z]*("""|''''''|"|'')("""|''''''|"|'')$'}}
    - has:
        field: body
        has:
          nthChild: 1
          kind: if_statement
          has:
            field: consequence
            has: {nthChild: 1, kind: break_statement}
message: "Move the first `if … break` into the `while` condition"
```

## How to fix

1. Negate the `if` test and make it the `while` condition.
2. Remove the `if … break` from the top of the body.

## Examples

### Python

#### Don't

```python
while True:
    if item is None:
        break
    item = handle(item)
```

#### Do

```python
while item is not None:
    item = handle(item)
```

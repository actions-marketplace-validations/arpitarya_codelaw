---
id: py/superfluous-else-break
title: "No else after break"
languages: [python]
fix: manual
fix_reason: "the `else` (or `elif`) is removed and its body dedented, which the engine cannot re-indent in Python yet"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/RET508, pylint/no-else-break]
since: 0.5.0
---

An `else` or `elif` that follows an `if` branch ending in `break` — every branch before it ending so — is not needed: its body can follow the `if` unindented.

**Why:** After `break` the code below only runs when the branch did not; the `else` adds a level of nesting for nothing.

```codelaw
utils:
  ends:
    kind: block
    has: {nthChild: {position: 1, reverse: true}, kind: break_statement}
match:
  any: [{kind: else_clause}, {kind: elif_clause}]
  all:
    - inside:
        kind: if_statement
        all:
          - has: {field: consequence, matches: ends}
          - not: {has: {kind: elif_clause, not: {has: {field: consequence, matches: ends}}}}
    - inside: {kind: for_statement, stopBy: {any: [{kind: function_definition}, {kind: class_definition}]}}
message: "Remove the `else`; the `break` already leaves the branch"
```

## How to fix

1. Delete the `else:` (or turn the `elif` into an `if`) and dedent its body one level.

## Examples

### Python

#### Don't

```python
for x in xs:
    if x is None:
        break
    else:
        use(x)
```

#### Do

```python
for x in xs:
    if x is None:
        break
    use(x)
```

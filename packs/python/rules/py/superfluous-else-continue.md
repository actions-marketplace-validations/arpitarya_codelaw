---
id: py/superfluous-else-continue
title: "No else after continue"
languages: [python]
fix: manual
fix_reason: "the `else` (or `elif`) is removed and its body dedented, which the engine cannot re-indent in Python yet"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [ruff/RET507, pylint/no-else-continue]
since: 0.5.0
---

An `else` or `elif` that follows an `if` branch ending in `continue` — every branch before it ending so — is not needed: its body can follow the `if` unindented.

**Why:** After `continue` the code below only runs when the branch did not; the `else` adds a level of nesting for nothing.

```codelaw
utils:
  ends:
    kind: block
    has: {nthChild: {position: 1, reverse: true}, kind: continue_statement}
match:
  any: [{kind: else_clause}, {kind: elif_clause}]
  all:
    - inside:
        kind: if_statement
        all:
          - has: {field: consequence, matches: ends}
          - not: {has: {kind: elif_clause, not: {has: {field: consequence, matches: ends}}}}
    - inside: {kind: for_statement, stopBy: {any: [{kind: function_definition}, {kind: class_definition}]}}
message: "Remove the `else`; the `continue` already leaves the branch"
```

## How to fix

1. Delete the `else:` (or turn the `elif` into an `if`) and dedent its body one level.

## Examples

### Python

#### Don't

```python
for x in xs:
    if not x:
        continue
    else:
        use(x)
```

#### Do

```python
for x in xs:
    if not x:
        continue
    use(x)
```

---
id: py/superfluous-else-break
title: "No else after break"
languages: [python]
fix: suggest
fix_reason: "the `else` goes and its body moves up one level, comments kept; a name it binds is now bound on the other path too, and an `elif`, or a body holding a multi-line string, is rewritten by hand"
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
    - inside: {any: [{kind: for_statement}, {kind: while_statement}], stopBy: {any: [{kind: function_definition}, {kind: class_definition}]}}
    - any:
      - all: [{ kind: else_clause }, { has: { field: body, pattern: $B } }, { pattern: $ELSE }]
      - regex: ''
message: "Remove the `else`; the `break` already leaves the branch"
fix:
  - unwrap-block: { target: $B, within: $ELSE }
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

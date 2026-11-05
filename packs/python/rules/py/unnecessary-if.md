---
id: py/unnecessary-if
title: "No if whose body is pass"
languages: [python]
fix: auto
fix_reason: "auto when the condition holds no call, await, walrus or subscript, so there is no side effect to keep"
stability: preview
ported_from: [ruff/RUF050]
since: 0.5.0
---

An `if` with no `else` whose body is only `pass` is removed when its condition has no side effects.

**Why:** It does nothing.

```codelaw
match:
  kind: if_statement
  all:
    - has: {field: consequence, regex: '^pass$'}
    - not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}
    - has: {field: condition, not: {any: [{kind: call}, {kind: await}, {kind: named_expression}, {kind: subscript}, {has: {stopBy: end, any: [{kind: call}, {kind: await}, {kind: named_expression}, {kind: subscript}]}}]}}
message: "This `if` does nothing; remove it"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
if ready:
    pass
go()
```

#### Do

```python
go()
```

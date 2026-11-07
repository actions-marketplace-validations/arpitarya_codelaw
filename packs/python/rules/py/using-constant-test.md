---
id: py/using-constant-test
title: "No if or while on a constant"
languages: [python]
fix: manual
fix_reason: "the condition meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/using-constant-test]
since: 0.5.0
---

An `if`, `elif`, `while` or conditional expression does not test a literal — a number, a string, a list, a dict, `None` — or a lambda. `while True` and `while 1` are not reported.

**Why:** The branch always or never runs; the test usually stands for a forgotten call or name.

```codelaw
utils:
  const: {any: [{kind: string}, {kind: integer}, {kind: float}, {kind: list}, {kind: dictionary}, {kind: tuple}, {kind: set}, {kind: none}, {kind: lambda}]}
match:
  any:
    - kind: if_statement
      has: {field: condition, matches: const}
    - kind: elif_clause
      has: {field: condition, matches: const}
    - kind: while_statement
      has: {field: condition, matches: const, not: {regex: '^1$'}}
message: "The test is a constant"
```

## How to fix

1. Test the variable or call that was meant.

## Examples

### Python

#### Don't

```python
if "debug":
    run()
```

#### Do

```python
if debug:
    run()
```

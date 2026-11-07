---
id: py/missing-parentheses-for-call-in-test
title: "A test calls the function it names"
languages: [python]
fix: manual
fix_reason: "whether the call was meant, and with which arguments, or a test for `None` instead, is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/missing-parentheses-for-call-in-test]
since: 0.5.0
---

The test of an `if`, an `elif`, a conditional expression or a comprehension's `if` is a bare name the file declares with `def`, not a call of it. A function is always true, so the test always passes. Where Pylint infers any callable — a `lambda` held in a name, a method reached through an instance — this rule reads names declared by `def`.

**Why:** `if is_ready:` with the parentheses forgotten runs the branch every time.

```codelaw
match:
  kind: identifier
  pattern: $F
  any:
    - inside: {kind: if_statement, field: condition}
    - inside: {kind: elif_clause, field: condition}
    - {nthChild: 2, inside: {kind: conditional_expression}}
    - inside: {kind: if_clause}
where:
  $F: {binding: function}
message: "`$F` is a function and always true; call it"
```

## How to fix

1. Call the function, with its arguments, or test what was meant (`is not None`).

## Examples

### Python

#### Don't

```python
def is_ready():
    return state.ok

if is_ready:
    start()
```

#### Do

```python
def is_ready():
    return state.ok

if is_ready():
    start()
```

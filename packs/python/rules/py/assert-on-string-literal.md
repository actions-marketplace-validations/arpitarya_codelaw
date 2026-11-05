---
id: py/assert-on-string-literal
title: "No assert on a string"
languages: [python]
fix: manual
fix_reason: "the condition that was meant is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLW0129]
since: 0.5.0
---

An `assert` tests a condition, not a string literal.

**Why:** A non-empty string is always true and an empty one always false; the assert decides nothing.

```codelaw
match:
  kind: assert_statement
  has: {kind: string, nthChild: 1}
message: "Assert a condition; a string literal is constant"
```

## How to fix

1. Write the condition, and pass the string as the message: `assert cond, "..."`.

## Examples

### Python

#### Don't

```python
assert "never empty"
```

#### Do

```python
assert items, "never empty"
```

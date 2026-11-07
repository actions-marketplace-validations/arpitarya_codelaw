---
id: py/assert-tuple
title: "`assert` doesn't test a tuple"
languages: [python]
fix: suggest
fix_reason: "without the parentheses the first item is the test and the second its message, as meant; the assert starts to check and can fail. A tuple of three or more items, or with a comment, is manual"
fix_exception: value
stability: preview
ported_from: [ruff/F631, pylint/assert-on-tuple]
since: 0.1.0
---

An `assert` tests a condition, not a parenthesised `(condition, message)` tuple.

**Why:** A non-empty tuple is always true, so the assert can never fail.

```codelaw
native: py/assert-tuple
```

## How to fix

1. Remove the parentheses so the message becomes the assert's second operand.

## Examples

### Python

#### Don't

```py
assert (count > 0, "count must be positive")
```

#### Do

```py
assert count > 0, "count must be positive"
```

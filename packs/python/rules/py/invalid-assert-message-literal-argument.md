---
id: py/invalid-assert-message-literal-argument
title: "No empty-collection assert message"
languages: [python]
fix: manual
fix_reason: "whether the literal was a condition or a message is the author's"
stability: preview
ported_from: [ruff/RUF040]
since: 0.5.0
---

An `assert`'s message is not a list, tuple, set or dict literal mistaken for a second condition.

**Why:** `assert x, [y]` asserts only `x`; the literal becomes the message.

```codelaw
match:
  kind: assert_statement
  pattern: assert $C, $M
constraints:
  M: {any: [{kind: list}, {kind: tuple}, {kind: set}, {kind: dictionary}]}
message: "The literal after the comma is the message, not a condition"
```

## How to fix

1. Write a string message, or assert the condition the literal was meant to be.

## Examples

### Python

#### Don't

```python
assert valid, [1, 2]
```

#### Do

```python
assert valid, "invalid input"
```

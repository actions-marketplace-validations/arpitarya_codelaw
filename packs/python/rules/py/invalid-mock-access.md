---
id: py/invalid-mock-access
title: "Mock assertions are called"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PGH005]
since: 0.5.0
---

`mock.assert_called_once` and the other mock assertions are called, with `()`.

**Why:** Without the call the attribute is just read, and the test asserts nothing.

```codelaw
match:
  kind: expression_statement
  has: {kind: attribute, nthChild: 1, pattern: $A, regex: '\.(assert_any_call|assert_called|assert_called_once|assert_called_once_with|assert_called_with|assert_has_calls|assert_not_called|assert_awaited|assert_awaited_once|assert_awaited_with|assert_awaited_once_with|assert_any_await|assert_has_awaits|assert_not_awaited)$'}
message: "This assertion is never called; add `()`"
fix: "$A()"
```

## Examples

### Python

#### Don't

```python
mock_send.assert_called_once
```

#### Do

```python
mock_send.assert_called_once()
```

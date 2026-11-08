---
id: py/pytest-patch-with-lambda
title: "A patch returns a value with return_value, not a lambda"
languages: [python]
fix: suggest
fix_reason: "the patch becomes a `MagicMock` returning the value: a call with any arguments still returns it, and the mock also records the call"
stability: preview
ported_from: [ruff/PT008]
since: 0.5.0
---

`mocker.patch(target, lambda: value)` and `mock.patch(target, lambda: value)`, whose lambda takes no parameter, are written `return_value=value`.

**Why:** `return_value` says what the patch does, and the mock records how it was called.

```codelaw
match:
  kind: lambda
  pattern: $L
  not: {has: {kind: lambda_parameters}}
  has: {field: body, pattern: $V}
  inside:
    kind: argument_list
    inside:
      kind: call
      has: {field: function, regex: '^(mocker|class_mocker|module_mocker|session_mocker|package_mocker|mock|unittest\.mock)\.patch(\.object)?$|^patch(\.object)?$'}
message: "Write `return_value=$V`"
fix:
  - replace: {target: $L, with: "return_value=$V"}
```

## Examples

### Python

#### Don't

```python
def test_now(mocker):
    mocker.patch("app.clock.now", lambda: 0)
```

#### Do

```python
def test_now(mocker):
    mocker.patch("app.clock.now", return_value=0)
```

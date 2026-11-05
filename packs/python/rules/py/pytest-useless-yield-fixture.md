---
id: py/pytest-useless-yield-fixture
title: "return, not yield, in a fixture with no teardown"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT022]
since: 0.5.0
---

A fixture whose last statement is `yield value`, with nothing after it, returns the value instead.

**Why:** `yield` signals teardown code that is not there.

```codelaw
match:
  kind: expression_statement
  has: {kind: yield, pattern: yield $V, not: {regex: '^yield\s+from'}}
  nthChild: {position: 1, reverse: true}
  inside: {kind: block, inside: {kind: function_definition, not: {has: {field: body, has: {kind: expression_statement, has: {kind: yield}, nthChild: {position: 1, reverse: true}, follows: {stopBy: end, has: {stopBy: end, kind: yield}}}}}, inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@pytest\.fixture\b'}}}}
message: "Nothing runs after the yield; `return $V`"
fix: "return $V"
```

## Examples

### Python

#### Don't

```python
@pytest.fixture
def user():
    yield make_user()
```

#### Do

```python
@pytest.fixture
def user():
    return make_user()
```

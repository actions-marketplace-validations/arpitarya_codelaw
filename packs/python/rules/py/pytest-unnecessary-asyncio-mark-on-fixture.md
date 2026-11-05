---
id: py/pytest-unnecessary-asyncio-mark-on-fixture
title: "No asyncio mark on a fixture"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT024]
since: 0.5.0
---

`@pytest.mark.asyncio` is not put on a fixture.

**Why:** Marks apply to tests; on a fixture it does nothing.

```codelaw
match:
  kind: decorator
  regex: '^@pytest\.mark\.asyncio(\(\))?$'
  inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@pytest\.fixture\b'}}
message: "This mark does nothing on a fixture"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@pytest.mark.asyncio
@pytest.fixture
async def client():
    return make()
```

#### Do

```python
@pytest.fixture
async def client():
    return make()
```

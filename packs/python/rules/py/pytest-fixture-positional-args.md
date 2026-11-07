---
id: py/pytest-fixture-positional-args
title: "pytest.fixture takes keyword arguments"
languages: [python]
fix: suggest
fix_reason: "suggest when the one positional argument is the scope, written `scope=`; with more than one, the finding is manual"
fix_exception: value
stability: preview
ported_from: [ruff/PT002]
since: 0.5.0
---

`@pytest.fixture` is configured by keyword: `@pytest.fixture(scope="module")`, not `@pytest.fixture("module")`.

**Why:** pytest dropped positional fixture arguments; the decorator fails at collection on pytest 8.

```codelaw
match:
  kind: call
  inside: {kind: decorator}
  all:
    - has: {field: function, regex: '^pytest\.fixture$'}
    - has:
        field: arguments
        has: {nthChild: 1, pattern: $A, not: {any: [{kind: keyword_argument}, {kind: comment}]}}
message: "Pass fixture options by keyword"
fix:
  - replace: {target: $A, with: "scope=$A"}
```

## Examples

### Python

#### Don't

```python
@pytest.fixture("module")
def db():
    return connect()
```

#### Do

```python
@pytest.fixture(scope="module")
def db():
    return connect()
```

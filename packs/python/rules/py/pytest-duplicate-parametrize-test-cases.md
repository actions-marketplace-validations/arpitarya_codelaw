---
id: py/pytest-duplicate-parametrize-test-cases
title: "No duplicate parametrize cases"
languages: [python]
fix: suggest
fix_reason: "the repeated case is removed; it ran the same test twice"
stability: preview
ported_from: [ruff/PT014]
since: 0.5.0
---

A `pytest.mark.parametrize` list holds each case once.

**Why:** A repeated case runs the same test twice and usually hides a typo in the case meant.

```codelaw
match:
  pattern: $C
  follows: {stopBy: end, pattern: $C}
  inside:
    kind: list
    nthChild: 2
    inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^pytest\.mark\.parametrize$'}}}
message: "This case is already in the list"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
@pytest.mark.parametrize("n", [1, 2, 1])
def test_n(n):
    assert n
```

#### Do

```python
@pytest.mark.parametrize("n", [1, 2])
def test_n(n):
    assert n
```

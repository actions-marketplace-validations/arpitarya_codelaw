---
id: py/pytest-parametrize-values-wrong-type
title: "parametrize values are a list"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PT007]
since: 0.5.0
---

The values of `@pytest.mark.parametrize` are a list, `[1, 2, 3]`, not a tuple. (Upstream also checks the type of each row; this rule checks the outer container.)

**Why:** One spelling across a test suite.

```codelaw
match:
  kind: tuple
  pattern: $T
  nthChild: 2
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^pytest\.mark\.parametrize$'}}}
transform:
  I: { substring: { source: $T, startChar: 1, endChar: -1 } }
message: "Write the values as a list"
fix: "[$I]"
```

## Examples

### Python

#### Don't

```python
@pytest.mark.parametrize("n", (1, 2, 3))
def test_n(n):
    pass
```

#### Do

```python
@pytest.mark.parametrize("n", [1, 2, 3])
def test_n(n):
    pass
```

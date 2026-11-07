---
id: py/pytest-parametrize-names-wrong-type
title: "Several parametrize names are a tuple"
languages: [python]
fix: suggest
fix_reason: "suggest for a double-quoted string or a list, rewritten as a tuple of strings; a single-quoted string is manual"
fix_exception: value
stability: preview
ported_from: [ruff/PT006]
since: 0.5.0
---

`@pytest.mark.parametrize` names several arguments as a tuple of strings, `("a", "b")`, not as `"a, b"` or `["a", "b"]`.

**Why:** One spelling across a test suite; a tuple also cannot hide a typo in a comma-separated string.

```codelaw
match:
  nthChild: 1
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^pytest\.mark\.parametrize$'}}}
  any:
    - kind: string
      pattern: $DQ
      regex: '^"[^"\\]*,[^"\\]*[^,\s]\s*"$'
    - kind: string
      regex: "^'[^']*,[^']*'$"
    - kind: list
      pattern: $L
      has: {nthChild: 2}
transform:
  I: { substring: { source: $DQ, startChar: 1, endChar: -1 } }
  Q: { replace: { source: $I, replace: '\s*,\s*', by: '", "' } }
  LI: { substring: { source: $L, startChar: 1, endChar: -1 } }
message: "Name the parameters as a tuple of strings"
fix:
  - replace: {target: $DQ, with: '("$Q")'}
  - replace: {target: $L, with: '($LI)'}
```

## Examples

### Python

#### Don't

```python
@pytest.mark.parametrize("a, b", [(1, 2)])
def test_add(a, b):
    pass
```

#### Do

```python
@pytest.mark.parametrize(("a", "b"), [(1, 2)])
def test_add(a, b):
    pass
```

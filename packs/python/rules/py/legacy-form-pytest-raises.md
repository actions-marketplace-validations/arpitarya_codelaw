---
id: py/legacy-form-pytest-raises
title: "pytest.raises as a context manager"
languages: [python]
fix: suggest
fix_reason: "the `with` body is indented four spaces, which a file indented with tabs would not match; a call whose result is used, or that passes keywords, is rewritten by hand"
stability: preview
ported_from: [ruff/RUF061]
since: 0.5.0
---

`pytest.raises`, `pytest.warns` and `pytest.deprecated_call` are used as context managers, not called with the function and its arguments.

**Why:** The `with` form shows the code that raises and lets the test inspect the exception.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.(raises|warns|deprecated_call)$'}
    - has: {field: arguments, has: {nthChild: 2, not: {kind: keyword_argument}}}
    - not: {inside: {kind: with_item, stopBy: end}}
  any:
    - all:
        - pattern: $FN($E, $F, $$$A)
        - not: {has: {field: arguments, has: {kind: keyword_argument}}}
        - inside: {kind: expression_statement, pattern: $STMT}
    - regex: ''
message: "Use `with pytest.raises(...):` around the call"
fix:
  - replace: { target: $STMT, with: "with $FN($E):" }
  - insert-after: { target: $STMT, text: "    $F($$$A)", line: true }
```

## How to fix

1. Write `with pytest.raises(Error):` and call the function inside the block.

## Examples

### Python

#### Don't

```python
pytest.raises(ValueError, int, "x")
```

#### Do

```python
with pytest.raises(ValueError):
    int("x")
```

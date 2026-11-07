---
id: py/pytest-raises-too-broad
title: "A broad exception in pytest.raises has a match"
languages: [python]
fix: manual
fix_reason: "the message the test expects is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PT011]
since: 0.5.0
---

`pytest.raises` given a broad exception — `BaseException`, `Exception`, `ValueError`, `OSError`, `IOError`, `EnvironmentError` or `socket.error` — also passes `match=` with a non-empty pattern.

**Why:** A broad exception is raised for many reasons; without `match` the test passes when the code fails for another one.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.raises$'}
    - has:
        field: arguments
        all:
          - has: {nthChild: 1, pattern: $E, regex: '^(BaseException|Exception|ValueError|OSError|IOError|EnvironmentError|socket\.error)$'}
          - not: {has: {kind: keyword_argument, has: {field: name, regex: '^match$'}, not: {has: {field: value, regex: '^[rbuRBU]*(""|'''')$'}}}}
message: "`pytest.raises($E)` is too broad; add `match=`"
```

## How to fix

1. Add `match=` with a regular expression for the message the test expects, or name a narrower exception.

## Examples

### Python

#### Don't

```python
with pytest.raises(ValueError):
    int("x")
```

#### Do

```python
with pytest.raises(ValueError, match="invalid literal"):
    int("x")
```

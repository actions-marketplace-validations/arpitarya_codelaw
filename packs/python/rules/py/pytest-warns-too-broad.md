---
id: py/pytest-warns-too-broad
title: "A broad warning in pytest.warns has a match"
languages: [python]
fix: manual
fix_reason: "the message the test expects is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PT030]
since: 0.5.0
---

`pytest.warns` given a broad warning — `Warning`, `UserWarning` or `DeprecationWarning` — also passes `match=` with a non-empty pattern.

**Why:** A broad warning is emitted for many reasons; without `match` the test passes for another one.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^pytest\.warns$'}
    - has:
        field: arguments
        all:
          - has: {nthChild: 1, pattern: $E, regex: '^(Warning|UserWarning|DeprecationWarning)$'}
          - not: {has: {kind: keyword_argument, has: {field: name, regex: '^match$'}, not: {has: {field: value, regex: '^[rbuRBU]*(""|'''')$'}}}}
message: "`pytest.warns($E)` is too broad; add `match=`"
```

## How to fix

1. Add `match=` with a regular expression for the message the test expects, or name a narrower warning.

## Examples

### Python

#### Don't

```python
with pytest.warns(DeprecationWarning):
    old_api()
```

#### Do

```python
with pytest.warns(DeprecationWarning, match="old_api is deprecated"):
    old_api()
```

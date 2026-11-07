---
id: py/boolean-positional-value-in-call
title: "A call passes `True` or `False` by keyword"
languages: [python]
fix: manual
fix_reason: "the keyword is the parameter's name, which the call does not show"
fix_exception: value
stability: preview
ported_from: [ruff/FBT003]
since: 0.5.0
---

A call passes `True` or `False` as a positional argument: `render(page, True)`. Pass it by keyword so the call says what it switches. Calls whose name upstream allows are left alone — conversions and lookups where the flag is the value itself (`bool`, `int`, `str`, `float`, `bytes`, `getattr`, `setattr`, `get`, `pop`, `setdefault`, `fromkeys`, `append`, `insert`, `index`, `count`, `remove`, `next`, `param`, `set_blocking`, `set_enabled`, `getboolean`, `getint`, `getfloat`, `is_`, `is_not`, `isnull`, `ifnull`, `nvl`, `coalesce`, `__setattr__` and the unittest equality asserts) — and so is a one-argument setter, named `set`, `set_…` or `setX…`, and `multiprocessing.Value`.

**Why:** `True` in a call reads as nothing; `force=True` reads as what it does.

```codelaw
match:
  any: [{kind: "true"}, {kind: "false"}]
  inside:
    kind: argument_list
    inside:
      kind: call
      all:
        - not:
            has:
              field: function
              regex: '(^|\.)(__setattr__|append|assertEqual|assertEquals|assertNotEqual|assertNotEquals|bool|bytes|coalesce|count|failIfEqual|failUnlessEqual|float|fromkeys|get|getattr|getboolean|getfloat|getint|ifnull|index|insert|int|is_|is_not|isnull|next|nvl|param|pop|remove|set_blocking|set_enabled|setattr|setdefault|str)$'
        - not: {has: {field: function, regex: '^(multiprocessing\.)?Value$'}}
        - not:
            all:
              - has: {field: function, regex: '(^|\.)set([_A-Z]\w*)?$'}
              - not: {has: {field: arguments, has: {nthChild: 2, not: {kind: keyword_argument}}}}
message: "Pass this flag by keyword"
```

## How to fix

1. Write the parameter's name: `render(page, preview=True)`.
2. If you own the function, make the flag keyword-only (`def render(page, *, preview=False)`).

## Examples

### Python

#### Don't

```python
render(page, True)
```

#### Do

```python
render(page, preview=True)
```

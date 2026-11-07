---
id: py/dict-index-missing-items
title: "A loop over a dict that reads each value iterates items()"
languages: [python]
fix: manual
fix_reason: "the value's name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLC0206, pylint/consider-using-dict-items]
since: 0.5.0
---

`for k in d:` whose body reads `d[k]` iterates `d.items()` instead: `for k, v in d.items():`.

**Why:** One lookup per key is done twice, and the loop hides that it uses the values.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: left, kind: identifier, pattern: $K}
    - has: {field: right, kind: identifier, pattern: $D}
    - has: {field: body, has: {kind: subscript, stopBy: end, pattern: '$D[$K]'}}
message: "Iterate `$D.items()`"
```

## How to fix

1. Iterate `d.items()` with a second loop variable, and use it in place of `d[k]`.

## Examples

### Python

#### Don't

```python
for name in scores:
    print(name, scores[name])
```

#### Do

```python
for name, score in scores.items():
    print(name, score)
```

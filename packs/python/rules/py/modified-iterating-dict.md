---
id: py/modified-iterating-dict
title: "A dict is not resized while it is iterated"
languages: [python]
fix: manual
fix_reason: "the loop iterates a copy, or collects the keys and changes the dict after"
fix_exception: restructure
stability: preview
ported_from: [pylint/modified-iterating-dict]
since: 0.5.0
---

A `for` loop over a dict (`d`, `d.keys()`, `d.values()`, `d.items()`) does not add or delete keys of that dict in its body.

**Why:** Python raises `RuntimeError: dictionary changed size during iteration`.

```codelaw
match:
  kind: for_statement
  all:
    - has: {field: right, any: [{kind: identifier, pattern: $D}, {kind: call, pattern: $D.keys()}, {kind: call, pattern: $D.values()}, {kind: call, pattern: $D.items()}]}
    - has:
        field: body
        has:
          stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
          any:
            - {kind: delete_statement, has: {kind: subscript, pattern: '$D[$K]'}}
            - {kind: call, any: [{pattern: '$D.pop($$$A)'}, {pattern: '$D.popitem()'}, {pattern: '$D.clear()'}, {pattern: '$D.setdefault($$$A)'}, {pattern: '$D.update($$$A)'}]}
message: "`$D` changes size while it is iterated"
```

## How to fix

1. Iterate over a copy (`list(d)`), or collect the keys and change the dict after the loop.

## Examples

### Python

#### Don't

```python
for k in cache:
    if stale(k):
        del cache[k]
```

#### Do

```python
for k in list(cache):
    if stale(k):
        del cache[k]
```

---
id: py/stop-iteration-return
title: "A generator returns instead of raising StopIteration"
languages: [python]
fix: suggest
fix_reason: "the generator now ends normally where it raised `RuntimeError` since PEP 479"
stability: preview
ported_from: [ruff/PLR1708, pylint/stop-iteration-return]
since: 0.5.0
---

Inside a generator, `raise StopIteration` is written `return` (and `raise StopIteration(v)` `return v`).

**Why:** Since Python 3.7 a `StopIteration` raised in a generator becomes a `RuntimeError` (PEP 479).

```codelaw
match:
  kind: raise_statement
  any:
    - all: [{regex: '^raise\s+StopIteration(\(\s*\))?\s*$'}, {pattern: $BARE}]
    - all: [{has: {kind: call, all: [{has: {field: function, regex: '^StopIteration$'}}, {has: {field: arguments, has: {nthChild: 1, pattern: $V}}}]}}, {pattern: $WITH}]
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: body, has: {kind: yield, stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}}}
message: "Return from the generator"
fix:
  - replace: {target: $BARE, with: return}
  - replace: {target: $WITH, with: "return $V"}
```

## Examples

### Python

#### Don't

```python
def items(xs):
    for x in xs:
        yield x
    raise StopIteration
```

#### Do

```python
def items(xs):
    for x in xs:
        yield x
    return
```

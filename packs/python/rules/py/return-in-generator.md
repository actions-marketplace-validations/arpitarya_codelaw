---
id: py/return-in-generator
title: "No return value in a generator"
languages: [python]
fix: manual
fix_reason: "whether the value should be yielded or the function split is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/B901]
since: 0.5.0
---

A function that `yield`s does not also `return` a value.

**Why:** The value goes to `StopIteration.value`, which a `for` loop discards; it was usually meant to be yielded.

```codelaw
match:
  kind: return_statement
  regex: '^return\s+\S'
  not: {regex: '^return\s+None$'}
  inside:
    kind: function_definition
    stopBy: {kind: function_definition}
    has: {stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}, kind: yield}
    not: {regex: '^async\s'}
message: "A generator's return value is discarded by `for`; yield it"
```

## How to fix

1. Yield the value, or move the non-generator part into its own function.

## Examples

### Python

#### Don't

```python
def gen():
    yield 1
    return 2
```

#### Do

```python
def gen():
    yield 1
    yield 2
```

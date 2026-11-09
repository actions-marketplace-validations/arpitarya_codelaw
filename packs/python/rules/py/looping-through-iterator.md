---
id: py/looping-through-iterator
title: "An iterator is not looped over again in a nested loop"
languages: [python]
fix: manual
fix_reason: "whether the iterator should be rebuilt each pass, listed once or consumed in one loop is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/looping-through-iterator]
since: 0.5.0
---

A `for` statement that is directly in the body of another loop iterates a name that was last assigned, before that outer loop, an iterator: a generator expression, `map`, `filter`, `zip`, `iter`, `reversed`, `enumerate`, an `itertools` producer, or a call to a generator function this file defines. The name may sit inside calls in the iterable (`list(zip(a, b))`). Not reported: a name rebound inside the outer loop, a list, `range` or set, an inner loop that returns or breaks out of itself, an outer loop that `break`s or returns right after the inner one, an inner `for` under an `if` or `try`, and an `else:` that breaks.

**Why:** An iterator runs out. The second pass of the outer loop finds the inner loop empty.

```codelaw
native: py/looping-through-iterator
```

## How to fix

1. Build the iterator inside the outer loop, or
2. turn it into a list before the loops, or
3. loop over it once.

## Examples

### Python

#### Don't

```python
def pairs(rows):
    cells = (c for c in range(3))
    for row in rows:
        for cell in cells:
            print(row, cell)
```

#### Do

```python
def pairs(rows):
    cells = list(range(3))
    for row in rows:
        for cell in cells:
            print(row, cell)
```

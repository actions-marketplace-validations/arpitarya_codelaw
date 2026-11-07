---
id: py/bad-chained-comparison
title: "A chained comparison does not mix kinds of operator"
languages: [python]
fix: manual
fix_reason: "whether the author meant one chain or two joined comparisons is theirs to say"
fix_exception: value
stability: preview
ported_from: [pylint/bad-chained-comparison]
since: 0.5.0
---

A chain of three or more operands mixes operators from different groups: ordering and equality (`<`, `<=`, `>`, `>=`, `==`, `!=`), identity (`is`, `is not`) and membership (`in`, `not in`). `a is None != b is None` is one chain, `a is None and None != b and b is None`, not two identity tests compared.

**Why:** Python chains every comparison operator the same way, so a mixed chain rarely means what it reads as.

```codelaw
match:
  kind: comparison_operator
  has: {nthChild: 3}
  any:
    - all:
        - has: {regex: '^(is|is\s+not)$'}
        - any:
            - has: {regex: '^(<|<=|>|>=|==|!=)$'}
            - has: {regex: '^(in|not\s+in)$'}
    - all:
        - has: {regex: '^(in|not\s+in)$'}
        - has: {regex: '^(<|<=|>|>=|==|!=)$'}
message: "This chain mixes comparison kinds; Python chains them all with `and`"
```

## How to fix

1. Decide what the chain should test.
2. Write two comparisons joined by `and`, or parenthesise the part meant to be compared as one value: `(a is None) != (b is None)`.

## Examples

### Python

#### Don't

```python
same = a is None != b is None
```

#### Do

```python
same = (a is None) == (b is None)
```

---
id: py/unnecessary-list-index-lookup
title: "Use the item from enumerate(), not xs[i]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLR1736]
since: 0.5.0
---

Inside `for i, x in enumerate(xs):`, `xs[i]` is written `x`.

**Why:** The item is already in hand.

```codelaw
match:
  kind: subscript
  pattern: $L[$I]
  all:
  - not:
      inside: {kind: assignment, field: left}
  - not:
      inside: {kind: augmented_assignment, field: left}
  - not:
      inside: {kind: delete_statement}
  inside:
    kind: block
    stopBy: end
    inside:
      kind: for_statement
      all:
      - has:
          field: left
          kind: pattern_list
          has:
            kind: identifier
            pattern: $I
            not:
              follows: {stopBy: end, kind: identifier}
            precedes: {stopBy: end, kind: identifier, pattern: $X}
      - has: {field: right, pattern: enumerate($L)}
constraints:
  I: {kind: identifier}
  L: {kind: identifier}
message: Use `$X`
fix: $X
```

## Examples

### Python

#### Don't

```python
for i, name in enumerate(names):
    print(names[i])
```

#### Do

```python
for i, name in enumerate(names):
    print(name)
```

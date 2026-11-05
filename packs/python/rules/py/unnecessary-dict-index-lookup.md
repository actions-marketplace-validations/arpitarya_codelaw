---
id: py/unnecessary-dict-index-lookup
title: "Use the value from items(), not d[k]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLR1733]
since: 0.5.0
---

Inside `for k, v in d.items():`, `d[k]` is written `v`.

**Why:** The value is already in hand; the lookup repeats it.

```codelaw
match:
  kind: subscript
  pattern: $D[$K]
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
            pattern: $K
            not:
              follows: {stopBy: end, kind: identifier}
            precedes: {stopBy: end, kind: identifier, pattern: $V}
      - has: {field: right, pattern: $D.items()}
constraints:
  K: {kind: identifier}
  D: {kind: identifier}
message: Use `$V`
fix: $V
```

## Examples

### Python

#### Don't

```python
for name, size in sizes.items():
    print(sizes[name])
```

#### Do

```python
for name, size in sizes.items():
    print(size)
```

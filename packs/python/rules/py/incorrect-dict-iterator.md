---
id: py/incorrect-dict-iterator
title: "d.keys() or d.values(), not items() with a discarded half"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PERF102]
since: 0.5.0
---

`for k, _ in d.items():` is written `for k in d.keys():`, and `for _, v in d.items():` `for v in d.values():`. The fix covers the keys form.

**Why:** Building pairs only to drop half of each is wasted work.

```codelaw
match:
  kind: for_statement
  all:
  - has:
      field: left
      kind: pattern_list
      regex: ^\w+\s*,\s*_$
      pattern: $LEFT
      has: {kind: identifier, regex: '^[^_]', pattern: $K}
  - has:
      field: right
      pattern: $D.items()
      all:
      - {pattern: $IT}
message: Iterate `$D.keys()`
fix:
- replace: {target: $LEFT, with: $K}
- replace: {target: $IT, with: $D.keys()}
```

## Examples

### Python

#### Don't

```python
for name, _ in sizes.items():
    print(name)
```

#### Do

```python
for name in sizes.keys():
    print(name)
```

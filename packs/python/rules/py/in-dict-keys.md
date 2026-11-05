---
id: py/in-dict-keys
title: "k in d, not k in d.keys()"
languages: [python]
fix: suggest
fix_reason: "the same for a dict; a non-dict whose `keys()` differs from its `in` would change"
stability: preview
ported_from: [ruff/SIM118, pylint/consider-iterating-dictionary]
since: 0.5.0
---

Membership in a dict is tested with `k in d`, not `k in d.keys()`.

**Why:** `in` on the dict is the same test without building a view.

```codelaw
match:
  kind: comparison_operator
  all:
  - any:
    - {pattern: $K in $D.keys()}
    - {pattern: $K not in $D.keys()}
  - has: {kind: call, pattern: $CALL, regex: \.keys\(\)$}
  - has: {kind: call, pattern: $D.keys()}
message: Test `in $D` directly
fix:
- replace: {target: $CALL, with: $D}
```

## Examples

### Python

#### Don't

```python
if "a" in opts.keys():
    pass
```

#### Do

```python
if "a" in opts:
    pass
```

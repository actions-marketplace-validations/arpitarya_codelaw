---
id: py/slice-to-remove-prefix-or-suffix
title: "str.removeprefix, not a startswith and a slice"
languages: [python]
fix: auto
fix_reason: "auto for a prefix, and for a suffix written as a non-empty literal; a suffix that may be empty is suggest, since `s[:-0]` is `''` where `removesuffix('')` keeps `s`"
fix_target: { python: "3.9" }
stability: preview
ported_from: [ruff/FURB188]
since: 0.5.0
---

`s[len(p):] if s.startswith(p) else s` is written `s.removeprefix(p)`, and `s[:-len(p)] if s.endswith(p) else s` `s.removesuffix(p)` (Python 3.9+).

**Why:** The method says what the expression does.

```codelaw
match:
  kind: conditional_expression
  any:
    - {pattern: '$S[len($P):] if $S.startswith($P) else $S', all: [{pattern: $PRE}]}
    - {pattern: '$S[:-len($Q)] if $S.endswith($Q) else $S', all: [{pattern: $SUF}]}
constraints:
  S: {any: [{kind: identifier}, {kind: attribute}]}
auto_when:
  any:
    - pattern: '$S[len($P):] if $S.startswith($P) else $S'
    - regex: '\[:-len\([rbuRBU]*("[^"]+"|''[^'']+'')\)\]'
message: "Use `str.removeprefix` or `str.removesuffix`"
fix:
  - replace: {target: $PRE, with: "$S.removeprefix($P)"}
  - replace: {target: $SUF, with: "$S.removesuffix($Q)"}
```

## Examples

### Python

#### Don't

```python
name = name[len("v"):] if name.startswith("v") else name
```

#### Do

```python
name = name.removeprefix("v")
```

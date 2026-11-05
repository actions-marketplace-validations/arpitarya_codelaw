---
id: py/if-expr-with-twisted-arms
title: "a if a else b, not b if not a else a"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/SIM212]
since: 0.5.0
---

`b if not a else a` is written `a if a else b`.

**Why:** The negated test reads backwards.

```codelaw
match:
  pattern: $B if not $A else $A
message: "Write `$A if $A else $B`"
fix: "$A if $A else $B"
```

## Examples

### Python

#### Don't

```python
name = default if not given else given
```

#### Do

```python
name = given if given else default
```
